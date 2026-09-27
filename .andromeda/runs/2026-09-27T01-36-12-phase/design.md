# design extract

## Relevance
partial. The chunk has no rendered surface (no web-spa, no tokens or motion). Design applies only to the CLI text `viola run` prints: its exit-1 start-refusal lines and hints, and its silence once the child runs.

## Constraints
- design-system §Surface: cli → Component Patterns 2 (exit-1 start refusals) requires every `viola run` start refusal to print one fixed-message line on stderr, then that cause's own `hint:` line on the next line. Wording is exact. For the causes this chunk lands:
  - already live: `unable: <name> is already live` → `hint: viola list`
  - stale heartbeat with a live pid: `unable: <name> is still running but not answering` → `hint: viola list shows it as stale; stop that process before starting <name> again`
  - pinned copy fails its SHA-256 re-hash: `unable: the pinned viola copy failed its integrity check` → `hint: the pinned copy was changed after it was written, so viola will not run it`
  - strict-modes home (only if this chunk lands the read-side check): `unable: the viola home is not private to you` → `hint: the viola home must be readable only by you; viola does not change its permissions`

  The squatted-endpoint line belongs to the channel chunk.
- design-system §Surface: cli → Component Patterns 5 (`run`) and §Exit-code phraseology (row 1) require that no start refusal shows a path or a pid. Exit 1 has no `--json` document "until arch amends it". `error: internal error` stays a fault with no hint, and its detail goes only to `diagnostics/`.
- design-system §Surface: cli → Tokens (Colour decision order, step 2) and §Per-Surface Bans (cli) require `viola run` to print nothing at all while the child runs: no colour, glyphs, cursor control or progress. The terminal belongs to the child. This covers the start sequence (pinned copy, plugin folder, snapshot, heartbeat), which must not write any human-visible progress lines.
- design-system §Surface: cli → Tokens (Streams) requires refusals and their `hint:` lines on stderr, never stdout.
- design-system §Brand Identity (Coasting track) and §Surface: cli → Component Patterns 2 set the domain meaning:
  - `stale` means lamp-off. The instance is dimmed, never deleted, and keeps its slot.
  - "Stale heartbeat + live pid" is a refusal, not a takeover.
  - The scope's "stale → taken over" rule therefore has to be read as "wrapper gone". Whether "wrapper pid still alive but heartbeat stale" is refused or taken over is a P3 premise to close against arch and security. Design mandates a refusal for it.
- design-system §Anti-Patterns → Per-Surface Bans (cli), "NEVER print upstream text, paths, pids or anyhow chains…" applies. Refusal and error text must not leak:
  - the `instances/<name>/` path
  - the `bin/<version>-<hash>/` path
  - pids or child_pid
  - hash values

## Patterns to follow
- One fixed-message line plus one hint per cause (T4), keyed by the start-refusal cause. This follows §Surface: cli → Component Patterns 2. It parallels the exit-21 per-cause hints under the same heading.
- Standard phraseology, per §Brand Identity ("Standard phraseology and 'unable' plus a reason"): short fixed words, no filler, no guessing. The liveness words are exactly `live` / `stale`, per §Surface: cli → Component Patterns 1 (LIVE column). Whatever state this chunk persists should map to these two words for later readers (`viola list`, `viola ui`).
- Fixed messages instead of stack traces, per §Surface: cli → Platform-Specific Notes ("Stack traces never print … full detail goes only to `instances/<name>/diagnostics/`").

## Anti-patterns to avoid
- Any output from `viola run` while the child runs, or any SGR, colour or glyph on a start refusal (§Per-Surface Bans cli: "NEVER emit colour, glyphs or cursor control under … `viola run`").
- Paths, pids, hashes or anyhow/serde chains in the refusal line or the hint (§Per-Surface Bans cli: "NEVER print upstream text, paths, pids…").
- Treating `stale` as an error colour or deleting the instance's state on stale detection (§Rejected Defaults, "Traffic-light status dots"; §Brand Identity, Coasting track). Takeover rewrites state. It does not erase the name's slot semantics.

## Contract bindings
- **design ↔ arch (exit codes and `--json`):** exit 1 has no `--json` document. Per-cause detail codes are pending an arch amendment (§Surface: cli → Component Patterns 2 and §Exit-code phraseology). Whether arch has since amended this is research's question.
- **design ↔ obs (D-20 detail codes):** the log detail codes `already-live` and `pinned-hash-mismatch` (and `strict-modes-failed` if it lands) pair with the design refusal lines. The stale-heartbeat cause has no obs code yet (§Surface: cli → Component Patterns 2; Design Decisions Log T4).
- **design ↔ tests (exit-cause matrix, test-plan §6):** the exit-1 causes and their fixed lines listed above are the human-output half of the matrix rows this chunk lands (already live, pinned re-hash mismatch).
- **design ↔ security (NEVER-log floor):** no paths or pids in refusals (§Per-Surface Bans cli).

## Acceptance criteria contributions
- A second `viola run <name>` against a live instance exits 1. Its stderr is exactly `unable: <name> is already live` followed by `hint: viola list`, and stdout is empty (per design-system §Surface: cli → Component Patterns 2 / 5).
- A pinned copy that fails its re-hash exits 1. Its stderr is exactly `unable: the pinned viola copy failed its integrity check` followed by `hint: the pinned copy was changed after it was written, so viola will not run it` (per design-system §Surface: cli → Component Patterns 2).
- No start-refusal stderr line contains a path separator under the viola home, a pid, a hex hash or an ANSI escape (per design-system §Per-Surface Bans (cli)).
- A successful start writes nothing to stdout or stderr from viola before or after the child spawn (per design-system §Surface: cli → Tokens, Colour decision order step 2; Component Patterns 5).

## Relevant amendment history
(none). `design-system-amendments.md` does not exist. For context, the plan's in-body Design Decisions Log has two 2026-09-24 overseer entries that govern this area:
- "Exit-1 start refusal hint": `already live` gets `hint: viola list`, with no paths or pids.
- T4: one fixed message and one hint per exit-1 cause, for already live, stale heartbeat, squatted endpoint, pinned SHA-256 mismatch, `.cmd`/`.bat` child and strict-modes home. `--json` detail codes are pending an arch amendment.

Why: the cross-plan fix pass aligned design with the test-plan exit-cause matrix and the obs D-20 codes.
