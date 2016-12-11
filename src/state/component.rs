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

bitflags! {
	pub flags Components: xkb_state_component {
    const MODS_DEPRESSED   = XKB_STATE_MODS_DEPRESSED,
    const MODS_LATCHED     = XKB_STATE_MODS_LATCHED,
    const MODS_LOCKED      = XKB_STATE_MODS_LOCKED,
    const MODS_EFFECTIVE   = XKB_STATE_MODS_EFFECTIVE,
    const LAYOUT_DEPRESSED = XKB_STATE_LAYOUT_DEPRESSED,
    const LAYOUT_LATCHED   = XKB_STATE_LAYOUT_LATCHED,
    const LAYOUT_LOCKED    = XKB_STATE_LAYOUT_LOCKED,
    const LAYOUT_EFFECTIVE = XKB_STATE_LAYOUT_EFFECTIVE,
    const LEDS             = XKB_STATE_LEDS,
	}
}
