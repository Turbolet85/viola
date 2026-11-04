@carried n=1 label=a11y-plan/a11y-scope-summary fact=fct-itb2wxsm
why: the openpty wrapper on Linux and macOS must hold the same passthrough boundary the ConPTY wrapper holds, for viola run and a passed viola revive alike.
quote: The assertion boundary is viola's wrapper:
  - viola writes zero bytes to the terminal except the child's output;
  - human keystrokes are never blocked, refused or delayed past the current atomic paste;
  - focus, mouse and resize sequences and terminal replies (F-W2's closed list) pass through and do not count as human editing; every other byte does. On Windows the platform decides what arrives (F-W3): the ConPTY host swallows focus reports, and under win32-input-mode an injected mouse report reaches viola as typed keys and takes the wheel.
  - under `viola revive --json` a `pump_child` error prints no document: the child has held the terminal, and viola writes nothing past it.
@carried n=1 run=2 label=a11y-plan/aria-patterns-roles fact=fct-u64ggudd
quote: | tui | N/A (vendor `claude` TUI, boundary-only) | n/a |
@carried n=1 run=3 label=a11y-plan fact=fct-rdegquaa
quote: viola writes zero bytes to the terminal except the child's output, human keystrokes are never blocked, refused or delayed past the current atomic paste, and focus, mouse and resize sequences and terminal replies pass through and do not count as human editing
@carried n=2 label=a11y-plan/a11y-scope-summary fact=fct-twqx5x46
why: the boundary must be asserted on the portable-pty outer-PTY driver on the Unix legs, and the openpty wrapper's diagnostics must go to diagnostics/, never to the terminal.
quote: Boundary assertions run on the tests' portable-pty `=0.8.1` outer-PTY driver (tests excerpt, E2E driver → tui). They check that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed.
@carried n=2 run=2 label=a11y-plan/a11y-scope-summary fact=fct-twqx5x46
quote: **Notes:** viola's diagnostics for `run` go to `diagnostics/`, never to the terminal (arch excerpt, A11y-Relevant Conventions → terminal output discipline).
@carried n=2 run=3 label=a11y-plan fact=fct-weaq66l7
quote: boundary assertions on the portable-pty outer-PTY driver check that viola emits zero terminal bytes of its own and never blocks or delays human keystrokes; viola's diagnostics for `run` go to `diagnostics/`, never to the terminal
@carried n=3 label=a11y-plan/aria-patterns-roles fact=fct-dx4kie52
why: P4's cli refusal and three outer-PTY cases must read the full assertion on Linux and macOS under the openpty wrapper while windows-2025 keeps its platform fact.
quote: **cli:** `send` refused with `human-typing` on stderr, with a typed exit code (assert_cmd).
  - **tui:** three portable-pty outer-PTY nextest cases on all three OS legs, one per Section 1 boundary clause (Section 3 → Keyboard test harness → Tooling): (1) zero viola-originated bytes on the outer-PTY stream (byte for byte against an unwrapped fake-agent run on Linux/macOS; viola's own literals absent on Windows ConPTY); (2) human keystrokes delivered unblocked past the current atomic paste; (3) `\x1b[I`/`\x1b[O`, mouse and resize sequences do not move the wheel — step-wise (the resize behind its size receipt, then the focus reports with a mouse report as their read barrier), the wheel probed by an `answer` to no pending dialog (exit 13 driver, 10 human); the full assertion on Linux and macOS, the platform fact on `windows-2025` (F-W3: focus reports swallowed, the injected mouse report takes the wheel), Section 3 → Keyboard test harness → Tooling. A further case sits under clause (2): a key written while a verified `send` waits in the readiness gate takes the wheel, the send exits 10 `human-typing` with nothing typed and the key reaches the child (the same Tooling paragraph, with its stated limit).
@carried n=3 run=2 label=a11y-plan fact=fct-jkkyxmll
quote: on the tui, outer-PTY nextest cases on all three OS legs — zero viola-originated bytes, keystrokes delivered unblocked past the current atomic paste, and focus, mouse and resize sequences not moving the wheel on Linux and macOS, on `windows-2025` the platform fact that an injected mouse report takes the wheel.
@carried n=3 run=3 label=a11y-plan fact=fct-hu3qpzu6
quote: tui: human keystrokes are never blocked or delayed, and focus/mouse/resize sequences and terminal replies are ignored for wheel purposes (on Windows an injected mouse report under win32-input-mode arrives as typed keys and takes the wheel, F-W3).
@carried n=4 label=a11y-plan/a11y-scope-summary fact=fct-yscgzykm
why: the feed-panic boundary must read no viola literal on every OS and the child's own bytes on the Unix legs under openpty.
quote: the driver's `send` is refused `not-delivered` / `input-not-ready` (exit 13) while a typed human key still reaches the child; none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, on all three OSes; on Unix the child's own bytes (`e4 b8 ad`) reach it.
@carried n=4 run=2 label=a11y-plan/a11y-scope-summary fact=fct-4etq6dn5
quote: the Windows outer-terminal reading of the child's own bytes (`e4 b8 ad`) after a contained vt100 feed panic — ConPTY re-renders a wide character on a 1-column screen, so windows-2025 reads only viola's literal absence; owed to "Windows-only live measurements".
@carried n=5 label=a11y-plan/a11y-scope-summary fact=fct-vtox22ts
why: the cli surface's output discipline must hold on the ubuntu-latest and macos-latest legs beside windows-2025.
quote: there is no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`; `viola revive --list` rows and the revive refusal pairs carry no ESC byte, and each `--json` failure document `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) drives carries none;
  - output is static and linear, with no spinner or redraw;
  - C0/C1 control characters are escaped except `\n` and `\t` [resolved: in `list` table rows `\n` / `\t` are escaped too (ratified T5, design cli); `wait` / `last` keep them; Section 4 P6].
  - This must hold on `[windows-2025, macos-latest, ubuntu-latest]` (tests excerpt, Coverage Triggers → multi-platform).
@carried n=5 run=2 label=a11y-plan fact=fct-sf2opzaw
quote: colour is only a second cue; no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`; output is static and linear; C0/C1 control characters are escaped except `\n` and `\t`, which `list` rows escape too.
@carried n=6 label=a11y-plan/visual-design-verification fact=fct-es7le4as
why: the outer-PTY dim witness this chunk takes up reads these SGR values, and the unwrapped-NAME cut reads the host width the openpty resize must report as ConPTY does.
quote: As built (chunk 2026-10-17-cli-output-discipline) `viola list`'s human mode on a terminal writes bold on every NAME value (wrapped and unwrapped), the attention (amber) token on the `DIALOG` word only of a wrapped row with a pending dialog (its kind word plain), and a wrapped `stale` row dim from its first byte to its last (NAME dim and bold, `DIALOG` amber and not dim); the BAY line, caption, empty-board line and separator carry no SGR, and each state is still carried by its printed word (`stale`, `DIALOG <kind>`, `expired`, `budget-paused`, `unverified-cli`, the STATUS words `idle` / `busy`, the unwrapped rack's `n/a` cells). The colour decision reads `NO_COLOR` (present, an empty value included), `TERM` (exactly `dumb`; `256color`) and `COLORTERM` (the founder's own word, chunk 2026-10-17-cli-output-discipline `inputs#I2`) and whether stdout is a terminal; it writes no SGR under `--json`, a passthrough role or any of those rungs, and picks a truecolor / 256 / 16 depth (on Windows after enabling VT processing on stdout, restored afterwards; a failed enable writes none). On a terminal too narrow, an unwrapped NAME is cut to the column less 3 characters plus `...`, never inside a `\xHH` escape (the width from `viola_pty::host_size()`); a wrapped NAME is never cut, and piped output and `--json` keep every name whole.
@carried n=6 run=2 label=a11y-plan/visual-design-verification fact=fct-es7le4as
quote: dim only on stale rows is proven at the unit tier only, its outer-PTY witness a CARRY on "Linux and macOS parity". viola emits zero SGR of its own under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`, and the revive refusal pairs and `--list` rows carry no ESC byte. Under `viola run` on Windows the outer-PTY stream also carries the ConPTY host's own SGR, cursor and query bytes (not viola's: the inbox host's as measured at chunk 2026-09-25-pty-wrapper-on-windows research fact 4, the sideloaded `OpenConsole.exe`'s `ESC[1t ESC[c ESC[?1004h ESC[?9001h` as measured at chunk 2026-09-29-sideloaded-conpty), so the windows-2025 `viola run` check is that viola's literals are absent, on both backends. `unable` / `fail` are never coloured. Checked with assert_cmd + portable-pty on `[windows-2025, macos-latest, ubuntu-latest]`.
@carried n=6 run=3 label=a11y-plan fact=fct-znmnfnxq
quote: amber SGR only on the `DIALOG` word, dim SGR on stale rows and bold SGR only on NAME values, always beside the word, each state still carried by its word; no SGR of viola's own under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`; `unable` / `fail` never coloured; an unwrapped NAME is cut with `...` only on a terminal too narrow, never inside a `\xHH` escape; a wrapped NAME, piped output and `--json` never cut.
@carried n=6 run=4 label=a11y-plan fact=fct-lchghj7u
quote: **Owed:** dim only on stale rows is proven at the unit tier only; its outer-PTY witness is owed to "Linux and macOS parity".
@carried n=7 label=a11y-plan/aria-patterns-roles fact=fct-uok334yy
why: P6's TTY-only behaviours, the per-span SGR check and the dim-only-on-stale-rows witness owed here must read under the outer PTY on the Unix legs.
quote: linear static output: under the outer PTY, `waiting: <name>` appears exactly once on stderr and the stream contains no `\r` redraw, cursor-movement or erase sequence (no spinner); under non-TTY the `waiting:` line is absent;
  - `[NN/NN] … pass|fail`;
  - **cli mirror:** a literal pins the caption order the deferred 0.2.0 page must share (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI), not a trycmd case: the `CAPTION` literal in the root case `list_human_board_is_the_literal_rows` (`tests/cli_list.rs`) and the unit case `board_row_caption_and_default_row_are_the_literals` (`src/human/board/`), chunk 2026-10-11-the-board-viola-list.
  - no SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json`;
  - under the outer PTY, every SGR span sits beside its word (amber only on `DIALOG`, dim only on stale rows, bold only on NAME column values), and `unable` / `fail` carry no SGR; as built (chunk 2026-10-17-cli-output-discipline) `viola list`'s spans are checked under the outer PTY by `cross_list_rows_style_each_span_beside_its_word_on_a_terminal` (bold NAME, amber only on `DIALOG`; on windows-2025 the parsed attribute set) and at the unit tier by the `board_row_style_*` cases; dim only on stale rows is proven at the unit tier only (`board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber`), its outer-PTY witness a CARRY on "Linux and macOS parity"; `viola wait` and `viola --help` carry no ESC byte and no lone CR on Unix (`tests/support/sgr.rs` `redraws`);
@carried n=7 run=2 label=a11y-plan/aria-patterns-roles fact=fct-uok334yy
quote: held by the `board_row_*` units and by `tests/cross_list_rows.rs` in three modes (piped, `NO_COLOR` / `TERM=dumb`, the outer PTY; the no-ESC assertion Unix-only, and on windows-2025 each escaped name read as text under a 200-column terminal, because ConPTY re-renders escapes)
@carried n=7 run=3 label=a11y-plan/aria-patterns-roles fact=fct-uok334yy
quote: Pass condition: each injected control appears only in escaped form. In `wait` / `last` text `\n` / `\t` stay raw. In `list` table rows `\n` / `\t` are escaped too (`\x0A`, `\x09`), so every row stays one fixed-width line.
  - Under non-TTY, `wait` / `last` stdout contains no raw C0 / C1 byte other than `\n` / `\t`, and `list` stdout contains none inside a row. Under the outer PTY, the injected controls are escaped, and viola's own SGR stays beside its word only.
@carried n=7 run=4 label=a11y-plan fact=fct-ijnl2u6y
quote: trycmd / assert_cmd assert the non-TTY modes and the portable-pty outer PTY the TTY-only behaviours, on all three OS legs — linear static output, the caption-order literal, no SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json`, every SGR span beside its word, C0/C1 escaped.
@carried n=7 run=5 label=a11y-plan fact=fct-dqbono2s
quote: NEVER add spinners (design ban).
@carried n=8 label=a11y-plan/a11y-scope-summary fact=fct-p4tq3d4m
why: the board's caption line and its separator only when unwrapped rows exist must read the same on the Unix legs.
quote: As built the caption line and the `-- UNWRAPPED - READ-ONLY --` separator stand (the separator since chunk 2026-10-16-unwrapped-sessions-on-the-board, printed only when unwrapped rows exist)
@carried n=8 run=2 label=a11y-plan fact=fct-w4hvnbpo
quote: the structural equivalents are the caption line, the `-- UNWRAPPED - READ-ONLY --` separator (only when unwrapped rows exist) and the `--help` group labels.
@carried n=8 run=3 label=a11y-plan fact=fct-pgo5ntgv
quote: Output is linear and static: `waiting: <name>` once on stderr (TTY only), `verify` step lines `[NN/NN] … pass|fail`.
@carried n=9 label=a11y-plan/aria-patterns-roles fact=fct-ai3fe6uo
why: the board order and the --help literal are fixed CLI strings that must read byte-identical on every OS, an OS-only wording failing the pin.
quote: BAY line → caption line → wrapped rows → (only when unwrapped items exist) the `-- UNWRAPPED - READ-ONLY --` separator and the unwrapped rows; the `--help` group labels built (chunk 2026-10-17-cli-output-discipline): `viola --help` and `viola help` print `Usage: viola [--home <dir>] <verb> [args]`, the rows `board:` · `traffic:` · `wheel:` · `handoff:` · `setup:` over the served verbs only, then `Options:`, plain ASCII, pinned by the `HELP` literal in `help_is_the_grouped_verb_table` (`tests/cli_output_plain.rs`)
@carried n=9 run=2 label=a11y-plan fact=fct-m7wf3gp4
quote: **cli** — N/A (terminal); its structural equivalents are the BAY context line, the uppercase caption line, the `-- UNWRAPPED - READ-ONLY --` separator and the `--help` group labels.
@carried n=10 label=a11y-plan/a11y-scope-summary fact=fct-k76k2lf7
why: the frame verbs' server-verify refusal must read the same document and hint on every OS, its macOS leg an euid mismatch alone, never a pid comparison.
quote: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` each refuse, before any frame, an endpoint whose serving process is not the recorded wrapper: exit 21, stdout empty and the server-verify `hint:` line last on stderr (plain ASCII, no SGR), or under `--json` `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` with stderr empty (the refusal's text: Cognitive Accessibility › Error recovery); on macOS no peer pid is read (the recorded gap, a named carve-out no route entry owns), so there the refusal means an euid mismatch alone; their `error` word and exit codes are unchanged
@carried n=10 run=2 label=a11y-plan/a11y-scope-summary fact=fct-k76k2lf7
quote: **Failures under `--json`:** each failure `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) drives is one document, a literal exit, empty stderr, no ESC byte and a clean body; the `viola answer` and `viola send` exit-2 paths write a fixed stderr line and no document (the text: Cognitive Accessibility › Error recovery).
  - **The start's refusals under `viola revive --json`:** `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused` each print one exit-1 document in place of the human pair, before the child spawns (`socket-dir-refused` before any snapshot or bind too); without `--json` each prints its pair.
  - **A `claude` not found at a start:** under `viola revive --json` it reads as the `internal-error` document; it has no human pair.
@carried n=10 run=3 label=a11y-plan fact=fct-z2htpz2e
quote: every refusal of the start it shares with `run` prints one exit-1 document, `batch-script-child`, `pinned-hash-mismatch` and `squatted-name` among them and a program not found `internal-error`; a `pump_child` error prints none: the child has held the terminal, and viola writes nothing past it.
@carried n=11 label=a11y-plan/a11y-scope-summary fact=fct-7hhxeiso
why: link and unlink's result lines and refusals are fixed strings the Unix legs must read identical to windows-2025.
quote: each prints one plain-ASCII result line in one write with no SGR in any mode (the lines: Cognitive Accessibility › Plain language), or under `--json` one `{"v":1,"ok":<ok>}` document; each refuses with exit 21 a driven instance that fails the strict-modes check, before any log opens, a stopped driven instance and a server that is not the recorded wrapper (the entity above), and a pair the wrapper refuses as invalid params reaches it as a fault, exit 20, no hint
@carried n=12 label=a11y-plan/a11y-scope-summary fact=fct-mc7fijew
why: link's result line keeps its escape rule on every OS, a C0/C1 exception the Unix legs must read as Windows does.
quote: a `since` that is no RFC 3339 instant (the restore's `unknown`, or a value from a hand-altered log) prints through `escape_message`, which keeps `\n`, `\t` and non-ASCII
@carried n=13 label=a11y-plan/a11y-scope-summary fact=fct-dssvg3hs
why: the frame verbs' strict-modes refusal is stated on all three OSes and must read so on the Unix legs.
quote: each refuses before any frame or log (`send` after its text read) with exit 21, stdout empty and `link`'s instance-state pair on stderr, `hint:` last, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty; no line is written and the mode or ACL is left as found, on all three OSes
@carried n=14 label=a11y-plan/a11y-scope-summary fact=fct-sjhco52d
why: the frame verbs' socket-directory refusal is the Unix-only leg that must run on ubuntu-latest and macos-latest.
quote: over a socket directory that fails its check (mode open to another user, a symlink, not a directory, another owner), on Unix, in the human TTY, plain and `--json` modes: each refuses before any frame with exit 21 and the socket-directory `hint:` line last on stderr, plain ASCII, no SGR, naming no path, pid or uid, or under `--json` one document with `error` `instance-unreachable` and `detail` `socket-dir-refused`; the directory is left as found; off Unix the check reads nothing
@carried n=15 label=a11y-plan/a11y-scope-summary fact=fct-hj6bq3jt
why: run's home refusal is pinned in tests/run_cli.rs and carries a Unix-versus-Windows split the parity suite must keep, each leg reading its own outcome.
quote: each refuses before any log opens with exit 1, stdout empty and the home-state pair on stderr, `hint:` last, and starts no child — `run` and revive's start print the instance-state pair for an instance-level refusal, and revive under `--json` gives its `strict-modes-failed` document (the CLI entity above); `verify` prints no step line and no summary, and its `update_stamps` refusal gives the same home-state pair; on Unix a home under a regular file is this refusal, on Windows (such a path stats `NotFound`) the `cli` catch-site fault line stands
@carried n=16 label=a11y-plan/a11y-scope-summary fact=fct-rts3hgep
why: the start's socket-directory refusal is a Unix behaviour of viola run and revive that the openpty wrapper must keep before any bind or child.
quote: each refuses with exit 1 before any snapshot, bind or child, a failed create reading the same, with the socket-directory pair on stderr, `hint:` last, plain ASCII, no SGR, naming only the parsed name, and revive under `--json` gives its one `socket-dir-refused` document (the CLI entity above); the directory is left as found
@carried n=17 label=a11y-plan fact=fct-pzkrh6fa
why: under the openpty wrapper the terminal must belong to the child and no human keystroke be blocked or delayed past the atomic paste.
quote: **tui:** the terminal belongs to the child. Human keystrokes are never blocked or delayed past the atomic paste.
@carried n=18 label=a11y-plan fact=fct-csjr2vg7
why: the Unix wrapper must never block a human key nor count focus, mouse, resize or replies as editing, while Windows keeps F-W3.
quote: NEVER block, refuse or delay a human keystroke in `viola run`, or in a start a passed `viola revive` made, past the current atomic paste, and never count focus / mouse / resize sequences or terminal replies as editing (P4) — a mouse report the Windows ConPTY already turned into win32 key-down records is typing (F-W3).
@carried n=19 label=a11y-plan fact=fct-633dmm7e
why: the Unix legs must read unable and fail uncoloured and no SGR under any plain rung or the openpty passthrough.
quote: NEVER colour CLI `unable` / `fail`, and never emit SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`.
@carried n=19 run=2 label=a11y-plan fact=fct-4w44zoy4
quote: NEVER convey information by color alone
@carried n=20 label=a11y-plan fact=fct-i7pow3j3
why: the fake-agent Unix suite must read only viola's own bytes and the boundary, never the rendered claude screen.
quote: NEVER parse the rendered `claude` TUI screen for content or take screenshots as a11y evidence (R7; tests "no screen-content parsing").
@carried n=21 label=a11y-plan fact=fct-vt5poetp
why: the CAPTION literal must hold unchanged on the Unix legs.
quote: NEVER let the caption set or order of CLI `viola list` (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI) drift from its `CAPTION` literal (SC 3.2.4).
@carried n=22 label=a11y-plan fact=fct-c6pzu5nt
why: every human-mode refusal on the Unix legs must end with its hint line, unknown alone excepted.
quote: Every human-mode CLI refusal except `unknown` (exit 14, its detail a code word or nothing, design cli) ends with a `hint:` line.
@carried n=23 label=a11y-plan fact=fct-lsnfzz6p
why: the human-typing refusal on the Unix legs must show its kebab-case code beside the plain word unable.
quote: a kebab-case reason/detail code (`not-delivered`, `human-typing`, `unverified-cli`) never appears without the plain word `unable`.
@carried n=24 label=a11y-plan fact=fct-by4mf7d7
why: the chunk re-reads ci.yml's matrix, whose three legs this line names.
quote: **Platform:** GitHub Actions, one push/PR workflow `ci.yml` (the scheduled `nightly.yml` carries no a11y step), matrix `[windows-2025, macos-latest, ubuntu-latest]` (upstream-context Architecture Excerpt → CI/CD Platform).
@carried n=25 label=a11y-plan fact=fct-tyva3sbt
why: green on ubuntu-latest and macos-latest means no gate breach on either leg.
quote: `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`).
@binding n=1
**visual-design-verification SGR values ↔ aria-patterns-roles P6**: the outer-PTY witness of dim only on stale rows, owed to this entry, must read under the openpty outer PTY on ubuntu-latest and macos-latest the values the visual entity states: a stale row dim from its first byte to its last, NAME dim and bold, DIALOG amber and not dim, each state still carried by its word, unable and fail never coloured, beside the P6 per-span check `cross_list_rows_style_each_span_beside_its_word_on_a_terminal` (a11y-plan/visual-design-verification fct-es7le4as) (a11y-plan/aria-patterns-roles fct-uok334yy) (a11y-plan fct-lchghj7u)
@binding n=2
**a11y-scope-summary TUI boundary ↔ aria-patterns-roles P4**: the openpty wrapper must pass P4's three outer-PTY cases with the full assertion on Linux and macOS (zero viola bytes byte for byte against an unwrapped fake-agent run, keys unblocked past the atomic paste, focus/mouse/resize and terminal replies never moving the wheel), while windows-2025 keeps its F-W3 platform fact unchanged (a11y-plan/a11y-scope-summary fct-itb2wxsm) (a11y-plan/aria-patterns-roles fct-dx4kie52) (a11y-plan fct-csjr2vg7)
@binding n=3
**visual-design-verification NAME truncation ↔ a11y-scope-summary TUI passthrough**: the unwrapped-NAME cut reads its width from `viola_pty::host_size()`, so it holds on the Unix legs only where the openpty resize reports the host size as ConPTY does; a wrapped NAME, piped output and --json must stay uncut on every OS (a11y-plan/visual-design-verification fct-es7le4as) (a11y-plan/a11y-scope-summary fct-itb2wxsm)
@binding n=4
**a11y-scope-summary feed-panic boundary ↔ its Owed Windows reading**: the feed-panic case must read no viola literal on all three OSes and the child's own bytes `e4 b8 ad` on Unix, while the Windows byte reading stays owed to "Windows-only live measurements" and is not claimed here (a11y-plan/a11y-scope-summary fct-yscgzykm) (a11y-plan/a11y-scope-summary fct-4etq6dn5)
@binding n=5
**a11y-scope-summary refusals ↔ the body's CI matrix**: the per-OS split must stand on the three legs: the server-verify refusal reads a peer pid on windows-2025 and ubuntu-latest only, on macOS an euid mismatch alone with no pid or start-time comparison, and the socket-directory refusals of the frame verbs and the start run on ubuntu-latest and macos-latest, their documents and hints identical across legs (a11y-plan/a11y-scope-summary fct-k76k2lf7) (a11y-plan/a11y-scope-summary fct-sjhco52d) (a11y-plan/a11y-scope-summary fct-rts3hgep) (a11y-plan fct-by4mf7d7)
@binding n=6
**aria-patterns-roles landmarks ↔ a11y-scope-summary P6 structure**: the BAY line, CAPTION literal, separator-only-with-unwrapped-rows and the HELP literal are fixed strings pinned by literal that must read byte-identical on every OS leg, an OS-only wording failing the pin (a11y-plan/aria-patterns-roles fct-ai3fe6uo) (a11y-plan/a11y-scope-summary fct-p4tq3d4m) (a11y-plan fct-vt5poetp)
