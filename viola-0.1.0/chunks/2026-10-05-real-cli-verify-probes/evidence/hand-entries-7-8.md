# Entries 7 and 8 driven by hand — the dated plan correction (2026-10-05)

**Correction (the overseer, founder-delegated, 2026-10-05; relayed by the operator):** entries 7 and 8 run by hand with
the plan's exact `run`, except `--home "$h/home"` becomes `--home "$h/vhome"`. The reason: the record home's own
`/home/` component survives the scrub, and the fixture hygiene check refuses it as an absolute path
(`SessionStart.default.json absolute-path`, `round-100046Z.txt`). The product works as designed. The cap is 18 (the
founder, live, 2026-10-05).

## Entry 6 — the round probe, by hand (2026-10-05T10:20:55Z)
- `claude --version` → exit 0, `2.1.288 (Claude Code)`: `exit 0` ✓ · `contains 2.1.288 (Claude Code)` ✓.

## Entry 7 — 2.1.288 (sessions 11-13), 2026-10-05T10:20:55Z → 10:21:06Z
- run: `h="target/e2e-home/viola-record-$(date -u +%Y%m%dT%H%M%SZ)" && cargo build -q && target/debug/viola --home
  "$h/vhome" verify --record "$h/out" -- "$HOME/.local/share/mise/installs/claude/2.1.288/claude" && mkdir -p
  fixtures/claude/2.1.288 && cp "$h"/out/2.1.288/*.json fixtures/claude/2.1.288/`
- exit 0 · atoms: `exit 0` ✓ · `contains stamped 2.1.288  10 pass  0 fail` ✓ (the W5 stamp) · artifact
  `fixtures/claude/2.1.288/` fresh (7 files written 10:21Z).
- **Recorded:**
  - four spine payloads (`cwd` and `transcript_path` scrubbed to `~` / `<user>`);
  - `Screen.modal.json`: one kept row, the trust literal at row 15;
  - `Screen.ready.json` and `Screen.turn.json`: one kept row each, the input-box footer at row 23.
- **After the run:**
  - STOP 1 clear: the modal screen holds the trust literal and no other modal literal.
  - STOP 2 clear: no `/tmp` Run A key or transcript dir.
  - The repo root's flags are unchanged.
  - No `claude` has a probe-dir cwd, and no probe dir is left.

## Entry 8 — 2.1.287 (sessions 14-16), 2026-10-05T10:21:40Z → 10:21:50Z
- run: `h="target/e2e-home/viola-record-$(date -u +%Y%m%dT%H%M%SZ)" && cargo build -q && target/debug/viola --home
  "$h/vhome" verify --record "$h/out" -- "$HOME/.local/share/mise/installs/claude/2.1.287/claude" && cp
  "$h"/out/2.1.287/Screen.*.json fixtures/claude/2.1.287/`
- exit 0 · atoms: `exit 0` ✓ · `contains stamped 2.1.287  10 pass  0 fail` ✓ · artifact `fixtures/claude/2.1.287/` fresh (the
  three `Screen.*.json`, written 10:21Z). The spine and relayed files keep their bytes (Oct 4 mtimes, untracked-only change
  in `git status`).
- **Recorded:** the same kept rows as 2.1.288:
  - the trust literal at row 15 (`modal`);
  - the input-box footer at row 23 (`ready`, `turn`).
- **After the run (2026-10-05T10:21:56Z):**
  - STOP 1 clear, and STOP 2 clear: no `/tmp` Run A key or dir.
  - The repo root's flags are unchanged.
  - `~/.claude/projects/` holds 18 dirs, the five new ones Run B transcripts (the planned residual).
  - No `claude` has a probe-dir cwd, and no probe dir is left.
- No `stop` form was needed for either entry: no survivor.

