//          flf2a$ 6 5 20 15 3 0 143 229    NOTE: The first five characters in
//            |  | | | |  |  | |  |   |     the entire file must be "flf2a".
//           /  /  | | |  |  | |  |   \
//  Signature  /  /  | |  |  | |   \   Codetag_Count
//    Hardblank  /  /  |  |  |  \   Full_Layout*
//         Height  /   |  |   \  Print_Direction
//         Baseline   /    \   Comment_Lines
//          Max_Length      Old_Layout*

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
    pub(crate) header: Header,
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
// -1  full-width layout by default
//  0  horizontal fitting (kerning) layout by default*
//  1  apply horizontal smushing rule 1 by default
//  2  apply horizontal smushing rule 2 by default
//  4  apply horizontal smushing rule 3 by default
//  8  apply horizontal smushing rule 4 by default
// 16  apply horizontal smushing rule 5 by default
// 32  apply horizontal smushing rule 6 by default
bitflags::bitflags! {
    pub struct OldLayout: i16 {
        const FULL_WIDTH = -1;
        const KERNING = 0;
        const HORZ_SMUSH_1 = 1;
        const HORZ_SMUSH_2 = 2;
        const HORZ_SMUSH_3 = 4;
        const HORZ_SMUSH_4 = 8;
        const HORZ_SMUSH_5 = 16;
        const HORZ_SMUSH_6 = 32;
    }
}

#[derive(Debug)]
pub(crate) struct Header {
    pub hard_blank: char,
    pub(crate) height: i16,
    baseline: i16,
    max_len: i16,
    pub old_layout: OldLayout,
    comment_lines: usize,
    print_dir: i16,
    full_layout: Option<i16>,
    code_tag_count: Option<i16>,
}

pub use parser::parse;
