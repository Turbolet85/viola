# Fan-out results — wrap of 2026-10-10-statusline-pass-through

Seven doc-agents, one batch. Each section holds the detector's return as it came (entity-decoded; the
probe for leftover entities read 0 on every doc), split into the comment lines stripping removed and the
proposal list. The dispositions (Validate) follow each list.

## architecture

verdict: 20 proposal(s) · comment lines stripped: 2

Stripped (the detector's comment lines, as returned):

```
# Checked, no proposal: no new workspace crate, channel method, event kind, env var, port, socket or listener (report Symbols / APIs and Outcome); the `statusline` hook argument and `budget.json.lock` are already registered; no Tokio and no new library (§Stack holds).
# Not proposed, for the orchestrator's eye: (1) Snapshot envelope says `read_snapshot` has "four product callers" — the report lands a snapshot read in the new arm but states no caller count; (2) Spec claims disproved 5 (revive opens its log before its instance check, read in source, not measured) touches the `viola revive` sentence of the instance-files entry — the report marks it a CARRY (`inputs#I5`).
```

```yaml
proposals:
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem (viola home)
    change: >-
      Add a bullet: `statusline-source.json` — the named statusline source of this home, in the Claude settings shape, read-only to viola (viola never writes it); whenever it exists it is the source `viola run` reads the user's statusline command from at start (bounded at `MAX_FRAME`; a command only from a `statusLine` of type `command`, non-empty, no NUL). It is the per-home redirect the tests plant.
    sidecar: >-
      2026-10-10-statusline-pass-through: registered `<home>/statusline-source.json` (read-only named statusline source) under Occupied Resources → Filesystem.
    rationale: >-
      Report Changes → Schema / config lands `<home>/statusline-source.json` ("read-only to viola, in the Claude settings shape; the source whenever it exists"); Expected amendments records `statusline-source` at 0 sites in architecture ("a new name there"). A new on-disk resource with no registry entry.
    basis: src/cmd/run.rs:448-463
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem (viola home)
    change: >-
      Add a bullet: Read-only, outside the viola home: `<user home>/.claude/settings.json` (`viola_agent_claude::statusline::USER_SETTINGS`), read by `viola run` at start only when the home is the default home (`cmd::default_home`) and holds no `statusline-source.json`; only its `statusLine` command is taken, through the same `MAX_FRAME` bound, and viola never writes it. A home named by `--home` reads no user settings.
    sidecar: >-
      2026-10-10-statusline-pass-through: registered the read-only read of the user's `~/.claude/settings.json` (default home only) under Occupied Resources → Filesystem.
    rationale: >-
      Report Changes → Symbols / APIs lands `USER_SETTINGS = [".claude", "settings.json"]` and `cmd::default_home(user_home)` "shared by `resolve_home` and the source rule"; Outcome: "the one source rule; viola writes neither file — met" and "`start` now also reads `std::env::home_dir()` for the source rule". Architecture registers no product read of a file under the user's `.claude/`.
    basis: src/cmd/run.rs:448-463
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Claude Code integration names (Flags passed to the child)
    change: >-
      The child's arguments are `--plugin-dir <viola home>/plugin/<version>-<hash>` first on every start, then `--settings <instance dir>/settings.json` when a settings override was written (`viola_agent_claude::statusline::SETTINGS_FLAG`; absent on Windows and whenever no override is written), then the caller's arguments. `viola verify`'s Runs C and D carry their own `--settings` literal in `src/cmd/verify/typed.rs`, so the agent crate is not the only product site of that literal.
    sidecar: >-
      2026-10-10-statusline-pass-through: registered the child flag `--settings <instance dir>/settings.json` (after `--plugin-dir`, only when an override was written).
    rationale: >-
      Report Changes → Symbols / APIs: "The child's arguments are now `--plugin-dir <dir>`, then `--settings <instance dir>/settings.json` when an override was written, then the caller's arguments". The registry bullet names `--plugin-dir` and the revive flags only. Spec claims disproved 1 places the literal also at `src/cmd/verify/typed.rs:132` and `:173`, so the entry must not say "nowhere else".
    basis: src/run/mod.rs:237-250
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Claude Code integration names (Per-session settings override)
    change: >-
      Per-session settings override: `instances/<ViolaName>/settings.json` holds `{"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}` with the absolute pinned path, rewritten whole at every start through `replace_private` at 0600, after the first snapshot and before the spawn. It is written only on Unix and only when every character of the pinned path is an ASCII letter, a digit or one of `_ - . / :` (`statusline::override_document`; on Windows it returns `None` for every input); when none is written no `--settings` is passed. It reaches the child as `--settings <instance dir>/settings.json`. That mechanism has no ledger row: it is relied on by the founder's word R (see Capability ledger).
    sidecar: >-
      2026-10-10-statusline-pass-through: settings override entry rewritten as landed — body shape, rewrite at every start at 0600, Unix-only and plain-character rule, `--settings`, no ledger row.
    rationale: >-
      The entry says the override is "passed to the child through the settings-override mechanism recorded in the capability ledger" and implies a write on every OS. Report Changes → Schema / config: "rewritten at every start through `replace_private` at 0600; written only on Unix and only when every character of the pinned path is an ASCII letter, a digit or one of `_ - . / :`"; Counts: `LedgerRow` 17 unchanged; Expected amendments: "relied on with no row, three live readings, no row and no fixture" (founder's word R, `inputs#I4`).
    basis: src/cmd/run.rs:479-489
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Hook contract
    change: >-
      Replace the `viola hook statusline` bullet: it is exempt from both rules, because it is the statusline wrapper, not a Claude Code hook (a hidden arm of `hook`, dispatched on the word before `HookEvent::from_arg`; `HookEvent` keeps nine events). `run` reads the user's command at start from one named source, read-only to viola: `<viola home>/statusline-source.json` whenever it exists, else, only in the default home, the user's `<user home>/.claude/settings.json`; a home named by `--home` reads no user settings. The reader is `viola_agent_claude::statusline::user_command` (a `statusLine` of type `command`, non-empty, no NUL), and `run` records the command in the instance snapshot as `statusline_command`. The arm's order: canonicalise the instance from `VIOLA_DIR`, the instance check (`check_instance`), then the log, stdin through `take(MAX_FRAME + 1)`, the `budget.json` write (only when the payload holds a `rate_limits` object), the snapshot read, the shell-out. The recorded command runs as one argument of `/bin/sh -c` (`statusline::shell_argv`; on Windows it is `None` for every input and nothing is run) with the same stdin, the environment and directory unchanged and stderr discarded, bounded by `STATUSLINE_DEADLINE` = 5 s (PROVISIONAL, `src/cmd/hook.rs`). It prints that command's stdout unchanged through `take(MAX_FRAME)`, and nothing when the command fails, when no command is recorded, when the instance check refuses, or on oversize or malformed stdin; always exit 0 with empty stderr. This is the only shell-out in viola; the shell is relied on with no ledger row (founder's word R, see Capability ledger).
    sidecar: >-
      2026-10-10-statusline-pass-through: Hook contract statusline bullet rewritten as landed — named source rule, the arm's order, `/bin/sh -c` on Unix, the 5 s PROVISIONAL bound, no ledger row.
    rationale: >-
      The bullet says `run` resolves the command "from the user's effective Claude Code settings" and that it runs "through the same shell Claude Code would use for it (a ledger row)". Report Changes: the named source rule (`statusline_source`, `USER_SETTINGS`), `shell_argv(command) -> Option<[&str; 3]>` (`/bin/sh`, `-c`, the command as one argument; `None` on Windows), `STATUSLINE_DEADLINE = 5 s`, PROVISIONAL; Expected amendments lists exactly these facts for this contract; Deviation 1: the instance check runs before the hook's log opens; no ledger row landed (`LedgerRow` 17).
    basis: src/cmd/hook/statusline.rs:43-81
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem (viola home) — the `instances/<ViolaName>/…` entry
    change: >-
      In the instance-files bullet: `settings.json` is "the per-session settings override that wraps the statusline; Unix only, rewritten whole at every start at 0600, absent when the pinned path fails the plain-character rule", and "One reader checks these files before it uses them" becomes two readers: `viola revive` as written, and `viola hook statusline`, which canonicalises the instance and runs the instance check before its log opens; a refusal there is exit 0 with nothing printed, no command run and no `budget.json` write (detail `strict-modes-failed`).
    sidecar: >-
      2026-10-10-statusline-pass-through: instance-files entry — `settings.json` qualified Unix-only and rewritten each start; `hook statusline` added as the second reader that runs the instance check.
    rationale: >-
      Duplicate occurrence of the every-OS override claim retired in the Per-session settings override entry, and a count the chunk moves: report Deviation 1 ("The instance check runs before the hook's log opens") and Outcome (security) "a home another user can write: no command, nothing printed, marker and `budget.json` unchanged, exit 0 — met" make `hook statusline` a second checking reader beside `viola revive`.
    basis: src/cmd/hook/statusline.rs:43-81
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem (viola home) — the `budget.json` entry
    change: >-
      Append to the `budget.json` + `budget.json.lock` bullet, as landed (chunk 2026-10-10-statusline-pass-through): the file is `{"v":1,"five_hour":…,"seven_day":…,"read_at":"<RFC 3339 UTC ms>"}`, replaced whole through `replace_private` at 0600 under an exclusive lock on `budget.json.lock` (`viola_state::budget::write_budget`); it is written only by `hook statusline` and only when the payload holds a `rate_limits` object; a window is the word `"unknown"` or `{used_percentage, resets_at}`, and either field may be `"unknown"`.
    sidecar: >-
      2026-10-10-statusline-pass-through: `budget.json` entry given its landed shape (`v` 1, two windows, `read_at`), 0600 replace under its lock, and the write-only-with-`rate_limits` rule.
    rationale: >-
      Report Changes → Schema / config lands the file's shape and write rule, and Symbols / APIs lands `budget::write_budget(home, &BudgetReading, read_at)` and `budget::BUDGET`. The registry entry carries no `v` key, no mode and not the condition that a payload without `rate_limits` writes nothing. Note for the orchestrator: the landed shape is flat, not the Snapshot envelope.
    basis: crates/viola-state/src/budget.rs:44-59
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Instance snapshot
    change: >-
      Add a field bullet: `statusline_command` is the user's statusline command `run` read from the named source at start (Hook contract); it is omitted when there is none (an additive field; `v` stays 1), no event carries it, and `hook statusline` reads it from the snapshot. `snapshot.json` is not changed by `hook statusline`.
    sidecar: >-
      2026-10-10-statusline-pass-through: Instance snapshot gained the `statusline_command` field bullet (optional, additive, `v` 1).
    rationale: >-
      Report Changes → Symbols / APIs: "`InstanceSnapshot.statusline_command: Option<String>` (omitted when absent, `v` stays 1)"; Outcome (security): "neither `snapshot.json` nor `ledger/stamps.json` changes — met (case 8)". The contract lists the key in `data` but, unlike every other field, gives it no bullet; Expected amendments names this contract.
    basis: crates/viola-state/src/snapshot.rs:346-365
  - detector: D-arch-resources
    severity: warning
    section: §Established Decisions → [Session Liveness]
    change: >-
      In the start order: the first snapshot carries `statusline_command` when the named source gave a command (the `Recorded` argument of `start_state`), and between the first snapshot and the child spawn `run` rewrites the settings override `instances/<ViolaName>/settings.json` (Unix only, see Per-session settings override); "`statusline_command` once written" loses "once written".
    sidecar: >-
      2026-10-10-statusline-pass-through: start order names the settings override write (after the first snapshot, before the spawn) and the snapshot's `statusline_command`.
    rationale: >-
      Duplicate occurrence of "written by `viola run` at start": the decision's start order lists every step and has no override write, and still marks `statusline_command` as "once written". Report Expected amendments: "the override is written after the first snapshot and before the spawn; the per-home source"; Symbols / APIs: `write_override`, "the `Recorded` argument of `start_state`".
    basis: src/cmd/run.rs:479-489
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Cross-cutting Patterns → Config management (Viola home)
    change: >-
      Add to the Viola home sub-bullet: the statusline source follows the home too — `<home>/statusline-source.json` is the source whenever it exists, and only the default home (`cmd::default_home(user_home)`, shared by `resolve_home` and the source rule) falls back to the user's `<user home>/.claude/settings.json`, so a home named by `--home` never reads the user's settings; this is part of the isolation point for tests. `start` reads `std::env::home_dir()` for the rule, the read `resolve_home` already made; the rule has no `config.json` key and no environment variable.
    sidecar: >-
      2026-10-10-statusline-pass-through: Config management records the per-home statusline source rule and `cmd::default_home`.
    rationale: >-
      Report Changes → Symbols / APIs: "`cmd::default_home(user_home)`, shared by `resolve_home` and the source rule"; Schema / config: "No `config.json` key"; Outcome: "no environment variable … met. Beside it: `start` now also reads `std::env::home_dir()` for the source rule, the read `resolve_home` already made". The pattern lists what follows the home and omits the statusline source; Expected amendments names Config management.
    basis: src/cmd/mod.rs:156-159
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Binary, subcommands and exit codes (Test-only binaries)
    change: >-
      `viola-fake-agent` has twelve argv options, not ten: beside the eight for verify's interactive runs and the two for `viola revive`'s tests, two serve the statusline tests — `--settings <file>` and `--statusline-stdin <file>` (only with both, and only when the settings file's `statusLine` command, split on ASCII white space, is an absolute command, the fake agent runs it with the stdin file's bytes and files a `statusline` receipt: the `hook` receipt's fields without `event`). It also takes one mode word, `statusline-echo` (prints its output and files its stdin; its exit is the code after the marker path).
    sidecar: >-
      2026-10-10-statusline-pass-through: fake agent argv options ten → twelve (`--settings`, `--statusline-stdin`), mode word `statusline-echo`, receipt kind `statusline`.
    rationale: >-
      Report Changes → Counts: "fake agent argv options: +2 (`--settings`, `--statusline-stdin`), and one mode word `statusline-echo` (the plan's count: ten → twelve, architecture's own list)" and "fake agent receipt kinds: +1, `statusline`". The registry still says "ten argv options".
    basis: src/bin/viola-fake-agent.rs:335-345
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Repository (the `fuzz/` entry)
    change: >-
      `fuzz/corpus/hook_stdin` has 12 committed seeds, not 10 (added `statusline-epoch` and `statusline-absent`); the `hook_stdin` target now also reaches the statusline payload reader.
    sidecar: >-
      2026-10-10-statusline-pass-through: `fuzz/corpus/hook_stdin` seed count 10 → 12.
    rationale: >-
      Report Changes → Counts: "`fuzz/corpus/hook_stdin`: 10 → 12 seeds (`ls | wc -l`)"; Files → New names both seeds; Outcome (tests): "the `hook_stdin` target reaches the reader". The entry states "`hook_stdin` has 10".
    basis: fuzz/fuzz_targets/hook_stdin.rs
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Repository (the proptest-regressions entry)
    change: >-
      `crates/viola-agent-claude/proptest-regressions/` holds `hook.txt` and `statusline.txt`.
    sidecar: >-
      2026-10-10-statusline-pass-through: registered `crates/viola-agent-claude/proptest-regressions/statusline.txt`.
    rationale: >-
      Report Changes → Files → New lists `crates/viola-agent-claude/proptest-regressions/statusline.txt`; Outcome (tests): "the property at `cases: 512` with its regressions file in the tree". The entry names `hook.txt` alone.
    basis: crates/viola-agent-claude/proptest-regressions/statusline.txt
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: >-
      In the tree comments: `src/cmd/` gains "hook/statusline.rs: the `hook statusline` arm (instance check, `budget.json` write, the bounded shell-out)"; `crates/viola-core/` gains `Reading`, `BudgetWindow`, `BudgetReading`; `crates/viola-state/` gains "the budget reading write (`budget.rs`: `write_budget`)"; `crates/viola-agent-claude/` "statusline parsing" becomes "the pure `statusline` module (payload reading, the settings reader, the override document, the shell argv)".
    sidecar: >-
      2026-10-10-statusline-pass-through: directory structure names `src/cmd/hook/statusline.rs`, `viola-state` `budget.rs`, `viola-core`'s budget reading types and the agent crate's `statusline` module.
    rationale: >-
      Report Changes → Crates / modules: "added `viola_agent_claude::statusline`, `viola_state::budget`, `cmd::hook::statusline` (root bin). No crate added or removed." The keyed tree names none of the three modules nor the new `viola-core` types (`Reading<T>`, `BudgetWindow`, `BudgetReading`). No workspace crate is added, so §Occupied Resources → Workspace crates and §Inherited Defaults need no change.
    basis: crates/viola-state/src/budget.rs:44-59
  - detector: D-arch-decisions
    severity: warning
    section: §Cross-cutting Patterns → Capability ledger
    change: >-
      "Two relied-on shapes have no row yet" becomes three, adding: the statusline pass-through's — the status line payload's `rate_limits` object, the `--settings` override the child honours, and `/bin/sh -c` as the shell for the user's command (read in three live sessions on `claude` 2.1.287 on the Linux dev host at chunk 2026-10-10-statusline-pass-through, `evidence/live-statusline.md`). It landed with no row, no probe and no fixture on the founder's word (R, option B: a manual reading, three live starts; relayed by the operator, 2026-10-10); the ledger stays at seventeen rows.
    sidecar: >-
      2026-10-10-statusline-pass-through: Capability ledger pattern records the statusline pass-through as a third relied-on shape with no row (founder's word R).
    rationale: >-
      The pattern locks "Any new dependence on an undocumented `claude` behaviour is added as a ledger row … never hard-coded as an unconditional assumption" and counts two exceptions. The chunk relies on the statusline payload, the `--settings` override and the shell with `LedgerRow` 17 unchanged; report Expected amendments: "The Capability-ledger sentence follows the founder's word R (`inputs#I4`): relied on with no row, three live readings, no row and no fixture"; Deviation 10 records R option B. Ratified, so warning. The report does not say whether or where a row is owed; the orchestrator decides that wording.
    basis: crates/viola-agent-claude/src/statusline.rs:23-36
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [CLI Version Compatibility] (the Ledger rows list, the Statusline bullet)
    change: >-
      Qualify the bullet "Statusline `rate_limits.*`, the settings-override mechanism, and the shell Claude Code runs a statusline command through on each OS": no such row has landed (seventeen rows); the statusline pass-through relies on the three with no row on the founder's word R (see Capability ledger), with `/bin/sh -c` on Unix and no override and no shell on Windows.
    sidecar: >-
      2026-10-10-statusline-pass-through: the Statusline ledger-row bullet marked as not landed; relied on with no row, Unix only.
    rationale: >-
      Second occurrence of the claim that the statusline shell and the settings override are ledger rows "on each OS". Report: `LedgerRow` 17 unchanged; "On Windows `override_document` and `shell_argv` return `None` for every input"; founder's word R (`inputs#I4`).
    basis: crates/viola-agent-claude/src/statusline.rs:107-111
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [CLI Version Compatibility] (closing paragraph)
    change: >-
      "The statusline rows, the `agents --json` join and plugin precedence land with Epochs 4 and 5" becomes: the statusline pass-through landed at chunk 2026-10-10-statusline-pass-through with no statusline row (founder's word R, see Capability ledger); the `agents --json` join and plugin precedence land with Epochs 4 and 5.
    sidecar: >-
      2026-10-10-statusline-pass-through: closing paragraph no longer promises the statusline rows with the statusline work.
    rationale: >-
      Third occurrence of the same claim, as a forecast: the behaviour is now relied on and the rows did not land with it (report Counts: `LedgerRow` 17 unchanged; Deviation 10: R option B, "no row, no fixture").
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Budget Governor]
    change: >-
      After "The statusline is wrapped only for sessions started by `viola run`, through a per-session settings override that then runs the user's own statusline command unchanged" add, as landed (chunk 2026-10-10-statusline-pass-through): the wrap is Unix only — on Windows no override is written and no `--settings` is passed (the Windows cut, confirmed by the founder, relayed by the operator, 2026-10-10); the user's command is the one of the named source (Hook contract), and a reading is recorded only when the payload holds `rate_limits`.
    sidecar: >-
      2026-10-10-statusline-pass-through: [Budget Governor] records the statusline wrap as Unix only (the founder-confirmed Windows cut).
    rationale: >-
      The locked decision states the wrap for every `viola run` session on a product whose first target is Windows. Report Changes: the override is "written only on Unix", "On Windows `override_document` and `shell_argv` return `None` for every input"; Deviation 10: "the Windows cut confirmed, so S3 is final". A ratified narrowing of a locked decision, so warning.
    basis: crates/viola-agent-claude/src/statusline.rs:87-93
  - detector: D-arch-decisions
    severity: warning
    section: §Infrastructure Patterns → Crate dependency direction
    change: >-
      In the `viola-agent-claude` bullet's "As landed it depends on" list add chrono (the workspace pin `=0.4.45`, features `clock`, `std`; already in the root graph through `viola-state` and the root bin) and the pure `statusline` module (`reading`, `user_command`, `override_document`, `shell_argv`, `SETTINGS_FLAG`, `USER_SETTINGS`; no file, clock or environment read; no new error enum or `AgentError` variant). The chunk added no `viola-state` edge.
    sidecar: >-
      2026-10-10-statusline-pass-through: `viola-agent-claude` as-landed dependencies gain chrono and the pure `statusline` module; still no `viola-state` edge.
    rationale: >-
      Report Changes → Dependencies: "`viola-agent-claude` now names the workspace's `chrono` (`=0.4.45`, features `clock`, `std`) … `Cargo.lock` gains one line and no package"; Crates / modules: "No edge between `viola-agent-claude` and `viola-state`". chrono 0.4.45 is in §Stack, so no new library or runtime; only the key's as-landed dependency list is behind.
    basis: crates/viola-agent-claude/Cargo.toml
  - detector: D-arch-decisions
    severity: warning
    section: §Conventions → Data model conventions (Percentages)
    change: >-
      Percentages: viola's own settings use the `Percent` newtype, 0–100, serialised as a JSON number; an external budget reading's `used_percentage` is a `Reading<f64>` in `viola_core::BudgetWindow`, kept only from 0 to 100 and otherwise the word `"unknown"`.
    sidecar: >-
      2026-10-10-statusline-pass-through: Percentages convention scoped — the budget reading's `used_percentage` is `Reading<f64>`, not `Percent`.
    rationale: >-
      The convention (and the §Stack Domain newtypes row) says percentages are the `Percent` newtype. Report Changes → Symbols / APIs lands `BudgetWindow {used_percentage: Reading<f64>, resets_at: Reading<String>}` in `viola-core`. The report gives no reason for `f64`; the orchestrator may prefer to carry this as a code finding instead of an amendment.
    basis: crates/viola-core/src/lib.rs:243-249
```

### Dispositions — architecture

Numbered in the list's order. Every coordinate the list carries was checked against the report's bullets and its
last section (`coords` check of this run: 0 outside the report). W1, W2, W3 are the three boundary widenings;
they halt as one card by the operator's direction (`inputs#I5` item 1) and apply after its confirm.

1. `statusline-source.json` registered — apply after the card (W3). Check 1: Boundary widening.
2. the user's settings file read by the default home — apply after the card (W3). Check 1.
3. `--settings` among the child's flags — apply after the card (W1); the sentence on the literal is written as
   `inputs#I5` item 4 directs (the constant is the agent crate's; `verify`'s two sites hold the literal).
4. the per-session override as landed — apply after the card (W1). Check 1.
5. Hook contract — apply after the card (W2 the shell, W3 the source); expected amendment 1 (check 5).
6. instance-files entry (dependent) — apply with 4; the second checking reader is Deviations 1.
7. `budget.json` as landed — apply, routine (Accurate this-chunk addition).
8. Instance snapshot's `statusline_command` bullet — apply, routine.
9. [Session Liveness] start order (dependent) — apply with 4 (W1).
10. Config management, the per-home source — apply after the card (W3).
11. fake agent's twelve options and its mode word — apply, routine; the count's rule is architecture's own list.
12. `hook_stdin` seeds 10 → 12 — apply, routine by the count rule: the rule (`ls fuzz/corpus/hook_stdin | wc -l`)
    is written beside the number at every site (test-plan 8 and 9 hold the same count).
13. `proptest-regressions/statusline.txt` — apply, routine.
14. Project directory structure key — apply, routine (the tree's comments already name modules at this grain).
15. Capability ledger, the relied-on shapes with no row — apply; the rows owed land where `inputs#I5` item 5 puts
    them (`rate_limits`: "Budget governor"; the override and the Unix shell: "Paste newline ledger row"; the Windows
    legs: "Windows-only live measurements"). The founder's word R is `inputs#I4`.
16. [CLI Version Compatibility], the Statusline rows bullet (dependent of 15) — apply.
17. [CLI Version Compatibility], closing paragraph (dependent of 15) — apply.
18. [Budget Governor], the wrap on Unix only — apply: the founder's recorded word on the Windows cut (`inputs#I4`)
    settles it, and the Windows half is owed to "Windows-only live measurements".
19. Crate dependency direction, `chrono` and the `statusline` module — apply, routine; expected amendment 4.
20. Percentages convention scoped — apply, routine: plan step 1 (P5-approved) gave `used_percentage` as a JSON
    number or `"unknown"`, and the kept range is 0 to 100. Named to the operator in the wrap's report.

Raised by the orchestrator (checks 5 and 6), each with its own read:
- R1 `read_snapshot`'s product callers, four → five: `hook statusline` calls it (read in `src/cmd/hook/statusline.rs`,
  inside the report's row 127-163). Sites: architecture Snapshot envelope, test-plan §4, obs-plan E5 (obs 6).
- R2 a revived child's arguments: `--plugin-dir`, then `--settings` when an override was written, then `--resume`
  (read: `revive` calls `run`'s `start`, which writes the override and builds the launch; `child_launch` pushes the
  flag before the caller's arguments). Sites: architecture's child-flags bullet, security-plan's revive bullet.
- R3 the revive session id is ratified by the founder (`inputs#I5` item 2) at architecture's child-flags bullet;
  security 11 and 12 carry the security-plan sites. In the card.
- R4 `viola revive` opens its log before its instance check (Spec claims disproved 5; read at `67ab367`:
  `src/cmd/revive.rs` calls `open_wrapper_log`, then `preflight`). The masters say so beside the check, as read in
  source and not measured on `revive`; the fix is the CARRY on "Budget governor" (`inputs#I5` item 3).
- R5 a `--home` session does not show the user's own status line until `statusline-source.json` is planted
  (the named cost of W3's option; resume-point finding 5), said where the source rule lands.

## security-plan

verdict: 15 proposal(s) · comment lines stripped: 8

Stripped (the detector's comment lines, as returned):

```
# security-plan drift proposals, chunk 2026-10-10-statusline-pass-through
# D-security-deps: no drift. The report's Dependencies bullet adds no package to the root graph (chrono =0.4.45 was
# already there; Cargo.lock gains one line), fuzz/Cargo.lock gains 24 packages at the root lockfile's versions, and
# CI supply-chain passed on both pushes. Nothing is on a §Dependency Security ban.
# D-security-auth: library and flow match (replace_private at 0600, check_instance, no token/crypto/secret source);
# the two warnings below record facts the §Authentication row does not yet state.
# Per inputs#I5 item 1, the W1/W2/W3 proposals halt as ONE card citing the founder's recorded words (inputs#I4).
# One fact below is from source, not the report: that `viola revive` launches through `start` (proposal 10).
```

```yaml
proposals:
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → new row: Statusline source (read at `viola run` start)"
    change: >-
      Add a row: at every start `run` reads the statusline source through `Read::take(MAX_FRAME)` (a source over the cap is no command): `<home>/statusline-source.json` whenever it exists (read-only to viola, Claude settings shape), else, only when the home is the default home (`cmd::default_home` of `std::env::home_dir()`, the read `resolve_home` already made), the user's `~/.claude/settings.json` (`viola_agent_claude::statusline::USER_SETTINGS`); a home named by `--home` reads no user settings. Parsing is tolerant and in the agent crate alone (`statusline::user_command`): a command only from a `statusLine` of type `command`, non-empty, no NUL; an absent, unreadable or other-shaped source is no command, with no log line and no span. The command is recorded as `snapshot.json`'s `statusline_command` (omitted when absent, `v` stays 1) and stands in no other file. viola writes neither source file. Boundary widening W3, the founder's word as recorded in inputs#I4 (live in the overseer dialogs of 2026-10-10, "the file in the viola home", relayed verbatim by the operator).
    sidecar: "2026-10-10-statusline-pass-through: §Input Validation gains the statusline-source row (statusline-source.json, else the default home's ~/.claude/settings.json; take(MAX_FRAME); statusLine type command, non-empty, no NUL) — W3, founder's word inputs#I4."
    rationale: >-
      Report Changes → Schema / config ("`<home>/statusline-source.json`: read-only to viola ... the source whenever it exists"), Symbols / APIs (`user_command`, `USER_SETTINGS`, `cmd::default_home`, `statusline_source`, `statusline_command`) and Coverage of new surfaces ("the statusline source read at start → validation ✓ (`take(MAX_FRAME)`; a command only from `statusLine` of type `command`, non-empty, no NUL)"). The validation is present, but §Input Validation has no row for this new external-input surface (the report's own site search: `statusline-source` security-plan 0), and the report's Expected amendments name §Input Validation. Note for the orchestrator: the report shows unit + integ coverage only for this reader, and the section's "Parser surfaces" list does not name it.
    basis: "src/cmd/run.rs:448-463 (statusline_source), src/cmd/run.rs:465-477 (statusline_command), crates/viola-agent-claude/src/statusline.rs:75-85 (user_command), src/cmd/mod.rs:156-159 (default_home)"
  - detector: D-security-input
    severity: escalate
    dependent-of: D-security-input
    section: "§Input Validation → Constants (the `MAX_FRAME` paragraph)"
    change: >-
      Add to the list of readers that share `MAX_FRAME`: the statusline source read at `run` start (`take(MAX_FRAME)`, over the cap is no command), `hook statusline`'s stdin (`take(MAX_FRAME + 1)`, a payload of exactly the cap is taken), and the stdout of the user's statusline command (`take(MAX_FRAME)`).
    sidecar: "2026-10-10-statusline-pass-through: MAX_FRAME's reader list gains the statusline source read, hook statusline's stdin and the statusline command's stdout."
    rationale: >-
      The paragraph enumerates what `MAX_FRAME` is "shared by"; the report's Coverage of new surfaces adds three capped readers (`take(MAX_FRAME + 1)` on the arm's stdin, `take(MAX_FRAME)` on the source, "stdout through `take(MAX_FRAME)`" on the shell-out). Left as is, the enumeration reads as complete and omits them.
    basis: "src/cmd/hook/statusline.rs:127-163 (serve; the cap test at 131-134), src/cmd/hook/statusline.rs:268-279 (drain), src/cmd/run.rs:465-477"
  - detector: D-security-input
    severity: escalate
    dependent-of: D-security-input
    section: "§Threat Model Summary → Data classification → config (includes a stored executable command string)"
    change: >-
      Add to the "Where" list: `<home>/statusline-source.json` (read-only to viola, Claude settings shape) and, for the default home only, the user's `~/.claude/settings.json` are the inputs that decide `snapshot.json`'s `statusline_command` at each start; and say that `instances/<ViolaName>/settings.json` holds `{"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}`, never the user's command.
    sidecar: "2026-10-10-statusline-pass-through: Threat Model config list names statusline-source.json and the default home's ~/.claude/settings.json as the inputs that decide statusline_command (W3)."
    rationale: >-
      The list enumerates the config files that hold or decide an executable command string; the report's Schema / config adds a new one in the home and Symbols / APIs adds the user's settings as the default home's source. Same claim as the primary, restated in the Threat Model's file list.
    basis: "src/cmd/run.rs:448-463"
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → Hook stdin (all hook events + `hook statusline`)"
    change: >-
      State the statusline arm as landed: the word `statusline` (with no `--capture`) is matched before `HookEvent::from_arg`, and `HookEvent` still holds nine events; its stdin goes through `take(MAX_FRAME + 1)` (exactly the cap is taken; over it is `oversize-stdin`), and a payload that is not one JSON object is `malformed-json` — in both cases exit 0, empty stderr, nothing printed and no `budget.json`. `viola_agent_claude::statusline::reading` is tolerant on every field: no `rate_limits` object is no reading and no write; a window that is absent or not an object is the word `"unknown"`; `used_percentage` is kept only from 0 to 100, else `"unknown"`; `resets_at` is epoch seconds or RFC 3339, written as RFC 3339 UTC milliseconds, and a malformed value is `"unknown"` (replacing "parsed with chrono in UTC only"). `budget.json` is written only by this arm, and only when the payload holds a `rate_limits` object.
    sidecar: "2026-10-10-statusline-pass-through: Hook stdin row states the statusline arm as landed (word dispatch before HookEvent, resets_at as epoch seconds or RFC 3339, used_percentage 0–100, budget.json only with rate_limits)."
    rationale: >-
      Report Symbols / APIs ("a new arm of the hidden `hook` verb, dispatched on the word before `HookEvent::from_arg`; `HookEvent` still holds nine events"; `reading(stdin)`: "not one JSON object → `AgentError::Malformed`; no `rate_limits` object → `None`"), Schema / config (`budget.json` "written only by `hook statusline`, and only when the payload holds a `rate_limits` object") and Coverage ("`take(MAX_FRAME + 1)`, one JSON object, every field tolerant"). The row's validation is present; its "Event argument" sentence says `<event>` maps into the closed `HookEvent`, which does not cover the new word, and its `resets_at` sentence does not name the epoch-seconds form the plan's expected amendment calls out.
    basis: "src/cmd/hook.rs:96-98, crates/viola-agent-claude/src/statusline.rs:23-36 (reading), :48-53 (used_percentage), :55-73 (resets_at), src/cmd/hook/statusline.rs:127-163"
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → new row: Statusline shell-out (`viola hook statusline`, Unix only)"
    change: >-
      Add a row: the recorded `statusline_command` is run only by `hook statusline`, only on Unix, as `/bin/sh -c <command>` with the command as one argument (`viola_agent_claude::statusline::shell_argv`; on Windows it returns `None` for every input and no command runs). Order of the arm: canonicalise the instance directory, the instance check, then the log, stdin, the `budget.json` write, the snapshot read, the shell-out. The command gets the arm's environment and directory unchanged and the payload on stdin; its stderr is discarded; its stdout is read through `take(MAX_FRAME)` and printed byte for byte, never parsed; a failing command prints nothing; the wait is bounded at `STATUSLINE_DEADLINE` = 5 s, PROVISIONAL (`src/cmd/hook.rs`; detail `deadline`). No log line and no span carries the command. Not controlled: what the command does (it runs as the user). It remains the only shell-out. Boundary widening W2, the founder's word as recorded in inputs#I4 (confirmed live 2026-10-10, relayed verbatim by the operator); relied on with three live readings, no ledger row and no fixture (his word R, option B).
    sidecar: "2026-10-10-statusline-pass-through: §Input Validation gains the statusline shell-out row (/bin/sh -c, one argument, Unix only, check first, stdout take(MAX_FRAME), 5 s PROVISIONAL bound, stderr discarded) — W2, founder's word inputs#I4."
    rationale: >-
      Report Symbols / APIs (`shell_argv(command) -> Option<[&str; 3]>` (`/bin/sh`, `-c`, the command as one argument); Windows `None`; `STATUSLINE_DEADLINE = 5 s`, PROVISIONAL), Expected amendments (the arm's order; "the command's environment and directory unchanged and stderr discarded") and Coverage ("the shell-out of the user's command → validation ✓ (the instance check first; the command one argument; stdout through `take(MAX_FRAME)`; a 5 s bound)"). The controls are present but no §Input Validation row states them; the "Child process output" row lists only `claude agents --json`, the `--version` probe and verify's PTY runs. Swept: the "only shell-out" sentences (Threat Model config; Anti-Patterns → Code Patterns) and the Hook stdin trust-boundary bullet still hold as written and need no edit.
    basis: "crates/viola-agent-claude/src/statusline.rs:107-111 (shell_argv), src/cmd/hook/statusline.rs:43-81 (the arm), :208-230 (shell_out), :232-256 (run_bounded), :281-298 (wait_bounded), src/cmd/hook.rs:562-565"
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → CLI arguments / stdin (the Home path interim-gap sentence for `viola hook <event>`)"
    change: >-
      Scope the first interim gap to the nine `HookEvent` arms and state that `hook statusline` is outside it: the arm canonicalises its instance directory (a missing directory, or a link to another place, is nothing done; a linked home resolves) and runs `viola_state::strict::check_instance` before its log opens and before any stdin, `budget.json`, snapshot or `statusline_command` use; without a valid `VIOLA_NAME` it writes nothing.
    sidecar: "2026-10-10-statusline-pass-through: CLI arguments row — the hook interim gap (no canonicalise, no strict-modes check) no longer reads as covering hook statusline, which does both first."
    rationale: >-
      The row says "`viola hook <event>` does not canonicalise `VIOLA_DIR` or run the strict-modes check", and the §Authentication row counts `hook statusline` among "every `hook <event>`". The report's Expected amendments give the arm's order as "canonicalise, instance check, then the log, stdin, the write, the snapshot read, the shell-out", Deviation 1 says the instance check runs before the hook's log opens, and the acceptance "a home another user can write: no command, nothing printed, marker and `budget.json` unchanged, exit 0" is met (case 5). As worded the gap sentence over-claims for the new arm.
    basis: "src/cmd/hook/statusline.rs:35-41 (canonical), :43-81 (the arm), :406-452 (the four canonical cases), tests/hook_statusline.rs:291-319"
  - detector: D-security-input
    severity: escalate
    section: "§Data Protection → At rest → Files → Code-bearing artefacts (the `instances/<name>/settings.json` bullet)"
    change: >-
      Say what landed: `instances/<name>/settings.json` is `{"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}`, rewritten whole by `run` at every start through `viola_state::fs::replace_private` at 0600, after the first snapshot and before the spawn, never reused from an existing copy; it is written only on Unix and only when every character of the pinned path is an ASCII letter, a digit or one of `_ - . / :` (`statusline::override_document`; on Windows it returns `None` for every input, so no override is written). The child gets `--settings <instance dir>/settings.json` after `--plugin-dir <dir>` and before the caller's arguments only when an override was written. The flag constant is `viola_agent_claude::statusline::SETTINGS_FLAG`; `src/cmd/verify/typed.rs` holds the same literal at two sites from before this chunk. Boundary widening W1, the founder's word as recorded in inputs#I4 (confirmed live 2026-10-10, relayed verbatim by the operator); the Windows half is cut by his word in the same input.
    sidecar: "2026-10-10-statusline-pass-through: settings.json bullet as landed — Unix only, plain-character rule on the pinned path, passed with --settings after --plugin-dir, no override on Windows — W1, founder's word inputs#I4."
    rationale: >-
      The bullet says the file "is rewritten by `viola run` at every start" with the `viola(.exe)` path, unconditionally. Report Schema / config: "rewritten at every start through `replace_private` at 0600; written only on Unix and only when every character of the pinned path is an ASCII letter, a digit or one of `_ - . / :`"; Symbols / APIs: "On Windows `override_document` and `shell_argv` return `None` for every input" and the new argument order; Coverage: "the settings override and `--settings` → validation ✓ (the plain-character rule on the pinned path)". The plain-character rule is a new validation that no section states. Spec claims disproved 1 and inputs#I5 item 4 bar the sentence "the literal is defined here and nowhere else" (`src/cmd/verify/typed.rs:132` and `:173`). What stands in a `settings.json` left by an earlier start when no override is written is not stated by the report.
    basis: "crates/viola-agent-claude/src/statusline.rs:87-93 (override_document), :95-101 (override_on), :103-105 (is_shell_plain), src/cmd/run.rs:479-489 (write_override), src/run/mod.rs:131-134, src/cmd/verify/typed.rs:132"
  - detector: D-security-input
    severity: escalate
    dependent-of: D-security-input
    section: "§Bootstrap phases → auth-scaffolding-baseline (the `viola run` / `viola-state` rewrite bullet)"
    change: >-
      Qualify "rewrite ... `instances/<name>/settings.json` ... at every start": the override is rewritten at every start on Unix when the pinned path passes the plain-character rule, and is not written on Windows or for a pinned path a shell could misread (§Data Protection, Code-bearing artefacts).
    sidecar: "2026-10-10-statusline-pass-through: Bootstrap bullet on the settings.json rewrite carries the Unix-only and plain-path conditions."
    rationale: >-
      Second occurrence of the retired claim that `settings.json` is rewritten at every start with no condition; report Schema / config ("written only on Unix and only when every character of the pinned path is ...").
    basis: "src/cmd/run.rs:479-489"
  - detector: D-security-input
    severity: escalate
    dependent-of: D-security-input
    section: "§Bootstrap phases → logging-redaction-wire (the Scope bullet)"
    change: >-
      Replace "`statusline_command` in `snapshot.json` and `instances/<name>/settings.json`" with "`statusline_command` in `snapshot.json`": the user's command stands in `snapshot.json` only, and `instances/<name>/settings.json` holds the pinned path and the words `hook statusline`.
    sidecar: "2026-10-10-statusline-pass-through: logging floor's Scope bullet — the user's statusline command is carried by snapshot.json only; settings.json holds no user command."
    rationale: >-
      The bullet states that `settings.json` carries `statusline_command`. Report Schema / config gives the override's whole content as the pinned path plus `hook statusline`, and Coverage says "PII ✓ (the command stands in `snapshot.json` only)". Same retired claim about what `settings.json` holds.
    basis: "crates/viola-agent-claude/src/statusline.rs:87-93"
  - detector: D-security-input
    severity: escalate
    dependent-of: D-security-input
    section: "§Threat Model Summary → Attack surface → Child process spawning and PATH resolution (the `revive` bullet)"
    change: >-
      Give the revived child's arguments as `--plugin-dir <dir>` (the wrapper's, first), then `--settings <instance dir>/settings.json` when an override was written, then `--resume <id>`, `--fork-session` when `--fork` is given, then the words after `--`.
    sidecar: "2026-10-10-statusline-pass-through: revive bullet's child-argument list gains --settings (when an override was written) between --plugin-dir and --resume."
    rationale: >-
      The bullet lists the child's arguments with nothing between `--plugin-dir <dir>` and `--resume <id>`. Report Symbols / APIs: "The child's arguments are now `--plugin-dir <dir>`, then `--settings <instance dir>/settings.json` when an override was written, then the caller's arguments", and `start` is `child_launch`'s one product caller. That `viola revive` launches through `start` was read in `src/cmd/revive.rs`, not stated by the report: the orchestrator should confirm it before applying. The session-id row's "after the wrapper's `--plugin-dir <dir>` and before the words after `--`" still holds.
    basis: "src/run/mod.rs:131-134"
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → Session id as a child argv value (`viola revive`)"
    change: >-
      Replace the "**Not ratified:**" sentence with the ratification: the logged session id as the value of `--resume` on the revived child's command line is ratified by the founder, live in the overseer's dialog of 2026-10-10 (read 16:04:10Z), shown to him as a crossing with two options priced (ratify · only an id the human types), the overseer as relay (inputs#I5 item 2 of chunk 2026-10-10-statusline-pass-through).
    sidecar: "2026-10-10-statusline-pass-through: the revive session-id crossing is ratified by the founder (2026-10-10, overseer as relay, inputs#I5 item 2); the row no longer reads 'Not ratified'."
    rationale: >-
      The row says "his word is owed, and no ratification is recorded here". Report Changes → Cross-project / external claims records input I5 as carrying "the founder's ratification of the revive session id"; inputs#I5 item 2 gives the word and directs that the places saying "not ratified" or "word owed" be brought to it. No code fact changes. The word reaches this pass as an overseer relay written into an operator-directions file, so it is escalated for confirmation, not applied silently.
  - detector: D-security-input
    severity: escalate
    dependent-of: D-security-input
    section: "§Threat Model Summary → Attack surface → Child process spawning and PATH resolution (the `revive` bullet)"
    change: >-
      Replace "The logged id on the child's command line is recorded as built; the founder's word on it as a crossing is owed" with: the logged id on the child's command line is a crossing the founder ratified on 2026-10-10, the overseer as relay (§ Input Validation, the session-id row).
    sidecar: "2026-10-10-statusline-pass-through: Threat Model revive bullet — the session-id crossing reads as ratified, no longer 'word owed'."
    rationale: >-
      Second occurrence of the retired "word owed" claim (inputs#I5 item 2, cited by the report's Changes). The section's only two "owed" / "Not ratified" sentences about the session id are this bullet and the session-id row.
  - detector: D-security-auth
    severity: warning
    section: "§Authentication & Authorization → `~/.viola/` access control (the strict-modes entry-point list, `hook statusline`)"
    change: >-
      At "every `hook <event>` including `hook statusline`", state the arm's check as landed: `hook statusline` runs `viola_state::strict::check_instance` on its canonicalised instance directory before its log opens, because obs init (`viola_obs_init` → `open_role_file` → `create_private_dir(home)`) sets an existing home to 0700 first, so in the other order a home at mode 0770 passes the check (measured). A refusal is `strict-modes-failed`: no command run, nothing printed, `budget.json` unchanged, exit 0. It is none of the dated interim gaps.
    sidecar: "2026-10-10-statusline-pass-through: ~/.viola/ access control row — hook statusline runs check_instance before its log opens (obs init narrows an existing home to 0700 first; measured), and is none of the interim gaps."
    rationale: >-
      The flow matches the row (the check runs; a refusal prints nothing and exits 0), so this is not a mismatch; the row does not state the order, and the report measured that the order decides whether the check can refuse at all: Spec claims disproved 2 ("In that order a home at mode 0770 passes the check ... case 5 red in the plan's order. The arm checks first") and Deviation 1. The report's Expected amendments name §Authentication & Authorization for the check-first order. The same mechanism for `viola revive` (Spec claims disproved 5) is read in source, not measured, and is a CARRY by inputs#I5 item 3, so no amendment is proposed for it.
    basis: "src/cmd/hook/statusline.rs:43-81 (the arm; obs init at 55-60), :514-525, tests/hook_statusline.rs:291-319"
  - detector: D-security-auth
    severity: warning
    section: "§Authentication & Authorization → `~/.viola/` access control (the trusted-file list)"
    change: >-
      Record beside the trusted-file list (`snapshot.json`, `ledger/stamps.json`, the pinned `viola`) that the two statusline sources are not on it: `<home>/statusline-source.json` sits inside the checked home but gets no owner or mode check of its own, and the user's `~/.claude/settings.json`, read only by the default home, lies outside the viola home and under no viola check. Both decide the command `hook statusline` later runs; their controls are the read-only, capped, tolerant read of §Input Validation. Boundary widening W3, the founder's word as recorded in inputs#I4.
    sidecar: "2026-10-10-statusline-pass-through: ~/.viola/ access control row records that statusline-source.json and the default home's ~/.claude/settings.json decide statusline_command and are not on the trusted-file list (W3)."
    rationale: >-
      The row ties trust in `statusline_command` to the strict-modes check on the home, the instance directory and three named trusted files. Report Schema / config and Symbols / APIs add two files that decide that command; Coverage of new surfaces lists only the cap and the shape rule as their validation and states no owner or mode check on either file. No off-spec library or flow is involved; the row is silent on a new input to a code-execution decision, and the plan's expected amendment names "the trusted-file list and `statusline-source.json`".
    basis: "src/cmd/run.rs:448-463 (statusline_source), :465-477 (statusline_command)"
  - detector: D-security-auth
    severity: warning
    dependent-of: D-security-auth
    section: "§Input Validation → Own state files on read (the Integrity sentence on `statusline_command`)"
    change: >-
      Extend "`snapshot.json`'s `statusline_command` is trusted only because of that check (write access to it equals code execution)": the value is what `run` read at start from `<home>/statusline-source.json`, or in the default home from the user's `~/.claude/settings.json`, so write access to the source in use also decides the command; neither source is on the trusted-file list (§ Authentication, `~/.viola/` access control; W3).
    sidecar: "2026-10-10-statusline-pass-through: Own state files row — statusline_command's trust also rests on the source file read at start (W3), not on the home check alone."
    rationale: >-
      Restates the claim the primary amends: that the home strict-modes check is the whole basis of trust in `statusline_command`. Report Symbols / APIs (`statusline_source`, `statusline_command`, the `Recorded` argument of `start_state`) shows the snapshot value now originates in one of two source files.
    basis: "src/cmd/run.rs:448-463"
```

### Dispositions — security-plan

1. Input Validation, the statusline-source row — apply after the card (W3). Check 1: Boundary widening.
2. `MAX_FRAME`'s reader list (dependent) — apply, routine: the three caps are in Coverage of new surfaces.
3. Threat Model config list (dependent) — apply after the card (W3); a verbatim copy is kept current (playbook).
4. Hook stdin row as landed — apply, routine; expected amendment 5 (`resets_at` as epoch seconds).
5. Input Validation, the shell-out row — apply after the card (W2). Check 1.
6. CLI arguments row, the hook interim gap scoped — apply, routine: the arm canonicalises and checks first.
7. Code-bearing artefacts, `settings.json` as landed — apply after the card (W1); the literal's sentence as
   `inputs#I5` item 4 directs.
8. Bootstrap `auth-scaffolding-baseline` (dependent) — apply with 7.
9. Bootstrap `logging-redaction-wire`, where the user's command stands — apply, routine.
10. the revive bullet's argument list — REJECTED before the checks: it rests on a source read the report does not
    carry (the detector says so itself). Its fact is raised by the orchestrator as R2 above, with its own read.
11. the session-id row, "Not ratified" → ratified — apply after the card (`inputs#I5` item 2, the founder's word
    relayed by the overseer). Check 1: what ratifies a widening.
12. the Threat Model revive bullet, "word owed" (dependent of 11) — apply with 11.
13. `~/.viola/` access control, the arm's check before its log — apply, routine; Spec claims disproved 2.
14. the trusted-file list and the two sources — apply after the card (W3).
15. Own state files row (dependent of 14) — apply with 14.

Rejected for a source the report does not carry: 1 (proposal 10).

## design-system

verdict: proposals: [] · comment lines stripped: 1

Stripped (the detector's comment lines, as returned):

```
# D-design-tokens: no drift. All five Coverage entries in the report (report.md lines 172-189) carry `tokens n/a`; none carries `hardcoded✗`. The chunk renders no new UI: Expected amendments says "design-system, layout-templates: none expected — holds: the chunk adds no line, hint, refusal or `verify` row" (report.md lines 167-168), and Outcome records "(design, layouts) no line of viola's, no byte added, no verb in `--help` — met" (line 274).
```

### Dispositions — design-system

None to disposition. The plan expected none (check 5 holds).

## layout-templates

verdict: proposals: [] · comment lines stripped: 7

Stripped (the detector's comment lines, as returned):

```
# D-layout-surface: no drift. The report's Changes add no user-facing surface or region of viola's.
# - Symbols / APIs: "No new CLI verb or flag of `viola`"; `viola hook statusline` is a new arm of the hidden `hook` verb.
# - Expected amendments: "design-system, layout-templates: none expected — holds: the chunk adds no line, hint, refusal or `verify` row."
# - Coverage of new surfaces: the settings override "adds no viola byte to the terminal, three OSes".
# Considered and not proposed: the cli §Component — Header / banner sentence "`viola hook` / `viola mcp` have no human surface".
# `hook statusline` prints only the user's own command stdout byte for byte, and nothing without a recorded command,
# so there is no viola-authored output to wireframe and I read the sentence as still holding.
```

### Dispositions — layout-templates

None to disposition. The plan expected none (check 5 holds).

## test-plan

verdict: 19 proposal(s) · comment lines stripped: 14

Stripped (the detector's comment lines, as returned):

```
  # ---- claim 1: "the statusline settings source is an open question; Path 6's pass-through bullets are blocked"
  # ---- claim 2: "the pass-through holds on every OS"
  # ---- claim 3: "the statusline `resets_at` property joins with Statusline pass-through" (not yet landed)
  # ---- claim 4: "`hook_stdin` holds 10 synthetic seeds"
  # ---- claim 5: "the fake agent has ten argv options; `statusline-echo` and reading `settings.json` are not built"
  # ---- claim 6: seed rules for the new surface
  # ---- claim 7: "`settings.json` is rewritten on start" (on every OS)
  # ---- runner configuration
# D-tests-obs-harness: no drift. The report's Harness / gate surface says "none under `scripts/`, `crates/viola-e2e`, `.github`"; `ObsEvent` stays 19 and `hook-decision` detail codes 7; the arm's role lines carry no `corr`, which the Log format key already gives for a non-dialog hook; the Status endpoint shape key is untouched. obs-plan holds no `statusline-echo` / `statusline-source-unresolved`, so the key edit proposed above has no twin to move.
# D-tests-coverage, the tier read itself: every new symbol the report lists carries unit tests, the arm and the source rule carry root integration cases, the reader carries the property and the fuzz seeds; the one tier gap is Path 6's harness-session scenario, recorded in the first proposal.
# Not proposed, outside what the report states (for the orchestrator's own read):
#  - §4 viola-state, "The four other snapshot readers (`src/cmd/client.rs`, `run`'s collision check, `src/cmd/hook.rs` twice) still call `read_snapshot`": the report's arm order names a snapshot read in `hook statusline`, so a fifth reader exists, but the report does not say which read function it calls.
#  - `tui_passthrough` gained a case that binds a `stamped` value (tests/tui_passthrough.rs:263-315); the report does not say whether it drives `viola verify`, so its place in the verify-driven list is unread.
#  - The key file's "`Instant::now() + WITHIN` reads 26 sites in 19 files" stands: the report measures 26 → 26 and gives no file count.
```

```yaml
proposals:
  - detector: D-tests-coverage
    severity: warning
    section: >-
      §6 E2E Test Strategy → Scenario: Path 6 — budget governor (Steps, step 1)
    change: >-
      Replace "The settings source it is written to is a §12 open question. Until that is resolved, only the pass-through bullets (…) are blocked." with: the source is named — `<home>/statusline-source.json` (the Claude settings shape, read-only to viola) whenever it exists, else in the default home the user's `.claude/settings.json`, and in any other home none; the pass-through bullets (marker stdout in step 2, the unchanged marker file in step 6) landed with chunk 2026-10-10-statusline-pass-through as root tests on a planted source — `tests/hook_statusline.rs` cases 1 and 5 and `tests/cli_instance_state.rs` `path6_wrapped_statusline_prints_the_user_output_unchanged`, through `tests/support/home.rs` `plant_statusline_source`, `statusline_marker` and `statusline_echo_command`; `agent-run boot --statusline-echo` is not built (the plan's cut to the route entry "Budget governor"), so the harness-session form of this step waits for that entry.
    sidecar: >-
      §6 Path 6 step 1: the statusline source is named (`<home>/statusline-source.json`, default-home fallback to the user's settings); the pass-through bullets landed in root `hook_statusline` / `cli_instance_state` on a planted source; `boot --statusline-echo` is owed to "Budget governor" (was: source a §12 open question, bullets blocked).
    rationale: >-
      Report Schema / config names `<home>/statusline-source.json` ("the source whenever it exists"), Symbols / APIs names `statusline_source`, `cmd::default_home` and `USER_SETTINGS = [".claude", "settings.json"]`, and Harness / gate surface lists the three plant helpers and says `boot --statusline-echo` and `statusline-source-unresolved` "are not built (the plan's cut to "Budget governor")". Outcome: "(tests) Path 6's signal under the fake agent — met (cases 1 and 5)". Tier: §2 mandates E2E on a harness-booted session for every critical path and Path 6's Cleanup is harness `cleanup`; the pass-through signal landed instead in root binaries the report's gate ran under `run --integration`, so the scenario must record where it stands and what is still owed.
    basis: >-
      tests/cli_instance_state.rs:476-519 (@481); tests/hook_statusline.rs:181-231 and 291-319; tests/support/home.rs:269-278
  - detector: D-tests-coverage
    severity: warning
    section: >-
      §3 → 5-command implementation (`boot`, Command body — the `--statusline-echo` sentences)
    change: >-
      Replace "That source is still a §12 open question. Until arch names it, `--statusline-echo` exits 2 with `reason:"statusline-source-unresolved"`, and Path 6's pass-through bullets are blocked. They are never written against a guessed source." with: the source `run` reads is named, `<home>/statusline-source.json` in the session home (read-only to viola, the source whenever it exists); `--statusline-echo` itself is not built — the plan's cut to the route entry "Budget governor" — and until that entry lands it is a usage error like every unbuilt flag (§3 Exit codes); no `reason:"statusline-source-unresolved"` was built; Path 6's pass-through bullets are no longer blocked, they landed in root tests on a planted source (§6 Path 6 step 1).
    sidecar: >-
      Key file (`boot` Command body): the statusline source is named; `--statusline-echo` is unbuilt and owed to "Budget governor"; the `statusline-source-unresolved` reason does not exist (was: source open, the flag exits 2 with that reason).
    rationale: >-
      Same retired claim as the Path 6 step 1 proposal, in the keyed contract. Report Harness / gate surface: "`boot --statusline-echo` and `statusline-source-unresolved` are not built (the plan's cut to "Budget governor")", and "none under `scripts/`, `crates/viola-e2e`, `.github`". The usage-error wording for the unbuilt flag is test-plan's own §3 Exit-codes rule, not a report fact: the report says only "not built". Checked for D-tests-obs-harness: obs-plan and its key files hold no `statusline-echo`, `statusline-source` or `statusline-source-unresolved`, so this edit is not one-sided.
    dependent-of: D-tests-coverage

  - detector: D-tests-coverage
    severity: warning
    section: >-
      §6 E2E Test Strategy → Scenario: Path 6 — budget governor (Verification signal, first bullet)
    change: >-
      Qualify "Statusline stdout equals the marker byte-for-byte, with exit 0 and empty stderr": the marker equality is Unix's, where `run` writes the instance's `settings.json` override and the arm runs the recorded command as `/bin/sh -c <command>` under a 5 s bound (PROVISIONAL); on Windows `run` writes no override and the arm has no shell to run (`override_document` and `shell_argv` return `None` for every input); exit 0 with empty stderr is not OS-bound; `budget.json` is written only when the payload holds a `rate_limits` object.
    sidecar: >-
      §6 Path 6 verification: the statusline pass-through is Unix-only (no override and no shell-out on Windows); `budget.json` only with a `rate_limits` object (was: unqualified).
    rationale: >-
      Report Symbols / APIs: "On Windows `override_document` and `shell_argv` return `None` for every input"; Schema / config: the instance `settings.json` is "written only on Unix", and `budget.json` is written "only when the payload holds a `rate_limits` object"; Coverage of new surfaces: the shell-out's tests are "unit (Unix) + integ + three live readings"; Deviations 10: "the Windows cut confirmed". The scenario states the pass-through with no OS bound.
    basis: >-
      crates/viola-agent-claude/src/statusline.rs:87-93 and 107-111; tests/hook_statusline.rs:386-405
  - detector: D-tests-coverage
    severity: warning
    section: >-
      §1 Test Scope Summary → Critical paths → budget governor (the Path sentence and the "Statusline stdout equals the user command's stdout" bullet)
    change: >-
      Add to "passes the user's statusline stdout through unchanged" and to "Statusline stdout equals the user command's stdout (empty on failure), exit 0": on Unix only — on Windows `run` writes no `settings.json` override and the arm runs no command; and `budget.json` is written only when the payload holds a `rate_limits` object.
    sidecar: >-
      §1 Critical paths (budget governor): pass-through qualified as Unix-only; `budget.json` only with a `rate_limits` object.
    rationale: >-
      Second occurrence of the unqualified pass-through claim retired by the Path 6 verification proposal; same report facts (Symbols / APIs on Windows `None`, Schema / config "written only on Unix").
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: >-
      §1 Test Scope Summary → Coverage scope → the entity "`viola hook <event>` and `viola hook statusline`"
    change: >-
      "…and write `rate_limits` → `budget.json` before passing the statusline through" gains: the pass-through on Unix only (on Windows the arm runs no command), and the write only when the payload holds a `rate_limits` object.
    sidecar: >-
      §1 Coverage scope (hook entity): the statusline pass-through is Unix-only; the `budget.json` write needs a `rate_limits` object.
    rationale: >-
      Third occurrence of the same unqualified pass-through claim; same report facts as the Path 6 verification proposal.
    dependent-of: D-tests-coverage

  - detector: D-tests-coverage
    severity: warning
    section: >-
      §6 E2E Test Strategy → Property suite (the hook stdin parser bullet)
    change: >-
      Replace "statusline JSON with arbitrary `resets_at` joins with "Statusline pass-through"" with: the statusline reader (`viola_agent_claude::statusline::reading` over arbitrary JSON and `resets_at`; landed at `cases: 512` with `crates/viola-agent-claude/proptest-regressions/statusline.txt` committed, chunk 2026-10-10-statusline-pass-through; its strategy draws whole seconds across the date range, because as first written it stayed green on a stub).
    sidecar: >-
      §6 Property suite: the statusline `resets_at` property landed at `cases: 512` with its regressions file (was: joins with "Statusline pass-through").
    rationale: >-
      Report Outcome: "(tests) the property at `cases: 512` with its regressions file in the tree — met"; Files, New: `crates/viola-agent-claude/proptest-regressions/statusline.txt`; Deviations 8: "The property's strategy gained whole seconds across the date range: as first written it stayed green on the stub". The bullet still reads as a future join.
    basis: >-
      crates/viola-agent-claude/src/statusline.rs:439-457 (strategy 398-421, config 388-396)
  - detector: D-tests-coverage
    severity: warning
    section: >-
      §6 E2E Test Strategy → Scenario: Path 6 — budget governor (Steps, step 2, the parenthesis)
    change: >-
      "…belong to the §6 Property suite's statusline `resets_at` property, which joins with "Statusline pass-through"" becomes "…belong to the §6 Property suite's statusline `resets_at` property (landed at chunk 2026-10-10-statusline-pass-through)".
    sidecar: >-
      §6 Path 6 step 2: the statusline property is landed, no longer a future join.
    rationale: >-
      Second occurrence of the "joins with Statusline pass-through" claim retired by the Property suite proposal; same report facts.
    dependent-of: D-tests-coverage

  - detector: D-tests-coverage
    severity: warning
    section: >-
      §6 E2E Test Strategy → Property suite (the cargo-fuzz paragraph)
    change: >-
      "`hook_stdin` (the `viola hook` stdin parser, 10 synthetic seeds)" becomes "`hook_stdin` (the `viola hook` stdin parser and, since chunk 2026-10-10-statusline-pass-through, the statusline reader; 12 synthetic seeds, the two added `statusline-epoch` and `statusline-absent`)".
    sidecar: >-
      §6 Property suite: `hook_stdin` corpus 12 seeds and the target reaches the statusline reader (was 10).
    rationale: >-
      Report Counts: "`fuzz/corpus/hook_stdin`: 10 → 12 seeds (`ls | wc -l`)"; Files, New: `fuzz/corpus/hook_stdin/statusline-epoch`, `fuzz/corpus/hook_stdin/statusline-absent`; Outcome: "(tests) the `hook_stdin` target reaches the reader; corpus replays green"; gate `run --fuzz-replay` green, 5 targets.
    basis: >-
      fuzz/fuzz_targets/hook_stdin.rs:1-3, 9, 17
  - detector: D-tests-coverage
    severity: warning
    section: >-
      §3 → 5-command implementation (`run`, the `--fuzz-replay` bullet — the per-target seed list)
    change: >-
      "`hook_stdin`: 10 synthetic seeds" becomes "`hook_stdin`: 12 synthetic seeds".
    sidecar: >-
      Key file (`run --fuzz-replay`): `hook_stdin` 12 synthetic seeds (was 10).
    rationale: >-
      Second occurrence of the seed count retired by the Property suite proposal; report Counts: "`fuzz/corpus/hook_stdin`: 10 → 12 seeds".
    dependent-of: D-tests-coverage

  - detector: D-tests-coverage
    severity: warning
    section: >-
      §7 Test Data & Fixtures → Fake agent (the Modes bullet)
    change: >-
      "ten argv options (no env), eight of them for verify's four interactive runs and their tests … and two for `viola revive`'s tests" becomes twelve: the same ten, and two for the statusline tests, `--settings <file>` and `--statusline-stdin <file>` (the settings command runs only with both options and only when it is absolute, with the file's bytes on its stdin); beside them one mode word, `statusline-echo` (it prints its output and files its stdin, and its exit is the code after the marker path); and "`statusline-echo` and `agents --json` land with their consumer chunks" becomes "`agents --json` lands with its consumer chunk; `statusline-echo` landed with chunk 2026-10-10-statusline-pass-through".
    sidecar: >-
      §7 Fake agent Modes: twelve argv options (`--settings`, `--statusline-stdin` added) and the mode word `statusline-echo`, landed (was ten options, `statusline-echo` owed).
    rationale: >-
      Report Counts: "fake agent argv options: +2 (`--settings`, `--statusline-stdin`), and one mode word `statusline-echo` (the plan's count: ten → twelve, architecture's own list)". The behaviour words are the landed tests' names in the report's New text: `fake_statusline_runs_only_with_both_options_and_only_an_absolute_command`, `fake_statusline_echo_exit_is_the_code_after_the_marker_path`, `fake_statusline_stdin_runs_the_settings_command_with_the_file_s_bytes`, `fake_statusline_echo_mode_prints_its_output_and_files_its_stdin`.
    basis: >-
      src/bin/viola-fake-agent.rs:1333-1367 and 1320-1331; tests/cli_fake_agent.rs:913-944 and 946-981
  - detector: D-tests-coverage
    severity: warning
    section: >-
      §7 Test Data & Fixtures → Fake agent (the hook-commands bullet, its last sentence)
    change: >-
      "Reading `settings.json` lands with `statusline-echo`." becomes: given `--settings <file>` it reads that file's `statusLine.command`, splits it on ASCII white space, and — only when `--statusline-stdin <file>` is given too and the command is absolute — runs it with that file's bytes on stdin and writes a `statusline` receipt (landed with chunk 2026-10-10-statusline-pass-through).
    sidecar: >-
      §7 Fake agent: reading the `--settings` file's `statusLine.command` is landed (was: lands with `statusline-echo`).
    rationale: >-
      Second occurrence of the "not built yet" claim retired by the Modes proposal. Report Counts (the two options, the `statusline` receipt kind) and New text: `statusline_words` reads `doc["statusLine"]["command"]`, with the unit `fake_statusline_words_are_the_command_split_on_ascii_white_space` and the root case `fake_statusline_settings_without_a_stdin_file_runs_nothing`.
    basis: >-
      src/bin/viola-fake-agent.rs:335-345 (@337) and 1298-1318; tests/cli_fake_agent.rs:902-911
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: >-
      §7 Test Data & Fixtures → Fake agent (the receipt-kinds list)
    change: >-
      Add one kind after `hook`: `statusline` {the `hook` kind's fields without `event`: `command_absolute`, `ran`, plus `exit_code`, `stderr_len`, `stdout_hex` and `stdin_hex` when it ran}; tests wait for it through `tests/support/fake.rs` `wait_statusline`.
    sidecar: >-
      §7 Fake agent receipt kinds: `statusline` added (the `hook` receipt's fields without `event`).
    rationale: >-
      The list is the closed set of receipt kinds and is now one short. Report Counts: "fake agent receipt kinds: +1, `statusline` (the `hook` receipt's fields without `event`)"; Harness / gate surface: `fake::wait_statusline`.
    basis: >-
      tests/support/fake.rs:78-85 (@80); src/bin/viola-fake-agent.rs:1369-1398
    dependent-of: D-tests-coverage

  - detector: D-tests-coverage
    severity: warning
    section: >-
      §7 Test Data & Fixtures → Seed strategies (a new row after `budget.json`)
    change: >-
      New row — Data type: the statusline source (`<home>/statusline-source.json`, or the user's `.claude/settings.json` for a default home); Strategy: written by the test, never by viola, which only reads it — `tests/support/home.rs` `plant_statusline_source(home, command)` with the command from `statusline_echo_command(marker, extra)` and the marker at `statusline_marker(home)`; a default-home case takes `TestHome::default_of_user` and `Wrapper::boot_as_user`, which sets `HOME` (on Windows `USERPROFILE`) on that one child; Lifecycle: per-test.
    sidecar: >-
      §7 Seed strategies: a row for the statusline source — test-planted through `plant_statusline_source`; default-home cases through `TestHome::default_of_user` / `Wrapper::boot_as_user`.
    rationale: >-
      The table has no row for the one input of the new path that a test writes by hand, beside §11's ban on hand-written state. Report Schema / config: `<home>/statusline-source.json` is "read-only to viola"; Outcome: "viola writes neither file"; Harness / gate surface: "Root test support gained `TestHome::default_of_user`, `Wrapper::boot_as_user` (sets `HOME`, or `USERPROFILE` on Windows, on that one child), `plant_statusline_source`, `statusline_marker`, `statusline_echo_command`".
    basis: >-
      tests/support/home.rs:269-278 (@272), 280-283, 289-309 (@293) and 185-192 (@188)
  - detector: D-tests-coverage
    severity: warning
    section: >-
      §7 Test Data & Fixtures → Seed strategies (the row "On-disk product state", its `seed_conpty` exception)
    change: >-
      Add to the Windows x64 `seed_conpty` exception: a home the seed made is not one viola created, and a first boot and stop does not change that — `hook statusline`'s instance check refuses it (three cases in CI run 38080061631; the refused home's DACL was not read); a case that needs a home viola created on every OS starts from `stamped_home`, as the every-OS `hook_statusline` cases that run the arm's whole course do.
    sidecar: >-
      §7 Seed strategies: on Windows x64 only a stamped home is one viola created; cases that run `hook statusline`'s whole course start from `stamped_home`.
    rationale: >-
      Report Spec claims disproved 3: the plan's "a home is "one viola has already created" after "a first boot and stop"" is false on Windows x64 — "`Wrapper::boot` calls `seed_conpty`, which makes the home with `create_dir_all`. CI run `38080061631`: three cases refused by the arm's check from such homes; the stamped-home case passed. The refused home's DACL itself was not read." Deviations 9: the three every-OS cases take the `stamped_home` fixture. The report's Expected amendments names this seed rule for §7 Seed strategies; the key file's `boot` step 2 already states the kin rule for harness homes ("A home pre-created by the harness would … fail the Windows strict-modes check").
    basis: >-
      tests/hook_statusline.rs:115-148 (@123) and 426-459
  - detector: D-tests-coverage
    severity: warning
    section: >-
      §5 Integration Test Strategy → Setup / teardown lifecycle (the Windows x64 `seed_conpty` carve-out)
    change: >-
      After "so for them the test creates the home before viola runs" add: such a home fails `hook statusline`'s instance check on Windows (as measured at chunk 2026-10-10-statusline-pass-through, CI run 38080061631, three cases refused; the home's DACL not read), so a case that drives a check-guarded path on every OS starts from `stamped_home` (§7 Seed strategies).
    sidecar: >-
      §5 carve-out: a seeded Windows home is refused by `hook statusline`'s instance check; such cases start from `stamped_home`.
    rationale: >-
      Second site that states what a seeded home is; same report facts as the Seed strategies proposal (Spec claims disproved 3, Deviations 9). The carve-out's existing sentences are not contradicted, the measured consequence is missing.
    dependent-of: D-tests-coverage

  - detector: D-tests-coverage
    severity: warning
    section: >-
      §6 E2E Test Strategy → Scenario: Path 1 — `run` start sequence (step 5 and Verification signal)
    change: >-
      Step 5's `settings.json` sentinel is a Unix step, and the Verification signal gains: on Unix the restart rewrites the instance's `settings.json` whole at 0600 as `{"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}` with the absolute pinned path, written only when every character of the pinned path is an ASCII letter, a digit or one of `_ - . / :`; on Windows no `settings.json` is written; landed as `run_rewrites_the_settings_override_each_start` (`tests/cli_instance_state.rs`, with a `cfg!(windows)` branch) and the unit `statusline_override_of_a_start_is_rewritten_whole_at_owner_only` (`src/cmd/run.rs`).
    sidecar: >-
      §6 Path 1: the `settings.json` rewrite is asserted on Unix (0600, absolute pinned path, plain-character rule); none is written on Windows (was: unqualified, no verification bullet).
    rationale: >-
      Report Schema / config: "`<home>/instances/<name>/settings.json` … rewritten at every start through `replace_private` at 0600; written only on Unix and only when every character of the pinned path is an ASCII letter, a digit or one of `_ - . / :`"; Outcome: "(security) the override rewritten whole at 0600 with the absolute pinned path … — met". The scenario plants a `settings.json` sentinel on every OS and its verification names only `plugin/` hooks.json.
    basis: >-
      tests/cli_instance_state.rs:372-407 (@376; the Windows branch 381-384); src/cmd/run.rs:1174-1213 (@1175)
  - detector: D-tests-coverage
    severity: warning
    section: >-
      §1 Test Scope Summary → Critical paths → `run` start sequence (Verification signal, the `snapshot.json` bullet)
    change: >-
      "`plugin/` and `settings.json` are rewritten" becomes "`plugin/` is rewritten, and on Unix the instance's `settings.json` (the statusline override: 0600, the absolute pinned path, only for a pinned path of plain characters); on Windows no `settings.json` is written".
    sidecar: >-
      §1 Critical paths (`run` start): the `settings.json` rewrite is Unix-only.
    rationale: >-
      Second occurrence of the unqualified rewrite claim retired by the Path 1 proposal; report Schema / config ("written only on Unix and only when …").
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: >-
      §1 Test Scope Summary → Coverage triggers → the negative-tests list holding "`plugin/` and `settings.json` rewritten on start"
    change: >-
      "`plugin/` and `settings.json` rewritten on start" becomes "`plugin/` rewritten on start, and on Unix the instance's `settings.json` (none is written on Windows)".
    sidecar: >-
      §1 Coverage triggers: the `settings.json` rewrite-on-start negative is Unix-only.
    rationale: >-
      Third occurrence of the same unqualified rewrite claim; report Schema / config ("written only on Unix").
    dependent-of: D-tests-coverage

  - detector: D-tests-framework
    severity: warning
    section: >-
      §3 → Bootstrap phases (derive for route / setup-project) → test-runner-install (the `[profile.ci]` overrides sentence)
    change: >-
      "the twelve verify-driven binaries (`channel_endpoint cli_answer cli_fake_agent cli_instance_state cli_send cli_verify cli_version_gate cli_wheel conpty_sideload contract_fake_agent_drift contract_ledger_probes tui_wheel`)" becomes "the thirteen verify-driven binaries (`channel_endpoint cli_answer cli_fake_agent cli_instance_state cli_send cli_verify cli_version_gate cli_wheel conpty_sideload contract_fake_agent_drift contract_ledger_probes hook_statusline tui_wheel`)", `hook_statusline` joining at chunk 2026-10-10-statusline-pass-through because five of its cases start from a stamped home.
    sidecar: >-
      Key file (test-runner-install): thirteen verify-driven binaries under the 20 s kill, `hook_statusline` added (was twelve).
    rationale: >-
      Report Counts: "`.config/nextest.toml` verify-driven binary list: 12 → 13 (`binary(hook_statusline)`)"; Files: `.config/nextest.toml` (one binary added to a list); Deviations 5: "`.config/nextest.toml` changed: five `hook_statusline` cases start from a stamped home". The key enumerates the runner's list by name and count and is one short. The rest of the runner is on-spec: nextest through `agent-run.sh run --unit` / `--integration`, proptest at `cases: 512`, `run --fuzz-replay`, coverage through `pre-push` (97.69 / 97.63 / 97.46 against floors 85 / 95 / 80, ignore regex unchanged), no mutation gate.
    basis: >-
      .config/nextest.toml:32
```

### Dispositions — test-plan

All nineteen: apply, routine (Accurate this-chunk addition), each fact in the report's Changes or Outcome.
1 and 2 are expected amendment 6 (Path 6 step 1; the `boot` key). The text of 2 is re-derived: the report says the
flag and its refusal are not built and are "Budget governor"'s; what an unbuilt flag exits with is the key's own
rule and is not restated. 3 to 5 qualify the pass-through as Unix only. 6 and 7 land the property. 8 and 9 move
the seed count with its rule (architecture 12). 10 to 12 are the fake agent's options, mode and receipt. 13 is the
source's seed row. 14 and 15 are Spec claims disproved 3 (the Windows home); its unread DACL is a route question
in the dialog. 16 to 18 qualify the `settings.json` rewrite as Unix only. 19 is the thirteen-binary list, its rule
the `binary(…)` names on the filter line.
The detector's three unproposed notes: the fifth snapshot reader is R1 (architecture's dispositions); the new
`tui_passthrough` case starts from `StampedHome::unstamped` (read at Validate), so it is not verify-driven and
the list stands at thirteen; the 26 waits stand.

## obs-plan

verdict: 10 proposal(s) · comment lines stripped: 0

```yaml
proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage → Scenario: Budget governor (Required log fields)"
    change: >-
      State the statusline lines as landed: `hook-invoked{hook_event:"statusline"}` with no `corr`; a `parse-rejected` line for stdin the arm rejects;
      `process-start` / `process-exit{subject:"statusline-shell", shell_exit_status, duration_ms}` only when the snapshot records a `statusline_command`
      and a shell is built (never on Windows, where `shell_argv` is `None`); `hook-decision{hook_event:"statusline", budget_written, duration_ms}` plus
      `detail` ∈ `strict-modes-failed|oversize-stdin|malformed-json|deadline` on a fail-open path and `deadline_hit` written only when true;
      `budget_written` is true only when the payload held a `rate_limits` object and the write succeeded; no line without `VIOLA_NAME`.
    sidecar: >-
      2026-10-10-statusline-pass-through — §4 Budget governor: the statusline arm's lines as landed (`parse-rejected`, the four `hook-decision` details,
      `deadline_hit` only when true, the shell pair only with a recorded command on Unix).
    rationale: >-
      The scenario lists `hook-decision{hook_event:"statusline", budget_written, duration_ms}` and an unconditional shell pair. The report's Coverage of
      new surfaces names `hook-invoked`, `parse-rejected` and `hook-decision` with `budget_written` for the arm, and its Expected amendments (obs-plan)
      give the details used and "`deadline_hit` is written only when true"; Symbols / APIs says `shell_argv` returns `None` on Windows and the Outcome
      says nothing is printed without a recorded command. The report names `parse-rejected` but not its `parser` / `detail` pair: read it at the basis
      before writing those two values.
    basis: "src/cmd/hook/statusline.rs:127-163"
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage → Scenario: Budget governor (Cleanup)"
    change: >-
      `statusline.shell_out` closes on child exit or at the 5 s PROVISIONAL bound `STATUSLINE_DEADLINE` (`src/cmd/hook.rs`), the bound ending in
      `hook-decision{detail:"deadline", deadline_hit:true}`; the user's stdout passes through unchanged and is never logged; oversize or malformed
      stdin prints nothing and runs no shell.
    sidecar: >-
      2026-10-10-statusline-pass-through — §4 Budget governor Cleanup: the shell-out span closes on child exit or at the 5 s PROVISIONAL
      `STATUSLINE_DEADLINE` (was "closes on child exit").
    rationale: >-
      The body says the span "closes on child exit" only. The report's Symbols / APIs adds `STATUSLINE_DEADLINE = 5 s`, PROVISIONAL, and Coverage of
      new surfaces gives the shell-out "a 5 s bound"; Expected amendments lists `deadline` among the details used and "no output on oversize or
      malformed stdin".
    basis: "src/cmd/hook/statusline.rs:281-298"
  - detector: D-obs-instrumentation
    severity: warning
    section: "§6 Log Coverage → `detail` code catalog (`hook-decision` fail-open codes)"
    change: >-
      Beside the seven codes (count unchanged), say that `viola hook statusline` uses four: `strict-modes-failed` (its instance check refused),
      `oversize-stdin`, `malformed-json`, and `deadline`, which on this arm is the 5 s PROVISIONAL bound on the user's command, not a dialog deadline;
      its `deadline_hit` is written only when true.
    sidecar: >-
      2026-10-10-statusline-pass-through — §6 detail catalog: the four `hook-decision` details the statusline arm writes, and what `deadline` means there.
    rationale: >-
      The report's Expected amendments names this catalog and the four details used; Counts verifies "`hook-decision` detail codes 7" unchanged. The
      catalog's `deadline` so far reads as the dialog deadline of the dialog scenario; the statusline arm gives it a second cause.
    basis: "src/cmd/hook/statusline.rs:165-185"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§1 Obs Scope Summary → Telemetry surfaces → cli (`viola hook <event>` and `viola hook statusline`) (Notes)"
    change: >-
      The `max < 1.0 s` latency budget is the hook events' gate; `viola hook statusline` is outside it — its shell-out of the user's command is bounded
      by the 5 s PROVISIONAL `STATUSLINE_DEADLINE` and no `run --perf` row times it.
    sidecar: >-
      2026-10-10-statusline-pass-through — §1 hook surface: the 1.0 s per-hook budget excludes the statusline arm, whose own bound is 5 s PROVISIONAL.
    rationale: >-
      The note gives "`max < 1.0 s` per hook" for a surface whose heading includes `viola hook statusline`. The report lands a 5 s bound on the arm's
      shell-out (Symbols / APIs; Coverage of new surfaces) and changes nothing under `scripts/` or `crates/viola-e2e` (Harness / gate surface: none),
      so no perf row was added for it.
    basis: "src/cmd/hook.rs:562-565"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage → Scenario: `viola run` start sequence to child spawn"
    change: >-
      Record two start steps that carry no span and no line by design, between `state.snapshot_write` and `pty.spawn`: the statusline source read
      (`statusline_source` / `statusline_command`, its result the snapshot's `statusline_command`) and the settings override write (`write_override`,
      `instances/<name>/settings.json` through `replace_private`, Unix only); the must-trace span list is unchanged, and the child's `--settings`
      argument, the source path and the command appear in no line or span.
    sidecar: >-
      2026-10-10-statusline-pass-through — §4 Scenario 1: the statusline source read and the settings override write at start are uninstrumented by
      design; span list unchanged.
    rationale: >-
      The detector reads an uninstrumented new operation as drift. The report's Coverage of new surfaces gives "instrumentation n/a (no line and no
      span by design)" for the source read and "n/a (no new span or line; the span-order test's list is unchanged)" for the override, while §1's
      `viola run` surface and §4's per-surface row promise spans on the start sequence and on `viola-state` writes. The override goes through
      `viola-state`'s replace helper (Outcome, arch) and is written after the first snapshot and before the spawn (Expected amendments, architecture).
      Either the body records the exemption or the operator rules that these two steps owe a span.
    basis: "src/cmd/run.rs:448-489"
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage → Edge flows → E5 (snapshot corruption)"
    change: >-
      Name `viola hook statusline`'s snapshot read (`src/cmd/hook/statusline.rs`) among the snapshot readers that write no `state-recovered` line:
      five other readers, not four.
    sidecar: >-
      2026-10-10-statusline-pass-through — §4 E5: the statusline arm is a fifth snapshot reader with no `state-recovered` line (was four).
    rationale: >-
      E5 counts "the four other snapshot readers" by file. The report's arm order (Expected amendments, architecture Hook contract) includes "the
      snapshot read", and none of the lines it lists for the arm is `state-recovered`. The report does not name the function the arm calls: confirm
      `read_snapshot` in one read before applying; if it is `read_snapshot_or_replay`, the sentence to amend is E5's "first product caller" one instead.
    basis: "src/cmd/hook/statusline.rs:127-163"
  - detector: D-obs-stack
    severity: warning
    section: "§3 → OTel SDK init"
    change: >-
      Init order step 4 and the `hook` anchor, for `viola hook statusline`: the instance check runs before `viola_obs_init`, because the role-file open
      itself narrows an existing home to 0700 (`open_role_file` → `create_private_dir(home)`) and a check made after it passes a home at 0770; a refused
      check then still opens the role file and writes `hook-invoked` and `hook-decision{detail:"strict-modes-failed"}`, so "a refused home gets no
      diagnostics line" does not hold for this arm.
    sidecar: >-
      2026-10-10-statusline-pass-through — §3 OTel SDK init: obs init narrows an existing home to 0700 as measured; the statusline arm checks first and
      logs its refusal as `strict-modes-failed`.
    rationale: >-
      Step 4 says the strict-modes check runs first, nothing under the home is created or opened before it passes, and a refused home gets no
      diagnostics line. The report's Spec claims disproved 2 measures the mechanism (case 5 red in the log-first order) and Deviations 1 puts the arm's
      check ahead of its log; Expected amendments (obs-plan) lists `strict-modes-failed` among the details the arm writes. The same key's `run` anchor
      for `viola revive` (log before preflight) is Spec claims disproved 5, read in source and not measured, and the operator's CARRY (inputs#I5): not
      proposed here.
    basis: "src/cmd/hook/statusline.rs:43-81"
  - detector: D-obs-stack
    severity: warning
    section: "§3 Observability Harness Contract (intro, the harness pattern bullets)"
    change: >-
      `hook` writes only its decision body to stdout and nothing to stderr, except `viola hook statusline`, whose stdout is the user's status-line
      command's output byte for byte (nothing when no command is recorded, on a refused instance, or on oversize or malformed stdin); no telemetry
      reaches stdout on either arm.
    sidecar: >-
      2026-10-10-statusline-pass-through — §3 intro: `hook statusline`'s stdout carries the user's command output, not a decision body.
    rationale: >-
      The harness pattern bullet says `hook` writes only its decision body to stdout. The report's Outcome has `hook statusline` exit 0 with empty
      stderr and print "the user's stdout byte for byte, nothing without a recorded command" (`hook_statusline` cases 1 to 4), and case 5 prints nothing
      for a home another user can write.
    basis: "tests/hook_statusline.rs:181-231"
  - detector: D-obs-stack
    severity: warning
    section: "§1 Obs Scope Summary → Telemetry surfaces → cli (`viola hook <event>` and `viola hook statusline`) (Exporter)"
    change: >-
      stdout carries only the decision body for `viola hook <event>`, and for `viola hook statusline` only the user's status-line command's output;
      stderr is forbidden on both.
    sidecar: >-
      2026-10-10-statusline-pass-through — §1 hook surface Exporter: the statusline arm's stdout is the user's command output.
    rationale: >-
      Same claim as the §3 intro bullet, restated on the surface whose heading names `viola hook statusline`; the report's Outcome contradicts
      "stdout carries only the decision body" for that arm.
    basis: "tests/hook_statusline.rs:181-231"
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: "§11 Obs Anti-Patterns → Logs (NEVER write telemetry to stdout on any role)"
    change: >-
      In the list of what stdout carries, the hook entry reads "the decision body, or for `hook statusline` the user's status-line output (hook)";
      the ban itself is unchanged.
    sidecar: >-
      2026-10-10-statusline-pass-through — §11 Logs: the stdout ban's hook entry names the statusline pass-through.
    rationale: >-
      Third statement of the same claim ("the decision body (hook)"), given as the reason for the stdout ban; the report's Outcome adds a second thing
      hook stdout carries.
    basis: "tests/hook_statusline.rs:181-231"
    dependent-of: D-obs-stack
```

### Dispositions — obs-plan

1. Budget governor, the lines as landed — apply, routine. The `parse-rejected` line's fields were read at
   Validate: `parser` `hook-stdin`, `detail` `oversize` or `malformed`, `count` 1 (`reject_stdin` in
   `src/cmd/hook.rs`, a function this chunk did not add).
2. Budget governor Cleanup, the 5 s bound — apply, routine.
3. §6 detail catalog (dependent) — apply; expected amendment 7.
4. §1 hook surface, the 1.0 s budget — apply, routine; a verbatim copy is kept current.
5. the start sequence's two steps with no span — ESCALATED (no rule; the detector asks for a ruling): in the
   dialog, recommended "record as by design" (the plan's constraint, P5-approved; the span-order list unchanged).
6. E5, the fifth snapshot reader — apply as R1: the arm calls `read_snapshot` (read at Validate).
7. OTel SDK init, the arm checks before obs init — apply, routine; Spec claims disproved 2. Read at Validate: a
   refused check still opens the role file, so the arm's refusal is logged and the opening narrows the home.
8. §3 intro, what `hook` writes to stdout — apply, routine.
9. §1 hook surface Exporter (dependent of 8) — apply.
10. §11 Logs, the stdout list (dependent of 8) — apply.

## a11y-plan

verdict: 2 proposal(s) · comment lines stripped: 6

Stripped (the detector's comment lines, as returned):

```
# D-a11y-obs-schema: no drift. The report changes neither schema: `Schema / config: none in schemas/`,
# `ObsEvent` 19 and `hook-decision` detail codes 7 unchanged; the new log content is additive fields and spans
# (`budget_written`, `shell_exit_status`, `duration_ms`, `deadline_hit`, subject `statusline-shell`), which obs
# may add. The statusline arm's `hook-invoked` / `hook-decision` lines carry no `corr` (Outcome, obs), matching
# the binding text reproduced in §1 (`corr` = `dialog_id` for dialog hooks, otherwise null). The a11y violation
# row (§3 → Structured violation JSON schema) is untouched.
```

```yaml
proposals:
  - detector: D-a11y-surface
    severity: warning
    section: "§3 → Keyboard test harness"
    change: "In Tooling, beside the revived-start reading of the zero-viola-bytes clause, add: a statusline-bearing start has one reading of its own — `passthrough_with_a_statusline_source_adds_no_viola_bytes` in `tests/tui_passthrough.rs` asserts that a start from a home holding a statusline source adds no viola byte to the outer-PTY stream (none of viola's own literals; the `--settings <instance dir>/settings.json` override is written on Unix only, none on Windows), green on the three CI runners (chunk 2026-10-10-statusline-pass-through, run 38080855246); `viola hook statusline` hands the user's own status line output to the child byte for byte and writes nothing to the terminal itself."
    sidecar: "2026-10-10-statusline-pass-through: §3 Keyboard test harness → Tooling gains the statusline-bearing start as a further reading of the zero-viola-bytes clause (tests/tui_passthrough.rs, three OS legs, run 38080855246)."
    rationale: "Report Changes → Expected amendments names `a11y-plan §3 → Keyboard test harness` as carried (Outcome: the statusline-bearing start). Coverage of new surfaces marks `the settings override and --settings` as `a11y ✓ (the statusline-bearing start adds no viola byte to the terminal, three OSes)`, and Outcome (a11y) records `the zero-viola-bytes cases green on three OS legs; the new start holds none of viola's literals — met (run 38080855246)`. The key's Tooling paragraph lists each start variant's reading of the clause (the fake-agent start, the revived start) and has no statusline line; grep of the body and all nine key files finds `statusline` only at the §1 not-assertable plugin entity. The detector's strict invariant is not breached: the chunk adds no interactive UI element (`No new CLI verb or flag of viola`; design/layouts `no line of viola's, no byte added, no verb in --help`), and the new start's boundary coverage exists — the doc only lacks its record. No existing claim is retired (`Each of the three boundary clauses is its own nextest case` and the §4 P4 tui bullet stay true), so there is no dependent occurrence."
    basis: "tests/tui_passthrough.rs:263-315 (@268 `fn passthrough_with_a_statusline_source_adds_no_viola_bytes`); report Coverage of new surfaces, last bullet; report Outcome (a11y)"
  - detector: D-a11y-surface
    severity: warning
    section: "§8 Cognitive Accessibility → Timeout extensions"
    change: "In the CLI bullet, after the `GATE_MAX_WAIT` sentences, add: `viola hook statusline` runs the user's own status line command under `STATUSLINE_DEADLINE` (5 s PROVISIONAL, `src/cmd/hook.rs`); the arm exits 0 with empty stderr in every case. That bound falls on the user's own command, never on a keystroke and never on the human: no key is blocked, refused or delayed by it and the wheel is untouched. No conformance claim for the terminal."
    sidecar: "2026-10-10-statusline-pass-through: §8 Timeout extensions → CLI lists the 5 s PROVISIONAL `STATUSLINE_DEADLINE` as a bound on the user's own status line command, never on a keystroke."
    rationale: "Report Changes → Expected amendments names `a11y-plan … §8 Cognitive Accessibility` as carried: `the 5 s bound falls on the user's own command, never on a keystroke`. Symbols / APIs adds `STATUSLINE_DEADLINE = 5 s, PROVISIONAL, in src/cmd/hook.rs`; Coverage of new surfaces gives the shell-out `a 5 s bound`; Outcome (arch) gives `hook statusline exits 0, empty stderr`. The §8 CLI bullet is the doc's list of every time bound viola holds and whom each falls on (`viola wait`'s caller timeout, `DIALOG_DEADLINE` 60 s, `GATE_MAX_WAIT` 8.5 s — `the deadline bounds the driver, never the human`); the new bound is absent from it. The web claim `v1 has no time limit` and the §11 Cognitive ban's `v1 has none` concern the GUI and SC 2.2.1 user time limits and are not retired by a bound on a shelled-out command, so there is no dependent occurrence."
    basis: "src/cmd/hook.rs (constant; the report gives no line for it) · src/cmd/hook.rs:562-565 (@563 `fn statusline_deadline_is_five_seconds`); report Symbols / APIs → Root bin"
```

### Dispositions — a11y-plan

1. Keyboard test harness, the statusline-bearing start — apply, routine; expected amendment 8.
2. §8 Timeout extensions, the command's bound — apply, routine; expected amendment 8.

