/*
 * SPDX-FileCopyrightText: 2026 sirinsidiator
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#include <stddef.h>
#include <stdint.h>

int32_t Kraken_Decompress(const uint8_t* src, size_t src_len, uint8_t* dst, size_t dst_len);

extern "C" int32_t ooz_kraken_decompress(
    const uint8_t* src, size_t src_len, uint8_t* dst, size_t dst_len) {
    return Kraken_Decompress(src, src_len, dst, dst_len);
}
