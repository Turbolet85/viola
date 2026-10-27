# Task — a11y-plan · 2026-10-27-pinned-copy-race-and-reproducible-builds

## The session
The pinned copy's absent-copy write (`bin/<version>-<hash>/viola`) is now a no-clobber placement of a closed temp file (`viola_state::fs::place_private_new`, `Placed { Written, Present }`), never a replace. A concurrent first start's agent can no longer exec the copy while another start holds it open for write (`ETXTBSY`): 4 of 24 two-instance boots failed before, 0 of 24 after.
A start that loses the placement re-hashes the copy that stands: equal bytes are reused, different bytes are the existing `PinError::HashMismatch`. No new refusal, `detail` code, log line, field, span, config key, flag or human output; `run.pin_copy` keeps `ok|pinned-hash-mismatch`, a lost placement reading `ok`.
Test-only: the fake agent's `hook` receipt gains `spawn_os_error`; Path 7's MCP-leg case `mcp_tools_path7_unverified_cli_answer` now boots the default unstamped `overseer` + `builder` pair instead of `builder` alone (the race workaround retired); the Windows plain-home oracle reads the inherited untrusted write grant by its right, whichever SID holds it, not by the Users SID.
Measured: two clean release builds of one tree give two files whose loaded sections are equal (`.note.gnu.build-id`, `.symtab`, `.strtab` differ), cause not explained, a founder item. Path 1's CI timeout red recorded as runner variance under a PROVISIONAL WATCH; the 20 s kill line not moved. CI `ci#40439188053` green 15/15 on three OSes; `tui_passthrough` green inside pre-push's coverage suite.

## Reached files
- crates/viola-state/src/fs.rs
- crates/viola-state/src/pin.rs
- crates/viola-state/src/strict.rs
- src/bin/viola-fake-agent.rs
- crates/viola-e2e/tests/mcp_tools.rs
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/preconditions.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/repro-unstamped.py
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/race-before.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/race-after.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/red-green.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/repro-builds.sh
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/reproducible-builds.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/path1-phases.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/operator-pass.md

## Touches
- `.andromeda/a11y-plan.md` §1 critical path 3 (dialog pending, then `answer`, unverified-CLI withholding), its source line citing Tests Critical Path 7: the report's `path7-pair` sweep read it as no-change; Path 7's MCP leg now runs on the default unstamped pair, and the `unverified-cli` withholding claim is unmoved.
- `.andromeda/a11y-plan-detail/cognitive-accessibility.md`'s tampered-copy refusal text (the report's `pin-copy` sweep, the cognitive-errors group): re-read no-change; the pinned-hash-mismatch refusal keeps its words, hint and exit, and is now also what a start reads when a concurrent first start placed different bytes first.
- Error recovery (§8, cognitive-errors): no refusal, `detail` code or `hint:` line is added or changed; a lost placement is a normal start with no output, so the per-refusal `unable` + `hint:` inventory and its trycmd pins stand.
- The tui boundary (§1 scope-surfaces, §4 P4, path-p4-wheel): `viola run`'s start, the entry to the passthrough, changed only in how the pinned copy is written; zero viola terminal bytes holds, `tui_passthrough` green inside the coverage suite.
- `viola run` and a passed `viola revive` entering the tui (scope-entities): two concurrent first starts on one fresh home now both reach a ready child; no change to what either writes to the terminal.
- Strict-modes refusal of a plain home on Windows (the state-files `unable:` / `hint:` pair, if cognitive-errors names it): only the test oracle changed, from the Users SID to the write right on any untrusted SID; the product verdict and its text are unchanged.
- CI integration (§9) and the coverage gate (§10): the new `viola-state` unit cases (`fs::tests::place_private_new_*`, `pin::tests::place_absent_*`) and the changed Path 7 case run inside the existing suites on all three OS legs; no new `suite` value, no a11y artifact, `gate --require coverage,doctest` green.
- Zero-flakiness parallel (§10): Path 1's red is recorded as runner variance under a WATCH with no retry added and no kill line moved; the `retries: 0` rule stands.
- Timeout extensions (§8, cognitive-timeouts): Path 1's 20 s kill line is a test bound, unchanged; no new time bound falls on the human.

## New facts
- (none)

## Untouched
- The `viola list` `CAPTION` literal, caption order, unwrapped separator and STATUS words: `list` is not in this session's change.
- SGR rules (amber `DIALOG`, dim stale rows, bold NAME, no SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run`): no output styling changed.
- The plain-language fixed copy set and the literal pins for `--help`, `plugin install`, the server-verify hint and `link` / `unlink`: no string changed.
- `viola revive --json`'s owed document after a passed preflight (owed to "Exit-cause code catalogue"): not reached.
- The dim-on-stale outer-PTY witness owed to "Linux and macOS parity": not reached.
- The Windows win32-input-mode mouse report taking the wheel (F-W3) and the wheel rules: the wheel is not in the change.
- The manual NVDA / VoiceOver / Orca passes and their surface table: unchanged.
- The deferred 0.2.0 frontend and its undecided technology: not reached.
- The not-assertable plugin / MCP / IPC entities: the MCP leg's session shape is a test change only; no MCP tool, result or code changed.
- Motion (no spinners) and the not-triggered triggers: nothing added.
- Measurement B's two build files: a build-keying fact for architecture and security, with no human-facing output.
