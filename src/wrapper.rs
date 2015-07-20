use pow::*;
use std::mem;
use std::io::{self, Write};
use std::fs::File;

/// Contains the Heartbeat and its window data buffer.
pub struct HeartbeatPow {
    pub hb: heartbeat_pow_context,
    pub hbr: Vec<heartbeat_pow_record>,
    pub log: Option<File>,
}

impl HeartbeatPow {
    /// Allocate and initialize a new `Heartbeat`.
    pub fn new(window_size: usize,
               hwc_callback: Option<heartbeat_pow_window_complete>,
               mut log: Option<File>) -> Result<HeartbeatPow, &'static str> {
        let mut hbr: Vec<heartbeat_pow_record> = Vec::with_capacity(window_size);
        unsafe {
            // must explicitly set size so we can read data later
            // (Rust isn't aware of native code modifying the buffer)
            hbr.set_len(window_size);
            let mut hb = mem::uninitialized();
            match heartbeat_pow_init(&mut hb,
                                     hbr.capacity() as u64,
                                     hbr.as_mut_ptr(),
                                     hwc_callback) {
                0 => {
                    if let Some(ref mut l) = log {
                        l.write_all("HB    Tag    Work    Start_Time    End_time    \
                            Global_Perf    Window_Perf    Instant_Perf    \
                            Start_Energy    End_Energy    \
                            Global_Pwr    Window_Pwr    Instant_Pwr\n".as_bytes()).unwrap()
                    }
                    Ok(HeartbeatPow { hb: hb, hbr: hbr, log: log, })
                },
                _ => Err("Failed to initialize heartbeat")
            }
        }
    }

    /// Issue a heartbeat
    pub fn heartbeat(&mut self,
                     tag: u64,
                     work: u64,
                     start_time: u64,
                     end_time: u64,
                     start_energy: u64,
                     end_energy: u64) {
        unsafe {
            heartbeat_pow(&mut self.hb, tag, work, start_time, end_time, start_energy, end_energy)
        }
    }

    fn write_log(r: &heartbeat_pow_record, l: &mut File) -> io::Result<usize> {
        l.write(format!("{}    {}    {}    {}    {}    \
                         {}    {}    {}    \
                         {}    {}    \
                         {}    {}    {}\n",
                        r.id, r.user_tag, r.work, r.start_time, r.end_time,
                        r.perf.global, r.perf.window, r.perf.instant,
                        r.start_energy, r.end_energy,
                        r.pwr.global, r.pwr.window, r.pwr.instant).as_bytes())
    }

    /// Rust-only function that logs the buffer (up to buffer_index) to a file.
    pub fn log_to_buffer_index(&mut self) -> io::Result<()> {
        match self.log {
            Some(ref mut l) => {
                for i in 0..self.hb.ws.buffer_index {
                    match HeartbeatPow::write_log(self.hbr.get(i as usize).unwrap(), l) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                }
                l.flush()
            }
            None => Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::HeartbeatPow;
    use std::fs::File;

    #[test]
    fn test_simple() {
        const TIME_INC: u64 = 1000000000;
        const ENERGY_INC: u64 = 1000000;
        let mut hb = HeartbeatPow::new(5, None, None).unwrap();
        let mut start_time: u64 = 0;
        let mut end_time: u64 = TIME_INC;
        let mut start_energy: u64 = 0;
        let mut end_energy: u64 = ENERGY_INC;
        for tag in 0..10 {
            hb.heartbeat(tag, 1, start_time, end_time, start_energy, end_energy);
            start_time = end_time;
            end_time += TIME_INC;
            start_energy = end_energy;
            end_energy += ENERGY_INC;
        }
    }

    #[test]
    fn test_file() {
        let mut hb = HeartbeatPow::new(5, None, Some(File::create("foo.log").unwrap())).unwrap();
        hb.heartbeat(0, 1, 0, 1, 0, 0);
        hb.log_to_buffer_index().unwrap();
    }
}
