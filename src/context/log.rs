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

use libc::c_int;
use ffi::*;
use LogLevel;
use context::Context;

#[derive(Debug)]
pub struct Log<'a>(pub &'a mut Context);

impl<'a> Log<'a> {
	pub fn level(&mut self, level: LogLevel) -> &mut Self {
		unsafe {
			xkb_context_set_log_level(self.0.as_ptr(), mem::transmute(level));
		}

		self
	}

	pub fn verbosity(&mut self, verbosity: i32) -> &mut Self {
		unsafe {
			xkb_context_set_log_verbosity(self.0.as_ptr(), verbosity as c_int);
		}

		self
	}
}
