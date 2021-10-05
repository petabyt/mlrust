#![no_std]

extern {
	fn printf(fmt: *const u8, ...);
	fn bmp_printf(id: u32, x: u32, y: u32, fmt: *const u8, ...);

	static rFONT_MED: u32;
}

#[no_mangle]
pub fn main() {
	loop {
		unsafe {
			bmp_printf(
				rFONT_MED,
				50,
				50,
				"Hello Rust World\n".as_ptr() as *const u8
			);
		}
	}
}
