# Task — design-system · 2026-11-01-unix-endpoint-and-home-hardening

## The session
Added a verified per-user socket directory check: a directory that is a symlink, not a directory, foreign-owned or open to another user is refused and left as found, at `viola run`, `revive`'s start, the eight CLI frame verbs, `viola hook` and MCP's `frame`.
Added a new closed word `socket-dir-refused` and a new fixed-message refusal pair for it, a planning form whose final wording is owed to the founder.
Added the peer euid check on both sides: the wrapper drops a refused peer silently before any frame, and every client on Linux and macOS reads a server of another euid as `server-verify-failed`.
Changed `viola list` to hold the home to the read rule alone on every OS, retiring its two rules on the home; `mcp`'s start, `plugin install` and `read_budget_stamped` keep the write rule.
Retired the dated owner "Unix endpoint and home hardening": each owed clause is discharged, and macOS's missing peer pid comparison stays a named gap owned by no route entry.
Measured: the exit set is unchanged, CI is green at `c4bf0a92`, and hook timing stays under `SPINE_DEADLINE` with no recorded value moved.

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
- Exit-code phraseology, row 21: the closed causes move from five to six, and any list of them extends with `socket-dir-refused` after `server-verify-failed`.
- Exit-code phraseology, row 1: the fixed-message `unable:` refusals with a hint, and the `--json` document of `revive`'s start, gain the socket-directory cause.
- Exit-code phraseology, row 21: the frame-verb form of the new cause reads `unable  <name>  instance-unreachable` then its hint, and is not one of the strict-modes `unable:` / `hint:` pairs.
- Component pattern 2, `viola send`: the sentence stating exit 21's document names one of five closed words now reads six.
- Component pattern 2, `viola send`: the hints and per-cause lines gain the `send_hint` cause `socket-dir` and the new start refusal's line.
- Component pattern 2, `viola send`: the frame-verb refusal on a socket directory prints in the mirror's form under `send`, unlike the strict-modes pair that replaces the `[/ ]` mirror.
- `revive --json`'s exit-1 codes move from ten to eleven: any statement of ten and any list holding all ten extends with `socket-dir-refused`.
- A row naming `batch-script-child` as `run`'s other code or as a refusal's own word does not change with the revive count.
- The D-20 sentence of Surface: cli, the report's named site of this chunk's amendment, is read against the new cause, the new codes and the owner's retirement.
- Two pending records in Surface: cli are named by the report as carried: read each against the new pair's owed wording and the provisional board rule.
- Dated owner "Unix endpoint and home hardening": each design-system row reading that owner's check or code as owed now reads it built, and its owed clause leaves.
- `.andromeda/design-system-detail/surface.md:46`, `viola list`'s as-built row: the home is now held to the read rule alone on every OS, and the whole board is still refused with exit 21.
- `viola list`'s refusal rows: a home another user may read now refuses the board with or without an instance, on Linux, macOS and Windows; the order and the no-open rule hold.
- Any statement that `viola list` applies two rules to the home changes; a statement that strict-modes has a write rule and a read rule stays true.
- `server-verify-failed`'s reach moves from off macOS to Linux, Windows and macOS, on macOS meaning an euid mismatch alone: any row limiting the word changes.
- A refused peer euid has no word of its own: any row describing `server-verify-failed` as only a pid or identity mismatch widens to carry it.
- macOS's missing peer pid comparison: a row stating it stays true, but any row giving it an owner in a route entry changes, since it now has no owner.
- `run` and `revive`'s statement that a start refusal shows no path or pid now also covers the new pair, which names only the parsed `ViolaName`.
- `run`'s rule of printing nothing once the child starts holds through the new refusal, which comes before any snapshot, child or bind, and through the wrapper's silent drop.
- The cli bans on upstream text, paths, pids and stack traces, and the rule that no hint names `viola release`, reach the new pair; `no_hint_names_release` covers it.
- The rule of one `hint:` line per cause, printed last and not under `--json`, gains the `socket-dir` cause; `no_two_codes_share_a_hint` and `every_code_is_a_code_word` cover it.
- `--json` stays one document with stderr empty and exit 21's cause in `detail`: `socket-dir-refused` is a new value of that field, and no new document shape.
- `hook`'s note, nothing on stderr and always exit 0, holds through the new refusal, which leaves both streams empty and sends no frame.
- `mcp`'s note, MCP frames only on stdout, holds through the new refusal, whose record is `isError` with no line written.
- The exit set is unchanged, so a row listing the exit codes themselves holds while the causes under 21 move.
- Indirect, through `schemas/diag-line.v1.json`: `process-exit`'s `detail` moves from 19 to 20 members and `hook-decision`'s from 7 to 8, if any group states those counts.
- Indirect, through `src/cmd/run.rs`: `human::refuse(` under `src/` moves from 18 to 19, and `run`'s from 7 to 8, if any group states those counts.
- Indirect, through added lines: any citation of a line in a reached file may have moved, chiefly in `src/cmd/run.rs`, `src/cmd/client.rs`, `src/cmd/hook.rs` and `src/human/hints.rs`.
- Indirect, through the new binary: a test inventory naming `security_negatives_server_identity` or the hint catalogue's cases now has `security_negatives_socket_dir` beside it.

## New facts
- F1 · surface · `viola run` and `revive`'s start refuse an untrusted socket directory at exit 1 with `unable: the socket directory for <name> cannot be trusted` then `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
- F2 · surface · The new pair's wording is a planning form: the founder's own wording is owed at Epoch 7's boundary batch.
- F3 · surface · `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` meeting an untrusted socket directory exit 21 before any frame, with `instance-unreachable` and the hint of cause `socket-dir`, or under `--json` the document with `detail` `socket-dir-refused` and stderr empty.
- F4 · surface · Under `revive --json` a socket-directory refusal prints one stdout document with the code `socket-dir-refused`, no hint and stderr empty; a failed create of the directory and a refused check read that one code.
- F5 · surface · The wrapper's drop of a refused peer prints nothing on the terminal the child holds, writes no line and moves no wheel; the client reads end-of-file.
- F6 · surface · `viola list`'s read rule on the home on every OS stands PROVISIONAL on the overseer's word as delegate, the founder's own word owed at the same Epoch 7 batch.

## Untouched
- Color Palette's core colors, surface scale, text hierarchy, border progression and contrast rules: the new refusal is plain ASCII with no SGR, and no value moved.
- Semantic and domain status colors: a 0.2.0 frontend input, with no site of this chunk's amendment there.
- Typography, Spacing, Depth Strategy, Border Radius and Iconography: 0.2.0 frontend inputs and terminal grid rules, none moved.
- Motion and the expression level: no motion was added; the wrapper's drop is silence, not a visual state.
- Brand Identity's anchors, the readback box and the cocked strip: their words and drawn states are unchanged, and no readback rule moved.
- Surface: web-spa: the frontend stays deferred to 0.2.0.
- CLI colour tokens, the colour decision order and the depth choice: the new lines carry no styling.
- `viola list`'s columns, captions, BAY line and row order: the home rule changed, while the layout and its literal pins did not.
- `viola --help`'s rows and the flat verb list: no verb was added, removed or regrouped.
- `viola verify`'s step counter and its refusals: no check or line of its own moved.
- `viola plugin install`'s output lines and its home-state refusal: it keeps the write rule on the home.
- Self-Validation Protocol, including the Token Test: no token or validation step moved.
- Git Bash and conhost notes: their warning and no-colour fallback are outside this chunk.
- Universal Bans and Rejected Defaults: the session adds no colour, icon, spinner or sort order.
