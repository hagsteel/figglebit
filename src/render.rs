use std::io::{self, Stdout, Write};

#[cfg(target_os = "windows")]
use crossterm::event::EnableMouseCapture;

#[cfg(not(target_os = "windows"))]
use crossterm::event::DisableMouseCapture;

use crossterm::cursor;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{execute, ExecutableCommand, Result};

use crate::smushing::{get_horizontal_smush_len, horizontal_smush};
use crate::Font;

fn raw_mode() -> Result<Stdout> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, DisableMouseCapture,)?;
    stdout.execute(cursor::Hide)?;
    stdout.execute(Clear(ClearType::All))?;
    Ok(stdout)
}

struct CharData {
    offset: usize,
}

pub struct Renderer {
    font: Font,
}

impl Renderer {
    pub fn new(font: Font) -> Self {
        Self { font }
    }

    pub fn render<T: Write + ?Sized>(&self, text: &str, buf: &mut T) -> std::io::Result<usize> {
        let mut chars = self.font.to_chars(text).into_iter().peekable();

        let line_count = self.font.header.height as usize;
        let mut bytes_written = 0;

        let mut overlap = 10_000;
        let mut output = vec!["".to_string(); line_count];

        for (idx, c) in chars.enumerate() {
            // TODO: in case of full width: just write each line, no need to do anything else

            overlap = 10_000;
            for row in 0..line_count {
                let next_overlap =
                    get_horizontal_smush_len(&output[row], &c.lines[row], &self.font.header);
                overlap = overlap.min(next_overlap);
            }

            output = horizontal_smush(&output, &c.lines, overlap, &self.font.header);

            // Replace hard blanks with space
            output.iter_mut().for_each(|mut line| {
                *line = line.replace(self.font.header.hard_blank, " ");
            });
        }

        output.iter().try_for_each::<_, io::Result<()>>(|line| {
            bytes_written += buf.write(line.as_bytes())?;
            bytes_written += buf.write(&[b'\r', b'\n'])?;
            Ok(())
        })?;

        Ok(bytes_written)
    }
}

pub fn init() -> Result<Stdout> {
    Ok(raw_mode()?)
}

pub fn cleanup() {
    let _ = disable_raw_mode();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Write;

    const FONT_DATA: &'static str = include_str!("../fonts/Slant.flf");

    fn font() -> Font {
        crate::parse(FONT_DATA.to_string()).unwrap()
    }

    #[test]
    fn full_horizontal() {
        let mut buf = Vec::new();
        let renderer = Renderer::new(&mut buf, font());
        let s = String::from_utf8(buf).unwrap();
        let expected = r#"
   ______
  / ____/
 / /      ______
/ /___   /_____/
\____/
"#;

        assert_eq!(expected, s);
    }

    // #[test]
    // fn fitted_horizontal() {
    //     render(&mut buf);
    //     let expected = r#"
    // ______
    // / ____/
    // / /   ______
    // / /___/_____/
    // \____/
    // "#;
    // }

    // #[test]
    // fn smushed_right() {
    //     render(&mut buf);
    //     let expected = r#"
    // ______
    // / ____/
    // / /  ______
    // / /__/_____/
    // \____/
    // "#;
    // }

    // #[test]
    // fn smushed_universal() {
    //     render(&mut buf);
    //     let expected = r#"
    // ______
    // / ____/
    // / /  ______
    // / /__/_____/
    // \____/
    // "#;
    // }
}
