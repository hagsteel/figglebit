pub(super) mod character;
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
    pub fn to_chars(&self, s: &str) -> Vec<&Char> {
        s.as_bytes()
            .into_iter()
            .map(|b| (b - 32) as usize)
            .map(|i| &self.chars[i])
            .collect::<Vec<_>>()
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

pub use parser::parse;
