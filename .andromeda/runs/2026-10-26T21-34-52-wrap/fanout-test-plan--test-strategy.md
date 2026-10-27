proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 63
    section: "## 2. Test Strategy"
    change: "In the Filesystem (V7) row's plan location, add §6 Security control negatives (root `tests/security_negatives_strict_modes.rs`: each of the six frame verbs `send` · `wait` · `last` · `answer` · `pause` · `release` refusing a widened instance before any log, exit 21 with the instance pair or the `instance-unreachable` document, no frame and no line (`frame_verb_refuses_a_widened_instance_before_any_log`); a home another user may read refusing `run` (`a_home_another_user_may_read_refuses_run`); a start under umask 000 making every home path owner-only (Unix); a Windows `--home` outside the profile taking the protected DACL, read back through `icacls /save`; only the wrapper writing `snapshot.json` and only `verify` writing `stamps.json`, byte-identical after every other writer-capable entry point) and E5's built steps 1–3 (root `tests/chaos_snapshot_replay.rs`)."
    sidecar: "test-strategy trigger map V7 row: the strict-modes security negatives landed in tests/security_negatives_strict_modes.rs and E5 steps 1–3 in tests/chaos_snapshot_replay.rs (chunk 2026-10-26-home-and-code-bearing-file-integrity)."
    rationale: "The report's Changes (Files: new tests/security_negatives_strict_modes.rs, 475 lines, and tests/chaos_snapshot_replay.rs; New text, by line: its five test functions) and Outcome ((security) six frame verbs refuse a widened instance; (tests) single writers, umask-independent modes, the Windows --home protected DACL, a pre-existing Users-read dir refused; (tests) E5 steps 1–3, each MET by these binaries on three CI legs) land the Vector 7 filesystem negatives, while the V7 row's plan location names only §5 Module ↔ disk and §6 Scenario 1, E5, where the V1 row names its security_negatives files."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:134
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 42
    section: "## 2. Test Strategy"
    change: "In the sync E2E suites' shared-helpers sentence, add: `home.rs` holds the home-widening helpers `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; on Windows `icacls`, its exit asserted), through which a root test widens or read-grants a home path, restores it and reads whether it is still widened."
    sidecar: "test-strategy naming: tests/support/home.rs gains the widen / grant_read / unwiden / still_widened helpers (chunk 2026-10-26-home-and-code-bearing-file-integrity, Deviation 9)."
    rationale: "The report's Changes (Harness / gate surface: tests/support/home.rs gains test helpers widen · grant_read · unwiden · still_widened, 331-389, Unix modes, Windows icacls with its exit asserted) and Deviation 9 add shared helpers to tests/support/home.rs; the file's line 42 states what events.rs, cli.rs, fake.rs and ndjson.rs hold, but nothing of home.rs's widening helpers."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:55
note: "No change at line 26 (Chaos tier's SIGSTOP heartbeat freeze): the report's Count 16 rules test-strategy.md:26 no-change, the chaos SIGSTOP stale-wrapper case being another test than the single writers; D-tests-coverage: the new diagnostics-open check (src/obs.rs check_diagnostics) is covered at the unit tier only (obs.rs 610-673), which this file's tier text does not contradict."
