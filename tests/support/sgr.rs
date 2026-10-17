//! The SGR reader of a terminal stream: what a styled or plain-output oracle reads, the stream never
//! parsed as a screen.

const ESC: u8 = 0x1B;

/// The parameter list of every `ESC [ params m` sequence, an empty one read as `[0]`.
fn sgr_sequences(stream: &[u8]) -> Vec<Vec<u32>> {
    let mut sequences = Vec::new();
    let mut i = 0;
    while i + 1 < stream.len() {
        if stream[i] != ESC || stream[i + 1] != b'[' {
            i += 1;
            continue;
        }
        let start = i + 2;
        let mut end = start;
        while end < stream.len() && (stream[end].is_ascii_digit() || stream[end] == b';') {
            end += 1;
        }
        if end < stream.len() && stream[end] == b'm' {
            let params = std::str::from_utf8(&stream[start..end]).expect("ascii");
            sequences.push(
                params
                    .split(';')
                    .map(|p| p.parse::<u32>().unwrap_or(0))
                    .collect(),
            );
        }
        i = end;
    }
    sequences
}

/// The parameters of every `ESC [ params m` sequence, an empty list read as `0`. Extended colour
/// sub-parameters (`38;5;n`, `38;2;r;g;b`) are skipped, so their values never read as attributes.
pub fn sgr_attributes(stream: &[u8]) -> Vec<u32> {
    let mut attributes = Vec::new();
    for sequence in sgr_sequences(stream) {
        let mut values = sequence.into_iter();
        while let Some(value) = values.next() {
            if matches!(value, 38 | 48 | 58) {
                let _ = match values.next() {
                    Some(5) => values.next(),
                    Some(2) => values.nth(2),
                    _ => None,
                };
                continue;
            }
            attributes.push(value);
        }
    }
    attributes
}

/// How many CR bytes the stream holds that are not directly followed by LF: a `\r` redraw.
pub fn redraws(stream: &[u8]) -> usize {
    stream
        .iter()
        .enumerate()
        .filter(|&(i, &b)| b == b'\r' && stream.get(i + 1) != Some(&b'\n'))
        .count()
}

/// Whether any SGR sequence sets a foreground colour: `30`–`37`, `90`–`97` or an extended `38;…`.
/// A background's or an underline colour's sub-parameters never read as one.
pub fn sets_a_foreground(stream: &[u8]) -> bool {
    sgr_sequences(stream).into_iter().any(|sequence| {
        let mut values = sequence.into_iter();
        while let Some(value) = values.next() {
            match value {
                30..=38 | 90..=97 => return true,
                48 | 58 => {
                    let _ = match values.next() {
                        Some(5) => values.next(),
                        Some(2) => values.nth(2),
                        _ => None,
                    };
                }
                _ => {}
            }
        }
        false
    })
}
