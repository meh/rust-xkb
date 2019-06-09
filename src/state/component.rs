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
	pub struct Components: xkb_state_component {
		const MODS_DEPRESSED   = XKB_STATE_MODS_DEPRESSED;
		const MODS_LATCHED     = XKB_STATE_MODS_LATCHED;
		const MODS_LOCKED      = XKB_STATE_MODS_LOCKED;
		const MODS_EFFECTIVE   = XKB_STATE_MODS_EFFECTIVE;
		const LAYOUT_DEPRESSED = XKB_STATE_LAYOUT_DEPRESSED;
		const LAYOUT_LATCHED   = XKB_STATE_LAYOUT_LATCHED;
		const LAYOUT_LOCKED    = XKB_STATE_LAYOUT_LOCKED;
		const LAYOUT_EFFECTIVE = XKB_STATE_LAYOUT_EFFECTIVE;
		const LEDS             = XKB_STATE_LEDS;
	}
}

pub const MODS_DEPRESSED: Components   = Components::MODS_DEPRESSED;
pub const MODS_LATCHED: Components     = Components::MODS_LATCHED;
pub const MODS_LOCKED: Components      = Components::MODS_LOCKED;
pub const MODS_EFFECTIVE: Components   = Components::MODS_EFFECTIVE;
pub const LAYOUT_DEPRESSED: Components = Components::LAYOUT_DEPRESSED;
pub const LAYOUT_LATCHED: Components   = Components::LAYOUT_LATCHED;
pub const LAYOUT_LOCKED: Components    = Components::LAYOUT_LOCKED;
pub const LAYOUT_EFFECTIVE: Components = Components::LAYOUT_EFFECTIVE;
pub const LEDS: Components             = Components::LEDS;
