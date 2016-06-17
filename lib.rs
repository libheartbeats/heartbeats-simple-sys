extern crate libc;

mod hbs_common_sys;
mod hbs_sys;
mod hbs_acc_sys;
mod hbs_pow_sys;
mod hbs_acc_pow_sys;

pub use hbs_common_sys::*;
pub use hbs_sys::*;
pub use hbs_acc_sys::*;
pub use hbs_pow_sys::*;
pub use hbs_acc_pow_sys::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        // just tests that library is linked properly
        unsafe {
            hb_log_header(0);
            hb_acc_log_header(0);
            hb_pow_log_header(0);
            hb_acc_pow_log_header(0);
        }
        return;
    }
}
