@history_item n=1 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — moved mutation testing out of CI and the pre-push into `agent-run run --mutants`, kept for the epoch-boundary audit (decision/mutation-at-epoch-boundary); item 1's one announced root-crate mutation run is exactly that audit run, and no CI or pre-push mutation stage may be reintroduced.

@history_item n=2 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** (the mutation scratch dir on the Linux host) — made `<repo parent>/viola-mutants-scratch/` the operator's NOCOW `TMPDIR` for every mutation run, because cargo-mutants copies the ~14 GB tree, `/tmp` is a quota tmpfs, and a COW reflink drops the fake agent's exec bit (decision/mutation-scratch-nocow); item 1's `TMPDIR` under `~/dev/projects/viola-mutants-scratch/` rests on this trap.

@history_item n=3 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (mutation runs: no scratch arm, no Windows leg) — removed the harness scratch arm (`scratch.rs`, `HOST_SCRATCH`), leaving the NOCOW `TMPDIR` an environment fact and `mutants.out/` at the repo root, and dropped `windows-mutants.yml`; the addendum run is Linux-only and its scratch is set by the operator, not by harness code.

@history_item n=4 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** (registry rows) — registered `target/mutants/`, the mutation run's own target dir, the origin of decision/mutants-own-target-dir; item 1's `--copy-target=false` and relative target dir must keep each copied tree building its own test binaries.

@history_item n=5 marker=2026-09-26-ci-chunk-base-and-union-verdict
**2026-09-26-ci-chunk-base-and-union-verdict** — made `AGENT_RUN_CHUNK_BASE` an override of the derived diff base and `target/agent-run/chunk.diff` the one file secret-scan skips (decision/chunk-diff-secret-scan-skip, contract/harness-environment/agent-run-chunk-base); item 1's `--in-diff` from `15a566e8` in the recorded form goes through this base and diff.

@history_item n=6 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — registered `target/run-archive/<n>/` (never uploaded) and the mutation scratch; the addendum's mutation outcomes carry absolute argv paths, so they stay in the local archive (decision/run-archive-never-uploaded).

@history_item n=7 marker=2026-10-13-epoch-4-survivors
**2026-10-13-epoch-4-survivors** — the precedent for killing boundary-audit survivors: const-gated `replace_private_with` mutants became reachable only once the caller passed the retry decision in (`replace_private_retrying`), and the PTY watch report went per-process; items 2 and 4 (viola-pty and viola-state survivors) follow this pattern of a test seam by parameter, never an env or feature seam.

@history_item n=8 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** (send and human split by phase) — split `src/human.rs` and `src/run/send.rs` into module roots plus phase files as a pure move, every caller's path kept through re-exports and no file over 800 code lines; the precedent item 6's three splits (`up.rs`, `pump.rs`, `run.rs`) follow.

@history_item n=9 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the split modules) — split `src/cmd/verify/typed.rs` and `src/human/board.rs` into module roots over sub-files, the key file's tree rows updated with the moved symbols; the second pure-move precedent, and a reminder that a split changes the paths architecture cites (e.g. `typed::turns::agents_read`).

@history_item n=10 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — made `cargo modules orphans --deny` per target a CI gate via `scripts/orphans-check.sh` (decision/no-orphan-modules); every file item 6 moves out must be declared as a module or the gate goes red.

@history_item n=11 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (bounded readers, held by a census) — `tests/contract_bounded_readers.rs` names each bounded read by file in its `BOUNDARIES` table and censuses the product sources (decision/bounded-readers); moving reads out of `pump.rs`, `run.rs` or `up.rs` must re-point those rows, the one test edit item 6 allows.

@history_item n=12 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the pattern and its contract binary) — `tests/contract_error_bodies.rs` names each error writer and witness, with a census of product error writers (decision/sanitised-error-surfaces); folding `open.rs`'s and `up.rs`'s `refused()` into one (item 7) and moving `run.rs` writers must keep each row naming a writer that stands.

@history_item n=13 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the instance check before any log, on both arms) — moved revive's start-arm `check_instance` ahead of its wrapper log because opening a log narrows the home to 0700, so a later check passed a 0770 home (decision/strict-check-before-log-open, decision/refused-check-writes-no-line); the ordering item 3's shared revive/adopt prologue must keep.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree check before the start's and verify's log; the pin key taken once) — built `check_home_tree` called through `checked_exe` / `refuse_tree` before the wrapper log, `pin_key` of the exe bytes read once and `pin_bytes` pinning those bytes (decision/start-pins-the-bytes-it-checked, contract/run-start-refusals/home-tree); this is the prologue `adopt.rs` copies from `revive.rs`.

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the strict-modes refusals' exits) — fixed exit 1 for `run`'s and revive's start refusing the home tree (home or instance pair, `strict-modes-failed` under `revive --json`), no log, no line (contract/cli-exit-codes/1, contract/viola-run-start-exit-codes/1); item 3's prologue move must leave these refusals byte-identical.

@history_item n=16 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (one read per revive, one state-recovered line) — revive reads the snapshot once through the replay and hands it to its collision check and recorded-directory reading (decision/revive-reads-snapshot-once); a shared prologue must not add a second replay or a second `state-recovered` line.

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (checked entry points and `check_instance`'s call sites) — records the call-site count by the rule `check_instance(` under `src/`, with files moved; trap: merging revive's and adopt's prologue or splitting files moves that count, which the master states by its rule.

@history_item n=18 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`check_instance` at three sites) — set the precedent that a call-site count is taken by the rule its master states, not the plan's count; any count item 3 or item 6 moves is re-measured by that rule.

@history_item n=19 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the snapshot's `cwd` and the child's second cwd source) — a revived start spawns in the recorded `cwd`, read only after the strict-modes check and required to exist (decision/child-cwd-explicit, contract/instance-snapshot/cwd); a prologue shared with adopt keeps that order.

@history_item n=20 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (revive refuses a session id live elsewhere) — added revive's `session-live` preflight reading, `--fork` skipping it and a failed read passing (decision/revive-refuses-live-session); adopt's live-session refusal mirrors it, part of what item 3 deduplicates.

@history_item n=21 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola revive --json` documents every refusal of its start) — added `Launch`'s `json` flag so `refused_with` prints `refused_document` for start refusals and an anyhow error of `trusted` / `checked_exe` / `open_wrapper_log` / `preflight` / `start` prints `internal-error` (contract/viola-revive-json/error-document, contract/revive-json/*); the prologue functions item 3 merges carry these documents.

@history_item n=22 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`stale-heartbeat`, a stale holder's own code) — the collision check refuses `already-live` or `stale-heartbeat`, printed and logged (decision/run-refuses-held-name); revive's and adopt's holder check share it.

@history_item n=23 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the start's refused socket directory) — `socket_dir_ready` in `src/cmd/run.rs` refuses before any snapshot, child or bind with exit 1 or the `socket-dir-refused` document (contract/run-start-refusals/socket-dir-refused); part of the `run.rs` start the split must keep in order.

@history_item n=24 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Established Decisions) — landed adopt through run's start with revive's live-session refusal, holder check and launch pair (decision/adopt-refuses-live-session, decision/no-start-over-live-instance, decision/run-refuses-held-name); the origin of the copied prologue item 3 makes one.

@history_item n=25 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Conventions / Standard Contracts) — adopt's start silent at the internal-error catch site and its `--json` refusal codes (decision/silent-roles-at-internal-error-catch, contract/cli-json-output/adopt-refusal-codes); the shared prologue must keep adopt's codes and silence unchanged.

@history_item n=26 marker=2026-11-24-adopt-s-resume-folder
**2026-11-24-adopt-s-resume-folder** — adopt gained its own `not-resumed` code and its document after the child released the terminal; adopt's tail differs from revive's, so only the prologue, not the whole start, is shared.

@history_item n=27 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (revive keeps the pair) — revive hands the recorded model and effort pair (decision/revive-in-place, decision/revive-resumes-in-place); revive's prologue reads the record the pair comes from.

@history_item n=28 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the replay's first product reader, revive as the second way a wrapper starts) — revive resumes the gone instance in place through the same start as `run`, its first record `wheel{driver, start}` (decision/revive-resumes-in-place); the shared start path item 3 must not fork.

@history_item n=29 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (Established Decisions) — built switch in place: the wrapper ends only its own PTY child, reaps it within `RESPAWN_REAP_BOUND`, appends `launch` and resumes the same id (decision/switch-in-place); this is the `pump.rs` restart path whose generation guard and `reap_within` / `drain` limits hold item 2's nine survivors and item 6's moved restart code.

@history_item n=30 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (Standard Contracts) — after a respawn the old child's late `session-end` is dropped once, `launch` and `window` re-appended, `child_pid` rewritten (contract/wrapper-event-appends/window, contract/instance-snapshot/child-pid); the restart code moved out of `pump.rs` must keep these appends and their order.

@history_item n=31 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** (Established Decisions: 5 row(s)) — ratified decision/switch-in-place, decision/switch-cli-only and decision/up-parents-first as standing (text unchanged); the rows item 2 and item 6 rest on are no longer provisional.

@history_item n=32 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** ([PTY]: openpty alone) — the pump keeps the input writer until the child exits, `pty_backend()` is always `openpty`, child exit stays on the handle wait (decision/child-exit-on-process-handle, decision/portable-pty-seam); behaviour the pump restart tests and split must not change.

@history_item n=33 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (portable-pty pinned at =0.9.0) — moved the seam to portable-pty `=0.9.0` with `viola-pty` unchanged; item 2's killing tests run against this pin, with no dependency move.

@history_item n=34 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — ratified `FAKE_AGENT_PUMP_DELAY_MS`, read only under `fake-agent` in `hold_pump_start` (`src/cmd/run.rs`), capped at 5 s (decision/fake-agent-pump-delay-seam, decision/fake-agent-test-seams); a `run.rs` split moves this seam with its feature gate intact.

@history_item n=35 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the Tokio client and viola-channel's tree-form ban) — viola-channel's `fake-agent = ["tokio"]` and the sync crates' code-free `fake-agent` features (decision/fake-agent-feature-on-named-members, decision/tokio-containment); package-scoped mutation builds of `viola-pty` and `viola-state` resolve through them.

@history_item n=36 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — the previous baseline-green chunk raised the shared root wait `WITHIN` to 12 s and moved test sockets under a short `/tmp` root rather than raising per-test limits; precedent for item 1 mending the `path4_…` timeout by moving it into the verify-class override, not raising the `mutants` profile's general 15 s.

@history_item n=37 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — added the start's trust step for an untrusted spawn folder, keyed by the spawn dir, its `key-present` / `removed` words read as lost by a start (decision/run-start-order, decision/spawn-directory-confined-to-snapshot, contract/trust-writer-words/*); the trust step item 6 moves out of `src/cmd/run.rs`.

@history_item n=38 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — reordered the start: bind, any automatic verify under `ledger/verify.lock`, then the trust record on the final verdict (decision/run-start-order, decision/start-verifies-new-cli-version, decision/automatic-verify-attempt-lock); the moved trust step must keep running after the automatic verify and before the first snapshot.

@history_item n=39 marker=2026-11-25-placing-without-moving-the-view
**2026-11-25-placing-without-moving-the-view** — verify refuses `untrusted-folder` when Runs B to E read the trust or imports form, and a start's automatic verify then records nothing (decision/verify-refuses-untrusted-folder); landed with the window step that keeps the view, both in the `run.rs` code item 6 splits.

@history_item n=40 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — landed `viola open`, its exit codes and refusal documents, the hidden `run --origin` / `--place` and the `window` event and snapshot record (contract/viola-open-exit-codes/1, contract/event-data/window, contract/instance-snapshot/window, contract/wrapper-event-appends/window); `open.rs`'s `refused()` (item 7) and `run.rs`'s window step (item 6) come from here.

@history_item n=41 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Conventions / Occupied Resources) — landed `viola up`, its exit codes 0 / 1 / 21, `boot-time-unread` and the shared `not-live` / `terminal-not-found` details (contract/viola-up-exit-codes/*, contract/cli-verbs/up); `up.rs`'s `refused()` copy and its `--json` refusal shape, which item 7 must leave unchanged.

@history_item n=42 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Established Decisions / Standard Contracts) — up's selection rule, parents-first order, and that it writes no state of its own (decision/up-selection-rule, decision/up-parents-first, decision/up-writes-no-state); the logic in `upped` item 6 splits as a pure move.

@history_item n=43 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — made `viola-hyprland` the one compositor seam, sync, tokio-free, depending on `viola-core` alone (decision/hyprland-seam-crate); item 5's `hyprctl locked` precondition in `scripts/r2-open.sh` is a script-side read outside the product seam, and the chunk dispatches nothing to the compositor.

@history_item n=44 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — landed `viola-state` with pid plus OS start time deciding `gone` (decision/liveness-heartbeat-and-pid, contract/session-liveness/gone); the liveness module whose ancestor-walk bound item 4's survivors sit in.
