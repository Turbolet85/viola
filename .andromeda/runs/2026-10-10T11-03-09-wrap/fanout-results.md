# Fan-out results — 2026-10-10-self-healing-state

Seven doc-agents, one batch. Entity probe after decode: 0 in every return. 24 proposals over four docs; three docs returned none. Each carrying doc's list is its return as received, comment lines included.

## Verdicts

- **architecture:** 7 proposals (D-arch-decisions 5, D-arch-resources 2). Nothing stripped.
- **security-plan:** 2 proposals (D-security-input 2). Stripped: three leading comment lines, D-security-auth and D-security-deps no drift, D-security-input finds no unvalidated boundary.
- **design-system:** `proposals: []`. Nothing stripped.
- **layout-templates:** `proposals: []`. Stripped: one trailing comment line (no surface added; the three count names already match the doc's `skipped` cells). Raw twin `.raw-fanout-layout-templates.md`.
- **test-plan:** 9 proposals (D-tests-coverage 9). Stripped: three leading comment lines, D-tests-framework and D-tests-obs-harness no drift; the detector says the owing entries' titles come from the chunk's scope.md, not the report.
- **obs-plan:** 6 proposals (D-obs-stack 3, D-obs-instrumentation 3). Stripped: three leading comment lines, D-obs-pii no drift.
- **a11y-plan:** `proposals: []`. Stripped: four trailing comment lines (no interactive element; no schema change; the E5 `state-recovered` mention in a key file read and left). Raw twin `.raw-fanout-a11y-plan.md`.

## architecture — the parsed list

```yaml
proposals:
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [Database / State Store]"
    change: "Replace the sentence 'As landed, the one reader so far (…) skips an unterminated last line — neither returned nor rewritten — and skips and counts an over-long or non-object line; healing and `state-recovered` are owed to the working-route entry \"Self-healing state\".' with: 'As landed (chunk 2026-10-10-self-healing-state): the heal is the appender's. Every public appender (`append_event`, `append_event_at`, `try_append_event`) reads the log's last byte under the lock the append already holds and, when that byte is not LF, writes one LF and the line in a single `write_all`, then logs one WARN `state-recovered` line (`detail` `torn-line-healed`, `file` `events.ndjson`, the offset, no `corr`); an append that does not heal logs none, the healed file is never shorter, `append_event_at` hands its builder and returns the offset the line really starts at (L + 1 after a heal, L otherwise), and `try_append_event` with the lock held writes and heals nothing. The reader `viola_state::events::read_from` (the wrapper's `wait` scan and `last` rebuild, and the library `replay`) never rewrites the log and counts per read in `Skipped{unknown_kinds, unknown_fields, torn_lines}`: a line whose `kind` is absent, not a string or not one of the 13 is not returned (`unknown_kinds`); a known-kind line with a top-level key outside the six or a `data` key outside its kind's row is returned and counted once (`unknown_fields`); an over-long line, a terminated line that is not one JSON object and the unterminated last line of the read count as `torn_lines`, the last still neither returned nor rewritten. The counts are not logged, `LoggedLines::skipped` has no product caller, and no surface shows them yet.'"
    sidecar: "2026-10-10-self-healing-state: [Database / State Store] as-landed rewritten — the appender heals a torn last line and logs `state-recovered`; the reader counts `unknown_kinds` / `unknown_fields` / `torn_lines`; the 'owed to Self-healing state' clause removed."
    rationale: "Report Changes → Symbols / APIs: 'the heal is the appender's' (`tail`, `write_line`, all three appenders), `Skipped`'s fields are now `unknown_kinds`, `unknown_fields`, `torn_lines` (were `oversize`, `malformed`), the unterminated last line is counted; Log lines: `state-recovered` now has product emitters (0 → 2). Spec claims disproved names this entry as describing an uncounted, unhealed last line with healing owed. Expected amendments lists this site (`grep -n 'Self-healing state' architecture.md`, 1 hit, line 50)."
    basis: "architecture.md:50 (the entry); crates/viola-state/src/events.rs:312-325 (`tail`), 336-342 (the heal's `obs_event!`), 305-309 (`Tail::line_start`), 200-203 and 205-217 (`known_kind`, `has_unknown_field`)"
  - detector: D-arch-decisions
    severity: escalate
    section: "§Established Decisions → [Database / State Store]"
    change: "The decision sentence 'Readers heal a torn last line.' becomes 'The next append heals a torn last line; readers skip and count it and never rewrite the log.'"
    sidecar: "2026-10-10-self-healing-state: [Database / State Store] decision sentence — the actor of the heal is the appender, not the reader."
    rationale: "Same entry, a second claim: the locked (KEYSTONE) sentence names the reader as the healer, and the report's Symbols / APIs lands the opposite actor ('the heal is the appender's'; the unterminated last line 'is still neither returned nor rewritten by the reader'). Applying only the as-landed rewrite would leave the entry contradicting itself. Escalated because it rewords the decision's mechanism, not only its landed status; the plan's expected amendment names only the 'as landed' and 'owed' sentences. Swept the rest of the document: §Cross-cutting Patterns 'Readers always tolerate a torn last line' still holds and needs no change; no other line says a reader heals, repairs or rewrites the log; 'viola never truncates `events.ndjson`' (§Standard Contracts SSE feed, §Occupied Resources Filesystem) still holds — the heal only adds a byte."
    basis: "architecture.md:50"
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: "§Infrastructure Patterns → Project directory structure"
    change: "The `viola-state/` comment becomes: 'ndjson logs (the appenders heal a torn last line), atomic snapshots, File::lock, the events reader (`events::read_from`: counts unknown kinds, unknown fields and torn lines), `replay` (the snapshot read-or-replay, library code with no product caller yet), tailing (with `ui`) (+ tests/, the crate-level integration tests)'."
    sidecar: "2026-10-10-self-healing-state: Project directory structure — `viola-state/` comment drops 'healing owed to the route entry \"Self-healing state\"' and the 'torn / oversize' count wording; names the healing appender, the `replay` module and the crate's first `tests/` directory."
    rationale: "Duplicate of the retired 'healing owed' claim and of the old count names, in the key file. Report Crates / modules: `viola-state` gains the module `replay` and its first crate-level `tests/` directory (two files); Symbols / APIs: the heal landed in the appender, the counts are three new names, `replay` is library code no product code calls. Expected amendments names this site (key file `registries/contracts/architecture/project-directory-structure.md:48`, which 'also holds the one `Self-healing state` hit among the key files')."
    basis: ".andromeda/registries/contracts/architecture/project-directory-structure.md:48"
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Snapshot envelope"
    change: "After the paragraph that begins 'A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored…', add: 'As landed (chunk 2026-10-10-self-healing-state): the fallback is library code in `viola-state` and no product reader takes it yet. `snapshot::read_snapshot_classified` returns `SnapshotRead` (`Present` · `Absent` · `Unreadable` · `Unsupported{v_seen}`) through the same `MAX_FRAME` bound: the envelope's `v` is read first, above 1 is `Unsupported` whatever else the envelope holds, 1 must then parse whole, and anything else (a `v` of 0 included) is `Unreadable`. `replay::read_snapshot_or_replay` replays an unreadable or unsupported snapshot in one pass over `events::read_from(dir, 0)` and logs one WARN `state-recovered` line (`snapshot-replayed`, or `snapshot-unsupported-v` with `v_seen`; none for a present or absent snapshot, none when the replay itself fails); the replay writes no file. It recovers `wheel`, `budget_paused`, `budget_override_until` and `agent_session_id`; `links` replays empty (no `link` / `unlink` kind exists yet) and `dialog_pending` is false. `read_snapshot` keeps its signature and results, and its four product callers take no fallback.'"
    sidecar: "2026-10-10-self-healing-state: Snapshot envelope — as-landed note: `read_snapshot_classified` / `SnapshotRead`, `replay` and `read_snapshot_or_replay` are library code with no product caller; `links` replays empty."
    rationale: "Report Symbols / APIs lands `SnapshotRead`, `read_snapshot_classified`, `Replayed`, `replay` and `read_snapshot_or_replay` and states 'Library code: no product code calls either function', `links` always empty, `read_snapshot`'s four product callers 'not edited, and none of them takes the fallback'. The contract paragraph states the rebuild unqualified and lists `links` among the recovered fields. Expected amendments names this site (`grep -n 'fails to parse' architecture.md`, 1 hit, line 247). The other restatements were read and left: the Stack row 'ORM / migrations' and §Cross-cutting 'Mixed-version tolerance' state the target design, and the Instance snapshot bullets ('a reader that falls back to replay reports `dialog_pending: false`', 'only a replay rebuild lacks it') agree with what landed."
    basis: "architecture.md:247; crates/viola-state/src/snapshot.rs:100-113, 121-143, 147-150; crates/viola-state/src/replay.rs:15-27, 48-73, 81-104"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Filesystem"
    change: "In the `viola-root-watch` bullet, '`Instant::now() + WITHIN` reads 22 sites in 16 files under `tests/`, as measured at chunk 2026-10-09-epoch-3-cleanup-ii's report on the Linux dev host's tree' becomes '`Instant::now() + WITHIN` reads 23 sites in 17 files under `tests/`, as measured at chunk 2026-10-10-self-healing-state's report on the Linux dev host's tree (`tests/chaos_torn_append.rs` adds one site and one file)'."
    sidecar: "2026-10-10-self-healing-state: Filesystem, root watch — the `Instant::now() + WITHIN` count moves 22 sites in 16 files → 23 sites in 17 files."
    rationale: "Report Counts / qualifiers moved: '22 sites in 16 files → 23 sites in 17 files (`tests/chaos_torn_append.rs` adds one site and one file). Stated in `architecture.md:407`' with `grep -n '22 sites'` 1 hit in architecture.md. The second hit the report names, `registries/contracts/test-plan/5-command-implementation.md:33`, is a test-plan key file and belongs to test-plan's detector, not this document."
    basis: "architecture.md:407; tests/chaos_torn_append.rs:45-123"
  - detector: D-arch-decisions
    severity: warning
    section: "§Stack and Technologies → Logging row"
    change: "In the Technology cell, 'tracing-subscriber 0.3.23 (default features off, `fmt,json,registry,std`; root bin only)' becomes 'tracing-subscriber 0.3.23 (default features off, `fmt,json,registry,std`; the root bin is its only product dependent; also a dev-dependency of `viola-state`, at the workspace pin, for that crate's `#[cfg(test)]` `test_capture` module)'."
    sidecar: "2026-10-10-self-healing-state: Stack, Logging — tracing-subscriber is no longer 'root bin only': `viola-state` takes it as a dev-dependency for its test capture."
    rationale: "Report Dependencies: '`tracing-subscriber` (the workspace pin, `=0.3.23`) as a dev-dependency of `viola-state`'; 'No new crate enters the graph, and no product dependency moved'. Crates / modules: `viola-state` gains a `#[cfg(test)]` module `test_capture`. No new library or runtime, so the stack itself holds; only the 'root bin only' qualifier is contradicted. Single occurrence of that qualifier in the document and its key files."
    basis: "crates/viola-state/Cargo.toml:36-37; Cargo.lock:1710; crates/viola-state/src/lib.rs:32-89"
  - detector: D-arch-decisions
    severity: warning
    section: "§Infrastructure Patterns → Crate dependency direction"
    change: "The `viola-state` row gains, at its end: 'with the dev-dependency tracing-subscriber (the workspace pin; the `#[cfg(test)]` `test_capture` module only, no product edge)'."
    sidecar: "2026-10-10-self-healing-state: Crate dependency direction — `viola-state` row records the tracing-subscriber dev-dependency."
    rationale: "The per-crate dependency registry, where the same edge the Stack row's 'root bin only' denied would be looked up; sibling rows already name their dev-dependencies (`viola-core`: serde_json; `viola-agent-claude`: proptest, rstest, insta) while the `viola-state` row names none. Report Dependencies gives the edge. Lowest-weight proposal of the set: the row does not itself restate 'root bin only', so the orchestrator may fold it into the Stack amendment or drop it."
    basis: "crates/viola-state/Cargo.toml:36-37"
    dependent-of: D-arch-decisions
```

## security-plan — the parsed list

```yaml
# D-security-auth: no drift. The report's Changes touch no identity, session, token or key; "No IPC method, endpoint, CLI flag, port, socket, env var or exit code was added or changed".
# D-security-deps: no drift. The one Dependencies entry is `tracing-subscriber` at the existing workspace pin `=0.3.23`, as a dev-dependency of `viola-state` (crates/viola-state/Cargo.toml 36-37, Cargo.lock 1710); "No new crate enters the graph, and no product dependency moved". It is above the §Dependency Security floor `>=0.3.20` and on no ban list.
# D-security-input: no boundary was found unvalidated. Every new read is inside `MAX_FRAME` and the replay writes no file. The drift is the row's stale "as landed" text, which the report lists under "Spec claims disproved by measurement" and "Expected amendments".
proposals:
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → row \"Own state files on read\""
    change: "Keep the Line cap sentence, the `Read::take(MAX_FRAME + 1)` line read and the whole Integrity part; replace the as-landed clause `Skipped{oversize, malformed}` … `an unterminated last line is neither returned nor healed (healing and its count are owed to the working-route entry \"Self-healing state\")` with: the one `events.ndjson` reader (`viola_state::events::read_from`: the wrapper's `wait` scan and `last` rebuild, and the library `viola_state::replay::replay`) counts per read, not logged, `Skipped{unknown_kinds, unknown_fields, torn_lines}` — a line whose `kind` is absent, not a string or not one of the 13 `EventKind`s is not returned (`unknown_kinds`); a known-kind line with a top-level key outside the six or a `data` key outside its kind's row is returned and counted once (`unknown_fields`); an over-long line, a terminated line that is not one JSON object and the unterminated last line of the read are `torn_lines`, the last still neither returned nor rewritten by the reader; no surface shows the counts yet. **Healing is the appender's:** under the lock every append already holds, an append reads the log's last byte and, when it is not LF, writes one LF and the line in a single `write_all` and logs one `state-recovered` WARN (`torn-line-healed`: a detail code, the basename and an offset, no content); a failed last-byte read is the append's `StateError` before any write, and `try_append_event` with the lock held writes and heals nothing. **Snapshot:** `read_snapshot_classified` reads through `take(MAX_FRAME)` and decides on the envelope's `v` first (`Present` · `Absent` · `Unreadable` · `Unsupported{v_seen}`: above 1 is `Unsupported` whatever else the envelope holds, a `v` of 0 is `Unreadable`); `replay` and `read_snapshot_or_replay` write no file and are library code with no product caller, so no product reader takes the replay fallback yet."
    sidecar: "2026-10-10-self-healing-state: §Input Validation row \"Own state files on read\" — reader counts are now `Skipped{unknown_kinds, unknown_fields, torn_lines}`, the unterminated last line is counted, the heal landed in the appender, the classified snapshot read and the library-only replay are recorded; the \"healing owed to Self-healing state\" clause is retired."
    rationale: "The report's Changes list this row twice: \"Spec claims disproved by measurement\" (the row describes `Skipped{oversize, malformed}` and an uncounted, unhealed last line, and names healing as owed to this entry; as landed the appender heals, the reader counts three names and the unterminated last line is counted) and \"Expected amendments\" (security-plan §Input Validation, row \"Own state files on read\", 1 hit, line 239). Symbols / APIs give the new facts: `Skipped`'s fields renamed; unknown-kind lines no longer returned; the heal in `tail` / `write_line`; `read_snapshot_classified` through the same `MAX_FRAME` bound with `v` read first; `replay` / `read_snapshot_or_replay` as library code that no product code calls and that writes no file. No validation is missing: \"Coverage of new surfaces\" reads validation ✓ (`MAX_FRAME`) for the three counts, the classified read and the replay, and the Outcome criterion \"(security) the replay writes no file; every read is inside `MAX_FRAME`\" is met. Severity is the detector's own; the amendment is a description catch-up on a security contract row, not an unvalidated boundary. The Integrity clause needs no change: the two new readers have no product caller, so no new entry point reads state before the strict-modes check."
    basis: "security-plan.md:239 (the row, as the report states it); crates/viola-state/src/events.rs:312-325 (`tail`), 336-342 (the heal's `obs_event!`), 200-217 (`known_kind`, `has_unknown_field`); crates/viola-state/src/snapshot.rs:100-113 (`SnapshotRead`), 121-143 (`read_snapshot_classified`, with `take(MAX_FRAME)` at 130-132); crates/viola-state/src/replay.rs:48-73 (`replay`), 81-104 (`read_snapshot_or_replay`)"
  - detector: D-security-input
    severity: warning
    section: "§Threat Model Summary → Attack surface → vector \"Filesystem state under `~/.viola/`\" → Trust boundary"
    change: "Reword the bullet \"Readers tolerate torn lines and replay the log when a snapshot fails to parse (Standard Contracts: Snapshot envelope).\" to: \"Readers skip and count torn lines, and the next append heals a torn tail. Replaying the log when a snapshot fails to parse, or carries a newer `v`, exists as library code (`viola_state::replay`); no product reader takes that fallback yet (Standard Contracts: Snapshot envelope).\""
    sidecar: "2026-10-10-self-healing-state: §Threat Model Summary, filesystem-state trust boundary — the torn-line and snapshot-replay sentence now names the appender's heal and states the replay as library code no product reader calls yet."
    rationale: "Second occurrence of the mechanism and actor the primary settles: this bullet names \"Readers\" as replaying the log on a snapshot parse failure. The report's Symbols / APIs state the opposite as landed: `read_snapshot`'s four product callers \"were not edited, and none of them takes the fallback\", and for `replay` / `read_snapshot_or_replay` \"Library code: no product code calls either function\"; the heal is \"the appender's\". Applied alone, the primary would leave the row saying no product reader replays while this bullet says readers do. The bullet's wording predates the chunk and sits in the section headed as a verbatim copy of threat-assessment.md, which already carries chunk-dated amendments; the orchestrator may prefer to leave it as stated intent, hence warning. The report gives no security-plan line for this bullet, so none is cited."
    basis: "crates/viola-state/src/replay.rs:81-104 (`read_snapshot_or_replay`); crates/viola-state/src/snapshot.rs:147-150 (`read_snapshot` defined through the classified read)"
    dependent-of: D-security-input
```

## test-plan — the parsed list

```yaml
# D-tests-framework: no drift. The report's runs go through `scripts/agent-run.sh run --unit` / `--integration --filter <nextest filterset>` and `pre-push`; the new cases are inline `#[cfg(test)]` modules, rstest `#[case]`, crate-level `tests/` and a root `chaos_` binary; the one dependency is `tracing-subscriber` at the workspace pin `=0.3.23` as a dev-dependency of `viola-state` (the version §3 → Log format already names).
# D-tests-obs-harness: no drift. Report: "Harness / gate surface: none", no endpoint or status shape changed, `ObsEvent` and `schemas/diag-line.v1.json` unchanged; `state-recovered` is emitted with no `corr`, which is what §3 → Log format and obs-plan's log-format key both state (`corr` = null, written as absence).
# D-tests-coverage: every new path carries unit + integration tests (report, Coverage of new surfaces), so no test is missing. The drift is in test-plan's own description of those tests: three claims the chunk retires (the torn-append mechanism, who heals, what the replay recovers and who rebuilds), one moved count, and one stale coverage list. Titles of the owing route entries are from the chunk's scope.md ("What else is owed elsewhere"), not from the report, which says only "owed by title" — re-derive there.
proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§6 E2E Test Strategy → Chaos suite (the torn-append bullet)"
    change: >-
      Torn append, as landed (chunk 2026-10-10-self-healing-state, `tests/chaos_torn_append.rs` `chaos_torn_tail_left_by_a_stopped_wrapper_is_healed_by_the_next_start`, all three CI OSes): the wrapper is stopped (no kill), `events.ndjson` is shortened with `File::set_len`, and `viola run` is started again on the same home; the count is read through the product reader (`viola_state::events::read_from`, `torn_lines`), the second start's first record `wheel{holder:"driver", cause:"start"}` starts on a fresh line, and the wrapper's role file holds one WARN `state-recovered{detail:"torn-line-healed", file:"events.ndjson"}` line that the case itself holds to `schemas/diag-line.v1.json`. Owed: the `skipped.torn_lines` reading on a surface — `list --json` ("The board: viola list") and `/api/sessions` (Epoch 8); no surface shows the counts yet.
    sidecar: >-
      §6 Chaos suite, torn append: was a kill mid-append (`Process::kill_with(Signal::Kill)`) then `File::set_len`, read on `skipped.torn_lines`; now, as landed in `tests/chaos_torn_append.rs`, a stopped wrapper and a shortened log, the count read through the product reader, the heal's `state-recovered` line asserted and schema-checked by the case; the surface reading owed to "The board: viola list" and the Epoch 8 `/api/sessions` entry.
    rationale: >-
      Report, Spec claims disproved: "test-plan §6 Chaos suite gives the torn-append case as a kill of the wrapper mid-append followed by `File::set_len`, read on `skipped.torn_lines` of a surface. As built: the wrapper is stopped, the log shortened with `File::set_len`, and the count read through the product reader; no surface shows the counts yet." The report's Expected amendments names this site (`test-plan.md:1011`). The chaos tier is covered, but not by the mechanism the bullet mandates.
    basis: "test-plan.md:1011 against tests/chaos_torn_append.rs 45-123"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage triggers → chaos-test (first Required-test-type bullet)"
    change: >-
      stop the wrapper and shorten the last ndjson line with `File::set_len` (no kill mid-write): the reader counts it in `torn_lines` and rewrites nothing; the next append heals it (one LF before its line) and logs one `state-recovered` line
    sidecar: >-
      §1 chaos-test trigger, torn line: was "kill the wrapper mid-write and truncate the last ndjson line: readers count `torn_lines` and heal"; now a stop and a shortened file, the reader counts and the appender heals.
    rationale: >-
      Second occurrence of both retired claims. The kill mid-write is the §6 Chaos mechanism the report disproves, and "readers ... heal" names the wrong actor: report, Symbols / APIs, "the heal is the appender's", and the unterminated last line "is still neither returned nor rewritten by the reader". The report's Expected amendments names this site (`test-plan.md:378`).
    basis: "test-plan.md:378 against crates/viola-state/src/events.rs 312-325"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test pyramid → Chaos / Fault row"
    change: >-
      In the fault list, replace "kill mid-write, `File::set_len` truncation" with "`File::set_len` truncation of a stopped wrapper's log (the torn append is a stop, never a kill mid-write)"; the rest of the row stands.
    sidecar: >-
      §2 Chaos / Fault row: the torn-append fault is a stopped wrapper's log shortened by `File::set_len` (was "kill mid-write, `File::set_len` truncation").
    rationale: >-
      Third occurrence of the kill-mid-write mechanism the §6 Chaos amendment retires, restated without the bullet's tokens in the pyramid table. Report, Spec claims disproved (§6 Chaos suite item) and Expected amendments ("a stop and a shortened file in place of a kill"). The row's other faults are untouched by the report.
    basis: "test-plan.md §2 Test pyramid, Chaos / Fault row (the report gives no line) against tests/chaos_torn_append.rs 45-123"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → viola-state (the torn-line bullet)"
    change: >-
      The reader counts and never panics: `torn_lines` for an over-long line, a terminated line that is not one JSON object and the unterminated last line of the read (neither returned nor rewritten by the reader); `unknown_kinds` for a line whose `kind` is absent, not a string or not one of the 13 (not returned); `unknown_fields`, once per line, for a known-kind line with a top-level key outside the six or a `data` key outside its kind's row (returned). The heal is the appender's: when the log's last byte is not LF, the append writes one LF and its line in a single `write_all`, `append_event_at` hands its builder and returns the line's real start (L + 1 after a heal, L otherwise), `try_append_event` with the lock held writes and heals nothing, and one WARN `state-recovered{torn-line-healed}` line is logged per heal, none without one (unit cases `torn_tail` and `three_counts`, log lines read in memory through the crate's `#[cfg(test)]` `test_capture`).
    sidecar: >-
      §4 viola-state: was "Torn-line healing counts the line in `torn_lines` and never panics"; now the reader's three counts (`unknown_kinds`, `unknown_fields`, `torn_lines`) and the appender's heal with its `state-recovered` line, as landed at chunk 2026-10-10-self-healing-state.
    rationale: >-
      The bullet makes the heal the thing that counts. Report, Symbols / APIs: "the heal is the appender's" (`tail`, `write_line`), while the counts are the reader's and per read, and `Skipped`'s public fields are now `unknown_kinds`, `unknown_fields`, `torn_lines` (they were `oversize`, `malformed`). The report's Expected amendments names this site (`test-plan.md:549`); Outcome: unit filter 43 of 43.
    basis: "test-plan.md:549 against crates/viola-state/src/events.rs 312-325 (the heal) and 200-203, 205-217 (the counts)"
  - detector: D-tests-coverage
    severity: warning
    section: "§6 E2E Test Strategy → Scenario: E5 — snapshot corruption → replay recovery"
    change: >-
      Mark E5 owed, not built (chunk 2026-10-10-self-healing-state built the replay as library code only): its steps need `link` ("Session links"), a budget pause and `list --json` ("The board: viola list"), and no product reader takes the replay fallback yet (first consumer "viola revive"). The replay writes no file, so nothing "rebuilds" `snapshot.json` from it, and it replays `links` empty until the link kinds land. In the Verification signal, `wheel:"human"` does not hold after step 3: the second start's first record is `wheel{holder:"driver", cause:"start"}`, so the wheel reads `driver`. Below the surface the recovery is covered by `crates/viola-state/tests/state_replay.rs` (a snapshot cut short and a `v` 99 snapshot, each replayed from the log and left untouched).
    sidecar: >-
      §6 Scenario E5: marked owed (needs "Session links", "The board: viola list" and a reader that takes the fallback, first "viola revive"); `wheel:"human"` after the second `viola run` struck — the start appends `wheel{driver, start}` (measured by `tests/chaos_torn_append.rs`); "rebuilt snapshot" and recovered `links` qualified: the replay writes no file and replays `links` empty.
    rationale: >-
      Report, Spec claims disproved: "test-plan §6 Scenario E5 states `wheel:"human"` after a second `viola run`. The chaos case reads the second start's first record as `wheel` `{holder:"driver", cause:"start"}` ... E5 is not built by this chunk." Symbols / APIs: `Replayed` has "`links` always empty", "No product code calls either function", "The replay writes no file", and of `read_snapshot`'s four product callers "none of them takes the fallback". The report's Expected amendments names this site (`test-plan.md:937`). This changes a planned scenario's expected signal, so the operator may want to read it before it is applied.
    basis: "test-plan.md:937 against tests/chaos_torn_append.rs 45-123 and crates/viola-state/src/replay.rs 15-27, 81-104"
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → viola-state (the Snapshot envelope and Replay bullets)"
    change: >-
      Snapshot envelope: `read_snapshot_classified` reads `v` first — above 1 is `Unsupported{v_seen}` whatever else the envelope holds, 1 must then parse whole, anything else (a `v` of 0 included) is `Unreadable` — and `read_snapshot_or_replay` replays for both, logging one WARN `state-recovered` line (`snapshot-unsupported-v` with `v_seen`, or `snapshot-replayed`) and none for a present or an absent snapshot; library code, no product reader takes the fallback yet. Replay recovers `agent_session_id`, `wheel`, `budget_paused` and `budget_override_until` from the log with `dialog_pending:false`, and `links` empty until the link kinds land with "Session links"; it writes no file (unit cases `snapshot_cause`, `replay_recovers`).
    sidecar: >-
      §4 viola-state: the snapshot and replay bullets as landed — the classified read (`v` first), the read-or-replay function's two `state-recovered` lines, `links` replayed empty until "Session links", no product reader on the fallback yet.
    rationale: >-
      Duplicate of the claim the E5 amendment retires: "Replay recovers exactly `links`, ..." while the report's `Replayed` has "`links` always empty" and "No `link` or `unlink` variant was added"; and "goes to replay" names no actor while the report says the replay is "Library code: no product code calls either function". Report's Expected amendments lists §4 viola-state.
    basis: "test-plan.md §4 viola-state, the two bullets after :549 (the report gives no line for them) against crates/viola-state/src/replay.rs 15-27, 48-73 and crates/viola-state/src/snapshot.rs 100-113, 121-143"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage triggers → chaos-test (second Required-test-type bullet)"
    change: >-
      corrupt `snapshot.json` or give it an unsupported `v`: the replay recovers only `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until` and `links` (empty until the link kinds land with "Session links"), with `dialog_pending:false`; it writes no file, and which reader takes the fallback is owed, first to "viola revive"
    sidecar: >-
      §1 chaos-test trigger, snapshot: "rebuild by replay recovers only `links`, ..." qualified — `links` replays empty until "Session links", the replay writes no file, no product reader takes the fallback yet.
    rationale: >-
      Second duplicate of the claim the E5 amendment retires, in the trigger list: "rebuild by replay recovers only `links`, ..." against the report's "`links` always empty", "The replay writes no file" and "none of them takes the fallback" (Symbols / APIs, `viola-state::replay` and `viola-state::snapshot`).
    basis: "test-plan.md, the bullet after :378 (the report gives no line for it) against crates/viola-state/src/replay.rs 15-27"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → 5-command implementation"
    change: >-
      In `run` step 2's root-chain sentence: `Instant::now() + WITHIN` reads 23 sites in 17 files under `tests/`, as measured at chunk 2026-10-10-self-healing-state's report (was 22 sites in 16 files at chunk 2026-10-09-epoch-3-cleanup-ii; `tests/chaos_torn_append.rs` adds one site and one file).
    sidecar: >-
      §3 → 5-command implementation: the root waits on a child read 23 sites in 17 files (was 22 in 16); `tests/chaos_torn_append.rs` adds one.
    rationale: >-
      Report, Counts / qualifiers moved: "22 sites in 16 files → 23 sites in 17 files (`tests/chaos_torn_append.rs` adds one site and one file). Stated in `architecture.md:407` and in the key file `registries/contracts/test-plan/5-command-implementation.md:33` (`grep -n '22 sites'`: 1 hit in each, 0 elsewhere in the seven masters and the key files)". One occurrence in this document; the architecture one is that detector's. No detector of mine names a count, so it rides on D-tests-coverage as the new root test's effect on a measured test-side figure.
    basis: ".andromeda/registries/contracts/test-plan/5-command-implementation.md:33"
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Cross-module patterns covered → On-disk"
    change: >-
      As landed (chunk 2026-10-10-self-healing-state), `crates/viola-state/tests/` holds two files: `state_events.rs` — all 13 kinds round-trip the six-key line through the crate's writer and reader with zero on all three counts, lines outside the contract are counted while known kinds are still returned, and a log cut short takes the next append on a fresh line — and `state_replay.rs` — a written snapshot reads present, a snapshot cut short and a `v` 99 snapshot are each replayed from the log with the file's bytes, modification time and the dir's file names unchanged. The root `tests/cli_send.rs` and `tests/cli_answer.rs` assert that a product-written log reads zero on all three counts (`assert_reads_clean`).
    sidecar: >-
      §5 On-disk: the crate-level `viola-state` suite exists (`state_events.rs`, `state_replay.rs`), with its cases named; the CARRY from chunk 2026-09-27-instance-state-and-start-order is discharged; `path2` and the two `path4` cases read zero counts on product-written logs.
    rationale: >-
      Not a contradicted claim: the round-trip bullet's `crates/viola-state/tests/` now exists, where before this chunk it did not (report, Crates / modules: `viola-state` gains "its first crate-level `tests/` directory (two files)"). The list is stale by omission: the integration-tier cases for the counts, the heal and the replay are named nowhere in §5. The report's Expected amendments lists §5 On-disk; Outcome: integration filter 43 of 43. Drop this one if as-landed records are not wanted where no claim is wrong.
    basis: "crates/viola-state/tests/state_events.rs 118-158, 160-197, 199-246; crates/viola-state/tests/state_replay.rs 103-114, 116-150, 152-173; tests/cli_send.rs 169-183; tests/cli_answer.rs 150-164"
```

## obs-plan — the parsed list

```yaml
# D-obs-pii: evaluated, no drift. The only logging added is state-recovered with a detail code, a basename file, an integer offset and an integer v_seen; both new spans are skip_all with neither err nor ret (report, Symbols / APIs and Coverage of new surfaces).
# D-obs-instrumentation: no uninstrumented path. Every new operation carries what §4-§6 require; the three proposals below bring the doc statement of emitter, cadence and per-detail fields current, as the report Expected amendments names for obs-plan §4 Edge flows and §6 Additive field catalog.
# D-obs-stack: the library and pin match §3 (tracing-subscriber =0.3.23); the drift is the "root bin only" placement claim, stated three times in two key files.
proposals:
  - detector: D-obs-stack
    severity: warning
    section: '§3 → Bootstrap phases (derive for route / setup-project)'
    change: 'logger-stack-install, the sentence "Root bin (`viola::obs`), the only crate that depends on tracing-subscriber" now reads: the root bin (`viola::obs`) is the only crate with a product dependency on tracing-subscriber (`MillisUtc`, `viola_obs_init`, `viola_panic_hook`); `viola-state` takes the same workspace pin `=0.3.23` as a dev-dependency only, for its `#[cfg(test)]` module `test_capture`, which captures emitted records in memory for unit cases; no product dependency moved and no new crate entered the graph.'
    sidecar: '2026-10-10-self-healing-state: Bootstrap phases / logger-stack-install — root bin is the only product dependent on tracing-subscriber; viola-state dev-depends on the workspace pin =0.3.23 for its cfg(test) test_capture module.'
    rationale: 'The report Dependencies bullet lists `tracing-subscriber` (the workspace pin, `=0.3.23`) as a dev-dependency of `viola-state`, with one new `Cargo.lock` edge (line 1710), no new crate in the graph and no product dependency moved; Crates / modules adds the `#[cfg(test)]` module `test_capture` in `crates/viola-state/src/lib.rs` 32-89. The key file states the root bin is "the only crate that depends on tracing-subscriber", which a second dependent retires. Version and library are on-spec; only the placement claim is stale.'
    basis: 'crates/viola-state/Cargo.toml:36-37'
  - detector: D-obs-stack
    severity: warning
    section: '§3 → Bootstrap phases (derive for route / setup-project)'
    change: 'logger-stack-install, the bullet "Add `tracing-subscriber 0.3.23` (`default-features = false`, `fmt,json,registry,std`; no `chrono` feature, D-26) to the root bin only" now ends: to the root bin only as a product dependency; a crate may take the workspace pin as a dev-dependency for in-memory capture in its own tests, as `viola-state` does.'
    sidecar: '2026-10-10-self-healing-state: Bootstrap phases / logger-stack-install — the "to the root bin only" install bullet scoped to product dependencies; viola-state dev-dependency noted.'
    rationale: 'Second statement of the same root-bin-only claim in the same key file, two bullets above the primary site. The report Dependencies bullet (`crates/viola-state/Cargo.toml` 36-37) makes it untrue as worded; left alone it survives a single-site apply.'
    basis: 'crates/viola-state/Cargo.toml:36-37'
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: '§3 → OTel SDK init'
    change: 'SDK packages, the bullet "Cargo entry for the root bin only: `tracing-subscriber = { version = "0.3.23", default-features = false, features = ["fmt", "json", "registry", "std"] }`" now reads: product Cargo entry for the root bin only (same entry text); `viola-state` carries the workspace pin `=0.3.23` under dev-dependencies for its `#[cfg(test)]` capture module, and no other product crate depends on it.'
    sidecar: '2026-10-10-self-healing-state: OTel SDK init / SDK packages — the root-bin-only Cargo entry scoped to product dependencies; viola-state dev-dependency on the workspace pin noted.'
    rationale: 'Third statement of the root-bin-only claim, in the other key file D-obs-stack reads. Same basis: the report Dependencies bullet lists the dev-dependency of `viola-state` at the workspace pin `=0.3.23`.'
    basis: 'crates/viola-state/Cargo.toml:36-37'
    dependent-of: D-obs-stack
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage → Edge flows → E5'
    change: 'E5 (snapshot corruption) now reads: `state-recovered` at WARN with no `corr` is written by `viola_state::replay::read_snapshot_or_replay` — `detail:"snapshot-replayed"` for an unreadable snapshot, `detail:"snapshot-unsupported-v"` with `v_seen` for a newer `v` (`v_seen` on that detail only), `file:"snapshot.json"`; no line for a present or an absent snapshot, and none when the replay itself fails (D-03). The spans are `state.replay` and `state.snapshot_recover`, both `skip_all` with neither `err` nor `ret`. As landed at chunk 2026-10-10-self-healing-state this is library code: no product code calls `replay` or `read_snapshot_or_replay`, and none of the four product callers of `read_snapshot` takes the fallback, so no product run writes these lines yet.'
    sidecar: '2026-10-10-self-healing-state: §4 Edge flows E5 — names the emitter (read_snapshot_or_replay), its cadence, v_seen on snapshot-unsupported-v only, the spans state.replay / state.snapshot_recover, and the as-landed limit (library code, no product caller).'
    rationale: 'Report Symbols / APIs: the event `state-recovered` now has product emitters (0 → 2 in Counts / qualifiers moved); `read_snapshot_or_replay` writes `snapshot-replayed` or `snapshot-unsupported-v` with `v_seen`, `file` `snapshot.json`, none for present or absent and none when the replay fails; spans `state.replay` and `state.snapshot_recover` are `skip_all`; "Library code: no product code calls either function", and the four product callers of `read_snapshot` do not take the fallback. obs-plan.md:728 gives the line as one shape with `v_seen` for both details and names no emitter, cadence or span, and reads as if the E5 flow writes it in a product run. The code is instrumented per §4-§6; the doc sentence is what lags. Named by the report Expected amendments (obs-plan §4 Edge flows).'
    basis: 'crates/viola-state/src/replay.rs:81-104 (the `obs_event!` at 96-102); obs-plan.md:728'
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage → Edge flows'
    change: 'Add one Edge-flow line beside E5 for the torn append (chaos): the appender heals, not the reader. Under the lock every append already holds, an append that finds the last byte of `events.ndjson` is not LF writes one LF and its line in a single `write_all`, then writes one `state-recovered{detail:"torn-line-healed", file:"events.ndjson", offset}` at WARN with no `corr`, where `offset` is the log length L before the append (the appended line starts at L + 1) — one line per heal, none for an append that did not heal, and none from `try_append_event` while the lock is held (it writes and heals nothing). All three public appenders go through it. The reader neither heals nor logs: its per-read counts `Skipped{unknown_kinds, unknown_fields, torn_lines}` are not logged and no surface shows them yet. Witnessed at run level by `tests/chaos_torn_append.rs`, which reads the line in the wrapper role file and holds it to `schemas/diag-line.v1.json` itself.'
    sidecar: '2026-10-10-self-healing-state: §4 Edge flows — torn-append line added: the appender writes state-recovered{torn-line-healed, events.ndjson, offset L}, one per heal; the reader counts are per read and unlogged.'
    rationale: 'Report Symbols / APIs: "the heal is the appender", `write_line` logs the heal (`obs_event!` at 336-342), all three public appenders go through it, the line is WARN with no `corr`, `detail` `torn-line-healed`, `file` `events.ndjson`, `offset` L, one line per heal and none otherwise; `try_append_event` with the lock held writes and heals nothing; the reader counts "are per read and are not logged". Outcome: met in memory (unit) and in a real wrapper role file (the chaos case, all three CI runners). §4 Edge flows carries no statement of where `torn-line-healed` is written or how often; the detail exists only as a value in the §6 catalog row. The path is instrumented; the doc does not yet say by whom. Named by the report Expected amendments ("the two emitters, one line per heal").'
    basis: 'crates/viola-state/src/events.rs:336-342 (the heal log; `tail` at 312-325); tests/chaos_torn_append.rs:45-123'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§6 Log Coverage → Additive field catalog → `state-recovered` (D-03) row'
    change: 'The `state-recovered` (D-03) row now scopes its fields per detail: `detail` (`torn-line-healed|snapshot-replayed|snapshot-unsupported-v`), `file` (basename only: `events.ndjson` on `torn-line-healed`, `snapshot.json` on the two snapshot details), `offset` (`torn-line-healed` only: the log length before the healing append), `v_seen` (`snapshot-unsupported-v` only).'
    sidecar: '2026-10-10-self-healing-state: §6 Additive field catalog, state-recovered row — offset scoped to torn-line-healed, v_seen to snapshot-unsupported-v, file basenames named per detail.'
    rationale: 'Same claim as the E5 site in table form: obs-plan.md:826 lists `offset` and `v_seen` for the event as a whole. As landed (report, Log lines) the heal line carries `detail`, `file` `events.ndjson` and `offset` L, and the snapshot lines carry `file` `snapshot.json` with `v_seen` only on `snapshot-unsupported-v`; Coverage of new surfaces gives the two field sets as "a detail code, a basename, an offset" and "a detail code, a basename, an integer `v_seen`". The row stays a valid allow-list (`ObsEvent` and `schemas/diag-line.v1.json` are unchanged); the amendment only adds the per-detail scoping the two emitters fixed. Named by the report Expected amendments (obs-plan §6 Additive field catalog).'
    basis: 'crates/viola-state/src/events.rs:336-342; crates/viola-state/src/replay.rs:96-102; obs-plan.md:826'
    dependent-of: D-obs-instrumentation
```

## Dispositions (Validate)

Rejected before the checks for a source the report does not carry: 0. Every line number a proposal cites is the
report's or its last section's. Check 2: no two proposals edit one section in opposing directions. Check 3: the
five deviations are justified and inside the intent; the scope record holds no line.

| id | proposal | disposition | decided by |
|---|---|---|---|
| A1 | architecture [Database / State Store], the as-landed sentence | apply | check 5: expected amendment 1 names the change; playbook "Accurate this-chunk addition" |
| A2 | architecture [Database / State Store], the decision sentence "Readers heal a torn last line." | escalated, resolved, apply | check 1: a locked decision's reword is not routine; the operator chose the appender wording at the P2 halt (`inputs#I5`) |
| A3 | architecture §Infrastructure Patterns → Project directory structure, the `viola-state/` comment | apply | check 5: expected amendment 2 |
| A4 | architecture §Standard Contracts → Snapshot envelope, as landed | apply | check 5: expected amendment 3 |
| A5 | architecture §Occupied Resources → Filesystem, the wait count 22/16 → 23/17 | apply | playbook "A count in a master that carries no rule": the site states its rule; both sites move in this pass (with T8) |
| A6 | architecture §Stack, Logging row, "root bin only" | apply, re-derived | playbook "Accurate this-chunk addition"; the applied text names both library crates with the dev-dependency (the report's Dependencies bullet) |
| A7 | architecture §Infrastructure Patterns → Crate dependency direction, the `viola-state` row | apply | same rule; sibling rows name their dev-dependencies |
| S1 | security-plan §Input Validation, row "Own state files on read" | apply | check 5: expected amendment 4. The detector's `escalate` is its default severity; it found no unvalidated boundary and nothing widens |
| S2 | security-plan Threat Model Summary, the filesystem-state trust-boundary bullet | apply | playbook "Verbatim upstream copy kept current"; a duplicate of the claim S1 settles |
| T1 | test-plan §6 Chaos suite, the torn-append bullet | apply | check 5: expected amendment 5; check 6 |
| T2 | test-plan §1 chaos-test trigger, the torn-line bullet | apply | duplicate of T1's claim; "Verbatim upstream copy kept current" |
| T3 | test-plan §2 Test pyramid, Chaos / Fault row | apply | duplicate of T1's claim |
| T4 | test-plan §4 viola-state, the torn-line bullet | apply | check 5: expected amendment 5 |
| T5 | test-plan §6 Scenario E5 | apply | check 5: expected amendment 5 names the change, and the operator's P5 word names it ("E5 owed as an amendment", `inputs#I2`); check 6 |
| T6 | test-plan §4 viola-state, the Snapshot envelope and Replay bullets | apply | duplicate of T5's claim; expected amendment 5 lists §4 |
| T7 | test-plan §1 chaos-test trigger, the snapshot bullet | apply | duplicate of T5's claim |
| T8 | test-plan §3 → 5-command implementation, the wait count | apply | with A5: every site of the count in one pass |
| T9 | test-plan §5 On-disk, the crate-level suite as landed | apply | check 5: expected amendment 5 lists §5 |
| O1 | obs-plan §3 → Bootstrap phases, "the only crate that depends on tracing-subscriber" | apply, re-derived | as A6 |
| O2 | obs-plan §3 → Bootstrap phases, "to the root bin only" | apply, re-derived | as A6 |
| O3 | obs-plan §3 → OTel SDK init, "Cargo entry for the root bin only" | apply, re-derived | as A6 |
| O4 | obs-plan §4 Edge flows, E5 | apply | check 5: expected amendment 6 |
| O5 | obs-plan §4 Edge flows, a torn-append line | apply | check 5: expected amendment 6 |
| O6 | obs-plan §6 Additive field catalog, the `state-recovered` row | apply | check 5: expected amendment 6 |

Check 5, the plan's six expected amendments: each is matched (1 → A1, A2; 2 → A3, A5, T8; 3 → A4; 4 → S1; 5 →
T1, T4, T5, T6, T9; 6 → O4, O5, O6). The ledger-note entry was written at phase P5; nothing is owed at P7.3.

Check 6, the report's four disproved claims: E5 → T5; the as-landed reader text in architecture and
security-plan → A1, S1; the chaos suite's kill → T1, T2, T3; the plan's notes on its G2 and G4 entries → not a
master's sentence, routed to curation (P3).

Escalations: 1 (A2), resolved with the operator at the halt. No playbook rule is proposed: the answer settles
one sentence of one locked decision, and the class already has its rule ("a reversal of a locked decision …
escalate that once to ratify it").
