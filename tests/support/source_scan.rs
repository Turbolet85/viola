//! The one source-masking scanner the census binaries share (test-plan · one shared test helper):
//! a Rust source text with its comments, and by its mode its string and char literals too, turned
//! into spaces with every newline kept, so a census searches only code and counts only code braces.
//! Each census binary takes it by `#[path]`, so only those binaries run its cases.

/// What `masked` turns into spaces.
#[derive(Clone, Copy)]
pub enum Mask {
    /// Line and (nested) block comments; literals kept.
    Comments,
    /// Comments, string literals (raw and byte forms included) and char literals.
    CommentsAndLiterals,
}

/// `text` with every byte `mask` names turned into a space, its newlines kept.
pub fn masked(text: &str, mask: Mask) -> Vec<u8> {
    let literals = matches!(mask, Mask::CommentsAndLiterals);
    let bytes = text.as_bytes();
    let mut out = bytes.to_vec();
    let blank = |out: &mut Vec<u8>, from: usize, to: usize| {
        for b in &mut out[from..to] {
            if *b != b'\n' {
                *b = b' ';
            }
        }
    };
    let ident = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let mut i = 0;
    while i < bytes.len() {
        let rest = &bytes[i..];
        if rest.starts_with(b"//") {
            let end = rest
                .iter()
                .position(|&b| b == b'\n')
                .map_or(bytes.len(), |n| i + n);
            blank(&mut out, i, end);
            i = end;
        } else if rest.starts_with(b"/*") {
            let mut depth = 0usize;
            let mut j = i;
            while j < bytes.len() {
                if bytes[j..].starts_with(b"/*") {
                    depth += 1;
                    j += 2;
                } else if bytes[j..].starts_with(b"*/") {
                    depth -= 1;
                    j += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    j += 1;
                }
            }
            blank(&mut out, i, j);
            i = j;
        } else if (rest[0] == b'r' || rest.starts_with(b"br"))
            && (i == 0 || !ident(bytes[i - 1]))
            && raw_string_opening(rest).is_some()
        {
            let (prefix, hashes) = raw_string_opening(rest).expect("checked");
            let close: Vec<u8> = std::iter::once(b'"')
                .chain(std::iter::repeat_n(b'#', hashes))
                .collect();
            let body = i + prefix;
            let end = bytes[body..]
                .windows(close.len())
                .position(|w| w == close.as_slice())
                .map_or(bytes.len(), |n| body + n + close.len());
            if literals {
                blank(&mut out, i, end);
            }
            i = end;
        } else if rest[0] == b'"' {
            let mut j = i + 1;
            while j < bytes.len() && bytes[j] != b'"' {
                j += if bytes[j] == b'\\' { 2 } else { 1 };
            }
            let end = (j + 1).min(bytes.len());
            if literals {
                blank(&mut out, i, end);
            }
            i = end;
        } else if rest[0] == b'\'' {
            match char_literal_len(rest) {
                Some(len) => {
                    if literals {
                        blank(&mut out, i, i + len);
                    }
                    i += len;
                }
                None => i += 1,
            }
        } else {
            i += 1;
        }
    }
    out
}

/// A raw string's opening (`r"`, `r#"`, `br##"` …): its length and its count of `#`.
fn raw_string_opening(rest: &[u8]) -> Option<(usize, usize)> {
    let after_r = if rest.starts_with(b"br") { 2 } else { 1 };
    let hashes = rest[after_r..].iter().take_while(|&&b| b == b'#').count();
    (rest.get(after_r + hashes) == Some(&b'"')).then_some((after_r + hashes + 1, hashes))
}

/// The length of a char literal starting at `rest`, or `None` for a lifetime.
fn char_literal_len(rest: &[u8]) -> Option<usize> {
    if rest.get(1) == Some(&b'\\') {
        // The escaped byte is never the closing quote: `'\''` closes at its fourth byte.
        let close = rest.get(3..)?.iter().position(|&b| b == b'\'')?;
        return Some(close + 4);
    }
    let text = std::str::from_utf8(&rest[1..rest.len().min(6)])
        .ok()
        .or_else(|| {
            (1..6)
                .rev()
                .find_map(|n| std::str::from_utf8(rest.get(1..1 + n)?).ok())
        })?;
    let c = text.chars().next()?;
    let len = c.len_utf8();
    (rest.get(1 + len) == Some(&b'\'')).then_some(len + 2)
}

#[cfg(test)]
mod tests {
    use super::{Mask, masked};
    use rstest::rstest;

    #[rstest]
    #[case::line_comment("a // b\nc", Mask::Comments, "a     \nc")]
    #[case::nested_block_comment("a /* b /* c */ d */ e", Mask::Comments, "a                   e")]
    #[case::block_comment_keeps_newlines("a /* b\nc */ d", Mask::Comments, "a     \n     d")]
    #[case::string_kept_by_comments("x = \"// no\";", Mask::Comments, "x = \"// no\";")]
    #[case::string_masked("x = \"s\";", Mask::CommentsAndLiterals, "x =    ;")]
    #[case::escaped_quote("\"a\\\"b\" c", Mask::CommentsAndLiterals, "       c")]
    #[case::raw_string_with_hashes(
        "r##\"a \"# b\"## c",
        Mask::CommentsAndLiterals,
        "              c"
    )]
    #[case::raw_string_kept_by_comments("r#\"/* x */\"#", Mask::Comments, "r#\"/* x */\"#")]
    #[case::byte_raw_string("br\"x\" y", Mask::CommentsAndLiterals, "      y")]
    #[case::identifier_ending_in_r("fr\"x\"", Mask::CommentsAndLiterals, "fr   ")]
    #[case::char_literal("c = 'x';", Mask::CommentsAndLiterals, "c =    ;")]
    #[case::escaped_char_literal("c = '\\'';", Mask::CommentsAndLiterals, "c =     ;")]
    #[case::lifetime_kept(
        "fn f<'a>(x: &'a u8)",
        Mask::CommentsAndLiterals,
        "fn f<'a>(x: &'a u8)"
    )]
    #[case::comment_in_literal_mode(
        "a // 'x' \"y\"\nb",
        Mask::CommentsAndLiterals,
        "a           \nb"
    )]
    fn masked_blanks_what_its_mode_names(
        #[case] text: &str,
        #[case] mask: Mask,
        #[case] expected: &str,
    ) {
        assert_eq!(String::from_utf8(masked(text, mask)).unwrap(), expected);
    }
}
