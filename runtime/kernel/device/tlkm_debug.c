/*
 * Copyright (c) 2014-2020 Embedded Systems and Applications, TU Darmstadt.
 *
 * This file is part of TaPaSCo 
 * (see https://github.com/esa-tu-darmstadt/tapasco).
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */
//! @file	tlkm_debug.c
//! @brief	Misc device interface to TaPaSCo debug capabilities
//! @authors	K. Meinhard, TU Darmstadt (kaimeinhard@hotmail.de)
//!
#include <linux/miscdevice.h>
#include <linux/uaccess.h>
#include <linux/fs.h>
#include <linux/slab.h>
#include "tlkm_module.h"
#include "tlkm_debug.h"
#include "tlkm_logging.h"
#include "tlkm_device_ioctl_cmds.h"

#define TLKM_DEBUG_BUFSZ 500
#define TLKM_DEV_DEBUG_FN "tlkm_debug_%02u"

static void *mapped_address;

inline static tlkm_component_t get_debug_component (struct tlkm_device* inst)
{
    tlkm_component_t empty_comp = {"", 0, 0};
    int i;
    for (i = 0; i < TLKM_COMPONENT_MAX; i += 1) {
        int cmp = strncmp("PLATFORM_COMPONENT_DEBUG", inst->components[i].name,
                TLKM_COMPONENTS_NAME_MAX);
        if (cmp == 0)
        {
            return inst->components[i];
        }
    }

	return empty_comp;
}

inline static dev_addr_t get_debug_offset_from_file(struct file *file)
{
	struct miscdevice *dev = (struct miscdevice *)file->private_data;
	struct tlkm_device *inst = container_of(dev, struct tlkm_device, debug_dev);
    tlkm_component_t debug_comp = get_debug_component(inst);

	return debug_comp.offset + inst->plat.base;
}


static ssize_t tlkm_debug_read(struct file *file, char __user *usr,
				       size_t sz, loff_t *loff)
{
	ssize_t sl;
	dev_addr_t dev_id = get_debug_offset_from_file(file);
	const char *const fmt = "Debug Offset: %lx\n"
                             "CtrlReg: %x\n"
                             "IDCODE: %x\n"
                             "dtmcs: %x\n"
                             "dmi_data: %x\n"
                             "dmi_addr: %x\n";

    unsigned int reg0 = ioread32(mapped_address);
    unsigned int reg1 = ioread32(mapped_address + 4);
    unsigned int reg2 = ioread32(mapped_address + 8);
    unsigned int reg3 = ioread32(mapped_address + 12);
    unsigned int reg4 = ioread32(mapped_address + 16);

	char tmp[TLKM_DEBUG_BUFSZ];
	snprintf(tmp, TLKM_DEBUG_BUFSZ, fmt, dev_id, reg0, reg1, reg2, reg3, reg4);
	sl = strlen(tmp) + 1;
	if (sl - *loff > 0) {
		ssize_t rl = strlen(&tmp[*loff]) + 1;
		*loff += rl - copy_to_user(usr, tmp, strlen(&tmp[*loff]) + 1);
		return rl;
	}
	return 0;
}

int tlkm_debug_mmap(struct file *fp, struct vm_area_struct *vm)
{
    struct miscdevice *dev = (struct miscdevice *)fp->private_data;
    struct tlkm_device *dp = container_of(dev, struct tlkm_device, debug_dev);
    tlkm_component_t debug_comp = get_debug_component(dp);
    ssize_t const sz = vm->vm_end - vm->vm_start;
    ulong const off = vm->vm_pgoff << PAGE_SHIFT;
    ulong kptr = addr2map_off(dp, off);
    DEVLOG(dp->dev_id, TLKM_LF_DEBUG, "received mmap: offset = 0x%08lx", off);
    if (kptr == -1) {
        DEVERR(dp->dev_id, "invalid address: 0x%08lx", off);
        return -ENXIO;
    }

    DEVLOG(dp->dev_id, TLKM_LF_DEBUG,
           "mapping %zu bytes from physical address 0x%lx to user space 0x%lx-0x%lx",
           sz, dp->base_offset + kptr + debug_comp.offset, vm->vm_start, vm->vm_end);
    vm->vm_page_prot = pgprot_noncached(vm->vm_page_prot);
    if (io_remap_pfn_range(vm, vm->vm_start,
            (dp->base_offset + kptr + debug_comp.offset) >> PAGE_SHIFT, sz,
            vm->vm_page_prot)) {
        DEVWRN(dp->dev_id, "io_remap_pfn_range failed!");
        return -EAGAIN;
    }

    DEVLOG(dp->dev_id, TLKM_LF_DEBUG, "register space mapping successful");
    return 0;
}

static const struct file_operations tlkm_debug_fops = {
    .owner = THIS_MODULE,
    .read = tlkm_debug_read,
    .mmap = tlkm_debug_mmap,
};

int tlkm_debug_init(struct tlkm_device *dev)
{
    dev_addr_t debug_offset = 0;
    u64 debug_size = 0;
    tlkm_component_t debug_comp;

	int ret = 0;
	char fn[256];
	snprintf(fn, 256, TLKM_DEV_DEBUG_FN, dev->dev_id);
	DEVLOG(dev->dev_id, TLKM_LF_DEBUG,
	       "setting up debug file %s ...", fn);
	dev->debug_dev.minor = MISC_DYNAMIC_MINOR;
	dev->debug_dev.name = kstrdup(fn, GFP_KERNEL);
	dev->debug_dev.fops = &tlkm_debug_fops;
	if ((ret = misc_register(&dev->debug_dev))) {
		DEVERR(dev->dev_id, "could not setup %s: %d", fn, ret);
		return ret;
	}

    debug_comp = get_debug_component(dev);

    debug_offset = debug_comp.offset + dev->plat.base;
    debug_size = debug_comp.size;

	if (debug_offset == 0 || debug_size == 0) {
		DEVERR(dev->dev_id, "could not find debug module %s: %d", fn, ret);
		return ret;
	}

    // Map den Debug module
    mapped_address = ioremap(debug_offset, debug_size);

	DEVLOG(dev->dev_id, TLKM_LF_DEBUG, "%s is set up", fn);
	return 0;
}

void tlkm_debug_exit(struct tlkm_device *dev)
{
	kfree(dev->debug_dev.name);
	misc_deregister(&dev->debug_dev);
	memset(&dev->debug_dev, 0, sizeof(dev->debug_dev));
	DEVLOG(dev->dev_id, TLKM_LF_DEBUG,
	       "removed debug miscdev");
}
