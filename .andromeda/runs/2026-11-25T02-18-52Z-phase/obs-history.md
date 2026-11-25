@history_item n=1 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (4. Span / Trace Coverage: 4 row(s)) — changed ~ contract/run-start-spans/span-chain (run.window follows the trust step under --origin, before state.snapshot_write) and added + contract/run-span-attributes/run-window (run.window carries `outcome` alone, never an address, pid or workspace), + contract/cli-log-file/open and + decision/window-line-untraced. Why it matters: this is the window step this chunk changes (`src/run/window.rs`). A move form that keeps the active workspace has to fit this span's closed field set and the span's place in the chain. Any new move outcome word needs a row change here; it cannot be added as a new field.

@history_item n=2 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (1. Obs Scope Summary: 2 row(s)) — added + contract/run-start-events/window-line (one window line {address, workspace, placement, origin}, source wrapper, after launch under --origin) and widened ~ contract/process-log/child-and-probe-lifecycle to the terminal open spawns. Why it matters: the window line's `workspace` and `placement` are what a placed window records. Not moving the view must leave this line's shape alone unless the plan amends the row, and v2-07's active-workspace predicate is not a window-line field.

@history_item n=3 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (6. Log Coverage: 2 row(s)) — widened ~ contract/process-exit-detail/exit-1 with open's terminal-not-found and not-live and added + contract/diag-subject/terminal. Why it matters: `viola open` is the first caller of the move. Its exit-1 words and its terminal spawn subject are the closed vocabulary a placing `open` logs, and a move-form change adds no word without a row.

@history_item n=4 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (2. Telemetry Strategy: 1 row(s)) — ~ contract/strict-modes-refusal-exit/1 gained `open` (exit 1, and the strict-modes-failed document under --json). Why it matters: open's refusal shape stands for the open caller. The chunk's witness for a placing open runs past this check.

@history_item n=5 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (6. Log Coverage: 5 row(s)) — added + contract/diag-subject/hyprctl-probe and + contract/parse-rejected/hyprctl-json, and widened the process-start subject, parse-rejected parser and oversize detail rows for the seam. Why it matters: every hyprctl spawn, the new move form's dispatch included, logs the hyprctl-probe pair through the seam. A reply the seam refuses gives one parse-rejected hyprctl-json line (oversize past MAX_FRAME, or malformed). A new dispatch form or reply in the tap or fake must keep to this closed vocabulary.

@history_item n=6 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Obs Scope Summary: 3 row(s)) — ~ decision/per-parser-parse-failure-counts (the hyprctl replies count their own failures), ~ decision/instrument-call-sites-not-os-internals (the hyprctl spawn is instrumented at viola's call site) and ~ decision/parse-failures-surface-as-unknown (except: a hyprctl reply failing its parse reads oversize or malformed, never unknown). Why it matters: these are the rules the move's new form is instrumented under. Instrumentation sits at the seam's call site only, never inside the compositor, and a malformed reply gets a closed word.

@history_item n=7 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. Obs Scope Summary: 2 row(s)) — ~ decision/os-apis-at-call-sites gained the hyprctl spawn, and ~ decision/parse-failures-unknown-counter gained the hyprctl exception. Why it matters: this is the twin pair of item 6, a picked row in the scope summary twin. Both twins move together if the move form changes what the seam parses.

@history_item n=8 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (2. Telemetry Strategy: 1 row(s)) — ~ decision/span-naming added `hyprland` to the closed span-area list (the seam's manual skip_all span `hyprland.read`). Why it matters: any span the move form adds in the seam has to be a static `hyprland.<operation>` name. P3 should name it if the form adds a dispatch span beside `hyprland.read`.

@history_item n=9 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** (1. Obs Scope Summary: 1 row(s)) (amd-rppgazi6 · decision/parse-failures-unknown-counter; amd-lqfol5pf · decision/parse-failures-surface-as-unknown) — each except was reworded to its current two-clause text: the hyprctl reply's oversize or malformed, and the StopFailure word. Why it matters: these are the exact standing exceptions of the picked unknown-counter rows. A seam reply-set change for the move must keep the hyprctl clause true.

@history_item n=10 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (6. Log Coverage: 4 row(s)) — ~ contract/diag-subject/terminal (a terminal that open or up spawns gets a process-start alone, with no process-exit: the claimed pair was disproved by measurement), ~ contract/diag-detail/strict-modes-failed (up's exit-1 is a writer), ~ decision/symlink-skip-logged-as-count (except up), and + contract/process-exit-detail/boot-time-unread. Why it matters: `viola up` is the second caller of the move. This is a trap: a claimed log pair was disproved live, so the chunk's per-caller lines are measured, not assumed.

@history_item n=11 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (1. Obs Scope Summary: 1 row(s)) — ~ contract/process-log/child-and-probe-lifecycle: the terminal that open or up spawns gets a process-start alone. Why it matters: this is the picked lifecycle row as it stands for both terminal-spawning callers of the move.

@history_item n=12 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (1. Obs Scope Summary: 2 row(s)) — ~ decision/strict-modes-failure-lines and ~ contract/process-exit-detail/strict-modes-failed gained up's process-exit for a home tree refused after its board scan. Why it matters: these are picked rows that name up's one logged strict-modes line. The up caller's path to its spawns and placement passes this check.

@history_item n=13 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (2. Telemetry Strategy: 2 row(s)) — ~ contract/strict-modes-refusal-exit/21 and ~ contract/strict-modes-refusal-exit/1 gained up's board scan (21) and home-tree check (1). Why it matters: these are up's refusal exits ahead of any placement.

@history_item n=14 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (3. Observability Harness Contract: 1 row(s)) — ~ decision/strict-modes-before-diagnostics except: up's home-tree check runs after its board scan has opened its log and ends that log with strict-modes-failed. Why it matters: this is a picked row's named exception for the up caller.

@history_item n=15 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (4. Span / Trace Coverage: 1 row(s)) — + contract/cli-log-file/up (one process-start{subject:"terminal"} per spawn inside the cli.up dispatch span). Why it matters: this is where up's per-spawn lines land. A per-caller difference in the move form shows here, or nowhere.

@history_item n=16 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (6. Log Coverage: 1 row(s)) — + contract/process-exit-detail/exit-1-up: up's exit 1 follows its per-instance outcomes (not-live, and others). Why it matters: up's exit vocabulary is closed. A failed move under up must map onto it or amend it.

@history_item n=17 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (4. Span / Trace Coverage: 1 row(s)) — + contract/cli-log-file/focus (manual span cli.focus into the cli role file). Why it matters: viola focus is the sibling Hyprland dispatch that deliberately moves focus, and a precedent for how a seam dispatch verb is logged. The chunk's «no focus change» move has to stay distinct from it.

@history_item n=18 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (6. Log Coverage: 1 row(s)) (amd-gqypaosh, then set by amd-mv3ctbds) — + then ~ contract/process-exit-detail/exit-1-focus: window-not-found, unsupported (no hyprctl reachable), unreachable (hyprctl reachable, no compositor answering), and the seam's oversize, malformed or refused, on the operator's F-E answer keeping the seam as built. Why it matters: these are the seam's settled outcome words for a dispatch. The fake's reply set and the move's refusals reuse these meanings, never new ones.

@history_item n=19 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (4. Span / Trace Coverage: 3 row(s) (2 of 2)) — ~ decision/window-line-untraced: a switch writes the window line again after its own launch line, plus + contract/cli-log-file/switch. Why it matters: the window line has a second writer. The plan must decide whether a switch re-places the window (and so takes the move's form) or only re-records it.

@history_item n=20 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (4. Span / Trace Coverage: 4 row(s)) — + decision/adopt-start-logs-as-run (no event, detail, field, subject or span of its own) and widened home-tree-check-before-start-log, launch-line-untraced and lineage-line-untraced to adopt. Why it matters: adopt's start runs run's start, run.window included under --origin. A move-form change reaches adopt too and must add nothing adopt-specific.

@history_item n=21 marker=2026-11-24-adopt-s-resume-folder
**2026-11-24-adopt-s-resume-folder** (4. Span / Trace Coverage: 1 row(s)) — ~ decision/adopt-start-logs-as-run except: the not-resumed detail on adopt's own self process-exit. Why it matters: this is the picked row's standing exception, the latest change before this chunk's take-up at `38d9b7e8`.

@history_item n=22 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (revive in the log: three more exit-1 details on role `run`, `refuse`'s second caller, a sixth panic exemption for `--list`) — revive's start arm is role run, logged into run-<name>.ndjson. Why it matters: `viola revive --origin` is the third caller of the move. Its window step logs as run (contract/cli-log-file/revive-start, decision/revive-start-logs-as-run).

@history_item n=23 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (4. Span / Trace Coverage: 2 row(s)) — + decision/launch-line-untraced and the launch step in decision/product-order-from-events-log. Why it matters: the launch line is the precedent the window line copies (log-only, no span, no process-log line). The window line follows launch in the start's product order.

@history_item n=24 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (1. Obs Scope Summary: 2 row(s)) — ~ contract/run-start-events/order (wheel, lineage, launch before session-start). Why it matters: this is the picked order row the window line sits beside. A placement change must not reorder the start's product lines.

@history_item n=25 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (4. Span / Trace Coverage: 2 row(s)) — the lineage step in ~ decision/product-order-from-events-log, and + decision/lineage-line-untraced. Why it matters: this is the history of the picked product-order row, which is read from that start's own events lines and never duplicated into process logs.

@history_item n=26 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (4. Span / Trace Coverage: 6 row(s) (1 of 2)) — ~ contract/run-start-spans/span-chain dropped run.budget_eval. Why it matters: this is the prior edit of the start span chain that run.window now extends. The chain is a closed sequence, amended whole when a step moves.

@history_item n=27 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (6. Log Coverage: 2 row(s)) — + contract/diag-subject/auto-verify (the start's verify child pair, with child_exit_status and duration_ms, in the run log) and the auto-verify subject in process-start. Why it matters: the folded item asks where the automatic verify runs and what it stamps. Its only obs record is this pair in run-<name>.ndjson.

@history_item n=28 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (4. Span / Trace Coverage: 1 row(s)) — + contract/run-process-log/auto-verify-stop: a stop during the automatic verify TERMs, then KILLs, the verify child, writes the auto-verify process-exit and re-raises. The start writes no process-exit of its own, no stamp, no snapshot and no session. Why it matters: this is the standing no-stamp rule on one self-caused failure (a stop). The fold's fix for the other self-caused failures (untrusted folder, modal) extends this precedent.

@history_item n=29 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (3. Observability Harness Contract: 1 row(s)) — contract/config-json/known-keys gained `verify_new_cli`. Why it matters: this key is what turns the automatic verify on. Moving where it runs changes no config key unless the fork's form says so.

@history_item n=30 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (3. Observability Harness Contract: 1 row(s)) — ~ contract/run-terminal-output/child-output: before the child, a keyed start's automatic-verify lines and the verify child's own output reach the terminal (none under revive --json). Why it matters: this is the picked row bounding what the automatic verify may print. Relocating it (another cwd or a temp folder) must keep this terminal rule.

@history_item n=31 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (1. Obs Scope Summary: 2 row(s)) — ~ decision/claude-internals-not-a-contract (verify writes and removes its own temp folder's trust record) and ~ decision/refusal-telemetry (verify's trust-record write refuses a link, a non-regular or a foreign file with no line). Why it matters: a hand-run verify trusts its own temp folder. The folded defect is the automatic verify running where the start happened, in an untrusted folder, so these picked rows' exceptions bound any fix that moves it or trusts its folder.

@history_item n=32 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (4. Span / Trace Coverage: 1 row(s)) — + decision/trust-record-write-uninstrumented (verify's trust-record write and removal emit no log line and no span; the outcome rides only the stamps). Why it matters: an automatic verify placed in a trusted temp folder inherits this no-line rule.

@history_item n=33 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (1. Obs Scope Summary: 2 row(s)) (amd-v67jn6xb) — ~ decision/claude-internals-not-a-contract (a verified start sets its folder's trust flag only) and ~ decision/refusal-telemetry (the verify and start trust-record writes refuse with no refusal line). Why it matters: the start trusts its folder only once the CLI is verified, and the automatic verify runs before that, which is why it met an untrusted folder. Both picked rows' current text comes from this entry.

@history_item n=34 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (1. Obs Scope Summary: 2 row(s)) (amd-ye23emtv) — ~ decision/r8-strip-logged-as-names-only (the start reads the kept CLAUDE_CONFIG_DIR value) and ~ decision/claude-boundary-only-instrumentation (the start's one trust-record line). Why it matters: these are picked rows. Any new trust step for the automatic verify reads config the same way and adds no other boundary line.

@history_item n=35 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (6. Log Coverage: 1 row(s)) (amd-es35y2ig · + contract/start-trust-log/trust-record; amd-yjk3figz · + contract/diag-line-fields/trust-record) — each wrapped start on a verified CLI writes exactly one INFO trust-record line with one of 16 closed start words. Why it matters: a fork that trusts the auto-verify's folder through the start's trust step meets this one-line, closed-word contract.

@history_item n=36 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (4. Span / Trace Coverage: 1 row(s)) — ~ decision/trust-record-write-uninstrumented: a start's trust step opens no span and writes only its one trust-record line. Why it matters: this is the picked row the folded item's fix must respect.

@history_item n=37 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (6. Log Coverage: 1 row(s)) (amd-6pluk4bc, ratified by amd-zuilxaw5) — ~ contract/start-trust-log/trust-record names run, revive and adopt starts. Why it matters: this is the picked row's current text across every start the automatic verify can run under.

@history_item n=38 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** (4. Span / Trace Coverage: 3 row(s)) — ~ contract/verify-exit-codes/0 (every relied row counted: passed, or browser-tools-prompt read absent) and verify's step and summary lines. Why it matters: the fold names «a modal» as one of the auto-verify's own failing preconditions. The verdict and stamp counting of a modal-bearing run lives here.

@history_item n=39 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** (4. Span / Trace Coverage: 1 row(s)) — ~ decision/browser-tools-prompt-reading-uninstrumented: verify's one fixed stdout line on a met prompt is the only record that the prompt stood at start. Why it matters: this is the picked row. A modal met by the automatic verify reaches no log line, so the fold's witness reads stamps or stdout, not logs.

@history_item n=40 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (4. Span / Trace Coverage: 1 row(s)) — + decision/standing-modal-reading-uninstrumented: the readiness gate reads a standing modal with no new line, span field or detail. Why it matters: this is the same no-telemetry stance for a modal that the folded item's precondition check would meet.

@history_item n=41 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`viola verify`'s home-tree check and the catch site's narrowed reach) — `update_stamps` runs `check_stamps` before writing and again under its lock, and a refusal exits 1 through verify's own process-exit. Why it matters: verify's stamp writer is a surface the fold touches, and decision/verify-sole-stamps-writer is a picked row. Any "do not stamp on own failure" path goes through this writer and its refusal lines (contract/viola-verify-refusal/stamp-write-refused).

@history_item n=42 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`verify` writes its refusal's code) — verify's process-exit under a parsed VIOLA_NAME carries its refusal code (program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture, interrupted), and a failing row carries no code. Why it matters: this is the picked row contract/process-exit-detail/verify-refusal. A verify that declines to stamp on its own failed preconditions is either a new closed code here, by row amendment, or no code at all. A failing row is not a refusal.

@history_item n=43 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** (`detail:"interrupted"` on verify's process-exit) — a stopped verify writes `interrupted`, closes every spawn pair it opened, prints the stop pair and writes no stamp. Why it matters: this is the precedent for verify ending with no stamp on a cause it did not check rows for. The fold's no-stamp-on-own-failure form can mirror it (picked rows contract/verify-process-exit/interrupted and contract/verify-stop-stderr/unable-hint-pair).

@history_item n=44 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (strict-modes-failed on the stamps read; five perf rows) — `parse-rejected{parser:"ledger-stamps"}` gained `strict-modes-failed` in run's version gate, which reads as unverified. Why it matters: this is the history of the picked ledger-stamps and unreadable-ledger rows. It is the gate through which a later start reads the stamp an automatic verify leaves.

@history_item n=45 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (verify-probe joins the spawn subjects; boot readiness checks the start records) — in CI, verify runs only against the fake agent, and the real-claude verify runs only through the local live run, refused under CI. Why it matters: these are the picked rows decision/ci-verify-fake-agent-only and decision/real-cli-verify-never-in-ci. The fold's witness against the automatic verify's place must run on the fake agent in CI, and no real claude runs in this phase.

@history_item n=46 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (2. Telemetry Strategy: 1 row(s)) (amd-cvulnlnx) — ~ contract/socket-dir-refused-exit/codes: exit 1 at the start of run, revive and adopt. Why it matters: this is the picked row's current text for every start that reaches the window step. A refusal there ends before any placement.
