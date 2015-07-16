extern crate libc;

pub mod simple;

use libc::{uint64_t, c_double, c_int};

/// Typedef for the window completion callback function.
pub type HeartbeatWindowCompleteFn = extern fn(*const Heartbeat, *const HeartbeatRecord, uint64_t);

/// Time data.
#[repr(C)]
pub struct HeartbeatTimeData {
    total_time: uint64_t,
    window_time: uint64_t,
}

/// Work data.
#[repr(C)]
pub struct HeartbeatWorkData {
    total_work: uint64_t,
    window_work: uint64_t,
}

/// Energy data
#[repr(C)]
pub struct HeartbeatEnergyData {
    total_energy: uint64_t,
    window_energy: uint64_t,
}

/// A Heartbeat record with current rates (performance and power).
#[repr(C)]
pub struct HeartbeatRecord {
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

/// A `Heartbeat` is used for tracking performance/power of recurring jobs.
#[repr(C)]
pub struct Heartbeat {
    counter: uint64_t,
    buffer_index: uint64_t,
    read_index: uint64_t,
    window_size: uint64_t,
    window_buffer: *mut HeartbeatRecord,
    hwc_callback: HeartbeatWindowCompleteFn,

    td: HeartbeatTimeData,
    wd: HeartbeatWorkData,
    ed: HeartbeatEnergyData,
}

extern "C" {
    // Core functions

    pub fn heartbeat_init(hb: *mut Heartbeat,
                          window_size: uint64_t,
                          window_buffer: *mut HeartbeatRecord,
                          hwc_callback: Option<HeartbeatWindowCompleteFn>) -> c_int;

    pub fn heartbeat(hb: *mut Heartbeat,
                     user_tag: uint64_t,
                     work: uint64_t,
                     start_time: uint64_t,
                     end_time: uint64_t);

    pub fn heartbeat_pow(hb: *mut Heartbeat,
                         user_tag: uint64_t,
                         work: uint64_t,
                         start_time: uint64_t,
                         end_time: uint64_t,
                         start_energy: uint64_t,
                         end_energy: uint64_t);

    pub fn heartbeat_log_window_buffer(hb: *const Heartbeat,
                                       fd: c_int,
                                       print_header: c_int) -> c_int;

    // Utility functions

    pub fn hb_get_window_size(hb: *const Heartbeat) -> uint64_t;

    pub fn hb_get_user_tag(hb: *const Heartbeat) -> uint64_t;

    pub fn hb_get_global_time(hb: *const Heartbeat) -> uint64_t;
    pub fn hb_get_window_time(hb: *const Heartbeat) -> uint64_t;
    pub fn hb_get_global_work(hb: *const Heartbeat) -> uint64_t;
    pub fn hb_get_window_work(hb: *const Heartbeat) -> uint64_t;

    pub fn hb_get_global_rate(hb: *const Heartbeat) -> c_double;
    pub fn hb_get_window_rate(hb: *const Heartbeat) -> c_double;
    pub fn hb_get_instant_rate(hb: *const Heartbeat) -> c_double;

    pub fn hb_get_global_energy(hb: *const Heartbeat) -> uint64_t;
    pub fn hb_get_window_energy(hb: *const Heartbeat) -> uint64_t;

    pub fn hb_get_global_power(hb: *const Heartbeat) -> c_double;
    pub fn hb_get_window_power(hb: *const Heartbeat) -> c_double;
    pub fn hb_get_instant_power(hb: *const Heartbeat) -> c_double;
}
