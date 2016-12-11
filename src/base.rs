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

macro_rules! define {
	($name:ident, $base:ident) => (
		#[derive(Eq, PartialEq, Copy, Clone, Debug)]
		pub struct $name(pub $base);

		impl From<$base> for $name {
			#[inline]
			fn from(value: $base) -> $name {
				$name(value)
			}
		}

		impl Into<$base> for $name {
			#[inline]
			fn into(self) -> $base {
				self.0
			}
		}
	);
}

define!(Keycode, xkb_keycode_t);

define!(LayoutIndex, xkb_layout_index_t);
define!(LayoutMask, xkb_layout_mask_t);

define!(LevelIndex, xkb_level_index_t);

define!(ModIndex, xkb_mod_index_t);
define!(ModMask, xkb_mod_mask_t);

define!(LedIndex, xkb_led_index_t);
define!(LedMask, xkb_led_mask_t);

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum LogLevel {
	Critical = 10,
	Error    = 20,
	Warning  = 30,
	Info     = 40,
	Debug    = 50,
}
