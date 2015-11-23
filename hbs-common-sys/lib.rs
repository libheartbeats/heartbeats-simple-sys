#![allow(non_camel_case_types)]

extern crate libc;

use libc::{uint64_t, c_double, c_int};

/// Unsigned global and window data.
#[repr(C)]
pub struct heartbeat_udata {
    pub global: uint64_t,
    pub window: uint64_t,
}

/// Rate data.
#[repr(C)]
pub struct heartbeat_rates {
    pub global: c_double,
    pub window: c_double,
    pub instant: c_double,
}

/// State for window buffer
#[repr(C)]
pub struct heartbeat_window_state {
    pub buffer_index: uint64_t,
    pub read_index: uint64_t,
    pub window_size: uint64_t,
    pub log_fd: c_int,
}
