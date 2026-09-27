# layouts extract

## Relevance
Partial. The chunk has no web-spa surface. It touches the cli surface only through what `viola run` prints: nothing during passthrough, and the exit-1 start-refusal lines for a live name and a pinned-hash mismatch. Other verbs' output (`viola list`, etc.) is out of scope.

## Constraints
- layout-templates §Surface: cli → §Output structure — `viola run` requires that a successful `viola run <name> -- claude` prints nothing: the terminal belongs to the wrapped claude TUI until it exits. None of the start-sequence steps may write a banner, progress or status line to the terminal.
- layout-templates §Surface: cli (Expression level) puts `viola run` passthrough at level 0.0: no colour, no non-ASCII glyph and no cursor control. This applies to anything the wrapper itself emits around the child.
- layout-templates §Output structure — `viola run` and §Component — Primary content block 2 (refusal lines) require the live-name refusal to use the fixed-message form `unable: <name> is already live` on stderr, exit 1, followed by `hint: viola list`, with no paths and no pids. It is not the two-space `unable  <name>  <reason>  <detail>` form used by other verbs.
- layout-templates §Component — Primary content block 2 (Hint line) and Decisions Log T4 require every exit-1 cause to have its own hint. The pinned-copy SHA-256 re-hash mismatch this chunk lands is a separate exit-1 cause and needs its own fixed-message line and hint, not the live-name pair. Neither line may name a path or a pid.
- layout-templates §Component — Primary content block 2 (Streams) requires refusals and hints on stderr, never on stdout. §Component — Footer / terminator requires the `hint:` line to be the last line on stderr, with the exit code as the terminator.
- layout-templates §Component — Header / banner (No banner anywhere) forbids an ASCII-art logo, tagline or version line on `viola run`. The version appears only in `--version` / `--help`.
- layout-templates §Component — Footer / terminator: the tape lives in `events.ndjson`, not in terminal output. The event log this chunk writes is the history carrier, so nothing is echoed to the terminal.

## Patterns to follow
- A fixed message plus a per-cause hint for `viola run` start refusals (§Component — Primary content block 2; the exact per-cause strings are in design-system cli pattern 2, which layout-templates points to).
- Streams stay split: stdout for results, stderr for refusals, hints and errors, never mixed (§Component — Primary content block 2, Streams).
- Lines are appended and never redrawn, with no spinner (§Surface: cli Motion tokens; Decisions Log, Motion trigger placement → cli).
- `hint:` lines act as breadcrumbs that name the next verb (`viola list`) and never quote upstream text (§Component — Primary navigation, Discoverability).

## Anti-patterns to avoid
- Printing a path (home dir, `instances/<name>/`, `bin/<version>-<hash>/`) or a pid in any refusal or hint line, including when the live check or re-hash check fails (§Output structure — `viola run`; §Component — Primary content block 2).
- Any start-progress output ("starting…", "pinned copy written", a version line) on the passthrough terminal (§Output structure — `viola run`; §Component — Header / banner).
- Adding a hint to `error: internal error` (the other exit-1 outcome, which is a fault and not a refusal), or printing hint text under `--json` (§Component — Primary content block 2; Decisions Log T4).

## Contract bindings
- layouts ↔ design: layout-templates §Component — Primary content block 2 defers the exact per-cause exit-1 wording to design-system cli pattern 2. That pattern has one hint per cause for:
  - already live
  - stale heartbeat with a live pid (`still running but not answering`)
  - squatted endpoint (channel chunk)
  - pinned SHA-256 mismatch
  - `.cmd` / `.bat` child
  - strict-modes home

  Which of these this chunk emits is research's question. Already-live and pinned-mismatch look in scope. Stale-with-a-live-pid needs a decision: the scope only says a stale instance is "taken over", while design treats stale-with-a-live-pid as a refusal.
- layouts ↔ tests: test-plan §6 exit-cause matrix requires a cause-specific hint per exit-1 cause (Decisions Log T4). The re-hash refusal test and the live-name refusal test should assert the last stderr line.
- layouts ↔ arch/obs: under `--json`, exit 1 has no document until arch amends it, and the detail codes (`already-live`, `pinned-hash-mismatch`) are obs-plan D-20 codes (Decisions Log T4). layout-templates adds no `--json` shape.

## Acceptance criteria contributions
- A second `viola run <name> -- claude` of a live name exits 1 with exactly `unable: <name> is already live` then `hint: viola list` on stderr. The hint is the last stderr line, stdout is empty, and neither line contains a path or pid (per layout-templates §Output structure — `viola run`).
- A pinned-copy re-hash mismatch exits 1 with its own fixed-message line and its own cause-specific `hint:` as the last stderr line, distinct from the live-name pair, with no path or pid (per layout-templates §Component — Primary content block 2, Hint line / Decisions Log T4).
- A successful start writes no bytes of its own to the terminal before or around the child spawn: no banner, version, progress or SGR/cursor sequences from the wrapper (per layout-templates §Output structure — `viola run` / §Surface: cli Expression level 0.0).

## Relevant amendment history
(none) — `D:/dev/projects/viola/.andromeda/layout-templates-amendments.md` does not exist. For context, the plan's own Decisions Log entry T4 (overseer fix pass 2026-09-24) gave each exit-1 `run` cause its own hint, because test-plan's exit-cause matrix requires cause-specific hints.
