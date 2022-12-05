//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

pub mod mods {
	use ffi::*;
	use libc::c_char;

	pub const SHIFT: *const c_char = XKB_MOD_NAME_SHIFT.as_ptr().cast::<c_char>();
	pub const CAPS:  *const c_char = XKB_MOD_NAME_CAPS.as_ptr().cast::<c_char>();
	pub const CTRL:  *const c_char = XKB_MOD_NAME_CTRL.as_ptr().cast::<c_char>();
	pub const ALT:   *const c_char = XKB_MOD_NAME_ALT.as_ptr().cast::<c_char>();
	pub const NUM:   *const c_char = XKB_MOD_NAME_NUM.as_ptr().cast::<c_char>();
	pub const LOGO:  *const c_char = XKB_MOD_NAME_LOGO.as_ptr().cast::<c_char>();
}

pub mod leds {
	use ffi::*;
	use libc::c_char;

	pub const CAPS:   *const c_char = XKB_LED_NAME_CAPS.as_ptr().cast::<c_char>();
	pub const NUM:    *const c_char = XKB_LED_NAME_NUM.as_ptr().cast::<c_char>();
	pub const SCROLL: *const c_char = XKB_LED_NAME_SCROLL.as_ptr().cast::<c_char>();
}
