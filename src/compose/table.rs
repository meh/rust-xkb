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
use std::ffi::CString;

use libc::size_t;
use ffi::*;
use Context;
use compose::{compile, Format};
use compose::state::{self, State};

pub struct Table(*mut xkb_compose_table);

impl Table {
	#[inline]
	pub unsafe fn from_ptr(ptr: *mut xkb_compose_table) -> Self {
		Table(ptr)
	}

	#[inline]
	pub unsafe fn as_ptr(&self) -> *mut xkb_compose_table {
		self.0
	}

	#[inline]
	pub fn new<L: AsRef<str>>(context: &Context, locale: L, flags: compile::Flags) -> Result<Self, ()> {
		unsafe {
			let locale = CString::new(locale.as_ref()).unwrap();

			xkb_compose_table_new_from_locale(context.as_ptr(),
				locale.as_ptr(), flags.bits())
					.as_mut().map(|ptr| Table(ptr)).ok_or(())
		}
	}

	#[inline]
	pub fn from_buffer<L: AsRef<str>, B: AsRef<[u8]>>(context: &Context, locale: L, buffer: B, format: Format, flags: compile::Flags) -> Result<Self, ()> {
		unsafe {
			let buffer = buffer.as_ref();
			let locale = CString::new(locale.as_ref()).unwrap();

			xkb_compose_table_new_from_buffer(context.as_ptr(),
				buffer.as_ptr() as *const _, buffer.len() as size_t,
				locale.as_ptr(), mem::transmute(format), flags.bits())
					.as_mut().map(|ptr| Table(ptr)).ok_or(())
		}
	}

	#[inline]
	pub fn state(&self, flags: state::Flags) -> State {
		unsafe {
			State::from_ptr(xkb_compose_state_new(self.0, flags.bits()))
		}
	}
}

impl Clone for Table {
	#[inline]
	fn clone(&self) -> Self {
		unsafe {
			Table(xkb_compose_table_ref(self.0))
		}
	}
}

impl Drop for Table {
	#[inline]
	fn drop(&mut self) {
		unsafe {
			xkb_compose_table_unref(self.0)
		}
	}
}
