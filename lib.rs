extern crate hbs_common_sys;
extern crate hbs_sys;
extern crate hbs_acc_sys;
extern crate hbs_pow_sys;
extern crate hbs_acc_pow_sys;

pub use hbs_common_sys::*;
pub use hbs_sys::*;
pub use hbs_acc_sys::*;
pub use hbs_pow_sys::*;
pub use hbs_acc_pow_sys::*;

#[cfg(test)]
mod test {
	use hbs_sys::*;
	use hbs_acc_sys::*;
	use hbs_pow_sys::*;
	use hbs_acc_pow_sys::*;
	#[test]
	fn test() {
		// just tests that libraries are linked properly
		unsafe {
			hb_log_header(0);
			hb_acc_log_header(0);
			hb_pow_log_header(0);
			hb_acc_pow_log_header(0);
		}
		return;
	}
}