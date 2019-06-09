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

use ffi::*;
use crate::{key, Keycode, ModMask, LayoutMask};
use crate::state::{State, Components};

#[derive(Debug)]
pub struct Update<'a>(pub &'a mut State);

impl<'a> Update<'a> {
	#[inline]
	pub fn key(&mut self, key: Keycode, direction: key::Direction) -> Components {
		unsafe {
			Components::from_bits(xkb_state_update_key(self.0.as_ptr(),
				key.into(), mem::transmute(direction))).unwrap()
		}
	}

	#[inline]
	pub fn mask<M1, M2, M3, L1, L2, L3>(&mut self, depressed_mods: M1, latched_mods: M2, locked_mods: M3, depressed_layout: L1, latched_layout: L2, locked_layout: L3) -> Components
		where M1: Into<ModMask>, M2: Into<ModMask>, M3: Into<ModMask>,
		      L1: Into<LayoutMask>, L2: Into<LayoutMask>, L3: Into<LayoutMask>
	{
		unsafe {
			Components::from_bits(xkb_state_update_mask(self.0.as_ptr(),
				depressed_mods.into().into(), latched_mods.into().into(), locked_mods.into().into(),
				depressed_layout.into().into(), latched_layout.into().into(), locked_layout.into().into())).unwrap()
		}
	}
}
