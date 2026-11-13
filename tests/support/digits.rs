//! The security negatives' number oracle: a pid, a uid or a start time printed anywhere shows as
//! one whole digit run, never inside a longer number.

/// The digit runs of `text`: a pid, a uid or a start time shows as one of them.
pub fn digit_runs(text: &str) -> Vec<&str> {
    text.split(|c: char| !c.is_ascii_digit())
        .filter(|run| !run.is_empty())
        .collect()
}
