// Copyright 2021 the Gigamono authors. All rights reserved. GPL-3.0 License.

use std::path;

pub fn get_platform_sep_pattern() -> String {
    // SEC: Create regex pattern of current platform's separator.
    if cfg!(windows) {
        String::from(r"\\")
    } else {
        String::from(path::MAIN_SEPARATOR)
    }
}
