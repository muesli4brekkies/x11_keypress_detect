extern crate x11;
use self::x11::xlib::_XDisplay;
use std::{ptr, slice};

pub fn get_display() -> *mut _XDisplay {
  match unsafe { x11::xlib::XOpenDisplay(ptr::null()) } {
    display if unsafe { display.as_ref().is_some() } => display,
    _ => panic!("Could not connect to X display!"),
  }
}

pub fn key_pressed(display: &*mut _XDisplay) -> bool {
  let keymap = [0; 32].as_mut_ptr();
  unsafe {
    x11::xlib::XQueryKeymap(*display, keymap);
    slice::from_raw_parts(keymap, 32)
  }
  .iter()
  .any(|byte| *byte != 0)
}
