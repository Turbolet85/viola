@history_item n=1 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** (test-side removals reach the dev host's tmpfs backing through the home link) — added the ban that a test-side removal under `target/e2e-home` takes only a dir the test side itself created (owner sweep by `owner.json`, its own `TempDir`, harness `cleanup` by the `viola-session-` name), recorded that on the dev host `target/e2e-home` links to `/tmp/viola-e2e-home-<uid>`, and bound the two keepers (`prepare_home_base`, `Workspace::ensure_e2e_home`) to follow only an absolute, owner-only real-directory target, creating it 0700 and removing nothing; founder-ratified widening. Matters here: the stamped-home fix rebuilds a home through exactly that link (the CI hypothesis is a restored `target/` stamp whose `/tmp` home is gone), so any rebuild or cleanup must stay inside these removal and keeper rules (rows decision/e2e-home-removal-own-dirs-only, decision/e2e-home-keepers-linked-base, decision/e2e-home-link-guard, decision/viola-ignores-e2e-home-backing). Its trap: with the relative-target check neutralised a keeper makes a directory in the test's working directory.

@history_item n=2 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the e2e-home keepers refuse a backing below the headroom floor) — the keepers also refuse a linked base whose filesystem has under 9 GiB free (`statvfs`), with a fixed no-path message; a compiled constant, no env or flag. Matters here: a rebuilt stamped home on a CI runner or dev host goes through the same keepers, and this refusal (row decision/e2e-home-link-guard) must not be bypassed or loosened by the fix.

@history_item n=3 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (Security Anti-Patterns: the stamp-dir sweep) — widened decision/test-removal-own-dirs-only with a fourth way a test-side removal knows its dir: another build's stamp dir whose lock it takes with `try_lock`; landed provisional. Matters here: this is the shared stamp (`stamp_dir` / `stamped_home`) the chunk's first step must make valid or rebuild; any new removal of a stale stamp marker or its absent home must fit this list or need its own ruling.

@history_item n=4 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (decision/test-removal-own-dirs-only state provisional → standing) — the founder ratified the stamp-dir `try_lock` removal. Matters here: the rule the stamped-home rebuild rests on is now standing; a fifth removal way (e.g. dropping a marker whose home is absent, or killing/removing another test's leftovers) is a new widening, not covered by this ratification.

@history_item n=5 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** (`FAKE_AGENT_PUMP_DELAY_MS` carve-out) — admitted the first fake-agent-only test seam (u64 ms capped at 5 000, compiled out of release, not `VIOLA_*`, configures nothing, disables no control) and added the NEVER for another seam without its own Decisions Log entry; a boundary widening. Matters here: if ending the booted wrapper or fake agent on a killed test needs a new env read or seam in the fake agent or `viola run`, it falls under rows decision/new-env-read-needs-log-entry and decision/test-seams-fake-agent-only and must be escalated, not added as routine.

@history_item n=6 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (test seam `FAKE_AGENT_HOOK_PANIC`) — the second fake-agent seam, closed value, fired after obs init so the hook still fails open, compiled out of release; "another seam needs its own entry" restated. Matters here: the precedent shape (and the founder-live ratification it needed) for any fake-agent change this chunk makes (row decision/fake-agent-hook-panic-seam).

@history_item n=7 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** (founder ratification of three standing boundary widenings) — the founder re-ratified the pump-delay seam live and set the standing rule that a boundary widening halts the wrap for a live answer; an earlier direction never ratifies one. Matters here: a new seam, env read or kill path the chunk adds must be shown to the founder live, not carried on the overseer's word.

@history_item n=8 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** (test-only feature proven absent) — recorded `release-check` refusing `test-support` / `fake-agent` artifacts (probe 5/5). Matters here: any fake-agent or test-support code added to end booted processes must stay behind those features (rows decision/release-build-no-test-binaries, decision/release-build-without-test-features, decision/test-seams-compiled-out).

@history_item n=9 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** (verify's dirs removed on a Ctrl-C / TERM stop) — verify's handler only sets a flag, a stopped run kills and reaps its child with no key, its guards remove only what that run created (never by pattern or listing), no remover of an earlier run's or another process's dirs; SIGKILL and crashes leave the dirs by hand. Matters here: the nearest precedent for ending a child on its owner's stop, and the same limit the chunk faces (SIGKILL runs no destructor); it also sets the own-only rule the census and kill harness echo ("no test sends a kill to a process it does not own").

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's stop and the colour decision lose their Windows arms) — verify's stop is `SIGINT` / `SIGTERM` alone and its dirs are left by hand after SIGKILL, a crash or a failed handler install. Matters here: the Linux-only signal set the chunk's kill and end-of-test handling sits beside.

@history_item n=11 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a killed wrapper's leftover socket file) — recorded by choice that a killed wrapper's socket file stays in the per-user 0700 directory until a next bind takes it over, nothing removing it; stack reference `crates/viola-e2e/src/harness/cleanup.rs` (`unconnectable`, which removes no leftover file). Matters here: killing booted wrappers in tests leaves these files; the harness must not start removing them without a new decision (row decision/leftover-socket-file-kept).

@history_item n=12 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at a hook and at a start) — `socket_dir_ready` runs `check_endpoint_dir` before any snapshot, child or `Server::bind`, creates the parent only when absent and checks again; a refusal is exit 1 `socket-dir-refused`. Also (the socket-directory check entry) `check_socket_dir` never chmods. Matters here: a booted wrapper in a rebuilt or fresh test home goes through this start order (rows decision/start-checks-socket-dir-before-bind, decision/run-refuses-socket-dir-before-start, contract/socket-dir-refusal/run-and-revive-start); test sockets under the short `/tmp` root must sit in an owner-only dir.

@history_item n=13 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the IPC endpoint is a Unix socket alone) — the endpoint is `<dir>/viola-<h12>.sock` under `$XDG_RUNTIME_DIR/viola/` or the `/tmp/viola-<uid>/` fallback, never the `/tmp` root, chmod 0600 after the bind. Matters here: CI runners and killed tests share this per-user socket dir; rows decision/per-user-socket-directory, decision/no-socket-in-tmp-root bound where any test-placed socket may go.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home tree checked at `run` and `revive`'s start, before the wrapper log) — `check_home_tree` holds the home, instance, `ledger/`, `bin/`, `plugin/`, the pinned copy and plugin folders to strict modes before the log opens; a refusal exits 1 with no log, line or child. Matters here: a rebuilt stamped home and every fixture home a booted test uses must pass this check as laid down (rows decision/start-checks-home-tree-before-log-opens, decision/strict-modes-at-start, decision/state-tree-strict-modes-before-use, decision/home-tree-strict-modes-before-log).

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`ledger/stamps.json` checked before `ledger/` is made, and `verify`'s home tree before its log) — `update_stamps` runs `check_stamps` before `create_private_dir(ledger/)` and again under its lock; `verify` checks the home tree before its cli log; new ban that the check never runs after a create, mode set or log open that narrows the path. Matters here: re-stamping the shared fixture home runs `verify` against the fake agent through this order (rows decision/stamps-checked-under-lock, decision/verify-home-tree-check, decision/strict-check-before-narrowing, decision/stamps-strict-check-first, contract/strict-modes-refusal/verify).

@history_item n=16 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the two instance checks ahead of any log) — measured that a log opened before or after a refused check narrows the home to 0700 and erases the state the check refuses, so checks moved ahead of every log. Matters here: a trap for any fixture or harness change that opens a log or role file before the home is checked (row decision/start-checks-home-tree-before-log-opens's why).

@history_item n=17 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** (the wiped mutation scratch re-created owner-only) — a harness lay-down failed the instance check inside a plainly re-created dir; fixed by `create_private_dir` in the harness, never by loosening `check_instance`. Matters here: the precedent for a rebuilt test home: make it owner-only so it passes strict modes, never widen the check.

@history_item n=18 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (run's strict stamps read) — `run`'s version gate reads stamps through `read_stamps_strict`; a refusal, unreadable or malformed file is one `parse-rejected{ledger-stamps}` and `cli_verified:false`; no non-null answer unless that read passed. Matters here: the CI red's booted wrapper refusing `unverified-cli` is this gate reading an absent stamp (rows decision/version-gate-strict-stamps-read, contract/strict-modes-refusal/version-gate, decision/dialog-answers-need-verify-stamp, contract/cli-exit-codes/12); the fix must restore a real verify stamp, never relax the gate.

@history_item n=19 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set holds on every build) — `LedgerRow::relied()` is all twenty-four rows, counter `/24`, a `20 pass  4 fail` stamp reading `cli_verified:false`. Matters here: a rebuilt stamped home must carry a stamp over the full relied set or booted tests read unverified, as in the CI red (rows decision/unverified-cli-gate, decision/dialog-answers-need-verify-stamp).

@history_item n=20 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (`MAX_FRAME` consumers extended, verify's CI split) — the real CLI runs only locally; `verify` runs in CI only against the fake agent, its real-CLI probe and `--record` only locally. Matters here: the stamped fixture home is stamped by a fake-agent verify in CI and the chunk starts no live `claude` (rows decision/real-cli-local-only, decision/ci-no-secrets-no-real-claude).

@history_item n=21 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the first-start race on the pinned copy closed, a lost placement re-hashed; the pinned `viola` placed by `place_private_new`) — the pinned copy is placed once by a no-clobber placement with its write handle closed (defeating `ETXTBSY`), never replaced; a lost placement is re-hashed and a mismatch left as found. Matters here: the CI red's `pinned: NotFound` is this copy in the stamped home; concurrent booted tests rebuilding a home race on exactly this placement, and pinned artefacts are never deleted (row decision/pinned-artefacts-never-deleted).

@history_item n=22 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (Measurement B recorded as known) — one tree's release builds can key two `bin/<key>/` folders; known with no action, the re-hash the guard. Matters here: a stamp marker restored from a `target/` cache may name a key or home that no longer matches the host, so validity must be judged on the home as present, not on the marker.

@history_item n=23 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** (pinned actions and toolchain source) — fixed the SHA-pinned action set, `Swatinem/rust-cache` among it, and event values only through `env:`. Matters here: the CI red's hypothesis is a `rust-cache` restore of `target/`; any cache-key or cache-scope change the fix makes must keep the pinned action and the no-event-value rule.

@history_item n=24 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** (registry migration: v1.x release prerequisites) — the release prerequisites name zizmor cache-poisoning and no `rust-cache` for the release workflow. Matters here: a standing statement that a restored cache is not trusted for release; this chunk's fix answers the same class (a restored `target/` the test fixture trusted) on the test side.

@history_item n=25 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** (CI runs the obs artifact canary scan before uploads) — every test-home upload waits for the artifact canary scan against the NEVER-log floor, never printing matched bytes. Matters here: any change to where test homes live or what a killed test leaves in CI stays under the scan-gated upload (rows decision/ci-artifact-canary-scan, decision/scan-gated-ci-uploads, decision/artifact-canary-scan).

@history_item n=26 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** (unscanned uploads; the `mutants.out/` CARRY closed by removal) — `mutants.out/` is not uploaded; only content-admissible uploads stay unscanned. Matters here: this chunk exists because of a mutation run; it runs none, and it adds no upload of mutation output or killed-test state (row decision/no-mutants-out-upload).

@history_item n=27 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** (the CI and pre-push mutation legs retired) — no CI job runs mutation; it belongs to the epoch-boundary audit. Matters here: the measured leftover wrappers came from a local mutation run; the fix must not reintroduce a CI mutation job as its witness (row decision/mutation-testing-at-audit).

@history_item n=28 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (the feed bound closed) — the tee to the vt100 feed is `sync_channel(256)` with `try_send` after the human's write, a dropped copy poisoning the model, never blocking passthrough. Matters here: P3 must explain why a killed test's wrapper spun at about 99 % of a core; the pump, tee and feed loop once the PTY or parent is gone is the likely site, and any change there must keep rows decision/vt100-feed-bounded-queue and decision/vt100-feed-isolated.

@history_item n=29 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** (the vt100 degrade as landed on the PTY output row) — the feed thread runs every feed and resize under `catch_unwind`, a caught panic poisoning the model until a size change while passthrough continues. Matters here: the same pump/feed loop the spin analysis reads; a fix ending the wrapper when its terminal is gone must not break the panic isolation (row decision/vt100-feed-under-panic-catch).

@history_item n=30 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (handle inheritance measured on portable-pty) — the wrapped child's fd table holds fds 0–2 on the pty and no viola socket or home file, witnessed by E2 and a live read. Matters here: a parent-death or PTY-gone mechanism added to end the wrapper or fake agent must not leak a channel descriptor into the child (row decision/no-handle-inheritance).

@history_item n=31 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** (the VIOLA_DIR home step ratified) — the CLI verbs' `VIOLA_DIR` step and the canonicalisation, `mcp` included, ratified. Matters here: booted tests choose their state root by `--home` / `VIOLA_DIR` (rows decision/cli-home-resolution-order, contract/cli-state-root/home-flag, contract/cli-state-root/viola-dir); nothing is created by resolution, so a missing stamped home must be rebuilt by the fixture, not by the product.

@history_item n=32 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** (the tempfile `persist` helper) — every replaced file goes through `replace_private`, mode set on the temp file before any byte. Matters here: a rebuilt fixture home written through product helpers keeps rows decision/explicit-mode-before-write and decision/home-owner-only; a test-side write of a marker should not drop to the umask.

@history_item n=33 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Secret Management: the live rig isolated child) — added decision/live-rig-isolated-child and decision/live-rig-credentials-copy for the standing rig, whose census the scope names as the precedent. Matters here: the rig owns only its own processes and is out of scope; the chunk's census copies its approach without reworking it or sharing its credentials handling.
