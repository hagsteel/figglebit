use std::fs::read_to_string;
use std::io;

use figglebit::font::parse;

fn main() -> io::Result<()> {
    // let font = read_to_string("fonts/graffiti.flf")?;
    let font = read_to_string("fonts/smslant.flf")?;
    parse(font);
    Ok(())
}



// #![warn(rust_2018_idioms, clippy::all, clippy::pedantic)]
// #![allow(clippy::non_ascii_literal)]
// #![warn(clippy::nursery)]

// use std::io::{self, Stdout, Write};

// #[cfg(target_os = "windows")]
// use crossterm::event::EnableMouseCapture;

// #[cfg(not(target_os = "windows"))]
// use crossterm::event::DisableMouseCapture;

// use crossterm::cursor;
// use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size};
// use crossterm::{execute, ExecutableCommand, Result};
// use crossterm::style::Color;


// fn raw_mode() -> Result<Stdout> {
//     enable_raw_mode()?;
//     let mut stdout = io::stdout();

//     #[cfg(target_os = "windows")]
//     execute!(stdout, EnableMouseCapture)?;

//     #[cfg(not(target_os = "windows"))]
//     execute!(stdout, DisableMouseCapture,)?;

//     stdout.execute(cursor::Hide)?;
//     stdout.execute(Clear(ClearType::All))?;
//     Ok(stdout)
// }


// fn main() -> Result<()> {
//     let stdout = raw_mode()?;

//     Ok(())
// }

