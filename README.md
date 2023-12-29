# x11_keypress_detect
A very simple crate providing access to the x11 display and a boolean to check if a key is pressed

This crate contains two public functions - 

* `get_display()` - Gets an instance of the X11 display. This needs to be run first to instantiate the X11 display connection.
                        - Only do this once per program, multiple calls will eventually stack up and crash.

* `key_pressed()` - Returns a boolean, true if a key is pressed during detection or false otherwise.

 ## Example
 ```rust
 use x11_keypress_detect::{get_display,key_pressed}
 fn main() {
    get_display();
    loop {
        println!("{}",key_pressed())
    }
 }
 ```