# Curation — 2026-09-29-h2-conpty-resize-probe

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "A plan gate that calls a harness selector not yet built (a clap usage exit 2) is a plan defect, not a red to fold: record it and pin it on the route entry that builds the selector, with that entry's chunk named as owner." (confidence 0.8)
    Proof: the operator's correction at the operator-pass invocation ("Entry 6 --e2e is a plan defect, not a red to fold …") (+0.4); measured — `bash scripts/agent-run.sh run --e2e` exits 2 `unexpected argument '--e2e'` on the tree and on a clean worktree at `90aba7c` (`chunks/2026-09-29-h2-conpty-resize-probe/evidence/entry-6-e2e-plan-defect.md`) (+0.4). Home by class: harness invocation → verification-harness.md.
  Tier 3 (.claude/docs/session-learnings.md): + "cargo-llvm-cov refuses `--no-report` with `--no-clean`" (confidence 0.8)
    Proof: measured on the dev host at cargo-llvm-cov 0.9.1 — the H2 loop's first form failed on iteration 1 with `--no-report may not be used together with --no-clean`, caught by its fail-closed branch (`evidence/h2-loop-step.md`) (+0.4); technical detail (+0.2); no other durable home this wrap — not on the route, not in a master, not a playbook/drift-base rule, not a matrix note (+0.2).
                                              + "Pin `TMP`/`TEMP` in a Windows-runner `shell: bash` step that must find what a test wrote there" (confidence 0.8)
    Proof: measured on the windows-2025 runner — ci#36527891850 job 109274838484 printed all 13 kept reports from the pinned `$RUNNER_TEMP/h2-tmp/viola-pty-watch/` (`evidence/h2-reproduction.md`), and the host dry run showed the redirect reaching the test (+0.4); technical detail (+0.2); no other durable home (+0.2).
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred (cap) · 3 rejected at the 0.6 threshold:
    - "A test that writes a key after a ConPTY resize waits for the child to observe the new size first" — 0.6 (measured +0.4, technical +0.2); amended into test-plan §5 by this wrap's P2, so neither conditional signal applies.
    - "A harness self-test's nested cargo over a throwaway crate strips cargo-llvm-cov's four names" — 0.6 (measured +0.4, technical +0.2); amended into test-plan §10 by this wrap's P2. (An additive facet of testing.md's 2026-09-24 throwaway-crate entry; the chain rejected it at Filter 4.)
    - "A `.profraw` file name never identifies its writer (cargo-llvm-cov names every process `<workspace>-%p-%m`)" — 0.6 (measured +0.4, technical +0.2); carried by test-plan §10's amended bullet ("the job log names no writer").
  Recurrence (→ handoff): `recurrence-despite-learning: host-win32.md Transports — "Documents: the Write tool … a script to a scratchpad file run by path"` — a classifier script was sent as a `cat > file` heredoc and the Bash guard blocked it; rewritten through the Write tool.
  CLAUDE.md size: 124/200 · T1 1.8 KB, 0 over 600 B
