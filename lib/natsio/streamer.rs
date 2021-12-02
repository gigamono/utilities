// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.
use std::fmt::Display;

pub struct RequestResponseSubjects {
    pub directives: String, // The namespace the sender and reciever send commands to.
    pub request_body: String, // The namespace request body is sent to.
    pub response_body: String, // The namespace response body is sent to.
}

impl Display for RequestResponseSubjects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestResponseSubjects")
            .field("directives", &self.directives)
            .field("request_body", &self.request_body)
            .field("response_body", &self.response_body)
            .finish()
    }
}
