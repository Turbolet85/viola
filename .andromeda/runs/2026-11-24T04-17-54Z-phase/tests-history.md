@history_item n=1 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (6. E2E Test Strategy) — `~ decision/strict-modes-before-any-log` added `viola focus` to the verbs that take the strict-modes check before opening any log, and `+ decision/window-raise-proven-live` set a real raise as proven only by the gated live reading R3, with a rehearse form that never counts as live (both PROVISIONAL, delegate). `viola up` is another window verb that reads the home: it extends the same rule, and its real re-open, workspace and placement can be proven only by an operator-granted live reading, never by the fakes.

@history_item n=2 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (5. Integration Test Strategy) — `~ contract/strict-modes-refusal/no-line` added `viola focus` to the surfaces whose refused check writes no line. `up` is a further surface. Its refused home or instance has to write no line, and the row's list grows again.

@history_item n=3 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (1. Test Scope Summary, list items) — `~ contract/viola-list-json/items` added `window? {address, workspace}` and `origin? cli|agent` to the board item (PROVISIONAL, delegate). `up` reads the board and the recorded window. This is the item shape its selection and its `--json` document rest on.

@history_item n=4 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (1. Test Scope Summary, start order) — `~ contract/run-start-events/order`: a start under `--origin` writes one `window` line (source wrapper) after `launch` and before session-start. Each start `up` makes through revive has to keep this order, and tests that read a re-opened instance's records read this sequence.

@history_item n=5 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (7. Test Data & Fixtures) — `~ decision/fake-agent-on-test-path`: the fake agent also stands as `foot`, the terminal stand-in, by its own file name, answering from files beside its exe with no env var. `up`'s window path is tested over this `foot` fake under a test-scoped PATH.

@history_item n=6 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (5. Integration Test Strategy) — `~ decision/fake-agent-as-claude`: the external child boundary is served by viola-fake-agent as claude, hyprctl and foot. `up` adds no new external binary. Its tests reuse these three stand-ins.

@history_item n=7 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (8. Mocking & Stubbing Discipline) — `~ decision/fakes-for-external-services`: the real foot terminal is replaced by the same binary as foot, and tests never reach the real services. This binds `up`'s tests: no real terminal, compositor or claude.

@history_item n=8 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (6. E2E Test Strategy, placement) — `+ decision/window-placement-proven-live`: real window placement on the parent's workspace is proven only by a live reading. The fakes prove the placement words and the window records only. `up`'s «in its recorded workspace and placement» clause falls under this tier split, so the plan gates it on the operator's live reading.

@history_item n=9 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (11. Test Anti-Patterns) — `+ decision/echo-oracle-skips-allowed-word`: an oracle asserting that a refusal echoes none of the given words skips the one allowed word that the fixed line itself names. This trap was met in open's refusal oracles, and `up`'s per-instance lines and refusals will meet the same sweep hazard.

@history_item n=10 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (1. Test Scope Summary, replay) — `~ decision/snapshot-replay-fields`: the replay recovers `window` too, with the last such line winning. `up` reading a recorded window after a reboot may read it through a replayed snapshot. This is the field set it can count on: no cwd, no endpoint.

@history_item n=11 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (6. E2E Test Strategy, live rig) — `+ decision/live-rig-clean-start` (PROVISIONAL): the live rig's stand-in takes the wrapper's arguments, the rig unsets every inherited `VIOLA_*` name, and every board read runs with no claude on PATH. This rule came from R2's red. Any live reading of `up` named at P4 reuses the rig under these three constraints.

@history_item n=12 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (6. E2E Test Strategy) — `~ decision/snapshot-replay-from-log`: the replay recovers the launch pair. The pair was proved through `list --json` after a killed wrapper's cut snapshot, because a revive over a replayed snapshot is refused `cwd-missing`. This trap applies directly to `up`: a revive-in-place over a replayed snapshot has no cwd, so `up` cannot re-open such an instance through revive.

@history_item n=13 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (1. Test Scope Summary) — `~ contract/run-start-events/order` and `~ decision/snapshot-replay-fields` added the `launch` line after lineage and the launch pair to the replayed set. `up` re-opens each instance with its recorded launch pair, and every revived start writes this record.

@history_item n=14 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (3. Test Harness Contract) — `~ contract/boot-readiness/events`: harness readiness reads four start lines (wheel, lineage, launch, session-start). `up` tests that boot through the harness wait on this probe, and a new start line would move it again.

@history_item n=15 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (1. Test Scope Summary, list items) — `~ contract/viola-list-json/items` added `model?` and `effort?` (PROVISIONAL, delegate). This is part of the board item `up` reads when it re-opens each instance with its pair.

@history_item n=16 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (4. Unit Test Strategy) — `~ decision/bounded-readers-keep-the-cap` added the `config.json` default-pair reader as a BOUNDARIES row. Any new external reader `up` adds, such as a board or record read, must be bounded or named in that enumeration.

@history_item n=17 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (7. Test Data & Fixtures) — `~ contract/fake-agent-receipt/start`: the start receipt carries the model and effort handed on argv. This is the witness for «the argv handed to each start» that `up`'s tests prove.

@history_item n=18 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (1. Test Scope Summary) — `~ contract/viola-list-json/items` (parent?, role?), `~ decision/snapshot-replay-fields` (lineage, last line wins, a later line with no parent clears it) and `~ contract/run-start-events/order` (lineage line, PROVISIONAL). `up` orders parents before children by this recorded lineage. A missing, cleared or cyclic parent is read through these rules.

@history_item n=19 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (3. Test Harness Contract) — `~ contract/boot-readiness/events` (the lineage line) and `+ decision/supervisor-spawn-strips-bridge-env`: the harness wrapper spawn drops the inherited `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`, so a session booted inside a wrapped session records no parent. Lineage-order tests for `up` depend on this.

@history_item n=20 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (1. Test Scope Summary) — `+ decision/run-start-verify-and-trust-after-bind`: a start runs its automatic verify after the bind and the trust record after it. `~ decision/human-never-blocked` names the start's verify window as an exception. Each revive `up` drives runs this start sequence, and on a fleet that cost multiplies (the scope's wall-time premise).

@history_item n=21 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (7. Test Data & Fixtures) — `~ contract/fake-agent-cli/argv-only-options` and `~ decision/fake-agent-argv-only`: the fake agent reads a fake-args file beside its exe ahead of argv, argv winning, never the environment. This is how `up` tests configure a stand-in they do not spawn themselves.

@history_item n=22 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (6. E2E Test Strategy) — `~ decision/single-writer-state-files`: only verify writes `stamps.json`, the verify a keyed start runs included, and every other entry point leaves `snapshot.json` and `stamps.json` byte-identical. `up` itself must write neither, and the single-writers witness gains `up` as a writer-capable entry point.

@history_item n=23 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (3. Test Harness Contract) — `+ decision/run-under-test-owned-user-home` and `+ decision/harness-wrappers-under-session-user-home`: every test-started run and revive runs under a test-owned user home, because the start's trust step writes the Claude config. The scope's «under a test-owned user home» rests on this rule, and every revive `up` drives in a test falls under it.

@history_item n=24 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (7. Test Data & Fixtures) — `+ decision/trust-start-folders-outside-repository`: trust-start test folders sit under `/tmp`, outside any repository. Recorded workspaces `up` resumes into in tests must follow this, or the trust key lands at the git root.

@history_item n=25 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (3. Test Harness Contract) — `+ decision/verify-under-test-owned-user-home`: a test-started verify points its trust write at a test-owned file. Each start `up` makes may run a verify, so its tests keep this home.

@history_item n=26 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (6. E2E Test Strategy) — `+ decision/live-failing-reading-not-verification`: a live reading that fails a row is recorded as read, never as verification, and a confounded one leaves the row unverified and its re-read owed. This governs how `up`'s gated live reading is recorded.

@history_item n=27 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (6. E2E Test Strategy) — `+ decision/standing-live-rig` replaced `decision/live-readings-not-a-test-layer`: live rounds reuse one committed rig whose rehearsals run in CI and whose live starts run by hand under the start cap. `up`'s live reading, if named, is an arm of this rig, not a launcher of its own.

@history_item n=28 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (1. Test Scope Summary) — `~ decision/real-claude-local-only`: the real claude is reached only locally through `viola verify` and the rig's live starts, never in CI. This holds `up` to the fake agent in CI and matches the scope's ban on any live start this phase.

@history_item n=29 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (6. E2E Test Strategy) — `~ decision/standing-live-rig` except: a rig arm whose contained rehearsal cannot run in CI is a local entry that skips by its printed sandbox probe. This precedent applies if `up`'s reboot reading needs a rehearsal CI cannot serve.

@history_item n=30 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** (5. Integration Test Strategy) — `~ decision/wrapper-drop-guard`, `+ decision/no-untied-viola-spawn` and `+ decision/tied-child-spawn-thread`: every long-lived viola child of a root test is tied to its spawning thread by a parent-death kill, and a census refuses untied spawns. The wrappers `up` re-opens in a test are long-lived children of `up`'s process. Tests must tie them, or the census fails.

@history_item n=31 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** (5. Integration Test Strategy) — `+ decision/no-untied-harness-boot`: no viola-e2e test boots a harness session untied, held by a census proven on a planted positive. This binds any harness-booted fleet that `up` tests stop and re-open.

@history_item n=32 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (3. Test Harness Contract, 5. Integration Test Strategy) — `+ decision/one-verify-stamp-per-build` and `~ decision/fresh-home-per-test`: a stamped home is a copy of one verify stamp per build, never a link, and nothing is shared between tests. `up`'s multi-instance homes start from this stamped copy.

@history_item n=33 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Test Harness Contract) — `~ decision/root-wait-bound`, `~ decision/within-7s-wait-bound` and `~ decision/mutants-kill-below-cargo-mutants-timeout`: `WITHIN` is 12 s, the mutants default kill 15 s, below cargo-mutants' 20 s floor. A fleet re-open waits on several starts in sequence, so `up`'s tests must keep each wait under these bounds or take a class kill of their own, moved only with a planted-hang control.

@history_item n=34 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (6. E2E Test Strategy) — `+ decision/start-lines-read-from-start-offset`: a reader of a start's lines reads from that start's own offset, because a reused name keeps earlier starts' lines at the head of the log. `up` revives reused names, so its per-instance assertions must read from the revived start's offset, never from line 1.

@history_item n=35 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (6. E2E Test Strategy, start record) — `~ contract/event-log/start-wheel-record`: every start, revived ones included, appends `wheel{driver, start}` first and session-start last. Folder-trust keys passing through can add a human-input wheel line. A revived start under `up` writes this record set.

@history_item n=36 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (5. Integration, 8. Mocking, 7. Test Data, 2. Test Strategy) — `~ decision/fake-agent-as-claude`, `~ decision/fakes-for-external-services`, `~ decision/fake-agent-on-test-path` and `~ decision/ci-never-real-claude` («or the real hyprctl»): the fake serves hyprctl from the recorded reply set, and CI never touches the real hyprctl. These are the Hyprland fakes the scope names for `up`'s tests.

@history_item n=37 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (6. E2E Test Strategy, 4. Unit Test Strategy) — `~ decision/property-tests-on-parsers` and `~ decision/bounded-readers-keep-the-cap` added the hyprctl reply parsers (512-case proptest) and the hyprctl reader to the frame cap. Any new Hyprland request or parser `up` adds joins these.

@history_item n=38 marker=2026-11-14-hyprland-exact-frame-bound-pinned
**2026-11-14-hyprland-exact-frame-bound-pinned** — `~ decision/fakes-for-external-services` except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests. This precedent applies if `up` adds code inside viola-hyprland.

@history_item n=39 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1. Test Scope Summary, 6. E2E Test Strategy) — `~ decision/snapshot-replay-fields`, `~ decision/snapshot-replay-from-log` and the start-order rows lost `budget_paused` and the budget-gate record. This explains why the replayed field set and the start records that `up`'s skip and re-open read hold no budget state.

@history_item n=40 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (exit-1 causes) — revive entered the test plan. Its exit-1 causes come in preflight order (strict-modes, already-live, no-session, cwd-missing). It is the first reader of read-or-replay, and a replayed snapshot carries no cwd. `up` resumes each instance «through the existing revive path», so these causes are the per-instance outcomes it must map.

@history_item n=41 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — under `--resume <id>` the fake agent sets `source` and `session_id` on the launch SessionStart. The kill path waits on `holder_gone`, because a killed wrapper leaves its socket file on Unix. `up`'s tests simulate the pre-reboot state by killed wrappers and assert resume through this payload.

@history_item n=42 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — the fake agent's `--resume` builds on `SessionStart.resume-1.json` where the set holds it, and Run E records the resume variant. This is the fixture basis for `up`'s «resumed in place» assertions over the fake agent.

@history_item n=43 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — revive's strict-modes refusal writes no log line on the start arm or `--list`, so the cases assert absence. The check comes first, ahead of the wrapper log. This precedent applies to `up`'s per-instance refusals: assert no line, not a line.

@history_item n=44 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (tests) — revive's preflight gained the `session-live` reading through one `claude agents --json` read. `--fork` skips it, and a failed agents read passes it. `up` reviving a session still open elsewhere meets `session-live`, and the fail-open rule decides its skip.

@history_item n=45 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (test data) — the `claude agents --json` stub is the fake agent placed as claude in a test-scoped bin dir, answering from files beside its exe. Every test-started list or revive runs under a test-scoped PATH. `up` reads the board and revives, so it is bound to this (`decision/tests-never-read-host-sessions`, `decision/agents-stub-from-files-no-env`).

@history_item n=46 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — revive takes `--json` with one document per code. The completeness case `every_flag_variable_and_config_key_is_a_row_of_the_table` reads `viola --help` Options, `VIOLA_*` and `config.json` keys. A test-started viola drops `VIOLA_*`. Any flag `up` adds becomes a controls-table row, and its `--json` document joins this contract.

@history_item n=47 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`revive --json`) — revive's codes became ten, a post-preflight start refusal prints one exit-1 document, and a `pump_child` error prints no document once the child holds the terminal. `up`'s per-instance outcome must handle a revive that hands the terminal to its child (it opens a window instead), and it maps these closed codes.

@history_item n=48 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (stale holder) — a stale holder refuses the start with exit 1 `stale-heartbeat`, held by `Frozen`. A live holder or a fresh replayed beat stays `already-live`. `up`'s «pid-and-start-verified live ones skipped» must tell live from stale by these rules.

@history_item n=49 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit-cause table) — `tests/cross_exit_causes.rs` is built. Every case asserts the exit, the `--json` document and a last hint naming its cause, with self-checks `no_two_codes_share_a_hint`, `no_hint_names_release` and `every_code_is_a_code_word`. Any new cause or hint `up` introduces joins this table.

@history_item n=50 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (root `cross_` binary) — `tests/support/start.rs` shares `first_life`, `stopped_life`, `revive`, `kill_wrapper` and the other start helpers, and no root test file keeps its own copy. `up`'s tests build their pre-reboot fleet from these helpers.

@history_item n=51 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (stopped instance) — a name whose endpoint is gone with its directory standing is `instance-dead`. A name with no directory is `unwrapped-name`, read by `without_endpoint`. This is the board's after-reboot reading `up` selects from.

@history_item n=52 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`cli` catch site) — under `--json` the catch site prints `{"v":1,"error":"internal-error","detail":null}`, and `json_asked` reads `--json` only before `--`. `up`'s `--json` internal-error path follows this.

@history_item n=53 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (wrapper lost) — a wrapper lost under an open call is a real kill (`kill_wrapper`), never a double. Simulating the host going down in `up`'s tests takes the same real process faults.

@history_item n=54 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (bare temp-dir home) — a bare temp dir reads 0755 and the read rule refuses it. This trap hides a test that never reached its subject, and `up`'s home must be viola-made.

@history_item n=55 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (enumeration) — `tests/contract_error_bodies.rs` holds `CLI_DOCUMENTS` and its census `product_error_writers_are_each_named`. Every product error writer needs a named row and a witness. `up`'s new `--json` failure documents must be rows.

@history_item n=56 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (surface witnesses) — `cli_error_documents_hold_no_path_or_upstream_text` holds each CLI `--json` failure to one document, a literal exit, empty stderr, no ESC and a clean body. `up`'s documents of codes and closed words join this witness.

@history_item n=57 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — `tests/contract_bounded_readers.rs` holds `BOUNDARIES` and the census `product_readers_are_each_bounded_or_named`. Any new reader `up` adds (records, board, hyprctl replies) is bounded in-statement or named with a witness.

@history_item n=58 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (no-clobber placement) — the pinned copy's absent-copy write is a no-clobber placement, so two first starts on one home both land. `up` re-opening several instances on one home rests on this race fix, witnessed by `place_private_new_placed_copy_execs_at_once`.

@history_item n=59 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (Path 7 MCP leg) — booting `builder` alone was a test-side workaround for the first-start race. The default pair now boots unstamped and witnesses two first starts landing. This precedent applies to `up`'s parallel or sequential multi-start tests.

@history_item n=60 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — a test that plants more than one fake-agent copy plants every copy one after another before any thread starts a child (ETXTBSY on tmpfs). `up`'s tests plant claude, hyprctl and foot copies, and must follow this.

@history_item n=61 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (start checks) — run and revive's start run `check_home_tree` before the wrapper log. A refused tree exits 1 with no log, no line and no child. Each start `up` drives inherits this, so a refused instance is a per-instance outcome with no line.

@history_item n=62 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (replay readers) — the collision check, hook connect, `live_endpoint` and `board::read` read through `read_snapshot_or_replay`. Over a replayed snapshot the collision check reads liveness from the beat alone. `up`'s «pid-and-start-verified live» skip must go through read-or-replay and this rule (`decision/snapshot-read-through-replay`).

@history_item n=63 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (E5) — `tests/chaos_snapshot_replay.rs`: a start over an unreadable snapshot replays the log, and a fresh beat over a replayed snapshot refuses `already-live`. `up` meets corrupt snapshots after a host crash, and these are its chaos precedents.

@history_item n=64 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (strict-modes exits) — a refused check at run, revive, verify, the frame verbs, the hook and MCP writes no line. Start refusals are exit 1, and CLI verbs exit 21. `up` needs its own exit for a refused home, placed by this rule.

@history_item n=65 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (security negatives) — a home another user may read refuses run (exit 1), and single writers leave `snapshot.json` and `stamps.json` byte-identical. `up` is another entry point for the read-granted-home negative and the single-writer witness.

@history_item n=66 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard) — a control whose neutralised guard lets a product process start needs a bounded witness (`Running` under `WITHIN`). An unbounded one hung 27 minutes. `up`'s remove-the-guard controls start processes, so their witnesses must be bounded.

@history_item n=67 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (instance check) — the instance check runs before any role file or log opens, because opening a role file narrows a 0770 home to 0700 and a later check passes it. This order trap applies to `up` checking each instance before it writes a per-instance line.

@history_item n=68 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (help) — `help_lists_only_served_verbs` and the result-line literals: a new verb joins the `HELP` literal and the served-verb table, and its human lines are pinned by literal. `up` is a new verb row.

@history_item n=69 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `viola --help` is pinned by the `HELP` literal in `tests/cli_output_plain.rs`, and the human layouts are pinned by literal, not trycmd. `up`'s per-instance lines and its help row are pinned the same way.

@history_item n=70 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `list` rows escape every control character, the board is pinned by `CAPTION` and the `board_row_*` units, and `board::read` is a read-or-replay caller. `up` naming instances in human lines must escape names (`decision/human-output-escapes-control`) and reads the same board.

@history_item n=71 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (start socket dir) — a start, revive's included, checks the socket directory before any bind, and a refused one exits 1 `socket-dir-refused`. After a reboot the runtime dir is fresh tmpfs, so `up`'s revives create the directory, and this refusal is one of its per-instance outcomes.

@history_item n=72 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (revive code) — `revive --json` gained `socket-dir-refused`, and the six exit-21 causes are closed. This is part of the closed code set `up` maps per instance.

@history_item n=73 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (cleanup) — after a force-killed wrapper on Unix the leftover socket file stays until a next bind takes it over, and `endpoint_gone` reads a refused connect. This is the after-crash state `up` re-opens over: a stale socket file is not liveness.

@history_item n=74 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`viola list`) — a home another user may read refuses `viola list` with exit 21, the whole board's refusal opening nothing. `up` reads the board, so it inherits this refusal.

@history_item n=75 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (endpoint dir) — a test placing an endpoint in its own directory makes it with `create_private_dir`, never a bare `tempdir`. This binds `up` tests that bind any endpoint.

@history_item n=76 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (Frozen) — `Frozen` lives only in `tests/support/frozen.rs` and is shared, and stale cases stand on it. `up`'s «stale versus live» skip cases freeze a wrapper through this guard.

@history_item n=77 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (terminated wrapper) — `run_child_ends_when_its_wrapper_is_terminated` holds that a killed wrapper's child is gone within `WITHIN`. This is the pre-reboot simulation `up`'s tests can build on: killed wrappers, gone children.

@history_item n=78 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — liveness is classified by a pure `classify(beat_age, same_process)`: a pid or start-time mismatch is gone, the same process at 5.0 s is live and at 5.1 s stale. `up`'s skip decision is tested at the unit tier through this pure function, never a clock.

@history_item n=79 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (OS scope) — the product is Linux only, with CI on `ubuntu-latest` and the window features on Hyprland. `up`'s tests have no other-OS arm.

@history_item n=80 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (census) — the OS-gate census reads a gate naming `windows` or `macos` as a fault even when named. Any OS-gated `up` test is a `GATES` row.

@history_item n=81 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (bootstrap) — the `ci` profile's 20 s kill line lists twenty-six verify-driven binaries by the `binary(…)` rule. A new stamped-home `up` test binary joins that line and moves the count.

@history_item n=82 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (bans) — CLI tests run over `std::process::Command`, and the ban on `.output()` / `.status()` on the long-lived wrapper requires `.spawn()` plus a `Drop` guard (`Running`). `up` leaves long-lived wrappers behind it, so a test must not block on it if it holds children.

@history_item n=83 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — the E2E regex is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_`, and the integration layer is its complement. `up`'s new test binary names decide its layer.
