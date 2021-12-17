// Copyright 2021 the Gigamono authors. All rights reserved. Apache 2.0 license.

use crate::{errors, result::Result};
use std::net::{SocketAddr, ToSocketAddrs};

pub fn parse_socket_address(address_str: &str) -> Result<SocketAddr> {
    let mut addr_iter = address_str.to_socket_addrs()?;
    if let Some(addr) = addr_iter.next() {
        Ok(addr)
    } else {
        errors::type_error_t("invalid socket address syntax")
    }
}
