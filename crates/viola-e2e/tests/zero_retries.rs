//! Zero flakiness is a checked property, not a default: no nextest profile may retry, and the CI
//! profile states `retries = 0` outright (test-plan §10 Zero-flakiness budget).

use std::fs;

use viola_e2e::harness::Workspace;

/// Every retry violation in a nextest config, as `line N: reason`.
fn retry_violations(config: &str) -> Vec<String> {
    let mut violations = Vec::new();
    let mut section = String::new();
    let mut ci_states_zero = false;
    for (n, raw) in config.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.starts_with('[') {
            section = line
                .trim_matches(|c| c == '[' || c == ']')
                .trim()
                .to_owned();
            continue;
        }
        let Some(value) = line
            .strip_prefix("retries")
            .and_then(|rest| rest.trim_start().strip_prefix('='))
        else {
            continue;
        };
        let value = value.trim();
        if value == "0" {
            ci_states_zero |= section == "profile.ci";
        } else {
            violations.push(format!("line {}: retries = {value}", n + 1));
        }
    }
    if !ci_states_zero {
        violations.push("profile.ci: no retries = 0".to_owned());
    }
    violations
}

#[test]
fn zero_retries_hold_in_the_repo_nextest_config() {
    let path = Workspace::from_build()
        .root
        .join(".config")
        .join("nextest.toml");
    let config = fs::read_to_string(path).expect(".config/nextest.toml");
    assert_eq!(retry_violations(&config), Vec::<String>::new());
}

#[test]
fn zero_retries_checker_flags_every_retry_form() {
    let clean = "[profile.ci]\nretries = 0\nfail-fast = false\n";
    assert!(retry_violations(clean).is_empty());
    for (config, expected) in [
        (
            "[profile.ci]\nretries = 0\n[profile.default]\nretries = 1\n",
            "line 4: retries = 1",
        ),
        (
            "[profile.ci]\nretries = { backoff = \"fixed\", count = 2 }\n",
            "line 2: retries = { backoff = \"fixed\", count = 2 }",
        ),
        (
            "[profile.ci]\nfail-fast = false\n",
            "profile.ci: no retries = 0",
        ),
        (
            "[profile.mutants]\nretries = 0\n",
            "profile.ci: no retries = 0",
        ),
        ("[profile.ci]\n  retries=3 # flaky\n", "line 2: retries = 3"),
    ] {
        assert!(
            retry_violations(config).contains(&expected.to_owned()),
            "{config:?} -> {:?}",
            retry_violations(config)
        );
    }
}
