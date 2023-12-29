use x11_keypress_detect::KeypressDetect;
 fn main() {
    let display = KeypressDetect::get_display();
    loop {
        println!("{}",KeypressDetect::key_pressed(&display))
    }
 }
