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
#ifndef PCIE_IRQ_AWS_H__
#define PCIE_IRQ_AWS_H__

#define AWS_NUM_IRQ_CONTROLLERS 4

int pcie_aws_irqs_init(struct tlkm_device *dev, struct list_head *interrupts);
void pcie_aws_irqs_exit(struct tlkm_device *dev);
int pcie_aws_irqs_request_platform_irq(struct tlkm_device *dev,
				       struct tlkm_irq_mapping *mapping);
void pcie_aws_irqs_release_platform_irq(struct tlkm_device *dev,
					struct tlkm_irq_mapping *mapping);

#endif /* PCIE_IRQ_H__ */
