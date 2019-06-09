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
use crate::context::{Flags, Include, Log};

#[derive(Debug)]
pub struct Context(*mut xkb_context);

impl Default for Context {
	#[inline]
	fn default() -> Self {
		Context::new(Default::default())
	}
}

impl Context {
	#[inline]
	pub unsafe fn from_ptr(ptr: *mut xkb_context) -> Self {
		Context(ptr)
	}

	#[inline]
	pub unsafe fn as_ptr(&self) -> *mut xkb_context {
		self.0
	}

	#[inline]
	pub fn new(flags: Flags) -> Self {
		unsafe {
			Context(xkb_context_new(flags.bits()))
		}
	}

	#[inline]
	pub fn include(&mut self) -> Include {
		Include(self)
	}

	#[inline]
	pub fn log(&mut self) -> Log {
		Log(self)
	}
}

impl Clone for Context {
	#[inline]
	fn clone(&self) -> Self {
		unsafe {
			Context(xkb_context_ref(self.0))
		}
	}
}

impl Drop for Context {
	#[inline]
	fn drop(&mut self) {
		unsafe {
			xkb_context_unref(self.0);
		}
	}
}
