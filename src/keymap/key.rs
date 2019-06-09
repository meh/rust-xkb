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

use ffi::*;
use crate::{Keymap, Keycode, Keysym};
use crate::{LayoutIndex, LevelIndex};

#[derive(Debug)]
pub struct Key<'a>(pub &'a Keymap, pub Keycode);

impl<'a> Key<'a> {
	pub fn layouts(&self) -> usize {
		unsafe {
			xkb_keymap_num_layouts_for_key(self.0.as_ptr(), self.1.into()) as usize
		}
	}

	pub fn levels(&self, layout: LayoutIndex) -> usize {
		unsafe {
			xkb_keymap_num_levels_for_key(self.0.as_ptr(), self.1.into(), layout.into()) as usize
		}
	}

	pub fn repeats(&self) -> bool {
		unsafe {
			xkb_keymap_key_repeats(self.0.as_ptr(), self.1.into()) != 0
		}
	}

	pub fn syms(&self, layout: LayoutIndex, level: LevelIndex) -> Vec<Keysym> {
		unsafe {
			let mut syms = ptr::null_mut();
			let     len  = xkb_keymap_key_get_syms_by_level(self.0.as_ptr(), self.1.into(),
				layout.into(), level.into(), &mut syms);

			let mut result = Vec::with_capacity(len as usize);
			for i in 0 .. len {
				result.push(Keysym::from(*syms.offset(i as isize)));
			}

			result
		}
	}
}
