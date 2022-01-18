#![no_std]
mod ml;

// Custom rust panic function, see patch.py
#[no_mangle]
fn panic() {
	ml::bmp_printf(0, 10, 10, "The Rust Has Panicked");
	loop {}
}

#[no_mangle]
pub fn mlrust_task() {	
	loop {
		ml::bmp_printf(0, 10, 10, "You are viewing a Rust program");
	}
}

#[no_mangle]
pub fn mlrust_deinit() -> u32{
	return 0;
}

#[no_mangle]
pub fn mlrust_init() -> u32 {
	unsafe {
		ml::c::add_menu();
	}

	return 0;
}
