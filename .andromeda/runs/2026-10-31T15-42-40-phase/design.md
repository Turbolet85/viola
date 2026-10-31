# design extract

## Relevance
relevant: CLI surface only (refusal phraseology, per-cause hints, exit-code forms, `--json` stream rules); no tokens, layout or frontend, the frontend being deferred to 0.2.0 per design-system §Surface: web-spa.

## Constraints
- Every exit-21 and exit-1 refusal in human mode needs its cause's own hint: per-cause `instance-unreachable` hints and a fixed-message `unable:` line plus hint for start refusals, wording as listed, per design-system §Surface: cli › Component Patterns (pattern 2; `.andromeda/design-system-detail/surface.md`, cli-send). Whether each listed hint is built is research's question; the unwrapped-name hint is carried as not built by the chunk scope.
- The plan's hint list names four exit-21 causes (not running, unwrapped, strict modes, server verification) and holds no hint wording for the scope's fifth, endpoint lost mid-call, per design-system §Surface: cli › Component Patterns. If that cause is built as distinct, its wording is new phraseology and must enter the plan; it cannot be improvised in `src/human/hints.rs` alone.
- The plan's exit-1 row names causes the scope's inferred list does not: `viola verify`'s CLI-not-found, script CLI, unreadable version, unclean fixture and stopped-before-finish refusals, each with its own pair, per design-system §Surface: cli › Exit-code phraseology (`surface.md`, cli-exit-codes). P3 closes the list against the code; each joins the catalogue or is named out by a reason.
- Under `--json` the cause travels as a code, never as prose: one stdout document, stderr empty, no hint, per design-system §Surface: cli › Streams. A table case therefore cannot read the document and a stderr `hint:` line from one invocation; the scope's single-case assertion needs one human run and one `--json` run.
- The plan's stated direction is a per-cause `detail` code, pending an arch amendment that also fixes the document as `detail: null` until then, per design-system §Surface: cli › Component Patterns (`surface.md`, cli-send). The scope leaves `detail` versus `error` to P4; P4 should weigh that this plan already leans to `detail`, and either choice amends the plan's exit-21 row, per design-system §Surface: cli › Exit-code phraseology.
- Faults take no hint: `error: internal error` prints a fixed message with no hint line, per design-system §Surface: cli › Component Patterns (`surface.md`, cli-send). This collides with the scope's "each case asserts a last stderr `hint:` line" for the internal-error row; P4 must state that row's exception or move the plan, not add a hint silently.
- One cause, one code, and the plan holds pairs that do not map one to one: the strict-modes refusal prints a home-state or an instance-state pair, and `viola revive`'s no-logged-session refusal prints two different pairs, per design-system §Surface: cli › Component Patterns (`surface.md`, cli-send, cli-list). P4 decides whether each pair is its own code, because "each code matches its hint" cannot hold otherwise; the same pair recurring across verbs is one cause, not a shared hint, for the table's shared-hint failure.

## Patterns to follow
- The fixed-message `unable: <text>` then `hint: <text>` pair for start and strict-modes refusals, not the `[/ ]` mirror, per design-system §Surface: cli › Component Patterns (pattern 2).
- The two-space `unable  <name>  instance-unreachable` line with the cause's hint last on stderr, modelled on the hint cause `server-verify` that the plan records beside `not-running`, plain ASCII with no SGR, per design-system §Surface: cli › Component Patterns (`surface.md`, cli-send).
- A hint table keyed by reason, or by reason and detail, so each key has one line and adding a cause adds a row, per design-system §Surface: cli › Component Patterns (`surface.md`, cli-send).
- The exit-1 `--json` document shape the plan gives `viola revive`, extended to the refusals `run`'s start makes after a passed preflight, which the plan marks as owed to this entry, per design-system §Surface: cli › Component Patterns (pattern 5; `surface.md`, cli-verify-run).
- New codes as short fixed lower-case words, the plan's phraseology and its code-word form, per design-system §Brand Identity and design-system §Surface: cli › Tokens (`surface.md`, cli-output).

## Anti-patterns to avoid
- No upstream text, path, pid, or anyhow chain with serde sources in a code, hint or error, per design-system §Anti-Patterns › Per-Surface Bans › cli.
- No hint names `viola release`, since a driver's `release` is refused, per design-system §Surface: cli › Component Patterns (pattern 2).
- No colour or glyph on any refusal, hint or `--json` output: amber and dim stay with `DIALOG` and `stale`, and stack traces never print, per design-system §Anti-Patterns › Per-Surface Bans › cli.

## Contract bindings
- design-system ↔ architecture §Standard Contracts: the exit-21 and exit-1 document shapes are arch's; the plan's per-cause `detail` waits on that amendment (design-system §Surface: cli › Component Patterns).
- design-system ↔ obs-plan D-20: the plan cites D-20's log detail codes and records stale-heartbeat and unwrapped-name as having no code yet, and the `viola revive` strict-modes refusal as carrying no logged code; the `--json`, MCP and log words must agree (`surface.md`, cli-send).
- design-system ↔ security plan: hints and codes sit under its NEVER-log floor (design-system §Anti-Patterns › Per-Surface Bans › cli); whether a distinct code widens a boundary is the scope's P3 read and a founder dialog at P4.
- design-system ↔ clients and tests: `reply_of` and `of_reply` screen a peer's `detail` through `viola_core::is_code_word`, so a string that is no code word reads as null (`surface.md`, cli-output); `tests/contract_error_bodies.rs` and `tests/cli_error_bodies.rs` hold the census, and test-plan §6's Exit-cause matrix holds `tests/cross_exit_causes.rs`.
- design-system ↔ `viola mcp` and `viola hook`: neither has a human design surface, `hook` never writes stderr and `mcp` writes only frames on stdout, so a refused `mcp` home check has no hint line to match (design-system §Surface: cli › Platform-Specific Notes); it joins the table or is named outside it by that reason.

## Acceptance criteria contributions
- (design) Each human-mode exit-21 and exit-1 refusal case prints its cause's plan wording with the `hint:` line directly after the refusal, plain ASCII with no SGR, holding no path and no pid; a named exception such as `error: internal error` asserts the absence of a hint (per design-system §Surface: cli › Component Patterns).
- (design) Each `--json` case prints exactly one stdout document carrying the cause's code, with stderr empty and no hint text (per design-system §Surface: cli › Streams).
- (design) No hint string in `src/human/hints.rs` or `src/human/mirror.rs` contains `viola release` (per design-system §Surface: cli › Component Patterns).
- (design) The plan's pending marks for this entry are retired through its amendment route: the per-cause `detail` line of the exit-21 row, the `revive` document owed here, and any new hint or code wording (per design-system §Surface: cli › Exit-code phraseology).
