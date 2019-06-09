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
use crate::{Keymap, Keycode};
use crate::state::{Key, Serialize, Update, Mods, Layouts, Leds};

#[derive(Debug)]
pub struct State(*mut xkb_state);

impl State {
	#[inline]
	pub unsafe fn from_ptr(ptr: *mut xkb_state) -> Self {
		State(ptr)
	}

	#[inline]
	pub unsafe fn as_ptr(&self) -> *mut xkb_state {
		self.0
	}

	#[inline]
	pub fn keymap(&self) -> Keymap {
		unsafe {
			Keymap::from_ptr(xkb_state_get_keymap(self.0))
		}
	}

	#[inline]
	pub fn key<T: Into<Keycode>>(&self, code: T) -> Key {
		Key(self, code.into())
	}

	#[inline]
	pub fn serialize(&mut self) -> Serialize {
		Serialize(self)
	}

	#[inline]
	pub fn update(&mut self) -> Update {
		Update(self)
	}

	#[inline]
	pub fn mods(&self) -> Mods {
		Mods(self)
	}

	#[inline]
	pub fn layouts(&self) -> Layouts {
		Layouts(self)
	}

	#[inline]
	pub fn leds(&self) -> Leds {
		Leds(self)
	}
}

impl Clone for State {
	#[inline]
	fn clone(&self) -> Self {
		unsafe {
			State(xkb_state_ref(self.0))
		}
	}
}

impl Drop for State {
	#[inline]
	fn drop(&mut self) {
		unsafe {
			xkb_state_unref(self.0);
		}
	}
}
