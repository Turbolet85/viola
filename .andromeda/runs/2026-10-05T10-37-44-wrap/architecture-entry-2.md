
## 2026-10-05-real-cli-verify-probes — registry rows for verify's interactive runs and screen fixtures
**Section:** [Plugin Scope] · §Standard Contracts → Ledger stamps envelope · §Occupied Resources (Binary · Claude Code integration names · `diagnostics/` · `ledger/stamps.json` · probe dirs · Repository) · §Infrastructure Patterns → CI/CD approach · Project directory structure · Crate dependency direction
**Change:**
- [Plugin Scope]: was "the one exception is verify's print-mode probe". Now verify's transient children are the exceptions: the print probe, Run A, and Run B, which runs the user's global hooks and status line and leaves the CLI's transcript (the accepted residual).
- Envelope and `ledger/stamps.json`: `measured` also holds `typed_probe {ready_settle_ms, turn_settle_ms, prompt_latency_ms, max_turn_gap_ms}`; `run` reads no number from it.
- `diagnostics/`: verify logs four spawn pairs (was two): `version-probe`, `verify-probe`, `verify-pty-probe` ×2.
- Filesystem: the two interactive probe dirs, 0700 and removed on every exit path: Run A's OS-temp `viola-verify-*` (`tempfile`), and Run B's `<cwd>/.viola-verify-<pid>/` (gitignored `/.viola-verify-*/`).
- Repository: `schemas/claude-screen.v1.json`. `fixtures/claude/<v>/` gains `Screen.<phase>.json` (signature rows only) and the `2.1.288` set. 2.1.287 / 2.1.288 are stamped at ten rows and 2.1.283 is drift-only. The relayed-fixture supersession is now owned by "Dialog rows and re-probe". The `--record` refusal was the fixed "a recorded payload still holds a path or a username" (also at the error-text row); now it names the file and the closed code, never the content.
- Binary: the fake agent gains `--trusted-root` / `--screens` / `--turn-stop` (argv, no env).
- Integration names: verify's two interactive children.
- Contracts: CI drives the fake agent's interactive modes too; the tree lists `verify/typed.rs`, `claude-screen.v1.json` and the screens; the root bin lists `tempfile =3.27.0` (Run A's dir); viola-agent-claude's landed API is updated (ten rows, `SIGNATURES`, `rows()`, the typed-probe helpers, compiled constants).
**Why:** the chunk landed these resources. The named refusal is the founder's live ruling at this wrap (through the overseer's AskUserQuestion, the closed code set shown). The dirs and the no-key rule are his 2026-10-05 live rulings, relayed by the overseer and ratified at this wrap.
**Kept:** the stamps-envelope example keeps `2.1.283` as an illustrative key.
**Ref:** .andromeda/runs/2026-10-05T10-37-44-wrap/
