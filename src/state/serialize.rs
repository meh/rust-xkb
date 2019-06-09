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
use crate::{ModMask, LayoutIndex};
use crate::state::{State, Components};

#[derive(Debug)]
pub struct Serialize<'a>(pub &'a mut State);

impl<'a> Serialize<'a> {
	#[inline]
	pub fn mods(&mut self, components: Components) -> ModMask {
		unsafe {
			xkb_state_serialize_mods(self.0.as_ptr(), components.bits()).into()
		}
	}

	#[inline]
	pub fn layouts(&mut self, components: Components) -> LayoutIndex {
		unsafe {
			xkb_state_serialize_layout(self.0.as_ptr(), components.bits()).into()
		}
	}
}
