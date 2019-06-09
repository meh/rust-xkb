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
use bitflags::bitflags;

bitflags! {
	pub struct Flags: xkb_context_flags {
		const NO_FLAGS             = XKB_CONTEXT_NO_FLAGS;
		const NO_DEFAULT_INCLUDES  = XKB_CONTEXT_NO_DEFAULT_INCLUDES;
		const NO_ENVIRONMENT_NAMES = XKB_CONTEXT_NO_ENVIRONMENT_NAMES;
	}
}

pub const NO_FLAGS: Flags = Flags::NO_FLAGS;
pub const NO_DEFAULT_INCLUDES: Flags = Flags::NO_DEFAULT_INCLUDES;
pub const NO_ENVIRONMENT_NAMES: Flags = Flags::NO_ENVIRONMENT_NAMES;

impl Default for Flags {
	fn default() -> Self {
		NO_FLAGS
	}
}
