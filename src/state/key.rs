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

use std::ptr;
use std::mem;
use std::ffi::CStr;

use libc::size_t;
use ffi::*;
use crate::{State, Keycode, Keysym};
use crate::{LayoutIndex, LevelIndex, ModMask};
use crate::state::Consumed;

#[derive(Debug)]
pub struct Key<'a>(pub &'a State, pub Keycode);

impl<'a> Key<'a> {
	#[inline]
	pub fn layout(&self) -> Option<usize> {
		unsafe {
			match xkb_state_key_get_layout(self.0.as_ptr(), self.1.into()) {
				XKB_LAYOUT_INVALID =>
					None,

				value =>
					Some(value as usize)
			}
		}
	}

	#[inline]
	pub fn level(&self, layout: LayoutIndex) -> Option<LevelIndex> {
		unsafe {
			match xkb_state_key_get_level(self.0.as_ptr(), self.1.into(), layout.into()) {
				XKB_LEVEL_INVALID =>
					None,

				value =>
					Some(value.into()),
			}
		}
	}

	#[inline]
	pub fn consumed(&self, mode: Consumed) -> ModMask {
		unsafe {
			xkb_state_key_get_consumed_mods2(self.0.as_ptr(), self.1.into(), mem::transmute(mode)).into()
		}
	}

	#[inline]
	pub fn utf8(&self) -> Option<String> {
		unsafe {
			let mut buffer = [0; 64];
			
			match xkb_state_key_get_utf8(self.0.as_ptr(), self.1.into(), buffer.as_mut_ptr(), buffer.len() as size_t) {
				-1 => unreachable!(),
				0  => None,
				_  => Some(CStr::from_ptr(buffer.as_ptr()).to_str().unwrap().into()),
			}
		}
	}

	#[inline]
	pub fn utf32(&self) -> Option<u32> {
		unsafe {
			match xkb_state_key_get_utf32(self.0.as_ptr(), self.1.into()) {
				0 => None,
				n => Some(n),
			}
		}
	}

	#[inline]
	pub fn sym(&self) -> Option<Keysym> {
		unsafe {
			match xkb_state_key_get_one_sym(self.0.as_ptr(), self.1.into()) {
				XKB_KEY_NoSymbol =>
					None,

				value =>
					Some(Keysym::from(value))
			}
		}
	}

	#[inline]
	pub fn syms(&self) -> Vec<Keysym> {
		unsafe {
			let mut syms = ptr::null_mut();
			let     len  = xkb_state_key_get_syms(self.0.as_ptr(), self.1.into(), &mut syms);

			let mut result = Vec::with_capacity(len as usize);
			for i in 0 .. len {
				result.push(Keysym::from(*syms.offset(i as isize)));
			}

			result
		}
	}
}
