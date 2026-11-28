@history_item n=1 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (1. Test Scope Summary: 2 rows, amd-kjinpspo) — `decision/cli-tests-as-subprocess` gained the except «viola console takes no --json; it is tested under its own outer PTY, its signal its exit code and its diagnostics lines», and `decision/structured-json-process-logs` names the console role. Every console-action test this chunk adds stands under that exception: outer PTY, exit code and `console.ndjson` lines, no `--json` document of the console itself (the action's verb outcome must be read from records, not a console document).

@history_item n=2 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (1. Test Scope Summary: 1 row, amd-i74nnjtl) — `decision/cli-json-one-document` gained «viola console takes no --json, its two refusals fixed stderr pairs». An action's `unable` line shown on the console's own surface is not a console `--json` document; the one-document rule still binds the CLI verb the action goes through, so the console must show that verb's own refusal, not a re-worded one.

@history_item n=3 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (6. E2E Test Strategy: 2 rows, amd-gshzlqsw) — `decision/hint-names-its-cause` except: the console's two exit-1 causes print no `--json` document; `decision/home-readable-by-others-refused` adds `viola console` exit 1. Any new console exit cause the actions add joins `tests/cross_exit_causes.rs` with its own hint (`no_two_codes_share_a_hint`) under the same no-document exception.

@history_item n=4 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (2. Test Strategy: 1 row, amd-p35d5u47; 5. Integration: 2 rows, amd-eeqadykn; 5. Integration: 2 rows, amd-kwxfjagh) — `decision/home-another-user-may-read-refused`, `decision/home-read-rule`, `contract/strict-modes-refusal/console` (new), `contract/strict-modes-refusal/21` (console exits 1) and `/no-line` were extended to the console. A verb reached from the console keeps its own strict-modes outcome (exit 21 with the instance pair, no line) while the console itself exits 1: the tests must tell the two refusals apart.

@history_item n=5 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (6. E2E Test Strategy: 3 rows, amd-shomicly) — `decision/strict-modes-before-any-log` names viola console; `decision/tui-verdicts-from-records` now reads «the fleet console's verdicts come from its console.ndjson lines and its terminal's line mode, never from the screen»; `+ decision/console-restore-read-on-own-terminal` (the PTY seam exposes no master fd; restore read raw while the frame stands and cooked after quit, never equality with the mode before). Every selection/action verdict and the no-key-trap leave-by-keyboard cases must be read from `console-key`/`console-frame`/`process-exit` lines and line mode, and a new mode's exit by the same own-terminal read (scope carry cry-pjtbjcco names it).

@history_item n=6 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (3. Test Harness Contract: 2 rows, amd-rtbhloai) — `contract/log-line/process` gained `console` («console writes console.ndjson with or without an instance»), and `decision/process-logs-via-tracing-subscriber` its role file. Action lines the console logs extend this closed process/event vocabulary; the harness owns the grepped fields (`decision/harness-log-fields-owned-by-test-plan`), so a new console event kind needs its row here first. Marked PROVISIONAL (delegate, Q3).

@history_item n=7 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (11. Test Anti-Patterns: 1 row, amd-grv2mh3q; 10. Quality Gates: 1 row, amd-rseb4e6d) — `decision/literal-test-oracles` except: the relied-row count is read from the ledger's count constant with one literal pin (PROVISIONAL); `+ decision/boot-paths-declared-once` notes viola console's start passes the entry point without the shared start. If an action runs `revive`/`adopt`/open through the shared start, the boot path changes; oracles for keys, lines and exits stay literals.

@history_item n=8 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** (5. Integration Test Strategy: 1 row, amd-enlb46ez; 2 rows, amd-nj2ygtum) — `contract/strict-modes-refusal/no-line` now reads «viola console's start»; `decision/home-read-rule` and `contract/strict-modes-refusal/console` add the mid-run refusal: exit 1 with the home pair after the terminal's restore and one `process-exit` line, detail `strict-modes-failed`. An action taken while the board scan refuses mid-run must meet this path; PROVISIONAL (delegate viola-overseer).

@history_item n=9 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** (2. Test Strategy: 1 row, amd-akcdh6jz) — `decision/tests-never-read-host-sessions` binds «every start of a verb that reads the claude agents list in the background» to a test-scoped PATH, because the console's agents thread made every console test start read the host's claude. Every new console-action test start keeps the planted/empty bin dir (scope carry cry-qjw64paz), and a child verb the console spawns inherits that PATH.

@history_item n=10 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (Path 5 as landed, amd-q6uxfnkg) — `tests/tui_wheel.rs` and `tests/cli_wheel.rs` hold pause/release, `human-typing` refusals with detail `null` | `manual-pause`, hints without `release`, `release-from-driver` exit 20, focus/mouse/resize never taking the wheel. Pause and release reached from the console must give the same outcome; console keys typed in the console's own terminal are not the wrapped child's editing keys, so the wheel rows (`decision/human-takes-wheel-on-edit`, `decision/focus-mouse-resize-keep-wheel`) must not move by a console action except through the verb itself.

@history_item n=11 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (Path 5 and Path 2 span the running turn, amd-frt6vbye) — a test that sends twice must end the first turn with a scripted Stop (the fake agent's interactive submit fires none); `send_after_a_confirmed_send_is_turn_running_until_turn_ended` pins the `[/ ] unable … turn-running` line plus `hint: a turn is running; viola wait builder first`. A console send-twice case needs the gated script, and its `unable` line is exactly that verb line.

@history_item n=12 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** (Path 2's local verdicts, amd-nsngbyjo) — the human mode lines: `[  ] unconfirmable  <name>  local command, no measured post-condition` on stdout exit 0, a post-condition-confirmed `/clear` printing `[RB] read back`. These are the `contract/cli-human-lines/read-back` / `contract/viola-send-human-mode/*` lines the console's send readback shows («the frame verb's own reply»).

@history_item n=13 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** (amd-gsat5bin) — the readiness gate waits up to `GATE_MAX_WAIT` 8 500 ms; `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` (exit 10, key reaches the child); kill lines move only for a floor by design with a planted-hang control both ways. A console send's readback can take seconds past the view's one-second re-read (scope premise): bound it from the product constants, never by raising a kill or `WITHIN`.

@history_item n=14 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the mutants profile's paste-hint class, amd-4gylryuv) — `send_window_` and `send_under_the_paste_hint_` classes take 30 s under `mutants`, pinned by `mutants_paste_hint_override_holds_the_class_above_the_default_kill`. A console send case waiting out the gate must join a named class or stay under the default kill (`decision/send-window-paste-hint-30s-kill`).

@history_item n=15 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Test Harness Contract rows, amd-tveox2hq, amd-kmpo2mdb, amd-gjhi22be) — `decision/root-wait-bound` at 12 s below the 15 s mutants kill and the 20 s floor; `decision/within-7s-wait-bound` now WITHIN = 12 s; `decision/mutants-kill-below-cargo-mutants-timeout` at 15 s, a stated departure from fixing slow cases by removing slow work. Every console-action wait (readback, mode change) is bounded by WITHIN and stays below these kills.

@history_item n=16 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** (amd-xmoq3n4c, amd-aboqz4ml, amd-grwbhpbc, amd-lout42p2, amd-rlsw5g76) — `contract/driver-send/typed-text`, `contract/viola-send/paste-framing`, `contract/send-event-log/send-issued` gained the paste-cut arm; `+ contract/viola-send/paste-cut`, `+ contract/viola-send/during-a-blocked-paste`, `+ contract/fake-agent-cli/stop-reading`; `decision/send-refusal-order` gained two paste-cut rungs. A console send shows these refusals as the verb shows them; a blocked-paste case uses `--stop-reading`.

@history_item n=17 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** (amd-6wgz6p2f, amd-anzg3tmd, amd-fi4mla3k) — `decision/send-confirmation-exact-match` replaced by `decision/send-confirmation-trailing-trim-match`; `contract/viola-send/records` holds the submitted text; fake agent `--trim-prompt-end`. A console readback reports the verb's confirmation; text typed in a console text entry is sent as typed and confirmed under this tolerance.

@history_item n=18 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the typed text, the `empty-text` refusal, amd-rreyzalx) — the typed text drops every trailing CR/LF; `send` refuses control-character → empty-text → human-typing. A console text entry whose Enter ends the text must not leave a CR the verb would type; an empty entry meets `empty-text` (exit 13), shown as the verb's `unable` line.

@history_item n=19 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** (amd-u2ylgksz) — every inner CR / CR LF is typed as one LF (`decision/sent-text-typed-form`, guards `typed_text_drops_every_trailing_newline`, `typed_text_every_inner_cr_is_one_lf`). A console send-entry's raw-mode Enter/CR bytes meet this normalisation; a key-sequence decoder must not turn a typed CR into a submitted newline inside the text unknowingly.

@history_item n=20 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (amd-ea56tn73, amd-owwzhlbv, amd-ponzfcew) — local commands are classified by name and confirmed only by post-condition, else `unconfirmable`; `decision/local-command-post-condition` (/clear, /compact, /mcp; /context added by 2026-11-19-context-as-a-local-command, amd-wh4fr3uz). A slash text sent from the console goes through `send` and shows its unconfirmable or confirmed line; a leading-slash argument is never a prompt (`decision/leading-slash-never-a-prompt`).

@history_item n=21 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (/compact's own window, amd-np7arnqh) — `/compact` waits inside `COMPACT_CONFIRM_WINDOW` 40 s (`decision/compact-confirm-window`). A `/compact` readback from the console may take 40 s: any console case of it must not wait on it under the default kill.

@history_item n=22 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (amd-6op5bd23, amd-4m3eiqpq, amd-eykl5id3) — `contract/viola-send/turn-running` covers a standing switch; `contract/exit-21-detail/socket-dir-refused` and `decision/frame-verbs-check-instance-first` now count nine frame verbs; `decision/unverified-cli-send-only` refuses `answer` and `switch` unverified-cli. The console's switch leg (carry cry-mf2w3ioc, v2-31) goes through `viola switch`, and the count of frame verbs the console reaches is nine.

@history_item n=23 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (amd-tps3btk4, amd-irpcaky7; ratified for F-A by amd-bp42qzsp) — focus joins `decision/strict-modes-before-any-log` and `contract/strict-modes-refusal/no-line`; `+ decision/window-raise-proven-live` (a real raise only by gated R3, rehearse form never live). Fork F-A's ratified arm makes focus «a command for the human only»: the console (human-driven) may reach it, but a real raise from a console action is GATED, proven over the fakes only.

@history_item n=24 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (amd-73ontppm, amd-s7tr7co2, amd-4e674c4h, amd-yyjvnoux, amd-2o2uut5r, amd-j2dzflf5) — the fake agent stands as foot, the terminal stand-in; `+ decision/window-placement-proven-live`; `+ decision/echo-oracle-skips-allowed-word`; `contract/run-start-events/order` gains the window line under `--origin`. «Open a child» from the console is served by fake foot/hyprctl in tests; real placement only live; refusal-echo oracles skip the one allowed word.

@history_item n=25 marker=2026-11-25-placing-without-moving-the-view
**2026-11-25-placing-without-moving-the-view** (amd-2batlkms) — `decision/window-placement-proven-live` adds «a placing open's kept view only by the gated live reading R7». A console open-a-child action claims no kept view beyond the fakes; any such clause is GATED.

@history_item n=26 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (amd-xjvduabe, amd-trdsiny2, amd-ajzq5lzc, amd-3276zmh4, amd-srhfftob) — `decision/window-reopen-proven-live`, `decision/hint-names-its-cause` except boot-time-unread, `+ contract/fake-foot/spawn-record`, `+ decision/stand-in-terminal-tree-teardown` (one shared teardown kills only leaves whose parent runs). A console action that spawns a stand-in terminal must read the spawn record and tear its tree down through that shared teardown, never its own kill loop.

@history_item n=27 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (amd-qvnp25qd) with 2026-10-17-cli-machine-contract (amd-jqjfbovx) and 2026-10-31-exit-cause-code-catalogue (amd-dtswm36t) — revive's exit-1 causes in preflight order, its `--json` refusal codes (now ten after 2026-11-05-linux-only-code-and-ci, amd-uvcoizuv), the human `unable:`/`hint:` pair, and `contract/revive-json/pump-child-error` (no document once the child held the terminal). Revive from the console is a start that takes a terminal: the console must not interleave its own frame with a child that holds a terminal, and its refusal shows as revive's `unable` pair.

@history_item n=28 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (amd-uztwp2ux, amd-h4gpb7h6) — `contract/cli-human/internal-error` silent at adopt's start; `decision/session-live-read-fails-open` on revive and adopt; `decision/real-claude-local-only` adds «the gated live-reading scripts». The console's live clauses are GATED (scope: no live reading in the first pass) and reach the real CLI only through such a gated script, never CI.

@history_item n=29 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (amd-qynxzky5) — `+ decision/run-under-test-owned-user-home` (a start writes the Claude config, so every test-started run/revive runs under a test-owned user home). A console action that starts revive/adopt/open is a start: its test must give that child a test-owned HOME.

@history_item n=30 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** (amd-vahqpute) — `decision/wrapper-drop-guard` (parent-death tie), `+ decision/no-untied-viola-spawn` (census over every root test source counts any spawn of the viola binary outside the tied builder), `+ decision/tied-child-spawn-thread`. A new console-action test that spawns `viola console` or a verb must use the tied builder on a thread that outlives it, or the census reds.

@history_item n=31 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (amd-xffhdjct, amd-lxnavnej) — all CLI frame verbs run `check_instance` through `refuse_untrusted` before any role file or log opens, exit 21 with the instance pair; the order trap: opening a role file narrows an existing home to 0700, so a check placed after it passes a 0770 home. If the console calls verbs in-process (scope premise), it must keep each verb's check ahead of any log the console already holds open.

@history_item n=32 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`, amd-3fwhzts6) — trap: a control whose neutralised guard lets a product process start needs a bounded witness spawned through `tests/support/cli.rs` `Running` (an unbounded one hung 27 minutes). Remove-the-guard controls for the console's no-key-trap and refusal paths must be bounded.

@history_item n=33 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the bare temp-dir home, amd-tf6f4x2s) — trap: «an oracle moved by a shared body can hide a test that never reached its subject» (a strict-modes refusal sharing one body with a dead instance). A console action case asserting an `unable` line must prove it reached the verb it names, not a refusal of the console's own home.

@history_item n=34 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (amd-tlolzudh, amd-ji43jawo, amd-xo4msppk, amd-g7svdriu) — the exit-cause table is built (`tests/cross_exit_causes.rs`, `no_two_codes_share_a_hint`, `no_hint_names_release`, `every_code_is_a_code_word`); exit 21's document names its closed cause; built hints in `src/human/hints.rs`; the `cli` catch site prints `internal-error` under `--json` and `json_asked` reads `--json` only before `--`. The `unable` lines a console action shows are these hints and causes byte for byte; driver hints never name release (`decision/no-release-in-driver-hints`) while the human console may offer release.

@history_item n=35 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (amd-gelnrgex, amd-ijyzrs56, amd-n6sbx22x) — exit 21's sixth cause `socket-dir-refused` on the frame verbs alone; the socket-directory negatives and `setting_does_not_disable_the_socket_dir_check`; a test's endpoint directory made through `create_private_dir`, never a bare tempdir. Console-action cases reaching frame verbs inherit these causes and the private-endpoint rule.

@history_item n=36 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (amd-vvb3f4ku, amd-ztu6cp6i, amd-tahrpyqs, amd-3hn5t2j2) — the reply screen (refusal not a code word reads `unknown`, string detail not a code word reads `null`), the fault screen `sanitised_fault`, `channel-response` closed detail, the exit-2 paths of answer/send writing a fixed stderr line and no document, the error-body enumeration with `NOT_ERROR_BODIES` (which names the console's refusals). A console showing a verb's refusal must show the screened words, and any new console error writer must be a row of `contract_error_bodies.rs`.

@history_item n=37 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (Path 5 records the classifier's tables, amd-e54mtnrn) — terminal replies are a closed list read as not editing (`classifier_terminal_reply_is_not_editing`), keys `n`, `t`, `m` still take the wheel; the reading on a real terminal is a local live measurement, not a suite, because the outer-PTY tier writes no terminal reply. A console key-sequence decoder (carry cry-pjtbjcco) meets the same reply shapes (focus, DA, CPR) on the console's terminal; its handling of them cannot be proven at the outer-PTY tier.

@history_item n=38 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** (amd-r32l3vjd) with 2026-10-17-cli-output-discipline (amd-o6hxlf3v) and 2026-11-04-linux-and-macos-parity (amd-pgmwvwfm) — `tests/cli_output_plain.rs` parses SGR parameters, never screen text (`tests/support/sgr.rs`); `NO_COLOR`/`TERM`/`COLORTERM` are control-negative cases; the stale-row dim witness under the outer PTY is the guard of `decision/human-tty-through-outer-pty`. A selection highlight in the console is asserted only by SGR parsing and lines, never the screen (`decision/tty-asserted-only-on-verb-sgr`).

@history_item n=39 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** (amd-4cs5rgtp) — `list` rows escape every control character (`decision/list-row-control-escaping`), the board pinned by literals (`CAPTION`). With scope carry cry-qjw64paz (an unwrapped NAME is not cut separately), a selection names its row by the board item, never the drawn (escaped, cut) text.

@history_item n=40 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (amd-c4x5jj55, amd-fwkd577g) — the fake agent as `claude` answers agents from files beside its exe, no env var (`decision/agents-stub-from-files-no-env`); `e1_send_naming_an_unwrapped_row_is_unreachable`: a row name is never a target. A console action on a selected unwrapped row reaches `unwrapped-name` exit 21 (`decision/row-name-never-a-target`, `decision/unwrapped-name-never-a-target`), shown as its `unable` line.

@history_item n=41 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (amd-quvutcb3) — `tests/contract_bounded_readers.rs` holds every external reader bounded or named (`product_readers_are_each_bounded_or_named`). The console's key-sequence decoder reads stdin (the scaffold's reads are at most 64 bytes): it must stay a bounded read or a row of `BOUNDARIES`; a console text entry is bounded by `MAX_FRAME` as the verbs are.

@history_item n=42 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (amd-zrj7cxkl, amd-fngxnwic) — Linux only; the OS-gate census reads a gate naming windows or macOS as a fault even when named (`decision/linux-only`, guard `contract_os_gated_tests`). New console-action tests take no Windows/macOS gate; any `cfg` gate is a `GATES` row.

@history_item n=43 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** (amd-qqdv5cf3) with 2026-10-31-exit-cause-code-catalogue (amd-gofdfhay) and 2026-10-26-server-verification-before-any-frame (amd-zw2his22) — the E2E regex `^(path[0-9]*|tui|mcp|cross|chaos|contract)_`, the integration layer its complement, `tests/support/start.rs` shared start helpers, the `security_negatives` suite prefix. A console-action binary's name decides its layer (`decision/integration-binary-surface-prefix`, `decision/e2e-layer-by-binary-prefix`); a `tui_`/`cross_` console binary is E2E.

@history_item n=44 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** (amd-3qaxqoxx) with 2026-11-04-linux-and-macos-parity (amd-c7gfdhws) — the shared root helpers `tests/support/{home,outer_pty,fake,events,cli,ndjson,frozen}.rs`, no root test file defines its own copy (`decision/shared-e2e-helpers`, `decision/one-shared-test-helper`). A console keystroke driver and its log reader extend these shared files, not a per-file copy.

@history_item n=45 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (amd-aml2265l, amd-uwsvrnci) — `contract/event-log/start-wheel-record` (trust keys passing the wrapper write a human-input wheel before session-start); `+ decision/start-lines-read-from-start-offset`. A console action reading a revived or re-opened instance's records reads from that start's own offset, a reused name keeping earlier starts' lines at the head.

@history_item n=46 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (amd-udylcpzx) — `+ decision/live-failing-reading-not-verification`. The chunk's GATED clauses (no live reading in the first pass) are recorded as owed, and a later failing or confounded live reading is recorded as read, never verification.

@history_item n=47 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0, amd-rdzriqfg) — E4 reads N/A, viola has no web or graphical frontend; `absence/browser-suite`. The fleet console stays a terminal surface tested through the outer PTY; no browser or GUI test layer opens for its actions.
