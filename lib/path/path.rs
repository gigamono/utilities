// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use std::path;

pub fn get_platform_sep_pattern() -> String {
    // SEC: Create regex pattern of current platform's separator.
    if cfg!(windows) {
        String::from(r"\\")
    } else {
        String::from(path::MAIN_SEPARATOR)
    }
}
