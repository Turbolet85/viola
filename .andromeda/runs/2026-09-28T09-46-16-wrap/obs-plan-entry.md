
## 2026-09-28-cli-output-tokens — run's pre-spawn stderr names every start refusal and the one writer
**Section:** §3 Observability Harness Contract (the `run` terminal bullet); §11 Obs Anti-Patterns → Logs (the print-macro ban)
**Change:**
- `run`'s one pre-spawn exception is every start refusal (`.cmd`/`.bat` child, live or stale name, tampered pinned copy, squatted endpoint), two fixed stderr lines through `src/human.rs` `refuse` → `write_refusal`, one `write_all` on the locked stderr (was "the `.cmd`/`.bat` refusal" alone).
- §11: an output module may carry a local `#[allow]` only where it uses a print macro; `run`'s refusals go through `src/human.rs`, which uses none and carries no `#[allow]` (was "the `run` path's one such site is the pre-spawn `.cmd`/`.bat` refusal fn").
**Why:** the chunk re-sited the writer and its five callers (report Changes → Symbols); `grep -cE 'print!|println!|eprint|#\[allow' src/human.rs` → 0; both clippy forms green.
**Ref:** .andromeda/runs/2026-09-28T09-46-16-wrap/
