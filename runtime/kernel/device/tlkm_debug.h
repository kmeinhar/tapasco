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
//! @file	tlkm_debug.h
//! @brief	Misc device interface to TaPaSCo debug capabilities
//! @authors	K. Meinhard, TU Darmstadt (kaimeinhard@hotmail.de)
//!
#ifndef TLKM_DEBUG_H__
#define TLKM_DEBUG_H__

#include "tlkm_device.h"

// TODO ability to turn of debug device
int tlkm_debug_init(struct tlkm_device *dev);
void tlkm_debug_exit(struct tlkm_device *dev);

#endif /* TLKM_DEBUG_H__ */
