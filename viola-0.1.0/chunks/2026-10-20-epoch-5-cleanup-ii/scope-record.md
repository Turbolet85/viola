# Scope record — 2026-10-20-epoch-5-cleanup-ii

<!-- in place of the listed crates/viola-agent-claude/src/ledger/tests.rs: cargo-modules 0.27 reads an out-of-line test module file as an orphan unless its stem begins test_ -->
- `crates/viola-agent-claude/src/ledger/test_cases.rs` · companion · serves step 3 · word: "#[path] test_cases.rs: `#[cfg(test)] #[path = \"ledger/test_cases.rs\"] mod tests;` keeps every test path `ledger::tests::…`. Its scope-record line carries this word." — the Viola overseer, 2026-10-20, relayed verbatim by the operator seat (inputs#I5, dialog 15 Q2)
