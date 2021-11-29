// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use crate::result::{Context, Result};
use hyper::body;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub parts: Parts,
    pub body: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Parts {
    pub method: String,
    pub uri: Uri,
    pub version: Version,
    pub headers: HashMap<String, Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
pub enum Version {
    Http09,
    Http10,
    Http11,
    H2,
    H3,
}

#[derive(Serialize, Deserialize)]
pub struct Uri {
    pub scheme: Option<String>,
    pub authority: Option<String>,
    pub path: String,
    pub query: Option<String>,
}

impl Request {
    const MAX_ALLOWED_REQUEST_SIZE: u64 = 1024;

    pub async fn from_hyper_request(req: hyper::Request<hyper::Body>) -> Result<Request> {
        // TODO: Replace to_vec with something that does not clone the content. as_ptr maybe.
        // TODO: SEC: Limit the number of chunks read from body to prevent OOM error. Maybe fetch 4 chunks eagerly.
        // TODO: How to implement polled reading/writing between engines, e.g. H1.1 chunked encoding, H2 stream.
        let method = Self::convert_hyper_method(req.method());
        let headers = Self::convert_hyper_headers(req.headers());
        let version = Self::convert_hyper_version(&req.version());
        let uri = Self::convert_hyper_uri(req.uri());

        let parts = Parts {
            method,
            uri,
            version,
            headers,
        };

        let body = req.into_body();

        let body = body::to_bytes(body)
            .await
            .context("reading body chunks")?
            .as_ref()
            .to_vec();

        Ok(Request { parts, body })
    }

    pub fn convert_hyper_method(method: &hyper::Method) -> String {
        method.to_string()
    }

    pub fn convert_hyper_version(version: &hyper::Version) -> Version {
        match version {
            &hyper::Version::HTTP_09 => Version::Http09,
            &hyper::Version::HTTP_10 => Version::Http10,
            &hyper::Version::HTTP_11 => Version::Http11,
            &hyper::Version::HTTP_2 => Version::H2,
            &hyper::Version::HTTP_3 => Version::H3,
            _ => unreachable!(),
        }
    }

    pub fn convert_hyper_headers(headers: &hyper::HeaderMap) -> HashMap<String, Vec<u8>> {
        let mut map: HashMap<String, Vec<u8>> = HashMap::new();
        for (k, v) in headers.into_iter() {
            map.insert(k.to_string(), v.as_bytes().to_owned());
        }

        map
    }

    pub fn convert_hyper_uri(uri: &hyper::Uri) -> Uri {
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
