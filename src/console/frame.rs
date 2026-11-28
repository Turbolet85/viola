//! The floor rule: the smallest terminal the frame stays whole at.

use viola_pty::Size;

/// 80×24 (`Size::DEFAULT`): below it the frame is one line saying so.
pub(crate) const FLOOR: Size = Size::DEFAULT;

/// Whether the whole frame fits `size`.
pub(crate) fn fits(size: Size) -> bool {
    size.cols >= FLOOR.cols && size.rows >= FLOOR.rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::at_the_floor(80, 24, true)]
    #[case::above_it(100, 30, true)]
    #[case::one_column_short(79, 24, false)]
    #[case::one_row_short(80, 23, false)]
    #[case::wide_but_short(200, 10, false)]
    #[case::tall_but_narrow(40, 60, false)]
    fn fits_reads_the_floor_on_both_axes(
        #[case] cols: u16,
        #[case] rows: u16,
        #[case] expected: bool,
    ) {
        assert_eq!(fits(Size { cols, rows }), expected);
    }
}
