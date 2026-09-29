
## 2026-09-29-sideloaded-conpty — D-36 sideloaded ConPTY telemetry
**Section:** §4 Scenario "`viola run` start sequence to child spawn" (must-trace chain, span attributes, log fields); §6 Additive field catalog (`process-start`); §12 D-36
**Change:**
- The chain was `run.pin_copy` › `run.version_gate`; now `run.pin_copy` › `run.conpty_sideload` (Windows) › `run.version_gate`, the span carrying `outcome` (`loaded|hash-mismatch|unreadable|load-failed|not-built`) and `search_restricted`; never a refusal, never a path or hash.
- `pty_backend` was `conpty|openpty`; now `conpty-sideload|conpty|openpty`, from `viola_pty::pty_backend()` (replacing the const `PTY_BACKEND`).
- `process-start{claude-child}` gains the additive `sideload_fallback` (`hash-mismatch|unreadable|load-failed|not-built`), present only on a Windows degrade, closed in `diag-line.v1.json`; a degrade adds no `process-exit{exit_code:1}` and no panic.
**Why:** the backends must be told apart and a degrade recorded without a terminal byte; a closed value needs a Log entry under §8 default-deny (D-35 the template). §1 keeps its verbatim wording (playbook "Verbatim scope copy").
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/
