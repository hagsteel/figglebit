use std::fs::read_to_string;
use std::thread;
use std::time::Duration;

use figglebit::{cleanup, init, parse};

fn main() {
    let font_data = read_to_string("fonts/graffiti.flf").unwrap();
    let font = parse(font_data).unwrap();
    let mut r = init(font).unwrap();

    let mut count = 0;
    r.render_text("ll");
    thread::sleep(Duration::from_secs(1));
    cleanup();
}
