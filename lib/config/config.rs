// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use serde::Deserialize;

use crate::nested_struct;

nested_struct! {
    Permissions {
        #[serde(default)]
        fs (
            #[derive(Default)]
            FsPermissions {
                open (Vec<String>),
                read (Vec<String>),
                write (Vec<String>),
            }
        ),

        #[serde(default)]
        http_event (
            #[derive(Default)]
            HttpEvent {
                read_request (Vec<String>),
                send_request (Vec<String>),
            }
        ),
    }
}
