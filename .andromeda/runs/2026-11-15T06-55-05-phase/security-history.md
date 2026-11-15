@history_item n=1 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `dialog_hold_s` is validated into `DialogHold` (60–3600 s, default 60). It is read once at the wrapper's start from `config.json` only, and no env var or flag sets it. The hook's dialog reply read is bounded at `DIALOG_HOLD_MAX + 5 s` (3605 s). Ratified by the founder at P4 fork Q1. Why it matters: this is the source of the 3600 s ceiling that v2-02 must read live (rows dialog-hold-config-only, hook-dialog-reply-read-bound, contract/config-json/dialog-hold-s). Moving the ceiling on fork (4) also moves the hook bound, and `plugin/<key>/` must stay byte-identical across starts.

@history_item n=2 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — The founder's Epoch 5 batch superseded the provisional card confirms of the dialog-hold, plugin-install residual, plugin-precedence rig keys, Run B/E, agents-read and MCP entries with his own word. Why it matters: the dialog hold's validated surface is ratified. Only the 3600 s value's PROVISIONAL mark (residual res-ujtjay7k) is open, and that mark is fork (4).

@history_item n=3 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — Served `hook.dialog` and `answer`. The hook's reply read was bounded, and `run`'s stamps read now goes through the strict check before any non-null decision. Why it matters: a live dialog-hold reading depends on the stamp gate (rows dialog-answers-need-verify-stamp, version-gate-strict-stamps-read, unverified-cli-gate). v2-03 also needs `cli_verified:true` on the ledger for the CLI the round runs.

@history_item n=4 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — The live rig runs outside the work tree, in a `proj/` on the e2e-home tmpfs that is its own git repo. It passes the folder-trust dialog only by keys into its own pty: at most 2 arrows and 1 Enter, and only on a read `selected`. On a wrapped start, `viola release` precedes the driver's send. Residual: the CLI's own `~/.claude.json` trust entry and transcripts. The same entry ratified the VIOLA_DIR home step (row cli-home-resolution-order). Why it matters: this is the precedent for fork (2), the per-chunk word for rig keys on a trust dialog. An in-tree `proj/` meets the external-imports modal (trap).

@history_item n=5 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — A live round's plugin-install residual under `~/.claude` (the CLI's own writes; viola writes nothing there). `marketplace/viola/.mcp.json` names the canonical exe by an absolute path. Why it matters: these are the residual class and the no-writes rule (no-writes-under-claude-home, exec-commands-absolute-pinned-path) that a live wrapped start through a freshly written plugin must respect.

@history_item n=6 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — `Mcp.` exchange fixtures are recorded by the test-only `viola-mcp-tap`. The tap makes a direct spawn of `viola mcp` (no shell), uses a `take(MAX_FRAME)` line read and a 0600 private capture that is never committed, and applies the same scrub with the closed refusal codes. Ratified as a boundary widening at a cap of one start, with the prompt on stdin, and each answer was spent. Why it matters: CARRY 1 re-records `fixtures/claude/2.1.302/Mcp.tools-list-1.json` through this tap (row mcp-fixtures-via-test-only-tap). The live start count needs fresh founder caps (fork 1).

@history_item n=7 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — The MCP `list` tool makes one `claude agents --json` read per call through the board path, and its agents-probe pair logs no row text, id, cwd or path. Tool inputs are validated, and leaving release/pause/link/unlink off the tool list is an affordance. Why it matters: the `list` tool's description is what CARRY 1 re-records (contract/mcp-tools/list; rows mcp-tool-list-affordance, agents-probe-log-content-free).

@history_item n=8 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — The budget gate, `budget.json`, `five_hour` and `seven_day` were removed. `config-cannot-disable-controls` lost its budget clause. The release params became `{from?}`, and the home-write-rule checkers lost the budget read. Why it matters: the MCP `list` description still says «and the usage budget» (CARRY 1, found at this chunk's P5 card). The documented start order now has no budget step, and v2-03 must show that.

@history_item n=9 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — Every status-line row was retired: the source read, the `/bin/sh -c` shell-out, the settings override and `--settings`, the statusline capture arm, and `statusline_command` in the snapshot. viola runs no shell. The rows for capture-arms-isolated, hook-silent-exit-zero, code-bearing/plugin-files-rewritten-each-start, max-frame-shared-cap, home-strict-modes-before-read, strict-modes-check, absolute-viola-path-in-commands, snapshot-written-only-by-wrapper, replayed-snapshot-fallbacks, cli-state-root contracts and user-content-only-to-diagnostics were amended to match. Why it matters: this is the trimmed product the live round proves. CARRY 3's unwrapped-vs-wrapped status-line reading rests on «no `--settings`, no `settings.json`». The retired StatusLine fixtures and the dropped `workspace.repo` except clear the path for a clean `--record` (CARRY 2).

@history_item n=10 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `screen-fixture-signature-rows-only` now keeps a box rule row as its rule chars alone and a prompt row as the prompt char alone, and refuses any other box row. `input-box-readiness-from-literals` was added: one predicate for verify and `run`'s pre-send gate. The card confirm was a delegate's (PROVISIONAL). Why it matters: CARRY 2's box-only screen fixture must pass exactly this form under the fixture walk, recorded by a clean `verify --record`, never hand-written.

@history_item n=11 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's interactive PTY children are direct spawns under the R8 strip at 80×24, with output capped and read only against compiled literals, and no byte goes into a dialog (a modal start is killed with no key). The probe dirs are 0700. The fixture refusal has the closed codes home-path/absolute-path/username/email. The transcript residual was accepted, and verify needs a pre-trusted folder. Why it matters: these are the binding rows for any live `verify --record` this round runs (verify-children-direct-spawn, verify-writes-no-dialog-byte, fixture-scrub-and-refusal, contract/fixture-refusal-*, verify-never-answers-import).

@history_item n=12 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — The hidden `hook --capture` arm, transient 0600 probe captures, and the `--record` scrub that refuses the whole recording. The real CLI runs only locally, and in CI verify runs only against the fake agent. Why it matters: these are the origins of hook-capture-arm, verify-probe-captures-transient and real-cli-local-only. The rigs' fake rehearsals belong in CI, and the live steps stay local.

@history_item n=13 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — Runs C and D were added. Probe dialogs are answered only by the capture arm's output, and no plan file goes under `~/.claude/plans`. Why it matters: these are the rows verify-probes-confined-to-own-dir and verify-writes-no-dialog-byte for a live verify run on the round's CLI.

@history_item n=14 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B's compiled pastes go only into rows holding an input-box literal and no modal. It waits up to the 120 s probe deadline, and two transcripts are left per verify. Why it matters: the verify child set and residual that a live stamp run on the round's CLI repeats.

@history_item n=15 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — Run B's sixth paste (`/compact`) has a measured three-file residual, and the stamp gate reads twenty-four rows. Why it matters: these are the residual count and stamp shape a live verify must reproduce to read `cli_verified:true`.

@history_item n=16 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E resumes only Run B's first SessionStart id when it has the session-id shape. Why it matters: rows verify-run-e-resume-target and session-id-argv-closed-shape bind the verify run on the round's CLI.

@history_item n=17 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — The `claude agents --json` child of list, revive and Run B is a direct spawn under R8 with a capped, tolerant parse that fails open. Each agents-probe pair logs no row text, id, cwd or path. Why it matters: these are the origins of agents-probe-log-redaction and child-output-bounded. A live verify writes eight spawn pairs.

@history_item n=18 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's probe dirs are removed on pass, fail, or a SIGINT/SIGTERM stop, each guard removing only what its own run created. Why it matters: verify-probe-dirs-cleanup governs a killed live verify. Every spawn has a kill deadline, and a SIGKILL leaves the dirs to be removed by hand.

@history_item n=19 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — The scratch footer reading of a newer CLI was accepted as one tmpfs-home run start with no key, keeping only scrubbed footer rows. It is no standing permission, and another reading needs its own answer. Why it matters: this is the precedent for the residual class (row scratch-footer-reading-residual). Each live start here needs its own founder word (fork 1).

@history_item n=20 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — verify's print-mode probe residual (an empty project dir under `~/.claude/projects/`) was measured on 2.1.296 and 2.1.302 and accepted into the trusted-runs class. Why it matters: the round's CLI choice (fork 3: 2.1.296 shim vs newer PATH `claude` vs the 2.1.302 fixture dir) lies inside this measured residual (rows verify-real-cli-transcripts-accepted, verify-trusted-runs-risk).

@history_item n=21 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — Two scratch measurements crossed the credential boundary, and the identity floor stays eleven names. Standing rule: the answers are spent, and another peer message or names probe needs its own showing. Why it matters: rows identity-floor-eleven-names and r8-claude-env-strip. A live round must not re-probe the child's `CLAUDE*` environment without a new word.

@history_item n=22 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — The human's terminal on `viola run`'s stdin is a vector. The closed non-editing list was widened once, by seven measured terminal replies, and no reply is ever added unmeasured. An agent-typed takeover key was ratified only for that one start, on a nested compositor. Why it matters: a rig opening its own plain pty writes to `viola run`'s stdin, and any byte it sends is typing that takes the wheel (rows human-typing-takes-wheel, stdin-closed-non-editing-list, contract/send-refusal/human-typing, contract/wheel-event/human-input). This chunk runs headless with no compositor, and the earlier answer does not carry over.

@history_item n=23 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — `target/e2e-home` links to a tmpfs backing. The keepers follow a linked base only when it is absolute and owner-only, and a test-side removal takes only test-created dirs. Trap: with the relative-target check neutralised, a keeper makes a dir in the working directory. Why it matters: the live homes are `target/e2e-home/viola-live-*` (rows e2e-home-link-guard, e2e-home-keepers-linked-base, test-removal-own-dirs-only, viola-ignores-e2e-home-backing).

@history_item n=24 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — The e2e-home keepers refuse a backing below the 9 GiB headroom floor with a fixed message naming no path. Why it matters: the scope reads `df -h /tmp` first, and a live home on a short tmpfs is refused by this floor (row e2e-home-link-guard).

@history_item n=25 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — `run` and `revive`'s start check the home tree (home, instance, ledger, bin, plugin, the pinned copy) before the wrapper log opens. verify checks the home tree before its cli log. The stamps are checked before `ledger/` is made and again under lock. Every client checks the instance before the snapshot. Trap: a check made after a log open or create narrows the path and passes a widened tree. Why it matters: these are the start-order rows a v2-03 wrapped start must show (start-checks-home-tree-before-log-opens, strict-check-before-narrowing, stamps-checked-under-lock, verify-home-tree-check, home-tree-strict-modes-before-log).

@history_item n=26 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — The pinned `viola` is placed once by `place_private_new` (mode before bytes, sync, handle closed, no-clobber) and never replaced. A lost placement re-hashes the standing copy. One tree's release builds can key two folders. Why it matters: v2-03 runs «the pinned copy through a freshly written plugin by absolute path». The rig's homes must not touch the product's `~/.viola/bin/0.1.0-…` (rows pinned-binary-placed-once, pinned-viola-never-replaced, lost-placement-rehash, pinned-copy-content-hash).

@history_item n=27 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — A start checks the socket directory before any bind, snapshot or child, makes it only when absent and checks again. `socket-dir-refused` is exit 1 at a start. A killed wrapper's leftover socket file is kept by choice. Why it matters: these are the wrapped-start order and kill semantics the live rigs meet (rows start-checks-socket-dir-before-bind, run-refuses-socket-dir-before-start, leftover-socket-file-kept, no-socket-overwrite).

@history_item n=28 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — The collision check prints and logs `stale-heartbeat` for a stale holder and `already-live` for a live holder or a fresh replayed beat. verify gained closed refusal codes (program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture). Why it matters: v2-03's «a live or stale name refused» reads these contracts (contract/collision-check/*, contract/verify-refusal-codes/*).

@history_item n=29 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — `run`'s collision check judges by pid, OS start time and heartbeat and opens no connection, so the server check is moot for it. Why it matters: this is the origin of collision-check-needs-no-server-verify, which a live name-refusal reading rests on.

@history_item n=30 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — The founder deferred v1-27 (the wrapped start) to 0.2.0. `statusline-rate-limits` was found to read the account's plan state, not the CLI (F-06i). Handle inheritance was measured live on 2.1.287 (the child's fd table held no viola socket and no home file). Why it matters: v2-03 is the 0.2.0 heir of that deferred v1-27. The live fd-table method is the precedent for reading the wrapped child.

@history_item n=31 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — viola is Linux-only. The R8 strip and the fixture scrub match names exactly. Child program resolution is a given path or a bare name on PATH (`ProgramNotFound`). Why it matters: «v2-03 on Linux». Rows run-resolves-absolute-program, claude-resolved-from-path, r8-claude-env-strip and contract/verify-refusal-codes/program-not-found. The round's CLI choice (fork 3) is decided by which `claude` PATH resolves.

@history_item n=32 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — `child-output-tolerant` and `child-output-tolerant-unknown` now read «unknown or a closed failure word». Why it matters: both rows are in the picked set and govern how the rigs' child-output reads may fail.

@history_item n=33 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — The vt100 feed runs under `catch_unwind` on a tee after the human's write, and a caught panic poisons the model to input-not-ready. Why it matters: these are rows vt100-feed-under-panic-catch, vt100-feed-isolated and vt100-inside-catch-unwind. They are the passthrough guarantee behind «the human's keys passed unchanged» on the live start.

@history_item n=34 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — The feed queue is bounded at 256 with `try_send`, and a dropped copy poisons the model. Why it matters: this is row vt100-feed-bounded-queue on the live passthrough.

@history_item n=35 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — `viola mcp`'s stdin is capped per line at `MAX_FRAME`. The bounded-reader census holds, and serde_json `unbounded_depth` is banned. The feed-panic passthrough is witnessed on the outer terminal. Why it matters: these are the caps the tap's live MCP exchange and the rigs' reads sit under (rows max-frame-cap, reads-capped-at-max-frame, bounded-external-readers, serde-json-bounded-depth).

@history_item n=36 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — release-check refuses `test-support` and `fake-agent` artifacts (probe 5/5). Why it matters: the rigs and fake rehearsals live in the test-only `viola-e2e` crate and under the fake-agent feature, and they must never reach a release build (rows release-build-no-test-binaries, release-build-without-test-features, contract/release-check-output/test-only-feature).

@history_item n=37 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — The second fake-agent test seam was added, and another seam or env read needs its own Decisions Log entry. Why it matters: a rig rehearsal under the fake agent must add no new seam without its own entry (rows test-seams-fake-agent-only, new-env-read-needs-log-entry, test-seams-compiled-out).

@history_item n=38 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** — Pre-push launches every child under `env -i` with HOME and PATH only. No uid-0 launch and no provisioning script exist. Why it matters: these are rows nothing-runs-as-root and no-uid-0-launch for any rig or operator step.

@history_item n=39 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — The artifact canary scan's classes are the twelve `claude-stripped` needles, the content canary and a non-0600 diagnostics file. The `chunk.diff` skip is by exact path. Why it matters: CI uploads of the rigs' fake rehearsals wait on this scan (rows artifact-canary-scan, ci-artifact-canary-scan, scan-gated-ci-uploads, contract/artifact-canary-scan-classes/claude-stripped).

@history_item n=40 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — Run B's probe `--settings` and the statusline capture arm were added (since retired). The kept `workspace.repo` fixture except was added (since dropped). Why it matters: this is the history behind the StatusLine fixtures and except clauses that 2026-11-14-status-line-interception-removed retired. The new box-only screen fixture must not lean on them.

@history_item n=41 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — This entry ratified the override, shell-out and source read of the user's status line. A `--home` home read no user settings. Why it matters: this is the design CARRY 3 compares against. The live reading must only READ the user's own setting and write nothing to `~/.claude/settings.json` (rows no-writes-under-claude-home, viola-writes-nothing-in-claude-dir).

@history_item n=42 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — `claude_env_keep` holds at most 32 names, each matching `^CLAUDE[A-Z0-9_]*$`. A bad value is rejected whole, and a floor name has no effect. Why it matters: this is the origin of contract/config-json/claude-env-keep and the R8 strip that every live child runs under.

@history_item n=43 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — The terminal colour facts joined the admitted env set. Any other env read needs its own entry. Why it matters: rows no-stored-secrets (except), env-not-config-channel and controls-not-switchable. A rig must not steer the product through an environment variable.
