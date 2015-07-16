use std::mem;
use heartbeat::{self, Heartbeat, HeartbeatRecord, HeartbeatWindowCompleteFn};
#[cfg(unix)]
use std::fs::File;
#[cfg(unix)]
use std::os::unix::io::AsRawFd;

/// Heartbeat wrapper. Contains the window data buffer.
pub struct HeartbeatSimple {
    pub hb: Heartbeat,
    pub hbr: Vec<HeartbeatRecord>,
}

impl HeartbeatSimple {
    /// Allocate and initialize a new `Heartbeat`.
    pub fn new(window_size: usize,
               hwc_callback: Option<HeartbeatWindowCompleteFn>) -> Result<HeartbeatSimple, &'static str> {
        let mut hbr = Vec::with_capacity(window_size);
        unsafe {
            let mut hb = mem::uninitialized();
            match heartbeat::heartbeat_init(&mut hb, hbr.capacity() as u64, hbr.as_mut_ptr(), hwc_callback) {
                0 => Ok(HeartbeatSimple { hb: hb, hbr: hbr, }),
                _ => Err("Failed to initialize heartbeat")
            }
        }
    }

    /// Issue a heartbeat.
    pub fn heartbeat(&mut self,
                     tag: u64,
                     work: u64,
                     start_time: u64,
                     end_time: u64) {
        unsafe {
            heartbeat::heartbeat(&mut self.hb, tag, work, start_time, end_time);
        }
    }

    /// Issue a heartbeat with energy data.
    pub fn heartbeat_pow(&mut self,
                         tag: u64,
                         work: u64,
                         start_time: u64,
                         end_time: u64,
                         start_energy: u64,
                         end_energy: u64) {
        unsafe {
            heartbeat::heartbeat_pow(&mut self.hb, tag, work, start_time, end_time, start_energy, end_energy)
        }
    }

    #[cfg(unix)]
    pub fn log_window_buffer(&self, log: File, print_header: bool) -> Result<(), &'static str> {
        let ph: i32 = match print_header {
            true => 1,
            false => 0,
        };
        unsafe {
            match heartbeat::heartbeat_log_window_buffer(&self.hb, log.as_raw_fd(), ph) {
                0 => Ok(()),
                _ => Err("Error logging window buffer"),
            }
        }
    }

    /// Utility function to get the most recent user-specified tag
    pub fn get_tag(&self) -> u64 {
        unsafe {
            heartbeat::hb_get_user_tag(&self.hb)
        }
    }

    /// Utility function to get the current window performance.
    pub fn get_window_rate(&self) -> f64 {
        unsafe {
            heartbeat::hb_get_window_rate(&self.hb)
        }
    }

    /// Utility function to get the current window power.
    pub fn get_window_pwr(&self) -> f64 {
        unsafe {
            heartbeat::hb_get_window_power(&self.hb)
        }
    }
}

#[cfg(test)]
mod test {
    use super::HeartbeatSimple;

    #[test]
    fn test_simple() {
        const TIME_INC: u64 = 1000000000;
        const ENERGY_INC: u64 = 1000000;
        let mut hb = HeartbeatSimple::new(5, None).unwrap();
        let mut start_time: u64 = 0;
        let mut end_time: u64 = TIME_INC;
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