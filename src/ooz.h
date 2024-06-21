/*
 * SPDX-FileCopyrightText: 2024 sirinsidiator
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#pragma once
#include <stdint.h>

int32_t Kraken_Decompress(const uint8_t* src, size_t src_len, uint8_t* dst, size_t dst_len);