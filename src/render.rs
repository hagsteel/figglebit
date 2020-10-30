use std::io::{self, Stdout, Write};

#[cfg(target_os = "windows")]
use crossterm::event::EnableMouseCapture;

#[cfg(not(target_os = "windows"))]
use crossterm::event::DisableMouseCapture;

use crossterm::cursor;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{execute, ExecutableCommand, Result};

use crate::Font;

fn raw_mode() -> Result<Stdout> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, DisableMouseCapture,)?;
    stdout.execute(cursor::Hide)?;
    stdout.execute(Clear(ClearType::All))?;
    Ok(stdout)
}

pub struct Renderer {
    font: Font,
}

impl Renderer {
    pub fn new(font: Font) -> Self {
        Self {
            font
        }
    }

    pub fn render<T: Write + ?Sized>(&self, text: &str, buf: &mut T) -> std::io::Result<usize> {
        let chars = self.font.to_chars(text);

        let count = self.font.header.height;

        let mut current_line = 0;
        let mut bytes_written = 0;

        for _ in 0..count {
            let _ = chars.iter().try_for_each::<_, io::Result<()>>(|c| {
                let line = &c.lines[current_line];
                bytes_written += buf.write(line.as_bytes())?;
                Ok(())
            })?;

            current_line += 1;

            bytes_written += buf.write(&[b'\r', b'\n'])?;
        }

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
    use std::io::Write;
    use super::*;

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
