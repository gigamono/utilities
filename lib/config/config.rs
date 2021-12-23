// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use serde::Deserialize;

use crate::nested_struct;

nested_struct! {
    Permissions {
        #[serde(default)]
        fs (
            #[derive(Default)]
            FsPermissions {
                #[serde(default)]
                open (Vec<String>),

                #[serde(default)]
                read (Vec<String>),

                #[serde(default)]
                write (Vec<String>),
            }
        ),

        #[serde(default)]
        http_event (
            #[derive(Default)]
            HttpEvent {
                #[serde(default)]
                read_request (bool),

                #[serde(default)]
                send_request (bool),
            }
        ),
    }
}
