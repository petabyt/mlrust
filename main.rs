#![no_std]

extern {
	fn msleep(ms: u32);
	fn printf(fmt: *const u8, ...);
}

#[no_mangle]
pub fn mlrust_task() {
	unsafe {
		printf("Hello Rust World\n".as_ptr() as *const u8);
	}
}
