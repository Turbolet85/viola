@history_item n=1 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap built without `color`, so every styled byte of human output is viola's own SGR module's. `CLICOLOR_FORCE` could otherwise force SGR into a pipe. Witnessed by `cli_output_plain`, green on 3 OSes. This is why a Unix SGR assertion (the v1-40 witness, the pipe/`NO_COLOR`/`TERM=dumb` no-SGR cases) reads that module alone. Re-enabling `color` reopens the ban.

@history_item n=2 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — gave message mode its rule: every control character but `\n`/`\t` prints as `\xHH`, never stripped, and `--json` stays serde-escaped. Unix `wait`/`last`/`link` cases must read the same escaped bytes as Windows (later reused for `link`'s non-RFC-3339 `since`).

@history_item n=3 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the readback box also fills on `/clear`'s measured post-condition (a `session-start` of cause `clear`), printing `[RB] read back` with no new word. The shared fake-agent send cases on Unix read the same `[RB]`/`[  ]` mirror outcomes.

@history_item n=4 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — reworded the `not-delivered · input-not-ready` hint (founder's wording). This chunk also carries that chunk's reply-probe CARRY. An exit-13 hint the Unix send cases read must print these exact words on every OS.

@history_item n=5 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — added `empty-text` as an exit-13 detail word, with its hint, after `control-character`. This is part of the closed exit-13 list, which the one-schema re-assertion must find identical on Linux, macOS and Windows.

@history_item n=6 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — added `revive`'s refusal pairs and its `--list` line form (`<time>  <cause>  <session id>`, plain ASCII). It also put a passed `viola revive` beside `viola run` in the 0.0 expression row and the no-colour ban. So the openpty wrapper must keep revive's passthrough (nothing printed once the child starts) exactly as ConPTY does.

@history_item n=7 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — added the `release --budget` lines (`budget gate lifted until <end>` with the `HH:MM:SSZ` / dated forms, `budget gate open`). It also ruled that `revive`'s strict-modes refusal carries no log detail code. The Unix suite re-reads this unstyled line byte for byte.

@history_item n=8 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — the board as built: `escape_row` escapes every control character as `\xHH`, WHEEL is padded to 23, there are five board forms, and `list`'s exit-21 whole-board refusal uses the home/instance `unable:`/`hint:` pairs. Unix list cases and the v1-40 witness rest on these fixed columns and pairs.

@history_item n=9 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder ratified WHEEL 23, the five board forms, `list`'s exit-21 pairs and the two `release --budget` lines. These are founder wording, not planning forms. A Unix case must not alter them to pass.

@history_item n=10 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a `verify` stopped by Ctrl-C/TERM prints no step line and no summary, only the fixed pair `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, and exits 1. On Unix, signal delivery differs from Windows, but this pair must still read identically.

@history_item n=11 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — introduced the per-OS relied count. `MM` became `LedgerRow::relied()`, and Unix got three more rows than Windows because the status line is wrapped only there (`statusline::WRAPPED_HERE`). This is the origin of the stated Unix/Windows verify divergence a test may OS-gate with its reason.

@history_item n=12 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — set the counter at `[01/22]`, MM 22 on Unix and 19 on Windows. `resume-session` is relied on every OS. The per-OS gap stays exactly the three statusline rows.

@history_item n=13 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — cut Surface: web-spa, web motion and web bans to a 0.2.0 stub, and dropped `viola ui` from the cli surface. This is why the chunk adds no rendered surface: parity is read on cli bytes alone.

@history_item n=14 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — removed the ubuntu CI render/contrast leg and DejaVu provisioning, so no CI step renders a page. Typography's Linux column now reads «Linux (CI-tested)». The ubuntu/macos jobs therefore carry no design render verdict, only byte-level cli assertions.

@history_item n=15 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — STATUS is joined from `claude agents --json`. Adds the `-- UNWRAPPED - READ-ONLY --` separator and unwrapped rows, `revive`'s `session-live` pair, and verify MM 23 Unix / 20 Windows. The colour layer and width cut were owed to "CLI output discipline". The Unix list cases read this separator and these rows.

@history_item n=16 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `revive` gained `--json` (one `{"v":1,"error":"<code>","detail":null}` document, no hint, stderr empty), and `--list --json` was added. These are the first of the closed revive codes the one-schema re-assertion checks on every OS.

@history_item n=17 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour layer (bold NAME, amber `DIALOG` with its kind word plain, a stale row dim end to end with NAME dim+bold, plain BAY/caption/separator, one write). It also built the width cut from `viola_pty::host_size()` (`TIOCGWINSZ`/`GetConsoleScreenBufferInfo`, `COLUMNS` not read, a piped stdout having no width) and the `NO_COLOR`-empty rule. This is the chunk whose P2 halt placed the v1-40 integration witness here: it was proven only at the unit tier (`board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber`) because of the mutants 10 s kill versus the ~4 s stamp plus the 5 s stale threshold.

@history_item n=18 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — `plugin install` is served in `setup:`. Its four plain-ASCII lines quote `<dir>` only where the host's shell would split it. Its exit-21 refusal prints the home-state pair, or the `instance-unreachable` document under `--json`. The shell-split quoting is the one place host shell differences touch output, so the Unix cases must read it.

@history_item n=19 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — set the current verify count: `[NN/24]`, MM 24 on Unix and 21 on Windows, `stamped 2.1.287  24 pass  0 fail`. `/compact` is read back by its same-session post-condition. These are the relied counts the extract's verify constraint cites.

@history_item n=20 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — Brand Identity's per-surface table now holds only the two cli rows. The cocked strip is amber on the CLI. The Signature Test's one 0.1.0 place is the `[RB]`/`[  ]`/`[/ ]` column of `viola send`. These cli rows are what the v1-40 witness and the mirror cases read.

@history_item n=21 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified the `host_size()` width source and served-verbs-only `--help`. Also ratified the `dialog-expired` line, the plugin install lines and verify's interrupted pair. The `TIOCGWINSZ` width the openpty wrapper relies on is settled and needs no new ruling.

@history_item n=22 marker=2026-10-25-u55-door
**2026-10-25-u55-door** — split design-system into its body and 9 category files (map `.andromeda/design-system.map.tsv`). The `design-system/surface`, `/color-palette`, `/typography`, `/iconography` and `/brand-identity` labels the cascade carries are those files.

@history_item n=23 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the hint as built, and its macOS gap) made the `server-verify` hint plain ASCII, with no SGR, pid, start time, path or endpoint. It first recorded macOS's gap: no peer pid, so the check passed with no comparison. That is the root of the macOS carve-out this chunk keeps.

@history_item n=24 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (exit 21's server-verification cause as built) set the rule: a server that is not the recorded wrapper gets no frame, exit 21. It was measured on Linux and Windows only by `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper`, so macOS parity of that witness is unmeasured.

@history_item n=25 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (`server-verify-failed` written by the six frame verbs) put the code in the CLI log as `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}`. This is a log detail code the one diagnostics schema must carry identically on every OS.

@history_item n=26 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (`link`/`unlink`'s exit-21 strict-modes refusal) the instance-state pair names the driven instance, and `list`, `link` and `unlink` share the fixed form. Under `--json` it is the `instance-unreachable` document. Unix strict-modes cases must read the same pair.

@history_item n=27 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (the server-verification cause and hint cover eight frame verbs) extended the cause, hint and log code from six verbs to eight. Unix server-verification cases span all eight.

@history_item n=28 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (`link`/`unlink`'s result lines as built) each is one plain-ASCII stdout write with no SGR whatever the terminal, showing the parsed argument names. `since` is `HH:MM:SSZ` or escaped. A repeated link prints the `already linked` form. Under openpty these must stay unstyled.

@history_item n=29 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (`--json` documents and exit 20) `link` gives `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}` and `unlink` gives `{"v":1,"ok":{}}`. A refused pair (`-32602`) gives exit 20 and the `wrapper-fault` document. These are part of the one schema re-read on Unix.

@history_item n=30 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the six frame verbs' strict-modes refusal) the six verbs check the instance before any frame. A refusal prints the instance-state pair, exit 21, writing no frame and no line (not the `[/ ]` mirror). The `instance-unreachable` strict-modes hint was retired. Unix strict-modes cases read this pair.

@history_item n=31 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (run's and revive's start refusals at the home-tree check) the home-tree check runs before any log: a home-level refusal prints the home pair, an instance-level one the instance pair, exit 1 with no child; `revive --json` gives `strict-modes-failed`. The openpty start path must refuse identically before spawning.

@history_item n=32 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (which pair viola list's refusal prints) recorded the board's write-rule/read-rule inconsistency, owned by a CARRY on «Unix endpoint and home hardening». This is the precedent for the cross-OS home rule that 2026-11-01 settled.

@history_item n=33 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (verify's home-tree and stamps refusals) `verify` checks its home tree before its log; a refusal prints only the home pair, exit 1, and a refused `stamps.json` update prints the same pair. The text is the same on every OS.

@history_item n=34 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (a home under a regular file) records a per-OS divergence. On Unix the path is a strict-modes refusal (verify gives the home pair, exit 1; frame verbs the instance pair, exit 21). On Windows it stats `NotFound` and keeps the catch-site line, measured on `windows-2025`. A test of it may stay OS-gated with this reason named.

@history_item n=35 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (`unknown`'s detail) exit 14 prints `unable  unknown  <detail>`, the detail a code word or nothing, never opaque text, with no hint. Unix cases must find no OS-only word here.

@history_item n=36 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the code word and the screen) `viola_core::is_code_word` screens a peer's `refusal`/`detail` in `reply_of` and `of_reply`. A non-code-word detail reads null. An unlisted code word stays visible and exits 14. This is the source of the "documented null, never a missing key or an OS-only word" rule this chunk re-asserts.

@history_item n=37 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (exit 20's closed `detail`) `viola_channel::sanitised_fault` closes the five known codes' `message`/`data`. An unknown code reads `"message":"wrapper fault","data":null`. Exit 20 documents must be byte-identical across OSes.

@history_item n=38 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (exit 2's fixed stderr lines) the `UNUSABLE_RESPONSE`/`UNUSABLE_TEXT` lines go to stderr whatever `--json` says, with no document (`tests/contract_error_bodies.rs` `NOT_ERROR_BODIES`). This is the stated exception to one-document-per-result, and it holds on Unix too.

@history_item n=39 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the `--json` failures the witness drives) `tests/cli_error_bodies.rs` holds one document, the literal exit code, empty stderr, and no ESC, path or upstream text. This is the witness the chunk must run green on `ubuntu-latest` and `macos-latest`.

@history_item n=40 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (exit 21's document names its cause) `detail` became one of the closed cause words, not null. Every strict-modes refusal (`list`, `link`, `unlink`, `plugin install`, six frame verbs) reads `strict-modes-failed`. `v` stays 1 and no exit moved.

@history_item n=41 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the `endpoint-lost` hint and the hint causes) added the `endpoint-lost` hint (`<name>'s wrapper went away during the call; …`). Hint causes are keyed `not-running`/`unwrapped-name`/`endpoint-lost`/`server-verify`. Unix wrapper-loss cases read it.

@history_item n=42 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (a stale holder's own code) a stale holder prints and logs `stale-heartbeat`, exit 1. A frame verb against a stale instance reads `instance-dead`. The v1-40 stale-wrapper fixture must expect these, not a stale word at exit 21.

@history_item n=43 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the pending arch block spent, the log's codes and the two carve-outs) set D-20's log detail codes. It left the socket-directory cause and macOS's server check without codes, owned by «Unix endpoint and home hardening» (since closed). This is the code list the one diagnostics schema checks per OS.

@history_item n=44 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the catch site's `internal-error` document) the catch site prints `{"v":1,"error":"internal-error","detail":null}` under `--json` (stderr empty) and reads `--json` only before a `--`. `run`, `revive`'s start, `mcp` and `hook` stay silent there, which must hold under the openpty wrapper too.

@history_item n=45 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (`revive --json`'s documents and its ten codes) one exit-1 document covers every shared start refusal. A program not found gives `internal-error`, and a `pump_child` error prints no document because the child held the terminal. The openpty spawn and pump paths must keep both.

@history_item n=46 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (how a frame verb reads no endpoint and a lost call) with no live endpoint, a directory reads `instance-dead`, while absent, a regular file or a symlink reads `unwrapped-name`. A failed connect reads `instance-dead`; a lost call reads `endpoint-lost`. Unix socket semantics must map to the same causes.

@history_item n=47 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder ratified the `endpoint-lost` wording and the instance-directory reading. They are settled and need no new ruling. Brand Identity took no edit.

@history_item n=48 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the board's one rule on the home) the scan holds the home to the read rule on every OS and `instances/` to the write rule. A readable home refuses the whole board with the home pair, witnessed on all three OSes. The rule is still PROVISIONAL (overseer as delegate), with the founder's word owed at Epoch 7's boundary.

@history_item n=49 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (a refused socket directory at the eight frame verbs) exit 21 before any frame, with the `socket-dir` hint and no strict-modes pair; under `--json`, `detail` `socket-dir-refused`. This is a Unix-native refusal the Unix suite must read on both OSes.

@history_item n=50 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the socket-directory pair at the start of run and revive) added `unable: the socket directory for <name> cannot be trusted` and its hint, refused before any snapshot, child or bind. Its wording is still an OPEN planning form owned by the Epoch 7 boundary batch. The openpty start must print it unchanged.

@history_item n=51 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`socket-dir-refused`, exit 21's sixth cause and a log code) brings the closed cause list to six and adds `socket-dir-refused` to D-20. These six words are the closed set the one-schema re-assertion checks, PROVISIONAL until the Epoch 7 batch.

@history_item n=52 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`server-verify-failed` on macOS and a refused peer euid) on macOS the check compares no pid and only the peer's euid (a named gap owned by no route entry). A mismatch alone prints the server-verification hint. On Linux and macOS a foreign or unreadable euid also refuses. This is the carve-out the scope keeps: no test reads macOS as a pid/start-time comparison.

@history_item n=53 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`revive --json`'s eleventh code) `socket-dir-refused` goes after `squatted-name`, giving eleven codes with `internal-error` last. One code covers both a failed create and a refused check (PROVISIONAL lean). Revive's closed list must be identical on every OS.

@history_item n=54 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the wrapper's silent drop of a refused peer) the wrapper drops a refused-euid peer before any frame: no line, nothing on the child's terminal, no wheel moved; the client reads EOF. The openpty wrapper's accept loop must keep this silence (PROVISIONAL lean).
