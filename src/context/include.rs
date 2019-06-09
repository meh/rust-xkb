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

use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use ffi::*;
use crate::context::Context;

#[derive(Debug)]
pub struct Include<'a>(pub &'a mut Context);

impl<'a> Include<'a> {
	pub fn append<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
		unsafe {
			xkb_context_include_path_append(self.0.as_ptr(), path.as_ref().as_os_str().as_bytes().as_ptr() as *const _);
		}

		self
	}

	pub fn default(&mut self) -> &mut Self {
		unsafe {
			xkb_context_include_path_append_default(self.0.as_ptr());
		}

		self
	}

	pub fn reset(&mut self) -> &mut Self {
		unsafe {
			xkb_context_include_path_reset_defaults(self.0.as_ptr());
		}

		self
	}

	pub fn clear(&mut self) -> &mut Self {
		unsafe {
			xkb_context_include_path_clear(self.0.as_ptr());
		}

		self
	}

	pub fn len(&self) -> usize {
		unsafe {
			xkb_context_num_include_paths(self.0.as_ptr()) as usize
		}
	}
}

// TODO: Iterator<Item = &Path>
// TODO: Index<usize>
