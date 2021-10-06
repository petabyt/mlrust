#![no_std]

#[allow(dead_code)]
mod c {
	extern {
		// msleep must be linked in C, arm32
		pub fn _msleep(ms: u32);

		pub fn printf(fmt: *const u8, ...);
		pub fn bmp_printf(id: u32, x: u32, y: u32, fmt: *const u8, ...);
		pub fn add_menu();

		pub static rFONT_MED: u32;
	}
}

#[allow(dead_code)]
mod io {
	use c;
	pub fn msleep(ms: u32) {
		unsafe {
			c::_msleep(ms);
		}
	}

	// How to pass varargs to C???
	pub fn printf(fmt: &str) {
		unsafe {
			c::printf(fmt.as_ptr());
		}
	}
}

#[allow(dead_code)]
mod bmp {
	use c;

	// TODO: rust enums
	pub fn printf(_id: u32, x: u32, y: u32, fmt: &str) {
		unsafe {
			let cid: u32 = c::rFONT_MED;
			c::bmp_printf(cid, x, y, fmt.as_ptr());
		}
	}
}

// Custom rust panic function, see patch.py
#[no_mangle]
fn panic() {
	bmp::printf(0, 10, 10, "The Rust Has Panicked");

	loop {}
}

#[no_mangle]
pub fn mlrust_task() {
	let mut x = 50;
	let mut right = true;
	loop {
		bmp::printf(0, x, 50, "Hello Rusty World");

		if right {
			x += 1;
		} else {
			x -= 1;
		}

		if x == 540 || x == 50 {
			right = !right;
		}

		io::msleep(10);
	}
}

#[no_mangle]
pub fn mlrust_deinit() -> u32{
	return 0;
}

#[no_mangle]
pub fn mlrust_init() -> u32 {
	unsafe {
		c::add_menu();
	}

	return 0;
}
