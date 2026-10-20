//! The state refusal pairs `viola revive` and `viola list` print.

/// A state-files refusal for one instance: `viola revive` and `viola list` print it alike.
pub(crate) fn instance_state_pair(name: &str) -> (String, String) {
    (
        format!("{name}'s state files can be written by another user"),
        "viola will not read them; make the viola home and its files owner-only".to_owned(),
    )
}

/// The home's state refusal (design-system §Surface: cli, Component Patterns 2).
pub(crate) fn home_state_pair() -> (&'static str, &'static str) {
    (
        "the viola home is not private to you",
        "the viola home must be readable only by you; viola does not change its permissions",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instance_and_home_state_pairs_are_the_fixed_texts() {
        assert_eq!(
            instance_state_pair("builder"),
            (
                "builder's state files can be written by another user".to_owned(),
                "viola will not read them; make the viola home and its files owner-only".to_owned()
            )
        );
        assert_eq!(
            home_state_pair(),
            (
                "the viola home is not private to you",
                "the viola home must be readable only by you; viola does not change its permissions"
            )
        );
    }
}
