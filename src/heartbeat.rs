use libc::{uint64_t, int64_t, c_double, c_int};
use std::mem;

pub type HeartbeatWindowCompleteFn = extern fn(*const Heartbeat, *const HeartbeatRecord, uint64_t);

#[link(name = "hbs-pow")]
extern {
    fn heartbeat_init(hb: *mut Heartbeat,
                      window_size: uint64_t,
                      window_buffer: *mut HeartbeatRecord,
                      hwc_callback: Option<HeartbeatWindowCompleteFn>) -> c_int;

    fn heartbeat(hb: *mut Heartbeat,
                 user_tag: uint64_t,
                 work: uint64_t,
                 start_time: int64_t,
                 end_time: int64_t);

    fn heartbeat_pow(hb: *mut Heartbeat,
                     user_tag: uint64_t,
                     work: uint64_t,
                     start_time: int64_t,
                     end_time: int64_t,
                     start_energy: uint64_t,
                     end_energy: uint64_t);

    fn hb_get_user_tag(hb: *const Heartbeat) -> uint64_t;

    fn hb_get_window_rate(hb: *const Heartbeat) -> c_double;

    fn hb_get_window_power(hb: *const Heartbeat) -> c_double;
}

#[derive(Clone, Copy)]
#[repr(C)]
struct HeartbeatTimeData {
    total_time: int64_t,
    window_time: int64_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct HeartbeatWorkData {
    total_work: uint64_t,
    window_work: uint64_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct HeartbeatEnergyData {
    total_energy: uint64_t,
    window_energy: uint64_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HeartbeatRecord {
    pub id: uint64_t,
    pub user_tag: uint64_t,

    pub work: uint64_t,
    pub start_time: int64_t,
    pub end_time: int64_t,
    pub global_perf: c_double,
    pub window_perf: c_double,
    pub instant_perf: c_double,

    pub start_energy: uint64_t,
    pub end_energy: uint64_t,
    pub global_pwr: c_double,
    pub window_pwr: c_double,
    pub instant_pwr: c_double,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy)]
#[repr(C)]
/// A `Heartbeat` is used for tracking performance/power of recurring jobs.
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

pub struct HeartbeatSimple {
    pub hb: Heartbeat,
    pub hbr: Vec<HeartbeatRecord>,
}

impl HeartbeatSimple {
    /// Allocate and initialize a new `Heartbeat`.
    pub fn new(window_size: usize,
               hwc_callback: Option<HeartbeatWindowCompleteFn>) -> Result<HeartbeatSimple, &'static str> {
        unsafe {
            let mut hb = HeartbeatSimple {
                hb: mem::uninitialized(),
                hbr: Vec::with_capacity(window_size)
            };
            match heartbeat_init(&mut hb.hb, window_size as u64, hb.hbr.as_mut_ptr(), hwc_callback) {
                0 => Ok(hb),
                _ => Err("Failed to initialize heartbeat")
            }
        }
    }

    /// Issue a heartbeat.
    pub fn heartbeat(&mut self,
                     tag: u64,
                     work: u64,
                     start_time: i64,
                     end_time: i64) {
        unsafe {
            heartbeat(&mut self.hb, tag, work, start_time, end_time)
        }
    }

    /// Issue a heartbeat with energy data.
    pub fn heartbeat_pow(&mut self,
                         tag: u64,
                         work: u64,
                         start_time: i64,
                         end_time: i64,
                         start_energy: u64,
                         end_energy: u64) {
        unsafe {
            heartbeat_pow(&mut self.hb, tag, work, start_time, end_time, start_energy, end_energy)
        }
    }

    /// Utility function to get the most recent user-specified tag
    pub fn get_tag(&self) -> u64 {
        unsafe {
            hb_get_user_tag(&self.hb)
        }
    }

    /// Utility function to get the current window performance.
    pub fn get_window_perf(&self) -> f64 {
        unsafe {
            hb_get_window_rate(&self.hb)
        }
    }

    /// Utility function to get the current window power.
    pub fn get_window_pwr(&self) -> f64 {
        unsafe {
            hb_get_window_power(&self.hb)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let mut hb = HeartbeatSimple::new(20, None).unwrap();
        hb.heartbeat(1, 1, 0, 1000000000);
        assert!(hb.get_tag() == 1);
        assert!(hb.get_window_perf() == 1.0);
        hb.heartbeat_pow(2, 1, 1000000000, 2000000000, 0, 2000000);
        assert!(hb.get_window_pwr() == 1.0)
    }
}
