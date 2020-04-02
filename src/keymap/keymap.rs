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

use ffi::*;
use std::ffi::CString;
use crate::Keycode;
use crate::keymap::{Mods, Layouts, Leds, Key};
use crate::context::Context;
use crate::state::State;
use std::str::FromStr;

#[derive(Debug)]
pub struct Keymap(*mut xkb_keymap);

impl FromStr for Keymap{
	type Err = ();

	fn from_str(s: &str) -> Result<Keymap, Self::Err>{
		let context = Context::default();
		Keymap::from_str(&context, s)
	}
}

impl Keymap {
	pub fn from_str(context: &Context, s: &str) -> Result<Self, ()> {
		unsafe{
			let mut v = s.as_bytes().to_vec();
			v.push('\0' as u8);

			let cstring = CString::from_vec_unchecked(v);
			let ptr = xkb_keymap_new_from_string(context.as_ptr(), cstring.as_ptr() as *const _ as *const std::os::raw::c_char, xkb_keymap_format::XKB_KEYMAP_FORMAT_TEXT_v1, 0);

			if ptr.is_null(){
				Err(())
			}
			else{
				Ok(Self::from_ptr(ptr))
			}
		}
	}

	pub unsafe fn from_ptr(ptr: *mut xkb_keymap) -> Self {
		Keymap(ptr)
	}

	pub unsafe fn as_ptr(&self) -> *mut xkb_keymap {
		self.0
	}

	pub fn mods(&self) -> Mods {
		Mods(self)
	}

	pub fn layouts(&self) -> Layouts {
		Layouts(self)
	}

	pub fn leds(&self) -> Leds {
		Leds(self)
	}

	pub fn key(&self, key: Keycode) -> Key {
		Key(self, key)
	}

	pub fn state(&self) -> State {
		unsafe {
			State::from_ptr(xkb_state_new(self.0))
		}
	}
}

impl Clone for Keymap {
	fn clone(&self) -> Self {
		unsafe {
			Keymap(xkb_keymap_ref(self.0))
		}
	}
}

impl Drop for Keymap {
	fn drop(&mut self) {
		unsafe {
			xkb_keymap_unref(self.0);
		}
	}
}
