@carried n=1 label=design-system/cli-verbs fact=fct-tkxomkux
why: v1-13's budget-release clause must be deferred or re-worded against this line, and the master text must agree with whichever the founder rules.
quote: `viola release builder --budget` → `builder  budget gate lifted until 21:00:00Z  wheel human` when a pause stood. The time is the override's end, taken from the reply's `override_until`: `HH:MM:SSZ` when it falls on the current UTC date, `YYYY-MM-DD HH:MM:SSZ` on any other date, and a value that is no RFC 3339 instant prints escaped. When no pause stood the line is `builder  budget gate open  wheel human`. Both are one unstyled stdout line with the two-space gutter of the other verb lines.
@carried n=1 run=2 label=design-system fact=fct-ybxjcc2w
quote: `viola release builder --budget` → `builder  budget gate lifted until <time>  wheel human` when a pause stood, else `builder  budget gate open  wheel human`, one unstyled stdout line.
@carried n=2 label=design-system/cli-verbs fact=fct-327x7jz3
why: v1-13's clause on the MCP result for `link` must be deferred or re-worded against the link / unlink result line, which stays as stated.
quote: `viola link` / `viola unlink` print their one result line on stdout in one write, plain ASCII with no SGR whatever the terminal, and exit 0; the names printed are the verb's own parsed arguments, never names read back from the reply or the log.
@carried n=2 run=2 label=design-system/cli-verbs fact=fct-clb6azzi
quote: The `since` of the `link` line is the reply's `since` as UTC clock time in whole seconds with `Z` (`HH:MM:SSZ`); a `since` that is no RFC 3339 instant (the restore's `unknown`) prints through the message-mode escaper of pattern 3.
@carried n=2 run=3 label=design-system/cli-verbs fact=fct-dfvj3huj
quote: A second `viola link` of a held pair prints the `already linked` form, the wrapper keeping the first `since`; `viola unlink` of a pair that is not held prints the same `unlink  <driver> -> <driven>` line; both exit 0.
@carried n=2 run=4 label=design-system fact=fct-glxbi6ql
quote: `viola link overseer builder` → `link    overseer -> builder  since 19:43:58Z` (an existing pair prints `link    overseer -> builder  already linked`)
@carried n=2 run=5 label=design-system fact=fct-vmpiq5lh
quote: `viola unlink overseer builder` → `unlink  overseer -> builder`
@carried n=2 run=6 label=design-system fact=fct-cj5433yq
quote: **`mcp`** writes only MCP frames on stdout. Neither has a human design surface. `mcp` is a hidden verb (chunk 2026-10-17-mcp-server-for-drivers): run by the plugin, never by a person, it appears in no `viola --help` row.
@carried n=3 label=design-system/cli-verify-run fact=fct-xi5ne4sz
why: item 6's `StampsError` / `StateError` fold or ratification must leave the home-state pair of a refused home tree or `stamps.json` update exactly as stated.
quote: Before its log, `verify` checks its home tree (the home, `ledger/`, `bin/`, `plugin/`, each that exists): a refusal prints no step line and no summary, only the home-state pair `unable: the viola home is not private to you` then `hint: the viola home must be readable only by you; viola does not change its permissions` (pattern 2's fixed form), and exits 1; a `stamps.json` update refused by its strict-modes check prints the same pair.
@carried n=4 label=design-system/cli-verify-run fact=fct-bctn63fi
why: item 6's folds of the strict-modes error enums must keep the home-state (exit 1) and instance-state (exit 21) pairs as stated.
quote: A home under a regular file cannot be read and is a strict-modes refusal, not the catch-site line: `verify` prints the home-state pair (exit 1) and the frame verbs the instance-state pair (exit 21).
@carried n=5 label=design-system/cli-verify-run fact=fct-gray6vne
why: the `portable-pty` move under `crates/viola-pty` must keep `revive`, as `run`, printing nothing once the child starts, and the live `viola run` reads that passthrough.
quote: **`revive`:** prints nothing once the child starts, as `run`.
@carried n=5 run=2 label=design-system fact=fct-w4pl2fni
quote: **`run`:** prints nothing once the child starts.
@carried n=5 run=3 label=design-system fact=fct-hod2cupz
quote: `viola run` (while the child runs, nothing is printed at all)
@carried n=5 run=4 label=design-system fact=fct-jphnedpy
quote: **NEVER emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`.** LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen; a passed `viola revive` is the same passthrough.
@carried n=6 label=design-system/cli-verify-run fact=fct-xtefeabf
why: the `portable-pty` move reaches `pump_child`, whose error under `revive --json` must still print no document.
quote: **A `pump_child` error under `revive --json`:** prints no document: the child has held the terminal and viola writes nothing past it.
@carried n=7 label=design-system/cli-verify-run fact=fct-mep5dn6i
why: item 7's socket-directory wording and item 6's enum folds must leave `revive --json`'s closed start codes exactly as stated.
quote: **`revive --json`'s documents:** one exit-1 document for every refusal of the start it shares with `run`, its closed codes `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`; `run` itself prints none of these documents.
@carried n=7 run=2 label=design-system fact=fct-2t54hjhs
quote: under `--json` every refusal of the start it shares with `run` prints one stdout document and no hint, stderr empty, its code one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`; a `pump_child` error prints none, the child having held the terminal.
@carried n=8 label=design-system/cli-verify-run fact=fct-gul3bnzl
why: item 7's founder word on the socket-directory pair's wording lands where `socket-dir-refused` is stated, its code and its no-hint document kept.
quote: **`revive --json`'s socket-directory refusal:** one stdout document with the code `socket-dir-refused`, no hint, stderr empty; a failed create of the directory and a refused check read that one code.
@carried n=8 run=2 label=design-system fact=fct-tj4jq2zv
quote: 21 `unable  instance-unreachable` then the cause's hint, or for a refused strict-modes check its fixed home-state or instance-state `unable:` / `hint:` pair instead (pattern 1 and the paragraphs below), its `--json` document `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, one per cause, the cause one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`, with stderr empty.
@carried n=9 label=design-system/cli-verify-run fact=fct-4md437yp
why: item 6's folds must keep a program-not-found and an anyhow start error mapped to the one `internal-error` document.
quote: **`revive --json`'s `internal-error`:** a program not found prints `internal-error`, a cause with no human pair; an anyhow error of its start prints the same document, and the catch site logs its chain and its line.
@carried n=10 label=design-system/cli-verify-run fact=fct-f2zfjygd
why: item 8's dated survivor note (the always-admit `peer_admitted` read) must agree with this refused-peer behaviour, which stays the target.
quote: **A peer the wrapper refuses:** dropped before any frame: no line written, nothing on the terminal the child holds, no wheel moved; the client reads end-of-file.
@carried n=11 label=design-system/cli-exit-codes fact=fct-nr7mro52
why: the live sessions' evidence reads the exit-0 result lines on stdout, and item 6's folds must keep the exit-0 forms as stated.
quote: | 0 | none (TTY context lines only); the result line goes to stdout (`[RB] read back`, `answered`, `wheel human` …) | `{"v":1,"ok":{…}}` |
@carried n=11 run=2 label=design-system fact=fct-iee6axpq
quote: `wait` prints one static `waiting: builder` line on stderr (TTY only), then one result line on stdout:
@carried n=11 run=3 label=design-system fact=fct-blptucet
quote: **NEVER print "done" or "success" without context.** Say what was read back, by whom, and at which cursor
@carried n=12 label=design-system/cli-exit-codes fact=fct-sedcqxww
why: the exit-1 row holds the socket-directory wording item 7 rules on and the human forms item 6's enum folds must keep exactly.
quote: | 1 | `error: internal error` (fixed message; no chain with serde sources, no paths, no hint), or a `viola run` start refusal `unable: <name> is already live` / `… is still running but not answering` / `the endpoint for <name> is held by another process` / `the pinned viola copy failed its integrity check` / `the viola home is not private to you` / `<name>'s state files can be written by another user` / `the socket directory for <name> cannot be trusted`
@carried n=12 run=2 label=design-system fact=fct-zumogwyu
quote: **Stack traces** never print. Errors are fixed messages, and full detail goes only to `instances/<name>/diagnostics/`.
@carried n=13 label=design-system/cli-exit-codes fact=fct-4zlzvxze
why: item 6's folds must keep exit 2's fixed stderr lines with no document, as the phraseology states.
quote: | 2 | clap usage text (never produced by `viola hook`), or the fixed `UNUSABLE_RESPONSE` line (an oversize `viola answer`) or `UNUSABLE_TEXT` line (an unusable `viola send`) on stderr, whatever `--json` says; `tests/contract_error_bodies.rs` names them in `NOT_ERROR_BODIES` | no document |
@carried n=14 label=design-system/cli-exit-codes fact=fct-f5dfwzmp
why: item 6's `Refused` fold or ratification must keep exit 10's human and `--json` forms.
quote: | 10 | `unable  human-typing` (+ `manual-pause`) | `{"v":1,"refusal":"human-typing","detail":…}` |
@carried n=15 label=design-system/cli-exit-codes fact=fct-spwfscwz
why: item 6's `Refused` fold must keep exit 11's forms, the budget-paused state v1-13's budget release lifts.
quote: | 11 | `unable  budget-paused  five-hour` / `seven-day` | refusal object |
@binding n=1
**surface cli-verbs ↔ the verification matrix (v1-13's disposition)**: v1-13's deferral or re-wording must agree with the link / unlink line and the `release --budget` line as stated (design-system/cli-verbs fct-327x7jz3) (design-system/cli-verbs fct-tkxomkux), and with `mcp` a hidden verb writing MCP frames only (design-system fct-cj5433yq); no verb, flag or channel method is added to make it provable.
@binding n=2
**surface cli-verify-run ↔ the `portable-pty` move (`Cargo.toml`, `crates/viola-pty`)**: the new pty release must keep `run` and `revive` printing nothing once the child starts and a `pump_child` error printing no document (design-system/cli-verify-run fct-gray6vne) (design-system/cli-verify-run fct-xtefeabf), and add no dependency that reads a colour or terminal variable, clap staying without its `color` feature (design-system fct-3n5f23up).
@binding n=3
**surface cli-exit-codes ↔ architecture's typed error enums (item 6)**: each fold or ratification of `ServeError`, `McpError`, `StampsError`, `StateError`, `PinError`, `StampsReadError` and `Refused` must keep every typed exit code, its human stderr form and its `--json` document as the exit-code phraseology states (design-system/cli-exit-codes fct-sedcqxww) (design-system fct-tj4jq2zv), the strict-modes pairs included (design-system/cli-verify-run fct-xi5ne4sz).
@binding n=4
**surface socket-directory wording ↔ the founder's Epoch-7 word (item 7)**: the socket-directory pair's ruled wording must land in the exit-1 `run` refusal line, the exit-21 cause and `revive --json`'s `socket-dir-refused` code together (design-system/cli-exit-codes fct-sedcqxww) (design-system/cli-verify-run fct-gul3bnzl).
@binding n=5
**surface peer refusal ↔ test-plan §1 untestable zone and matrix#v1-44 (item 8)**: the dated survivor note and the untestable zone must agree with the refused-peer behaviour stated for the wrapper (design-system/cli-verify-run fct-f2zfjygd).
@binding n=6
**surface Platform-Specific Notes ↔ item 17's Windows remnants**: where a remnant removal reaches the Git Bash on Windows rewritten-path warning, the design-system line must follow, removed or kept with its reason (design-system fct-7u3hdiu2), and the cli Platform line must still state Linux terminals with CI on `ubuntu-latest` (design-system fct-7z5o76y5).
