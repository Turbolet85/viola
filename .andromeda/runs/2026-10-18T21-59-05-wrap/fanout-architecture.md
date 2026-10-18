proposals:
  - detector: D-retired
    severity: warning
    section: §Established Decisions → [Delivery Confirmation]
    change: "The sentence 'If a listed command's post-condition is none or not yet measured on this CLI version (`/remote-control` and `/compact` today), the send returns ok unconfirmable' names `/remote-control` alone as today's command with no post-condition; `/compact` now has a measured post-condition (a SessionStart with source `compact` and the session's own `session_id`) and is unconfirmable only on a CLI version that is not verified."
    sidecar: "[Delivery Confirmation]: `/compact` leaves the 'none or not yet measured' list; only `/remote-control` has no post-condition (chunk 2026-10-18-compact-post-condition-ledger-row)."
    rationale: "Report Retired: the '`/compact` reads `unconfirmable` at once on a verified CLI' behaviour is retired; Symbols / APIs: `LOCAL_COMMANDS` `/compact` now carries `Some(PostCondition::CompactedSession)`, so `classify(\"/compact\", true)` is `Confirms::Post(CompactedSession)`, while on an unverified CLI it stays `Confirms::Nothing`. Counts `compact` / `readback`: architecture 46 changed."
    basis: crates/viola-agent-claude/src/ledger.rs:201-202
  - detector: D-retired
    severity: warning
    section: §Established Decisions → [Delivery Confirmation]
    change: "The `LOCAL_COMMANDS` parenthesis reads `/clear` with the post-condition 'a new session', `/remote-control` with none, `/compact` with 'a SessionStart with source `compact` and the session's own `session_id`' (`PostCondition::CompactedSession`; listed since chunk 2026-10-14-resume-and-compact-ledger-rows, its post-condition and row `local-command-compact` landed at chunk 2026-10-18-compact-post-condition-ledger-row, the working-route entry 'Compact post-condition ledger row'), replacing 'with none ... its post-condition row is owed to the working-route entry Compact post-condition ledger row'."
    sidecar: "[Delivery Confirmation]: the local-command list's `/compact` entry carries `CompactedSession`; the owed post-condition row is discharged."
    rationale: "Report Symbols / APIs (`ledger::PostCondition::CompactedSession`, `LOCAL_COMMANDS`, `LedgerRow::LocalCommandCompact`) and Counts `owner-title` (architecture 46 changed: the owed `/compact` row discharged; the route entry is now done)."
    basis: crates/viola-agent-claude/src/ledger.rs:201-202
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Established Decisions → [Delivery Confirmation]
    change: "The unconfirmable-at-once parenthesis reads (`/remote-control` on every version, `/clear` and `/compact` with no passing stamp), replacing (`/remote-control` and `/compact` on every version, `/clear` with no passing stamp)."
    sidecar: "[Delivery Confirmation]: `/compact` is unconfirmable at once only on an unverified CLI."
    rationale: "Report Retired (the verified-CLI unconfirmable `/compact` behaviour and test `send_compact_on_a_verified_cli_is_unconfirmable_at_once`, replaced by `send_compact_on_a_verified_cli_is_confirmed_by_its_same_session` and `send_compact_on_an_unverified_cli_is_unconfirmable_at_once`); Symbols / APIs: on an unverified CLI `/compact` stays `Confirms::Nothing`."
    dependent-of: D-retired
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Delivery Confirmation]
    change: "The confirmation-window sentence adds: a `send` of `/compact` waiting for `Post(CompactedSession)` has its own compiled window, `viola_agent_claude::screen::COMPACT_CONFIRM_WINDOW` = 40 s (PROVISIONAL), never in `config.json`, read only through `SendSlot::window()` (every other confirmed send keeps `CONFIRM_WINDOW_FALLBACK` = 10 s); `confirm`'s deadline and the `run.confirm_window` span's `window_ms` both read `window()`, so a compact send's span reads 40 000; the `local-command-compact` row checks the measured `compact_latency_ms` against it."
    sidecar: "[Delivery Confirmation]: `COMPACT_CONFIRM_WINDOW` 40 s PROVISIONAL beside the unchanged 10 s fallback."
    rationale: "Report Symbols / APIs: `COMPACT_CONFIRM_WINDOW: Duration` = 40 s PROVISIONAL (screen.rs 22-27), `CONFIRM_WINDOW_FALLBACK` stays 10 s; `SendSlot::window()` (slot.rs 173-180); Counts `windows`: architecture 46 'states the 10 s fallback, stays, gains the compact bound'. The doc's 'The confirmation window is the compiled CONFIRM_WINDOW_FALLBACK = 10 s' as the only window is contradicted."
    basis: crates/viola-agent-claude/src/screen.rs:22-27
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Delivery Confirmation]
    change: "The blocking-total sentences read: `send` blocks for at most the gate's maximum wait plus the paste plus its window — 18.5 s with the compiled 8.5 s and 10 s, and for a `/compact` send 48.5 s plus the paste with the 40 s compact window; on the MCP surface the larger total must stay below the client's tool-call timeout, which on claude 2.1.287 exceeds 50 s (the `wait` read at 50 012 ms), above the 48.5 s plus paste, replacing 'above the 18.5 s total'."
    sidecar: "[Delivery Confirmation]: the compact send's worst-case total, 8.5 s + paste + 40 s = 48.5 s plus the paste, sits under the MCP ceiling."
    rationale: "Report Counts `mcp-ceiling`: 'GATE_MAX_WAIT 8.5 s + paste + 40 s = 48.5 s plus the paste, under the `wait` read at 50 012 ms' — architecture 46 changed (58, the measurement, no change); Outcome: 'the worst case 8.5 s + paste + 40 s sits under 50 012 ms — met'."
    basis: crates/viola-agent-claude/src/screen.rs:22-27
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Delivery Confirmation]
    change: "After the `/clear` claim, add the `/compact` claim: on a verified version a `/compact` send waits its own window for its post-condition only and no `prompt-submitted` claims it; the hook tap claims it on a `session-start` whose `cause` is `compact` and whose `agent_session_id` is a string EQUAL to the id remembered before that line (`SendSlot::session_started` reads it through `std::mem::replace`), appends the line unchanged, then settles with that line's `ts`, so the `session-start` precedes `send-confirmed`; another cause, another id, no string id, or nothing remembered claims nothing (with nothing remembered nothing equals it); if the 40 s window closes first the send is `not-delivered` / `no-prompt-submitted` with its cursor (no new detail). The remembered-id sentence notes that 'with nothing remembered any string id is new' holds for `/clear` alone."
    sidecar: "[Delivery Confirmation]: `/compact` is confirmed by a same-id `session-start` of cause `compact`; nothing remembered claims nothing."
    rationale: "Report Symbols / APIs: `src/run/send/slot.rs` `SendSlot::session_started` (slot.rs 102-121) claims on cause `compact` with the id EQUAL to the prior one, 'with nothing remembered nothing equals it'; Schema / config: a compact send's expiry reuses `not-delivered` / `no-prompt-submitted`; Outcome (arch) acceptance met."
    basis: src/run/send/slot.rs:102-121
  - detector: D-retired
    severity: warning
    section: §Established Decisions → [CLI Version Compatibility]
    change: "The Local commands bullet's `/compact` sentence reads: `/compact` (listed since chunk 2026-10-14-resume-and-compact-ledger-rows) has the post-condition a SessionStart with source `compact` and the session's own `session_id` (row `local-command-compact`, landed at chunk 2026-10-18-compact-post-condition-ledger-row): as measured on 2.1.287 on the Linux dev host in that chunk's `viola verify --record` round (`evidence/live-round.md`), Run B's pasted `/compact` fired a SessionStart of source `compact` with Run B's first session id and no UserPromptSubmit, its answer read 21 413 ms after the paste; its confirm window is `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL) beside the 10 s `CONFIRM_WINDOW_FALLBACK`; a `send` of it is confirmed on a verified CLI and `unconfirmable` at once only on an unverified one — replacing 'has none either and no probe types it ... its post-condition row ... is owed to the working-route entry Compact post-condition ledger row'."
    sidecar: "[CLI Version Compatibility] Local commands: `/compact`'s post-condition row landed, probed by Run B and stamped on 2.1.287."
    rationale: "Report Retired (verified-CLI unconfirmable `/compact`); Cross-cutting claims: the live round `stamped 2.1.287  24 pass  0 fail`, `compact_latency_ms` 21413, `compact-1` `source` `compact` with Run B's first id; Counts `compact`, `windows` (84 states the window as OWED, now built), `owner-title` (84 changed)."
    basis: crates/viola-agent-claude/src/ledger.rs:851-864
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Cross-cutting Patterns
    change: "The Capability ledger bullet's `/compact` sentence reads: `/compact` is on the local-command list since chunk 2026-10-14-resume-and-compact-ledger-rows; its post-condition row landed at chunk 2026-10-18-compact-post-condition-ledger-row: `local-command-compact` (cause `compact`, the same id, its own 40 s PROVISIONAL `COMPACT_CONFIRM_WINDOW`), probed by Run B's sixth paste, relied on every OS and stamped live on 2.1.287 on the Linux dev host (`stamped 2.1.287  24 pass  0 fail`, `evidence/live-round.md`), not measured on Windows; so a `send` of it is confirmed on a verified CLI and reads `unconfirmable` only on an unverified one — replacing 'with no post-condition, so a send of it reads unconfirmable ... needs no row under the ruled limit ... stays owed'."
    sidecar: "Cross-cutting Patterns (Capability ledger): `/compact` leaves the ruled limit; its row `local-command-compact` is stamped."
    rationale: "Report Retired; Symbols / APIs `LedgerRow::LocalCommandCompact` (relied on every OS); the live round in Cross-project claims; Counts `compact` / `readback` / `windows` / `owner-title`: architecture 410 changed; expected amendment 2 ('§Cross-cutting Patterns (Capability ledger) — `/compact` leaves the ruled limit')."
    dependent-of: D-retired
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [CLI Version Compatibility]
    change: "The ledger sentence reads: as landed by chunk 2026-09-28-capability-ledger-and-viola-verify and widened by chunks … 2026-10-16-unwrapped-sessions-on-the-board and 2026-10-18-compact-post-condition-ledger-row, the ledger holds twenty-four rows (the members of `LedgerRow::ALL`, in its order), the list ending `… · resume-session · agents-join · local-command-compact` (rows 1–23 keep their slots)."
    sidecar: "[CLI Version Compatibility]: twenty-four ledger rows, `local-command-compact` the twenty-fourth and last."
    rationale: "Report Symbols / APIs: `ledger::LedgerRow::LocalCommandCompact`, 'the twenty-fourth and last member of `LedgerRow::ALL: [Self; 24]` (rows 1–23 keep their slots)'; Counts `rows-count` (architecture 87 changed). The doc's closed set of twenty-three rows is contradicted."
    basis: crates/viola-agent-claude/src/ledger.rs:851-864
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [CLI Version Compatibility]
    change: "The relied-set sentence reads: a Unix host relies on all twenty-four rows and a Windows host on twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`, the last three relied on every OS."
    sidecar: "[CLI Version Compatibility]: relied set 24 on Unix, 21 on Windows."
    rationale: "Report Symbols / APIs: `LocalCommandCompact` relied on every OS (no arm in `relied_on`), so `LedgerRow::relied()` is 24 on Unix and 21 on Windows (was 23 / 20); Counts `rows-count`."
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [CLI Version Compatibility]
    change: "The `check(row, &Probes)` description adds: the `local-command-compact` row (`local_command_compacted(trusted, typed)`) passes only when `compact_start` (the first SessionStart of source `compact` after the newline turn's prompt) finds a SessionStart whose `session_id` is non-empty and equals Run B's first SessionStart `session_id`, no UserPromptSubmit carries `/compact`, and `compact_latency_ms` is within `COMPACT_CONFIRM_WINDOW`; and the landed-rows list adds: the `local-command-compact` row landed with chunk 2026-10-18-compact-post-condition-ledger-row through Run B's `/compact` paste, as measured on live 2.1.287 on the Linux dev host at that chunk (`evidence/live-round.md`: `stamped 2.1.287  24 pass  0 fail`, `compact_latency_ms` 21413; one round, six starts), relied on every OS and not measured on Windows."
    sidecar: "[CLI Version Compatibility]: the `local-command-compact` check and its live stamp on 2.1.287."
    rationale: "Report Symbols / APIs: the check arm (ledger.rs 851-864) and `compact_start` (ledger.rs 841-849); Cross-project claims: the live round 2026-10-18T21:24:14Z, `stamped 2.1.287  24 pass  0 fail`, `compact_latency_ms` 21413, 6 starts."
    basis: crates/viola-agent-claude/src/ledger.rs:841-849
  - detector: D-arch-decisions
    severity: escalate
    section: §Established Decisions → [CLI Version Compatibility]
    change: "Run B's paste sentence reads: it gets six bracketed pastes, all compiled texts: `PROBE_PROMPT`, then `PROBE_LONG_PASTE`, `PROBE_TAG_PASTE`, `PROBE_NEWLINE_PASTE`, `PROBE_COMPACT_COMMAND` (the list's `/compact`) and `PROBE_LOCAL_COMMAND` (the list's `/clear`), each of the five added ones only into rows that hold the input-box literal and no modal literal, none after the stop (`ledger::FRAMING_TURNS`, in paste order `paste-1` · `paste-2` · `paste-3` · `compact-1` · `clear-1`); the `/compact` paste goes only into the settled input box the newline turn left, waits `local_command_answered` up to `PROBE_DEADLINE` (reading the stop), and a failed wait ends the framing turns; verify still makes six starts. The sixth paste is the founder's own answer to P4 fork 1 of chunk 2026-10-18-compact-post-condition-ledger-row (a boundary widening of the ratified verify child set, relayed by the operator, that chunk's `inputs#I3`)."
    sidecar: "[CLI Version Compatibility]: Run B pastes six compiled texts, `/compact` fifth; the founder's own fork-1 answer (`inputs#I3`)."
    rationale: "Report Symbols / APIs: `FRAMING_TURNS: [(&str, &str); 5]` with `compact-1` before `clear-1`, `PROBE_COMPACT_COMMAND` (ledger.rs 214-215), `framing_turns` in `src/cmd/verify/typed.rs` ('Run B now pastes SIX compiled texts … verify still makes six starts'); Decisions: the founder's own answer, the probe is a sixth compiled Run B paste, a boundary widening his (`inputs#I3`). Escalate: a founder-answered widening of the verify child set (the report's expected amendment 3 names the playbook pattern as escalate)."
    basis: crates/viola-agent-claude/src/ledger.rs:214-215
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [CLI Version Compatibility]
    change: "The `box_wait` sentence reads: after each of Run B's added text turns (the long, the tag-like and the newline turns) and after the `/compact` turn (the rows `/clear` is pasted into), the rows the paste guard reads come from `Run::box_wait`."
    sidecar: "[CLI Version Compatibility]: the `/clear` paste after `/compact` reads its rows from `box_wait`."
    rationale: "Report Symbols / APIs, `src/cmd/verify/typed.rs`: after the compact paste it 'records the latency, then `box_wait`s for the rows `/clear` is pasted into'."
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [CLI Version Compatibility]
    change: "The step-line parenthesis reads `MM` being `LedgerRow::relied().len()`: 24 on Unix, the last `[24/24] local-command-compact …`; 21 on Windows, the last `[21/21] local-command-compact …`."
    sidecar: "[CLI Version Compatibility]: verify's step lines `[NN/24]` / `[NN/21]`, last `local-command-compact`."
    rationale: "Report Counts `rows-count`: step lines `[NN/24]` / `[NN/21]`, the last `[24/24] local-command-compact …` / `[21/21] local-command-compact …`; Outcome (design / layouts / a11y) met."
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [CLI Version Compatibility]
    change: "The older-stamp parenthesis adds 'or exactly the twenty-three ids every stamp written before chunk 2026-10-18-compact-post-condition-ledger-row holds' to the all-`pass` stamps that read unverified on every OS; one holding the eighteen, `resume-session` and `agents-join` now reads unverified on Windows too, and one holding the eighteen, `resume-session`, `agents-join` and `local-command-compact` reads verified on Windows only; so every home stamped before chunk 2026-10-18-compact-post-condition-ledger-row reads `cli_verified:false` until its next `viola verify`."
    sidecar: "[CLI Version Compatibility]: a twenty-three-id all-pass stamp reads unverified; homes stamped before this chunk read `cli_verified:false`."
    rationale: "Report Symbols / APIs: `stamps_verdict_needs_the_dialog_rows` — the twenty-three pre-chunk ids all passing read `(false, None)`, adding `LocalCommandCompact` reads `(true, None)`; the Windows relied set gains `local-command-compact`; Outcome (arch) met. Counts `rows-count` keeps the historical twenty-two clause, which this adds to rather than replaces."
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [CLI Version Compatibility]
    change: "The PermissionRequest `updatedInput` sentence reads 'adding one needs a 25th row', replacing 'a 24th row' (the twenty-fourth is now `local-command-compact`)."
    sidecar: "[CLI Version Compatibility]: the hypothetical next row is the 25th."
    rationale: "Report Symbols / APIs: `LocalCommandCompact` is the twenty-fourth and last member of `LedgerRow::ALL: [Self; 24]`."
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources
    change: "The `fixtures/claude/<cli-version>/` entry's framing captures read `UserPromptSubmit.paste-<n>.json`, `SessionStart.compact-<n>.json`, `SessionEnd.clear-<n>.json` and `SessionStart.clear-<n>.json` (stems `paste`, `compact`, `clear`; `ledger::framing_variants` names the compact SessionStart `compact-1`), through the same scrub, refusal and walk."
    sidecar: "Occupied Resources (Repository): the `compact` framing stem and `SessionStart.compact-<n>.json` registered."
    rationale: "Report Symbols / APIs: `ledger::framing_variants` names the compact SessionStart `compact-1` (ledger.rs 885-887); Files: new `fixtures/claude/2.1.287/SessionStart.compact-1.json`; Schema / config: recorded fixture class `SessionStart.compact-1.json`, scrubbed, schema `claude-fixture.v1.json`."
    basis: crates/viola-agent-claude/src/ledger.rs:885-887
  - detector: D-arch-resources
    severity: warning
    section: §Established Decisions → [CLI Version Compatibility]
    change: "The `--record` sentence's framing variants read `UserPromptSubmit.paste-<n>.json`, `SessionStart.compact-<n>.json`, `SessionEnd.clear-<n>.json` and `SessionStart.clear-<n>.json` variants (`ledger::framing_variants`)."
    sidecar: "[CLI Version Compatibility]: `--record` writes the `compact-1` framing variant."
    rationale: "Report Symbols / APIs: `ledger::framing_variants` (ledger.rs 885-887); Counts `framing-var` (architecture 87 changed: rows listing the stems)."
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources
    change: "The committed `2.1.287` set reads: spine + 3 screens + 12 dialog variants + 6 framing variants — `paste-1`, `paste-2`, `paste-3`, `compact-1`, `clear-1` ×2 (the `compact-1` SessionStart recorded live at chunk 2026-10-18-compact-post-condition-ledger-row and copied alone) …; 34 files, the count's rule `ls fixtures/claude/2.1.287 | wc -l`; stamped at the relied set (`LedgerRow::relied()`: 24 rows on Unix, 21 on Windows) by `contract_ledger_probes` (its `ROW_IDS` the twenty-four ids; `24 pass  0 fail` on Unix)."
    sidecar: "Occupied Resources (Repository): the 2.1.287 set holds 34 files with `compact-1`; `ROW_IDS` twenty-four."
    rationale: "Report Counts `set-287` (33 → 34 files, framing variants five → six) and `framing-var`, `rows-count`, `ledger-lits` (architecture 359 changed); Files: `SessionStart.compact-1.json` recorded live, copied alone; Outcome: `contract_ledger_probes` `24 pass  0 fail`."
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources
    change: "The `viola-fake-agent` `--framing` clause adds: the compact stem fires the set's recorded `SessionStart.compact-1` with `session_id` set to the last SessionStart the agent fired (the launch one or a replayed clear's; the remembered session moves only on a SessionStart whose fixture exists), and no UserPromptSubmit; a set without the variant fires nothing for it; no option or seam added (the twelve argv options stand)."
    sidecar: "Occupied Resources (test-only binaries): the fake agent's `--framing` replays `compact-1` with the last fired session id."
    rationale: "Report Symbols / APIs, `src/bin/viola-fake-agent.rs`: the compact stem replay, `Agent::session: Mutex<Option<String>>` set only when the SessionStart's fixture exists, no option or seam added; unit case `fake_compact_replay_carries_the_last_fired_session_id` (fake-agent 1919-2004); Deviations 5."
    basis: src/bin/viola-fake-agent.rs:1919-2004
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts
    change: "The Ledger stamps envelope's `typed_probe` reads `{ready_settle_ms, turn_settle_ms, prompt_latency_ms, max_turn_gap_ms, compact_latency_ms}` (`compact_latency_ms`: from the `/compact` paste to the instant its answer was read; additive, no `v` bump; `run` reads none of it)."
    sidecar: "Standard Contracts (Ledger stamps envelope): `measured.typed_probe` gains `compact_latency_ms`, additive, no `v` bump."
    rationale: "Report Schema / config: the stamp's `measured.typed_probe` gains `compact_latency_ms` (additive, no `v` bump); Symbols / APIs: `TypedRun.compact_latency_ms` (ledger.rs 613-614), written by `merge_stamp` (ledger.rs 1161); Counts `typed-probe`: architecture 198 changed."
    basis: crates/viola-agent-claude/src/ledger.rs:613-614
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources
    change: "The `ledger/stamps.json` entry's `typed_probe` gloss reads: the interactive runs' ready and turn settle times, prompt latency, largest turn gap and `/compact` answer latency, which validate the compiled gate and window constants (the compact latency against `COMPACT_CONFIRM_WINDOW`)."
    sidecar: "Occupied Resources (`ledger/stamps.json`): the `typed_probe` gloss names the compact latency."
    rationale: "Same claim as the envelope's field list: this entry enumerates `typed_probe`'s readings and omits `compact_latency_ms` (Report Schema / config; Symbols / APIs `TypedRun.compact_latency_ms`, ledger.rs 613-614). The report's `typed-probe` pass reads 349 'no change (they name the other fields)'; proposed as the duplicate occurrence for the orchestrator to weigh."
    dependent-of: D-arch-resources
  - detector: D-arch-decisions
    severity: warning
    section: §Standard Contracts
    change: "The envelope's verified sentence reads: every row of `LedgerRow::relied()` `pass` (all twenty-four on Unix; on Windows twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`), so an all-`pass` stamp of the eighteen, twenty-one, twenty-two or twenty-three pre-chunk id sets (the last: the ids before chunk 2026-10-18-compact-post-condition-ledger-row) reads unverified on every OS, so do one of the eighteen and `resume-session` and one of the eighteen, `resume-session` and `agents-join` on Windows, and one of the eighteen, `resume-session`, `agents-join` and `local-command-compact` reads verified on Windows only."
    sidecar: "Standard Contracts (Ledger stamps envelope): the relied set 24 / 21; the twenty-three-id stamp reads unverified."
    rationale: "Report Symbols / APIs (`relied()` 24 Unix / 21 Windows; `stamps_verdict_needs_the_dialog_rows`: the twenty-three pre-chunk ids all passing read `(false, None)`); Counts `rows-count` architecture 198 changed."
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Cross-cutting Patterns
    change: "The Capability ledger bullet's relied-set parenthesis reads `LedgerRow::relied()`: 24 rows on Unix, 21 on Windows (the historical `stamped 2.1.287  23 pass  0 fail` of `agents-join` stays)."
    sidecar: "Cross-cutting Patterns (Capability ledger): relied set 24 / 21."
    rationale: "Report Counts `rows-count` (architecture 410 changed; a row stating a historical stamp's count stays)."
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Infrastructure Patterns → CI/CD approach
    change: "Line 19 reads the relied ledger rows (`LedgerRow::relied()`: all twenty-four on the Unix runners, twenty-one on `windows-2025`, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`; `tests/cli_verify.rs`, harness `boot` step 4)."
    sidecar: "CI/CD approach key: CI's verify reads 24 / 21 relied rows."
    rationale: "Report Counts `rows-count`: registry `architecture/ci-cd-approach.md:19` standing; Harness / gate surface: boot's stamp at twenty-four rows; Outcome: CI's Windows leg ran the twenty-one-line literals."
    basis: .andromeda/registries/contracts/architecture/ci-cd-approach.md:19
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Infrastructure Patterns → Crate dependency direction
    change: "Line 6's pure `ledger` module reads 'the twenty-four rows (`LedgerRow::ALL`)', replacing 'the twenty-three rows'."
    sidecar: "Crate dependency direction key: the ledger module holds twenty-four rows."
    rationale: "Report Symbols / APIs: `LedgerRow::ALL: [Self; 24]`; Counts `rows-count`: registry `architecture/crate-dependency-direction.md:6` standing."
    basis: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Deployment / Distribution]
    change: "'Whether a registered `viola mcp` that exits 1 blocks the CLI's start is unmeasured, owed to the route entry Plugin precedence live readings', replacing 'Compact post-condition ledger row' (now done)."
    sidecar: "[Deployment / Distribution]: the exit-1 MCP start reading moves to the route entry 'Plugin precedence live readings'."
    rationale: "Report Counts `owner-title`: the route entry 'Compact post-condition ledger row' is now done and CARRY 2's parts move to the new entry 'Plugin precedence live readings' (architecture 95 standing); `inputs#I3` F3 and `inputs#I4` card row 1 name 'whether a registered `viola mcp` that exits 1 blocks an unwrapped start' in that entry."
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Plugin Scope]
    change: "'Whether Claude Code loads both a `--plugin-dir` plugin and an installed plugin with the same name, and which `viola` MCP server a wrapped session then starts and uses … is not measured: owed to the route entry Plugin precedence live readings, its Windows leg to Windows-only live measurements', replacing 'Compact post-condition ledger row'."
    sidecar: "[Plugin Scope]: the plugin-precedence reading moves to the route entry 'Plugin precedence live readings'."
    rationale: "Report Counts `owner-title` (architecture 96 standing); `inputs#I4` card row 1: the new entry carries CARRY 2 whole, 'which `viola` MCP server or servers a wrapped session holding both … starts and which answers `list`'."
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions → [Plugin Scope]
    change: "The transcripts clause reads: Run B three per real-CLI verify (a `<id>-backup-<n>.jsonl` the CLI writes at the compaction and two session transcripts, its session and the one its pasted `/clear` opens, the second inferred, not read), C and D one each; Run E adds none (as measured on 2.1.287 at chunk 2026-10-18-compact-post-condition-ledger-row, `evidence/live-round.md` §Transcripts: the `viola-verify` transcript count 60 → 65), replacing 'Run B two per real-CLI verify'."
    sidecar: "[Plugin Scope]: Run B leaves three transcript files per real-CLI verify since its `/compact` paste (delta 5 measured)."
    rationale: "Report Spec claims disproved by measurement: 'Run B leaves two per real-CLI verify' at architecture.md:96 is stale — measured delta 5: Run B three (backup 380 222 B, two session transcripts), Run C one, Run D one, Run E none (`evidence/live-round.md` §Transcripts, entries 8 and 11)."
note: "No new IPC method, endpoint, event kind, socket, port, env var, config key, workspace crate or dependency this chunk (report Crates / modules, Dependencies, Schema / config), so D-arch-resources fires only on the stamp field, the fixture stem and the fake agent's replay. Left unchanged as historical: architecture 83 and 410's `agents-join` `stamped 2.1.287  23 pass  0 fail`, 87's 'twenty-two ids … before chunk 2026-10-16' clause (added to, not replaced), 58 (the MCP measurement), and the generic 'none or not yet measured' unconfirmable sentences at 4, 125 and 408 (still true of `/remote-control` and an unverified CLI). `.andromeda/registries/contracts/architecture/project-directory-structure.md:108` names `<Event>.<stem>-<n>.json` framing variants without listing stems: no change."
