@history_item n=1 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (a peer's refusal screened to a code word) `reply_of` / `of_reply` screen a peer's reply with `viola_core::is_code_word`; a refusal that is no code word reads `unknown`, a string detail that is no code word reads nothing, and a code word outside the closed list stays visible by design. This is the precedent behind decision/peer-replies-screened-to-code-words and contract/code-word/format: each of the nine `cause` words must pass `is_code_word`, and a value outside the map collapsing to one word (`api-error`) mirrors the `unknown` fallback.

@history_item n=2 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (`unknown`'s detail, a code word or nothing) `unknown`'s detail changed from opaque free text to a code word or nothing, still with no hint. This is the rule that no upstream text crosses beside a word. The chunk's boundary on `last_assistant_message`, `error_details` and the raw `error` value extends it to `turn-ended`'s `cause`.

@history_item n=3 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the `--json` failure documents' witness, the exit-2 exception, the fault body, the census of error writers) `cli_error_documents_hold_no_path_or_upstream_text` holds each `--json` failure to one document with a literal exit, empty stderr, no ESC and a clean body. The exit-2 paths are the named exception. `sanitised_fault` reduces an unknown fault to `wrapper fault`. `tests/contract_error_bodies.rs` holds the writer census and `assert_error_body_clean`. These are the witnesses behind contract/cli-json-failure/document and decision/json-failure-documents-are-clean, and the checks that `wait --json` carrying the new causes must still pass.

@history_item n=4 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (exit 21's `--json` document names its cause; MCP's `isError` body names its cause) One opaque failure was split into a closed set of cause words carried in `detail`, with `error` and `v` unchanged so a driver can tell causes apart. MCP's body is the CLI's document without `v`, so a cause crosses to CLI and MCP together. This is the direct precedent for widening `turn-ended`'s one `api-error` into nine words on `wait --json` and MCP `wait` at once. It was the founder's word given after the widening was shown, and it went through check-7 rows as PROVISIONAL.

@history_item n=5 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (a lost wrapper and a stale holder each read their own code; the exit-cause table and one hint per code) The overloaded words were split: `endpoint-lost` was split from `instance-dead`, and `stale-heartbeat` from `already-live`. `tests/cross_exit_causes.rs` became a `fake-agent` E2E table with one row per cause, checking human and `--json` modes, and `every_code_is_a_code_word` holds the catalogue. This is the pattern for proving the nine causes on the fake agent with no live start, and for holding each new word to the code-word format.

@history_item n=6 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — The PROVISIONAL records of 2026-10-31-sanitised-error-surfaces and 2026-10-31-exit-cause-code-catalogue were superseded by the founder's own word, and the tier stays `Standard (1)`. The precedents in items 1–5 therefore stand ratified, and decision/a11y-tier-standard is not re-opened.

@history_item n=7 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (exit 21's sixth cause; hook and MCP on a refused socket directory, not-assertable) A closed cause set grew by one founder-given word (`socket-dir-refused`) on CLI and MCP. The MCP and hook paths stayed not-assertable because they write no human-facing byte. This is a precedent for growing a closed word set, and for decision/no-assertions-for-plugin-and-ipc covering the MCP `wait` side of the new causes.

@history_item n=8 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the tui boundary and the wrapper's drop of a refused peer) Added the boundary clause that a peer the wrapper refuses is dropped before its worker thread and any frame, writes nothing to the outer terminal and moves no wheel. This is the origin of contract/tui-passthrough/refused-peer. A wrapper-side change to the appended-line check for `turn-ended` must keep the outer terminal untouched.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (revive's start refusals without batch-script-child) A closed word left the set with its emitter by the overseer's answer, recorded as a narrowing of the scope's «no closed word changes» boundary. This is a trap for this chunk: any change to a closed set beyond the founder's nine words (add, rename, drop) breaks the CARRY's «not re-opened» boundary.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the a11y cases' CI home, the one ubuntu-latest leg) The cli and tui output-discipline cases run in the `test` job's `coverage` suite on `ubuntu-latest` alone, gated by `gate --require coverage,doctest`. This is where the guards of decision/word-beside-every-sgr, decision/no-sgr-without-tty and decision/c0-c1-escaped run, and where any new case of this chunk lands.

@history_item n=11 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — The browser a11y leg was removed. The cli and tui cases run as nextest cases in the `coverage` suite with no a11y driver or artifact of their own, and §11 bans a separate a11y driver or `suite` value. This is the basis of decision/output-discipline-a11y-fallback and decision/no-a11y-tool-output-discipline-assertions: the chunk adds no a11y tool and no new suite.

@history_item n=12 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — The web entities left 0.1.0, and the terminal surfaces (CLI output discipline, the TUI boundary, the manual terminal passes) stand. The tier `Standard (1)` now rests on the terminal surfaces alone. This is the ground for decision/a11y-tier-standard and the absences (web or graphical frontend, rendered page).

@history_item n=13 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (cli checks on std::process::Command and literal pins; §8 verification lines pinned by literal; the cli tooling in the a11y testing contracts) trycmd and assert_cmd were reworded to `std::process::Command` with literal pins: every fixed CLI string is pinned by literal, the typed exits and the hint-free `--json` are asserted, and the tool pick reads so. This is the current form of decision/fixed-cli-strings-pinned(-by-literal) and decision/cli-a11y-by-literal-tests. Any fixed text the nine causes add to `wait` or the board takes a literal pin.

@history_item n=14 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — Built the colour spans: bold NAME, amber only on `DIALOG`, dim on stale rows, each beside its word, and none under `--json`, a pipe, `NO_COLOR` or `TERM=dumb`. `viola wait` and `viola --help` carry no ESC and no lone CR. This is the origin of contract/viola-list-board/colour, decision/word-beside-every-sgr and decision/no-sgr-in-plain-modes. A cause word shown on the board or in `wait` stays a plain word, never colour alone.

@history_item n=15 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — Pinned the board's caption order (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI) by the `CAPTION` literal, and the row escape (ESC, C1, DEL, `\n`, `\t` → `\xHH`) by units on a recorded CLI version. This is the basis of decision/critical-path-cli-board-output (fixed captions and slot order) and decision/control-bytes-escaped. The board as a consumer of `turn-ended` keeps its captions and slots.

@history_item n=16 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — Unwrapped NAMEs are printed through `escape_row` on one line, and the STATUS words `idle` / `busy` and the `n/a` cells are printed words with no SGR. This is the list-row escape behind decision/control-character-escaping's exception and decision/state-carried-by-printed-word.

@history_item n=17 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (6. Visual Design Verification; 8. Cognitive Accessibility) `~ decision/state-carried-by-printed-word` dropped `budget-paused` from the board's printed state words. `~ contract/cli-exit-codes/11`: a retired word from an older wrapper reads `unknown` (exit 14), never reused. This is the latest edit to a picked row, and the precedent for an unmapped value collapsing to the catch-all word: here, an unmapped `error` reads `api-error`.

@history_item n=18 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — Recorded the first live reading of the takeover clause, supplemental and never gating. This is the origin of decision/live-readings-never-gate, consistent with this chunk's directive of no live `claude` start: the fake agent and recorded payloads carry the proof.

@history_item n=19 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — Added the second live takeover reading, on a live `claude` 2.1.296, still supplemental and never gating. This reinforces decision/live-readings-never-gate: a mapping that can only be honest with a live reading is named as a probe, not made a gate.

@history_item n=20 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — The CLI output-discipline invariants meet mutation only at the epoch-boundary audit (`run --mutants`), not per chunk. This matches the directive's «no mutation run» for this chunk.

@history_item n=21 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — A passed `viola revive` enters the TUI passthrough boundary and the zero-SGR list. This is the origin of decision/single-vendor-content-boundary and decision/no-sgr-in-plain-modes naming revive, and it bounds where a `turn-ended` cause may surface (never on the outer terminal).

@history_item n=22 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the plugin against an impostor wrapper writes no human-facing byte) This is the first detail on the not-assertable plugin entity: hook calls fail open and MCP frame tools (`wait` among them) return `isError` with a code word only, writing no human-facing byte. This is the precedent for decision/no-assertions-for-plugin-and-ipc as it applies to MCP `wait` carrying the new cause words.

@history_item n=23 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — The master's text became rows (decision · contract · absence), each verified against the code. This is the origin of every picked row's handle, so the chunk's a11y edits are row amendments (`+` / `~` / `−` by handle).
