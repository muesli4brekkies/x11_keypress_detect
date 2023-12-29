extern crate x11;
use self::x11::xlib::_XDisplay;
use std::{ptr, slice};
pub struct KeypressDetect {
  handle: *mut _XDisplay,
}

impl KeypressDetect {
  pub fn get_display() -> KeypressDetect {
    match unsafe { x11::xlib::XOpenDisplay(ptr::null()) } {
      display if unsafe { display.as_ref().is_some() } => KeypressDetect { handle: display },
      _ => panic!("Could not connect to a X display"),
    }
  }

  pub fn key_pressed(display: &KeypressDetect) -> bool {
    let keymap = [0; 32].as_mut_ptr();
    unsafe {
      x11::xlib::XQueryKeymap(display.handle, keymap);
      slice::from_raw_parts(keymap, 32)
    }
    .iter()
    .any(|byte| *byte != 0)
  }
}
