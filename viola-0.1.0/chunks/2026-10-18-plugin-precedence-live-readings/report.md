# Report — 2026-10-18-plugin-precedence-live-readings

**Chunk:** the two plugin readings on claude 2.1.287 (which `viola` MCP server a session holding both plugins starts
and answers `list`; whether an exit-1 `viola mcp` blocks an unwrapped start), the method past the folder-trust
dialog, v1-42
**Date:** 2026-10-19
**Commits:** `2a3d5691 chore(2026-10-18-plugin-precedence-live-readings): operator pre-CI commit, for the run this
chunk's verdict reads` (since `last_wrap` 2026-10-18T22:32:35Z; `git log --since`)

## Changes (structured — detectors read this)
- **Files:** none in product code, tests, harness, fixtures or plugin. New, all under
  `viola-0.1.0/chunks/2026-10-18-plugin-precedence-live-readings/`: `evidence/live-pty.py`, `evidence/live-ledger.py`
  (both `cmp` exit 0 against `chunks/2026-10-17-optional-plugin-install/evidence/`), `evidence/live-pty-arrow.py`
  (that chunk's `live-pty-trust.py`, its key rule replaced; `declared_env`, `Screen`, `stamp`, `write_status`
  byte-equal by an `ast` source-segment compare), `evidence/rig-controls.py`, `evidence/live-round.md`,
  `evidence/live-sessions.ndjson`, `evidence/operator-pass.md`; inputs `I4`, `I5`. Ledger:
  `viola-0.1.0/verification-matrix.json` (v1-42). Basis: `git diff --name-only bc2e3bf7` (69 paths, the rest run dirs,
  the route and phase take-up files); `gate.py scope` `clean — changed 0` (evidence excluded by design); `cites.py added`:
  0 files.
- **Symbols / APIs:** none changed. Measured as built (read, not edited): `viola plugin install`'s tree
  (`marketplace_files`, `src/cmd/plugin.rs`), the `--plugin-dir` plugin's `viola mcp` (`plugin/.mcp.json`, `args`
  `["mcp"]`, home through `VIOLA_DIR`), `home_passes` (`src/cmd/mcp.rs`), the wheel (`src/run/wheel.rs:42`, a rig key
  moved it to `holder:"human", cause:"human-input"`; `viola release driver` returned it, `cause:"release"`).
- **Crates / modules:** none.
- **Dependencies:** none.
- **Schema / config:** none.
- **Spec-master edits:** none this chunk (the take-up's P4/P5 wrote scope and plan only).
- **Retired:** none.
- **Counts / qualifiers moved:** none — verified (no derived value of a master changed: the ledger stays 24 rows, no
  row added — plugin precedence is no row, `architecture.md:87` PREC standing, true as it stands).
  Sweep reach: 7 patterns · 6 moved facts, 6 with a pattern (the report's own count: PREC, EXIT1, RESID, VDIR, EXTIMP,
  MCPEXIT) · owner pattern 2 standing · by line 4 lines · 8 offsets (`cascade.py sweep --pre-pass`, trail
  `cascade-2026-10-18-plugin-precedence-live-readings-pre-pass.json`; TRUSTKEY `folder-trust|trust dialog|Одна
  клавиша|one Enter` refused `control UNFIRED over the seven masters at 2a3d5691` — 0 master sites; the generic owner
  phrase dropped by rule).
- **Dev-tool versions:** none — `claude` (the CLI, the masters' "CLI") re-read at `2.1.287 (Claude Code)` by absolute
  path before each start, Linux dev host.
- **Harness / gate surface:** none changed. Gate block fired: 8 green, 4 deferrals voided (`run --unit`:
  `.andromeda/master-route.md` read by `crates/viola-e2e/src/harness/run/mutants/base.rs`; `run --integration`,
  `run --e2e`, `pre-push`: walk-class rust 4 marked walkers, 60 uncommitted), clippy deferred (zero Rust delta).
- **Cross-project / external claims:**
  - **The precedence reading (start 2, wrapped, claude 2.1.287, Linux dev host, 2026-10-19T06:13Z):** a session
    started by `viola run driver` from a `proj/` where the installed `viola@viola` is enabled at `--scope local`
    started ONE `viola` MCP server, the `--plugin-dir` plugin's: `whome/diagnostics/mcp.ndjson` 1
    `process-start{subject:"self"}` + the `agents-probe` pair during the `list` call; `ihome` (the installed plugin's
    home, `--home <ihome>`) has no `diagnostics/` at all — the installed server never started. The CLI's own MCP log
    (`~/.cache/claude-cli-nodejs/<proj>/mcp-logs-plugin-viola-viola/`, outcome words only) holds one server name,
    `plugin-viola-viola`, one connection (`Successfully connected (transport: stdio) in 62ms`) and one `list` call
    completed: both plugins register that one name, and the CLI starts one. One `session-start` in `events.ndjson`,
    one `hook-invoked{hook_event:"session-start"}` in `hook-driver.ndjson` (one hook process per event; the installed
    plugin carries no hooks). No permission event. The send confirmed (`send-confirmed`), `turn-ended`, `last` equal
    to the fixed word. Basis: `evidence/live-round.md` §Start 2; `evidence/live-sessions.ndjson` rows 3–4.
  - **The exit-1 reading (start 3, unwrapped, 06:15Z):** with `<ihome>` at 0770 the control `viola --home <ihome> mcp
    </dev/null` exited 1, stdout 0 B, stderr 0 B, `<ihome>/diagnostics/` absent before and after. The unwrapped
    `claude --model haiku` from the same `proj/` (only the installed plugin registers `viola`) read `idle` in one
    bounded `claude agents --json` read 20 s after its first `absent` read (the start's pid row, 12 rows, exit 0):
    **an exit-1 `viola mcp` does not block the CLI's start.** The CLI logged the server `Connection failed after 101ms
    (CONNECTION_CLOSED)` after one respawn; `<ihome>` stayed 0770, no `mcp.ndjson`. Basis: `evidence/live-round.md`
    §Start 3; ledger rows 5–6.
  - **The method past the folder-trust dialog (`inputs#I2` F1):** the rig `live-pty-arrow.py` read its own pty's
    screen and on 2.1.287 found the default pointer ONE row above the trust row (`default_offset` -1, both starts
    that showed the dialog), typed 1 down arrow, read `selected`, typed 1 Enter — into its own pty only, never a
    desktop key. On the wrapped start the key moved the wheel to the human and `viola release driver` (`ok.wheel`
    `driver`) preceded the send. The CLI then wrote `proj/`'s trust entry in `~/.claude.json` itself (`no-entry` →
    `trusted`), so start 3 showed no dialog. Controls: `rig-controls.py` `controls: 10 of 10 as expected`.
  - **Start 1 (red, 05:32Z):** from the plan's `target/plugin-live-<stamp>/proj/` (inside this work tree, its own git
    root), the trust dialog passed by the rule and the next screen read `other-modal`, the `Yes, allow external
    imports` row: no key (F1), no `session-start` in 90 s, no `mcp.ndjson` in either home; at the stop a SessionEnd
    hook with no SessionStart. The CLI keys the external-import answer on the git root (`security-plan.md:263`, the
    dev-host prerequisite), and the repository's `CLAUDE.md` imports `@.claude/session-handoff.md` — the dialog's
    subject inferred, not read (no screen text kept).
  - **The founder's word after start 1 (`inputs#I4`, relayed by the operator, 2026-10-19T05:36Z):** «Стенд вне
    репо» — a fresh `proj/` outside this work tree on the e2e-home tmpfs (`target/e2e-home/viola-live-plugin-<stamp>/proj/`,
    its own `git init`, no ancestor `CLAUDE.md` read up to the root), a fresh four `claude plugin` commands, the
    remaining 2 starts for the two readings, the arrow rule kept, rig A's plugin removed first (commands 3–4).
  - **The cap (`inputs#I2` F2 «3 старта»):** 3 starts of 3, each ledgered before it (`live-sessions.ndjson`), 1 red
    never re-fired; `claude plugin` commands 4 of 4 in rig A, 4 of a fresh 4 in rig B.
  - **The residual (`evidence/live-round.md` §Rig A's close, §Rig B's close):** `~/.claude/plugins/` per rig 157 756
    files before and after, added 0, removed 0, changed 4 each, all same-size: the CLI's orphaned
    `cache/viola/viola/0.1.0/.mcp.json` and `.orphaned_at` (rewritten by its own install/uninstall; its `.mcp.json` now
    names rig B's `<ihome>`), and one `synced/<id>/` pair (unattributed: other live sessions on the host).
    `~/.claude/settings.json` sha256 prefix `8e7b63f4b538634a` unchanged; the repository root's
    `.claude/settings.local.json` absent throughout; `~/.claude.json` trust entries for both rigs' `proj/` (`no-entry`
    → `trusted`, the CLI's writes on the accepted dialogs); `~/.claude/projects/` 106 → 107 (rig B's transcripts);
    the CLI's MCP log dirs for rig B's `proj/` under `~/.cache/claude-cli-nodejs/`. viola and the rig wrote nothing
    under `~/.claude` or into `~/.claude.json`.
  - **CI:** `ci#39209618442` on `2a3d569123b1e464e64c95193e07e0d27a305417` (the pre-CI commit, HEAD): `verdict: green ·
    checks 15/15 · wall 654 s`, attempt 1, event push (`evidence/operator-pass.md` §5).
  - **Inputs (`inputs.py verify`):** I1 · message · n/a · cited (scope, research, plan) · I2 · message · n/a · cited
    (scope, plan) · I3 · message · n/a · cited (scope, plan) · I4 · message · n/a · cited here (`inputs#I4`, snapped at
    implement, printed UNCITED by verify before this report) · I5 · message · n/a · cited here (`inputs#I5`, snapped at
    this wrap: the operator's wrap direction). Verify: `4 entries — … n/a 4 · uncited 1 · unparsed 0` before I5. No
    entry names a P5 review card in its `why`; the matrix note's card `.andromeda/runs/2026-10-18T22-39-54-phase/p5-card-2.md`
    reads `## Beyond the plan` `none`.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - `architecture.md:96` ([Plugin Scope], PREC ×2 @c976/@c1157, TITLE): "which `viola` MCP server a wrapped session
    then starts and uses … is not measured" — now MEASURED (start 2 above). Not false: owed, and now discharged.
  - `architecture.md:95` ([Deployment / Distribution], EXIT1 @c3093, TITLE): "Whether a registered `viola mcp` that
    exits 1 blocks the CLI's start is unmeasured" — now MEASURED: it does not block (start 3).
  - `security-plan.md:262` (§Data Protection, RESID ×2): "from a rig dir under `target/` that is its own git repo" — the
    round's starts that took readings ran from a `proj/` outside the work tree (`inputs#I4`), because the in-tree rig
    met the external-imports modal (start 1); and the residual now also holds the CLI's two trust entries and one
    `~/.claude/projects/` dir, and the cache's rewrite (same sizes) rather than its creation.
  - The plan's own premise (plan.md Goal, step 6) that a start passes once the trust dialog is accepted — falsified for
    an in-tree `proj/` (no master site: TRUSTKEY unfired, 0 master sites). Disposed: plan-only, recorded in
    `evidence/live-round.md`.
  - MCPEXIT (measured, no master claim found): `viola mcp` ended with the session without a `process-exit{subject:"self"}`
    line (`whome/diagnostics/mcp.ndjson` 3 lines: `process-start` self, the `agents-probe` pair). Search:
    `grep -n -o -E '(every process|each process|process-exit\{subject:"self"|self pair|exit pair)' obs-plan.md` — 4
    hits (`:320`, `:407`, `:832`, `:990`), none stating an `mcp` exit line at session end (`:832` is the panic path).
- **Expected amendments (from plan):**
  - architecture §Established Decisions → [Plugin Scope]: **carried** — Cross-project, the precedence reading; site
    `architecture.md:96` (PREC ×2 + TITLE, 1 hit line; 0 in the other six masters and the registries).
  - architecture §Established Decisions → [Deployment / Distribution]: **carried** — Cross-project, the exit-1
    reading; site `architecture.md:95` (EXIT1 @c3093 + TITLE; the other 15 EXIT1 standing rows are the `exit-1
    refusal` wording of other verbs, no change).
  - security-plan §Data Protection (Accepted residual, `security-plan.md:262`): **carried** — Cross-project, the
    method, the founder's word and the residual; site RESID 1 standing (`security-plan.md:262` ×2) + leaf
    `.claude/docs/security-summary.md:73`. Pattern "Boundary widening the founder answered himself to a P4 fork"
    (`inputs#I2` F1) and his own word «Стенд вне репо» (`inputs#I4`) — the answer card.
  - security-plan §Input Validation (Home resolution) and architecture §Cross-cutting Patterns → Config management —
    the NOT-ratified marks on the `VIOLA_DIR` step and the canonicalisation lift: **carried** — Decisions & corrections
    (the founder's «Ратифицировать», `inputs#I2`; the operator's correction `inputs#I3`; the operator's wrap direction
    `inputs#I5`); sites VDIR 4 standing (`architecture.md:323`, `architecture.md:402` ×3, `security-plan.md:204` ×4,
    `test-plan.md:597` ×3) + leaves `.claude/rules/api.md:31`, `.claude/rules/security.md:23`,
    `.claude/docs/conventions.md:44`, `.claude/docs/security-summary.md:64`, `.claude/docs/services/viola-state.md:6` +
    base `.andromeda/playbook.md:66`.
  - For the wrap's route-resolve (`inputs#I1`): the WATCH re-authored in the watch grammar and the second `CARRY:`
    re-separated by two spaces (`INDETERMINATE: working-route.md:156 — after ONE space`): **carried** — P5's, no master
    site.
- **Coverage of new surfaces:** none — no new external surface, hot-path op or UI element (the rig scripts are
  evidence-only, test-plan §6 "Skipped as untestable").

## Deviations from intent
- **Rig B outside the work tree (plan steps 3–8):** starts 2 and 3 ran from a fresh `proj/` on the e2e-home tmpfs
  with a fresh four `claude plugin` commands, not from `target/plugin-live-<stamp>/proj/`; start 2 took the precedence
  reading and start 3 the exit-1 reading, so no spare remained. Justification: start 1 met the external-imports modal
  from the plan's location; the founder's own word «Стенд вне репо» (`inputs#I4`, relayed verbatim by the operator).
- **A sixth verdict word, `unselected`** (a trust row with no single pointer row: no key, red), beyond plan step 1's
  list; F1's "anything else no key and red" decides it; no start read it.
- **The v1-42 matrix write despite plan step 12** ("Matrix and route records are P5's and the wrap's, never
  implement's"): the implement contract sets `ref` + `implemented` on green for the chunk's claimed cap; the write was
  made (`matrix.py implement`). The operator's wrap direction (`inputs#I5`) names it for this wrap to settle.
- **Waits by bounded scripts:** the 90 s `session-start` wait and start 3's 20 s after the first `absent` read
  (start 3 read `absent` at its first read, no dialog).
- scope record: none — `gate.py scope` clean, 0 recorded.

## Decisions & corrections
- The operator's correction at the /implement invocation: this chunk's cap is 3 starts (`inputs#I2` F2), not the
  previous chunk's 12 the session's dashboard had named — a dashboard figure carried from the wrong chunk.
- The founder's ruling «Стенд вне репо» (`inputs#I4`) after start 1 — the in-tree rig cannot pass the external-imports
  modal; a live rig outside the work tree with no ancestor `CLAUDE.md`.
- Measured: on claude 2.1.287 the folder-trust dialog's default pointer sits one row above the trust row, so one down
  arrow then Enter accepts it (`default_offset` -1 on both starts that showed it) — closes the handoff's open question
  "which row 2.1.287's trust dialog selects by default".
- Measured: a live rig `proj/` inside this repository's work tree meets the `Yes, allow external imports` modal after
  the trust dialog (the repository `CLAUDE.md`'s `@.claude/session-handoff.md` import, keyed on the git root).
- Sweep hazard: `viola last --json` carries `ok.last_assistant_message`, and `viola wait --json` nests the same field
  under `ok.event.data` — a filter dropping `message`/`text` keys passed the assistant text through (one fixed word,
  recorded nowhere); compare that field by name in a script.
- Sweep hazard: the CLI's MCP log under `~/.cache/claude-cli-nodejs/` is ndjson, not one JSON document (`json.load`
  read `Extra data`).
- Guard refusal: the Bash guard refused a `cd` that moved the cwd (re-issued with an absolute path), once.
- `hyprctl locked` read `true` before each start and did not stop the round (the plan's lean; the rig types no desktop
  key).
- The operator's wrap direction (`inputs#I5`): settle the plan-step-12 vs implement-letter matrix write; clippy (entry
  5) pins as a PREREQ; the founder's ratification of the `VIOLA_DIR` home step, canonicalisation included (`inputs#I2`,
  `#I3`), lifts both NOT-ratified marks; his V63 ruling waits for a 0-pending wrap after this one.

## Outcome
- (tests) `rig-controls.py` prints `controls: 10 of 10 as expected` last — **met** (gate `python3
  viola-0.1.0/chunks/…/evidence/rig-controls.py` green).
- (manual, live) the round on claude 2.1.287, inside the cap: 3 starts, each ledgered before it; ≤ 4 `claude plugin`
  commands per rig (4 + 4, the second a fresh count by `inputs#I4`); no red re-fired — **met**. Precedence: whome 1
  `process-start{self}`, ihome 0; the `agents-probe` pair in whome — **met**. One `session-start` / one
  `hook-invoked{session-start}` — **met**. Exit-1: `idle` in one agents read; control exit 1, both streams empty; home
  0770, no `mcp.ndjson` — **met**. Keys per start (s1 1 arrow + 1 Enter, offset -1; s2 the same; s3 none) and the
  wheel/release reading — **met**. Residual recorded — **met**.
- (security, obs) no file under `evidence/` holds a `CLAUDE*` value, an absolute path, prompt or assistant text, a
  session id or a detail line — **met** (grep over evidence and the implement run dir for `/home/`, `/tmp/`, a UUID
  shape, the fixed word, `CLAUDE_`: no hits; `gate.py hygiene` clean).
- (security, layouts) the installed tree: `command` the canonical exe, `args` `["--home", <canonical ihome>, "mcp"]`,
  no hooks, files 0600, dirs 0700; the `marketplace add` argument equals the printed `<dir>` — **met**, both rigs.
- (tests) `tests/cli_plugin_install.rs` green unchanged by binary; CI green on the final HEAD — **met**
  (`ci#39209618442`).
- `verification-matrix.json#v1-42` — implemented (ref the binary gate + `evidence/live-round.md` start 2); its
  acceptance names "a rig whose `proj/` is its own git repo" — rig B is one — **met as written**.
- Gates (implement run `.andromeda/runs/2026-10-19T05-23-09-implement/`, by `run`):
  `cargo fmt --all --check` green · `python3 …/rig-controls.py` green (exit 0, last line) · `cargo build --bin viola`
  green · `bash scripts/agent-run.sh run --integration --filter 'binary(cli_plugin_install)'` green (`"ok":true`, not
  `passed":0`) · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` **not run — defer**
  (zero Rust delta; no void reaches it) · `bash scripts/agent-run.sh run --unit` green, `defer voided —
  .andromeda/master-route.md` · `bash scripts/agent-run.sh run --integration` green, `defer voided — walk-class
  tests/cli_controls_not_disableable.rs` · `bash scripts/agent-run.sh run --e2e` green, `defer voided — walk-class
  tests/contract_fake_agent_drift.rs` · `bash scripts/agent-run.sh pre-push` green, `defer voided — walk-class
  tests/contract_fixture_hygiene.rs` (and again in the operator pass, 87.73 s) · `gate.py hygiene` leg operator —
  `hygiene: clean` twice (`evidence/operator-pass.md` §2) · `git diff --quiet && … git push origin HEAD` leg operator —
  green, history moved `bc2e3bf7→2a3d5691` (§4) · `ci.py conclusion --sha HEAD --wait 1800` leg operator — `verdict:
  green · checks 15/15` (§5). Smoke: skipped — no boot-path / UI-surface change.
- Watches: `run::send::budget::tests::send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 — not
  exercised by a plain `cargo test` this chunk; nextest runs green in `run --unit` (implement entry 6), pre-push ×2 and
  CI `ci#39209618442`; no recurrence (since 2026-10-18-compact-post-condition-ledger-row; its count moves only on a
  plain `cargo test` run, plan Implementation notes).
- Outcome basis: the operator pass ran (pre-CI commit `2a3d5691`, the commit list `2a3d5691` alone); the final HEAD's
  CI run `ci#39209618442` recorded in `evidence/operator-pass.md`; implement's report held whole in this window and
  persisted at `.andromeda/runs/2026-10-19T05-23-09-implement/outcome-2026-10-18-plugin-precedence-live-readings.md`.
- Process hygiene: implement P4's census — every start terminated by its stop file (3 Ctrl-C, exit 0, not killed),
  the `viola mcp` control exited 1 at once, the gate legs left no survivor; `gate.py census` `none running`
  (implement trail). Re-measured at this wrap below.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: bc2e3bf7 (the parent of the oldest pre-CI commit 2a3d5691) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

(no file outside the pipeline's own artifacts changed)
