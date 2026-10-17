# Report — 2026-10-17-cli-output-discipline

**Chunk:** CLI output discipline — linear static output, grouped --help, colour beside its word, zero wrapper bytes, unblocked human keys
**Date:** 2026-10-17
**Commits:** since `last_wrap` 2026-10-17T04:26:30Z (`git log --format='%h %s' 6d8b662c..HEAD`, 6d8b662c = the parent of the oldest pre-CI commit):
`94a3d25 chore(2026-10-17-cli-output-discipline): operator pre-CI commit, for the run this chunk's verdict reads` ·
`4a8a0ca fix(2026-10-17-cli-output-discipline): the served-verbs case reads a verb's usage program as viola or viola.exe`

## Changes (structured — detectors read this)
- **Files:** (basis: `cites.py added` over 6d8b662c → the work tree, `{run_dir}/new-text.md`, pasted below; `gate.py scope`: clean — changed 14 · listed 14 · recorded 0)
  - new: `src/human/style.rs` (264 lines) · `tests/support/sgr.rs` (84 lines)
  - modified: `src/cmd/mod.rs` · `src/cmd/list.rs` · `src/human.rs` · `src/human/board.rs` · `tests/cli_output_plain.rs` · `tests/cli_wait_last.rs` · `tests/cli_controls_not_disableable.rs` · `tests/cross_list_rows.rs` · `tests/cli_answer.rs` · `tests/support/events.rs` · `tests/support/mod.rs` · `.github/workflows/windows-mutants.yml`
  - records: the chunk folder (`evidence/red-01…red-04`, `evidence/operator-pass.md`), `.andromeda/runs/2026-10-17T07-44-20-implement/`, `viola-0.1.0/verification-matrix.json` (`v1-40` implemented, ref set)
- **Symbols / APIs:**
  - `viola --help` (the CLI surface): `src/cmd/mod.rs` `Cli` gains `override_usage = "viola [--home <dir>] <verb> [args]"` and `help_template = HELP_TEMPLATE` (`const HELP_TEMPLATE`, `src/cmd/mod.rs:25-37` per the listing). The output is exactly `Usage: viola [--home <dir>] <verb> [args]`, a blank line, the four rows `board:    list` · `traffic:  send, wait, last, answer` · `wheel:    pause, release` · `setup:    run, revive, verify`, a blank line, `Options:` and clap's options (`--home <HOME>`, `-h, --help`, `-V, --version`). The `about` line and clap's `Commands:` list (with its `help` row) leave `viola --help`; `viola help` prints the same table. No `handoff:` row and no `plugin install` (served verbs only, `inputs#I1` Q3, the Viola overseer's technical answer, PROVISIONAL). Each verb's own `--help` is clap's per-verb usage, unchanged; on Windows its usage line names the program `viola.exe` (clap's executable name), as measured on `windows-2025` (`evidence/red-04-ci-windows-help-oracle.txt`), while the grouped help's usage line is the override literal on every OS.
  - New module `src/human/style.rs` (declared `mod style;` in `src/human.rs`; re-export `pub(crate) use style::read as read_colour`): `pub(crate) enum Depth { Truecolor, Ansi256, Ansi16 }` · `pub(crate) struct Tokens { attention, stale, callsign, reset }` with `Depth::tokens()` — attention `\x1b[38;2;217;119;6m` / `\x1b[38;5;172m` / `\x1b[33m`, stale `\x1b[2m`, callsign `\x1b[1m`, reset `\x1b[0m` at every depth; no background and no blue token · `pub(crate) struct Facts { json, passthrough, no_color, term, colorterm, stdout_tty, vt }` · `pub(crate) fn decide(&Facts) -> Option<Depth>`: no colour on `--json`, a passthrough role, `NO_COLOR` present (an empty value included), `TERM` exactly `dumb`, a non-terminal stdout, or `vt == Some(false)`; else truecolor for `COLORTERM` `truecolor` / `24bit`, 256 for a `TERM` containing `256color` or `vt == Some(true)`, else 16 · `pub(crate) struct Colour { depth, (Windows) the VT guard }` · `pub(crate) fn read(json, passthrough) -> Colour`: the fact reader (`std::env::var_os` of `NO_COLOR`, `TERM`, `COLORTERM`, lossy text; stdout `IsTerminal`) and, on Windows only once rungs 1–6 pass, the VT enable (`GetConsoleMode` then `SetConsoleMode(mode | 0x0004)` on stdout, `const VT_PROCESSING: u32 = 0x0004` and `const fn vt_mode`, both `cfg(any(windows, test))`; never viola-pty's `0x000c`), its saved mode restored by `vt::Restore`'s `Drop`; a failed get or set reads as rung 7 and leaves no mode changed. The facts reach no log line, error or refusal and change no escaping or check.
  - `src/human/board.rs`: `board_rows(items, width: Option<usize>, depth: Option<Depth>)` and `write_board(out, budget, board, now, width, depth)` (two parameters added; callers: `list` and the board's own units — sole product caller `src/cmd/list.rs`). With a width, NAME's column is the largest of the caption's 4, the longest wrapped escaped NAME and the longest unwrapped escaped NAME capped at `width − 65` (`const UNWRAPPED_REST` = LIVE 5 + STATUS 7 + WHEEL 23 + DIALOG 17 + five 2-space gutters + `n/a`); an unwrapped NAME longer than the column is cut to `column − 3` characters + `...` (`fn cut`), the cut moved back before a `\xHH` escape it would split (`fn escape_at`); a wrapped NAME is never cut; with `None` the rows are byte-identical to the uncoloured, uncut board. At a depth the spans go around the words after layout (`struct Spans`, `fn padding`): bold on every NAME value (wrapped and unwrapped); the attention token on the `DIALOG` word of a wrapped row with a pending dialog, its kind word plain; a wrapped `stale` row dim from its first byte to its last, every span inside re-opening dim after its reset, NAME dim and bold, and on a stale row with a dialog `\x1b[0m` then the attention token so only `DIALOG` is amber and not dim. The BAY line, caption, empty-board line and separator carry no SGR. Still ONE `write_all`. New `const DIALOG_WORD`, `const NAME_CAPTION`, `const ELLIPSIS`.
  - `src/cmd/list.rs` (human mode only): `viola_pty::host_size().map(|size| usize::from(size.cols))` as the width (a stdout `TIOCGWINSZ` ioctl / the Windows console buffer; no env read; a piped stdout reads none) and `human::read_colour(false, false)` held across the one board write. `--json` and the exit-21 refusal pairs untouched; `unable` / `hint:` carry no SGR.
  - Env vars: `NO_COLOR`, `TERM`, `COLORTERM` — read by every `viola` build's human output (`src/human/style.rs::read`, called only by `viola list`'s human mode), deciding only whether and how deep SGR bytes are written; never set by viola; disable no control and widen no redaction. The widening is the founder's own word: «Читать их» (`inputs#I2`, live in the Viola overseer's dialog at 2026-10-17T07:31:27Z, relayed verbatim by the operator). `COLUMNS`, `CLICOLOR`, `CLICOLOR_FORCE` and `CI` are not read (`grep -rn 'COLUMNS\|CLICOLOR' src` → one comment line, `src/cmd/list.rs:60`, stating `COLUMNS` is not read).
  - Test support: `tests/support/sgr.rs` (`pub mod sgr;`): `sgr_attributes` (moved from `tests/cli_output_plain.rs`, behaviour unchanged), `redraws` (CRs not directly followed by LF), `sets_a_foreground`; `tests/support/events.rs` `pub fn raise(wrapper, kind, nth) -> u64` (moved from `tests/cli_answer.rs`, which imports it, no case changed).
- **Crates / modules:** root bin `viola`: module `human::style` added. No crate added or removed.
- **Dependencies:** none added or bumped (clap stays `=4.6.7` without `color`; the VT enable uses the root's existing `cfg(windows)` `windows-sys` with `Win32_System_Console`; `Cargo.toml` / `Cargo.lock` untouched).
- **Schema / config:** none — no config key, no log line, no event, no schema (`schemas/` untouched).
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - [colour-env, env-exceptions] the env variables any `viola` build reads beside `VIOLA_*`: the two test seams → the two seams + `NO_COLOR`, `TERM`, `COLORTERM` (in every build). Pattern `colour-env`: 22 standing — architecture.md:403 changed (the exhaustive env sentence names the three as closed with clap's `color`); a11y-plan.md:258 changed ("reads no `NO_COLOR` / `TERM` / `COLORTERM`"); 20 no-change (design-system.md:43, :391, :400, :404, :405, :410, :411, :555; layout-templates.md:42, :79; test-plan.md:102, :588, :878; a11y-plan.md:27, :74, :152, :189, :233, :236, :368 — each states the no-colour rule or the depth rule, now built as stated). Pattern `env-exceptions`: 8 standing — architecture.md:104 (§Naming "Two ratified exceptions"), :157 (§Conventions "The two ratified exceptions"), :403 (Config management), security-plan.md:410 (Storage), :542 (Universal) changed; architecture.md:333 (§Occupied Resources env registry) and security-plan.md:206 (an Input Validation row for the seam) are where the new group / row joins, beside rows that stay; test-plan.md:555 no-change (the seam's own measurement).
  - [colour-layer, owner-title] "uncoloured / emits no SGR / owed to 'CLI output discipline'" → the colour layer built. Pattern `colour-layer`: 10 standing — design-system.md:441 (@c123, @c825), layout-templates.md:91, a11y-plan.md:234, a11y-plan.md:258 changed; design-system.md:79, :384, :387, :504, layout-templates.md:23, :29 no-change (each true as built: the kind word uncoloured, the hand-written SGR module, `fail` uncoloured). Pattern `owner-title`: 10 standing — design-system.md:441 @c850, layout-templates.md:59, :91, a11y-plan.md:73, :134, :209, :234, :258 changed (each an "owed to 'CLI output discipline'" now discharged); a11y-plan.md:105, :125 no-change (they name the check class "CLI output discipline").
  - [width] unwrapped NAMEs "never truncated" → cut on a terminal too narrow; the width source `COLUMNS` → `viola_pty::host_size()`. Pattern `width`: 21 standing — design-system.md:420 (`COLUMNS elsewhere`), :441 @c569 ("never truncated"), layout-templates.md:91 ("never truncated"), :298 ("no styling, no truncation"), a11y-plan.md:258 ("with no truncation") changed; layout-templates.md:71, :89, design-system.md:421 no-change (already the rule: cut only when the width is known); security-plan.md:211 no-change (`host_size` never zero); 12 no-change sharing the token only (truncated SHA-256 / events never truncated / a truncated coverage profile: architecture.md:22, :90, :343; security-plan.md:26, :201, :246, :496; test-plan.md:166, :377, :911, :957; registries/contracts/test-plan/5-command-implementation.md:86).
  - [help] the built help: clap's ordered `Commands:` list → the grouped verb table, served verbs only. Pattern `help`: 8 standing — design-system.md:526 (lists `handoff: link, unlink` and `plugin install`), layout-templates.md:59 ("not built"), :202/:205 (the wireframe's five rows), a11y-plan.md:209 ("not built … clap's ordered list") changed; architecture.md:403 (F1's measurement, a quotation of history), test-plan.md:589, :595 no-change.
  - [trycmd] see Spec claims disproved. [sgr-parser] test-plan.md:589 changed (the parser now lives in `tests/support/sgr.rs`; `tests/cli_output_plain.rs` gains the grouped-help and served-verbs cases and the Unix CR reading). [stderr-writer] see Expected amendments (CARRY 1). [win-mutants] 18 standing, 0 changed — no master lists a label's files (pattern `viola-main` part: 0 master hits); the workflow's `viola-main` item lists `src/human/style.rs` beside `src/main.rs src/conpty.rs`, held by `tests/contract_windows_mutation_scope.rs`.
  - Sweep reach: 10 patterns · 6 moved facts, 6 with a pattern (the report's own count: env set, colour layer, width rule + source, help table, trycmd/help pin, the parser's home; CARRY 1's stderr writers and the mutants list counted under their own patterns) · owner pattern 10 standing · by line 4 lines · 4 offsets (the generic owner phrase `owned by a (route entry|CARRY)` dropped: its control unfired over the masters). Basis: `cascade.py sweep --pre-pass`, trail `.andromeda/runs/2026-10-17T08-28-14-wrap/cascade-2026-10-17-cli-output-discipline-pre-pass.json`, `total (10 patterns) · 179 rows over 36 files`.
- **Dev-tool versions:** none — no host tool installed or changed.
- **Harness / gate surface:** none — no `agent-run` verb, status shape or CI step changed; `.github/workflows/windows-mutants.yml` adds `src/human/style.rs` to the `viola-main` matrix item's `files:` (dispatch-only, no gate).
- **Cross-project / external claims:**
  - CI: `ci#39015978414` on `94a3d25e976f` — completed `failure`, 14/15 green, `test (windows-2025)` red on `help_lists_only_served_verbs` (this chunk's own test oracle; `evidence/red-04-…`); `ci#39016750772` on `4a8a0ca52bfe` (run_attempt 1) — `verdict: green · checks 15/15 · wall 545 s` (`evidence/operator-pass.md`). This wrap's commit adds to `4a8a0ca`.
  - inputs (`inputs.py verify`, trail `{run_dir}/inputs-2026-10-17-cli-output-discipline.json`: re-fired after I4's snap and this report: `inputs: 4 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 4 · uncited 0 · unparsed 0`; no DRIFT):
    - `inputs#I1` · `inputs/I1-relay-1.md.txt` · copy message · n/a — the P4 fork dialog: Q1 HELD for the founder (superseded by I2), Q2 the width source `viola_pty::host_size()` and Q3 served verbs only — the Viola overseer's technical answers, a delegate's, PROVISIONAL.
    - `inputs#I2` · `inputs/I2-relay-2.md.txt` · copy message · n/a — the founder's own answer to Q1, «Читать их»: the colour layer reads `NO_COLOR`, `TERM`, `COLORTERM`, the widening ratified by him.
    - `inputs#I3` · `inputs/I3-relay-3.md.txt` · copy message · n/a (printed UNCITED before this report; cited here) — the P5 word "yes" typed by the operator on the review card `.andromeda/runs/2026-10-17T04-35-05-phase/p5-card-1.md`; its `## Beyond the plan` rows, verbatim, with the word's authority (the operator's word approving the card; the Q2 / Q3 answers it carries stay the overseer's PROVISIONAL ones):
      - The `handoff:` help row (`link`, `unlink`) is owned by the existing entry "Explicit session links", which adds the row when it serves the verbs.
      - `plugin install` in the `setup:` help row is owned by the existing entry "Optional plugin install", which adds the word when it serves the verb.
      - The dim span on a real stale row under the outer PTY is not witnessed at the integration tier in this chunk or by any entry: it is proven at the unit tier only, with no owner minted (the 10 s `mutants` kill against the 5 s stale threshold plus the ~4 s stamp).
      - The overseer's provisional answers Q2 (the width source) and Q3 (served verbs only) are carried to the founder's next batch for his own word, as delegate answers (`inputs#I1`).
    - `inputs#I4` · `inputs/I4-relay-1.md.txt` · copy message · n/a (snapped at this wrap) — the operator's direction at the wrap's invocation: the integration-level witness of "dim only on stale rows" (`v1-40`) gets an owner at P5 (a CARRY onto its real first consumer, or the armed orphan halt), never left unowned; the env-read widening restated as the founder's own word, `inputs#I2`.
- **Reverted / negative API facts:** `drop(colour)` after the board write was written and removed (clippy `drop_non_drop` on Unix, where `Colour` holds no guard): the guard is held by scope to the end of `list`'s human branch. `Colour` was re-exported and the re-export removed (unused outside `human`).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - [trycmd] test-plan and a11y-plan state that fixed CLI strings (the `viola-help` case, the `[RB]` mirror, `unable … hint:`) are pinned by trycmd / `tests/cmd/*.toml` and checked with assert_cmd: no manifest holds `trycmd` or `assert_cmd` (`grep 'trycmd|assert_cmd' Cargo.toml crates/*/Cargo.toml` → 0, research.md §Sweeps) and no `tests/cmd/` exists. Pattern `trycmd`: 28 standing (test-plan.md:383, :403, :542, :585, :586, :614, :666, :850, :931, :1211; a11y-plan.md:60, :91, :191, :192, :216, :222, :229, :232, :293, :303, :309, :323, :358; registries a11y-testing-tool-pick.md:4, a11y bootstrap-phases…:7, keyboard-test-harness.md:3, architecture project-directory-structure.md:48, test-plan bootstrap-phases…:22). Measured this chunk: `viola --help` is pinned by the `HELP` literal in `tests/cli_output_plain.rs` (`help_is_the_grouped_verb_table`). Disposition sought: the `viola-help` sites (test-plan §5 / §6 / §7, the plan's expected amendment) amended to the literal; the rest are a pre-existing tooling claim this chunk did not move — DISPOSED at Validate (amend or carry), never left open.
  - [width] design-system.md:420 names `COLUMNS` as the width source off Windows: `viola list` reads `viola_pty::host_size()` (stdout `TIOCGWINSZ`), `inputs#I1` Q2.
- **Expected amendments (from plan):** (each with its pattern; basis: the pre-pass sweep above)
  - architecture §Cross-cutting Patterns → Config management (architecture.md:403) — carried: Symbols / APIs, Env vars; patterns `colour-env` (arch 1 hit, :403), `env-exceptions` (:403). Authority: the founder, `inputs#I2`.
  - architecture §Occupied Resources → Environment variables (architecture.md:333 group beside), §Naming (:104), §Conventions → Environment variables (:157) — carried: Env vars; pattern `env-exceptions` (arch :104, :157, :333). Authority `inputs#I2`.
  - security-plan §Secret Management Storage (security-plan.md:410), §Security Anti-Patterns Universal (:542), an §Input Validation row (beside :206) — carried: Env vars; pattern `env-exceptions` (sec :206, :410, :542; `colour-env` 0 security-plan hits). Authority `inputs#I2`.
  - design-system §Component Patterns pattern 1 As built (design-system.md:441) — carried: Symbols board.rs; patterns `colour-layer` (:441 @c123, @c825), `width` (:441 @c569), `owner-title` (:441 @c850).
  - design-system §Surface: cli Width (design-system.md:420) — carried: Symbols list.rs; pattern `width` (:420). Authority: the overseer, `inputs#I1` Q2, PROVISIONAL.
  - design-system §Navigation Pattern (design-system.md:526) — carried: Symbols `viola --help`; pattern `help` (:526). Authority `inputs#I1` Q3, PROVISIONAL.
  - design-system §Tokens (platform-specific) — `NO_COLOR` set read as present, an empty value included (design-system.md:404); on a stale row NAME dim and bold (:394) — carried: Symbols style.rs / board.rs; pattern `colour-env` (:404); the stale-NAME composition has no pattern hit of its own (:394 named by reading the Tokens table).
  - layout-templates §Output structure — `viola --help` (layout-templates.md:202-210), §Primary screens (:59), §Output structure — `viola list` As built (:91), §Component — Primary content block 1 (:298) — carried; patterns `help` (:59, :202, :205), `colour-layer` (:91), `width` (:91, :298), `owner-title` (:59, :91).
  - a11y-plan §4 ARIA Patterns & Roles P6 (a11y-plan.md:234, :258), the cli landmark row (:134, :209), §1 cli Notes (:73) — carried; patterns `colour-layer` (:234, :258), `owner-title` (:73, :134, :209, :234, :258), `help` (:209), `colour-env` (:258), `width` (:258).
  - test-plan §5 / §6 / §7 — `viola-help` pinned by literal, no trycmd case; the SGR parser in `tests/support/sgr.rs` — carried: Spec claims disproved [trycmd], Counts [sgr-parser]; patterns `trycmd` (test-plan 10 hits), `sgr-parser` (:589).
  - obs-plan §3 (intro, obs-plan.md:505) and §11 Logs (:1071) — CARRY 1: `human::refuse` → `write_refusal` is called by `run` (5 sites), `verify` (5), `list` (2) and `revive` (1) — re-derived on the final tree, `grep -rcE 'human::refuse\(' src` → `src/cmd/verify.rs:5 · src/cmd/run.rs:5 · src/cmd/revive.rs:1 · src/cmd/list.rs:2`; `human::` used by 12 files under `src/` (`grep -rlE 'human::' src`: answer, client, last, list, pause, release, revive, run, send, verify, wait, main). The colour layer adds no stderr writer. Carried: this bullet; pattern `stderr-writer` (obs :505, :1071; registries build-system.md:4 and project-directory-structure.md:21 no-change — they already name `refuse` / `write_refusal` and every caller verb).
- **Coverage of new surfaces:**
  - `viola --help` grouped table → validation n/a · instrumentation n/a (clap prints, no line) · PII n/a · tests integ (`help_is_the_grouped_verb_table`, `help_lists_only_served_verbs`, `help_is_plain_*`, the completeness case) · a11y ✓ (plain ASCII, group labels as the structural equivalent, no SGR under a forced pipe, no CR redraw on Unix) · tokens n/a
  - `viola list` colour spans + width cut → validation ✓ (rows escaped before layout; the cut never splits an escape) · instrumentation ✓ (no new line; `cli.list` span unchanged) · PII n/a (names already escaped) · tests unit (`board_row_width_*`, `board_row_style_*`) / e2e (`cross_list_rows_*`, ubuntu, macOS and windows-2025 green in `ci#39016750772`) · a11y ✓ (colour only beside its word; none under NO_COLOR / dumb / pipe / --json) · tokens design-token✓ (the design-system §Tokens bytes, literal-pinned in `colour_tokens_are_the_design_bytes`)
  - `src/human/style.rs` fact reader + Windows VT enable → validation ✓ (closed `Depth`, values only compared, never echoed) · instrumentation n/a (no line by design: obs-plan §11) · PII n/a · tests unit (`colour_*` on every OS; the `cfg(windows)` body exercised by the windows-2025 e2e cases) · a11y ✓ · tokens ✓

## Deviations from intent
1. Step 13 — `sgr_attributes` moved to `tests/support/sgr.rs`, but its self-check case stays in `tests/cli_output_plain.rs` (calling the support reader): a `#[test]` inside `tests/support/` would compile into every binary declaring `mod support` and run in each.
2. Step 14 — the `windows-2025` styled reading asserts the parsed attribute set (holds `1`, sets a foreground, lacks `2`); the exact ConPTY sequence the plan says to copy from the first `windows-2025` run is not pinned: no run of it could be made on the Linux host, and the CI job logs print no passing case's stream. The attribute-set case read green on `windows-2025` in `ci#39015978414` and `ci#39016750772`.
3. Step 15 — `assert_send_paste_control_holds` now removes `NO_COLOR`, `TERM` and `COLORTERM` before setting the case's own (listed file): its remove-the-guard run read all 8 cases red under the host's inherited `COLORTERM=truecolor` until it did, then exactly the three new cases.
4. Step 18 — the Unix CR reading is the shared `redraws` helper (`tests/support/sgr.rs`).
5. Two units beyond the plan's names: `colour_decision_a_terminal_with_every_rung_passing_is_coloured` and `colour_vt_mode_adds_only_the_processing_bit` (pins `0x0004` alone; testing.md 2026-09-27).
6. Step 2's per-verb usage reading takes the program word with a trailing `.exe` off (after `ci#39015978414`); the plan's stated oracle (exit 0 per verb, no unserved verb in the table) unchanged.
7. The terminal values are read with `to_string_lossy`.
scope record: none — gate.py scope clean, 0 recorded (`scope: clean — changed 14 · listed 14 · recorded 0`).

## Decisions & corrections
- The founder's own word ratifies the env-read widening: «Читать их» — `NO_COLOR`, `TERM`, `COLORTERM` (`inputs#I2`); the operator's wrap invocation restates it ("The founder widening (NO_COLOR, TERM, COLORTERM) is his own word, inputs#I2").
- PROVISIONAL, for the founder's next batch: the width source `viola_pty::host_size()` (Q2) and served verbs only in `--help` (Q3) are the Viola overseer's technical answers, a delegate's (`inputs#I1`; card row 4).
- Owed at P5 (the operator's direction at this wrap's invocation): "the integration-level witness of 'dim only on stale rows' (v1-40) is proven at the unit tier only and has no owner; give it one at P5 (a CARRY onto its real first consumer, or the armed orphan halt), never leave it unowned." The card's row 3 records the same gap with no owner minted.
- Measurement: a test that runs `viola` without clearing `NO_COLOR` / `TERM` / `COLORTERM` inherits the host terminal's values (`COLORTERM=truecolor` on the Linux dev host), so a colour-sensitive case — and a remove-the-guard run over one — sets or removes them per child (`evidence/red-03-remove-the-guard.txt`, the paste control's first firing: 8 of 8 red).
- Measurement: on Windows clap's per-verb usage names the program `viola.exe`; only an `override_usage` literal reads `viola` on every OS (`evidence/red-04-…`, `ci#39015978414`).
- Hazard: a Rust `"…\n\` string continuation strips the next line's leading whitespace, so a literal of indented output lines is written with `concat!` (`evidence/red-01-…`).
- Hazard: an ad hoc `viola list` typed in the session's own shell reads the host's real `claude agents --json` (11 live sessions listed); a probe of `list` puts the fake agent first on `PATH`, as tests do (outcome file, Decisions).
- Guard refusals (implement): a `cd` into the chunk's `inputs/` (re-issued with absolute paths) and a `cat` heredoc to a file (re-issued through the Edit tool); (wrap) a `cd` into the skill's references dir (re-issued with an absolute path).
- The operator's word for this chunk's implement: "No live claude start in this chunk … No mutation run of any kind. Then run the operator pass with the ci.py conclusion read (leg=operator) as usual, and stop ready for the wrap." — honoured: no live start, no mutation run (the remove-the-guard runs were hand neutralisations through `agent-run.sh run`, never cargo-mutants).

## Outcome
- Acceptance re-asserted against the diff:
  - (layouts) `viola --help` the grouped table, zero ESC piped under `CLICOLOR_FORCE=1` — MET (`help_is_the_grouped_verb_table`, three OSes green); served verbs only per `inputs#I1` Q3, PROVISIONAL.
  - (design) every listed verb answers `--help` exit 0; no `hook`/`help`/`link`/`unlink`/`plugin` in the table — MET (`help_lists_only_served_verbs`).
  - (tests) the completeness case and the three plain-output cases stay green — MET.
  - (design) the width cut, wrapped never cut, no split escape, piped and `--json` whole — MET (`board_row_width_*`, `cross_list_rows_truncate_…`); the width source per `inputs#I1` Q2, PROVISIONAL.
  - (design) the colour decision and the design bytes — MET (`colour_decision_each_rung_switches_colour_off` (six rungs: the plan's seven-rung order with `run` and `hook`/`mcp` as one `passthrough` fact), `colour_decision_picks_the_depth`, `colour_tokens_are_the_design_bytes`).
  - (a11y) spans beside their words under the outer PTY; dim only on stale rows at the unit tier — MET as the acceptance words it; the integration-tier dim reading is unowned (Decisions, card row 3) → P5.
  - (a11y) no SGR under `NO_COLOR` (empty included), `TERM=dumb`, a pipe, `--json` — MET (`cross_list_rows_print_no_sgr_…`).
  - (security) no colour variable changes escaping or the paste control — MET (`board_row_style_an_escaped_control_reads_the_same_at_every_depth`, the three cases of `setting_does_not_disable_the_send_paste_control`).
  - (a11y) no ESC and no lone CR on Unix for `wait` and `--help` — MET.
  - (a11y) the P4 witnesses re-run green on all three OS legs — MET (`ci#39016750772` test jobs green; `tests/tui_passthrough.rs`, `send_under_the_paste_hint_…`, `tui_focus_mouse_and_resize_…`).
  - (obs) `viola verify` step-line literals — MET (`tests/cli_verify.rs` green).
  - (matrix) `v1-40` — implemented, ref set (`.andromeda/runs/2026-10-17T07-44-20-implement/matrix-2026-10-17-cli-output-discipline.json`); every clause but the `ui` line (out of 0.1.0 by the founder's 2026-10-16 ruling) proven.
  - (tests) every gate green on this host and CI `verdict: green` on all three OSes — MET (`ci#39016750772`).
- Gates (implement's block run, trail `.andromeda/runs/2026-10-17T07-44-20-implement/gate-2026-10-17-cli-output-discipline.json`, `entries 21 · green 18 · red 0 · recorded 0 · timeout 0 · not-run 3`): `cargo fmt --all --check` green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` green · `CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc …` green · `rg … instrument …` green (exit 1, no output) · `bash scripts/lint-probes.sh` green · `bash scripts/agent-run.sh run --unit` green · `run --unit --filter 'test(/board_row_|colour_/)'` green · `run --integration --filter 'binary(cli_output_plain) | …'` green · `run --integration` green · `run --e2e --filter 'binary(cross_list_rows)'` green · `run --e2e` green · `cleanup --session p-cod-smoke` green · `boot --session p-cod-smoke` green · `status --session p-cod-smoke` green · `bash scripts/g2-zero-panics.sh` green · `agent-run.sh schema-check` green · `cleanup --session p-cod-smoke` green · `agent-run.sh pre-push` green · `gate.py hygiene` (leg operator) → `evidence/operator-pass.md`: `hygiene: clean` both rounds · `git diff --quiet && git diff --cached --quiet && git push origin HEAD` (leg operator) → pushed `94a3d25`, then `4a8a0ca` · `ci.py conclusion --sha HEAD --wait 1800` (leg operator) → `94a3d25`: red (test windows-2025, this chunk's oracle, fixed) · `4a8a0ca`: `verdict: green · checks 15/15`. After the fix, entries 1, 2, 3, 8 re-ran green (implement trail, firing 2). Smoke: fired (boot path) — a by-hand session: help, `list` piped plain and under `script(1)` at 80 columns bold NAME values only; cleaned exactly.
- Watches: none folded.
- Outcome basis: the operator pass ran — Setup 4's commit list (`94a3d25`, `4a8a0ca`) and the final HEAD's CI run `ci#39016750772` recorded in `evidence/operator-pass.md`; implement's report `.andromeda/runs/2026-10-17T07-44-20-implement/outcome-2026-10-17-cli-output-discipline.md` (this window holds that run whole).
- Process hygiene: implement's census (outcome file): the block's `p-cod-smoke` and P3's `p-cod-p3` sessions terminated (`processes_gone:true`, `endpoint_gone:true`); the control batches, the gate block, `script(1)` and the ad hoc `viola list` terminated. Re-measured at this wrap with `ps -eo pid,comm,args` after cleanup: every running `viola` is the overseer bridge's prototype build (`~/dev/projects/additional/viola-lab/prototype/target/debug/viola`), none this repository's.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 6d8b662c (the parent of the oldest pre-CI commit 94a3d25e) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/workflows/windows-mutants.yml — added 1 line(s) in 1 range(s)
added: 41
### src/cmd/list.rs — added 12 line(s) in 1 range(s)
added: 60-71
### src/cmd/mod.rs — added 16 line(s) in 2 range(s)
added: 25-37 · 42-44
- 25-28 @27 «const HELP_TEMPLATE: &str = "\»
- 35-36 «Options:»
### src/human.rs — added 2 line(s) in 2 range(s)
added: 11 · 22
### src/human/board.rs — added 472 line(s) in 29 range(s)
added: 9 · 14 · 113-115 · 121-122 · 140-156 · 158-178 · 180-183 · 190-230 · 232-244 · 246-248 · 250 · 257-266 · 268
       270 · 277 · 294-295 · 298 · 482-486 · 506 · 578 · 591 · 606 · 632 · 651 · 664-674 · 701 · 717 · 730 · 740-1063
- 140-149 @142 «struct Spans {»
- 151-153 «fn padding(text: &str, width: usize) -> String {»
  - 158-178 «let (shown_name, shown_dialog, open, close) = match styled {»
- 192-195 @194 «const UNWRAPPED_REST: usize =»
- 199-206 @200 «fn escape_at(chars: &[char], at: usize) -> bool {»
  - 201-205 «chars.get(at) == Some(&'\\')»
- 208-219 @210 «fn cut(name: &str, column: usize) -> String {»
  - 213-215 «if let Some(at) = (keep.saturating_sub(3)..keep).find(|&at| escape_at(&chars, at)) {»
  - 232-239 «let longest = |wrapped: bool| {»
  - 240-243 «let unwrapped = match width {»
  - 743-757 @744 «fn board_row_width_none_cuts_nothing() {»
  - 759-788 @761 «fn board_row_width_cuts_an_unwrapped_name_to_fit_the_width() {»
  - 790-798 @791 «fn board_row_width_never_cuts_a_wrapped_name() {»
  - 800-833 @802 «fn board_row_width_never_cuts_inside_an_escape() {»
  - 835-852 @836 «fn board_row_width_too_small_keeps_the_captions_four_columns() {»
  - 854-880 @855 «fn board_row_width_an_unwrapped_name_that_fits_is_never_cut() {»
  - 886-890 «fn pending(mut row: Item) -> Item {»
  - 892-904 @893 «fn plain(text: &str) -> String {»
  - 906-929 @910 «fn board_row_style_a_live_row_bolds_its_name_and_ambers_the_dialog_word(»
  - 931-959 @935 «fn board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber(»
  - 961-974 @962 «fn board_row_style_an_unwrapped_row_bolds_only_its_name() {»
  - 976-1002 @977 «fn board_row_style_the_context_lines_carry_no_sgr() {»
  - 1004-1027 @1010 «fn board_row_style_keeps_the_plain_boards_columns(#[case] depth: Depth) {»
### src/human/style.rs — new file · 264 line(s)
- 8-14 @10 «pub(crate) enum Depth {»
- 16-27 @18 «pub(crate) struct Tokens {»
- 29-43 «impl Depth {»
  - 30-42 «pub(crate) const fn tokens(self) -> Tokens {»
- 45-58 @48 «pub(crate) struct Facts {»
- 60-85 @63 «pub(crate) fn decide(facts: &Facts) -> Option<Depth> {»
  - 64-72 «if facts.json»
  - 73-79 «if matches!(facts.colorterm.as_deref(), Some("truecolor" | "24bit")) {»
  - 80-84 «{»
- 91-95 @93 «const fn vt_mode(mode: u32) -> u32 {»
- 97-102 @98 «pub(crate) struct Colour {»
- 104-135 @107 «pub(crate) fn read(json: bool, passthrough: bool) -> Colour {»
  - 109-117 «let facts = Facts {»
  - 118-129 @119 «let colour = {»
  - 130-133 @131 «let colour = Colour {»
- 137-172 @139 «mod vt {»
  - 141-143 «use windows_sys::Win32::System::Console::{»
  - 145-148 «pub(super) struct Restore {»
  - 150-155 «impl Drop for Restore {»
  - 157-171 @158 «pub(super) fn enable() -> Option<Restore> {»
- 174-264 @175 «mod tests {»
  - 179-186 @180 «fn colourful() -> Facts {»
  - 188-198 @196 «fn colour_decision_each_rung_switches_colour_off(#[case] facts: Facts) {»
  - 200-208 @201 «fn colour_decision_a_terminal_with_every_rung_passing_is_coloured() {»
  - 210-217 «fn facts(term: Option<&str>, colorterm: Option<&str>, vt: Option<bool>) -> Facts {»
  - 219-238 @231 «fn colour_decision_picks_the_depth(»
  - 240-252 @244 «fn colour_tokens_are_the_design_bytes(#[case] depth: Depth, #[case] attention: &'static str) {»
  - 254-263 @261 «fn colour_vt_mode_adds_only_the_processing_bit(#[case] mode: u32, #[case] want: u32) {»
### tests/cli_answer.rs — added 1 line(s) in 1 range(s)
added: 25
### tests/cli_controls_not_disableable.rs — added 7 line(s) in 2 range(s)
added: 137-140 · 187-189
### tests/cli_output_plain.rs — added 83 line(s) in 4 range(s)
added: 15-17 · 37-38 · 49-122 · 153-156
- 54-68 @56 «const HELP: &str = concat!(»
- 70-76 @71 «fn help_is_the_grouped_verb_table() {»
  - 81-83 «let served = [»
  - 84-104 «for verb in served {»
  - 106-111 «let rows: Vec<String> = String::from_utf8(table.stdout)»
  - 112-118 «let words: Vec<&str> = rows»
  - 120-122 «for unserved in ["hook", "help", "link", "unlink", "plugin"] {»
### tests/cli_wait_last.rs — added 9 line(s) in 1 range(s)
added: 383-391
  - 384-390 @387 «{»
### tests/cross_list_rows.rs — added 246 line(s) in 2 range(s)
added: 97-104 · 150-387
  - 97-104 @99 «let mut pty = OuterPty::spawn_sized(»
- 151-159 @152 «const INHERITED: [&str; 6] = [»
- 161-180 @163 «fn on_a_terminal(bin: &Path, home: &Path, env: &[(&str, &str)], cols: u16) -> Vec<u8> {»
  - 168-171 «let remove: Vec<&str> = INHERITED»
  - 172-175 «let size = Size {»
- 182-201 @184 «fn piped_with(»
  - 192-194 «for name in INHERITED {»
  - 195-199 «let out = command»
- 203-221 @205 «fn escape_sequences(stream: &[u8]) -> Vec<String> {»
  - 208-219 «while i < stream.len() {»
- 228-246 @230 «fn builder_with_a_dialog(»
  - 233-237 «let wrapper = support::events::boot(»
  - 239-243 «let scratch = wrapper»
- 248-292 @252 «fn cross_list_rows_style_each_span_beside_its_word_on_a_terminal(stamped_home: StampedHome) {»
  - 255-290 «for (env, amber) in [»
- 294-342 @297 «fn cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json(stamped_home: StampedHome) {»
  - 300-315 «let assert_plain = |label: &str, stream: &[u8]| {»
  - 316-328 «for (label, env) in [»
- 347-387 @350 «fn cross_list_rows_truncate_an_unwrapped_name_only_on_a_terminal_too_narrow_for_it(»
  - 356-358 «let row = format!(»
  - 371-374 «assert!(»
  - 379-384 «let names: Vec<&Value> = doc["ok"]["items"]»
### tests/support/events.rs — added 18 line(s) in 1 range(s)
added: 34-51
- 34-50 @35 «pub fn raise(wrapper: &Wrapper, kind: &str, nth: usize) -> u64 {»
  - 37-43 «let of_kind = |lines: &[Value]| -> Vec<Value> {»
  - 44-46 «let lines = wait_events(&wrapper.instance_dir(), "the dialog line", |l| {»
  - 47-49 «of_kind(&lines)[nth - 1]["data"]["dialog_id"]»
### tests/support/mod.rs — added 1 line(s) in 1 range(s)
added: 14
### tests/support/sgr.rs — new file · 84 line(s)
- 6-32 @7 «fn sgr_sequences(stream: &[u8]) -> Vec<Vec<u32>> {»
  - 10-30 «while i + 1 < stream.len() {»
- 34-53 @36 «pub fn sgr_attributes(stream: &[u8]) -> Vec<u32> {»
  - 38-51 «for sequence in sgr_sequences(stream) {»
- 55-62 @56 «pub fn redraws(stream: &[u8]) -> usize {»
  - 57-61 «stream»
- 64-84 @66 «pub fn sets_a_foreground(stream: &[u8]) -> bool {»
  - 67-83 «sgr_sequences(stream).into_iter().any(|sequence| {»
