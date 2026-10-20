//! The BAY context line: the budget windows, the reading's age, the gate and the skip counts.

use chrono::{DateTime, Utc};
use viola_core::{BudgetWindow, Reading};
use viola_state::board::SkippedTotal;
use viola_state::budget::StampedReading;

/// The budget the BAY line shows: the stamped reading and the gate's verdict over it.
pub(crate) struct BayBudget<'a> {
    pub(crate) reading: &'a StampedReading,
    pub(crate) paused: bool,
}

/// `5H 91 %`, with ` expired` once a known reset is not later than `now`; `5H unknown` when the
/// window or its figure was not read. The figure prints rounded down to a whole number.
fn window_cell(label: &str, window: &Reading<BudgetWindow>, now: DateTime<Utc>) -> String {
    let Reading::Known(BudgetWindow {
        used_percentage: Reading::Known(used),
        resets_at,
    }) = window
    else {
        return format!("{label} unknown");
    };
    let expired = matches!(resets_at, Reading::Known(at)
        if DateTime::parse_from_rfc3339(at).is_ok_and(|at| at <= now));
    let suffix = if expired { " expired" } else { "" };
    format!("{label} {:.0} %{suffix}", used.floor())
}

/// `read 4m ago` in whole minutes, rounded down; `read unknown` for no instant, one that does not
/// parse, or one ahead of `now`.
fn read_cell(read_at: &Reading<String>, now: DateTime<Utc>) -> String {
    let at = match read_at {
        Reading::Known(at) => DateTime::parse_from_rfc3339(at).ok(),
        Reading::Unknown(_) => None,
    };
    match at.filter(|at| *at <= now) {
        Some(at) => format!("read {}m ago", (now - at.with_timezone(&Utc)).num_minutes()),
        None => "read unknown".to_owned(),
    }
}

/// The BAY context line (layout-templates §Surface: cli, Component — Header / banner): its cells
/// in the fixed order, two spaces apart.
pub(crate) fn bay_line(
    budget: Option<&BayBudget<'_>>,
    skipped: &SkippedTotal,
    now: DateTime<Utc>,
) -> String {
    let (five_hour, seven_day, read, gate) = match budget {
        Some(budget) => (
            window_cell("5H", &budget.reading.reading.five_hour, now),
            window_cell("7D", &budget.reading.reading.seven_day, now),
            read_cell(&budget.reading.read_at, now),
            if budget.paused {
                "budget-paused"
            } else {
                "gate open"
            },
        ),
        None => (
            "5H unknown".to_owned(),
            "7D unknown".to_owned(),
            "read unknown".to_owned(),
            "gate open",
        ),
    };
    format!(
        "BAY  {five_hour}  {seven_day}  {read}  {gate}  skipped {} - {} - {}",
        skipped.unknown_kinds, skipped.unknown_fields, skipped.torn_lines
    )
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{NOW, at, counts};
    use super::*;
    use rstest::rstest;

    fn window(used: Reading<f64>, resets_at: &str) -> Reading<BudgetWindow> {
        Reading::Known(BudgetWindow {
            used_percentage: used,
            resets_at: Reading::Known(resets_at.to_owned()),
        })
    }

    fn stamped(
        five_hour: Reading<BudgetWindow>,
        seven_day: Reading<BudgetWindow>,
        read_at: Reading<String>,
    ) -> StampedReading {
        StampedReading {
            reading: viola_core::BudgetReading {
                five_hour,
                seven_day,
            },
            read_at,
        }
    }

    #[test]
    fn bay_line_without_a_reading_is_the_unknown_line() {
        assert_eq!(
            bay_line(None, &counts(0, 0, 0), at(NOW)),
            "BAY  5H unknown  7D unknown  read unknown  gate open  skipped 0 - 0 - 0"
        );
        assert_eq!(
            bay_line(None, &counts(2, 0, 1), at(NOW)),
            "BAY  5H unknown  7D unknown  read unknown  gate open  skipped 2 - 0 - 1"
        );
    }

    #[test]
    fn bay_line_with_a_reading_shows_the_figures_the_age_and_the_gate() {
        let reading = stamped(
            window(Reading::Known(91.0), "2099-01-01T00:00:00.000Z"),
            window(Reading::Known(23.5), "2099-01-06T00:00:00.000Z"),
            Reading::Known("2025-03-01T11:56:00.000Z".to_owned()),
        );
        let paused = BayBudget {
            reading: &reading,
            paused: true,
        };
        assert_eq!(
            bay_line(Some(&paused), &counts(0, 0, 0), at(NOW)),
            "BAY  5H 91 %  7D 23 %  read 4m ago  budget-paused  skipped 0 - 0 - 0"
        );
        let open = BayBudget {
            reading: &reading,
            paused: false,
        };
        assert_eq!(
            bay_line(Some(&open), &counts(0, 0, 0), at(NOW)),
            "BAY  5H 91 %  7D 23 %  read 4m ago  gate open  skipped 0 - 0 - 0"
        );
    }

    #[rstest]
    #[case::rounded_down(window(Reading::Known(89.6), "2099-01-01T00:00:00.000Z"), "5H 89 %")]
    #[case::zero(window(Reading::Known(0.0), "2099-01-01T00:00:00.000Z"), "5H 0 %")]
    #[case::a_reset_at_now_is_expired(window(Reading::Known(91.0), NOW), "5H 91 % expired")]
    #[case::a_passed_reset(
        window(Reading::Known(91.0), "2025-02-01T16:00:00.000Z"),
        "5H 91 % expired"
    )]
    #[case::a_reset_just_ahead(window(Reading::Known(91.0), "2025-03-01T12:00:00.001Z"), "5H 91 %")]
    #[case::a_reset_that_is_no_instant(window(Reading::Known(91.0), "soon"), "5H 91 %")]
    #[case::an_unknown_reset(
        Reading::Known(BudgetWindow { used_percentage: Reading::Known(91.0), resets_at: Reading::UNKNOWN }),
        "5H 91 %"
    )]
    #[case::an_unknown_figure(window(Reading::UNKNOWN, NOW), "5H unknown")]
    #[case::an_unknown_window(Reading::UNKNOWN, "5H unknown")]
    fn bay_line_window_cell(#[case] five_hour: Reading<BudgetWindow>, #[case] cell: &str) {
        let reading = stamped(five_hour, Reading::UNKNOWN, Reading::Known(NOW.to_owned()));
        let budget = BayBudget {
            reading: &reading,
            paused: false,
        };
        assert_eq!(
            bay_line(Some(&budget), &counts(0, 0, 0), at(NOW)),
            format!("BAY  {cell}  7D unknown  read 0m ago  gate open  skipped 0 - 0 - 0")
        );
    }

    #[rstest]
    #[case::thirty_seconds("2025-03-01T11:59:30.000Z", "read 0m ago")]
    #[case::just_under_a_minute("2025-03-01T11:59:00.001Z", "read 0m ago")]
    #[case::a_minute("2025-03-01T11:59:00.000Z", "read 1m ago")]
    #[case::seventy_five_minutes("2025-03-01T10:45:00.000Z", "read 75m ago")]
    #[case::ahead_of_now("2025-03-01T12:00:00.001Z", "read unknown")]
    #[case::no_instant("yesterday", "read unknown")]
    fn bay_line_read_cell(#[case] read_at: &str, #[case] cell: &str) {
        assert_eq!(
            read_cell(&Reading::Known(read_at.to_owned()), at(NOW)),
            cell
        );
    }

    #[test]
    fn bay_line_read_cell_of_an_unknown_instant_is_unknown() {
        assert_eq!(read_cell(&Reading::UNKNOWN, at(NOW)), "read unknown");
    }
}
