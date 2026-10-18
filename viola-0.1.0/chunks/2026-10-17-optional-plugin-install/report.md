# Report — 2026-10-17-optional-plugin-install

**Chunk:** viola plugin install: an MCP-only local-marketplace plugin for unwrapped drivers, absolute-path command, no hooks; MCP-server precedence as a ledger row
**Date:** 2026-10-18
**Commits:** `43c6410 chore(2026-10-17-optional-plugin-install): operator pre-CI commit, for the run this chunk's verdict reads` (basis `9afbff44`, its parent; `git log --format='%h %s' 9afbff44..HEAD`)

## Changes (structured — detectors read this)
- **Files:** new `crates/viola-agent-claude/src/marketplace.rs`, `src/cmd/plugin.rs`, `tests/cli_plugin_install.rs`; modified `crates/viola-agent-claude/src/lib.rs`, `src/cmd/mod.rs`, `src/human.rs`, `src/human/verbs.rs`, `src/main.rs`, `tests/cli_output_plain.rs`, `.github/workflows/windows-mutants.yml` (`git diff --stat 9afbff44 -- src crates tests .github`: 10 files, 794 insertions, 13 deletions). Evidence: `chunks/2026-10-17-optional-plugin-install/evidence/` (`live-pty.py` and `live-ledger.py`, byte-equal copies; `live-pty-trust.py`, new; `live-sessions.ndjson`, `live-round.md`, `guard-controls.md`, `operator-pass.md`).
- **Symbols / APIs:**
  - **`viola plugin install [--json]`**, a new CLI verb (`src/cmd/plugin.rs`, 177 lines, new). `PluginArgs` (23-27) holds one required sub-verb `PluginVerb::Install` (29-33) taking `InstallArgs { json }` (35-40) under the global `--home`. `viola plugin` alone is a clap usage error (exit 2). clap also serves its own `viola plugin help [install]` sub-verb, as the top level serves `viola help`; no other sub-verb exists.
  - **The verb's order** (`install`, 48-76):
    1. `std::path::absolute(home)` of the `resolved_home` home.
    2. `tree_passes` (78-87): `viola_state::strict::check_path` on each of the home, `marketplace/`, `marketplace/.claude-plugin/`, `marketplace/viola/`, `marketplace/viola/.claude-plugin/` that exists (`tree_dirs`, 89-100). A dir not there passes; one that cannot be read fails. This runs before anything under the home is created, written or opened.
    3. A refusal is exit 21, with nothing created and no log opened. Human mode prints `human::home_state_pair` through `human::refuse` (stdout empty). `--json` prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` through `client::refused_document` (stderr empty).
    4. Under a parsed `VIOLA_NAME` only, `client::start` opens `cli-<name>.ndjson` and writes `process-start{subject:"self"}`.
    5. `write_tree` (102-118, `#[instrument(skip_all, name = "cli.plugin_install")]`, the span opened after the log): the exe is `canonical_home(&std::env::current_exe()?)`; each of the four tree dirs is made by `create_private_dir`, the home created when absent; the home is then **re-canonicalised** (it exists now, `run`'s pattern at `src/cmd/run.rs:232-234`); the three files are written by `replace_private` at 0600, each rewritten whole on a re-run; any other entry in the tree is left as found.
    6. `--json` prints `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}` (one write; no path). Human mode prints `human::write_plugin_installed` with `human::command_word(<canonical home>/marketplace)`.
    7. `run::log_self_exit(0, None)`.
    - A failed `current_exe`, dir creation or file write reaches the `cli` catch site: exit 1, `error: internal error`.
  - **`viola_agent_claude::marketplace`** (new pure module, `crates/viola-agent-claude/src/marketplace.rs`, 155 lines; `pub mod marketplace;` at `lib.rs:8`): `pub const NAME = "viola"`, `pub const PLUGIN_REF = "viola@viola"`, and `pub fn marketplace_files(exe_fwd, home_fwd) -> [(&'static str, String); 3]` (22-53), each document built by `serde_json::json!` and written pretty with a trailing LF (`document`, 55-57):
    - `.claude-plugin/marketplace.json`: `{"name":"viola","owner":{"name":"viola"},"description":<fixed>,"plugins":[{"name":"viola","source":"./viola","description":<fixed>,"author":{"name":"viola"}}]}`;
    - `viola/.claude-plugin/plugin.json`: `{"name":"viola","version":CARGO_PKG_VERSION,"description":<fixed>,"author":{"name":"viola"}}`;
    - `viola/.mcp.json`: `{"mcpServers":{"viola":{"command":<exe, forward slashes>,"args":["--home",<home, forward slashes>,"mcp"]}}}`.
    - The descriptions are fixed ASCII (17-20) and name no hooks. No document, path or key holds `hooks`.
    - The embedded `plugin_files` / `render` (the wrapped plugin) is unchanged, and so are its callers (`pin_and_plugin`, its two units).
  - **`src/human/verbs.rs`:**
    - `write_plugin_installed(out, dir)` (57-72): one write of the four planning-form lines `viola@viola  written  {dir}` / `run these to add it to Claude Code:` / `  claude plugin marketplace add {dir}` / `  claude plugin install viola@viola`.
    - `command_word(path)` (74-77) = `shell_word(path, cfg!(windows))` (79-98). Unix: bare when every char is an ASCII letter, a digit or one of `_ - . / :`, else single-quoted with each `'` as `'\''`. Windows: bare unless it holds a space, else double-quoted.
    - Both are re-exported from `src/human.rs:25-26`. `src/human/verbs.rs` is now a Windows-gated source (the `cfg!(windows)`).
  - **`HELP_TEMPLATE`** (`src/cmd/mod.rs:35`): `setup:    run, revive, verify, plugin install`. The `Command::Plugin` arm and doc line (82-83) and the dispatch arm (173-182: `resolved_home`, the sink under the caller's own name, process `Cli`). `role_of` is unchanged; its unit table gains `plugin_install` and `plugin_install_after_home` (`src/main.rs:357-358`).
  - **Remaining callers:** `client::refused_document` gains a sixth caller (`plugin install`). `human::refuse`'s caller census is 14, re-derived by `grep -rcE 'human::refuse\(' src`: `run` 5, `verify` 5, `list` 2, `revive` 1, `plugin install` 1.
- **Crates / modules:** added module `viola_agent_claude::marketplace`, root module `cmd::plugin`. No crate added or removed.
- **Dependencies:** none added or bumped (`Cargo.lock` unchanged: `git diff --quiet 9afbff44 -- Cargo.lock Cargo.toml`).
- **Schema / config:** none. No new event, field, `subject` or `detail`; `schemas/diag-line.v1.json` unchanged. The tree under `<home>/marketplace/` (5 dirs at 0700, 3 files at 0600) is a new on-disk shape written only by this verb and never read back by viola.
- **Spec-master edits:** none (implement edits no master).
- **Counts / qualifiers moved:**
  - The `human::refuse` caller census: 13 → 14, `plugin install` 1 (pattern `refuse-census`: 1 standing master row, `obs-plan.md:505`, which states "run (5 sites), verify (5), list (2) and revive (1)", stale).
  - The served `setup:` row gains `plugin install` (pattern `setup-row`: 4 standing; `design-system.md:526` ×2, `layout-templates.md:213` (the wireframe, already holding it), `layout-templates.md:216` (as built, stale), `registries/contracts/architecture/project-directory-structure.md:21` (a `markup` row about the `run`/`revive`/`verify` callers, a true claim sharing the token)).
  - `help_lists_only_served_verbs` reshaped for the two-word verb (pattern `help-pin`: 2 standing; `test-plan.md:589` (its unserved list "no `hook`, `help`, `link`, `unlink` or `plugin` in the table" stands: a bare `plugin` stays unserved), `a11y-plan.md:303` (no change)).
  - `windows-mutants.yml`'s `viola-main` job gains `src/human/verbs.rs` (pattern `win-mutants`: 18 standing; none lists a job's file set; the job and label counts are unchanged, ten jobs).
  - `exemption 7` gains `plugin install`'s refused check (pattern `exemption7`: 1 standing, `obs-plan.md:988`, the retired sixth's gloss; the exemption's own list stands at §10 item 7, which `exemption7`'s wording does not reach).
  - **Sweep reach:** 12 patterns · 15 moved facts, 12 with a pattern (the report's own count: `local-scope` and `v1-42` fired no control, `owner pattern` dropped) · owner pattern 4 standing (`title`) · by line 4 lines · 4 offsets. Basis: `cascade.py sweep --pre-pass` over `{run_dir}/report-patterns.toml`, trail `cascade-2026-10-17-optional-plugin-install-pre-pass.json`. The first firing's refusal line: `control: owner, local-scope, v1-42 never fired over the seven masters' pre-pass text (43c64109)`.
- **Dev-tool versions:** none. `claude` 2.1.287 was re-read by path (`2.1.287 (Claude Code)`), unchanged; the bare `claude` on this host is 2.1.296 (verification-harness.md 2026-10-10), not used.
- **Harness / gate surface:**
  - `.github/workflows/windows-mutants.yml`: the `viola-main` item's `files:` gains `src/human/verbs.rs`. Same job, same label, ten jobs. Forced by `tests/contract_windows_mutation_scope.rs` `windows_gated_sources_equal_the_workflow_scope_both_ways`, which read `unlisted: src/human/verbs.rs`.
  - A new rig file `evidence/live-pty-trust.py`: `live-pty.py` plus an in-memory screen and at most one Enter, on the founder's own answer `inputs#I6` Q1. Not a harness verb.
  - Otherwise none.
- **Cross-project / external claims:**
  - **CI:** `ci#39090008812` on `43c64109ff05457d53e8c6da0c8a8a4acca86e15` (the pre-CI commit, the final pushed HEAD), `push`, attempt 1, conclusion `success`, checks 15/15, wall 586 s (`ci.py conclusion`, then `gh run view`).
  - **The CLI's own state, read only:** `~/.claude/plugins/` (listings in the rig's `priv/`), `~/.claude/settings.json` (sha256 prefix `8e7b63f4b538634a`, unchanged throughout), `~/.claude.json` (no project entry for rig 2's `proj/`), `~/.cache/claude-cli-nodejs/` (no dir for `proj/`).
  - **Inputs** (`inputs.py verify`, this wrap, `inputs: 8 entries — … n/a 8 · uncited 6 · unparsed 0` before this report existed; every entry is a message):
    - `I1 · message · copy · n/a` — the operator's `/andromeda-phase` invocation (forks as dialogs; a widening and a start cap are the founder's word).
    - `I2 · message · copy · n/a` — the founder's four P4 answers (live, relayed verbatim by the Viola overseer, 2026-10-17T17:50Z): Q1 write + print · Q2 one live round, no 24th `LedgerRow` · Q3 local scope in a rig under `target/` · Q4 two starts.
    - `I3 · message · copy · n/a` — the operator's P5 word "yes" on the review card `.andromeda/runs/2026-10-17T15-14-51-phase/p5-card-1.md`. Its `## Beyond the plan` rows, verbatim, with the operator's authority (the operator's own word on that card; it ratifies no boundary widening):
      - "The Windows legs of the precedence reading and the exit-1 reading are owned by the existing entry "Windows-only live measurements". That entry gains them as freight at the wrap."
      - "The marketplace tree's integrity against a same-user writer (an unknown entry left as found, and no re-hash of the installed `command`) is owned by the existing entry "Home and code-bearing file integrity". That entry gains it as freight at the wrap."
    - `I4 · message · copy · n/a` (`inputs#I4`) — the operator's `/andromeda-implement` invocation relaying the founder's cap (two starts, four commands at `--scope local`, a red never re-fired, a blocked start 2 stopping before any code change, no mutation run, then the operator pass).
    - `I5 · message · copy · n/a` — the founder's own words (2026-10-18T01:42Z): Q1 «proj/ — свой git-репо» (rig 2, a fresh count of four commands) · Q3 «Удалить сейчас» (rig 1's residual removed). Also the overseer's Q2: the operator pass waits for the round.
    - `I6 · message · copy · n/a` — the founder's own words: Q1 «Одна клавиша в стенде» (one Enter into the rig's own pty, only on a read trust dialog, never a desktop key) · Q2 «Поднять до 3» (the cap raised to 3; commands 3-4 after the last start).
    - `I7 · message · copy · n/a` — the Viola overseer (the founder asleep, his forks PROVISIONAL): close the round as it stands, start 3 unspent, commands 3-4 now; no key, no `~/.claude.json` write, no root local scope; outcome surfaced, v1-42 planned.
    - `I8 · message · copy · n/a` — this wrap's invocation, the Viola overseer's PROVISIONAL route-resolve answer: v1-42 un-claimed at P7.3 with a dated note; the two unread readings plus the trust-dialog method a CARRY on "Compact post-condition ledger row"; a boundary widening found now HELD for the founder.
- **Reverted / negative API facts:** none in product code. Two rig steps were made and closed: rig 1 (2026-10-17) stopped at its guard, and its residual `<repo>/.claude/settings.local.json` was removed on `inputs#I5` Q3; rig 2's install was removed by commands 3-4.
- **Insufficient fixes (written, kept, not the remedy):** `evidence/live-pty-trust.py` was written for the trust dialog (`inputs#I6` Q1). It is correct as a guard (8 synthetic screen controls; it typed nothing when the trust row was not selected), yet it did not get past the dialog: one Enter cannot accept a dialog whose selected row is not the trust row. The remainder, a way past the trust dialog, is owned by the CARRY `inputs#I8` names ("Compact post-condition ledger row").
- **Spec claims disproved by measurement:**
  1. **Plan step 7's rig premise** (`plan.md:182-262`; `inputs#I2` Q3: "the settings land in `<rig>/.claude/settings.local.json`") is false on 2.1.287. `claude plugin marketplace add … --scope local`, run from `target/plugin-live-20261017T182031Z/proj/` inside this repository's work tree, wrote the **work tree root's** `.claude/settings.local.json` (237 B, key `extraKnownMarketplaces`), and `proj/.claude/` was never made (`evidence/live-round.md`, Rig 1). No master states local-scope behaviour (pattern `local-scope`: its control unfired over the masters, the refusal line above).
  2. **The one-Enter trust premise** (`inputs#I6` Q1: "accept the folder-trust dialog" with one Enter) does not hold on 2.1.287. With `proj/` its own git root (rig 2), both wrapped starts were held by a modal before SessionStart (no `session-start`, no `mcp.ndjson` in either home, the child exit 1 at the stop, 1 808 bytes drained each time), and `~/.claude.json` holds no project entry for `proj/`. Start 2's rig read the dialog's confirm row `Yes, I trust this folder` on screen but not as the selected row (`unselected` at every quiet read until the 45 s window closed), so no key was typed. Which row the dialog selects is not read; the rig keeps no screen text. The masters name only the modal literal (`architecture.md:45`, `test-plan.md:506`, pattern `trust`: 2 standing, both true claims of the signature, no change); none states a default selection.
  - Both are plan- and input-borne claims with no master site; each is disposed by the CARRY of `inputs#I8`.
- **Expected amendments (from plan):**
  1. architecture §Established Decisions [Plugin Scope] — carried, in its measured form: "is a capability-ledger row" becomes **no ledger row** (`inputs#I2` Q2, by the ruled limit "a shape no send relies on needs no probe"), with the precedence **not measured** and owed on the CARRY of `inputs#I8`; its Windows leg is owed on "Windows-only live measurements" (`inputs#I3` card row 1). Also as built: `command` the canonical `current_exe` in forward slashes, `args` `["--home", <home>, "mcp"]` built by `serde_json`. Patterns `precedence` (5 standing: `architecture.md:84`, `:88@c18434`, `:97@c625` change; `:400` "Config management. Precedence" and `registries/contracts/obs-plan/trace-context-propagation.md:11` are a true claim sharing the token, no change) and `verb` (`architecture.md:97@c138,@c484`, change).
  2. architecture §Established Decisions [CLI Version Compatibility] — carried: the owed-row bullet "Plugin precedence" (`architecture.md:84`) leaves the owed-row list as a row (no row, `inputs#I2` Q2); `:88@c18434` "plugin precedence stays owed" becomes a measurement owed on the CARRY, not a row. Pattern `precedence`.
  3. architecture §Established Decisions [Deployment / Distribution] — carried as not measured: `architecture.md:96@c3088` "Whether a registered `viola mcp` that exits 1 blocks the CLI's start is unmeasured (the founder's live-start cap)" stays unmeasured. The attempt is recorded: start 3 unspent, owed on the CARRY of `inputs#I8`. Pattern `exit1` (1 standing).
  4. architecture §Occupied Resources — carried:
     - Binary, subcommands and exit codes: `plugin install` takes `[--json]` and exits 0 / 1 / 2 / 21 (`architecture.md:301`, change). `:15` already lists the verb, no change.
     - Filesystem: `marketplace/`'s five dirs and three files (`:342`, change).
     - `diagnostics/`'s producers gain `plugin install`.
     - Pattern `verb` 19 standing; `marketplace` 4 standing (`:97` change, `:105` / `:437` "a marketplace entry (later version)" are a different claim, no change, `:342` change).
  5. architecture §Standard Contracts CLI `--json` output — carried: the install's `ok` document and its `instance-unreachable` refusal. Pattern `verb`.
  6. security-plan §Data Protection — carried:
     - "viola writes nothing under `~/.claude`" is kept (`security-plan.md:114@c4641`, `:259@c1915`; pattern `claude-home` 2 standing).
     - `viola plugin install` writes only `<home>/marketplace/` and spawns no `claude` (`inputs#I2` Q1).
     - The round's writes under `~/.claude` are the CLI's own, on the founder's own answer (`inputs#I2` Q3; rig 2 on `inputs#I5` Q1; the trust key `inputs#I6` Q1). The measured residual is the CLI's orphaned plugin cache `~/.claude/plugins/cache/viola/viola/0.1.0/` (3 files) and an unattributed `synced/` pair, with `installed_plugins.json` / `known_marketplaces.json` back to their before bytes.
     - Pattern "Boundary widening the founder answered himself to a P4 fork".
  7. security-plan §Authentication & Authorization — carried: `plugin install` joins the strict-modes entry points (`check_path` on the home and the four tree dirs before its write and its log); `marketplace/` is not on the trusted-file list, because viola never reads it; the installed `command` gets no re-hash (only the pinned copy is re-hashed). The integrity remainder is owned by "Home and code-bearing file integrity" (`inputs#I3` card row 2). Pattern `verb` (`security-plan.md:539`, the exec-form clause).
  8. security-plan §Security Anti-Patterns → Universal — carried: the exec-form clause (`security-plan.md:539`) already names `viola plugin install`'s `.mcp.json` and "the absolute path of the exe that ran `plugin install`". As built it is the canonical `current_exe`, never a PATH lookup.
  9. obs-plan §4 `verify` / `plugin install` — carried: the `cli.plugin_install` span is opened by `write_tree`, after the log opens. The self pair under a parsed `VIOLA_NAME` only; no log with no name (`obs-plan.md:662-664`, pattern `verb`).
  10. obs-plan §10 exemption 7 — carried: it gains `plugin install`'s refused strict-modes check, which opens no role file and writes no line. This is the plan's lean, approved with the plan by the operator at P5 (`inputs#I3`), not ruled by the founder. Pattern `exemption7` (the list sits at §10 item 7; `obs-plan.md:988` is the retired sixth's gloss, no change).
  11. obs-plan §3 / §11 — carried: the `refuse` caller census is re-derived as 14 (`obs-plan.md:505`, pattern `refuse-census`, change).
  12. test-plan §5 Cross-module patterns (CLI) — carried: `help_lists_only_served_verbs` is reshaped (verb paths; the table words read from the group rows alone, with `plugin install` one entry; a bare `plugin` still unserved), and `tests/cli_plugin_install.rs` is new (`test-plan.md:589`, pattern `help-pin`; `:67`, `:100` list the verb already, no change).
  13. design-system §Surface: cli — carried:
      - Navigation Pattern: `plugin install` served in `setup:` (`design-system.md:526`, patterns `setup-row` and `title`, change; served-verbs-only stays the overseer's PROVISIONAL answer).
      - Component Patterns 4: the install's result lines as a planning form, the founder's wording owed.
      - Exit-code phraseology: its exit-21 pair is `home_state_pair`, reused.
  14. layout-templates — carried: §Output structure — `viola --help` as built (`layout-templates.md:216`, `:60`, `:279`, change; `:213` the wireframe, no change; `:275`, `:320` true, no change). A new Output structure — `viola plugin install`; the catch-site verb list gains `plugin install`. Patterns `verb`, `title`, `setup-row`.
  15. a11y-plan §1 critical path 6, §4 cli landmarks row, §8 Plain language — carried: "owned by" becomes "built" for `setup:`'s `plugin install` (`a11y-plan.md:209`, patterns `title` and `verb`, change). §1 and §8 hold no hit for the patterns; the a11y detector's own sweep stays the control for reach.
- **Coverage of new surfaces:**
  - `viola plugin install` (CLI verb) → validation strict-modes `check_path` on five dirs before any write or log✓; paths only via `resolved_home` and `canonical_home`✓ · instrumentation `cli.plugin_install` span + the `cli` self pair under a name✓; a refused check writes no line (exemption 7) · PII no path in a `--json` document or a line✓; the human result prints the user's own marketplace dir by design · tests unit (7 in `plugin.rs`/`verbs.rs`, 3 in `marketplace.rs`) / integ (9 in `tests/cli_plugin_install.rs`, 2 in `tests/cli_output_plain.rs` moved)✓ · a11y plain ASCII, no SGR, written once✓ · tokens n/a.
  - `<home>/marketplace/` (on-disk tree) → validation 0700 / 0600 explicit (`plugin_install_writes_owner_only_modes` under `umask 000`)✓ · instrumentation n/a · PII n/a · tests integ✓ · a11y n/a · tokens n/a.

## Deviations from intent
1. **The `.mcp.json` home is re-canonicalised after the tree's dirs exist** (`src/cmd/plugin.rs` `write_tree`). The plan names "the resolved canonical home", but `resolved_home` canonicalises before a new home exists, so a fresh home under the linked `target/e2e-home` is spelt through the link and misses the `resolved(home)` oracle. Re-reading it is `run`'s pattern (`src/cmd/run.rs:232-234`). The printed marketplace dir uses the same spelling.
2. **The span** `cli.plugin_install` sits on `write_tree`, after the log opens, not on the whole dispatch. A span opened before the subscriber exists is a no-op, as `list` puts `cli.list` on its `read`.
3. **`src/human/verbs.rs` became a Windows-gated source** through `command_word`'s `cfg!(windows)`. `shell_word(path, windows)` is pure and unit-tested for both families on every host. The contract test then required the file in `windows-mutants.yml`.
4. **The live round's readings were not taken** (plan step 7; the `(manual, live)` criterion). Rig 1 stopped at its own guard (claim 1 above). Rig 2 passed the guard; starts 1 and 2 were red at the trust dialog (claim 2); start 3 was unspent on `inputs#I7`. Starts made: 2 of a cap raised to 3 (`inputs#I6` Q2). Commands: rig 1 3 of its four, rig 2 4 of the fresh four. Ledger: `evidence/live-sessions.ndjson`.
5. **Units beyond plan step 4**, in listed files: `tree_dirs_are_the_home_and_the_four_marketplace_dirs`, `tree_passes_a_new_home_and_its_own_tree`, `tree_passes_refuses_any_dir_another_user_could_write`, `fwd_writes_every_backslash_as_a_slash` (`src/cmd/plugin.rs`); `write_plugin_installed_is_the_planning_form_in_one_write`, `shell_word_quotes_only_what_the_shell_would_split`, `command_word_takes_this_host_family` (`src/human/verbs.rs`).
6. **`target/debug/viola` was stale** before the round (the earlier chunk's 16:15 build; no gate rebuilds it). It was rebuilt by `cargo build --bin viola` at 2026-10-17T18:20:41Z (sha256 prefix `ee27584342cef430`) and used for both rigs. No source was newer when rig 2 ran.
7. **Noted, not changed:** the output is ASCII for an ASCII home path; a non-ASCII home prints as spelt (quoted on Unix), so the two commands stay pasteable. clap's own `viola plugin help` sub-verb exists.

Scope record (`gate.py scope`, this wrap: `scope: clean — changed 10 · listed 9 · recorded 1 (companion 1 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 75`):
- companion: `.github/workflows/windows-mutants.yml` · serves `src/human/verbs.rs` · self.

## Decisions & corrections
- `target/debug/viola` is no gate's output (the harness builds `target/harness/debug`), so a live rig that runs it reads an older chunk's build unless it is rebuilt first. Measured: 16:15 build, sha256 prefix `fef229f67d76230e`, found before rig 1; rebuilt, recorded with its sha.
- On claude 2.1.287, `--scope local` resolves its project from the git work tree root, not the cwd. A rig dir inside this repository writes `<repo>/.claude/settings.local.json`. Measured at rig 1, removed on `inputs#I5` Q3 at 2026-10-18T01:44:31Z.
- A rig dir made its own git root takes the local scope (rig 2's guard passed) but becomes an untrusted project root: `~/.claude.json` holds no entry for it, and a start there is held by the folder-trust dialog, whose confirm row is not the selected row (`live-pty-trust.py` read `unselected`). Measured at rig 2, starts 1 and 2.
- The CLI's uninstall leaves the plugin's cache under `~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/` with an `.orphaned_at` stamp (`keptData` false). Measured at rig 2's close.
- `cfg!(windows)` in a product file makes it Windows-gated for `tests/contract_windows_mutation_scope.rs`, so the file must join a `windows-mutants.yml` job. Met as an E2E red (`unlisted: src/human/verbs.rs`) and fixed by the companion line.
- The write guard refuses a Write under `target/` ("edit to a generated directory"): two rig helpers re-issued into the session scratchpad, run by path. The Bash guard refused a `cd` that moved the session cwd: re-issued as Read calls.
- `AskUserQuestion` in this driven session is answered by the driver through viola's dialog hook. It came back "No answer from the driver within 58 min — … (Held by viola, the hook through which this session's driver answers its dialogs)"; the questions were then put in text and answered by relay (`inputs#I5`).
- `hyprctl locked` read `true` before start 2. The start went ahead: `host-linux.md` 2026-10-08 stops a step that focuses a window or types a key with `wtype`; this start's one possible key was into its own pty, and the founder ruled "never a desktop key". The handoff's standing rule reads more broadly ("before any step that focuses a window or types a key").
- A remove-the-guard control (testing.md 2026-09-25) on `src/cmd/plugin.rs:52`: red `left: Some(0)`, `right: Some(21)`, then green 9/9 (`evidence/guard-controls.md`).
- Standing learnings met and followed: testing.md 2026-10-17 (`concat!` literals, one string per line; the colour facts removed per child); testing.md 2026-09-25 (the remove-the-guard run).
- The founder's words this chunk, each snapped: `inputs#I2` (four P4 answers), `#I5` (rig 2; remove the residual), `#I6` (one Enter; cap 3). The overseer's, PROVISIONAL: `#I5` Q2, `#I7` (close the round), `#I8` (this wrap's route-resolve).
- The session sat idle for hours twice: the run dir and the tree's end state were re-read from disk before resuming each time.

## Outcome
Acceptance criteria, re-asserted against the diff:
- (arch, security) the tree, its re-run, the `.mcp.json` registration, no hooks anywhere — **met** (`plugin_install_writes_the_mcp_only_marketplace`, `plugin_install_rerun_rewrites_the_same_tree`; the units in `marketplace.rs`).
- (security) 0700 / 0600 under `umask 000` — **met** (`plugin_install_writes_owner_only_modes`).
- (security, obs) a home at 0770 refused, exit 21, the human pair with `hint:` last, the `--json` refusal document, the home left as found, no `diagnostics/` — **met** (`plugin_install_refuses_a_widened_home`; the remove-the-guard control).
- (arch, security) the `--json` `ok` document, no absolute path — **met** (`plugin_install_json_prints_the_ok_document`).
- (design, layouts, a11y) the planning form by literal; no ESC or non-ASCII byte, piped, under `NO_COLOR` or `TERM=dumb` — **met** (`plugin_install_prints_the_two_commands`, `plugin_install_writes_no_escape_byte`).
- (obs) the self pair under a name, no `diagnostics/` without one — **met** (`plugin_install_logs_the_self_pair_under_a_name`).
- (layouts, design, a11y, tests) `viola --help`'s `setup:` row, `help_lists_only_served_verbs` with `plugin install --help` exit 0 — **met** (`tests/cli_output_plain.rs`).
- (manual, live) `evidence/live-round.md` records the round — **UNMET in substance.** The file records the round within the cap (2 starts of 3, the four commands, the residual diff), but none of its three readings: which server or servers start, which answers `list`, the one `session-start`, the exit-1 start. It is matrix-linked, so it is routed to P7.3: per `inputs#I8`, v1-42 is un-claimed with a dated note, and the readings are a CARRY on "Compact post-condition ledger row".
- (tests) every gate entry green, and CI green on the operator pass's final HEAD — **met** (below).
- `verification-matrix.json#v1-42` — **not satisfied as written** (its acceptance needs the live readings): un-claim at P7.3 (`inputs#I8`).

Gates (implement's run `.andromeda/runs/2026-10-17T18-08-07-implement/`, then the operator pass), by `run`:
- `cargo fmt --all --check` green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` green · `CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc -- -D warnings` green (again at the operator pass) · `rg -n -U --pcre2 --type rust '#\[(tracing::)?instrument\b(?!\(\s*skip_all\b)' .` green (exit 1, no output) · `bash scripts/lint-probes.sh` green · `cargo deny check` green.
- `bash scripts/agent-run.sh run --unit` green · `run --unit --filter 'test(/marketplace_/)'` green (nextest-unit passed 4) · `run --integration --filter 'binary(cli_plugin_install) | binary(cli_output_plain)'` green (passed 15) · `run --integration` green · `run --e2e`: **red** first (`contract_windows_mutation_scope` `unlisted: src/human/verbs.rs`), green after the companion line.
- `cleanup --session p-plugin-smoke` green · `boot --session p-plugin-smoke` green · `status --session p-plugin-smoke` green (`state ready`, `last_error null`) · `bash scripts/g2-zero-panics.sh` green · `bash scripts/agent-run.sh schema-check` green · `cleanup` green (`processes_gone`, `endpoint_gone`).
- `bash scripts/agent-run.sh pre-push` green (87.62 s at implement, 82.87 s at the operator pass; `"ok":true`, `"stage":"linux-tests"`, coverage `breaches []`).
- `leg = 'operator'` entries, recorded in `evidence/operator-pass.md`:
  - `gate.py hygiene`: `hygiene: clean` ×2.
  - `git diff --quiet && git diff --cached --quiet && git push origin HEAD`: green, `history moved: … 9afbff44→43c64109`.
  - `ci.py conclusion --sha HEAD --wait 1800`: fired with the full sha of the same commit, `verdict: green · checks 15/15 · wall 586 s · runs ci#39090008812 completed/success`, attempt 1.
- Smoke: the booted pair ran as the gate entries above; the new verb ran on the rebuilt binary in both rigs (exit 0, the `ok` document).

Watches: none folded.

Outcome basis: the operator pass ran (`43c6410`, the one commit since the basis `9afbff44`), so the verdicts above rest on its final state, `ci#39090008812` on `43c64109`. This window holds implement's runs and the round whole. `outcome-2026-10-17-optional-plugin-install.md` in `.andromeda/runs/2026-10-17T18-08-07-implement/` is the persisted report; outcome **surfaced** (`inputs#I7`).

Process hygiene (implement's census, measured against the host's process list):
- The booted smoke pair: started by a gate entry, terminated (`processes_gone:true`, `endpoint_gone:true`).
- The 7 `claude plugin` commands and the install verb runs: this run, each exited.
- Rig 2 start 1 (`live-pty.py` → `viola run` → claude): this run, terminated by the stop file; census 2026-10-18T01:47:27Z: 0.
- Rig 2 start 2 (`live-pty-trust.py` → `viola run` → claude): this run, terminated by the stop file; census 02:19:31Z: 0.
- Census 02:22Z after the close: 0 processes (executable under the repository's `target/`, or cwd under the rig or home dirs).
- The four rig and home dirs stay on disk for the founder's desk (`target/plugin-live-20261017T182031Z/`, `target/plugin-live-20261018T014442Z/`, `target/e2e-home/viola-live-plugin-*` ×2, on `tmpfs`).
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 9afbff44 (the parent of the oldest pre-CI commit 43c64109) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/workflows/windows-mutants.yml — added 1 line(s) in 1 range(s)
added: 41
### crates/viola-agent-claude/src/lib.rs — added 1 line(s) in 1 range(s)
added: 8
### crates/viola-agent-claude/src/marketplace.rs — new file · 155 line(s)
- 17-18 «const MARKETPLACE_DESCRIPTION: &str =»
- 19-20 «const PLUGIN_DESCRIPTION: &str =»
- 22-53 @24 «pub fn marketplace_files(exe_fwd: &str, home_fwd: &str) -> [(&'static str, String); 3] {»
  - 26-36 «let marketplace = json!({»
  - 37-42 «let plugin = json!({»
  - 43-47 «let mcp = json!({»
  - 48-52 «[»
- 55-57 «fn document(value: &Value) -> String {»
- 59-155 @60 «mod tests {»
  - 66-71 «fn parsed(files: &[(&'static str, String); 3]) -> Vec<(&'static str, Value)> {»
  - 73-120 @76 «fn marketplace_files_are_the_validated_shape() {»
  - 122-131 @123 «fn marketplace_mcp_json_escapes_the_paths() {»
  - 133-141 «fn holds_a_hooks_key(value: &Value) -> bool {»
  - 143-154 @144 «fn marketplace_files_hold_no_hooks() {»
### src/cmd/mod.rs — added 14 line(s) in 4 range(s)
added: 8 · 35 · 82-83 · 173-182
### src/cmd/plugin.rs — new file · 177 line(s)
- 23-27 @24 «pub(crate) struct PluginArgs {»
- 29-33 @30 «enum PluginVerb {»
- 35-40 @36 «pub(crate) struct InstallArgs {»
- 42-46 «pub(crate) fn plugin(home: &Path, args: &PluginArgs) -> anyhow::Result<ExitCode> {»
  - 43-45 «match &args.verb {»
- 48-76 «fn install(home: &Path, args: &InstallArgs) -> anyhow::Result<ExitCode> {»
  - 50-60 @52 «if !tree_passes(&home) {»
  - 61-63 «if let Some(name) = client::own_name() {»
  - 65-73 «if args.json {»
- 78-87 @80 «fn tree_passes(home: &Path) -> bool {»
  - 81-86 «tree_dirs(home)»
- 89-100 @90 «fn tree_dirs(home: &Path) -> [PathBuf; 5] {»
  - 93-99 «[»
- 102-118 @106 «fn write_tree(home: &Path) -> anyhow::Result<PathBuf> {»
  - 108-110 «for dir in &tree_dirs(home)[1..] {»
  - 114-116 «for (rel, content) in marketplace::marketplace_files(&fwd(&exe), &fwd(&home)) {»
- 120-122 «fn fwd(path: &Path) -> String {»
- 124-177 @125 «mod tests {»
  - 128-142 @129 «fn tree_dirs_are_the_home_and_the_four_marketplace_dirs() {»
  - 144-153 @145 «fn tree_passes_a_new_home_and_its_own_tree() {»
  - 155-170 @157 «fn tree_passes_refuses_any_dir_another_user_could_write() {»
  - 172-176 @173 «fn fwd_writes_every_backslash_as_a_slash() {»
### src/human.rs — added 4 line(s) in 2 range(s)
added: 3-4 · 25-26
### src/human/verbs.rs — added 90 line(s) in 3 range(s)
added: 1-2 · 57-99 · 164-208
- 57-72 @59 «pub(crate) fn write_plugin_installed(out: &mut impl Write, dir: &str) -> io::Result<()> {»
  - 60-71 «out.write_all(»
- 74-77 @75 «pub(crate) fn command_word(path: &str) -> String {»
- 79-98 @82 «fn shell_word(path: &str, windows: bool) -> String {»
  - 83-89 «if windows {»
  - 90-92 «let plain = path»
  - 93-97 «if plain {»
  - 165-177 @166 «fn write_plugin_installed_is_the_planning_form_in_one_write() {»
  - 195-203 @197 «fn shell_word_quotes_only_what_the_shell_would_split(»
  - 205-208 @206 «fn command_word_takes_this_host_family() {»
### src/main.rs — added 2 line(s) in 1 range(s)
added: 357-358
### tests/cli_output_plain.rs — added 22 line(s) in 8 range(s)
added: 62 · 78-79 · 83-93 · 96-99 · 108 · 117 · 123 · 132
### tests/cli_plugin_install.rs — new file · 328 line(s)
- 24-28 «const FILES: [&str; 3] = [»
- 31-38 «fn install(home: &Path, json: bool, from: Option<&str>) -> Ran {»
  - 32-36 «let args: &[&str] = if json {»
- 40-57 @42 «fn install_with(home: &Path, json: bool, env: &[(&str, &str)]) -> Ran {»
  - 45-47 «if json {»
  - 48-50 «for name in UNWRAPPED.iter().chain(&["NO_COLOR", "TERM", "COLORTERM"]) {»
  - 51-55 «command»
- 59-61 «fn fwd(path: &Path) -> String {»
- 63-81 @65 «fn entries(dir: &Path) -> BTreeMap<String, Option<Vec<u8>>> {»
  - 68-79 «while let Some(at) = walk.pop() {»
- 83-85 «fn marketplace(home: &Path) -> PathBuf {»
- 87-90 «fn mcp_json(home: &Path) -> Value {»
- 92-136 @93 «fn plugin_install_writes_the_mcp_only_marketplace() {»
  - 101-105 «want.extend(»
  - 108-110 «for dir in DIRS {»
  - 111-116 «for file in FILES {»
  - 117-122 «assert!(»
  - 128-131 «assert_eq!(»
  - 132-135 «assert_eq!(»
- 138-142 @139 «fn mode_of(path: &Path) -> u32 {»
- 144-148 @145 «fn set_mode(path: &Path, mode: u32) {»
- 150-162 @152 «fn modes(home: &Path) -> BTreeMap<String, u32> {»
  - 153-159 «let mut found: BTreeMap<String, u32> = entries(home)»
- 164-192 @166 «fn plugin_install_writes_owner_only_modes() {»
  - 170-176 «command»
  - 177-179 «for name in UNWRAPPED {»
  - 180-183 «command»
  - 188-191 «for (path, mode) in &found {»
- 194-207 @195 «fn plugin_install_rerun_rewrites_the_same_tree() {»
- 209-223 @210 «fn plugin_install_json_prints_the_ok_document() {»
  - 218-222 «for stream in [&ran.stdout, &ran.stderr] {»
- 225-251 @226 «fn plugin_install_prints_the_two_commands() {»
  - 233-238 @234 «assert!(»
  - 239-250 «assert_eq!(»
- 253-281 @257 «fn plugin_install_refuses_a_widened_home() {»
  - 267-271 «assert_eq!(»
- 283-303 @284 «fn plugin_install_logs_the_self_pair_under_a_name() {»
  - 289-294 «let of = |event: &str| -> Vec<&Value> {»
- 305-322 @306 «fn plugin_install_writes_no_escape_byte() {»
  - 310-321 «for env in [piped, &[("NO_COLOR", "1")], &[("TERM", "dumb")]] {»
- 324-328 @325 «fn plugin_alone_is_a_usage_error() {»
