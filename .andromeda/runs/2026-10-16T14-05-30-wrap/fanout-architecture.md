proposals:
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [CLI Version Compatibility] (the "Ledger rows:" sub-bullet list)
    change: >-
      Add a sub-bullet for the session resume row: `claude --resume <id>` reopens the session under that id (row
      `resume-session`, words `--resume <id> reopens the session under that id`, the 22nd and last member of
      `LedgerRow::ALL`, relied on every OS since chunk 2026-10-14-resume-and-compact-ledger-rows); it passes only when
      Run E's first SessionStart has `source` `resume` and the `session_id` of Run B's first SessionStart capture
      (`resume_target`, which admits only an id `is_session_id` accepts); as measured live on 2.1.287 on the Linux dev
      host (`evidence/live-round.md`) the CLI found Run B's session from Run E's own `-resume` project directory (the
      cross-directory scan), and Run E added no transcript file; its Windows reading is owed to "Windows-only live
      measurements".
    sidecar: >-
      [CLI Version Compatibility] Ledger rows: the `resume-session` row (`claude --resume <id>`), relied on every OS,
      probed by verify's Run E and stamped live on 2.1.287 (chunk 2026-10-14-resume-and-compact-ledger-rows).
    rationale: >-
      The report's Symbols bullet lands `LedgerRow::ResumeSession` (id `resume-session`, relied under `_ => true`) and
      its check arm; Cross-cutting/external claims record the live cross-directory resume. The decision's ledger-row
      list names every other relied dependence but has no entry for `--resume`, so the locked decision no longer
      describes the ledger the chunk landed.
    basis: crates/viola-agent-claude/src/ledger.rs:43
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [CLI Version Compatibility] (the ledger paragraph after the row list)
    change: >-
      The ledger (widened also by chunk 2026-10-14-resume-and-compact-ledger-rows) holds twenty-two rows, the list
      ending `· statusline-rate-limits · resume-session`; the relied set is all twenty-two on Unix and on Windows the
      first eighteen plus `resume-session` (nineteen; `relied_on` leaves it under `_ => true`); the step lines' `MM` is
      22 on Unix (last `[22/22] resume-session …`) and 19 on Windows (last `[19/19] resume-session …`); the stamp
      sentence reads: an all-`pass` stamp of the eighteen ids every stamp written before chunk 2026-10-13-owed-ledger-rows
      holds, or of the twenty-one ids every stamp written before chunk 2026-10-14-resume-and-compact-ledger-rows holds,
      reads unverified on every OS, so every home stamped before that chunk reads `cli_verified:false` until its next
      `viola verify` (a stamp of the eighteen plus `resume-session` verifies on Windows only); and the S3 sentence's
      "adding one needs a 22nd row" becomes "a 23rd row".
    sidecar: >-
      [CLI Version Compatibility] ledger 21 → 22 rows (`resume-session`), relied 22 Unix / 19 Windows, MM 22/19,
      older 21- and 18-id stamps unverified.
    rationale: >-
      Report Counts "Ledger rows 21 → 22. The relied set moves 21 → 22 on Unix and 18 → 19 on Windows", `[NN/MM]` MM =
      22 / 19, "An all-`pass` older stamp of 21 (Unix) or 18 (Windows) ids reads unverified. An 18-plus-resume stamp
      verifies on Windows only"; pattern `rows21`/`rows18` site architecture.md:91.
    basis: architecture.md:91
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [CLI Version Compatibility] (the `viola verify` run description in the ledger paragraph)
    change: >-
      verify drives five interactive PTY runs, not four: after Run D comes Run E (`fn resume_run`, the fifth
      `RunStep`), which reads Run B's captures, takes the id from `ledger::resume_target` (Run B's first SessionStart
      `session_id`, only of the session-id shape; with none it spawns nothing and the row fails), runs in
      `<cwd>/.viola-verify-<pid>-resume/` (0700, its guard removing it on every exit path) with the run args, then
      `--plugin-dir` and the resume capture plugin, then `viola_agent_claude::resume_args(id, false)`, input
      `io::empty()` (no key, no `--settings`), waits for one SessionStart capture up to the probe deadline, settles and
      is ended by a kill; `check` reads `Probes { …, resumed }` (Run E's captures in claim order); "a verify is five
      `claude` starts" becomes six; and CI's fake-agent verify runs Run E too.
    sidecar: >-
      [CLI Version Compatibility] verify's interactive runs 4 → 5: Run E resumes Run B's first session id through
      `resume_args`; `Probes.resumed`; a verify is six `claude` starts.
    rationale: >-
      Report Symbols "Run E, `fn resume_run`" (typed.rs:235-266) and `Probes::resumed`; Counts "verify's interactive
      runs move 4 → 5 (Run E). Live starts per verify round move 5 → 6", pattern `verify-runs` site architecture.md:91.
    basis: architecture.md:91
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [CLI Version Compatibility] (the `--record` sentence in the ledger paragraph)
    change: >-
      `--record` also writes Run E's first SessionStart as `SessionStart.resume-1.json` (`ledger::resume_variants`),
      through the same scrub, refusal and hygiene walk as the other variants.
    sidecar: >-
      [CLI Version Compatibility] `--record` writes `SessionStart.resume-1.json` (`ledger::resume_variants`).
    rationale: >-
      Report Symbols: `resume_variants` names Run E's SessionStart `resume-1`, sole caller verify.rs:540, and
      `named_payloads` adds `SessionStart.resume-1.json` scrubbed by the existing scrub (verify.rs:540-545); Schema "New
      fixture variant stem `SessionStart.resume-<n>.json`". The paragraph's `--record` list names every other variant
      family.
    basis: src/cmd/verify.rs:540-545
    dependent-of: D-arch-resources
  - detector: D-arch-decisions
    severity: warning
    section: §Stack and Technologies (Screen model row)
    change: >-
      "the screen of `viola verify`'s four interactive probe runs" becomes "five interactive probe runs".
    sidecar: >-
      §Stack Screen model: verify's interactive probe runs 4 → 5.
    rationale: >-
      Report Counts "verify's interactive runs move 4 → 5 (Run E)", pattern `verify-runs` site architecture.md:18.
    basis: architecture.md:18
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Plugin Scope]
    change: >-
      "The other four are the interactive runs" becomes five: after Runs A–D, Run E, trusted in
      `<cwd>/.viola-verify-<pid>-resume/`, loading the spine capture plugin from `ledger/probes/<pid>/resume/plugin/`
      and resuming Run B's session with `--resume <id>`; Run E adds no session transcript of its own (the transcript
      residual stays Run B two, C and D one each).
    sidecar: >-
      [Plugin Scope] verify's transient unwrapped children gain Run E (the `-resume` dir and the `resume/` plugin); it
      adds no transcript.
    rationale: >-
      Report Counts "verify's interactive runs move 4 → 5" (site architecture.md:100 under `probe-dirs`) and
      "Accepted-risk transcripts: … Run E adds none"; Symbols `ProbeDir::resume()` holding the same capture plugin as
      `typed/`.
    basis: architecture.md:100
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Delivery Confirmation]
    change: >-
      `viola_agent_claude::ledger::LOCAL_COMMANDS` holds three entries: `/clear` with the post-condition "a new
      session", `/remote-control` with none and `/compact` with none; the "not yet measured" example reads
      "(`/remote-control` and `/compact` today)" and the at-once `unconfirmable` example "(`/remote-control` and
      `/compact` on every version, `/clear` with no passing stamp)"; `PROBE_LOCAL_COMMAND` stays the list's `/clear`,
      and `classify` is unchanged.
    sidecar: >-
      [Delivery Confirmation] `LOCAL_COMMANDS` 2 → 3: `/compact` listed with no post-condition, so a `send` of it is
      `unconfirmable` on a verified and an unverified CLI alike.
    rationale: >-
      Report Symbols "`LOCAL_COMMANDS: [(&str, Option<PostCondition>); 3]` (`ledger.rs:197`) appends `("/compact",
      None)`", "`/compact` maps to `Confirms::Nothing` on a verified and an unverified CLI alike"; Coverage `send` of
      `/compact` (`send_compact_on_a_verified_cli_is_unconfirmable_at_once`); Counts `local-cmd` site
      architecture.md:49.
    basis: architecture.md:49
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [CLI Version Compatibility] (Local commands sub-bullet)
    change: >-
      Beside `/remote-control`, `/compact` is listed with no post-condition and no probe types it; its post-condition
      row (cause `compact`, the same id, a confirm window of its own) stays owed.
    sidecar: >-
      [CLI Version Compatibility] Local commands: `/compact` listed with "none"; its post-condition row owed.
    rationale: >-
      Report Symbols `LOCAL_COMMANDS` appends `/compact` and "`PROBE_LOCAL_COMMAND` is still `LOCAL_COMMANDS[0].0`
      (`/clear`), the only entry with a post-condition"; I2 card `## Beyond the plan` row 1; Counts `local-cmd` site
      architecture.md:88.
    basis: architecture.md:88
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Cross-cutting Patterns → Capability ledger as the single gate
    change: >-
      Retire "One relied-on dependence has no row yet and is not under that limit: `viola revive`'s" and the sentences
      that owe its row: `viola revive`'s `claude --resume <id>` has its row since chunk
      2026-10-14-resume-and-compact-ledger-rows, `resume-session`, probed by verify's Run E, relied on every OS and
      stamped live on 2.1.287 on the Linux dev host (`stamped 2.1.287  22 pass  0 fail`, `evidence/live-round.md`), its
      Windows reading owed to "Windows-only live measurements"; `/compact` is listed in `LOCAL_COMMANDS` with no
      post-condition (`unconfirmable`) and its post-condition row stays owed, its owner the operator's word at this
      chunk's wrap (`inputs#I2` card row 1); and the closing relied-set count reads 22 rows on Unix, 19 on Windows.
    sidecar: >-
      Capability ledger: `--resume` no longer the relied-on dependence with no row (`resume-session`); `/compact`'s
      post-condition row stays owed; relied 22/19.
    rationale: >-
      Report Counts "'One relied-on dependence has no row' (`--resume`) is no longer true", pattern `resume-norow`
      site architecture.md:473 (×3); `compact` architecture.md:473; `rows21` architecture.md:473; Outcome "the live
      stamp `stamped 2.1.287  22 pass  0 fail`"; I2 card rows 1 and 2.
    basis: architecture.md:473
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem (verify's interactive probe dirs)
    change: >-
      Register Run E's `<cwd>/.viola-verify-<pid>-resume/` (0700, `TrustedDir::create(inputs.cwd, "-resume")`, a drop
      guard) beside Runs A–D's dirs, removed like them when verify passes, fails or is stopped by Ctrl-C or TERM.
    sidecar: >-
      Occupied Resources: verify's interactive probe dirs gain Run E's `<cwd>/.viola-verify-<pid>-resume/`.
    rationale: >-
      Report Schema "a new interactive dir `<cwd>/.viola-verify-<pid>-resume/`"; Symbols `TrustedDir::create(inputs.cwd,
      "-resume")` (typed.rs:247), 0700, "removed by its guard on every exit path"; Counts `probe-dirs` site
      architecture.md:413. The registry lists every other interactive dir and not this one.
    basis: architecture.md:413
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem (`ledger/probes/<pid>/…`)
    change: >-
      The probe dir reads `ledger/probes/<pid>/{plugin/,captures/,questions/,plan/,typed/,resume/}`; `resume/` (Run E)
      holds the same capture plugin as `typed/` and its own `captures/`, read back into `Probes.resumed`.
    sidecar: >-
      Occupied Resources: the verify probe dir gains `resume/` (plugin + `captures/`).
    rationale: >-
      Report Schema "the verify probe dir gains `resume/` (plugin + `captures/`)"; Symbols `ProbeDir::resume()`
      (verify.rs:357-359) and `Probes.resumed = read_captures(resume)` (verify.rs:252); Counts `verify-runs` site
      architecture.md:412.
    basis: architecture.md:412
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Claude Code integration names (verify's probe plugin and children)
    change: >-
      "verify's four interactive children" becomes five; add Run E: cwd `<cwd>/.viola-verify-<pid>-resume/`, its argv
      the run args, then `--plugin-dir` with the `resume/` capture plugin, then `viola_agent_claude::resume_args(id,
      false)` (`--resume <id>`, a second caller beside `viola revive`), the id being Run B's first SessionStart
      `session_id` admitted by `is_session_id` (`ledger::resume_target`), one argv element of a direct `Run::spawn`
      under the R8 strip with input `io::empty()` and no `--settings`.
    sidecar: >-
      Claude Code integration names: verify's children gain Run E (`--resume <id>` from Run B's first session id
      through `resume_args`).
    rationale: >-
      Report Symbols Run E (typed.rs:251 "a second caller of `resume_args` beside `viola revive`", `io::empty()`, "there
      is no `--settings`"); Outcome (security) "The id reaches argv only through `resume_target`'s `is_session_id`
      filter and `resume_args`, as one element of a direct `Run::spawn` under the R8 strip"; Counts `verify-runs` and
      `probe-dirs` site architecture.md:372.
    basis: architecture.md:372
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem (`diagnostics/`, verify's spawn pairs)
    change: >-
      verify writes seven spawn pairs, not six: `version-probe`, `verify-probe` and one `verify-pty-probe` pair around
      each interactive run (Run A, Run B, Run C, Run D, then Run E).
    sidecar: >-
      Occupied Resources diagnostics: verify's spawn pairs 6 → 7 (five `verify-pty-probe`, Run E last).
    rationale: >-
      Report Counts "Spawn pairs of an uninterrupted `viola verify` move 6 → 7: `version-probe`, `verify-probe` and five
      `verify-pty-probe`", pattern `spawn-pairs` site architecture.md:406 ("six spawn pairs").
    basis: architecture.md:406
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Filesystem (`diagnostics/`, verify's stopped-run pair)
    change: >-
      Retire "a child killed at a Ctrl-C or TERM stop closes its pair with `duration_ms` and no `child_exit_status`":
      verify's interactive PTY runs close a child killed at the stop with the pump's exit code as `child_exit_status`
      (portable-pty reports a code for a signalled child) beside `duration_ms`; the rest (only the pairs it started,
      `exit_code:1, detail:"interrupted"`) stands.
    sidecar: >-
      Occupied Resources diagnostics: a child killed at verify's stop closes its pair with `child_exit_status` (the
      pump's exit code), disproved by measurement at chunk 2026-10-14-resume-and-compact-ledger-rows.
    rationale: >-
      Report "Spec claims disproved by measurement" 1, pattern `killed-exit` site architecture.md:406; evidence the
      stop case `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` (tests/cli_verify.rs:1071-1178);
      scope pre-existing behaviour.
    basis: tests/cli_verify.rs:1071-1178
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Repository (`fixtures/claude/<cli-version>/`)
    change: >-
      Register the variant stem `SessionStart.resume-<n>.json` (`ledger::resume_variants`, scrubbed, refused and
      hygiene-walked like the others, walked by `claude_resume_variants_are_walked_for_every_resume_set`); `2.1.287`
      gains `SessionStart.resume-1.json` (recorded live at chunk 2026-10-14-resume-and-compact-ledger-rows, 10 key
      names) and reads 32 files, not 31.
    sidecar: >-
      Repository: new fixture stem `SessionStart.resume-<n>.json`; the 2.1.287 set 31 → 32 files.
    rationale: >-
      Report Files "New: `fixtures/claude/2.1.287/SessionStart.resume-1.json` (578 B … recorded live)"; Schema "New
      fixture variant stem `SessionStart.resume-<n>.json`" with its 10 key names; Counts "The 2.1.287 fixture set moves
      31 → 32 files", pattern `fixtures31` site architecture.md:422.
    basis: architecture.md:422
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Repository (`fixtures/claude/<cli-version>/`, the 2.1.287 stamp)
    change: >-
      `2.1.287` is stamped at the relied set (22 rows on Unix, 19 on Windows) by `contract_ledger_probes` (its
      `ROW_IDS` the twenty-two ids; `22 pass  0 fail` on Unix).
    sidecar: >-
      Repository: the 2.1.287 relied-set stamp 21/18 → 22/19.
    rationale: >-
      Report Counts "Ledger rows 21 → 22 … 22 / 19", pattern `rows21` and `rows18` site architecture.md:422; Outcome
      "`contract_ledger_probes` and `cli_verify` name `resume-session` once per OS literal and end `22 pass` / `19
      pass`".
    basis: architecture.md:422
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources → Binary, subcommands and exit codes (test-only `viola-fake-agent`)
    change: >-
      Under `--resume <id>` the SessionStart fired at launch is built on the set's recorded `SessionStart.resume-1.json`
      when the set holds it (`launch_variant`, `RESUME_VARIANT = "resume-1"`), else on `SessionStart.default` as
      before, with `source` `resume` and `session_id` the id given (or the compiled fork id under `--fork-session`);
      `--resume` now also serves verify's Run E under the fake agent; the option count stays twelve.
    sidecar: >-
      Test-only binaries: the fake agent's `--resume` launch payload builds on the recorded `SessionStart.resume-1.json`
      when the set holds it (2.1.287), else on `default`.
    rationale: >-
      Report Symbols "Fake agent: `RESUME_VARIANT = "resume-1"` … `Agent::launch_variant` … under `--resume`, the launch
      SessionStart's base is the set's recorded `SessionStart.resume-1.json` when it exists, else `default` as before"
      and "No option was added, so the count stays twelve"; Counts `fake-resume` site architecture.md:362; Harness
      "`boot`'s `viola verify` stamp against the fake agent now runs Run E".
    basis: architecture.md:362
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts → Ledger stamps envelope
    change: >-
      A version is verified only when every row of `LedgerRow::relied()` reads `pass` (all twenty-two on Unix; on
      Windows the first eighteen plus `resume-session`), so an all-`pass` stamp of the eighteen pre-2026-10-13-owed-ledger-rows
      ids or of the twenty-one pre-2026-10-14-resume-and-compact-ledger-rows ids reads unverified on every OS, and one
      of the eighteen plus `resume-session` reads verified on Windows only.
    sidecar: >-
      Ledger stamps envelope: relied 22 Unix / 19 Windows; older 18- and 21-id stamps unverified.
    rationale: >-
      Report Counts "The relied set moves 21 → 22 on Unix and 18 → 19 on Windows … An all-`pass` older stamp of 21
      (Unix) or 18 (Windows) ids reads unverified. An 18-plus-resume stamp verifies on Windows only (`version_gate.rs`
      oracle)", pattern `rows21`/`rows18` site architecture.md:253.
    basis: architecture.md:253
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → CI/CD approach
    change: >-
      `viola verify` in CI runs against the fake agent all five interactive runs (not "all four typed runs") and the
      relied ledger rows (`LedgerRow::relied()`: all twenty-two on the Unix runners, nineteen on `windows-2025`, the
      first eighteen plus `resume-session`).
    sidecar: >-
      CI/CD approach key: CI verify runs five interactive runs and 22 / 19 relied rows.
    rationale: >-
      Report Counts `rows21`/`rows18` site `contracts/architecture/ci-cd-approach.md:20`; Harness "`boot`'s `viola
      verify` stamp against the fake agent now runs Run E: print probe and five interactive runs, every relied row (22
      Unix / 19 elsewhere)"; CI `ci#38868756782` windows-2025 leg at 19.
    basis: .andromeda/registries/contracts/architecture/ci-cd-approach.md:20
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Crate dependency direction
    change: >-
      `viola verify`'s five interactive probe runs (not four) feed their own screen; the ledger module holds the
      twenty-two rows (`LedgerRow::ALL`).
    sidecar: >-
      Crate dependency direction key: verify's interactive runs 4 → 5; the ledger's rows 21 → 22.
    rationale: >-
      Report Counts `rows21` and `verify-runs` site `contracts/architecture/crate-dependency-direction.md:6`.
    basis: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Crate dependency direction
    change: >-
      The pure `ledger` module's listed API gains `resume_target` (Run B's first SessionStart id of the session-id
      shape) and `resume_variants` (Run E's SessionStart named `resume-1`), and `Probes` gains `resumed`.
    sidecar: >-
      Crate dependency direction key: `viola-agent-claude::ledger` gains `pub fn resume_target`, `pub fn
      resume_variants` and `Probes::resumed`.
    rationale: >-
      Report Symbols "`pub fn resume_target(trusted: &[Capture]) -> Option<&str>` (`ledger.rs:842-850`)", "`pub fn
      resume_variants(resumed) -> Vec<(&'static str, &Capture)>` (`ledger.rs:867-873`)", "`Probes::resumed:
      Vec<Capture>` (`ledger.rs:652`)"; the key enumerates the ledger module's public symbols and lists neither.
    basis: crates/viola-agent-claude/src/ledger.rs:842-850
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: >-
      `verify/typed.rs`: verify's five interactive PTY runs (Run A untrusted, Run B trusted, Run C dialogs, Run D plan,
      Run E resume).
    sidecar: >-
      Project directory structure key: `verify/typed.rs` holds five interactive runs (Run E resume).
    rationale: >-
      Report Counts `verify-runs` site `contracts/architecture/project-directory-structure.md:31`; Symbols Run E `fn
      resume_run` in `src/cmd/verify/typed.rs:235-266`.
    basis: .andromeda/registries/contracts/architecture/project-directory-structure.md:31
    dependent-of: D-arch-decisions
note: The report's `local-cmd` sites architecture.md:91 (the list's `/clear` as `PROBE_LOCAL_COMMAND`) and project-directory-structure.md:37 (`classify` over `LOCAL_COMMANDS`) still read true and carry no proposal; the `/compact` row's owner is the operator's word at this wrap (I2 card row 1), not yet in the report; no new dependency, runtime or crate, so D-arch-decisions found no stack drift; the killed-exit claim's leaves (`.claude/rules/observability.md:39`, `.claude/docs/services/viola.md:35`) are the cascade's.
