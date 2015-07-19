#![allow(non_camel_case_types)]

extern crate libc;

pub mod wrapper;

use libc::{uint64_t, c_double, c_int};

/// Typedef for the window completion callback function.
#[repr(C)]
pub type heartbeat_pow_window_complete = extern fn(*const heartbeat_pow_context);

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
}

/// A heartbeat record with current rates (performance and power).
#[repr(C)]
pub struct heartbeat_pow_record {
    pub id: uint64_t,
    pub user_tag: uint64_t,

    pub work: uint64_t,
    pub start_time: uint64_t,
    pub end_time: uint64_t,
    pub perf: heartbeat_rates,

    pub start_energy: uint64_t,
    pub end_energy: uint64_t,
    pub pwr: heartbeat_rates,
}

/// A `heartbeat_pow_context` is used for tracking performance/power of recurring jobs.
#[repr(C)]
pub struct heartbeat_pow_context {
    ws: heartbeat_window_state,
    window_buffer: *mut heartbeat_pow_record,
    counter: uint64_t,
    lock: c_int,
    hwc_callback: heartbeat_pow_window_complete,

    td: heartbeat_udata,
    wd: heartbeat_udata,
    ed: heartbeat_udata,
}

extern "C" {
    // Core functions

    pub fn heartbeat_pow_init(hb: *mut heartbeat_pow_context,
                              window_size: uint64_t,
                              window_buffer: *mut heartbeat_pow_record,
                              hwc_callback: Option<heartbeat_pow_window_complete>) -> c_int;

    pub fn heartbeat_pow(hb: *mut heartbeat_pow_context,
                         user_tag: uint64_t,
                         work: uint64_t,
                         start_time: uint64_t,
                         end_time: uint64_t,
                         start_energy: uint64_t,
                         end_energy: uint64_t);

    pub fn heartbeat_pow_log_window_buffer(hb: *const heartbeat_pow_context,
                                           fd: c_int,
                                           print_header: c_int) -> c_int;

    // Utility functions

    pub fn hb_pow_get_window_size(hb: *const heartbeat_pow_context) -> uint64_t;

    pub fn hb_pow_get_user_tag(hb: *const heartbeat_pow_context) -> uint64_t;

    pub fn hb_pow_get_global_time(hb: *const heartbeat_pow_context) -> uint64_t;
    pub fn hb_pow_get_window_time(hb: *const heartbeat_pow_context) -> uint64_t;
    pub fn hb_pow_get_global_work(hb: *const heartbeat_pow_context) -> uint64_t;
    pub fn hb_pow_get_window_work(hb: *const heartbeat_pow_context) -> uint64_t;

    pub fn hb_pow_get_global_perf(hb: *const heartbeat_pow_context) -> c_double;
    pub fn hb_pow_get_window_perf(hb: *const heartbeat_pow_context) -> c_double;
    pub fn hb_pow_get_instant_perf(hb: *const heartbeat_pow_context) -> c_double;

    pub fn hb_pow_get_global_energy(hb: *const heartbeat_pow_context) -> uint64_t;
    pub fn hb_pow_get_window_energy(hb: *const heartbeat_pow_context) -> uint64_t;

    pub fn hb_pow_get_global_power(hb: *const heartbeat_pow_context) -> c_double;
    pub fn hb_pow_get_window_power(hb: *const heartbeat_pow_context) -> c_double;
    pub fn hb_pow_get_instant_power(hb: *const heartbeat_pow_context) -> c_double;
}
