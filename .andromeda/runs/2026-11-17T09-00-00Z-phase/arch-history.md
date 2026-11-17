@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (test-only crate, bins, env vars, paths) — registered `viola-e2e` (harness + `viola-harness`, no-op `fake-agent` feature), the fake agent bin, `AGENT_RUN_KEEP_HOMES` and the `AGENT_RUN_` prefix, and the repo paths `target/e2e-home/`, `target/harness/`; the registry tracks only variables the product reads or sets. Any new test-side marker or path this chunk adds (a kill harness, a census marker) follows this naming and registration precedent.

@history_item n=2 marker=2026-09-24-fake-agent-and-test-data-fixtures
**2026-09-24-fake-agent-and-test-data-fixtures** — named the two home classes (`viola-session-*/home` harness, `viola-test-*/home` root rstest), registered `AGENT_RUN_KEEP_FAILED` (root test chain only) and the `Wrapper::boot` paths and `tests/support/`. These are exactly the fixtures that boot the wrappers this chunk must end on every exit path.

@history_item n=3 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — liveness by pid + start time (a `gone` name is taken over at once), the start order, the pinned copy re-hashed before reuse with mismatch refusing `pinned-hash-mismatch`, and `target/e2e-home` gaining its owner record and gone-owner sweep. The sweep's owner rule is the existing reclaim path for homes of killed tests; a leftover wrapper keeps its owner "alive" only if the owner record names it, so P3 must read whose pid the record holds.

@history_item n=4 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — the per-user 0700 socket dir, chmod 0600 after bind, the `<socket dir>/viola-<h12>.lock` start arbiter, and the test-only `viola-test-chan-<pid>-<label>` endpoints outside the product namespace (rows decision/test-endpoints-outside-namespace, decision/wrapper-ipc-local-socket). A killed test's surviving wrapper still holds its lock and endpoint.

@history_item n=5 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — `AGENT_RUN_KEEP_FAILED` also read by viola-e2e's `harness_lifecycle` `Booted` guard; the test-only `<temp dir>/viola-pty-watch/<test name>.report`. The `Booted` guard is a Drop-based ender of a booted wrapper — the very mechanism SIGKILL skips, so this chunk needs a non-destructor path beside it.

@history_item n=6 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — the first ratified test seam `FAKE_AGENT_PUMP_DELAY_MS` (fake-agent builds only, capped 5 s, configures nothing) and the local pre-push gate preceding every operator push. Precedent and constraint: any new fake-agent or test seam is a boundary widening needing ratification and must stay out of release builds (rows decision/fake-agent-pump-delay-seam, decision/test-seams-not-configuration, decision/local-pre-push-gate).

@history_item n=7 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — the second seam `FAKE_AGENT_HOOK_PANIC` (fires only on exactly `1`, ratified by the founder live) and the G2 zero-panics script `scripts/g2-zero-panics.sh` exempting only the hook seam module (row decision/zero-panics-gate). A kill-proof test must not leave panic lines under the e2e home, and a third seam would need the same founder ratification.

@history_item n=8 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the six landed ledger rows, `viola verify`'s probe, the stamps envelope) — `ledger/stamps.json` has one writer (`update_stamps`, called by verify only), its envelope and merge rules, and `verify` runs in CI against the fake agent. The shared stamped home is valid only through these stamps; a rebuild must re-stamp through verify, never by writing stamps directly (rows decision/stamps-single-writer, contract/ledger-stamps/writer).

@history_item n=9 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — CI and the pre-push run no mutation job; mutation runs only through `agent-run run --mutants`, kept for the epoch-boundary audit (row decision/mutation-at-epoch-boundary). The killed-at-timeout wrappers this chunk exists for came from such a run; the scope forbids a mutation run here.

@history_item n=10 marker=2026-09-29-h2-conpty-resize-probe
**2026-09-29-h2-conpty-resize-probe** (the viola-pty-watch dir holds the test's own report beside the child's) — real-PTY test reports, codes and counts only, kept on a panic or kill and removed on a pass, never read by viola. The precedent behind decision/test-evidence-survives-kill that a kill-proof census report should follow.

@history_item n=11 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (verify's spawn pairs, the harness CI read, verify-stamped homes and the ninth root wait) — harness session homes stamped at `boot` step 4 by `viola verify` against the fake agent, root rstest homes by `stamped_home`; `CI` refuses `run --local-live` (`live-in-ci`); `wait_endpoint_gone` added in `Wrapper::stop` / `stop_keep`. This is the origin of the shared stamped home whose marker-without-home this chunk's first step fixes, and of the stop path's endpoint wait.

@history_item n=12 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** — both removals of a test home (its drop and the gone-owner sweep) go through `remove_owned`, deleting `owner.json` last; ownerless remnants left by deadline-failing runs are not closed and their mechanism not established. Directly bears on killed tests' leftovers and on the rule that a sweep never removes by age or name (row decision/test-home-owner-sweep).

@history_item n=13 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (pre-push native on the Linux host; the mutation scratch dir on the Linux host) — pre-push runs natively under `env -i HOME PATH`, Linux only; mutation runs on the dev host take a NOCOW `TMPDIR` because cargo-mutants copies the tree and `/tmp` is a quota'd tmpfs. Context for the mutation run whose killed tests spawned the spinning wrappers.

@history_item n=14 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (fuzz targets, the chaos test home, the send layout) — the `<temp dir>/viola-chaos-*/home` (`TestHome::outside_scan()`) outside `target/e2e-home` and so outside G2, G4 and the secret scan, the founder's named carve-out (row decision/chaos-home-outside-gates). A kill-proof home must either live under the scanned tree or be ratified as its own carve-out.

@history_item n=15 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — `target/e2e-home` prepared by two keepers (`prepare_home_base` in `tests/support/home.rs`, `Workspace::ensure_e2e_home`); on the dev host it links to `/tmp/viola-e2e-home-<uid>` (an environment fact, no variable); a linked base needs an absolute 0700 real directory; `cargo clean` removes the link; removals outside the working directory ratified as a widening; standing rule: a stalled-start red is a backing finding, never re-run for green. The CI-red hypothesis (a restored `target/` whose stamp marker's home under `/tmp` is gone) rests on this link mechanism (rows decision/dev-host-e2e-home-on-tmpfs, decision/linked-e2e-home-base-strict, decision/test-side-removal-outside-workdir).

@history_item n=16 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** — the root `WITHIN` waits counted by the pattern `Instant::now() + WITHIN`, not a named list; trap: a wait written through `EXIT_WITHIN` or a qualified path is outside the pattern, so the count is true only with its rule beside it. Any new wait this chunk adds moves that count.

@history_item n=17 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (root wait count, the kill path's endpoint wait, ten fake-agent options) — `Wrapper::kill` waits on `holder_gone` because a killed wrapper leaves its socket file on Unix; the plan's endpoint wait for a killed wrapper was disproved (it could never end). Trap for a kill-path wait: wait on connect-refused/absent, never on the file vanishing.

@history_item n=18 marker=2026-10-13-epoch-4-survivors
**2026-10-13-epoch-4-survivors** — real-PTY report files named per test process (`<test name>.<pid>.report`) so two processes running one test never share one. Precedent for any per-test evidence or census file this chunk writes under a parallel or repeated runner.

@history_item n=19 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's stop on SIGINT/SIGTERM kills and reaps its child with no key, its drop guards remove only what that run created, never by a pattern or a listing; SIGKILL and a crash keep the OS default and leave the dirs. The closest precedent for process hygiene under a kill, and the known limit (SIGKILL runs no destructor) this chunk must get past.

@history_item n=20 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (Run E's resources and a killed PTY child's exit status) — a child of an interactive PTY run killed at a stop closes its spawn pair with a `child_exit_status`, the PTY pump's exit code, as measured; a print-mode child killed so is unmeasured. Bears on how a killed fake agent's exit is recorded (rows decision/child-exit-on-process-handle, contract/child-exit-record/exit-source).

@history_item n=21 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the fake agent's agents mode) — the fake agent writes a live `sessions/<pid>.json` registry beside its executable, a dead or reused pid left out through one `liveness::start_times()` read; files beside the exe chosen by the founder over a third env seam. A precedent for the fake agent recording its own processes and for avoiding a new env seam.

@history_item n=22 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** (the CLI verbs take the VIOLA_DIR step and canonicalise the home) — `resolve_home` takes `--home`, else the `VIOLA_DIR` instance home, else the default, and `run`'s start canonicalises after its log creates the home (ratified at 2026-10-18-plugin-precedence-live-readings). Per-test home isolation (row decision/viola-home-resolution) is how a booted wrapper's home is scoped to its test.

@history_item n=23 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tools' contract as landed) — `viola-e2e` takes rmcp `client` alone, never `transport-child-process` (row decision/e2e-no-rmcp-child-transport: the E2E crate owns the `viola mcp` child and awaits its exit). A standing precedent that test code owns and reaps the children it spawns.

@history_item n=24 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — viola-e2e's `harness::session` guard reads `AGENT_RUN_KEEP_HOMES` / `AGENT_RUN_KEEP_FAILED` in its `Drop` after `cleanup`. Another Drop-only ender of a booted session, absent under SIGKILL.

@history_item n=25 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — the keepers refuse a linked base below the 9 GiB `HEADROOM_FLOOR_MIB` with a fixed message (rows decision/e2e-home-headroom-floor, contract/e2e-home-refusal/headroom-message); the profraw census's second form found a hangup reaching the fake agent's hooks truncating profiles. Constraint on any keeper change in the stamped-home fix, and a prior measured effect of a booted wrapper's end on the coverage run.

@history_item n=26 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the test-only child marker and the orphans probe's two cases; the recorded live MCP exchange) — `MCP_SERVE_TEST_CHILD`, a test-only marker outside `VIOLA_*` that lets `viola-mcp`'s tests kill `serve`'s survivor; `viola-mcp-tap` spawns directly and exits with its child. Precedent for a test-only marker on a spawned child (row decision/viola-env-namespace-product-only).

@history_item n=27 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — the pinned copy placed once, no clobber, with the write handle closed (`place_private_new`), a lost placement re-hashing the copy that stands; an open-handle rename failed concurrent first starts with `ETXTBSY`. The stamped home holds a pinned copy (`pinned: NotFound` is one of the CI-red messages), and a rebuilt stamped home must keep this placement (rows decision/pinned-copy-*, decision/pinned-binary-copy).

@history_item n=28 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree check before the start's and verify's log; the stamps writer's strict-modes check) — `check_home_tree` before any log, `verify` refusing an untrusted home or stamps, `update_stamps` checking strict modes before and under its lock, and a path under a regular file refused. A rebuilt or relinked stamped home must pass these checks (rows decision/verify-refuses-untrusted-home-or-stamps, decision/stamps-writer-checks-before-and-under-lock, decision/path-under-a-file-is-a-strict-modes-refusal).

@history_item n=29 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`v1-47`'s reading over the homes CI keeps) — each CI job keeps its homes after its end-to-end run (`AGENT_RUN_KEEP_HOMES` `"1"`, `if: always()`), the `viola-chaos-*` home out; mark cleared at 2026-11-01-t02-46-44-wrap. Kept homes must not keep live wrappers, and a home-rebuild must not delete a kept home (row decision/end-to-end-homes-kept-after-each-job).

@history_item n=30 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a force-killed wrapper's socket file; the test-owned socket roots) — a force-killed wrapper's socket file stays until the next bind of that name, nothing removes it; harness `cleanup`'s `endpoint_gone` reads connect-refused or absent as gone and removes no leftover; test socket roots under `/tmp` via `SocketRoot` (rows decision/stale-socket-file-left, decision/cleanup-reads-refused-or-missing-socket-gone). A kill census must judge a wrapper gone by its process, not by its socket file.

@history_item n=31 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the OS-gated tests census; the shared Frozen guard; the root waits) — every OS-gated test named with its reason in `tests/contract_os_gated_tests.rs`; `tests/support/frozen.rs` `Frozen` freezes a wrapper by SIGSTOP and SIGCONTs on drop (row decision/root-tests-freeze-wrappers-with-one-guard). A test that SIGKILLs a test process must leave no wrapper stopped, and any new OS gate needs its census row.

@history_item n=32 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (CI on the one `ubuntu-latest` leg; [PTY]: openpty alone; test support and harness; mutation runs; the root `WITHIN` waits; the OS-gate census) — CI is one `ubuntu-latest` leg with `run --coverage` through `scripts/agent-run.sh`; the pump keeps the input writer until the child exits, child exit still on the handle wait; `Frozen` ungated; `holder_gone` a connect refused or path absent; mutation runs have no scratch arm; a gate naming another OS reads as a fault. The run loop's dependence on the child's exit is where a wrapper whose test is gone may spin.

@history_item n=33 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (portable-pty pinned at =0.9.0) — the pty seam moved to portable-pty `=0.9.0`, handle-based exit detection and `pty_backend()` `openpty` unchanged (rows decision/portable-pty-seam, decision/portable-pty-behind-pty-seam). Any wrapper-side fix for its loop once the PTY or parent is gone stays behind this seam.

@history_item n=34 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Cross-cutting Patterns: 1 row) — `decision/mixed-version-tolerance`: a stamp holding a retired ledger row verifies on the relied rows alone. The stamped fixture home's validity is read through this rule when the relied set changes.

@history_item n=35 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — the shared root wait bound `WITHIN` moved from 7 s to 12 s, and test channel sockets bind under a short test-owned `/tmp` root, never under `TMPDIR` (rows decision/root-waits-share-one-bound, decision/test-sockets-under-short-tmp-root). This is the chunk the scope names as moving the test homes and the shared stamp before the CI cache key changed — the hypothesised root of the absent stamped home.

@history_item n=36 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Established Decisions: 2 row(s) (2 of 2)) — added the relied `local-command-echo` ledger row with its verify probe (row decision/local-command-echo-probe), so every home stamped before reads unverified until re-stamped (rows decision/version-verified-all-relied-rows). A stamp marker that outlives a relied-row change is a second way the shared stamped home can be stale.

@history_item n=37 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — registered the test-only `viola-live-rig` and the census marker `AGENT_RUN_RIG` set on the rig's child and never read by viola, and vt100 as the rig's screen model (rows name/binary/viola-live-rig, name/other/agent-run-rig, decision/harness-env-unread-by-product, decision/test-binaries-never-released). The rig's own-process census is the precedent this chunk's kill proof follows; the rig itself is out of scope to rework.
