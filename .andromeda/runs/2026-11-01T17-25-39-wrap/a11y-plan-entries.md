## 2026-11-01-unix-endpoint-and-home-hardening — a refused socket directory at the start: exit 1, revive's eleventh code
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: the CLI entity's `revive --json` codes, its start-refusals bullet, a new start entity); §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: the CLI pattern's fixed-message clause, a new start item; a11y-plan.md: the Pattern line)
**Change:**
- `revive --json`'s exit-1 codes: was ten; now eleven, `socket-dir-refused` after `squatted-name`, `internal-error` staying last.
- The start's refusals under `viola revive --json`: was `batch-script-child`, `pinned-hash-mismatch` and `squatted-name`; now `socket-dir-refused` too, before any snapshot or bind as well as before the child spawns.
- New entity, assertable on text and streams only: `viola run` and a `viola revive` start over a socket directory that fails its check, on Unix, exit 1 before any snapshot, bind or child, a failed create reading the same; the socket-directory pair on stderr, `hint:` last, plain ASCII, no SGR, naming only the parsed name; revive under `--json` its one `socket-dir-refused` document; the directory left as found. Source: `socket_dir_ready`, `refuse_socket_dir`, `send_hint_is_the_design_string`, `security_negative_a_start_refuses_the_socket_directory_before_any_bind`, `a_loosened_socket_directory_on_a_start_is_socket_dir_refused`.
- The fixed-message form (one `unable: <text>` line, one `hint:` line last): was the exit-1 refusals of `viola revive`, and in the master's Pattern `viola verify`'s pair too; now also `viola run`'s socket-directory refusal (`refuse_socket_dir`, on the start both verbs share), pinned by `a_loosened_socket_directory_on_a_start_is_socket_dir_refused`.
- New Error recovery item: exit 1 with that pair; under `viola revive --json` one document, `socket-dir-refused` its code, holding no path, pid or uid, stderr empty and no hint.
**Why:** The chunk checks the directory at the start, before any snapshot, child or `Server::bind`, and revive's start shares it. The new word is the founder's own (`inputs#I2` Q1), given live on 2026-11-01 before 16:27 Z through the overseer's dialog, after the widening was shown to him: the eleventh code of `revive --json` at exit 1. A failed create and a refused check reading one code is a lean of the overseer's P5 word as delegate, PROVISIONAL (`inputs#I3`). That these edits lie inside the founder's answers is the overseer's confirm on the answer card as his delegate, PROVISIONAL, on a word-filter read of its rows; the card's standing reads superseded by the founder's own word in the batch at Epoch 7's boundary, the ratification staying `inputs#I2` Q1 and Q2.
**Kept:** The pair's words and their status are not written in a11y-plan: the pass's disposition keeps the wording and its one gap marker at design-system surface.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — `server-verify-failed` reaches macOS, as an euid mismatch alone
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: the CLI entity, the plugin entity against an unverified server); §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: Verification, its cli and cli, macOS items; a11y-plan.md: Verification)
**Change:**
- CLI entity: was on macOS no peer pid is read and the refusal cannot occur (the recorded gap); now no peer pid is read there (the recorded gap, a named carve-out no route entry owns), so the refusal means an euid mismatch alone.
- Plugin entity: was off macOS, the check reading as passed there; now on every OS, on macOS such a server being one of another euid alone.
- Verification, cli: `tests/security_negatives_server_identity.rs` on windows-2025 and ubuntu-latest only now reads as a limit of its peer pid witness and not of the word.
- Verification, cli, macOS: was the server-verify refusal cannot occur, its cases compiling out, its macOS witness owed to "Unix endpoint and home hardening"; now the peer pid cases compile out, the refusal's decision and real read held by `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, `client_server_euid_is_this_user` and `tokio_client_server_euid_is_this_user`, and the socket-directory refusal's macOS witness is `tests/security_negatives_socket_dir.rs` on `macos-latest`; the owed clause is gone.
- The master's Verification: was the frame verbs' server-verify refusal on windows-2025 and ubuntu-latest only; now their server-verify peer pid witness there, the word on macOS an euid mismatch alone.
**Why:** The client euid check is built, so the clauses owed to that route entry are discharged; macOS's pid gap stays named, owned by no route entry. The reach is the founder's own word (`inputs#I2` Q2), given live on 2026-11-01 before 16:27 Z through the overseer's dialog, after the widening was shown to him: a client meeting a server of another euid reads `server-verify-failed` on Linux and macOS, on macOS an euid mismatch alone, the pid and start-time comparison staying absent, the named carve-out. That the Verification clause lies inside his answers is the overseer's confirm on the answer card as his delegate, PROVISIONAL, on a word-filter read of its rows; the card's standing reads superseded by the founder's own word in the batch at Epoch 7's boundary, the ratification staying `inputs#I2` Q1 and Q2.
**Kept:** macOS reads no peer pid; `tests/security_negatives_server_identity.rs` stays gated `not(target_os = "macos")`; the eight frame verbs' `error` word and exit codes are unchanged; the plugin entity stays not-assertable.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — a refused socket directory at the eight frame verbs: exit 21's sixth cause
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: a new frame-verb entity); §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: exit 21 under `--json`, a new frame-verb item)
**Change:**
- Exit 21's closed words under `--json`: was five; now six, `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed` and `socket-dir-refused`.
- New entity, assertable on text and streams only: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` over a socket directory that fails its check (mode open to another user, a symlink, not a directory, another owner), on Unix, in human TTY, plain and `--json` modes: refused before any frame with exit 21 and the socket-directory `hint:` line last on stderr, plain ASCII, no SGR, naming no path, pid or uid; under `--json` one document with `error` `instance-unreachable` and `detail` `socket-dir-refused`; the directory left as found; off Unix the check reads nothing. Source: `connect` in `src/cmd/client.rs`, `send_hint`, `security_negative_a_frame_verb_refuses_the_socket_directory_before_any_frame`, `a_loosened_socket_directory_on_a_frame_verb_is_socket_dir_refused`, `cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text`.
- New Error recovery item: the hint through `send_hint`'s cause `socket-dir`, `send`'s first line in its mirror's form; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"socket-dir-refused"}` on stdout with stderr empty and no hint.
**Why:** The check sits in the one `connect` the eight verbs call, before any frame. The word `socket-dir-refused` is the founder's own (`inputs#I2` Q1), given live on 2026-11-01 before 16:27 Z through the overseer's dialog, after the widening was shown to him: the sixth exit-21 cause on CLI and MCP. That these edits lie inside his answers is the overseer's confirm on the answer card as his delegate, PROVISIONAL, on a word-filter read of its rows; the card's standing reads superseded by the founder's own word in the batch at Epoch 7's boundary, the ratification staying `inputs#I2` Q1 and Q2.
**Kept:** The hint's words and their status are not written in a11y-plan: the pass's disposition keeps the wording and its one gap marker at design-system surface.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the tui boundary and the wrapper's drop of a refused peer
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: the `viola run` TUI passthrough entity's boundary clauses)
**Change:**
- New boundary clause: a peer the wrapper refuses is dropped in `accept_loop`, before its worker thread and any frame; it writes nothing to the outer terminal and moves no wheel (`serve_drops_a_refused_peer_before_any_frame`, `channel_endpoint_answers_protocol_faults` the control).
**Why:** The wrapper now filters each accepted stream by the peer's euid, and the a11y acceptance on that drop is met.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — hook and MCP on a refused socket directory: not-assertable
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: two new not-assertable entities)
**Change:**
- New entity: the `viola hook` event and dialog calls against a socket directory that fails its check, on Unix: no frame, fail open, exit 0, empty stdout and stderr, its one record a `hook-decision` at WARN with `detail` `socket-dir-refused`. Source: `connect`, `NotSent`, `Asked` in `src/cmd/hook.rs`, `security_negative_a_hook_fails_open_on_the_socket_directory`.
- New entity: the stdio MCP server's frame tools (`send`, `wait`, `last`, `answer`) against the same directory, on Unix: past the live endpoint `isError` with the code `instance-unreachable` and `detail` `socket-dir-refused`, no frame, no line, `isError` the record. Source: `frame` in `crates/viola-mcp/src/tools.rs`, `handlers_frame_tools_refuse_a_loosened_socket_directory`.
- Neither writes a human-facing byte or touches a human keystroke.
**Why:** The chunk puts the check on both seams, with empty streams at the hook and no line at MCP, so neither adds a surface an a11y assertion can read. The word `socket-dir-refused` is the founder's own (`inputs#I2` Q1), given live on 2026-11-01 before 16:27 Z through the overseer's dialog, after the widening was shown to him.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the client euid refusal's limit of proof
**Section:** §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: Verification, a new cli item)
**Change:**
- New item: no foreign peer is driven; the decision is tested over injected euids and its real read on Linux and macOS; each client seam's wiring is read only by forced refusals in remove-the-guard runs, which no committed test repeats.
**Why:** One account can make no foreign peer and no seam may be added, so a mutant replacing a seam's read with an always-admit survives every committed test by construction.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the socket-directory refusal's witnesses and literal pins
**Section:** §8 Cognitive Accessibility › Error recovery (cognitive-accessibility.md: a new Verification item; a11y-plan.md: Verification); §8 Cognitive Accessibility › Plain language (cognitive-accessibility.md: a new pin; a11y-plan.md: Verification)
**Change:**
- New item: the refusal has no Windows witness: `tests/security_negatives_socket_dir.rs` is `#![cfg(unix)]`, covering `ubuntu-latest` and `macos-latest`; off Unix the check reads nothing and passes (`check_socket_dir_off_unix_reads_nothing`).
- The master's Error recovery Verification adds the socket-directory refusal on ubuntu-latest and macos-latest beside its three-leg rule.
- New pin: the hint (`send_hint`'s cause `socket-dir`, `src/human/hints.rs`) by the unit `send_hint_is_the_design_string` and the literals `SOCKET_DIR` (`tests/cross_exit_causes.rs`) and `HINT` (`tests/security_negatives_socket_dir.rs`), with `no_hint_names_release`, `no_two_codes_share_a_hint` and `every_code_is_a_code_word` holding the catalogue.
- The master's Plain language Verification adds the socket-directory hint to the strings pinned by literal.
**Why:** The new binary's cases ran on the two Unix legs and none on Windows, and the check off Unix returns at once. The closed word `socket-dir-refused` is the founder's own (`inputs#I2` Q1), given live on 2026-11-01 before 16:27 Z through the overseer's dialog, after the widening was shown to him; the hint's wording is not part of that answer. That the pin lies inside his answers is the overseer's confirm on the answer card as his delegate, PROVISIONAL, on a word-filter read of its rows; the card's standing reads superseded by the founder's own word in the batch at Epoch 7's boundary, the ratification staying `inputs#I2` Q1 and Q2.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/
