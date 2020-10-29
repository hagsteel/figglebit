use std::io::{self, Stdout, Write};

#[cfg(target_os = "windows")]
use crossterm::event::EnableMouseCapture;

#[cfg(not(target_os = "windows"))]
use crossterm::event::DisableMouseCapture;

use crossterm::cursor;
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{execute, ExecutableCommand, Result};

use crate::{Char, Font};

fn raw_mode() -> Result<Stdout> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, DisableMouseCapture,)?;
    stdout.execute(cursor::Hide)?;
    stdout.execute(Clear(ClearType::All))?;
    Ok(stdout)
}

fn render_chars(mut stdout: Stdout, chars: Vec<&Char>, font: &Font) {}

pub struct Renderer {
    stdout: Stdout,
    font: Font,
}

impl Renderer {
    pub fn clear(&mut self, width: u16, height: u16) {
        let mut stdout = &mut self.stdout;
        for y in 0..height {
            let clr = " ".repeat(width as usize);
            stdout.execute(MoveTo(0, y));
            stdout.execute(Print(clr));
        }
    }

    pub fn render_text(&mut self, text: &str) {
        let chars = self.font.to_chars(text);

        let mut offset = 0;
        let mut stdout = &mut self.stdout;
        for c in chars {
            // draw the char
            c.lines
                .iter()
                .enumerate()
                .map(|(y, line)| {
                    let x = offset;
                    stdout.execute(MoveTo(x, y as u16));
                    stdout.execute(Print(line.replace("$", " ")));
                })
                .count();
            offset += c.width;
        }
    }
}

pub fn init(font: Font) -> Result<Renderer> {
    Ok(Renderer {
        stdout: raw_mode()?,
        font,
    })
}

pub fn cleanup() {
    disable_raw_mode();
}
