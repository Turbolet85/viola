//! The BAY context line: the skip counts.

use viola_state::board::SkippedTotal;

/// The BAY context line (layout-templates §Surface: cli, Component — Header / banner): its one
/// cell, the skip counts.
pub(crate) fn bay_line(skipped: &SkippedTotal) -> String {
    format!(
        "BAY  skipped {} - {} - {}",
        skipped.unknown_kinds, skipped.unknown_fields, skipped.torn_lines
    )
}

#[cfg(test)]
mod tests {
    use super::super::test_support::counts;
    use super::*;

    #[test]
    fn bay_line_is_the_skip_counts() {
        assert_eq!(bay_line(&counts(0, 0, 0)), "BAY  skipped 0 - 0 - 0");
        assert_eq!(bay_line(&counts(2, 0, 1)), "BAY  skipped 2 - 0 - 1");
    }
}
