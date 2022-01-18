//#![no_std]

#[allow(dead_code)]
pub mod c {
	extern {
		// msleep must be linked in C, arm32
		pub fn _msleep(ms: u32);

		pub fn printf(fmt: *const u8, ...);
		pub fn bmp_printf(id: u32, x: u32, y: u32, fmt: *const u8, ...);
		pub fn bmp_putpixel(x: u32, y: u32, col: u8);

		// TODO: make this less hacky
		pub fn add_menu();

		pub static rFONT_MED: u32;
	}
}

#[allow(dead_code)]
pub fn msleep(ms: u32) {
	unsafe {
		c::_msleep(ms);
	}
}

// How to pass varargs to C???
#[allow(dead_code)]
pub fn printf(fmt: &str) {
	unsafe {
		c::printf(fmt.as_ptr());
	}
}

// TODO: rust enums
#[allow(dead_code)]
pub fn bmp_printf(_id: u32, x: u32, y: u32, fmt: &str) {
	unsafe {
		let cid: u32 = c::rFONT_MED;
		c::bmp_printf(cid, x, y, fmt.as_ptr());
	}
}

#[allow(dead_code)]
pub fn putpixel(x: u32, y: u32, col: u8) {
	unsafe {
		c::bmp_putpixel(x, y, col);
	}
}
