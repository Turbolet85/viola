
## 2026-09-28-cli-output-tokens — the plain-output witness for clap's own output
**Section:** §5 Integration Test Strategy → CLI
**Change:**
- `tests/cli_output_plain.rs` is listed: `--help` (exit 0) and a clap usage error (exit 2) under `CLICOLOR_FORCE=1` / `CLICOLOR=1` into a pipe carry no ESC byte; `--help` under the outer PTY carries no SGR with bold `1` or underline `4` (SGR parameters parsed, never the screen text); one case self-checks the parser; 3 OSes.
**Why:** the chunk landed the witness; red with clap `color` restored and green without it (archives 161/162), PASS on ubuntu/windows/macos in ci#36404931982.
**Ref:** .andromeda/runs/2026-09-28T09-46-16-wrap/
