// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use crate::result::Result;
use hyper::Body;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Uri, Version};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub parts: RequestParts,
    #[serde(skip)]
    pub body: Body, // Body is streamed on per need basis.
}

#[derive(Serialize, Deserialize)]
pub struct RequestParts {
    pub method: String,
    pub uri: Uri,
    pub version: Version,
    pub headers: HashMap<String, Vec<u8>>,
}

impl Request {
    pub async fn from_hyper_request(req: hyper::Request<hyper::Body>) -> Result<Request> {
        let method = Self::from_hyper_method(req.method());
        let headers = Self::from_hyper_headers(req.headers());
        let version = Self::from_hyper_version(&req.version());
        let uri = Self::from_hyper_uri(req.uri());

        let parts = RequestParts {
            method,
            uri,
            version,
            headers,
        };

        let body = req.into_body();

        Ok(Request { parts, body })
    }

    pub fn from_hyper_method(method: &hyper::Method) -> String {
        method.to_string()
    }

    pub fn from_hyper_version(version: &hyper::Version) -> Version {
        match version {
            &hyper::Version::HTTP_09 => Version::Http09,
            &hyper::Version::HTTP_10 => Version::Http10,
            &hyper::Version::HTTP_11 => Version::Http11,
            &hyper::Version::HTTP_2 => Version::H2,
            &hyper::Version::HTTP_3 => Version::H3,
            _ => unreachable!(),
        }
    }

    pub fn from_hyper_headers(headers: &hyper::HeaderMap) -> HashMap<String, Vec<u8>> {
        let mut map: HashMap<String, Vec<u8>> = HashMap::new();
        for (k, v) in headers.into_iter() {
            map.insert(k.to_string(), v.as_bytes().to_owned());
        }

        map
    }

    pub fn from_hyper_uri(uri: &hyper::Uri) -> Uri {
        let scheme = uri.scheme().map(|s| s.to_string());
        let authority = uri.authority().map(|s| s.to_string());
        let path = uri.path().to_owned();
        let query = uri.query().map(|s| s.to_owned());

        Uri {
            scheme,
            authority,
            path,
            query,
        }
    }
}
