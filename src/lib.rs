/*
 * SPDX-FileCopyrightText: 2024 sirinsidiator
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("rust-ooz/src/ooz.h");

        unsafe fn Kraken_Decompress(
            src: *const u8,
            src_len: usize,
            dst: *mut u8,
            dst_len: usize,
        ) -> i32;
    }
}

pub fn read_partial_file(path: &str, offset: usize, length: usize) -> Result<Vec<u8>, String> {
    match File::open(path) {
        Ok(mut file) => {
            let mut output = vec![0; length];
            if file.seek(SeekFrom::Start(offset as u64)).is_err() {
                return Err(format!("Unable to seek to offset {}", offset));
            }
            if file.read_exact(&mut output).is_err() {
                return Err(format!("Unable to read {} bytes", length));
            }
            return Ok(output);
        }
        Err(err) => {
            let err = format!("{:?}", err);
            return Err(err);
        }
    }
}

pub fn decompress(
    path: &str,
    offset: usize,
    compressed_size: usize,
    file_size: usize,
) -> Result<Vec<u8>, String> {
    let input = read_partial_file(path, offset, compressed_size);
    if input.is_err() {
        return Err(input.unwrap_err());
    }
    let input = input.unwrap();

    unsafe {
        // ooz tends to write outside of the buffer, so we need to allocate a bit more
        let mut output = vec![0; file_size + 64];
        let result_size = ffi::Kraken_Decompress(
            input.as_ptr(),
            compressed_size,
            output.as_mut_ptr(),
            file_size,
        );
        if result_size < 0 {
            return Err(format!(
                "Error: Failed to decompress (result size: {})",
                result_size
            ));
        } else if result_size as usize != file_size {
            return Err(format!(
                "Error: Decompressed size mismatch (expected: {}, actual: {})",
                file_size, result_size
            ));
        }
        output.truncate(file_size);

        if output.len() > 16 && output[0..4] == [0, 0, 0, 0] {
            // output has some strange header which we skip for now (seen in game.mnf related archives)
            let mut offset = 4;
            offset += 4 + u32::from_be_bytes([
                output[offset],
                output[offset + 1],
                output[offset + 2],
                output[offset + 3],
            ]) as usize;
            offset += 4 + u32::from_be_bytes([
                output[offset],
                output[offset + 1],
                output[offset + 2],
                output[offset + 3],
            ]) as usize;
            output = output[offset..].to_vec();
        }
        return Ok(output);
    }
}
