#![no_std]

use core::panic::PanicInfo;

extern {
	fn printf(fmt: *const u8, ...);
	fn bmp_printf(id: u32, x: u32, y: u32, fmt: *const u8, ...);

	static rFONT_MED: u32;
}

// Custom rust panic function, see patch.py
#[no_mangle]
fn panic() {
	loop {}
}

const MAX: u32 = 400;
const MIN: u32 = 50;

#[no_mangle]
pub fn main() {
	let mut x = MIN;
	let mut right = true;
	loop {
		unsafe {
			bmp_printf(
				rFONT_MED,
				x,
				50,
				"Hello Rust World\n".as_ptr() as *const u8
			);

			if (right) {
				x += 1;
			} else {
				x -= 1;
			}

			if (x == MAX || x == MIN) {
				right = !right;
			}
		}
	}
}
