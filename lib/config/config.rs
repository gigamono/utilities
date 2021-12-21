// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use serde::Deserialize;

use crate::nested_struct;

nested_struct! {
    Permissions {
        fs (FsPermissions {
            open (Vec<String>),
            write (Vec<String>),
        }),
    }
}
