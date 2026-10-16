
## 2026-10-14-resume-and-compact-ledger-rows — Run E's resources and a killed PTY child's exit status
**Section:** §Occupied Resources → Binary, subcommands and exit codes (test-only binaries), Claude Code integration names, Filesystem (`diagnostics/`, `ledger/probes/<pid>/`, interactive probe dirs), Repository
**Change:**
- **Integration names:** verify has five children.
  - Run E's argv: the run args, `--plugin-dir` with the `resume/` plugin, then `--resume <id>` from `resume_args`, its second caller beside `viola revive`.
  - Run E's cwd: `-resume/`.
- **`diagnostics/`: seven spawn pairs** (was six), Run E's `verify-pty-probe` last.
- **A child killed at a Ctrl-C or TERM stop:**
  - Was: it closes with `duration_ms` and no `child_exit_status`.
  - Now: a child of an interactive PTY run killed at the stop closes with `duration_ms` and a `child_exit_status`, the PTY pump's exit code, as measured by the resume-run stop case.
  - A print-mode or `--version` probe child killed at the stop is not measured.
- **Probe dirs:** the probe dir gains `resume/`, and the interactive dirs gain Run E's `-resume/`.
- **Repository:**
  - The stem `SessionStart.resume-<n>.json` (`resume_variants`) is hygiene-walked.
  - `2.1.287` gains `SessionStart.resume-1.json` (10 key names) and reads 32 files (rule `ls fixtures/claude/2.1.287 | wc -l`).
  - It is stamped at 22 / 19 by `contract_ledger_probes`.
- **The fake agent** (test-only):
  - `--resume <id>` builds the launch SessionStart on the set's `SessionStart.resume-1.json` when the set holds it (`launch_variant`, `RESUME_VARIANT`), else on `SessionStart.default`.
  - It also serves Run E under the fake agent.
  - It still has twelve options.
**Why:**
- The `child_exit_status` wording was measured false. The behaviour predates this chunk's code. It is amended to the measured scope on the overseer's founder-delegated word (`inputs#I6`).
- The resources follow Run E, ratified by the founder's own answer to P4 fork 2 (`inputs#I1`, relayed verbatim by the operator; card confirm PROVISIONAL, delegate viola-overseer).
**Ref:** .andromeda/runs/2026-10-16T14-05-30-wrap/
