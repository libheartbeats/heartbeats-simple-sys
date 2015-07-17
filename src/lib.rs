#![allow(non_camel_case_types)]

extern crate libc;

pub mod wrapper;

use libc::{uint64_t, c_double, c_int};

/// Typedef for the window completion callback function.
#[repr(C)]
pub type heartbeat_window_complete = extern fn(*const heartbeat_context,
                                               *const heartbeat_record,
                                               uint64_t);

/// Unsigned global and window data.
#[repr(C)]
pub struct heartbeat_udata {
    global: uint64_t,
    window: uint64_t,
}

/// A heartbeat record with current rates (performance and power).
#[repr(C)]
pub struct heartbeat_record {
    pub id: uint64_t,
    pub user_tag: uint64_t,

    pub work: uint64_t,
    pub start_time: uint64_t,
    pub end_time: uint64_t,
    pub global_perf: c_double,
    pub window_perf: c_double,
    pub instant_perf: c_double,

    pub start_energy: uint64_t,
    pub end_energy: uint64_t,
    pub global_pwr: c_double,
    pub window_pwr: c_double,
    pub instant_pwr: c_double,
}

/// A `heartbeat_context` is used for tracking performance/power of recurring jobs.
#[repr(C)]
pub struct heartbeat_context {
    counter: uint64_t,
    buffer_index: uint64_t,
    read_index: uint64_t,
    window_size: uint64_t,
    window_buffer: *mut heartbeat_record,
    hwc_callback: heartbeat_window_complete,

    td: heartbeat_udata,
    wd: heartbeat_udata,
    ed: heartbeat_udata,
}

extern "C" {
    // Core functions

    pub fn heartbeat_init(hb: *mut heartbeat_context,
                          window_size: uint64_t,
                          window_buffer: *mut heartbeat_record,
                          hwc_callback: Option<heartbeat_window_complete>) -> c_int;

    pub fn heartbeat(hb: *mut heartbeat_context,
                     user_tag: uint64_t,
                     work: uint64_t,
                     start_time: uint64_t,
                     end_time: uint64_t);

    pub fn heartbeat_pow(hb: *mut heartbeat_context,
                         user_tag: uint64_t,
                         work: uint64_t,
                         start_time: uint64_t,
                         end_time: uint64_t,
                         start_energy: uint64_t,
                         end_energy: uint64_t);

    pub fn heartbeat_log_window_buffer(hb: *const heartbeat_context,
                                       fd: c_int,
                                       print_header: c_int) -> c_int;

    // Utility functions

    pub fn hb_get_window_size(hb: *const heartbeat_context) -> uint64_t;

    pub fn hb_get_user_tag(hb: *const heartbeat_context) -> uint64_t;

    pub fn hb_get_global_time(hb: *const heartbeat_context) -> uint64_t;
    pub fn hb_get_window_time(hb: *const heartbeat_context) -> uint64_t;
    pub fn hb_get_global_work(hb: *const heartbeat_context) -> uint64_t;
    pub fn hb_get_window_work(hb: *const heartbeat_context) -> uint64_t;

    pub fn hb_get_global_rate(hb: *const heartbeat_context) -> c_double;
    pub fn hb_get_window_rate(hb: *const heartbeat_context) -> c_double;
    pub fn hb_get_instant_rate(hb: *const heartbeat_context) -> c_double;

    pub fn hb_get_global_energy(hb: *const heartbeat_context) -> uint64_t;
    pub fn hb_get_window_energy(hb: *const heartbeat_context) -> uint64_t;

    pub fn hb_get_global_power(hb: *const heartbeat_context) -> c_double;
    pub fn hb_get_window_power(hb: *const heartbeat_context) -> c_double;
    pub fn hb_get_instant_power(hb: *const heartbeat_context) -> c_double;
}
