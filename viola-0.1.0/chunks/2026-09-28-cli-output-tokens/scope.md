# Scope — 2026-09-28-cli-output-tokens · CLI output tokens

**Source:** `viola-0.1.0/working-route.md:51` (Epoch 2b — Windows slice I b: events and ledger), taken up 2026-09-28.
**Working entry (verbatim title + hint):** CLI output tokens — SGR attention/stale/callsign tokens with depth fallback,
colour decision order, stdout/stderr split, plain under non-TTY/NO_COLOR/TERM=dumb/--json, escaped controls

## Narrowed at P4 (operator ruling — validation-1: intent-incomplete, scope amended)
The overseer (founder-delegated) ruled at the P4 fork "Narrow to live consumers": no abstraction over emptiness, no
code without a real consumer plus a test. This chunk now builds only:
- clap without its `color` feature, so its help and usage output is plain (research F1), with a process witness;
- `run`'s start refusals routed through one stderr refusal writer in `src/human.rs`.

Everything else in the list below leaves as route CARRYs, each named with its owner in plan.md §Implementation notes:
the tokens, the decision and depth rule, the fact reader, the env-var admission and the row-mode escaper go to
working-route.md:79; the message-mode escaper goes to :62; the result/context stream split goes to :60. The list below
stays as the working entry's full intent, for the CARRY text to cite.

## What the working entry intended (full, pre-narrowing)
The shared human-output layer every later human-facing verb (`send`, `wait`, `last`, `list`, `answer`, `release`,
`ui`'s launch line) prints through. It lands before its consumers, so it is proved on its own seam.

- **SGR tokens.** A small hand-written SGR module in the `viola` bin, with no new dependency (design-system §Surface: cli →
  Toolkit). Tokens: `attention` (arrival amber, used only for the `DIALOG` word and always followed by the uncoloured kind
  word); `stale` (SGR dim over a whole `stale` row, where the cock wins for `DIALOG`); `callsign` (bold, NAME values);
  `reset` after every styled span. Handoff blue is never emitted at any depth, and no background is ever set
  (design-system §Tokens (platform-specific)).
- **Depth fallback.** The depth is chosen once per process, first match wins: truecolor when `COLORTERM` is `truecolor` or
  `24bit`; 256-colour when `TERM` contains `256color`, or on Windows once VT enabling has succeeded; otherwise the
  16-colour column. Each token has a value per depth.
- **Colour decision order.** The first match wins and means no colour, no glyph and no SGR: `--json` → `viola run` →
  `viola hook` / `viola mcp` → `NO_COLOR` set → `TERM=dumb` → stdout is not a terminal (std `IsTerminal`) → Windows VT
  enable failed (windows-sys `SetConsoleMode(ENABLE_VIRTUAL_TERMINAL_PROCESSING)`; on failure, no colour).
- **stdout/stderr split.** stdout carries results and data. stderr carries context lines, refusals and their `hint:`
  line, `error: …` lines, and the `viola ui` launch line. Under `--json` the whole result, refusals and `error` objects
  included, is one JSON document on stdout and no `hint:` line is printed (design-system §Streams).
- **Plain output.** Under non-TTY, `NO_COLOR`, `TERM=dumb` and `--json` there is no colour, no glyph and no cursor
  control. The ASCII words (`[RB]`, `[  ]`, `[/ ]`, `->`) still print in every human mode, and human output is ASCII only.
  `viola run` prints nothing while the child runs, and `viola hook` / `viola mcp` never gain an output path.
- **Escaped controls.** Upstream-origin fields have C0/C1 controls rendered as `\x1B`-style hex text (security-plan
  §Input Validation). `list` table rows also escape `\n` and `\t` (`\x0A`, `\x09`), so a row stays one fixed-width line
  (T5, ratified). `wait` / `last` message text keeps `\n` and `\t` (design-system component 1).

## Boundaries
- No human-output verb is built here: `send`, `wait`/`last`, `answer`, `list` and `release` land in their own entries
  (Epochs 3–5) and consume this layer.
- Width detection and NAME truncation (`GetConsoleScreenBufferInfo` / `COLUMNS`) belong to `viola list`'s table, not to
  this layer (design-system §Width applies to human tables, and none exists at HEAD: `src/cmd/` holds `run` and `hook` only).
- No web surface and no change to the tape or strip design.
- No colour on anything but `DIALOG` (amber) and `stale` rows (dim), and never either without its word.

## Folded freight
- **CARRY (chunk 2026-09-24-log-redaction-and-never-log-floor):** the catch site stayed silent because `run` was the only
  verb (obs-plan §7 per-role: `run` prints nothing). When a `cli`-process verb exists, its catch-site error prints exactly
  `error: internal error` on stderr, uncoloured and with no hint (obs-plan §7; design-system §Surface: cli → Exit-code
  phraseology). The chain stays only in the detail file.
  - **Does not bind here (verified at P3).** At HEAD no `cli`-process verb exists: `src/cmd/mod.rs:28-34` holds `Run` and
    a hidden `Hook`, and `main.rs:69-72`'s `Role` is `Hook | Other`. This chunk adds no verb, so no `cli` catch site is
    reachable through the real binary. The CARRY moves on at this chunk's wrap route-resolve to the first entry that adds
    a `cli` verb: working-route.md:53, Capability ledger and viola verify (`viola verify`).
- **CARRY (chunk 2026-09-27-browser-verdict-reachability; overseer live ratification; operator-only):** before the WSL
  distro is next re-provisioned, `scripts/wsl-provision.sh --install-deps` must stop running user-writable code as root.
  Root runs only `apt-get install` over the package list an unprivileged dry run produced, checked against a committed
  allowlist. The chunk that first re-provisions takes it. Until then the CARRY moves on with the first markerless entry.
  - **This chunk does not re-provision the distro (verified at P3).** `scripts/wsl-provision.sh` takes its pins only from
    `rust-toolchain.toml` (line 41), ci.yml's test-job tool line (lines 44, 125-127), `NODE_PIN_VERSION` (line 52) and the
    e2e-web lockfile's Chromium (line 134). The chunk changes none of these. Its only manifest change, clap's `color`
    feature turned off, edits the workspace `Cargo.lock`, which is not a provisioning input, and it needs no windows-sys
    feature (`Win32_System_Console` is already enabled, Cargo.toml:177). The CARRY moves on at this chunk's wrap
    route-resolve to working-route.md:53 (Capability ledger and viola verify).

## CI verdict read at Setup
- `c04e332` (the last wrap's flip = HEAD): ci#36398308023 · green · checks 18/18 · wall 299 s. Nothing to fold.

## Premises closed at P3
- [premise-corrected: architecture.md:586 and security-plan.md:427 are exhaustive ("the only other variables any `viola`
  build reads are the two test seams"), and clap's anstream already reads CLICOLOR/CLICOLOR_FORCE/NO_COLOR/TERM/COLORTERM/CI
  at HEAD] **Env-var tension.** The current wording does not admit `NO_COLOR` / `TERM` / `COLORTERM`. Reading them needs
  a named exception in architecture (Config management, Conventions → Environment variables, Occupied Resources →
  Environment variables) and security-plan (Secret Management, Security Anti-Patterns → Universal). That is a boundary
  widening, ratified live by the founder at wrap (P4 fork 1).
- [premise-corrected: `Win32_System_Console` is already in the workspace windows-sys features (Cargo.toml:177) and the
  root bin takes `windows-sys.workspace = true` (Cargo.toml:123)] Windows VT enabling needs no Cargo change. The
  `vt_output_mode` in viola-pty (`0x000c`, VT processing plus DISABLE_NEWLINE_AUTO_RETURN) is the child-screen mode and
  is not reused for viola's own lines.
- No new obs event: the colour decision logs nothing (the `ObsEvent` enum is closed; obs extract §Constraints).
- The decision function is pure over injected facts (the verb/mode, env readings, TTY, VT result), so the whole order and
  the depth table are unit-testable on every OS without a console (precedent `viola_state::liveness::classify`).
- **Consumer-less layer (research F2):** no production verb calls the token table, the decision or the escaper until
  Epochs 3–5. Only `run`'s `refuse` (run.rs:360) is a live stderr consumer. How the rest lands under `-D warnings` is P4
  fork 2.

## Added at P3 (research F1)
- **clap's own output becomes plain.** clap's default `color` feature colours `--help` and usage errors (bold/underline
  headers) and lets `CLICOLOR_FORCE` put SGR into piped output. Measured: `CLICOLOR_FORCE=1 viola --help | …` gave 25
  ESC bytes, and 0 without it. That breaks design-system §Surface: cli's colour ban and non-TTY rule for viola's own
  output. The chunk turns clap's `color` feature off (a lockfile removal of `anstream`, `anstyle-query`,
  `anstyle-wincon`, `colorchoice`). A process-level witness (`tests/cli_output_plain.rs`) pins it.
