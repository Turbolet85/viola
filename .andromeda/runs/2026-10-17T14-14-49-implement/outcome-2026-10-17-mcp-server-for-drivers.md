# /implement outcome — 2026-10-17-mcp-server-for-drivers (re-entry at step 15)

**GREEN + LIVE LEG.** Re-entry at plan step 15 only, on the founder's own word «Ещё один старт» (live in the
overseer's dialog, 2026-10-17T14:14Z; `inputs#I4`, the chunk's cap on live `claude` 2.1.287 starts raised from 1 to 2).
The earlier run (`.andromeda/runs/2026-10-17T11-33-11-implement/outcome-2026-10-17-mcp-server-for-drivers.md`)
holds the build, the gates, the deviations, the operator pass and the surfaced `pin_exe` race; this run adds only the
second live start and `v1-38`'s ref.

- **The second start (14:16Z, ledgered 14:15:52Z):** on the fixed build (`target/debug/viola` of commit `a902e6e4`'s
  tree, sha256 prefix `fef229f67d76230e`) the wrapped driver session listed the five tools (no `tools/list` failure in
  the CLI's MCP log), called `list` (one `agents-probe` pair in `mcp.ndjson`, exit 0) and `wait` for `builder` with no
  `timeout_ms`: one `wait` frame with `timeout_ms` 50000, answered `timed-out` after 50 012 ms, on both sides' logs;
  the turn ended with the fixed word; no permission dialog. Every step 15 (d) reading holds
  (`chunks/2026-10-17-mcp-server-for-drivers/evidence/live-mcp.md`, rewritten whole for both starts;
  `evidence/live-sessions.ndjson`, 4 rows). Live starts: 2 of the cap of 2.
- **`v1-38`:** `planned` → `implemented`, its ref the `mcp_tools` E2E cases, the `viola-mcp` unit and handler cases
  and the live record (`matrix.py implement`, trail in this run dir). Its acceptance now holds as written: the CI
  half on `a902e6e4` (`ci#39033242394`, green 15/15), the live half by start 2.
- **No code changed, no gate re-run, no push.** The files this run changed are records only: the chunk's
  `inputs/` (`I4`), `evidence/live-mcp.md`, `evidence/live-sessions.ndjson`, `viola-0.1.0/verification-matrix.json`,
  this run dir and the friction log. No test reads any of them: the two test files that name one (`cli_answer.rs`,
  `mcp_tools.rs`) name it in a comment only, and the four walk-class tests read `fixtures/`, `src/` and `crates/`.
  The pushed `a902e6e4` stays the verdict's commit; the wrap commits these records.

## Process hygiene

Measured 2026-10-17T14:17:32Z against `/proc` (every process whose executable or working directory lies under the
repository, this leg's live home or its session dirs):

| process | started by | final state |
|---|---|---|
| `builder` wrapper + fake agent | this run (the rig) | terminated — stop file, 1 Ctrl-C, exit 0, not killed |
| `driver` wrapper + `claude` 2.1.287 + its `viola mcp` | this run (the rig) | terminated — stop file, 3 Ctrl-C, exit 0, not killed |

None left running.

## Decisions & corrections

- The founder's own word «Ещё один старт» (`inputs#I4`): the cap rises to 2; one more start made, on the fixed
  build, ledgered first; it read green, so nothing was re-fired.
- The operator's direction for this re-entry: re-read the run dir and `live-mcp.md` from disk first (done); on green
  record `v1-38`'s ref (done); no push unless a file a gate reads changes (none did); then stop ready for the wrap.
