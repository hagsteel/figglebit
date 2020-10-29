mod character;
mod parser;

use character::Char;

#[derive(Debug)]
pub enum ParseErr {
    MissingFlf2a,
    MissingHardBlank,
    IncompleteHeader,
    IncompleteFile,
    EmptyGlyph,
    EOLCharacterMissmatch,
}

type Result<T> = std::result::Result<T, ParseErr>;

// -----------------------------------------------------------------------------
//     - Font -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct Font {
    header: Header,
    chars: Vec<Char>,
}

impl Font {
    fn write(&self, s: &str) {
        let indices = s
            .as_bytes()
            .into_iter()
            .map(|b| (b - 32) as usize)
            .collect::<Vec<_>>();


        for index in indices {
            eprintln!("{:#?}", self.chars[index]);
        }

    }
}

// -----------------------------------------------------------------------------
//     - Header -
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Header {
    hard_blank: char,
    height: i16,
    baseline: i16,
    max_len: i16,
    old_layout: i16,
    comment_lines: usize,
    print_dir: i16,
    full_layout: Option<i16>,
    code_tag_count: Option<i16>,
}

pub fn parse(font_data: String) {
    match parser::parse(font_data) {
        Ok(font) => font.write("hello"),
        Err(e) => eprintln!("{:?}", e),
    }
}
