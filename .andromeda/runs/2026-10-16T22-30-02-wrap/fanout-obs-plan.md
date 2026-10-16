proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: §6 Log Coverage → `detail` code catalog (`process-exit` exit 1)
    change: >-
      viola revive's start-arm preflight exit-1 details are `no-session`, `session-live` (the logged session id is
      listed live in another claude by one `claude agents --json` read; skipped under `--fork`; a failed, oversize or
      malformed read passes, fail open) and `cwd-missing`; the preflight reads the holder, then the session, then
      session-live, then the directory (four readings), each refusal one `process-exit{subject:"self", exit_code:1,
      detail}` in `run-<name>.ndjson` with no child; `schemas/diag-line.v1.json` closes the `process-exit` `detail` at
      13 values (12 before chunk 2026-10-16-unwrapped-sessions-on-the-board, 11 before chunk
      2026-10-13-interrupted-verify-cleanup, 9 before chunk 2026-10-10-viola-revive).
    sidecar: >-
      2026-10-16-unwrapped-sessions-on-the-board: §6 detail catalog gains `session-live` (revive preflight, four
      readings), the process-exit detail enum 12 → 13.
    rationale: >-
      Report Schema / config: `schemas/diag-line.v1.json` `process-exit.detail` gains `session-live` (12 → 13);
      Symbols / APIs: revive preflight holder → logged session → `session-live` → recorded directory (four readings,
      was three), one `process-exit{subject:"self", exit_code:1, detail:"session-live"}`, no child, a failed read
      passes; Counts: pattern `detail-enum` obs 753 stale (no master names `session-live`).
    basis: .andromeda/obs-plan.md:753; schemas/diag-line.v1.json:78; src/cmd/revive.rs:153-158
  - detector: D-obs-instrumentation
    severity: warning
    section: §4 Span / Trace Coverage → Scenario `viola run` start sequence to child spawn (Required log fields)
    change: >-
      viola revive's own preflight refusals add `detail` ∈ `no-session|session-live|cwd-missing` (and `already-live`,
      from the same collision check) before any child is started; its `session-live` reading (skipped under `--fork`)
      writes one `process-start` / `process-exit{subject:"agents-probe"}` pair in `run-<name>.ndjson`, plus one
      `parse-rejected{parser:"claude-agents-json"}` on an oversize or malformed answer, and passes on a failed read.
    sidecar: >-
      2026-10-16-unwrapped-sessions-on-the-board: §4 run-start scenario names revive's `session-live` refusal and its
      agents-probe pair.
    rationale: >-
      Report Symbols / APIs: `viola revive` preflight gains `session-live`; "its spawn pair lands in
      `run-<name>.ndjson`"; `read_logged` writes the `agents-probe` spawn pair and the `parse-rejected` line. The
      scenario's `no-session|cwd-missing` list restates the catalog claim the primary retires.
    basis: src/run/agents.rs:36-96; src/cmd/revive.rs:153-158
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §1 Obs Scope Summary → critical path `viola run` start sequence to child spawn (Required log fields)
    change: >-
      Exit-1 causes use a distinct `detail` code: already-live, squatted-name, sha-256 mismatch, `.cmd`/`.bat` child,
      and for a start made by `viola revive` also no-session, session-live and cwd-missing.
    sidecar: >-
      2026-10-16-unwrapped-sessions-on-the-board: §1 run-start critical path adds revive's session-live exit-1 cause.
    rationale: >-
      Report Schema / config (`session-live` joins the `process-exit` detail enum) and Symbols / APIs (revive's
      `session-live` refusal, exit 1); this line restates the revive exit-1 list without it.
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §4 Span / Trace Coverage → Edge flows → E5 (snapshot corruption)
    change: >-
      viola revive's start arm reads the logged session as the preflight's second reading, second of four since chunk
      2026-10-16-unwrapped-sessions-on-the-board (second of three since chunk 2026-10-10-budget-governor).
    sidecar: >-
      2026-10-16-unwrapped-sessions-on-the-board: §4 E5 revive preflight reading count three → four.
    rationale: >-
      Report Symbols / APIs: revive preflight chain holder → logged session → `session-live` → recorded directory
      (four readings; was three); Counts `revive-pairs` flags the same "second of three" wording stale in architecture,
      and E5 states it too.
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §3 → Logging stack
    change: >-
      a `viola revive` on a `gone` name appends its own lines to the same file and either starts the child or ends
      with an exit-1 `process-exit` (`no-session`, `session-live`, `cwd-missing`), its `session-live` reading adding
      one `agents-probe` spawn pair to that file.
    sidecar: >-
      2026-10-16-unwrapped-sessions-on-the-board: Logging stack key adds revive's session-live exit-1 detail and
      agents-probe pair.
    rationale: >-
      Report Symbols / APIs: revive's `session-live` refusal and "its spawn pair lands in `run-<name>.ndjson`"; this
      key restates the revive exit-1 list without `session-live`.
    basis: .andromeda/registries/contracts/obs-plan/logging-stack.md
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §3 → OTel SDK init
    change: >-
      After a passed check viola revive runs the same log init and own start line before its preflight, so the
      preflight's exit-1 refusals (`already-live`, `no-session`, `session-live`, `cwd-missing`) and its agents-probe
      spawn pair are logged in `run-<name>.ndjson`.
    sidecar: >-
      2026-10-16-unwrapped-sessions-on-the-board: OTel SDK init key adds revive's session-live refusal to the
      preflight's logged exit-1 list.
    rationale: >-
      Report Symbols / APIs: the revive preflight gains `session-live` (exit 1, one `process-exit`) and its spawn pair
      lands in `run-<name>.ndjson`; this key restates the preflight's exit-1 list without it.
    basis: .andromeda/registries/contracts/obs-plan/otel-sdk-init.md
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §4 Span / Trace Coverage → Edge flows → `verify` / `plugin install`
    change: >-
      verify logs its eight child spawns: `version-probe`, `verify-probe`, one `verify-pty-probe` pair per Run A–E, and
      one `agents-probe` pair for Run B's single `claude agents --json` read (made once its first turn has settled,
      before the framing pastes; capture in memory, no row text, session id, cwd or path in a line); a stop leaves the
      eight pairs as the uninterrupted run's; the step lines' `MM` is 23 on Unix and 20 on Windows, the 23rd row
      `agents-join` relied on every OS; "the eight spawn pairs stay eight".
    sidecar: >-
      2026-10-16-unwrapped-sessions-on-the-board: §4 verify spawn pairs 7 → 8 (Run B's agents-probe), relied rows
      22 / 19 → 23 / 20 (`agents-join`).
    rationale: >-
      Report Symbols / APIs (`viola verify`: Run B makes ONE `claude agents --json` read, spawn pairs per uninterrupted
      verify 7 → 8; `LedgerRow::AgentsJoin` appended last, relied on every OS); Counts: `spawn-pairs` and
      `relied-rows` both list obs 664 stale.
    basis: .andromeda/obs-plan.md:664; src/cmd/verify/typed.rs:365-376; src/run/agents.rs:36-96
  - detector: D-obs-instrumentation
    severity: warning
    section: §6 Log Coverage → Boundary-call wrappers → Child / shell spawns
    change: >-
      verify logs its eight spawns at the call site (the eighth Run B's `agents-probe` pair), and `run_bounded` and
      `run_capped` stay unlogged; the `agents-probe` pair is written by `run::agents::read_logged` for `viola list`
      (into `cli-<name>.ndjson`, only under a parsed `VIOLA_NAME`), `viola revive`'s `session-live` reading (into
      `run-<name>.ndjson`) and verify's Run B; an oversize or malformed answer adds one
      `parse-rejected{parser:"claude-agents-json"}`.
    sidecar: >-
      2026-10-16-unwrapped-sessions-on-the-board: §6 child spawns — verify's seven → eight, the agents-probe callers
      named.
    rationale: >-
      Report Counts: obs 788 "verify logs its seven spawns" stale (7 → 8); Symbols / APIs: `read_logged` writes the
      `agents-probe` spawn pair and the `parse-rejected` line, `run_capped` new under an unlogged `run_bounded`, list's
      pair only under a parsed `VIOLA_NAME`, revive's in `run-<name>.ndjson`.
    basis: .andromeda/obs-plan.md:788; src/run/agents.rs:36-96
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §6 Log Coverage → Boundary-call wrappers → Child / shell spawns (diag-line `subject` sub-bullet)
    change: >-
      each of the eight `verify` spawn pairs' `process-exit` carries `child_exit_status` and `duration_ms` (D-35); the
      closed `subject` enum is unchanged (Run B's agents read reuses `agents-probe`, Run E reuses `verify-pty-probe`).
    sidecar: >-
      2026-10-16-unwrapped-sessions-on-the-board: §6 diag-line subject bullet — seven → eight verify pairs.
    rationale: >-
      Report Counts: `spawn-pairs` obs 789 stale (verify spawn pairs 7 → 8); no subject added (Schema / config names
      only the `detail` enum change).
    basis: .andromeda/obs-plan.md:789
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §6 Log Coverage → `detail` code catalog (`parse-rejected` details, per `parser`)
    change: >-
      `oversize` is also a `claude-agents-json` detail (an answer past `MAX_FRAME`, `AgentsFailure::Oversize`),
      beside `channel-frame`, `hook-stdin` and `vt100-feed`; `malformed` covers its non-array or unparsable answer.
    sidecar: >-
      2026-10-16-unwrapped-sessions-on-the-board: §6 parse-rejected per-parser details add `oversize` for
      `claude-agents-json`.
    rationale: >-
      Report Symbols / APIs: `AgentsFailure` (`Oversize` · `Malformed`, `detail()`) and `read_logged` writing the
      `parse-rejected` line; Decisions: the operator's P5 confirm-class lean "`oversize` logged"; Expected amendments
      note the parser and the word are already enum members (obs 751), but the per-parser list names `oversize` only
      for three other parsers.
    basis: crates/viola-agent-claude/src/agents.rs:72-86; src/run/agents.rs:78-90
note: >-
  D-obs-stack and D-obs-pii fire nothing — no dependency added (report Dependencies), every new line through
  `obs_event!` in `read_logged`, and the report's Coverage states no row text, session id, cwd or path is logged
  (§8's `claude agents --json` row at obs 866 holds). The relied-row figures on obs 664 are folded into that site's
  proposal because the report lists the line stale for both patterns.
