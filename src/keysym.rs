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

use std::fmt;
use std::str;
use std::ffi::{CStr, CString};

use libc::{size_t};
use ffi::*;

define!(Keysym, xkb_keysym_t);

impl Keysym {
	#[inline]
	pub fn utf8(&self) -> String {
		unsafe {
			let mut buffer = [0; 64];
			
			match xkb_keysym_to_utf8(self.0, buffer.as_mut_ptr(), buffer.len() as size_t) {
				-1 => unreachable!(),
				0  => "".into(),
				_  => CStr::from_ptr(buffer.as_ptr()).to_str().unwrap().into(),
			}
		}
	}

	#[inline]
	pub fn utf32(&self) -> u32 {
		unsafe {
			xkb_keysym_to_utf32(self.0)
		}
	}
}

impl fmt::Display for Keysym {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		unsafe {
			let mut buffer = [0; 64];

			if xkb_keysym_get_name(self.0, buffer.as_mut_ptr(), buffer.len() as size_t) < 0 {
				return Err(fmt::Error);
			}

			f.write_str(CStr::from_ptr(buffer.as_ptr()).to_str().ok().ok_or(fmt::Error)?)
		}
	}
}

impl str::FromStr for Keysym {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		unsafe {
			let name = CString::new(s).ok().ok_or(())?;

			match xkb_keysym_from_name(name.as_ptr(), XKB_KEYSYM_NO_FLAGS) {
				XKB_KEY_NoSymbol =>
					match xkb_keysym_from_name(name.as_ptr(), XKB_KEYSYM_CASE_INSENSITIVE) {
						XKB_KEY_NoSymbol =>
							Err(()),

						value =>
							Ok(Keysym(value)),
					},

				value =>
					Ok(Keysym(value))
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use ffi::*;
	use super::*;

	#[test]
	fn parse() {
		assert_eq!(Keysym::from(XKB_KEY_a), "a".parse().unwrap());
		assert_eq!(Keysym::from(XKB_KEY_A), "A".parse().unwrap());

		assert_eq!(Keysym::from(XKB_KEY_Sys_Req), "Sys_Req".parse().unwrap());
		assert_eq!(Keysym::from(XKB_KEY_Sys_Req), "sys_req".parse().unwrap());
	}

	#[test]
	fn display() {
		assert_eq!("a", Keysym::from(XKB_KEY_a).to_string());
		assert_eq!("A", Keysym::from(XKB_KEY_A).to_string());
	}

	#[test]
	fn utf8() {
		assert_eq!("a", Keysym::from(XKB_KEY_a).utf8());
		assert_eq!("Á", Keysym::from(XKB_KEY_Aacute).utf8());
	}
}
