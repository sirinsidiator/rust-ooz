/*
 * SPDX-FileCopyrightText: 2024 sirinsidiator
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::fs;

fn main() {
    remove_main_function();
    cxx_build::bridge("src/lib.rs")
        .file("src/ooz/kraken.cpp")
        .file("src/ooz/bitknit.cpp")
        .file("src/ooz/lzna.cpp")
        .flag_if_supported("-fno-exceptions")
        .compile("cxx-rust-ooz");
}

fn remove_main_function() {
    let path = "src/ooz/kraken.cpp";
    let content = fs::read_to_string(path).unwrap();
    if content.contains("#ifdef WITH_MAIN") {
        return;
    }

    let modified = content.replace("int main(", "#ifdef WITH_MAIN\r\nint main(");
    let modified = format!("{}\r\n#endif", modified);
    fs::write(path, modified).unwrap();
}
