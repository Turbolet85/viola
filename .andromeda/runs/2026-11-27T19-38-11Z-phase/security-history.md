@history_item n=1 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the CI and pre-push mutation legs retired, mutation moved to the epoch-boundary audit (decision/mutation-testing-at-audit), the distro `TMPDIR` carve-out and the leg-verdict upload retired as a narrowing. Item 1's one announced mutation run is that audit's own addendum, run locally: it must add no CI mutation job and no upload.

@history_item n=2 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — closed the `mutants.out/` CARRY by removing that upload; its unscanned-upload list admits only content-safe artifacts (decision/no-mutants-out-upload). Item 1's addendum lands in the audit ledger record. It must never upload `mutants.out`, which holds absolute argv paths and test output.

@history_item n=3 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** — the Linux host's NOCOW mutation `TMPDIR` belongs to the operator and the harness never wipes it; `env -i` crosses HOME and PATH only. Item 1's `TMPDIR` is a short-named dir under `~/dev/projects/viola-mutants-scratch/`: create it fresh and never let the harness wipe a directory it did not make.

@history_item n=4 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** — a root-package mutation baseline once failed `check_instance` inside a plainly re-created scratch. The fix was an owner-only re-create, and NEVER loosen the check for it. This is a trap for item 1's baseline: a group- or world-permissioned `TMPDIR` fails the strict-modes tests with a refusal, not a timeout. Fix the scratch, never the check (decision/strict-modes-check, decision/controls-not-switchable).

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (CI one leg; windows-mutants.yml and the mutation scratch retired) — `windows-mutants.yml`, `scratch.rs` and the `run --mutants` scratch-wipe ban are retired; CI is one `ubuntu-latest` leg. Item 1 must not revive a Windows mutation workflow or a harness scratch wipe. The same entry made an unstattable home a strict-modes refusal on Linux (decision/unstattable-home-refused) and the strict-modes check owner and mode only.

@history_item n=6 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — `target/e2e-home` is a link to `/tmp/viola-e2e-home-<uid>`. Test-side removals take only their own dirs, and the keepers refuse a relative or linked target. Recorded trap: a mutation run of the harness keeper leaves `backing/` in its copied tree. This bears on item 1's `--copy-target=false` run and on any test re-pointed by the splits.

@history_item n=7 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — the e2e-home keepers also refuse a backing with less than the 9 GiB headroom floor free, a compiled constant that no env var, flag or config sets. Item 1's `-j 5` run on the dev host can meet this refusal. The floor must not be lowered or switched off to make the baseline pass.

@history_item n=8 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — `FAKE_AGENT_PUMP_DELAY_MS` was ratified as the one fake-agent-only, 5 s-capped seam that holds the run pump back, and any other seam needs its own Decisions Log entry (decision/fake-agent-pump-delay-seam, decision/new-env-read-needs-log-entry, decision/test-seams-fake-agent-only). Item 2's killing tests for the pump's restart and time limits must use this seam or injected values, never a new env seam.

@history_item n=9 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — the `FAKE_AGENT_HOOK_PANIC` seam and G2's single exact-path exemption, which counts panic lines and compares the location as a whole string (decision/g2-single-panic-exemption, decision/g2-zero-panics-single-exemption). New killing tests (items 2 and 4) and the structural moves must not add a panic line or a second exemption.

@history_item n=10 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the tests holding the directory and peer checks; a refused socket directory at a start) — sets the precedent for items 2 and 4's remove-the-guard controls: they were one-off forced runs, and a committed seam to force a refusal is banned. The same entry built `socket_dir_ready`, which `revive`'s start shares before any snapshot, child or bind (decision/start-checks-socket-dir-before-bind, contract/socket-dir-refusal/run-and-revive-start), so item 3's shared prologue must keep that order.

@history_item n=11 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — closed the vt100 feed bound: `sync_channel(FEED_CAPACITY)` of 256, `try_send` after the human's write, and a dropped copy poisons the model (decision/vt100-feed-bounded-queue). Item 6's pump/restart split and item 2's tests must keep the feed off the passthrough: bounded, never blocking.

@history_item n=12 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — the tee writes the human's bytes first and feeds vt100 under `catch_unwind`; a caught panic poisons the model until the size changes (decision/vt100-feed-isolated, decision/vt100-feed-under-panic-catch). The order must survive the restart code's move out of `crates/viola-pty/src/pump.rs`.

@history_item n=13 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the passthrough after a caught feed panic; the bounded-reader census) — `chaos_feed_panic_refuses_send_and_keeps_passthrough` guards the feed row. `tests/contract_bounded_readers.rs` `product_readers_are_each_bounded_or_named` requires every product reader to be bounded in its statement or named (decision/bounded-readers-census, decision/bounded-external-readers). The three splits move readers: any named exception's path must be re-pointed, never widened.

@history_item n=14 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error bodies' census; the clients' screen) — `tests/contract_error_bodies.rs` `product_error_writers_are_each_named` fails on any unnamed product error writer (decision/every-error-writer-named). The `sanitised_fault`/closed-word screen covers what clients print (decision/client-reads-closed-values-only, decision/sanitised-external-errors). Item 7's one `refused()` helper and item 6's moves must stay named in the census by their new path, with the `--json` refusal shape unchanged.

@history_item n=15 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21 names its cause; revive --json documents every refusal of its start) — the closed exit-21 causes and the `revive --json` closed exit-1 codes (contract/cli-instance-unreachable/json-document, contract/instance-unreachable-cause/strict-modes-failed, contract/revive-json/error-document, contract/cli-json-error/internal-error). An anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints `internal-error`. Item 3's shared prologue must keep each document and code byte-identical for revive and adopt.

@history_item n=16 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the two instance checks ahead of any log) — `revive`'s check was moved before the wrapper log because a log opened before or after a refused check narrowed the home to 0700 and erased the state the check refuses. This is the measured origin of decision/strict-refusal-leaves-no-trace and decision/strict-check-before-narrowing, and the order item 3's prologue must hold in one place.

@history_item n=17 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (the arm's check before its log; revive's log before its check) — measured that obs init sets an existing home to 0700, so a check made after it passes a 0770 home. It also recorded `revive`'s start opening its log before the check, the trap the next entry fixed. This is a precedent for item 3: the move must not slip `open_wrapper_log` ahead of the tree check.

@history_item n=18 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`viola link` / `viola unlink`'s instance check before any log) — recorded trap: opening a role file narrows an existing home to 0700, so a check placed after `client::start` passes a 0770 home. Its guard reads the mode after the refusal, and its remove-the-guard control went red when the check was moved after. This is the template for item 3's pure-move proof that the order stayed.

@history_item n=19 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home tree checked at run and revive's start; `check_instance`'s set and the read rule; `ledger/stamps.json` before `ledger/` is made) — built `check_home_tree` through `checked_exe` before the wrapper log, with `refuse_tree` exiting 1 with no log, line or child (decision/start-checks-home-tree-before-log-opens, decision/home-tree-strict-modes-before-log, contract/strict-modes-refusal/run-revive). It set the `Readable` and `Symlink` messages (contract/strict-refusal-message/*) and the ban on checking after a create, mode set or log open. This is the code item 3 unifies.

@history_item n=20 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder accepted `viola revive`'s refused strict-modes check opening no log and writing no line, as built. Item 3's shared prologue carries that ratified no-line shape for adopt as well.

@history_item n=21 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the recorded cwd ratified) — the recorded cwd is a spawn directory read only after the strict-modes check; later it also became the trust-record key (decision/revive-recorded-cwd-spawn-dir, decision/revive-cwd-spawn-only, decision/revive-strict-modes-before-use). The prologue move must keep the cwd read after the check, and it must stay off log lines and stdout (decision/cwd-only-in-snapshot).

@history_item n=22 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** — adopt's start joined every start-order row: home tree before log, socket dir before bind, the trust step, `CLAUDE_CONFIG_DIR`, and internal-error silence (contract/strict-modes-refusal/adopt, decision/start-checks-*, decision/state-tree-strict-modes-before-use). It also added decision/r5-adopt-script-ends-own-processes. This is the copy item 3 folds into one function with revive's prologue, and each row naming "run, revive and adopt" must still hold after the move.

@history_item n=23 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (W1/W2/W5 ratified) — the operator ratified adopt's rows as landed (2026-11-25 wrap): the trust flag, the `CLAUDE_CONFIG_DIR` read and the R8 except as "any start". Because they are ratified, item 3 must change no behaviour of adopt's start; any change would reopen them.

@history_item n=24 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — the start's trust step, ratified 2026-11-22. It is flag-only, skips trusted folders and the home folder, goes through the one guarded writer, ends in 16 closed start words, and reads `CLAUDE_CONFIG_DIR` only when kept (decision/start-writes-trust-record-flag-only, decision/start-trust-skips-trusted-folders, decision/trust-record-writer-guarded, decision/trust-record-closed-words, decision/credential-strip-by-name-only, decision/r8-claude-env-strip). Item 6 moves this trust step out of `src/cmd/run.rs` and must change none of it.

@history_item n=25 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — the trust-record writer's own exception: it keeps Claude's config mode (decision/explicit-mode-before-write except) and leaves nothing under `~/.claude` but the bounded excepts (decision/nothing-written-under-claude-home, decision/viola-writes-nothing-in-claude-dir, decision/no-writes-under-claude-home). Moving the trust step out of `run.rs` must keep it on that one writer and on `replace_private`-style mode rules (decision/one-replace-helper).

@history_item n=26 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — added the wrapper's window step (decision/window-step-moves-own-terminal-window). It also added viola run's hidden `--origin`/`--place` checks (decision/run-window-flags-checked), the hyprctl PATH resolution, R8 strip and 5 s deadline (decision/hyprctl-read-deadline), open's direct terminal spawn (decision/open-terminal-direct-spawn), the in-memory 8-pid ancestor walk (decision/open-ancestor-walk-in-memory) and the window record kept to the snapshot and event log. These are the window code item 6 moves out of `run.rs`, and item 4's `ancestors(pid, max)` bound is that walk's limit of 8.

@history_item n=27 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — added focus's second ancestor walk, at most 8 pids in memory and never recorded (decision/focus-ancestor-walk-in-memory), and exit 21 for focus (contract/cli-exit-codes/21). Item 4's killing test for `chain.len() < max` at `crates/viola-state/src/liveness.rs:66` guards the bound both walks rely on.

@history_item n=28 marker=2026-11-25-placing-without-moving-the-view
**2026-11-25-placing-without-moving-the-view** — the window step switches back once only when the move changed the workspace, and a failed read changes no placement. It added decision/r7-place-script-closes-own-terminal (refuses on a locked desktop, unsets inherited `VIOLA_` names, closes only its own terminal) and verify's `untrusted-folder` refusal (decision/verify-never-answers-import-dialog, contract/verify-refusal-codes/untrusted-folder). R7 is the sibling whose lock and fresh-scratch preconditions item 5 copies into `scripts/r2-open.sh`. The window step is code item 6 moves.

@history_item n=29 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — viola up runs the strict board scan before its log and re-hashes the pinned copy before its first spawn (decision/up-checks-before-log-and-spawn), with exit 21 and the `strict-modes-failed` document (contract/strict-modes-refusal/up, contract/cli-exit-codes/21). It spawns its terminal through open's shared direct spawn, and the recorded cwd goes on the terminal argv (decision/cwd-only-in-snapshot, decision/spawn-cwd-snapshot-only). Item 6's `upped` split and item 7's shared `refused()` must keep this check order and refusal shape.

@history_item n=30 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** — the wrapper ends only its own live child and resumes in place, with a fixed refusal order (decision/switch-ends-only-its-own-child, decision/switch-refusal-order). This is the behaviour behind the pump's restart path, whose nine survivors item 2 kills and whose code item 6 moves out of `pump.rs`. The move must not let the restart touch any process but its own child.

@history_item n=31 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the tokio ban's tree form) — tokio is banned per sync crate as sole root, and `viola-channel`'s Tokio client sits behind its feature (decision/sync-crate-tokio-ban, decision/sync-crates-tokio-free, decision/no-tokio-in-sync-crates). `viola-pty` and `viola-state` are sync crates, so item 6's pump split and items 2 and 4's tests add no tokio edge.

@history_item n=32 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — the stamp rule: every relied ledger row must count, either pass or absent on the browser-tools-prompt row alone (decision/unverified-cli-gate, decision/dialog-answers-need-verify-stamp, contract/cli-exit-codes/12). `tests/cli_answer.rs`'s `path4_…` test, which item 1 moves into the verify kill class, runs a whole verify on a custom-stamped home to prove this gate. Moving its kill limit must not edit what it asserts.

@history_item n=33 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** (Authentication & Authorization; Input Validation; Data Protection: provisional marks ended) — the operator's own word of 2026-11-27 set standing, text unchanged: decision/window-step-moves-own-terminal-window, decision/r7-place-script-closes-own-terminal, decision/up-checks-before-log-and-spawn, decision/run-window-flags-checked, decision/open-terminal-direct-spawn, decision/open-ancestor-walk-in-memory, decision/window-record-kept-to-snapshot-and-event-log and their sibling switch, launch and board rows. Every row this chunk's moves touch is ratified now, so a pure move must leave them true word for word.
