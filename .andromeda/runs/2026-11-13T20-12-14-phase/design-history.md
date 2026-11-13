@history_item n=1 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows (243 records to 344 rows), each verified against the code, with divergences marked "as the code shows", "as written" or "as written, owed by the route". Every rows-file handle this chunk closes (`contract/refusal-hints/*`, `contract/viola-verify-refusals/unclean-fixture`, the strict-modes rows) comes from this landing. Its route-owed lines from the 2026-11-13T01-15-10 landing are this chunk's items 1-12, so a "owed by the route" divergence is what each item must close in code or tests.

@history_item n=2 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`unknown`'s detail; the code word screen; exit 20's closed detail; the `--json` failures witness) — `unknown` (exit 14) and the two faults (exit 20, exit 1) keep NO hint line because a code-word detail would make a hint guess. A code word outside the closed list exits 14 as `unknown`. `tests/cli_error_bodies.rs` holds each `--json` failure to one document with no ESC, path or upstream text. This is a trap for item 1: `hint_cause` maps `UnverifiedCli | Unknown => None`, and only `UnverifiedCli` should gain a hint (`decision/no-hint-for-unknown`, `decision/no-hint-for-faults`). No hint under `--json` (`decision/no-hint-under-json`).

@history_item n=3 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — rewrote the `not-delivered · input-not-ready` hint in the founder's wording, chosen among three. The `contract/refusal-hints/not-delivered-*` strings are fixed founder text. This is a precedent for item 1: a missing hint takes the row's string byte for byte and invents no wording.

@history_item n=4 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — added `not-delivered · empty-text`, its hint (founder's wording) and the exit-13 detail list. Item 1 enumerates which `answer` `not-delivered` details print no hint, and this entry fixes the closed detail set and the hint for each (`contract/cli-exit-codes/13`, `contract/refusal-hints/not-delivered-empty-text`).

@history_item n=5 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — revive's fixed `unable:` / `hint:` pairs, one hint per cause, read in a fixed order. It set the rule that no hint names `viola release` and that no refusal shows a recorded directory, pid or logged id. Item 1's new hints must obey `decision/no-hint-names-viola-release` and `decision/no-internals-in-errors`.

@history_item n=6 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — the `budget-paused` hint stands as the design's string, and so does the row `contract/refusal-hints/budget-paused`. Item 1 must leave it as it is, and the chunk's boundary rules out any budget-gate removal. Item 12's replay guard replaces the budget leg's `e5_the_replay_reads_what_the_wrapper_wrote` without touching this hint.

@history_item n=7 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the server-verification hint as built) — the `server-verify` hint cause prints the hint row's line byte for byte, plain ASCII, no SGR, with no pid, start time, path or endpoint, and tests assert this. It is the pattern for item 1's new hint causes (`contract/refusal-hints/server-verify`).

@history_item n=8 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `endpoint-lost` hint and the hint causes; how a frame verb reads no endpoint) — `instance-unreachable` keeps one hint per cause (`decision/instance-unreachable-hint-per-cause`), and `not-running` stays the hint key of the log's `instance-dead`. With no live endpoint, an instance directory that is absent, a regular file or a symlink reads `unwrapped-name`. Item 6's symlink refusal must not change this post-strict-modes reading of a symlinked instance directory (`contract/exit-21-json-cause/unwrapped-name`).

@history_item n=9 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (exit 2's fixed stderr lines) — an oversize `viola answer` (`UNUSABLE_RESPONSE`) and an unusable `viola send` (`UNUSABLE_TEXT`) exit 2 with no document, and `tests/contract_error_bodies.rs` names them in `NOT_ERROR_BODIES`. This bounds item 1 on `send` / `answer`: these lines are not refusals and gain no hint.

@history_item n=10 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — `verify`'s fixed refusal became the named `a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`), with closed codes and never the content (the founder's live ruling). Items 2-4 extend the fixture email check to every fixture kind and to `--record`, reported through this refusal's `email` code (`contract/viola-verify-refusals/unclean-fixture`). No new code word is needed: the chunk's boundary rules out a new refusal reason.

@history_item n=11 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — the first form of the fixture refusal, `a recorded payload still holds a path or a username`, covered paths and usernames only. That history explains why `text_fault` checks absolute path and user word but not email (items 2-4).

@history_item n=12 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six frame verbs' strict-modes refusal; run's and revive's start home-tree check; verify's home-tree and stamps refusals) — built `check_instance` / `check_stamps` / `check_path` / `check_read` and the fixed home-state and instance-state pairs: exit 21 for frame verbs, exit 1 for `run` / `revive` / `verify`, no frame and no line written, and `strict-modes-failed` under `--json`. Item 6 makes these checks refuse a symlink. The refusal must land in these same pairs and exits (`contract/strict-modes-refusal/*`, `contract/viola-run-start-refusals/home-tree-check`, `contract/viola-verify-refusals/home-tree-check`, `contract/viola-verify-refusals/stamps-update`), with no new reason or code.

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (a home under a regular file) — a path the checks cannot stat became a strict-modes refusal, not the catch-site line: the home-state pair from `verify`, the instance-state pair from the frame verbs. This is the precedent for item 6: a symlink found by the stricter `lstat`-style read should map to the same pairs (`contract/strict-modes/home-under-regular-file`).

@history_item n=14 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (strict-modes readings without an OS arm) — removed `strict.rs`'s Windows owner/DACL arm and the board's OS arms. The home-under-a-regular-file reading has no OS lead. Item 6 edits `crates/viola-state/src/strict.rs` `unix::reading`, which is now the only arm.

@history_item n=15 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the board's one rule on the home) — the board scan holds the home to the read rule and `instances/` to the write rule, then each instance's set, and refuses the whole board with the home-state pair. Item 6's symlink refusal in `check_path` / `check_read` reaches this scan and must keep its order and pairs (`contract/viola-list-board/strict-modes-scan`, `decision/list-refuses-board-on-strict-modes-failure`).

@history_item n=16 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`link` / `unlink`'s exit-21 strict-modes refusal takes the fixed instance-state pair) — `link` / `unlink` check the driven instance before any frame and print the instance-state pair, exit 21. These verbs are among item 6's strict-modes callers (`contract/strict-modes-refusal/exit-21`).

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the eight frame verbs; the socket-directory pair at run and revive) — the socket-directory trust check and its `socket-dir` hint and pair: exit 21 at frame verbs, exit 1 at start, `socket-dir-refused` under `--json`. If item 6's or item 11's symlink tightening touches this path check, these outputs stay fixed (`contract/frame-verb-refusals/socket-dir`, `contract/viola-run-start-refusals/socket-dir`).

@history_item n=18 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `list`'s exit-21 whole-board refusal with the fixed home and instance pairs, which first set the `unable: <text>` fixed form beyond the exit-1 refusals. The pair strings that item 6's refusals print originate here (`contract/fixed-refusal-pairs/home-state`), confirmed by the founder at 2026-10-13-t06-38-35-wrap.

@history_item n=19 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's `internal-error` document) — `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site. Item 5 (`run` parsing its name through the shared `parse_name`) and item 10 (hook/MCP payload types deriving `Redact`) must keep `run`'s refusals and the silent verbs' behaviour unchanged (`decision/silent-catch-sites`, `contract/hook-verb/stderr`, `contract/mcp-verb/stdout`).
