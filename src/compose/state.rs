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

use std::mem;
use std::ffi::CStr;

use libc::size_t;
use ffi::*;
use Keysym;
use compose::{Table, Result, Status};

pub struct State(*mut xkb_compose_state);

impl State {
	#[inline]
	pub unsafe fn from_ptr(ptr: *mut xkb_compose_state) -> Self {
		State(ptr)
	}

	#[inline]
	pub unsafe fn as_ptr(&self) -> *mut xkb_compose_state {
		self.0
	}

	#[inline]
	pub fn table(&self) -> Table {
		unsafe {
			Table::from_ptr(xkb_compose_state_get_compose_table(self.0))
		}
	}

	#[inline]
	pub fn feed<T: Into<Keysym>>(&mut self, sym: T) -> Result {
		unsafe {
			mem::transmute(xkb_compose_state_feed(self.0, sym.into().into()))
		}
	}

	#[inline]
	pub fn reset(&mut self) {
		unsafe {
			xkb_compose_state_reset(self.0)
		}
	}

	#[inline]
	pub fn status(&self) -> Status {
		unsafe {
			mem::transmute(xkb_compose_state_get_status(self.0))
		}
	}

	#[inline]
	pub fn utf8(&self) -> Option<String> {
		unsafe {
			let mut buffer = [0; 64];
			
			match xkb_compose_state_get_utf8(self.0, buffer.as_mut_ptr(), buffer.len() as size_t) {
				-1 => unreachable!(),
				0  => None,
				_  => Some(CStr::from_ptr(buffer.as_ptr()).to_str().unwrap().into()),
			}
		}
	}

	#[inline]
	pub fn sym(&self) -> Option<Keysym> {
		unsafe {
			match xkb_compose_state_get_one_sym(self.0) {
				XKB_KEY_NoSymbol => None,
				sym              => Some(sym.into()),
			}
		}
	}
}

impl Clone for State {
	#[inline]
	fn clone(&self) -> Self {
		unsafe {
			State(xkb_compose_table_ref(self.0))
		}
	}
}

impl Drop for State {
	#[inline]
	fn drop(&mut self) {
		unsafe {
			xkb_compose_state_unref(self.0)
		}
	}
}

bitflags! {
	pub flags Flags: xkb_compose_state_flags {
		const NO_FLAGS = XKB_COMPOSE_STATE_NO_FLAGS,
	}
}

impl Default for Flags {
	fn default() -> Self {
		NO_FLAGS
	}
}
