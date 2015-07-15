use libc::{uint64_t, int64_t, c_double, c_int};
use std::mem;

/// Typedef for the window completion callback function.
pub type HeartbeatWindowCompleteFn = extern fn(*const Heartbeat, *const HeartbeatRecord, uint64_t);

#[link(name = "hbs-pow")]
extern {
    /// Initialize a Heartbeat.
    fn heartbeat_init(hb: *mut Heartbeat,
                      window_size: uint64_t,
                      window_buffer: *mut HeartbeatRecord,
                      hwc_callback: Option<HeartbeatWindowCompleteFn>) -> c_int;

    /// Issue a heartbeat.
    fn heartbeat(hb: *mut Heartbeat,
                 user_tag: uint64_t,
                 work: uint64_t,
                 start_time: int64_t,
                 end_time: int64_t);

    /// Issue a heartbeat with energy data.
    fn heartbeat_pow(hb: *mut Heartbeat,
                     user_tag: uint64_t,
                     work: uint64_t,
                     start_time: int64_t,
                     end_time: int64_t,
                     start_energy: uint64_t,
                     end_energy: uint64_t);

    /// Writes the window buffer to the log specified by the file descriptor.
    pub fn heartbeat_log_window_buffer(hb: *const Heartbeat,
                                       fd: c_int,
                                       print_header: c_int) -> c_int;

    /// Utility function to get the most recent user-specified tag
    fn hb_get_user_tag(hb: *const Heartbeat) -> uint64_t;

    /// Utility function to get the current window performance.
    fn hb_get_window_rate(hb: *const Heartbeat) -> c_double;

    /// Utility function to get the current window power.
    fn hb_get_window_power(hb: *const Heartbeat) -> c_double;
}

/// Time data.
#[derive(Clone, Copy)]
#[repr(C)]
struct HeartbeatTimeData {
    total_time: int64_t,
    window_time: int64_t,
}

/// Work data.
#[derive(Clone, Copy)]
#[repr(C)]
struct HeartbeatWorkData {
    total_work: uint64_t,
    window_work: uint64_t,
}

/// Energy data
#[derive(Clone, Copy)]
#[repr(C)]
struct HeartbeatEnergyData {
    total_energy: uint64_t,
    window_energy: uint64_t,
}

/// A Heartbeat record with current rates (performance and power).
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

/// A `Heartbeat` is used for tracking performance/power of recurring jobs.
/// This represents a C struct.
#[allow(raw_pointer_derive)]
#[derive(Clone, Copy)]
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

/// Heartbeat wrapper. Contains the window data buffer.
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
    use libc::uint64_t;

    extern fn heartbeat_window_complete_callback(hb: *const Heartbeat,
                                                 _hbr: *const HeartbeatRecord,
                                                 _size: uint64_t) {
    const STDOUT: i32 = 1;
    unsafe {
        heartbeat_log_window_buffer(hb, STDOUT, 0);
    }
}

    #[test]
    fn test_simple() {
        const TIME_INC: i64 = 1000000000;
        const ENERGY_INC: u64 = 1000000;
        let mut hb = HeartbeatSimple::new(5, Some(heartbeat_window_complete_callback)).unwrap();
        let mut start_time: i64 = 0;
        let mut end_time: i64 = TIME_INC;
        let mut start_energy: u64 = 0;
        let mut end_energy: u64 = ENERGY_INC;
        let mut tag: u64 = 0;
        for _ in 0..10 {
            hb.heartbeat_pow(tag, 1, start_time, end_time, start_energy, end_energy);
            assert!(hb.get_tag() == tag);
            assert!(hb.get_window_perf() == 1.0);
            assert!(hb.get_window_pwr() == 1.0);
            tag += 1;
            start_time = end_time;
            end_time += TIME_INC;
            start_energy = end_energy;
            end_energy += ENERGY_INC;
        }
    }
}
