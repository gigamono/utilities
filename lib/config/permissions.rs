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

                #[serde(default)]
                execute (Vec<String>),
            }
        ),

        #[serde(default)]
        http_event (
            HttpEvent {
                #[serde(default = "HttpEvent::default_request_read")]
                request_read (bool),

                #[serde(default = "HttpEvent::default_response_send")]
                response_send (bool),
            }
        ),

        #[serde(default)]
        env (Vec<String>),

        #[serde(default)]
        db (
            #[derive(Default)]
            DB {
                #[serde(default)]
                create (Vec<String>),

                #[serde(default)]
                database_delete (Vec<String>),

                #[serde(default)]
                table_create (Vec<String>),

                #[serde(default)]
                table_delete (Vec<String>),

                #[serde(default)]
                column_create (Vec<String>),

                #[serde(default)]
                column_delete (Vec<String>),

                #[serde(default)]
                row_create (Vec<String>),

                #[serde(default)]
                row_read (Vec<String>),

                #[serde(default)]
                row_write (Vec<String>),

                #[serde(default)]
                row_delete (Vec<String>),
            }
        ),

        #[serde(default)]
        p2p (
            #[derive(Default)]
            P2P {
                #[serde(default)]
                socket_open (Vec<String>),

                #[serde(default)]
                socket_close (Vec<String>),

                #[serde(default)]
                peer_connect (Vec<String>),

                #[serde(default)]
                peer_disconnect (Vec<String>),

                #[serde(default)]
                peer_send (Vec<String>),

                #[serde(default)]
                peer_receive (Vec<String>),
            }
        )
    }
}

// SEC: Must default to false to prevent privilege escalation.
impl HttpEvent {
    fn default_request_read() -> bool {
        false
    }

    fn default_response_send() -> bool {
        false
    }
}

impl Default for HttpEvent {
    // SEC: Must default to false to prevent privilege escalation.
    fn default() -> Self {
        Self {
            request_read: false,
            response_send: false,
        }
    }
}
