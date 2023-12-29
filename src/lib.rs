use std::{
  ptr, slice,
};
use x11::xlib::{self, _XDisplay};
pub struct DisplayHandle {
  handle: *mut _XDisplay,
}

impl DisplayHandle {
 pub  fn get_display() -> DisplayHandle {
    match unsafe { xlib::XOpenDisplay(ptr::null()) } {
      display if unsafe { display.as_ref().is_some() } => DisplayHandle { handle: display },
      _ => panic!("Could not connect to a X display"),
    }
  }

pub  fn key_pressed(display: &DisplayHandle) -> bool {
    let keymap = [0; 32].as_mut_ptr();
    unsafe {
      xlib::XQueryKeymap(display.handle, keymap);
      slice::from_raw_parts(keymap, 32)
    }
    .iter()
    .any(|byte| *byte != 0)
  }
}
