# Task — a11y-plan · 2026-11-01-unix-endpoint-and-home-hardening

## The session
Added a per-user socket directory check on Unix (symlink, not a directory, foreign owner or foreign mode bit refused, never chmodded) at `viola run`, `revive`'s start, the eight CLI frame verbs, `viola hook` and MCP's `frame`, reading the new closed word `socket-dir-refused`.
Added a peer euid check on both sides of the channel: a Unix client meeting a server of another euid reads `server-verify-failed`, and the wrapper's `accept_loop` drops a refused peer before any worker thread or frame, writing no line and nothing on the terminal.
Moved `viola list`'s home to the read rule alone on every OS, provisional on the overseer's delegated answer; `mcp`'s start, `plugin install` and `read_budget_stamped` keep the write rule.
Added one refusal pair and one test binary, `tests/security_negatives_socket_dir.rs`, compiled on Unix only and green on `ubuntu-latest` and `macos-latest`; `cleanup`'s `endpoint_gone` on Unix now reads a connect refused or a path absent.
Retired the dated owner "Unix endpoint and home hardening": every clause it owed is discharged, except that macOS's peer pid gap stays named and now has no route entry as owner.
Measured: CI `ci#41329861618` green on `c4bf0a92`, 15 of 15 checks; the hook perf rows stayed under `SPINE_DEADLINE` and no perf value moved.

## Reached files
- crates/viola-core/src/lib.rs
- crates/viola-state/src/strict.rs
- crates/viola-state/src/board.rs
- crates/viola-channel/src/lib.rs
- crates/viola-channel/src/client.rs
- crates/viola-channel/src/tokio_client.rs
- crates/viola-channel/src/server.rs
- crates/viola-mcp/src/tools.rs
- src/cmd/run.rs
- src/cmd/client.rs
- src/cmd/hook.rs
- src/human/hints.rs
- schemas/diag-line.v1.json
- crates/viola-e2e/src/harness/cleanup.rs
- tests/security_negatives_socket_dir.rs
- crates/viola-e2e/tests/harness_lifecycle.rs
- crates/viola-mcp/tests/handlers.rs
- tests/support/home.rs
- tests/cli_list.rs
- tests/cross_exit_causes.rs
- tests/contract_error_bodies.rs
- tests/cli_error_bodies.rs
- tests/cli_controls_not_disableable.rs
- tests/hook_fail_open.rs
- tests/security_negatives_server_identity.rs
- Cargo.toml
- .config/nextest.toml

## Touches
- `revive --json`'s exit-1 codes moved from ten to eleven, the eleventh `socket-dir-refused`: a row stating the count of ten or listing all ten changes; a row naming `batch-script-child` as one code among others does not.
- Exit 21's closed causes moved from five to six: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, `socket-dir-refused`. A row stating five or listing the five changes; the exit set itself is unchanged.
- `server-verify-failed`'s reach moved from off macOS to Linux, Windows and macOS, where on macOS it means an euid mismatch alone: any row reading that word as absent on macOS changes.
- The verification clause that limits the frame verbs' server-verify refusal to `windows-2025` and `ubuntu-latest` because macOS reports no peer pid: the peer pid witness stays off macOS, since the server-identity negatives remain compiled out there, but the clause no longer covers the word's whole reach.
- macOS's peer pid gap, the mac-gap pattern: a row stating macOS compares no peer pid stays true; its recorded gap wording is kept or read as the named carve-out, and any owner it names changes because no route entry owns it now.
- The dated owner "Unix endpoint and home hardening", the owner-title pattern: a row reading the client euid check, the socket directory check, the macOS witness or their code as owed to it changes, because each is built.
- The plain-language verification's list of strings pinned by literal: it names the server-verify, `unwrapped-name` and `endpoint-lost` hints and the new socket directory hint is not among them.
- The error-recovery rows on the refusal shape and the all-three-legs witness: the socket directory refusal has the plain ASCII, `hint:` last, no SGR and hint-free `--json` shape, but its integration witness runs on two OS legs.
- The error-recovery row on which refusals print one `unable: <text>` line: `run`'s and `revive`'s start now print the new pair in that form, reached through `refuse_socket_dir`.
- The tui boundary entity and its zero-bytes claim, through the wrapper's new drop in `accept_loop`, which writes nothing to the outer terminal and moves no wheel.
- The `viola revive --json` row on every refusal of the start it shares with `run`: `socket-dir-refused` joins those refusals, printed as one document before any snapshot, child or bind.
- The not-assertable plugin and wrapper IPC entity, through the new check at `viola hook` and MCP's `frame` and the peer euid check on the channel: it stays not-assertable, and only a row describing what those paths refuse or emit moves.
- A row naming the sibling `security_negatives_server_identity` as the security negatives' witness, the neg-binary pattern: it stays true of itself, and a row meant to name every such binary extends with `security_negatives_socket_dir`.
- `human::refuse(` under `src/` moved from 18 to 19 and `run`'s from 7 to 8: any row carrying a refusal census moves.
- `viola list`'s home moved from two rules to the read rule on every OS: a row stating the rule `viola list` applies to its home changes, while a row stating strict-modes has two rules stays true because both rules still stand.

## New facts
- F1 · cognitive-accessibility · The fixed pair `unable: the socket directory for <name> cannot be trusted` and `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions` is a planning form, its wording owed to the founder at Epoch 7's boundary batch.
- F2 · cognitive-accessibility · The new hint is pinned by literal through `send_hint_is_the_design_string`, with `no_hint_names_release`, `no_two_codes_share_a_hint` and `every_code_is_a_code_word` over the catalogue and hint constants in `tests/cross_exit_causes.rs` and `tests/security_negatives_socket_dir.rs`.
- F3 · cognitive-accessibility · On the eight CLI frame verbs a refused socket directory exits 21 with `socket-dir-refused` and no frame, reaching the hint through `send_hint` cause `socket-dir`, with `send`'s first line printing its mirror's form.
- F4 · cognitive-accessibility · At `viola run` and `revive`'s start a refused socket directory exits 1 with the new pair before any snapshot, child or bind, and `--json` prints one document holding no path, pid or uid.
- F5 · cognitive-accessibility · The socket directory refusal has no Windows witness: `tests/security_negatives_socket_dir.rs` is `#![cfg(unix)]`, covering `ubuntu-latest` and `macos-latest`, and the check off Unix reads nothing and passes.
- F6 · cognitive-accessibility · The client euid refusal has no driven foreign peer: its decision is tested over injected euids, the real read on Linux and macOS, and each seam's wiring was read only by this run's forced refusals, which no committed test repeats.
- F7 · a11y-scope-summary · The tui boundary now covers the wrapper's drop of a refused peer in `accept_loop`: nothing on the outer terminal, no wheel movement, witnessed by `serve_drops_a_refused_peer_before_any_frame` with `channel_endpoint_answers_protocol_faults` as control.
- F8 · a11y-scope-summary · `viola hook` meeting a refused socket directory exits 0 with empty streams and no frame, its one record a `hook-decision` at WARN, so it adds no a11y surface.
- F9 · a11y-scope-summary · MCP's `frame` meeting a refused socket directory sends no frame and writes no line, its record the `isError` result, so it adds no a11y surface.

## Untouched
- The tier, the no-conformance claim, the compliance cross-check and the not-triggered list (wcag-mapping, regulated-compliance, cognitive, motion, multi-language, target-size): the session names no regime or trigger and adds no such surface.
- The structural equivalents in aria-landmarks (the BAY context line, the caption line, the unwrapped separator, the `--help` group labels): `viola list`, `viola send` and `viola --help` keep their columns, captions and rows.
- The `CAPTION` literal and caption order in cognitive-consistency and path-p6-board: those pins were not edited, and the new `viola list` case is a refusal, not a board row.
- The SGR rules, NAME cutting and escaping in visual-sgr: the new pair carries no SGR and only follows them, so no rule of that group moves.
- The owed clauses under "Linux and macOS parity" (dim on stale rows) and "Windows-only live measurements" (the vt100 feed panic's byte reading): other owners, not discharged here, however close a macOS or Unix row reads.
- The manual NVDA, VoiceOver and Orca passes and their founder ownership: no manual pass ran or moved, and no live `claude` ran.
- Reading level and no readability scorer in cognitive-plain-language: SC 3.1.5 stays unclaimed; the new copy adds no gate.
