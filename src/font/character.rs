use super::{ParseErr, Result};

#[derive(Debug)]
pub struct Char {
    pub width: u16,
    pub lines: Vec<String>,
}

impl Char {
    pub(super) fn new(lines: Vec<&str>) -> Result<Char> {
        let mut lines = lines.into_iter().map(String::from).collect::<Vec<_>>();

        // Get the EOL character
        let eol_char = lines
            .first()
            .ok_or(ParseErr::EmptyGlyph)?
            .chars()
            .last()
            .ok_or(ParseErr::EmptyGlyph)?;

        // Compare last character with the eol character
        let compare_eol = |c: (Option<char>, char)| match c.0 == Some(c.1) {
            true => Ok(()),
            false => Err(ParseErr::EOLCharacterMissmatch),
        };

        // Remove the end characters for each line
        lines.iter_mut().map(String::pop).count();

        // Make sure the last character is the same
        // as the eol character
        lines
            .iter_mut()
            .rev()
            .map(String::pop)
            .zip(Some(eol_char))
            .map(compare_eol)
            .next()
            .ok_or(ParseErr::EOLCharacterMissmatch)??;

        // Get the character width
        let width = lines.first().map(String::len).unwrap() as u16;

        Ok(Char {
            width,
            lines,
        })
    }
}
