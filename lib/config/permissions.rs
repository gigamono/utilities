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
                create (Vec<String>),

                #[serde(default)]
                read (Vec<String>),

                #[serde(default)]
                write (Vec<String>),
            }
        ),

        #[serde(default)]
        http_event (
            HttpEvent {
                #[serde(default = "HttpEvent::default_read_request")]
                read_request (bool),

                #[serde(default = "HttpEvent::default_send_response")]
                send_response (bool),
            }
        ),
    }
}

// SEC: Must default to false to prevent privilege escalation.
impl HttpEvent {
    fn default_read_request() -> bool {
        false
    }

    fn default_send_response() -> bool {
        false
    }
}

impl Default for HttpEvent {
    // SEC: Must default to false to prevent privilege escalation.
    fn default() -> Self {
        Self {
            read_request: false,
            send_response: false,
        }
    }
}
