# Classified — a11y-plan · a11y-scope-summary.b1

## Answers
a11y-scope-summary#1.1 · no · the reached files changed only in test code, and the report shows no human output, refusal word, exit code or SGR byte moved, so the verb output claims hold
a11y-scope-summary#2.1 · no · src/run/gate.rs and src/cmd/run.rs changed only in #[cfg(test)] captures and the one global subscriber install is untouched, so no byte reaches the terminal and none newly falls on the human
a11y-scope-summary#3.1 · no · Cargo.toml gained only a Windows dev-dependency table, no a11y tool version or CLI string moved and the run JSON shape is unchanged, so the tool reach and the multi-platform claim hold
a11y-scope-summary#4.1 · no · the run-path files changed only in #[cfg(test)] captures, the one global subscriber install is untouched and the Windows PTY path is not reached, so zero terminal bytes and run diagnostics kept in diagnostics/ still hold
a11y-scope-summary#5.1 · no · viola list's code and output were not reached and no fixed CLI string changed, so the caption line, the UNWRAPPED separator and the --help group labels stand
