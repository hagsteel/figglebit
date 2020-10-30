use super::character::Char;
use super::{Font, Header};
use super::{ParseErr, Result};

const SIGNATURE: &'static str = "flf2";

// -----------------------------------------------------------------------------
//     - Parsing-
// -----------------------------------------------------------------------------
fn parse_header(src: &str) -> Result<Header> {
    if !src.starts_with(SIGNATURE) {
        return Err(ParseErr::MissingFlf2a);
    }

    let mut skip = SIGNATURE.len() + 1;

    let hard_blank: char = *&src[skip..skip + 1].as_bytes()[0] as char;
    skip += 2;

    let values = src[skip..]
        .split(' ')
        .filter_map(|n| n.parse().ok())
        .collect::<Vec<i16>>();

    if values.len() < 4 {
        return Err(ParseErr::IncompleteHeader);
    }

    let mut values = values.into_iter();

    Ok(Header {
        hard_blank,
        height: values.next().unwrap(),
        baseline: values.next().unwrap(),
        max_len: values.next().unwrap(),
        old_layout: super::OldLayout::from_bits_truncate(values.next().unwrap()),
        comment_lines: values.next().unwrap_or(0) as usize,
        print_dir: values.next().unwrap_or(0),
        full_layout: values.next(),
        code_tag_count: values.next(),
    })
}

pub fn parse(font_data: String) -> Result<Font> {
    // First line is the header
    let first_line = font_data
        .find('\n')
        .map(|index| &font_data[..index])
        .ok_or(ParseErr::IncompleteFile)?;

    let header = parse_header(first_line)?;

    // Parsing character
    let chars = font_data
        .lines()
        .skip(header.comment_lines + 1) // + 1 for the header
        .collect::<Vec<_>>()
        .chunks_exact(header.height as usize)
        .map(Vec::from)
        .map(Char::new)
        .take(102)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    Ok(Font { chars, header })
}
