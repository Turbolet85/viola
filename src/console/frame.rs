//! The floor rule: the smallest terminal the frame stays whole at.

use viola_pty::Size;

/// 80×24 (`Size::DEFAULT`): below it the frame is one line saying so.
pub(crate) const FLOOR: Size = Size::DEFAULT;

/// Whether the whole frame fits `size`.
pub(crate) fn fits(size: Size) -> bool {
    size.cols >= FLOOR.cols && size.rows >= FLOOR.rows
}

/// The rows between the title row and the key line: `rows` − 2.
pub(crate) fn body_rows(size: Size) -> usize {
    usize::from(size.rows).saturating_sub(2)
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

    #[rstest]
    #[case::at_the_floor(24, 22)]
    #[case::taller(30, 28)]
    #[case::none(0, 0)]
    fn body_rows_are_the_rows_less_the_title_and_the_key_line(
        #[case] rows: u16,
        #[case] body: usize,
    ) {
        assert_eq!(body_rows(Size { cols: 80, rows }), body);
    }
}
