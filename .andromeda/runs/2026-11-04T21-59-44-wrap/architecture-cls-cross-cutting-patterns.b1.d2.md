@answer n=1 entity=cross-cutting-patterns#1.1 v=no
why: the session adds no config key, environment variable or product read, and the TERM, NO_COLOR and COLORTERM reading rule is unchanged (the witness only sets TERM)
@answer n=2 entity=cross-cutting-patterns#2.1 v=no
why: no claude-behaviour dependence, ledger row, probe or stamp was added or changed, and the session is test-only
@answer n=3 entity=cross-cutting-patterns#3.1 v=no
why: tests/contract_bounded_readers.rs is not reached and its census of product readers still holds, while the new OS-gated census binary checks gates, not readers
@answer n=4 entity=cross-cutting-patterns#4.1 v=no
why: no error body, writer or witness changed and tests/contract_error_bodies.rs is not reached; the nextest ci kill-line still names both binaries, since binary(run_cli) is only added to .config/nextest.toml
