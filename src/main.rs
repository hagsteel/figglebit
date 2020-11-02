use std::fs::read_to_string;

use crossterm::ExecutableCommand;
use crossterm::style::Print;

use figglebit::{init, cleanup, parse, Renderer};

fn main() {
    let font_data = read_to_string("fonts/Slant.flf").unwrap();
    let font = parse(font_data).unwrap();
    let renderer = Renderer::new(font);

    let mut buf = String::new();
    let res = renderer.render("C-", unsafe { buf.as_mut_vec() });

    // let mut stdout = init().unwrap();
    // let _ = stdout.execute(Print(buf));

    // cleanup();
}
