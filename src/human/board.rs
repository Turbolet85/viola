//! The board of `viola list`: the state refusal pairs, the BAY line, the rows and the one write.
//! The wrapped rows come first; the sessions viola does not wrap follow under their own separator,
//! read-only, with `n/a` where only a wrapper could know.

mod bay;
mod cells;
mod pairs;
mod rows;
#[cfg(test)]
mod test_support;

pub(crate) use bay::BayBudget;
pub(crate) use pairs::{home_state_pair, instance_state_pair};
pub(crate) use rows::write_board;
