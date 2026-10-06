/*
 * SPDX-FileCopyrightText: 2024 sirinsidiator
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::fs;
use std::path::Path;

fn main() {
    copy_portable_stdafx();
    remove_main_function_and_load_lib();
    cxx_build::bridge("src/lib.rs")
        .file("src/ooz/kraken.cpp")
        .file("src/ooz/bitknit.cpp")
        .file("src/ooz/lzna.cpp")
        .flag_if_supported("-fno-exceptions")
        .compile("cxx-rust-ooz");
}

fn copy_portable_stdafx() {
    let src = Path::new("src/stdafx.h");
    let dst = Path::new("src/ooz/stdafx.h");
    fs::copy(src, dst).unwrap_or_else(|e| panic!("failed to copy {:?} -> {:?}: {e}", src, dst));
}

fn remove_main_function_and_load_lib() {
    let path = "src/ooz/kraken.cpp";
    let content = fs::read_to_string(path).unwrap();
    if content.contains("#ifdef WITH_MAIN") {
        return;
    }

    let modified = content.replace("void LoadLib() {", "#ifdef WITH_MAIN\r\nvoid LoadLib() {");
    let modified = format!("{}\r\n#endif", modified);
    fs::write(path, modified).unwrap();
}
