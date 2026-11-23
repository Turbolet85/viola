@history_item n=1 marker=amd-vwmlqbwm
**amd-vwmlqbwm** — (2026-11-23-model-and-effort-at-launch) `contract/run-start-events/order` went from 3 to 4 start records (wheel, lineage, launch, session-start). This is the closest precedent for a new wrapper-written log kind. If the `window` line is written at start, this order row and its readers must say where it falls. Log-only placement after session-start is the other choice.

@history_item n=2 marker=amd-c5n5bfiq
**amd-c5n5bfiq** — (model-and-effort) `contract/boot-readiness/events` now reads 4 complete start lines with launch third. A `window` record written among the start lines would leave the harness readiness probe and this row stale. The chunk has to keep it out of lines 1–4 or move both.

@history_item n=3 marker=amd-voiwyzhm
**amd-voiwyzhm** — (model-and-effort) `contract/viola-list-json/items` gained `model?`/`effort?` after `role?`, PROVISIONAL on the delegate's word. This chunk's `window?`/`origin?` extend the same item list and row. The MCP `list` item stays equal to it (`cli-mcp-identical-payloads`).

@history_item n=4 marker=amd-um343dxz
**amd-um343dxz** — (model-and-effort) `decision/snapshot-replay-fields` recovers links, agent_session_id, wheel, lineage and launch, with the last lineage and launch line winning. The snapshot `window?` replayed with the last line winning joins this fixed field set and its last-line rule.

@history_item n=5 marker=amd-wuzhcs73
**amd-wuzhcs73** — (model-and-effort) `decision/snapshot-replay-from-log` gained the launch pair. Its proof went through `list --json` after a killed wrapper's cut snapshot, because a revive over a replayed snapshot is refused `cwd-missing`. A `window?` replay witness hits the same trap and needs the same route.

@history_item n=6 marker=amd-webnvrk3
**amd-webnvrk3** — (model-and-effort) `contract/fake-agent-receipt/start` carries the model and effort handed on argv, as proof the pair reached the child. `viola open --model/--effort` passes them through `run`. The receipt is the existing oracle for that hand-off.

@history_item n=7 marker=amd-tjq3cxd7
**amd-tjq3cxd7** — (model-and-effort) the config.json default-pair reader joined `decision/bounded-readers-keep-the-cap` and the BOUNDARIES list. This is the precedent that any new reader this chunk adds (the `clients` poll, an ancestry read) joins the same row and table.

@history_item n=8 marker=amd-vdo6ha64
**amd-vdo6ha64** — (2026-11-22-lineage-registry) `list --json` items gained `parent?`/`role?`. `viola open --parent/--role` feed this lineage, and its defaulting to the caller's `VIOLA_NAME` lands in the same fields.

@history_item n=9 marker=amd-qibcnfiz
**amd-qibcnfiz** — (lineage-registry) the replay gained lineage with the last line winning, and a later line with no parent clears it. The start order gained the lineage line, on delegate overseer F3, PROVISIONAL. This is the pattern `window` follows for a last-line-wins replayed field.

@history_item n=10 marker=amd-k26auyso
**amd-k26auyso** — (lineage-registry) boot readiness grew to 3 lines. New `decision/supervisor-spawn-strips-bridge-env`: the harness wrapper spawn drops inherited names. `open` spawning `run` through foot must likewise control what env crosses, and the parent default reads `VIOLA_NAME`.

@history_item n=11 marker=amd-5mya4wvt
**amd-5mya4wvt** — (2026-11-14-hyprland-seam) `decision/fixtures-recorded-by-live-verify`: the Hyprland replies are refreshed only by the test-only `viola-hypr-tap`. Any new `clients` reply this chunk's fake serves, keyed to the stand-in foot's pid, rests on that recorder's set.

@history_item n=12 marker=amd-pfdqr7rr
**amd-pfdqr7rr** — (hyprland-seam) The hygiene walk holds each Hyprland reply to its closed per-request keep-list. The fake agent gained a hyprctl personality chosen by its own file name, configured from files beside its exe, as an exception to argv-only. viola-hypr-tap scrubs. A stand-in `foot` personality and a pid-keyed `clients` reply extend all four rows.

@history_item n=13 marker=amd-eefnob23
**amd-eefnob23** — (hyprland-seam) `decision/dirty-payload-refuses-recording` covers a Hyprland reply set refused whole by file and closed code. A new recorded `clients` variant falls under it.

@history_item n=14 marker=amd-erzqi2ha
**amd-erzqi2ha** — (hyprland-seam) Hyprland fixtures joined `decision/fixtures-walked-for-hygiene` with planted cases. `decision/fake-agent-argv-only` gained the hyprctl-files exception. A foot stand-in mode read from beside the exe is a further exception the row must name.

@history_item n=15 marker=amd-trilsjw7
**amd-trilsjw7** — (hyprland-seam) `decision/fake-agent-as-claude`: the only external boundary is served by viola-fake-agent as claude, and as hyprctl in a test-scoped bin dir. `foot` becomes a third external child process, so this row grows a stand-in clause.

@history_item n=16 marker=amd-itty4qan
**amd-itty4qan** — (hyprland-seam) `decision/fakes-for-external-services` lists the real hyprctl replaced by the same binary. The real `foot` is another external service the tests must replace.

@history_item n=17 marker=amd-rxbjdtib
**amd-rxbjdtib** — (hyprland-seam) `decision/ci-never-real-claude` was extended to "or the real hyprctl" because no CI image carries it. The same reasoning applies to a real `foot` and a real move dispatch. R2 stays local.

@history_item n=18 marker=amd-hs3diylx
**amd-hs3diylx** — (hyprland-seam) the hyprctl reader joined `decision/bounded-readers-keep-the-cap`, its cut reply read as oversize under a BOUNDARIES row. The seam's first product caller rests on that cap.

@history_item n=19 marker=amd-srg4fj2s
**amd-srg4fj2s** — (hyprland-seam) `decision/property-tests-on-parsers` names the hyprctl reply parsers among the 512-case proptest surfaces. If open parses `clients` or the move reply through new code, that code joins them.

@history_item n=20 marker=amd-4gd3soxf
**amd-4gd3soxf** — (hyprland-seam) `decision/parser-property-fuzz-coverage` names the hyprctl reply parsers, fuzzed as `hyprctl_json`. Any widened reply parsing stays under that fuzz target.

@history_item n=21 marker=amd-sbmeablu
**amd-sbmeablu** — (hyprland-seam) `decision/property-tests-with-proptest`: the Hyprland reply suites commit their own proptest-regressions files.

@history_item n=22 marker=amd-gxqopq3w
**amd-gxqopq3w** — (hyprland-seam) `decision/no-production-data-in-tests`: fixtures come from `viola verify` probe text or a test-only recorder's capture, scrubbed. Applies to any recorded foot or `clients` data.

@history_item n=23 marker=amd-jqhveuv3
**amd-jqhveuv3** — (2026-11-14-hyprland-exact-frame-bound-pinned) `decision/fakes-for-external-services` except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests. The record-move code tests over `hypr_tap.rs` stubs and the seam's new product caller live under this split.

@history_item n=24 marker=amd-zt4mirze
**amd-zt4mirze** — (2026-11-22T17-29-41Z-wrap) `contract/fake-agent-cli/argv-only-options`: options come from argv plus a fake-args file beside the exe, argv winning, never env. Standing as hyprctl, the mode comes from a file. A stand-in foot must be configured the same way: no env seam.

@history_item n=25 marker=amd-ufypq5a6
**amd-ufypq5a6** — (2026-11-22T17-29-41Z-wrap) `decision/fake-agent-argv-only` except gained the fake-args file beside the exe. Any further fake personality (foot) is one more named exception here.

@history_item n=26 marker=amd-c4x5jj55
**amd-c4x5jj55** — (2026-10-16-unwrapped-sessions-on-the-board) The fake agent answers from files beside its exe, with no env var, by the founder's own word ("no third env seam"). Every test-started `list`/`revive` runs under a test-scoped `PATH`. `open` finds `foot` on `PATH`, so its tests plant the stand-in in a test-scoped bin dir, never via env.

@history_item n=27 marker=amd-mrxt76c2
**amd-mrxt76c2** — (2026-10-26-server-verification-before-any-frame) Trap: on the tmpfs e2e home each fake-agent copy is written, not linked. A child forked by one thread inherits another thread's write handle (ETXTBSY), so every copy is planted before any child starts. Planting both a foot and a hyprctl stand-in is exactly this case.

@history_item n=28 marker=amd-sreor2sv
**amd-sreor2sv** — (2026-10-27-pinned-copy-race-and-reproducible-builds) `decision/forced-window-hook-is-test-parameter`: a forced-window hook passed to a private function under test is a test parameter, not a seam. No env, no feature, no mockall double, nothing added to the test-seam list. This is the form for driving the 5 s window poll or the move path without a live compositor.

@history_item n=29 marker=amd-qoo7oxzm
**amd-qoo7oxzm** — (2026-11-21-mcp-calls-bounded) `decision/time-faked-through-seams`: time is faked only through injected seams, with no paused Tokio clock. The MCP call deadline is fired by a signal, never by waiting out 55 s. This is the precedent for open's 30 s liveness bound and 5 s/100 ms window poll: inject them, never wait them out.

@history_item n=30 marker=amd-tveox2hq
**amd-tveox2hq** — (2026-11-15-mutation-and-coverage-baselines-green) `decision/root-wait-bound`: root waits on a child are bounded at 12 s, below the 15 s mutants kill and cargo-mutants' 20 s floor. A test that would block on open's 30 s `not-live` bound crosses these kills.

@history_item n=31 marker=amd-kmpo2mdb
**amd-kmpo2mdb** — (mutation-and-coverage-baselines-green) `decision/within-7s-wait-bound` now reads WITHIN = 12 s, the bound of every test wait not bounded by a per-test kill.

@history_item n=32 marker=amd-gjhi22be
**amd-gjhi22be** — (mutation-and-coverage-baselines-green) `decision/mutants-kill-below-cargo-mutants-timeout`: the 15 s mutants default kill. Raising it was a stated departure, and a slow case is otherwise fixed by removing its slow work. New open waits must stay under it.

@history_item n=33 marker=amd-vahqpute
**amd-vahqpute** — (2026-11-17-booted-wrapper-does-not-outlive-its-test) `decision/wrapper-drop-guard` now ties every long-lived viola child to its test thread by a parent-death kill. New rows: `decision/no-untied-viola-spawn` (census over every spawn ending in the viola binary constant) and `decision/tied-child-spawn-thread`. `open` spawns foot, which spawns `viola run`, so tests must keep that chain tied and the census green.

@history_item n=34 marker=amd-qynxzky5
**amd-qynxzky5** — (2026-11-18-session-folder-trusted-by-viola) `decision/run-under-test-owned-user-home`: every test-started run and revive runs under a test-owned user home, because the start's trust step writes the Claude config. The `run` that `open` starts is covered. `decision/harness-wrappers-under-session-user-home` was added too.

@history_item n=35 marker=amd-5bz3flwz
**amd-5bz3flwz** — (session-folder-trusted-by-viola) `decision/trust-start-folders-outside-repository`: trust-start test folders sit under /tmp, outside any repository. This bears on `open --cwd` cases and on R2's scratch home "outside every repository".

@history_item n=36 marker=amd-cpftmyp5
**amd-cpftmyp5** — (2026-11-17-standing-live-rig) `decision/standing-live-rig`: live rounds against the real claude reuse one committed test-only rig, viola-live-rig, with rehearsals in CI and a live arm refusing under CI. R2's new `scripts/r2-open.sh` must reconcile with this row: become a rig arm, or carry a named exception as the SF and KR arms did.

@history_item n=37 marker=amd-udylcpzx
**amd-udylcpzx** — (2026-11-17-trust-record-ledger-row) `decision/live-failing-reading-not-verification`: a failing live reading is recorded as read, never as verification. A confounded one leaves the row unverified and its re-read owed. This governs R2's verdict for v2-07.

@history_item n=38 marker=amd-zrj7cxkl
**amd-zrj7cxkl** — (2026-11-05-linux-only-code-and-ci) OS scope: Linux only, CI on `ubuntu-latest`, the 0.2.0 window features on Hyprland. Open's off-Hyprland path (`unsupported`/`unreachable`, no env read) is the only off-compositor branch. No other-OS gate is admissible.

@history_item n=39 marker=amd-fngxnwic
**amd-fngxnwic** — (linux-only-code-and-ci) The OS-gate census (`contract_os_gated_tests`) reads a gate naming windows or macOS as a fault even when a row names it. Any gate on open's tests must be named in GATES with a reason.

@history_item n=40 marker=amd-o6hxlf3v
**amd-o6hxlf3v** — (2026-10-17-cli-output-discipline) `viola --help` is pinned by the `HELP` literal in `tests/cli_output_plain.rs`. `help_lists_only_served_verbs` checks each served verb. Open changes the `setup:` row to `open, run, revive, verify, plugin install`, so the literal and that case move.

@history_item n=41 marker=amd-fakhmvq4
**amd-fakhmvq4** — (2026-10-17-optional-plugin-install) `help_lists_only_served_verbs` reads the table's words from the group rows alone, with `plugin install` as one entry of `setup:`. This is the precedent for adding a verb to the `setup:` group and its served-verb `--help` check.

@history_item n=42 marker=amd-bb4rhxnd
**amd-bb4rhxnd** — (2026-10-26-explicit-session-links) New verbs' result lines were pinned by literal in their own test file beside HELP and the board. A printed time was asserted by shape. `help_lists_only_served_verbs` was revised as verbs joined. This is the template for open's human lines.

@history_item n=43 marker=amd-jqjfbovx
**amd-jqjfbovx** — (2026-10-17-cli-machine-contract) The completeness case `every_flag_variable_and_config_key_is_a_row_of_the_table` walks `viola --help`'s Options, product `VIOLA_*` literals and config keys against the controls table. A test-started viola drops VIOLA_NAME/VIOLA_DIR/VIOLA_BIN. Open's parent default reads `VIOLA_NAME` and its tests must set it per child.

@history_item n=44 marker=amd-tlolzudh
**amd-tlolzudh** — (2026-10-31-exit-cause-code-catalogue) `tests/cross_exit_causes.rs` is built. Every case asserts the exit code, the `--json` document and a last `hint:` naming its cause, with self-checks `no_two_codes_share_a_hint`, `no_hint_names_release` and `every_code_is_a_code_word`. Open's new `terminal-not-found` and `not-live` codes join this table.

@history_item n=45 marker=amd-con7cy2d
**amd-con7cy2d** — (exit-cause-code-catalogue) A stale holder refuses a start with `stale-heartbeat` (exit 1). A live holder or fresh replayed beat refuses with `already-live`. `Frozen` (SIGSTOP) is the real stale holder. Open reads both before any spawn, and its cases stand on the same guard.

@history_item n=46 marker=amd-dtswm36t
**amd-dtswm36t** — (exit-cause-code-catalogue) `revive --json` prints one exit-1 document per refusal of the start, including the shared run refusals. A `pump_child` error prints none. This is the closest precedent for open's `{"v":1,"error":"<code>","detail":null}` documents over a start it hands to `run`.

@history_item n=47 marker=amd-g7svdriu
**amd-g7svdriu** — (exit-cause-code-catalogue) Under `--json` the cli catch site prints `{"v":1,"error":"internal-error","detail":null}` with empty stderr. `json_asked` reads `--json` only before a `--` separator. Open has an `-- claude` tail, so this parsing rule applies directly.

@history_item n=48 marker=amd-tahrpyqs
**amd-tahrpyqs** — (2026-10-31-sanitised-error-surfaces) The exit-2 paths of `answer`/`send` write a fixed stderr line whatever `--json` says, with no document, named in `NOT_ERROR_BODIES`. Open's fixed line for a word after `--` other than `claude` (beside `PAIR_AFTER_DASHES`) is the same shape and needs the same naming.

@history_item n=49 marker=amd-3hn5t2j2
**amd-3hn5t2j2** — (sanitised-error-surfaces) `tests/contract_error_bodies.rs` enumerates every error document with a census of writers (`product_error_writers_are_each_named`, `thiserror_messages_interpolate_nothing`). Open's new error documents must be named writers with witnesses.

@history_item n=50 marker=amd-vgmbfb4o
**amd-vgmbfb4o** — (sanitised-error-surfaces) `cli_error_documents_hold_no_path_or_upstream_text` holds each CLI `--json` failure to one document, a literal exit, empty stderr, no ESC and a clean body. Open's refusals (`terminal-not-found`, `not-live`, …) need the same witness.

@history_item n=51 marker=amd-64rglrie
**amd-64rglrie** — (2026-10-26-home-and-code-bearing-file-integrity) A start runs `check_home_tree` before the wrapper log opens. A refused tree exits 1 with the home or instance pair, no log, no line, no child (`run_refuses_a_widened_home_tree_before_its_log`). Open's `strict-modes-failed` before any log extends this.

@history_item n=52 marker=amd-4d4qi4t7
**amd-4d4qi4t7** — (home-and-code-bearing-file-integrity) A strict-modes refusal exits 1 for run/mcp/revive/verify and 21 for every other CLI verb, and a refused check writes no line. Open's exit 1 makes it a fifth exit-1 verb, so the `strict-modes-refusal` rows must name it.

@history_item n=53 marker=amd-lxnavnej
**amd-lxnavnej** — (2026-10-26-explicit-session-links) The instance check runs before any role file or log opens. Trap: opening a role file narrows an existing home to 0700, so a check placed after it passes a 0770 home. A remove-the-guard control read red. Open's check order must keep this.

@history_item n=54 marker=amd-3y22aq63
**amd-3y22aq63** — (2026-10-10-budget-governor) Strict-modes cases assert an absence: no line added to the role log. The group-writable case reads the home's mode while the start still runs, because a check that wrongly passes runs on as a wrapper. Open's refusal witness needs the same care.

@history_item n=55 marker=amd-ijb4wtz4
**amd-ijb4wtz4** — (2026-11-14-budget-gate-removed) `contract/viola-send/exit-11`: exit 11 is retired and never renumbered or reused. Open's scope says "no exit 11".

@history_item n=56 marker=amd-wxxquupj
**amd-wxxquupj** — (2026-11-20-a-new-cli-version-stamped-automatically) `decision/single-writer-state-files`: only the wrapper writes `snapshot.json`, including a keyed start's own verify. This is why the window record is the wrapper's and never open's. The single-writer case must still read byte-identical after `open`.

@history_item n=57 marker=amd-hwmom55r
**amd-hwmom55r** — (server-verification-before-any-frame) On-disk product state is never hand-written, save named exceptions (chaos, the impostor's test-built snapshot, the MCP handler suite). A test wanting a snapshot with `window?` gets it from a real wrapper, or joins the exception list explicitly.

@history_item n=58 marker=amd-xntsyub4
**amd-xntsyub4** — (budget-governor) `decision/state-files-never-hand-written` is scoped. A reader's own unit cases may plant a malformed state file only by altering the writer's own bytes. This is the route for `window?` replay or reader negatives.

@history_item n=59 marker=amd-t7mnlacx
**amd-t7mnlacx** — (home-and-code-bearing-file-integrity) The four readers moved onto `read_snapshot_or_replay`. Over a replayed snapshot the collision check reads liveness from the beat alone, and `live_endpoint` gives None. Open's liveness wait via `live_endpoint` and its pre-spawn collision read inherit these semantics.

@history_item n=60 marker=amd-irieg6c7
**amd-irieg6c7** — (home-and-code-bearing-file-integrity) E5 is built (`tests/chaos_snapshot_replay.rs`). Six product readers take the replay, and a fresh beat over a replayed snapshot refuses `already-live`. A replayed `window?` and open's pre-spawn reading of a replayed parent fall under this suite.

@history_item n=61 marker=amd-kdzdt5fs
**amd-kdzdt5fs** — (2026-10-10-self-healing-state) The read-or-replay function as landed. A field no line gave is absent, no file is written, and each replay logs one `state-recovered` line. This is the base `window?` replay follows. A replayed snapshot has no cwd or endpoint.

@history_item n=62 marker=amd-tgf2f6sm
**amd-tgf2f6sm** — (2026-10-10-session-links) `state_events.rs` round-trips every kind by the rule "length of `EventKind::ALL`". A line of a kind no build knows is counted, not returned. The new `window` kind must join ALL and the round trip, and older builds read it as unknown.

@history_item n=63 marker=amd-6grdlnwe
**amd-6grdlnwe** — (2026-10-19-dialog-hold-for-the-driver) `EventKind::ALL` 16 and `WAIT_WAKE` 6 are counted by rule. A new kind moved the round-trip count. `contract/wait/wake-kinds` lists the waking kinds, and the log-only `window` kind must not wake `wait`.

@history_item n=64 marker=amd-hlgfwft6
**amd-hlgfwft6** — (self-healing-state) The event reader's three counts (`three_counts`): torn, unknown kind and unknown fields. A known kind with unknown keys is counted and still returned. `window` lines and new snapshot keys meet these counts in older readers.

@history_item n=65 marker=amd-4cs5rgtp
**amd-4cs5rgtp** — (2026-10-11-the-board-viola-list) The board is pinned by literals (`CAPTION`, `list_human_board_is_the_literal_rows`) and `board::read` takes the replay. Open's scope keeps the six-column board unchanged, and those literals are the guard.

@history_item n=66 marker=amd-krbwmvhv
**amd-krbwmvhv** — (mcp-calls-bounded) `decision/cli-mcp-identical-payloads` holds one named exception (call-deadline). MCP `list` items gaining `window?`/`origin?` must stay equal to the CLI document less v, or name a new exception.

@history_item n=67 marker=amd-cnisrb36
**amd-cnisrb36** — (2026-10-17-mcp-server-for-drivers) `mcp_tools_list_is_exactly_the_five_tools_and_list_matches_the_cli` holds MCP list equal to the CLI. MCP tests own their `viola mcp` child, never TokioChildProcess. The equality case carries the new list keys.

@history_item n=68 marker=amd-quvutcb3
**amd-quvutcb3** — (2026-10-27-bounded-inputs-at-every-boundary) `tests/contract_bounded_readers.rs` holds literal BOUNDARIES and EXCEPTIONS tables plus a census of product reads (`product_readers_are_each_bounded_or_named`). Any new read of foot or compositor output fails the census unless capped or named.

@history_item n=69 marker=amd-ge6miket
**amd-ge6miket** — (sanitised-error-surfaces) Each BOUNDARIES row names a cap-driving case, not only a needle. A new bounded reader row this chunk adds owes one.

@history_item n=70 marker=amd-qqdv5cf3
**amd-qqdv5cf3** — (2026-10-17-end-to-end-path-binary-and-harness-legs) E2E is the binaries matching `^(path[0-9]*|tui|mcp|cross|chaos|contract)_`, and integration is its complement. An unmatched `binary()` regex is a parse error. Open's new test binaries' names decide their layer.

@history_item n=71 marker=amd-gofdfhay
**amd-gofdfhay** — (exit-cause-code-catalogue) The root package holds a `cross_` binary. `tests/support/start.rs` shares the start setups (`first_life`, `stopped_life`, `kill_wrapper`, …), and no root test file copies what two share. Open's start helpers belong there.

@history_item n=72 marker=amd-c7gfdhws
**amd-c7gfdhws** — (2026-11-04-linux-and-macos-parity) `Frozen` lives only in `tests/support/frozen.rs`, shared by the stale cases. Open's `stale-heartbeat` case reuses it, with no local copy.

@history_item n=73 marker=amd-vi7uvtsa
**amd-vi7uvtsa** — (2026-11-06-version-done-check) The CLI is tested as a subprocess with `std::process::Command` over `CARGO_BIN_EXE_viola`. Fixed strings are pinned by literal, with no trycmd or assert_cmd in any manifest. Open's CLI cases take this form.

@history_item n=74 marker=amd-aml2265l
**amd-aml2265l** — (2026-11-15-live-round-on-the-trimmed-product) `contract/event-log/start-wheel-record`: where folder-trust keys pass through the wrapper, a `wheel{human, human-input}` precedes session-start. A start's records are not a fixed count of lines. Open's live start through foot (R2) can show it.

@history_item n=75 marker=amd-uwsvrnci
**amd-uwsvrnci** — (live-round-on-the-trimmed-product) `decision/start-lines-read-from-start-offset`: a reused name's log keeps earlier starts' lines, so a reader takes the start's own offset. Open-then-reopen of one name, and its `window` line, must be read this way.

@history_item n=76 marker=amd-tknehnla
**amd-tknehnla** — (2026-11-14-status-line-interception-removed) `contract/viola-run-start/settings`: a start hands its child no `--settings` and writes no settings.json. Open's argv allowlist (only `claude` after `--`, no claude flag) keeps this intact.

@history_item n=77 marker=amd-a522hse3
**amd-a522hse3** — (2026-11-20-harness-prefix-and-identity-floor-ledger-rows) `decision/r8-claude-env-strip` names every identity-floor name with no count. The `run` that open starts through foot still strips CLAUDE* for its child. A test over the foot-spawned run reads the strip through the same literal-oracle rules.

@history_item n=78 marker=amd-gfgiaeyt
**amd-gfgiaeyt** — (harness-prefix-and-identity-floor-ledger-rows) `decision/strip-control-oracle-keeps-guarded-names`: a remove-the-guard control over the strip uses an oracle keeping the floor names. This applies if open's env hand-off gets a control.

@history_item n=79 marker=amd-3fwhzts6
**amd-3fwhzts6** — (home-and-code-bearing-file-integrity) Trap: a remove-the-guard witness whose neutralised guard lets a product process start needs a bounded witness spawned through `Running` under WITHIN. An unbounded one hung 27 minutes. Open's allowlist and no-shell controls let a process start.
