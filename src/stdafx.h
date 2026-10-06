/*
 * SPDX-FileCopyrightText: 2024 sirinsidiator
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#pragma once

#ifdef _WIN32

#include "targetver.h"

#define _CRT_SECURE_NO_WARNINGS 1

#include <tchar.h>
#include <intrin.h>
#include <Windows.h>

#else

#include <stdint.h>
#include <emmintrin.h>   // SSE2: __m128i, _mm_*
#define __forceinline  __attribute__((always_inline)) inline
#define WINAPI

#ifndef _BitScanForward
#define _BitScanForward(p, x) do { \
    unsigned int _t = (unsigned int)(x); \
    *(p) = _t ? (unsigned int)__builtin_ctz(_t) : 0u; \
} while (0)
#endif

#ifndef _BitScanReverse
#define _BitScanReverse(p, x) do { \
    unsigned int _t = (unsigned int)(x); \
    *(p) = _t ? ((unsigned int)__builtin_clz(_t) ^ 31u) : 0u; \
} while (0)
#endif

#ifndef _byteswap_ushort
#define _byteswap_ushort(x)  __builtin_bswap16(x)
#endif

#ifndef _byteswap_ulong
#define _byteswap_ulong(x)   __builtin_bswap32(x)
#endif

#ifndef _byteswap_uint64
#define _byteswap_uint64(x)  __builtin_bswap64(x)
#endif

#ifndef _rotl
#  define _rotl(x, n)  ((unsigned int)(x) << (n) | (unsigned int)(x) >> (32 - (n)))
#endif

#ifndef _rotr
#  define _rotr(x, n)  ((unsigned int)(x) >> (n) | (unsigned int)(x) << (32 - (n)))
#endif

#endif // _WIN32

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#ifdef _MSC_VER
#pragma warning (disable: 4244)
#endif

typedef unsigned char byte;
typedef unsigned char uint8;
typedef unsigned int uint32;
#ifdef _WIN32
typedef unsigned __int64 uint64;
typedef signed   __int64 int64;
#else
typedef unsigned long long uint64;
typedef signed   long long int64;
#endif
typedef signed   int      int32;
typedef unsigned short    uint16;
typedef signed   short    int16;
typedef unsigned int      uint;
