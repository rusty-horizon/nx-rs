#![allow(warnings)]
#![no_std]

include!(concat!(env!("OUT_DIR"),"/libnx.rs"));

#[cfg(feature = "twili")]
include!(concat!(env!("OUT_DIR"),"/twili.rs"));
