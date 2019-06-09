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

use xcb;
use ffi::*;
use crate::{Keycode, ModMask, LayoutMask};
use crate::{Context, Keymap, State};
use crate::keymap::compile;

pub const MIN_MAJOR_XKB_VERSION: u16 = XKB_X11_MIN_MAJOR_XKB_VERSION as u16;
pub const MIN_MINOR_XKB_VERSION: u16 = XKB_X11_MIN_MINOR_XKB_VERSION as u16;

impl From<u8> for Keycode {
	fn from(value: u8) -> Self {
		Keycode(value as xkb_keycode_t)
	}
}

impl From<u8> for ModMask {
	fn from(value: u8) -> Self {
		ModMask(value as xkb_mod_mask_t)
	}
}

impl From<i16> for ModMask {
	fn from(value: i16) -> Self {
		ModMask(value as xkb_mod_mask_t)
	}
}

impl From<u8> for LayoutMask {
	fn from(value: u8) -> Self {
		LayoutMask(value as xkb_layout_mask_t)
	}
}

impl From<i16> for LayoutMask {
	fn from(value: i16) -> Self {
		LayoutMask(value as xkb_layout_mask_t)
	}
}

#[inline]
pub fn device(connection: &xcb::Connection) -> Result<i32, ()> {
	unsafe {
		match xkb_x11_get_core_keyboard_device_id(connection.get_raw_conn() as *mut _) {
			-1 => Err(()),
			id => Ok(id)
		}
	}
}

#[inline]
pub fn keymap(connection: &xcb::Connection, device: i32, context: &Context, flags: compile::Flags) -> Result<Keymap, ()> {
	unsafe {
		xkb_x11_keymap_new_from_device(context.as_ptr(), connection.get_raw_conn() as *mut _, device, flags.bits())
			.as_mut().map(|ptr| Keymap::from_ptr(ptr)).ok_or(())
	}
}

#[inline]
pub fn state(connection: &xcb::Connection, device: i32, keymap: &Keymap) -> Result<State, ()> {
	unsafe {
		xkb_x11_state_new_from_device(keymap.as_ptr(), connection.get_raw_conn() as *mut _, device)
			.as_mut().map(|ptr| State::from_ptr(ptr)).ok_or(())
	}
}
