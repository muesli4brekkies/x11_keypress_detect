use std::{ptr::null, slice::from_raw_parts};
use x11::xlib::{XOpenDisplay, XQueryKeymap, _XDisplay};

pub fn get_display() -> *mut _XDisplay {
  match unsafe { XOpenDisplay(null()) } {
    display if unsafe { display.as_ref().is_some() } => display,
    _ => panic!("Could not connect to X display!"),
  }
}

pub fn key_pressed(display: &*mut _XDisplay) -> bool {
  let keymap = [0; 32].as_mut_ptr();
  unsafe {
    XQueryKeymap(*display, keymap);
    from_raw_parts(keymap, 32)
  }
  .iter()
  .any(|byte| *byte != 0)
}
