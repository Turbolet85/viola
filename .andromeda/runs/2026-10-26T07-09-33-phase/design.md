# design extract

## Relevance
partial — the chunk has no page and no new visual; its one design surface is the human and `--json` output of a CLI verb refused because server verification failed (exit 21). `viola hook` and `viola mcp` have no human design surface.

## Constraints
- A failed or unmakeable server verification on a CLI verb (`send`, `wait`, `last`, `answer`, `pause`, `release`) is the shared `instance-unreachable` refusal, exit 21: stderr `unable  instance-unreachable` with no detail, then that cause's own hint line directly after it (per design-system §Surface: cli › Exit-code phraseology; detail `.andromeda/design-system-detail/surface.md` cli-exit-codes, row 21).
- The server-verification cause has its own fixed hint, one of the four `instance-unreachable` causes (T4): `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`. It is distinct from the not-running, unwrapped and strict-modes hints. If the chunk takes the strict-modes half (the scope's boundary question), that cause uses the strict-modes hint, not this one (per design-system §Surface: cli › Component Patterns, pattern 2 `viola send`; detail `surface.md` cli-send, `instance-unreachable` bullet). Whether each verb already maps the cause to this hint is research's question.
- Under `--json` the refusal is one stdout document `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty and no hint line. A per-cause `detail` code (obs D-20's `server-verify-failed`) does not appear in `--json` until an arch amendment names it (per design-system §Surface: cli › Streams; detail `surface.md` cli-send, the "Pending an arch amendment" bullet).
- Refusals and hints go to stderr and results to stdout. Under `--json` the whole answer is the stdout document (per design-system §Surface: cli › Streams).
- `viola hook` writes nothing to stderr and always exits 0, and `viola mcp` writes only MCP frames on stdout. Neither gets a human refusal line, a hint or SGR for a verification failure: the hook fails open silently, and the MCP tool's `instance-unreachable` result is not human output (per design-system §Surface: cli › Platform-Specific Notes; §Surface: cli › Tokens, colour decision order item 3).
- The refusal line and its hint are plain: only `DIALOG` (amber) and `stale` rows (dim) are ever coloured (per design-system §Anti-Patterns › Per-Surface Bans › cli).

## Patterns to follow
- The two-space phraseology refusal form `unable  <reason>`, then one `hint:` line per cause, as the existing `instance-unreachable` causes print it. Reuse the existing per-cause hint selection; do not add a new reason word or exit code (per design-system §Surface: cli › Component Patterns, pattern 2; §Brand Identity, "Standard phraseology and 'unable' plus a reason").
- The fixed-message rule for every refusal: short fixed words, nothing guessed, faults kept apart from refusals (`error: wrapper fault` / `error: internal error` take no hint) (per design-system §Surface: cli › Component Patterns, pattern 2; detail `surface.md` cli-send, the last bullet).
- The verbs' existing `--json` error document, the same shape for every exit-21 cause, so a driver cannot tell a verification failure from any other unreachable cause in `--json` (per design-system §Surface: cli › Exit-code phraseology).

## Anti-patterns to avoid
- Never print the observed or recorded pid, the start time, the endpoint path, the socket or pipe name, or an anyhow chain in the refusal or its hint (per design-system §Anti-Patterns › Per-Surface Bans › cli, "NEVER print upstream text, paths, pids …").
- Never print a stack trace or raw OS error for a failed peer-credential or start-time read. Full detail goes only to `diagnostics/` (per design-system §Anti-Patterns › Per-Surface Bans › cli, "NEVER print stack traces"; §Surface: cli › Platform-Specific Notes, "Stack traces").
- No colour, glyph or cursor control on the refusal, and none at all under `--json`, a pipe, `NO_COLOR`, `TERM=dumb`, `hook` or `mcp` (per design-system §Anti-Patterns › Per-Surface Bans › cli).

## Contract bindings
- design ↔ arch §Standard Contracts / §Conventions: exit 21 and its `--json` document `{"v":1,"error":"instance-unreachable","detail":null}` are arch's. The design side keeps `detail` null until arch names the per-cause codes (detail `surface.md` cli-send, "Pending an arch amendment").
- design ↔ obs D-20: `server-verify-failed` is the log detail code for this cause. It goes in the obs line only, never in the human hint text or `--json` (detail `surface.md` cli-send).
- design ↔ security NEVER-log floor / §Error Handling: a hint with no pid or path, fixed messages, and MCP `isError` text `refused: instance-unreachable` (security owns the MCP side; design gives `mcp` no human surface).
- design ↔ tests: the stderr pair and the stdout `--json` document are what the exit-21 assertions read.

## Acceptance criteria contributions
- (design) A CLI verb refused on a server-verification mismatch exits 21 and prints on stderr exactly `unable  instance-unreachable`, then `hint: the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`, with stdout empty (per design-system §Surface: cli › Component Patterns, pattern 2; §Surface: cli › Exit-code phraseology).
- (design) The same refusal under `--json` prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout, stderr empty, no hint (per design-system §Surface: cli › Streams).
- (design) No refusal or hint byte holds a pid, a start time, a path or an endpoint name, and none carries an SGR byte (per design-system §Anti-Patterns › Per-Surface Bans › cli).
- (design) A `viola hook` that fails verification writes zero bytes to stdout and stderr and exits 0 (per design-system §Surface: cli › Platform-Specific Notes).
