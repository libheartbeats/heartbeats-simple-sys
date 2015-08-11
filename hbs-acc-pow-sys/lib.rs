#![allow(non_camel_case_types)]

extern crate libc;
extern crate hbs_common_sys;

use libc::{uint64_t, c_double, c_int};
use hbs_common_sys::{heartbeat_udata, heartbeat_rates, heartbeat_window_state};

/// Typedef for the window completion callback function.
#[repr(C)]
pub type heartbeat_acc_pow_window_complete = extern fn(*const heartbeat_acc_pow_context);

/// A heartbeat record with current rates (performance and accuracy).
#[repr(C)]
pub struct heartbeat_acc_pow_record {
    pub id: uint64_t,
    pub user_tag: uint64_t,

    pub work: uint64_t,
    pub wd: heartbeat_udata,
    pub start_time: uint64_t,
    pub end_time: uint64_t,
    pub td: heartbeat_udata,
    pub perf: heartbeat_rates,

    pub accuracy: uint64_t,
    pub ad: heartbeat_udata,
    pub acc: heartbeat_rates,

    pub start_energy: uint64_t,
    pub end_energy: uint64_t,
    pub ed: heartbeat_udata,
    pub pwr: heartbeat_rates,
}

/// A `heartbeat_acc_pow_context` is used for tracking performance/accuracy of recurring jobs.
#[repr(C)]
pub struct heartbeat_acc_pow_context {
    pub ws: heartbeat_window_state,
    pub window_buffer: *mut heartbeat_acc_pow_record,
    pub counter: uint64_t,
    pub lock: c_int,
    pub hwc_callback: heartbeat_acc_pow_window_complete,

    pub td: heartbeat_udata,
    pub wd: heartbeat_udata,
    pub ad: heartbeat_udata,
    pub ed: heartbeat_udata,
}

extern "C" {
    // Core functions

    pub fn heartbeat_acc_pow_init(hb: *mut heartbeat_acc_pow_context,
                                  window_size: uint64_t,
                                  window_buffer: *mut heartbeat_acc_pow_record,
                                  hwc_callback: Option<heartbeat_acc_pow_window_complete>) -> c_int;

    pub fn heartbeat_acc_pow(hb: *mut heartbeat_acc_pow_context,
                             user_tag: uint64_t,
                             work: uint64_t,
                             start_time: uint64_t,
                             end_time: uint64_t,
                             accuracy: uint64_t,
                             start_energy: uint64_t,
                             end_energy: uint64_t);

    pub fn hb_acc_pow_log_header(fd: c_int) -> c_int;

    pub fn hb_acc_pow_log_window_buffer(hb: *const heartbeat_acc_pow_context,
                                        fd: c_int) -> c_int;

    // Utility functions

    pub fn hb_acc_pow_get_window_size(hb: *const heartbeat_acc_pow_context) -> uint64_t;

    pub fn hb_acc_pow_get_user_tag(hb: *const heartbeat_acc_pow_context) -> uint64_t;

    pub fn hb_acc_pow_get_global_time(hb: *const heartbeat_acc_pow_context) -> uint64_t;
    pub fn hb_acc_pow_get_window_time(hb: *const heartbeat_acc_pow_context) -> uint64_t;
    pub fn hb_acc_pow_get_global_work(hb: *const heartbeat_acc_pow_context) -> uint64_t;
    pub fn hb_acc_pow_get_window_work(hb: *const heartbeat_acc_pow_context) -> uint64_t;

    pub fn hb_acc_pow_get_global_perf(hb: *const heartbeat_acc_pow_context) -> c_double;
    pub fn hb_acc_pow_get_window_perf(hb: *const heartbeat_acc_pow_context) -> c_double;
    pub fn hb_acc_pow_get_instant_perf(hb: *const heartbeat_acc_pow_context) -> c_double;

    pub fn hb_acc_pow_get_global_accuracy(hb: *const heartbeat_acc_pow_context) -> uint64_t;
    pub fn hb_acc_pow_get_window_accuracy(hb: *const heartbeat_acc_pow_context) -> uint64_t;

    pub fn hb_acc_pow_get_global_accuracy_rate(hb: *const heartbeat_acc_pow_context) -> c_double;
    pub fn hb_acc_pow_get_window_accuracy_rate(hb: *const heartbeat_acc_pow_context) -> c_double;
    pub fn hb_acc_pow_get_instant_accuracy_rate(hb: *const heartbeat_acc_pow_context) -> c_double;

    pub fn hb_acc_pow_get_global_energy(hb: *const heartbeat_acc_pow_context) -> uint64_t;
    pub fn hb_acc_pow_get_window_energy(hb: *const heartbeat_acc_pow_context) -> uint64_t;

    pub fn hb_acc_pow_get_global_power(hb: *const heartbeat_acc_pow_context) -> c_double;
    pub fn hb_acc_pow_get_window_power(hb: *const heartbeat_acc_pow_context) -> c_double;
    pub fn hb_acc_pow_get_instant_power(hb: *const heartbeat_acc_pow_context) -> c_double;
}
