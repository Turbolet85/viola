# security extract

## Relevance
partial — a test-and-restatement chunk with no new surface, but several survivor sites sit on security controls (the home-file replace helper, the board's strict-modes scan, the only shell-out, the wheel's stdin classifier, hook fail-open / dialog matching), so a restatement must not weaken them and a new test must not add a seam.

## Constraints
- A behaviour-preserving restatement of `viola_state::fs::replace_private_with` (`fs.rs` survivors) must keep the one-helper contract: the temp file's mode set explicitly (0600, or 0700 for the pinned `viola`) before any byte is written, then `sync_all` + `persist`, and a write whose mode cannot be set is discarded; if the restatement hands the Windows retry decision to the caller, no caller may reach a path that skips the mode step (per security-plan §Data Protection, `~/.viola/` access control row).
- The `board.rs` `check_if_present` / `scan` NotFound guards implement "each that exists — absent passes, an unstattable one is refused" for `viola list`'s strict-modes scan, whose first failure refuses the whole board with exit 21; any restatement must keep a non-NotFound stat error a refusal, never a pass (per security-plan §Data Protection, `~/.viola/` access control row, `list` clause).
- `statusline::shell_argv` is the only shell-out: `/bin/sh -c` with the command as exactly one argument on Unix, `None` for every input on Windows; a restatement or test must not add a second shell-out or change the argv shape (per security-plan §Input Validation, Statusline shell-out row; §Security Anti-Patterns ### Code Patterns, "statusline_command is the only shell-out").
- The `wheel.rs` `Classifier` / `is_reply` survivors sit on the stdin classifier's closed non-editing list: only an exact-grammar listed sequence never counts as typing, every other byte does, and the nearest human keys stay typing; a restatement must not widen the list or relax a grammar, field count or bound (per security-plan §Threat Model Summary, human-stdin entry point / trust boundary bullets).
- A test-only seam a new test needs stays behind `cfg(test)` or the existing test features; no new env-var seam beyond `FAKE_AGENT_PUMP_DELAY_MS` / `FAKE_AGENT_HOOK_PANIC` without its own Decisions Log entry, and no test-only feature may reach the release build (per security-plan §Security Anti-Patterns ### Universal, the two-carve-outs ban; per security-plan §Input Validation, the two test-seam rows).
- The `hook.rs` `answer` and `dialog.rs` `DialogSlot::hook_dialog` survivors sit on dialog answering: a non-`null` decision only under a strict-modes-read verify stamp, and `viola hook` always exits 0 with no stderr; whether the mutated conditions guard either rule is research's question, and a restatement must keep both (per security-plan §Security Anti-Patterns ### Universal, the two `hook.dialog` bans; per security-plan §Security Anti-Patterns ### Logging, the `viola hook` stderr / exit ban).
- The `viola-e2e` `scratch.rs` `prepare` survivors sit on the mutation-scratch guard: only `<repo parent>/viola-mutants-scratch` is ever wiped, re-created owner-only through `create_private_dir`, never a plain `create_dir_all`, never a `%TEMP%` / repository fallback, never the path printed, and only on a Windows host (per security-plan §Security Anti-Patterns ### Code Patterns, the `run --mutants` wipe ban).

## Patterns to follow
- Home files written only through `viola_state::fs::replace_private` (and `create_private_new` / `create_private_dir`), with a test asserting the resulting mode rather than trusting the umask (per security-plan §Data Protection, `~/.viola/` access control row).
- Strict-modes refusals proven by `cfg(unix)` cases that set an offending mode on a fixture home and read the refusal (per security-plan §Data Protection, `~/.viola/` access control row, `revive` clause).
- The classifier's listed replies each pinned by a unit case beside their nearest human keys, one field or one prefix away kept typing (per security-plan §Threat Model Summary, human-stdin trust boundary bullet).
- A test driving `viola verify`'s Run B framing uses a fake PTY and compiled literals only, never a live `claude` child or a byte typed into a dialog (per security-plan §Input Validation, `viola verify` interactive children row as summarised in `.claude/rules/security.md`).

## Anti-patterns to avoid
- NEVER let a test-side removal under `target/e2e-home` take anything but a directory the test side itself created, and NEVER loosen the strict-modes check to make a test home pass (per security-plan §Security Anti-Patterns ### Code Patterns, the test-side removal ban and the scratch ban).
- NEVER add another env-var seam, or let `fake-agent` / `test-support` reach a release build (per security-plan §Security Anti-Patterns ### Universal; per security-plan §Dependency Security, `release-check`).
- NEVER commit a fixture or test literal holding a home path, username, real prompt, token or R8-stripped `CLAUDE*` value (per security-plan §Security Anti-Patterns ### Data Protection, the fixtures ban; ### Secrets, the floor ban).

## Contract bindings
- security ↔ obs/tests: new test output lands in the nextest JUnit report, which the CI `test` job's `viola-harness secret-scan` canary checks against the NEVER-log floor before any upload (per security-plan §Bootstrap phases, `logging-redaction-wire`).
- security ↔ tests: the `fs.rs` sites stay bound to the "Paste newline ledger row" CARRY and a Windows-only site's record is owed to a route entry; `windows-mutants.yml` stays dispatch-only, never a gate (per security-plan §Dependency Security, CI integration bullets).

## Acceptance criteria contributions
- `bash scripts/release-check.sh` (with `--probe` 5/5) still passes: no test-only feature or binary in the release build after the chunk's tests and seams land (per security-plan §Dependency Security, release-check).
- `git diff` of the chunk adds no `std::env::var` / `var_os` read outside the two named seams, no `Command::new("sh"|"bash"|"cmd")` beyond `shell_argv`, and no `create_dir_all` in a home or scratch path (per security-plan §Security Anti-Patterns ### Universal; ### Code Patterns).
- Any `fs.rs` / `board.rs` restatement keeps its existing mode and refusal cases green, and a case shows an unstattable (non-NotFound) home or `instances/` refused by `list`'s scan — whether one already exists is research's question (per security-plan §Data Protection, `~/.viola/` access control row).
- `cargo deny check` passes unchanged: the chunk adds no dependency, ignore, skip or allow (per security-plan §Dependency Security).
