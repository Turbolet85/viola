@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — (interim supervisor, readiness, status and cleanup) made the supervisor an ordinary child with its detach flags removed, and added `processes_gone` to the cleanup report. This is where decision/supervisor-ordinary-child and contract/cleanup-document/processes-gone come from. A wrapper that outlives a killed test has to be reasoned about against this process tree: boot, then supervise, then the wrappers, then the agents.

@history_item n=2 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — (nextest mutants profile) set `[profile.mutants]` to fail-fast with `terminate = "immediate"` because a caught mutant had been graded Timeout. This is the first ruling on how nextest kills tests under mutation. The chunk exists because a mutation run killed tests at their timeout.

@history_item n=3 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — (mutation gate prebuilds the root bins) has `run --mutants` prebuild the root bins and pass `--copy-target=true`, so each cargo-mutants scratch tree carries a copy of `target/`. That copy includes `target/e2e-home` and any stamp marker in it. This matters both for where mutant-build wrappers run and for the stamp-marker-without-home trap.

@history_item n=4 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — made the root `booted_wrapper` readiness bounded and exit-aware: it fails at once when `child.try_wait()` reports an exit. It also set the mutants slow-timeout below cargo-mutants' 20 s floor. Why: two `wait_ready` consumers spun to the bound under that floor. This is the start of decision/root-wait-bound and decision/mutants-kill-below-cargo-mutants-timeout, and it is the kill-line arithmetic that any new end-of-test guard must stay inside.

@history_item n=5 marker=2026-09-24-epoch-1-cleanup
**2026-09-24-epoch-1-cleanup** — a leaked supervisor locked `viola-harness.exe`, and a mutation leg then read 135 unviable as ok. This is an earlier measured leak of a harness process outliving its run, and the reason an unviable swamp is now red. It is a precedent for this chunk's hygiene census.

@history_item n=6 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — `supervise` now owns one outer PTY per instance. Cleanup re-presses Ctrl-C every 500 ms. The root `booted_wrapper` runs over `OuterPty` and waits for the fake agent's `start` receipt. The `--exit-no-eof` holder gets its own process group. This is the origin of decision/cleanup-ctrl-c-repeated-until-exit and the booted-wrapper shape this chunk must end on every exit path.

@history_item n=7 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — `run --mutants` sets `AGENT_RUN_KEEP_HOMES=0` and `AGENT_RUN_KEEP_FAILED=0`. The fixture gained an owner record (pid and start time) and a gone-owner sweep. The sweep reclaims homes of killed tests but ends no process. This is the base of decision/test-home-owner-record-sweep, which a killed test's wrapper cleanup would extend.

@history_item n=8 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — the viola-e2e booted lifecycle tests got a `Booted` drop guard that keeps a failing test's home under `AGENT_RUN_KEEP_FAILED=1`. It is a Drop-based guard, and a SIGKILL never runs it. That is the gap this chunk closes.

@history_item n=9 marker=2026-09-27-browser-verdict-reachability
**2026-09-27-browser-verdict-reachability** — set the root `booted_wrapper` bound to 7 s (`tests/support/watch.rs` `WITHIN`) with a streamed `viola-root-watch` report. Why: a remove-the-guard pair, 7 s FAIL with the report against 12 s TIMEOUT with the output lost. Trap: a wait at or above the kill line loses its report.

@history_item n=10 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — (stamped root seam, stop waits for the endpoint) built the root `stamped_home` over `viola verify`; it is stamped only on exit 0 and the `stamped` last line. `Wrapper::stop` / `stop_keep` wait for the exit and then for the endpoint to be unconnectable, because an exit code is not the endpoint gone. This is the fixture whose absent-home failure (`stamps written by verify: NotFound`) is this chunk's first step.

@history_item n=11 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** — a root `TestHome` removes its home through `remove_owned`, which deletes `owner.json` last, and the sweep uses the same function. Ownerless remnants were recorded on runs failing at deadlines. This is decision/owner-record-removed-last: any new end-of-test cleanup must keep that removal order.

@history_item n=12 marker=2026-10-04-windows-boundary-mutation-workflow
**2026-10-04-windows-boundary-mutation-workflow** — (the mutants profile waits for running tests) moved `[profile.mutants]` to `terminate = "wait"` so that running tests finish and their temp dirs and session guards drop. The slow-timeout kill still bounds a hang. It recorded 17 kill-path leftovers by design. This is the direct precedent: a test killed at its slow-timeout drops no guard, so its booted wrapper and fake agent survive. That is the 5 h 50 min spin this chunk exists for.

@history_item n=13 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — traced the 21 half-removed fixture repos to a detached `git maintenance run --auto` spawned by each fixture commit, now run with `-c maintenance.auto=false`. This is a measured precedent of a test-spawned process outliving its test, closed by stopping the spawn rather than by cleanup.

@history_item n=14 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — (the fake agent quiesces its hooks before it exits) has the fake agent exit on `\x03` or at stdin EOF, waiting for a running hook first. The agent is its PTY's session leader. This bears on contract/fake-agent-exit/exit-triggers and on whether a fake agent ends once its PTY or wrapper is gone.

@history_item n=15 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — `target/e2e-home` may be an operator-set link to an owner-only tmpfs directory. A linked base needs an absolute target and is recreated 0700 when gone. Cleanup, a `TestHome` drop and the owner sweep delete through the link, a ratified boundary widening. After a reboot or `cargo clean` the target or link is gone. This is the mechanism behind the CI-red hypothesis: a restored `target/` carries a stamp whose home, through the link, is absent.

@history_item n=16 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — measured that an instrumented child killed during its exit-time profile write leaves a short profile that llvm-profdata refuses; the start test's child is now an uninstrumented `whoami`. This is decision/killed-test-child-uninstrumented. Trap for this chunk's SIGKILL proof: a killed instrumented process corrupts the coverage merge.

@history_item n=17 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — (the torn append as landed) simulates the torn append by stopping the wrapper and shortening its log rather than killing it mid-append, because a killed instrumented process can leave a short coverage profile. This is decision/torn-append-by-truncation, a precedent for how a kill proof avoids the profile trap.

@history_item n=18 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — (the kill path's endpoint wait) has `Wrapper::kill` wait for the wrapper's exit, then for its recorded child to be gone by pid and start time, then on `holder_gone`. The plan's `wait_endpoint_gone` after a kill was disproved: a killed wrapper leaves its socket file. These are the kill-path oracles that a killed-test census can reuse (decision/killed-wrapper-endpoint-wait).

@history_item n=19 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — (the state-file ban scoped) says the ban on a hand-written `ledger/stamps.json` or `snapshot.json` binds every booted-wrapper test, and only a reader's own unit cases may plant altered writer bytes. This is decision/state-files-never-hand-written. A test that plants a stamp marker with its home absent must stay inside it.

@history_item n=20 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** — named the shared root helpers (`tests/support/{home,outer_pty,fake,events,cli,ndjson}.rs`). `cli.rs` holds the one started-child guard and runner (`Running`). No root test file defines its own copy. This is decision/one-shared-test-helper: a new end-of-test guard must live in the shared support, not be copied.

@history_item n=21 marker=2026-10-13-epoch-4-survivors
**2026-10-13-epoch-4-survivors** — two cargo-mutants jobs under one `TMPDIR` shared a report file, and the report files now carry the test process's id. This is a precedent of cross-run process interference during a mutation run, the class this chunk measures.

@history_item n=22 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — `verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1` signals, by `libc::kill`, only the pid it started, and asserts every spawn pair closed. This is a precedent for this chunk's rule that no test sends a kill to a process it does not own.

@history_item n=23 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — (boot's two flags, harness_session) made `harness_session` a plain function and drop guard that keeps the spawned `supervise`, so that `cleanup`'s pid and start-time proof stays exact. `Drop` cleans up. This is a viola-e2e boot path whose wrappers a SIGKILLed test would orphan, and the `cleanup` proof (decision/supervisor-spawned-only-by-boot) a census can lean on.

@history_item n=24 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — (Path 1 as one flow; the E2E regex widened) put Path 1 on the root chain with no `viola_e2e::fixtures` copy, and set the E2E regex to `^(path[0-9]*|tui|mcp|cross|chaos|contract)_`. This holds decision/fixture-chain-exists-once and decision/e2e-layer-by-binary-prefix: a new kill-proof binary is placed in a layer by its name.

@history_item n=25 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (the MCP tests as landed) found that the rmcp child transport killed the server 3 s after a close and truncated a coverage profile. Now a test-owned `viola mcp` child has its stdin closed and its exit awaited. This is a precedent for ending a test's own child without truncating its profile.

@history_item n=26 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — (the mutants profile's override classes) moved each kill with a planted-hang pair, never by raising the default. Standing rule: a kill line moves only for a floor that is by design, with that control recorded both ways. This constrains any timeout this chunk adds.

@history_item n=27 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — (the headroom floor, the hook hangup named) refuses a linked base below the 9 GiB headroom floor. On Unix every command the fake agent spawns runs in its own process group, so a killed wrapper's PTY hangup no longer reaches a hook. This is decision/fake-agent-children-own-process-group and decision/e2e-home-headroom-floor. Process groups shape what a hangup reaches when a wrapper dies.

@history_item n=28 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the fake agent's exit ignores SIGHUP) has the fake agent, as its PTY's session leader, ignore SIGHUP from the first step of its exit. A hangup landing before the exit begins still ends it. This bears on why a fake agent under a killed wrapper may stay alive, which P3 must re-measure (decision/fake-agent-exit-ignores-sighup).

@history_item n=29 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the server-identity impostor; fake-agent copies planted one after another) has every held process killed and reaped by a `Drop` guard. On the tmpfs `target/e2e-home`, fake-agent copies are written rather than hard-linked and must be planted before any thread forks (ETXTBSY). Both are test-side process and file-handle hygiene precedents (decision/fake-agent-copies-planted-first).

@history_item n=30 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (remove-the-guard witnesses bounded by `WITHIN`) records that an unbounded remove-the-guard witness hung 27 minutes, because nextest's default profile has no kill line. A control whose neutralised guard lets a product process start needs a bounded witness. This is decision/bounded-remove-the-guard-witness, which this chunk's before/after controls must follow.

@history_item n=31 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (a home a test makes before viola) requires a home a test makes before viola to go through `create_private_dir`, since a `create_dir_all` home is refused by the read rule. This is decision/test-homes-made-private. A rebuilt or planted stamp home must be made this way.

@history_item n=32 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (a wrapper lost under an open call; the root shared start helpers) says a wrapper lost under an open call is a real kill through `support::start::kill_wrapper`, with no double. `tests/support/start.rs` shares `kill_wrapper` and the other start setups. This is decision/real-process-faults and the helper home for any kill this chunk's proof needs.

@history_item n=33 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (a stale holder reads `stale-heartbeat`) has `Frozen` stand for a stale holder as a real `SIGSTOP`ped wrapper, never a double. This is a process-fault precedent beside the kill and is named in decision/real-process-faults.

@history_item n=34 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`cleanup`'s `endpoint_gone` on Unix) says that after a force-killed wrapper the leftover socket file stays. `endpoint_gone` reads a connect refused or the path absent, and cleanup removes nothing of it. This is contract/harness-cleanup/endpoint-gone and contract/cleanup-document/endpoint-gone: a killed-test census must not count that file as a leak.

@history_item n=35 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (a test's endpoint directory is made private) requires a test that places an endpoint in its own directory to make that directory through `create_private_dir`; a bare `tempfile::tempdir()` reads 0755 and is refused. This is decision/private-dir-for-test-endpoints and decision/test-endpoint-dirs-made-private.

@history_item n=36 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (a terminated wrapper's child on every OS) added `run_child_ends_when_its_wrapper_is_terminated`: `kill -KILL` on the wrapper, then the child, by pid and start time from the snapshot, is gone within `WITHIN`. This is the witness of contract/viola-run/child-on-wrapper-kill, the product promise that a SIGKILLed test's wrapper chain rests on.

@history_item n=37 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (the Frozen guard's one home) moved `Frozen` to `tests/support/frozen.rs` as its sole home: `kill -STOP`, poll until `T` within `WITHIN`, `kill -CONT` on drop. Its no-copy rule names the freeze guard too. A SIGKILLed test leaves a frozen wrapper stopped, which is another orphan case for the census.

@history_item n=38 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (OS scope; one harness shim; bootstrap phases without the Windows arms) makes Linux the only OS with one CI leg, `ubuntu-latest`. `scripts/agent-run.sh` is the one shim, viola makes the boot home 0700, and `Frozen` is a `SIGSTOP` guard. `unconnectable` and `holder_gone` read a connect refused or the path absent. Root waits count 31 sites in 23 files. These are decision/linux-only and decision/single-os-ci-leg: the kill proof and the cache fix need only hold on Linux.

@history_item n=39 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — (root waits and the mutants kill) set `WITHIN` and the root wait bound to 12 s and the mutants default kill to 15 s, below cargo-mutants' 20 s floor. This is a stated departure, because the stamped-home class is now the default population. It is the baseline that "moved the test homes and the shared stamp" and that the CI-red hypothesis dates against. Any new end-of-test wait sits under these lines (decision/root-wait-bound, decision/within-7s-wait-bound, decision/mutants-kill-below-cargo-mutants-timeout).

@history_item n=40 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — (one verify stamp per build) has stamped homes copy one verify stamp per build, kept under `target/e2e-home`, keyed by the binaries' hashes and taken under an exclusive lock. The stamp dir carries no owner record. Another build's stamp is swept only when its lock is taken without waiting. A stamped home is a copy, never a link. This adds decision/one-verify-stamp-per-build and the exception to decision/test-home-owner-record-sweep, and is exactly the stamp marker this chunk's first step must revalidate or rebuild when its home is absent.

@history_item n=41 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — (verify-driven tests' 20 s kill) moved the rationale for decision/verify-driven-tests-20s-kill to "one verify stamp taken per build keeps each stamped-home test under it". A fix that rebuilds the stamp inside a test must still keep that test under the 20 s `ci` kill line (decision/ci-stamped-home-kill-line).

@history_item n=42 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (stamping verifies give the fake agent a transcript) has the harness boot and the stamped-home helpers pass `--transcript`, a convention every stamping helper must keep. This is decision/stamping-verify-passes-transcript: a rebuilt stamp must use the same verify argv, or the stamp loses a row.

@history_item n=43 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — (6. E2E Test Strategy: 2 rows) replaced the live-readings row with decision/standing-live-rig: one committed test-only rig in viola-e2e that owns its own isolation and process hygiene. The scope bounds this chunk off reworking it. Its census is the model for this chunk's killed-test census.

@history_item n=44 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — (7. Test Data & Fixtures: 4 rows) gave the fake agent a trust dialog and an exit 1 on the trust form's No, and held its SessionStart behind Enter under `--screens`. This moves contract/fake-agent-exit/exit-triggers and decision/wrapper-boots-pass-trusted-screens: a killed-test proof on the fake agent must boot past, or around, the trust form.

@history_item n=45 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — (2. Test Strategy: 1 row; 1. Test Scope Summary: 2 rows) adds the rig's screen and trust-grant exception to decision/discipline-at-every-layer. It re-points decision/real-claude-local-only to the rig's live starts. The exception is the rig's alone: this chunk's fixtures get no screen-reading or blanket-approval exception, and it runs no live start.
