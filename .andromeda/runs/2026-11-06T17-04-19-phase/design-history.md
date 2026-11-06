@history_item n=1 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** (amd-afdghrws, clap built without `color`) — clap 4.6.7 was built without its `color` feature so that no dependency reads a colour or terminal variable and every styled byte comes from viola's own SGR module. The reason was that clap's `color` let `CLICOLOR_FORCE` push SGR into a pipe, and `cli_output_plain` was the witness. This is the floor the `portable-pty` move (item 12) must not break by bringing in a terminal or colour reading dependency, and it is the pin `tests/cli_output_plain.rs` holds while its remnants at :104/:116 are removed.

@history_item n=2 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (amd-gtarjlax, revive's refusals and the 0.0 passthrough) — the cli 0.0 expression row and the no-colour ban gained a passed `viola revive` as the same passthrough as `viola run`, revive "prints nothing once the child starts", and the `unable: <text>` form was widened to revive's exit-1 refusals. The `portable-pty` / `crates/viola-pty` move must keep this two-verb silence, and the item 6 folds must keep revive's fixed pairs exactly.

@history_item n=3 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (amd-2y7mmemb, release --budget lines as planning forms) — this entry built `<name>  budget gate lifted until <end>  wheel <holder>` and `<name>  budget gate open  wheel <holder>`, with the date form, as planning forms awaiting the founder's wording. This is the built CLI half of v1-13's budget-release clause, which item 4 defers or re-words.

@history_item n=4 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** (amd-etdfhyhl, the board as built) — this entry set the as-built form of `viola list`: its six fixed columns, WHEEL padded to 23, the `escape_row` row mode, five planning forms, and the exit-21 whole-board refusal with the home and instance pairs. This is the board that v1-36's «link set on the board» is weighed against (no link column, so no new column), and it is the origin of the board refusal whose read rule item 7 rules on.

@history_item n=5 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** (amd-7ogxwyhq, the board's forms and the two budget lines are the founder's) — the founder ratified WHEEL 23, the five board forms, the list refusal pairs and the two `release --budget` lines, and the superseded marks stand in the body as the operator's word, superseded. This is the precedent for how the founder's batch word (item 7) lands: provisional marks lifted, the body worded, the history kept. It also confirms that v1-13's budget-release CLI lines are his wording.

@history_item n=6 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** (amd-pl7cdoiz, readback fills on a post-condition too) — besides the matching `prompt-submitted`, `/clear`'s measured post-condition (a `session-start` with cause `clear`) now also fills the readback, and no new word was added. The live driver-drives-builder session's evidence judges `[RB] read back` by these confirmers only, never by `turn-ended`.

@history_item n=7 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (amd-3xbtldxd, /compact read back by its post-condition) — a `/compact` on a verified CLI reads back on a `session-start` of cause `compact` with the same session id, and on an unverified CLI it stays `unconfirmable`. Any `/compact` or `/clear` in the live session counts by this rule, `[  ] unconfirmable` being its other legitimate outcome.

@history_item n=8 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (amd-5ei6gkq6, the `input-not-ready` hint) — the founder worded the `not-delivered · input-not-ready` hint (send again, and if it repeats a human must look at the session; no `viola wait`). This is the prior live driver session's precedent, and it is the exit-13 hint the live session's `[/ ] unable` evidence would print. The item 6 `Refused` fold must keep it.

@history_item n=9 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (amd-yrrcz2jf, the `empty-text` row) — this entry added `RefusalDetail::empty-text` to the palette rows, to the hint list, and as the last detail of exit 13. It is part of the closed RefusalDetail set that the item 6 `Refused` fold must leave unchanged, with the palette rows and the exit-13 row naming one set.

@history_item n=10 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** (amd-dw5o7rze, colour layer and width cut built) — this entry built the colour layer: NAME bold, only the `DIALOG` word amber, the stale row dim end to end, and rung 4 reading a present `NO_COLOR`, empty included. It also set the width to stdout's own size via `viola_pty::host_size()` with `COLUMNS` not read, and limited `--help` to served verbs. This is the colour decision order and plain-output contract that the item 17 removals and `plain_drive` must leave intact. The `host_size()` read in `crates/viola-pty` is what the `portable-pty` move must keep.

@history_item n=11 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** (amd-2tr3m7w6, the founder's Epoch 5 batch) — the founder ratified the width source and the served-verbs-only help, and the provisional marks of six prior entries were superseded with none pruned. This is the procedural precedent for discharging item 7's four PROVISIONAL items at the founder's Epoch 7 batch word.

@history_item n=12 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (amd-fwe5wj34, `mcp` a hidden verb) — `mcp` is run by the plugin, never by a person, and appears in no `viola --help` row. The v1-13 disposition (the MCP result for `link`) must add no verb or help row against this.

@history_item n=13 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** (amd-uw4p2fkr, Surface: web-spa cut to a 0.2.0 stub) — on the founder's ruling, the web surface, web motion and web bans were cut, and the kept sections were framed «A 0.2.0 frontend design input; 0.1.0 renders no page.». This is the basis for scope's rule that a cap whose acceptance names a frontend surface can only be deferred on the founder's word. v1-36's board clause is weighed against the CLI board, not a page.

@history_item n=14 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (amd-2ssfmmra, web references cut, Signature Test at one 0.1.0 place) — the Signature Test's one 0.1.0 place became the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`, and the readback box's only 0.1.0 place became its CLI mirror. The live session's acceptance (`[RB] read back … cursor <n>`, exit 0) is read against this.

@history_item n=15 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** (amd-n23kcyxj, `dialog-expired` takes the dialog form) — `viola wait` prints `dialog-expired  builder  dialog 7  cursor 49400` for an expired hold, and the founder ratified the wording at 2026-10-20. This is the line a live read of a dialog hold near the 3600 s ceiling (item 2) would print if the hold expires.

@history_item n=16 marker=2026-10-25-u55-door
**2026-10-25-u55-door** (amd-ezdsjc5r, design-system split into body and 9 category files) — design-system is a body plus category files, mapped in `.andromeda/design-system.map.tsv`. The master edits this chunk makes (Platform-Specific Notes, any v1-13/v1-36 re-wording) land in the body or in the detail file (`surface.md`, `iconography.md`, `cli-verbs`) that holds the fact.

@history_item n=17 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (amd-5tkzrfet / amd-i54n6er6 / amd-sd7qs7m4, server verification) — this entry built exit 21's server-verification cause and hint for the six frame verbs, plain ASCII with no pid, path or endpoint, and the `server-verify-failed` log code. It was measured on Linux and Windows and carried a macOS gap. This is the client seam where item 8's surviving always-admit `peer_admitted` read sits. Its Windows/macOS readings are remnants this chunk's masters-agree check should find gone.

@history_item n=18 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (amd-d35mdbkj / amd-mynhk6mh / amd-gzo4lpft / amd-6wdqmohd / amd-4hm32gtb, link and unlink built) — `viola link` / `viola unlink` result lines are one plain write from the parsed arguments: `link    a -> b  since HH:MM:SSZ`, `already linked`, `unlink  a -> b`. Their `--json` documents are `{"v":1,"ok":{"since":…,"already_linked":…}}` / `{"v":1,"ok":{}}`, a refused pair is exit 20, the strict-modes refusal is the exit-21 instance pair, server verification covers eight verbs, and `handoff: link, unlink` is a help row. This is the built CLI half that v1-13's "MCP result for `link`" disposition and v1-36's board clause must agree with, with no verb, row or column added.

@history_item n=19 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (amd-exzbkfwb / amd-w5cbpn6n / amd-nnbuuwmz / amd-w6phpluc / amd-rkcwqs6h, strict-modes pairs) — this entry set the six frame verbs' exit-21 instance-state pair, the exit-1 home and instance pairs for `run`/`revive` at the start, `verify`'s home-tree and `stamps.json` refusals with the home pair, and the home-under-a-regular-file refusal (with a Windows `NotFound` arm, later removed). It also flagged the board's two-rule inconsistency with an owner. These are the human forms the `StampsError`/`StateError` fold or ratification (item 6) must keep. It is the origin of the board read-rule question in item 7.

@history_item n=20 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (amd-ngs74kgp / amd-owfr5fvz / amd-eohruztr / amd-k65k5y2q / amd-skxu24vm, sanitised error surfaces) — this entry added the `is_code_word` screen in both clients (`src/cmd/client.rs`, `crates/viola-mcp/src/result.rs`), exit 14's detail as a code word or nothing, exit 20's closed `detail` via `viola_channel::sanitised_fault` with the fixed `wrapper fault`, exit 2's fixed `UNUSABLE_*` lines with no document, and the `tests/cli_error_bodies.rs` witness (one document, literal code, empty stderr, no ESC, path or upstream text). A `ServeError`/`McpError` fold (item 6) must keep both client screens and these closed forms, and the witness must read green at wrap.

@history_item n=21 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (amd-qvu57cdd / amd-aun6jtda / amd-naaf5x6y / amd-umh66io5 / amd-db4gplsy / amd-4rauf2gc / amd-nhh7jcfw, exit causes) — exit 21's document names its cause in `detail` (now six closed words). This entry added the `endpoint-lost` hint, `stale-heartbeat` as its own code, the D-20 log codes, the catch site's `internal-error` document with `run`/`revive`/`mcp`/`hook` silent at the catch site, `revive --json`'s start codes, and no document for a `pump_child` error after the child held the terminal. These are the exit phraseology the item 6 folds must keep. The `pump_child` and catch-site silence are what the `portable-pty` move must not disturb.

@history_item n=22 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** (amd-4tbohrca, `endpoint-lost` wording and the unwrapped reading ratified) — the founder's dialog word lifted two OPEN marks, the provisional records of three chunks were superseded with none pruned, and a `marks-discharged.md` named each heading. This is the precedent for recording item 7's PROVISIONAL discharges.

@history_item n=23 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (amd-fzdxcsou / amd-t3mpp5ny / amd-3pr4zv7x / amd-2jwvtljf / amd-io7txdcs / amd-742wp5gz / amd-fx4wprsg, Unix hardening) — this entry set the board's strict-modes scan (the home to the read rule, `instances/` to the write rule, a readable home refusing the whole board with the home pair), PROVISIONAL with the founder's word owed at the Epoch 7 batch. It added the socket-directory pair `unable: the socket directory for <name> cannot be trusted` / its hint, landed as a planning form under `OPEN — … owner: the Epoch 7 boundary batch`, plus `socket-dir-refused` as exit 21's sixth cause and as revive's code. It also added the euid peer check in `server-verify-failed` and the wrapper's silent drop of a refused peer. These are exactly the item 7 rulings (the board's read rule, the socket-directory wording) and the item 8 untestable `peer_admitted` read. The founder's wording must land at the run/revive start pair, the eight verbs' exit-21 hint and the `--json` code together.

@history_item n=24 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (amd-vnn4v2zy, the stale row's end-to-end witness) — this entry recorded `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (outer PTY, `TERM=xterm-256color`, exactly DIM, BOLD, RESET, DIM, RESET, with no SGR on the BAY line, caption or separator). It is the colour-layer witness that «every gate green on Linux» re-reads after the pty dependency moves.

@history_item n=25 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (amd-ritqi256, Typography on the Linux-only target) — the Windows-first wording was retired, but each role kept its per-OS fallback stack and the table kept its Windows and macOS columns, as 0.2.0 inputs. This is the precedent for item 17's «removed or kept with its reason»: Windows mentions in frontend-input sections stay with a stated reason rather than being deleted.

@history_item n=26 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (amd-q7us3h6f, the cli Platform narrowed to Linux terminals) — the Platform line reads «Linux terminals (the core on any current distribution, CI on `ubuntu-latest`)» on the founder's 2026-11-04 rulings. The masters-agree check must find this line still standing after the item 17 removals.

@history_item n=27 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (amd-q5giruei, Windows VT enable retired from the colour rules) — this entry removed the windows-sys VT enable, rung 7 of the colour decision order, the Windows depth branch («256-colour if `TERM` contains `256color`;» alone) and the conhost note from Platform-Specific Notes. It left the Git Bash rewritten-path warning in Platform-Specific Notes, the Windows remnant this chunk's item 17 must remove or keep with its reason. This is the current form of the decision order the plain-output pins hold.

@history_item n=28 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (amd-pa2lbwql, the .cmd/.bat refusal and batch-script-child retired) — revive's closed `--json` codes went back to ten without `batch-script-child`, `run`'s and `verify`'s `.cmd`/`.bat` refusals left exit row 1, and the codes are pinned in `tests/cross_exit_causes.rs`. This is the precedent for how a Windows remnant leaves the exit phraseology and the D-20 codes together. It is also the current count the item 6 folds must keep.

@history_item n=29 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (amd-mutrsnzo, the cli width read by TIOCGWINSZ alone) — `viola_pty::host_size()` lost its `GetConsoleScreenBufferInfo` arm, so `TIOCGWINSZ` is unconditional. The `portable-pty` move in `crates/viola-pty` must keep this width source, with `COLUMNS` unread.

@history_item n=30 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (amd-jkmcqhpy, strict-modes readings without an OS arm) — this entry dropped «on every OS» from the board scan and removed the Windows `NotFound` reading of a home under a regular file. It is the stale-OS-wording cleanup precedent that item 7's «the board's read rule on every OS» ruling should read against, since the rule now has no OS arm.

@history_item n=31 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (amd-eycdzfxk, the stale-row witness on ubuntu-latest alone) — the witness is ungated and runs on `ubuntu-latest`, the one CI runner, passing at ci#42030025050. This is the measured baseline the wrap's CI read must re-confirm after item 12.

@history_item n=32 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (amd-cwulfg6q, the server check without the macOS carve-out) — the macOS `Matches` arm and `SERVER_PID_REPORTED` / `PEER_EUID_REPORTED` were removed, `peer_euid_admitted` is `observed == Some(own)`, and an unreadable euid is still refused. This is the current shape of the peer read whose always-admit mutation survives (item 8), so it is recorded as untestable by construction, not proved.

@history_item n=33 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** (amd-2i2njccw, verify's relied count, per-OS) — this entry introduced `LedgerRow::relied()` with a per-OS count (21 on Unix, 18 on Windows) via `statusline::WRAPPED_HERE`. It is the origin of the OS split folded away at 2026-11-05 (amd-c43esfh6), which item 17's remnant sweep can check is gone.

@history_item n=34 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (amd-c43esfh6, verify's relied count on every build) — `relied()` is 24 on every build, `WRAPPED_HERE` was folded, and the `shim-resolution` row passes when `claude --version` answers, its master wording left to this wrap. This is the current verify counter that the masters-agree check compares against the tests. Its leftover wording is a near-remnant for item 17.
