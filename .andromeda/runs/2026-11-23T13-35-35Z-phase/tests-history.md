@history_item n=1 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Test Scope Summary: 1 row(s)) — (amd-5mya4wvt) ~ decision/fixtures-recorded-by-live-verify: the recorded Hyprland replies are refreshed only by the test-only viola-hypr-tap. The `monitors` record and the R1 move-to-workspace fixture must come from that tap, never be hand-written.

@history_item n=2 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (7. Test Data & Fixtures: 4 row(s) (1 of 2)) — (amd-pfdqr7rr) ~ decision/fixture-hygiene-walk (each Hyprland reply held to its closed per-request string keep-list), ~ decision/fake-agent-on-test-path (the fake stands as hyprctl by its own file name, from files beside its exe), ~ contract/fake-agent-cli/argv-only-options (as hyprctl its mode is read from a file beside its exe), ~ decision/recorder-scrub (viola-hypr-tap applies the home-path and username scrub). This is the register the chunk extends: `monitors` needs its own keep-list entry, the walk must check it, and the fake's hyprctl replay (exact argv, 4 modes) must serve the new request and the dispatch.

@history_item n=3 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (7. Test Data & Fixtures: 1 row(s) (2 of 2)) — (amd-eefnob23) ~ decision/dirty-payload-refuses-recording: a Hyprland reply set that fails the unclean check refuses the whole recording, by file and closed code. The tap's `monitors` record and its R1 window record fall under this.

@history_item n=4 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (7. Test Data & Fixtures: 2 row(s)) — (amd-erzqi2ha) ~ decision/fixtures-walked-for-hygiene (Hyprland fixtures walked with their own planted cases) and ~ decision/fake-agent-argv-only except (as hyprctl the fake reads its mode and reply set from files beside its exe). New fixtures under `fixtures/hyprland/<version>/` join the walk, and each new class needs planted red cases.

@history_item n=5 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (5. Integration Test Strategy: 1 row(s)) — (amd-trilsjw7) ~ decision/fake-agent-as-claude: hyprctl is an external child process served in tests by viola-fake-agent in a test-scoped bin dir. Integration cases for `monitors` and the dispatch go through that replay.

@history_item n=6 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (8. Mocking & Stubbing Discipline: 1 row(s)) — (amd-itty4qan) ~ decision/fakes-for-external-services: the real hyprctl is replaced by the fake agent standing as hyprctl, and tests never reach the real service. This holds for the new query and the dispatch.

@history_item n=7 marker=2026-11-14-hyprland-exact-frame-bound-pinned
**2026-11-14-hyprland-exact-frame-bound-pinned** (8. Mocking & Stubbing Discipline: 1 row(s)) — (amd-jqhveuv3) except on decision/fakes-for-external-services: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests. Trap for this chunk: the new `Request`, its argv builder, its parse, the read deadline and the drain witness must be killed by `crates/viola-hyprland`'s own tests (decision/mutants-tested-by-owning-package), not by root tests.

@history_item n=8 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (2. Test Strategy: 1 row(s)) — (amd-rxbjdtib) ~ decision/ci-never-real-claude: CI never touches the real hyprctl, and no CI image carries it. The seam is held in CI only by the recorded set replayed through the fake, so R1's live move and R3's `monitors` record stay local and out of every suite.

@history_item n=9 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (4. Unit Test Strategy: 1 row(s)) — (amd-hs3diylx) ~ decision/bounded-readers-keep-the-cap: the hyprctl reader keeps at most the frame cap and reads a cut reply as oversize, held by its BOUNDARIES row. This is the row whose witness the chunk replaces with a drain-driving case.

@history_item n=10 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (4. Unit Test Strategy: 1 row(s)) — (amd-tjq3cxd7) ~ decision/bounded-readers-keep-the-cap, latest wording (the config.json default-pair reader added). Any edit to the row for the hyprctl drain witness starts from this text.

@history_item n=11 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader enumeration in the Contract suite) — (amd-quvutcb3) introduced `tests/contract_bounded_readers.rs`: literal `BOUNDARIES` / `EXCEPTIONS` tables (never the product's list), `boundaries_each_name_a_bounded_read_that_stands` (each row's witness must be a defined test), the census `product_readers_are_each_bounded_or_named`. The chunk re-points the `hyprctl reply` row's witness, so the new witness name must exist, and a changed read site must still pass the census. Masters state no row counts.

@history_item n=12 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (two owed test gaps, owned by «Sanitised error surfaces») — (amd-v6vvtmyi) a precedent for the same gap this chunk closes: rows whose witness reads the bounded read without driving past its cap, their caps held only by the needle and the census, were recorded as owed and carried as a PREREQ.

@history_item n=13 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the bounded readers' cap-driving witnesses) — (amd-ge6miket) discharged those gaps by re-pointing each BOUNDARIES row to a cap-driving case, among them `drain_keeps_cap_bytes_of_an_endless_pipe` (`src/run/version_gate.rs`) for a child pipe drain. This is the direct precedent for a witness that drives the hyprctl stdout drain past `MAX_FRAME`.

@history_item n=14 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (6. E2E Test Strategy: 1 row(s)) — (amd-srg4fj2s) ~ decision/property-tests-on-parsers: the hyprctl reply parsers joined the 512-case proptest surfaces with regressions committed. The new `monitors` parse is a new reply parser and falls under the same row.

@history_item n=15 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Test Scope Summary: 1 row(s)) — (amd-4gd3soxf) ~ decision/parser-property-fuzz-coverage: the hyprctl reply parsers are covered by property tests and by the fuzz replay as target `hyprctl_json`. A `monitors` parse should reach that target and its corpus (decision/fuzz-weekly-and-corpus-gate: each parser gets a target as it lands).

@history_item n=16 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (6. E2E Test Strategy: 1 row(s)) — (amd-sbmeablu) ~ decision/property-tests-with-proptest: proptest-regressions files are committed for the Hyprland reply suites. A new `monitors` property commits its seeds there.

@history_item n=17 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (11. Test Anti-Patterns (NEVER do these): 1 row(s)) — (amd-gxqopq3w) ~ decision/no-production-data-in-tests: fixtures come from `viola verify` probe text or a test-only recorder's capture, scrubbed. This is what admits the viola-hypr-tap `monitors` and R1 captures as fixtures.

@history_item n=18 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (7. Test Data & Fixtures: 1 row(s)) — (amd-zt4mirze) ~ contract/fake-agent-cli/argv-only-options: options come from argv and from a fake-args file beside the exe (ahead of argv, argv winning), never the environment; standing as hyprctl, the mode comes from a file beside the exe. The hyprctl replay of the new request takes no env seam.

@history_item n=19 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (7. Test Data & Fixtures: 1 row(s)) — (amd-ufypq5a6) ~ decision/fake-agent-argv-only except: the hyprctl files beside the exe plus the fake-args file. This is the current wording of the exception the hyprctl replay modes rest on.

@history_item n=20 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (test data: the fake agent's agents files, the test-scoped PATH, the Agents fixture class, the agents property and fuzz target) — the precedent for the hyprctl personality: the fake answers from files beside its exe in closed modes (`recorded` · `oversize` · `malformed`) with no env var (the founder's answer: no third env seam), a recorded fixture class with its own schema and hygiene cases, and a property plus fuzz target for its parser. The hyprctl replay's 4 modes follow this shape.

@history_item n=21 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the `Mcp.` exchange: its walk, its seed row and its replay) — the precedent for a test-only tap recorder (viola-mcp-tap): `record` lines read through `take(MAX_FRAME)` into a 0600 capture that is never committed, then a `fixture` step through the scrub and refusal (closed codes, file name only), a separate walk against its own schema with planted cases, and a replay test holding the served shape. viola-hypr-tap's `monitors` record and R1 record follow it.

@history_item n=22 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Test Harness Contract: 1 row(s)) — (amd-tveox2hq) ~ decision/root-wait-bound: root waits are bounded at 12 s, below the mutants profile's 15 s kill and cargo-mutants' 20 s floor. A test that times a hyprctl read against the 5 s `HYPRCTL_DEADLINE` must stay inside these bounds.

@history_item n=23 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Test Harness Contract: 2 row(s)) — (amd-kmpo2mdb) ~ decision/within-7s-wait-bound: WITHIN = 12 s bounds every test wait that has no per-test kill. A timed-read case that blocks for the 5 s deadline sits under this bound.

@history_item n=24 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Test Harness Contract: 1 row(s)) — (amd-gjhi22be) ~ decision/mutants-kill-below-cargo-mutants-timeout: the default mutants kill is 15 s, below the 20 s floor, and a slow case is fixed by removing slow work, never by raising the default (the one recorded departure). A test that waits out the full 5 s deadline, possibly several times, must stay below 15 s so a caught mutant is not graded Timeout.

@history_item n=25 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** (8. Mocking & Stubbing Discipline: 1 row(s)) — (amd-qoo7oxzm) ~ decision/time-faked-through-seams: time is faked only through injected seams (no mock_instant, no paused Tokio clock), and a production deadline is fired by a signal, never waited out. This is the closest precedent for the read deadline: a test that TIMES a real read against 5 s must be squared with this row and with decision/no-test-owned-real-time.

@history_item n=26 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — the origin of decision/bounded-remove-the-guard-witness: a control whose neutralised guard lets a product process start needs a witness bounded by `WITHIN`, because nextest's default profile has no kill line (an unbounded witness hung 27 minutes). Trap: a remove-the-deadline or remove-the-cap control over a hyprctl child that never ends must be bounded.

@history_item n=27 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (fake-agent copies planted one after another (ETXTBSY)) — a test that plants more than one fake-agent copy (here as hyprctl in a test bin dir) plants every copy one after another before any thread starts a child. On the tmpfs `target/e2e-home` the hard link fails and a forked child can inherit another thread's write handle (ETXTBSY).

@history_item n=28 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the pinned copy's no-clobber placement: its forced-window witness, test parameter and unit cases) — the origin of decision/placed-copy-execs-at-once (guard `place_private_new_placed_copy_execs_at_once`): a placed copy is persisted only once its write handle is closed, so it can be executed at once. This bears on a hyprctl stub or fake copy that is written and then executed.

@history_item n=29 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS scope: Linux only, one CI leg) — Linux only, CI on `ubuntu-latest`, and the window features on Hyprland. The seam's tests run on the one leg against the replay, with no other OS arm.

@history_item n=30 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS-gate census reads another OS as a fault) — `tests/contract_os_gated_tests.rs` (decision/linux-only's guard) reads a gate naming windows or macos as a fault even when a row names it, and every OS-gated test needs a `GATES` row. Any gated hyprland test the chunk adds must be named there.

@history_item n=31 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (6. E2E Test Strategy: 1 row(s)) — (amd-udylcpzx) + decision/live-failing-reading-not-verification: a live reading that fails a row is recorded as read, never as verification, and a confounded reading leaves the row unverified with its re-read owed. This governs how R1's one live move, and its fixture, are recorded if the reading fails or is confounded.

@history_item n=32 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (6. E2E Test Strategy: 2 row(s)) — decision/live-readings-not-a-test-layer was replaced by decision/standing-live-rig. Live readings now run on one committed test-only rig whose rehearsals run in CI. R1 is a tap-driven live reading outside that rig, so the plan must say where it stands against this row and against decision/real-claude-local-only.

@history_item n=33 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error-body enumeration in the contract suite) — `tests/contract_error_bodies.rs` carries `thiserror_messages_interpolate_nothing` (decision/thiserror-messages-interpolate-nothing) and the census of error writers. Any new `HyprlandError` word or message the move dispatch or `monitors` brings must interpolate no value.

@history_item n=34 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** (Windows host mutation scratch, scoped inner loop, run archive, keep-failed lifecycle guard, probe 5/5) — the release-check probe also fails an artifact built with a test-only feature (decision/release-build-viola-only; decision/no-test-binary-in-release). Extending the test-only viola-hypr-tap and the fake's hyprctl replay must leave the release build carrying viola alone.
