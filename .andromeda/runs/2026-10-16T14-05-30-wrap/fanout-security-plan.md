proposals:
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation (the row Session id as a child argv value)"
    change: "Retitle the row (`viola revive`, `viola verify` Run E) and add the third id source: verify's Run E takes the id from Run B's FIRST SessionStart capture through `viola_agent_claude::ledger::resume_target`, only when `is_session_id` accepts it (with no target Run E spawns nothing and `resume-session` fails); the id is one argv element of a direct spawn built by `resume_args(id, false)` (its second caller beside `revive`), after Run E's run args and its `--plugin-dir` pair, with no `--fork-session` and no `--settings`; the founder's own ratification of the verify child set is `inputs#I1` fork 2 of chunk 2026-10-14-resume-and-compact-ledger-rows."
    sidecar: "security-plan §Input Validation session-id row: a third id source, verify's Run E (Run B's first SessionStart id through `resume_target`'s `is_session_id` filter into `resume_args`); chunk 2026-10-14-resume-and-compact-ledger-rows."
    rationale: "The report's Symbols (resume_target, Run E) and Counts bullet 'The Session id as a child argv value row gains a third id source: Run B's first SessionStart capture, inside verify' (pattern sid-argv, security :129, :246); the row now names revive's two sources only, so a new argv-bound input is undocumented against §Input Validation."
    basis: "crates/viola-agent-claude/src/ledger.rs:842-850; src/cmd/verify/typed.rs:235-266; .andromeda/security-plan.md:246"
  - detector: D-security-input
    severity: escalate
    section: "§Threat Model Summary → Attack surface (Child process spawning and PATH resolution, the `revive` entry point)"
    change: "After the revive entry's session-id sentence, add that `viola_agent_claude::resume_args` has a second caller, verify's Run E (the `viola verify` entry point below), whose id comes from Run B's first SessionStart capture rather than a log, and point to the same §Input Validation session-id row."
    sidecar: "security-plan §Threat Model Summary revive entry: names verify's Run E as `resume_args`' second caller; chunk 2026-10-14-resume-and-compact-ledger-rows."
    rationale: "Report Counts bullet 'Session id as a child argv value row' lists sid-argv standing at security :129 as well as :246; the revive entry is the threat model's only statement of a `--resume <id>` child launch."
    basis: ".andromeda/security-plan.md:129"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Threat Model Summary → Attack surface (Child process spawning and PATH resolution, the `viola verify` entry point)"
    change: "verify spawns FIVE interactive `claude` children (A to E); add Run E: runs trusted in `<cwd>/.viola-verify-<pid>-resume/` (a 0700 `TrustedDir`, removed by its guard when verify passes, fails or is stopped), argv the run args, then `--plugin-dir` and the resume probe plugin, then `--resume <id>` from `resume_args(id, false)` with the id from Run B's first SessionStart capture (`resume_target`, `is_session_id`), no `--settings`; input `io::empty()`, so no key is ever typed; it waits for one SessionStart capture up to the probe deadline, settles, and is ended by a kill; with no shaped target it spawns nothing. An uninterrupted verify writes seven spawn pairs (version-probe, verify-probe, five verify-pty-probe). Ratification: the founder's own answer, `inputs#I1` fork 2 of chunk 2026-10-14-resume-and-compact-ledger-rows."
    sidecar: "security-plan §Threat Model Summary verify entry: four → five interactive children, Run E (`--resume` of Run B's session, `-resume` dir, empty input); chunk 2026-10-14-resume-and-compact-ledger-rows."
    rationale: "Report Symbols 'Run E, fn resume_run' and Counts 'verify's interactive runs move 4 → 5 (Run E)' (pattern verify-runs: security :131, :252, :282, :283, :291) and 'Spawn pairs … move 6 → 7' (spawn-pairs: security :131); Run E is a new spawned child whose output and captures verify reads, so its bounds belong in the entry point."
    basis: "src/cmd/verify/typed.rs:235-266; .andromeda/security-plan.md:131"
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation (Constants: MAX_FRAME)"
    change: "The MAX_FRAME child-output clause reads 'the output of verify's five interactive PTY runs, A to E'."
    sidecar: "security-plan §Input Validation Constants: the MAX_FRAME child-output list covers Run E (five runs, A to E); chunk 2026-10-14-resume-and-compact-ledger-rows."
    rationale: "Report Counts 'verify's interactive runs move 4 → 5' lists verify-runs standing at security :252, which still says 'four interactive PTY runs, A to D'."
    basis: ".andromeda/security-plan.md:252"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Data Protection → At rest (Probe captures)"
    change: "Add Run E's root: the probe dir gains `resume/` (Run E), holding the same capture plugin as `typed/` and its own `captures/`, read by `ledger::resume_variants` / the `resume-session` check, removed with the probe dir by the same drop guard; only a scrubbed `--record` copy (`SessionStart.resume-1.json`) leaves it."
    sidecar: "security-plan §Data Protection probe captures: the `resume/` root for Run E; chunk 2026-10-14-resume-and-compact-ledger-rows."
    rationale: "Report Schema/config 'Probe dirs: the verify probe dir gains resume/ (plugin + captures/)' and Symbols `ProbeDir::resume()` (src/cmd/verify.rs:357-359); Counts lists verify-runs at security :282."
    basis: "src/cmd/verify.rs:357-359; .andromeda/security-plan.md:282"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Data Protection → At rest (Interactive probe dirs)"
    change: "Add Run E's `<cwd>/.viola-verify-<pid>-resume/` to the list of interactive probe dirs in the user's current folder: 0700, removed when verify passes, fails, or is stopped by Ctrl-C or TERM, its guard removing only what that run created."
    sidecar: "security-plan §Data Protection interactive probe dirs: Run E's `-resume` dir; chunk 2026-10-14-resume-and-compact-ledger-rows."
    rationale: "Report Schema/config 'a new interactive dir <cwd>/.viola-verify-<pid>-resume/' and Counts 'Probe dirs gain -resume' (pattern probe-dirs: security :131, :283, :291)."
    basis: "src/cmd/verify/typed.rs:235-266; .andromeda/security-plan.md:283"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Data Protection (Accepted risk (verify's trusted runs, B, C and D))"
    change: "Name Run E beside B, C and D: Run E resumes Run B's session with `--resume` from its own `.viola-verify-<pid>-resume` dir with empty input, and adds no transcript file (measured live on 2.1.287: each round adds four, Run B ×2, Run C, Run D; `~/.claude/projects/` transcripts 643 → 647 → 651 over two rounds), although its payload names a `-resume` project path; the dead-path list reads `.viola-verify-<pid>[-dialogs|-plan|-resume]`."
    sidecar: "security-plan §Data Protection verify accepted risk: Run E named, measured to add no transcript; chunk 2026-10-14-resume-and-compact-ledger-rows."
    rationale: "Report Counts 'Accepted-risk transcripts: each round adds four … Run E adds none, although its payload names a -resume project path (643 → 647 → 651, live-round.md)' with pattern transcripts and verify-runs and probe-dirs at security :291."
    basis: ".andromeda/security-plan.md:291"
    dependent-of: D-security-input
  - detector: D-security-auth
    severity: warning
    section: "§Security Anti-Patterns (NEVER do these) → Universal (the stamp-gated non-null dialog decision)"
    change: "The ledger has twenty-two rows: add the resume row `resume-session` (`--resume <id> reopens the session under that id`), relied on every OS; counter `/22` on Unix and, on Windows, the first eighteen plus `resume-session`, counter `/19`; add that an all-`pass` stamp of exactly the twenty-one ids (Unix) or the eighteen ids (Windows) written before chunk 2026-10-14-resume-and-compact-ledger-rows also records `cli_verified:false`."
    sidecar: "security-plan §Security Anti-Patterns Universal: ledger 21 → 22 rows (`resume-session`), relied 22 Unix / 19 Windows, the older 21/18 stamps unverified; chunk 2026-10-14-resume-and-compact-ledger-rows."
    rationale: "Report Symbols 'LedgerRow::ResumeSession … the 22nd and last member of LedgerRow::ALL … relied on every OS' and Counts 'Ledger rows 21 → 22. The relied set moves 21 → 22 on Unix and 18 → 19 on Windows … An all-pass older stamp of 21 (Unix) or 18 (Windows) ids reads unverified' (pattern rows21: security-plan.md:617); the stamp is what authorizes a non-null dialog decision, PermissionRequest allow included."
    basis: "crates/viola-agent-claude/src/ledger.rs:43; .andromeda/security-plan.md:617"
note: "D-security-deps: no proposal (report Dependencies: none). security-plan.md:600 (pattern local-cmd) states no count or member list of the local-command list, and `/compact` is a compiled LOCAL_COMMANDS entry (ledger.rs:200), so it stays true; :288's fixture scrub rule already covers the new SessionStart.resume-1.json and needs no edit."
