# Curation — 2026-09-25-pty-wrapper-on-windows (wrap 2026-09-25T17-43-18)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "A claim that reaches a wrap only through a relayed direction, with no artifact on disk behind it, is carried as a labelled HYPOTHESIS on the route, never written into a spec master as fact." (confidence 0.8)
    Proof: overseer direction 4 named "ConPTY swallows focus reports"; `grep -rn -E 'x1b\[I|focus'` over `.andromeda/runs/2026-09-25T14-10-08-implement/` and `chunks/2026-09-25-pty-wrapper-on-windows/evidence/` → 0 hits; the overseer's E3 answer: "My direction relayed an implement-report claim without an artifact behind it; your grep is the measurement." (+0.4 operator correction, +0.4 measurement)
  Tier 2 (.claude/rules/*):
    + verification-harness.md: "On the Windows host, a diff that carries `#[cfg(unix)]` bodies cannot pass plain `run --mutants` … run it as `run --mutants --leg windows-2025` and take the verdict from the CI union of both legs." (confidence 0.7)
      Proof: implement gate record `.andromeda/runs/2026-09-25T14-10-08-implement/gate-….json` recs 8, 9, 10 — entry `run --mutants` red in its plan form three times (5 `cfg(unix)` survivors in `crates/viola-pty/src/lib.rs` `HostTerminal::enter` / `host_size`), the CI form `--leg windows-2025` ok:true (186 / 151 caught / 30 unviable / 0 timeout; operator-pass.md). (+0.4 real gate failure, +0.3 three runs)
    + verification-harness.md: "Never pipe `agent-run.sh boot` … the supervisor it leaves running inherits boot's stdout, so the pipe never reaches EOF …" (confidence 0.8)
      Proof: friction `2026-09-25T17:10:29Z-a` / `-b` — the first smoke drive piped boot stdout into `cut` and outlived its 600 s tool bound; re-driven with each step written to a file. (+0.4 measured failure, +0.2 specific technical detail, +0.2 no other durable home this wrap: no master amendment, no route annotation, no playbook rule carries it)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 5 dup (the ConPTY `CTRL_CLOSE` on a dropped input writer, the swallowed Ctrl-C before raw mode, the Linux SIGHUP / macOS output-end measurement, the no-op `fake-agent` feature per crate — each amended into a master this wrap; the union-rule defect — a route CARRY) · 1 task-specific (the `secret-scan` read of `target/agent-run/chunk.diff` residue: a one-off harness-scope question, 0.1) · 0 conflict · 1 deferred (→ handoff: rust-analyzer holding `mutants.out`, 0.7 — an additive facet of host-win32.md 2026-09-24 "Stopping a Monitor/background task …")
  No-other-home: "Never pipe `agent-run.sh boot` …"
  CLAUDE.md size: 122/200 · T1 1.5 KB, 0 over 600 B
