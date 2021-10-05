#![no_std]

extern {
	fn msleep(ms: u32);
	fn printf(fmt: *const u8, ...);
	fn bmp_printf(id: u32, x: u32, y: u32, fmt: *const u8, ...);
	fn add_menu();

	static rFONT_MED: u32;
}

// Custom rust panic function, see patch.py
#[no_mangle]
fn panic() {
	loop {}
}

#[no_mangle]
pub fn mlrust_task() {
	let mut x = 50;
	let mut right = true;
	loop {
		unsafe {
			bmp_printf(
				rFONT_MED,
				x,
				50,
				"Hello Rust World\n\0\0".as_ptr()
			);
		}

		if right {
			x += 1;
		} else {
			x -= 1;
		}

		if x == 540 || x == 50 {
			right = !right;
		}	
	}
}

#[no_mangle]
pub fn mlrust_deinit() -> u32{
	return 0;
}

#[no_mangle]
pub fn mlrust_init() -> u32 {
	unsafe {
		add_menu();
	}

	return 0;
}
