# Gate 17 — surfaced plan-entry premise gap (implement run 2026-09-28T09-07-16)

**Entry:** `bash scripts/agent-run.sh run --mutants --file src/cmd/run.rs` · expect `exit 0` + `"verdict":"scoped"`.
**Reading:** red, exit 1. `"verdict":"scoped"` held; 5 tested, 4 caught, 1 missed:
`src/cmd/run.rs:373:5: replace refuse_stale with ()` (archive 169).

**Cause (by construction, not a product defect):** the chunk's diff touches the `refuse_stale` call line, so the scoped
run regenerates that mutant. Its only killer, `tests/cli_instance_state.rs::run_refuses_a_stale_name`, is
`#[cfg(unix)]`, because it freezes the wrapper with SIGSTOP. A Windows-host scoped run cannot kill it. That is the class
in verification-harness.md Session Addition 2026-09-25: take the verdict from the union of both legs.

**The union (gate 24, `pre-push`, same run):** `ok:true`, `stage:"union"`, gate `breaches: []`.
- ubuntu-latest leg: 7 tested, 6 caught; caught `refuse_stale`, missed `refuse_batch_script`
  (its killer drives a `.cmd`/`.bat` refusal, Windows only).
- windows-2025 leg: 7 tested, 6 caught; caught `refuse_batch_script`, missed `refuse_stale`.

Each mutant is killed on the leg whose test compiles and reaches it, so neither survives the union.

**Operator word (AskUserQuestion, 2026-09-28):** "Surface; union is verdict". Note: "overseer: agreed. The mutant is
killed in the union, which is the chunk mutation verdict since chunk 7 and is not a survivor. Record gate 17 as a
plan-entry premise gap, have the wrap reconcile the entry, and confirm the union in CI."

**For the wrap:** reconcile entry 17's `expect` (a host-only scoped run over a function whose killer is OS-gated). CI's
mutation union on the pushed HEAD is the confirming reading.
