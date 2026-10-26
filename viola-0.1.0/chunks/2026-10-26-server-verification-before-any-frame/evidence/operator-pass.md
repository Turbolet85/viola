# Operator pass — 2026-10-26-server-verification-before-any-frame

Fired by the implementer on the operator's word for this chunk (`inputs#I6`: "please run implement per the skill on
2026-10-26-server-verification-before-any-frame, through its operator pass and the CI read. …" — the operator, live,
2026-10-26). Order per `.claude/rules/ci.md` §The operator pass. Lines copied verbatim, without the tool header.

## 1. The native gate (plan entry 21, on the uncommitted tree)
`python -X utf8 {tools_dir}/gate.py run --plan viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/plan.md --entry 21`
```
 21 probe       green · exit 0 · 87.67s · 399328 B → 21.log · bash scripts/agent-run.sh pre-push
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```
The Windows-target lint (ci.md 2026-10-10) read green on the host in the implement block, run 3, entry 4; no source
file changed after it.

## 2. Hygiene (plan entry 22, fired as written)
`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` — exit 0
```
hygiene: clean — read 459 (runs 447 · evidence 4 · inputs 8) · trails 15 not read · copies 6 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
Re-read right before the commit (this file then held sections 1–2), exit 0:
```
hygiene: clean — read 460 (runs 447 · evidence 5 · inputs 8) · trails 15 not read · copies 6 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## 3. The pre-CI commit
`git add -A` then `git commit` — exit 0:
`4aa9def9fc508bf24e1d315e8ce5825d536b3a24` · `chore(2026-10-26-server-verification-before-any-frame): operator pre-CI commit, for the run this chunk's verdict reads`
(parent `6bdfdda8`); `git status --short` empty after it. Sections 3–5 of this file were written after the push.

## 4. The push (plan entry 23)
`python -X utf8 {tools_dir}/gate.py run --plan viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/plan.md --operator 23`
```
operator entry 23 · history tripwire: git
 23 probe       green · exit 0 · 3.49s · 92 B → 23.log · history moved: refs/remotes/origin/HEAD 6bdfdda8→4aa9def9; refs/remotes/origin/build/viola-0.1.0 6bdfdda8→4aa9def9 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```
The move is the one the entry intends (the pushed pre-CI commit); no force push.

## 5. The CI read (plan entry 24, fired as written) — red
`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2400` — exit 0
```
repo Turbolet85/viola (the push remote `origin`) · polled 3× over 64 s
4aa9def9fc50 verdict: red · checks 15/15 · first-fail +58 s lint (macos-latest) · runs ci#40295285860 in_progress/-
runs: ci#40295285860 push in_progress/-
failed 1: lint (macos-latest) (failure)
log: gh api --allow-escape-sequences repos/Turbolet85/viola/actions/jobs/121479746655/logs
running 11: oldest lint (ubuntu-latest) 69 s
run open: ci#40295285860 in_progress
```
Job 121479746655's log: `error: unused import: super::*` at `src/cmd/last.rs:85:9` and `src/cmd/pause.rs:95:9`, then
`could not compile viola (bin "viola" test) due to 2 previous errors`. The cause: the two test modules this chunk added
to `last.rs` and `pause.rs` held one case each, gated `#[cfg(not(target_os = "macos"))]`, so on macOS each module's
`use super::*` had nothing to serve. This host had no macOS target to lint.

## 6. Fix round (ci.md step 6, a new commit on the pre-CI commit, the pass repeated from step 1)
The fix: both modules gated whole (`#[cfg(test)]` stacked with `#[cfg(not(target_os = "macos"))]`), the inner gate
removed.

The host now lints the macOS target, check only: `rustup target add aarch64-apple-darwin`, then
`CARGO_TARGET_DIR=target/maccheck cargo clippy --workspace --all-targets --features fake-agent --target aarch64-apple-darwin -- -D warnings`
— exit 0 on the fix. Its known-positive control, `last.rs` put back in the pushed shape: exit 101,
`src/cmd/last.rs:85:9: error: unused import: super::*` (CI's error, line for line); restored, re-read (one module gate).
Host clippy (Linux, and the Windows target under `target/wincheck`): exit 0 each.

1. The native gate (plan entry 21) on the uncommitted fix:
```
 21 probe       green · exit 0 · 88.17s · 399328 B → 21.log · bash scripts/agent-run.sh pre-push
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```
2. Hygiene (plan entry 22), exit 0:
```
hygiene: clean — read 2 (runs 1 · evidence 1 · inputs 0) · trails 0 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
Steps 3–5 of the fix round follow below, written after its push.
