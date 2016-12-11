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

use std::ffi::{CStr, CString};

use ffi::*;
use keymap::Keymap;

#[derive(Debug)]
pub struct Leds<'a>(pub &'a Keymap);

impl<'a> Leds<'a> {
	pub fn len(&self) -> usize {
		unsafe {
			xkb_keymap_num_leds(self.0.as_ptr()) as usize
		}
	}

	pub fn by_index(&self, index: usize) -> Option<&str> {
		unsafe {
			xkb_keymap_led_get_name(self.0.as_ptr(), index as xkb_led_index_t)
				.as_ref().map(|ptr| CStr::from_ptr(ptr).to_str().unwrap())
		}
	}

	pub fn by_name<S: AsRef<str>>(&self, name: S) -> Option<usize> {
		unsafe {
			let string = CString::new(name.as_ref()).unwrap();

			match xkb_keymap_led_get_index(self.0.as_ptr(), string.as_ptr()) {
				XKB_LED_INVALID =>
					None,

				index =>
					Some(index as usize)
			}
		}
	}

	pub fn iter(&self) -> Iter {
		Iter::new(self)
	}
}

pub struct Iter<'a> {
	inner:   &'a Leds<'a>,
	current: xkb_led_index_t,
	length:  xkb_led_index_t,
}

impl<'a> Iter<'a> {
	fn new(inner: &'a Leds<'a>) -> Iter<'a> {
		Iter {
			inner:   inner,
			current: 0,
			length:  inner.len() as xkb_led_index_t,
		}
	}
}

impl<'a> Iterator for Iter<'a> {
	type Item = (usize, &'a str);

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.length {
			return None;
		}

		unsafe {
			let index = self.current;
			let name  = xkb_keymap_led_get_name(self.inner.0.as_ptr(), index);

			self.current += 1;

			Some((index as usize, CStr::from_ptr(name).to_str().unwrap()))
		}
	}
}

impl<'a> IntoIterator for &'a Leds<'a> {
	type Item     = (usize, &'a str);
	type IntoIter = Iter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}
