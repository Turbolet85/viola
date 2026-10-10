# Report — 2026-10-10-self-healing-state

**Chunk:** Self-healing state — torn-line healing, snapshot rebuild by log replay, unknown kinds and fields counted
and surfaced, newer-snapshot and newer-peer tolerance (the cut of the P5 review: scope items 1 to 5, `inputs#I2`)
**Date:** 2026-10-10
**Commits:** `0af8283 chore(2026-10-10-self-healing-state): operator pre-CI commit, for the run this chunk's
verdict reads` (`git log 2f1efe3..HEAD`, one commit; the base is the pre-CI commit's parent `2f1efe3a8962`)

## Changes (structured — detectors read this)
- **Files:** 12 source, test and manifest files (`gate.py scope`: changed 12 · listed 12), every coordinate below
  from the last section's listing. Modified, 8: `crates/viola-core/src/lib.rs`, `crates/viola-state/src/events.rs`,
  `crates/viola-state/src/snapshot.rs`, `crates/viola-state/src/lib.rs`, `crates/viola-state/Cargo.toml`,
  `Cargo.lock`, `tests/cli_send.rs`, `tests/cli_answer.rs`. New, 4: `crates/viola-state/src/replay.rs`,
  `crates/viola-state/tests/state_events.rs`, `crates/viola-state/tests/state_replay.rs`,
  `tests/chaos_torn_append.rs`. Nothing under `src/`, `tests/support/`, `schemas/`, `fixtures/`, `scripts/`,
  `.github/` or the root `Cargo.toml` changed (the plan's preservation guard against `2f1efe3a8962` exits 0).
  Beside them: this chunk's `evidence/red-green.md` and `evidence/operator-pass.md`.
- **Symbols / APIs:**
  - `viola-core`, `EventKind` (`crates/viola-core/src/lib.rs`): `ALL`, the 13 variants in declaration order
    (66-81); `from_name(&str) -> Option<Self>`, the kind a kebab name spells (93-96); `data_keys(self) ->
    &'static [&'static str]`, the `data` keys of each kind as architecture's per-kind list gives them (98-116).
    `as_str` and `WAIT_WAKE` are unchanged. No `link` or `unlink` variant was added.
  - `viola-state::events` (`crates/viola-state/src/events.rs`): **the heal is the appender's.** `tail` (312-325)
    reads the log's length and, when it is not empty, its last byte, under the lock every append already holds;
    `write_line` writes one LF and the line in a single `write_all` when that byte is not LF, then logs the
    heal (`obs_event!` at 336-342). All three public appenders (`append_event`, `append_event_at`,
    `try_append_event`) go through it; their signatures are unchanged and none of their seven product callers
    was edited. `append_event_at` now hands its builder, and returns, the offset the line really starts at:
    L + 1 after a heal, L otherwise (`Tail::line_start`, 305-309). `end_offset` still returns the file's length.
    `try_append_event` with the lock held writes and heals nothing. A failure to read the last byte is the
    append's `StateError`, before any write.
  - **The reader's counts.** `Skipped`'s public fields are now `unknown_kinds`, `unknown_fields`, `torn_lines`
    (they were `oversize`, `malformed`). `LoggedLines::skipped` still has no product caller. `read_from`'s
    signature, `LoggedLine` and the offsets are unchanged, and its two product callers (`rebuild` and `scan` in
    `src/run/wait.rs`) were not edited. What changed in what it returns: a line whose `kind` is absent, not a
    string or not one of the 13 is no longer returned (counted `unknown_kinds`); a known-kind line with a
    top-level key outside the six, or a `data` key outside its kind's row, is returned and counted once
    (`unknown_fields`; `known_kind` 200-203, `has_unknown_field` 205-217); `torn_lines` counts an over-long
    line, a terminated line that is not one JSON object, and the unterminated last line of the read, which is
    still neither returned nor rewritten by the reader. The counts are per read and are not logged.
  - `viola-state::snapshot` (`crates/viola-state/src/snapshot.rs`): `pub enum SnapshotRead { Present, Absent,
    Unreadable, Unsupported { v_seen: u64 } }` (100-113) and `pub fn read_snapshot_classified` (121-143), through
    the same `MAX_FRAME` bound. The envelope's `v` is read first: above 1 is `Unsupported` whatever else the
    envelope holds; 1 must then parse whole; anything else, a `v` of 0 included, is `Unreadable`.
    `read_snapshot` keeps its signature and results and is defined through the new read (147-150); its four
    product callers were not edited, and none of them takes the fallback.
  - `viola-state::replay` (new, `crates/viola-state/src/replay.rs`): `pub struct Replayed` (15-27: `wheel`,
    `budget_paused`, `budget_override_until`, `agent_session_id` as options, `links` always empty,
    `dialog_pending` always false, `skipped`); `pub enum ReplayCause` (29-34); `pub enum Recovered` (36-46);
    `pub fn replay` (48-73), one pass over `read_from(dir, 0)`; `pub fn read_snapshot_or_replay` (81-104).
    **Library code: no product code calls either function.** The replay writes no file.
  - Spans, both `skip_all` with neither `err` nor `ret`: `state.replay`, `state.snapshot_recover`.
  - Log lines: the event `state-recovered` now has product emitters, two of them, each at WARN with no `corr`:
    the healing append (`detail` `torn-line-healed`, `file` `events.ndjson`, `offset` L: one line per heal, none
    for an append that did not heal) and `read_snapshot_or_replay` (`detail` `snapshot-replayed` for an
    unreadable snapshot, `snapshot-unsupported-v` with `v_seen` for a newer one, `file` `snapshot.json`; none
    for a present or an absent snapshot, and none when the replay itself fails). Before this chunk no product
    code emitted the event. `ObsEvent` and `schemas/diag-line.v1.json` are unchanged.
  - No IPC method, endpoint, CLI flag, port, socket, env var or exit code was added or changed.
- **Crates / modules:** `viola-state` gains the module `replay`, a `#[cfg(test)]` module `test_capture`
  (`crates/viola-state/src/lib.rs` 32-89) and its first crate-level `tests/` directory (two files). The root
  package gains one test binary, `chaos_torn_append`, auto-discovered: the root `Cargo.toml` holds no `[[test]]`
  entry for it and was not edited. No crate was added or removed.
- **Dependencies:** `tracing-subscriber` (the workspace pin, `=0.3.23`) as a dev-dependency of `viola-state`
  (`crates/viola-state/Cargo.toml` 36-37); `Cargo.lock` gains that one edge (line 1710). No new crate enters the
  graph, and no product dependency moved. `viola-state` is the second library crate with this dev-dependency:
  `viola-channel` already carries it for its own `#[cfg(test)]` capture (`research.md`, Patterns detected;
  `grep -n tracing-subscriber crates/*/Cargo.toml`: 2 hits, `viola-channel` and `viola-state`). The root bin
  stays the only crate with a product dependency on it.
- **Schema / config:** none. The event line, the snapshot envelope, both diagnostics schemas and every config
  key are as they were.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - the root waits on a child, by the pattern `Instant::now\(\) \+ WITHIN` under `tests/` (`grep -rnE`): 22 sites
    in 16 files → **23 sites in 17 files** (`tests/chaos_torn_append.rs` adds one site and one file). Stated in
    `architecture.md:407` and in the key file `registries/contracts/test-plan/5-command-implementation.md:33`
    (`grep -n '22 sites'`: 1 hit in each, 0 elsewhere in the seven masters and the key files);
  - product emitters of `state-recovered`: 0 → 2 (above);
  - crate-level `tests/` directories outside `viola-e2e`: 1 (`viola-channel`) → 2;
  - `EventKind` variants: 13, unchanged; `ObsEvent` values: 19, unchanged;
  - test totals, read from the runs, stated in no master (`grep -cE '\b(1502|1857|1886|1853)\b'` over the seven
    masters and the key files: 0 hits): unit 1502 → 1540 on the dev host; the three CI `test` jobs 1886 / 1853 /
    1857 → 1931 / 1898 / 1902.
- **Dev-tool versions:** none.
- **Harness / gate surface:** none. No harness verb, script, workflow, nextest profile or verdict shape changed.
- **Cross-project / external claims:**
  - CI run `ci#38046300968` (`push`, attempt 1, the only run on its sha) measured `0af82832bfa6` and concluded
    `success`, 15 of 15 checks (`evidence/operator-pass.md`, step 5). The verdict was taken on the pre-CI
    commit's tree; this wrap's commit adds spec, route and record files to it and no source.
  - `I1 · message (the /andromeda-phase invocation) · copy · n/a — a message has no live source`, cited.
  - `I2 · message (the P5 review word) · copy · n/a`, cited.
  - `I3 · message (the /andromeda-implement invocation) · copy · n/a`: the operator's word that the implementer
    runs the operator pass, with no live start and no mutation run. `inputs.py verify` printed it `UNCITED`
    before this report existed; it is cited here (`inputs#I3`) and by `evidence/operator-pass.md`.
  - `I4 · message (the /andromeda-wrap-session invocation) · copy · n/a`, snapped at this wrap and cited here
    (`inputs#I4`): the founder's word of 2026-10-10T10:26:30Z as the operator relays it (a narrow route entry
    for `viola verify`'s leftover dirs), the five CARRYs, and the `events.md` curation. The wrap did not see the
    overseer dialog; the snapshot holds the operator's relay of it.
  - No entry reads `drifted`, `vanished` or `broken`; `unparsed 0`.
- **Reverted / negative API facts:** none shipped and then removed. Three one-off controls each neutralised a
  guard in product code and put it back; the four touched files hash the same after as before
  (`evidence/red-green.md`, section 3). The API first landed as stubs with no behaviour, for the red reading,
  and the bodies replaced them in the same session.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - **test-plan §6 Scenario E5** states `wheel:"human"` after a second `viola run`. The chaos case reads the
    second start's first record as `wheel` `{holder:"driver", cause:"start"}` (`tests/chaos_torn_append.rs`
    45-123, green on the dev host and on all three CI runners). E5 is not built by this chunk.
  - **architecture §Established Decisions [Database / State Store]** and **security-plan §Input Validation**
    (row "Own state files on read") describe the reader as landed with the counts `Skipped{oversize,
    malformed}` and an uncounted, unhealed last line, and name healing as owed to this entry. As landed now: the
    appender heals, the reader counts three names, the unterminated last line is counted (`events.rs` unit
    cases `three_counts` and `torn_tail`, 43 of 43).
  - **test-plan §6 Chaos suite** gives the torn-append case as a kill of the wrapper mid-append followed by
    `File::set_len`, read on `skipped.torn_lines` of a surface. As built: the wrapper is stopped, the log
    shortened with `File::set_len`, and the count read through the product reader; no surface shows the counts
    yet.
  - **The plan's notes on its `g2-zero-panics.sh` and `schema-check` entries** say they read the kept test
    homes, the chaos home among them. On the dev host no `.ndjson` file under `target/e2e-home` held a
    `state-recovered` line after two whole-block firings (`grep -rl`, 0 files): a local run removes a test
    home when its test ends. This is a plan sentence, not a master's. The chaos case holds its line to the
    schema itself, and CI keeps its homes.
- **Expected amendments (from plan):**
  - architecture §Established Decisions [Database / State Store], the "as landed" and "owed" sentences —
    carried: Symbols / APIs (the heal, the counts, the replay as library code) and Spec claims disproved.
    Site: `grep -n 'Self-healing state' architecture.md`, 1 hit, line 50.
  - architecture §Infrastructure Patterns → Project directory structure, the `viola-state` comment and the root
    wait count — carried: Crates / modules and Counts / qualifiers moved. Sites: the key file
    `registries/contracts/architecture/project-directory-structure.md:48` (`grep -n 'viola-state/'`, 1 hit;
    it also holds the one `Self-healing state` hit among the key files); the count at `architecture.md:407`
    and `registries/contracts/test-plan/5-command-implementation.md:33`.
  - architecture §Standard Contracts → Snapshot envelope, as landed — carried: Symbols / APIs (no reader takes
    the fallback; `links` replays empty). Site: `grep -n 'fails to parse' architecture.md`, 1 hit, line 247.
  - security-plan §Input Validation, row "Own state files on read" — carried: Symbols / APIs and Spec claims
    disproved. Site: `grep -n 'Own state files on read' security-plan.md`, 1 hit, line 239.
  - test-plan §4 viola-state, §5 On-disk, §6 Chaos suite and §6 Scenario E5 — carried: Symbols / APIs, Crates /
    modules and Spec claims disproved (a stop and a shortened file in place of a kill; the surface readings and
    E5 owed by title). Sites: `test-plan.md:549` (§4), `:378` and `:1011` (chaos), `:937` (E5);
    `grep -c 'E5' test-plan.md` 2 hits.
  - obs-plan §4 Edge flows and §6 Additive field catalog, where `state-recovered` is written and its cadence —
    carried: Symbols / APIs (the two emitters, one line per heal). Sites: `grep -c 'state-recovered'
    obs-plan.md` 6 hits (`:532`, `:537`, `:728`, `:770`, `:826`, `:844`); key files
    `registries/contracts/obs-plan/log-format-json-schema.md` 2 hits, `registries/contracts/test-plan/log-format.md`
    1, `registries/contracts/a11y-plan/focus-management-test-harness.md` 1.
  - The plan's route proposals (a new entry for the killed-verify leftover dirs; five CARRYs) are not
    amendments: P5 of this wrap, on `inputs#I4`.
  - `matrix#v1-41 notes` — written at phase P5 (`matrix.py note`, 2026-10-10T10:22Z); no write is owed at P7.3.
- **Coverage of new surfaces:**
  - the healing append (every append gains one one-byte read; a heal adds one byte and one log line) →
    validation n/a · instrumentation log✓ (`state-recovered`, WARN) · PII redacted✓ (a detail code, a basename,
    an offset) · tests unit + integ (crate-level and a root chaos case) · a11y n/a · tokens n/a
  - the reader's three counts → validation ✓ (`MAX_FRAME` line cap, unchanged) · instrumentation n/a (obs-plan
    names no process-log carrier for a read's counts) · PII n/a · tests unit + integ · a11y n/a · tokens n/a
  - `read_snapshot_classified` → validation ✓ (`MAX_FRAME`) · instrumentation n/a · PII n/a · tests unit + integ
    · a11y n/a · tokens n/a
  - `replay` and `read_snapshot_or_replay` → validation ✓ (through `read_from`'s cap) · instrumentation span✓
    + log✓ · PII redacted✓ (a detail code, a basename, an integer `v_seen`) · tests unit + integ · a11y n/a ·
    tokens n/a
  - no UI element, no CLI line and no terminal write was added.

## Deviations from intent
- **Step 3's red reading needed the new API to exist.** The cases do not compile without the new names, so the
  API landed first as stubs with no behaviour and the red reading was taken on that tree. 33 of the 39 inline
  cases of steps 4 to 7 and 6 of the 7 new crate-level and root cases failed on an assertion. The seven that
  could not fail there assert that something does not happen; the plan's sentence "every new case fails" did
  not hold for them.
- **Two controls beyond the plan's one.** The plan's key-table control turned `path2` red. An absence control
  (three guards neutralised) turned the five remaining absence cases of the unit filter red. The two `path4`
  cases' zero count has no red reading: their logs hold no `prompt-submitted` line.
- **The chaos case stops its second wrapper before reading the files.** The plan lists the assertions before
  the stop. The wait for the `state-recovered` line still runs on the live wrapper under `WITHIN`; stopping
  first means no read meets an append part-way.
- **One assertion added to the chaos case:** its `state-recovered` line must pass the diag-line schema, with
  its own red control, because the local schema-check entry never reads that line (Spec claims disproved, last
  item). The block was fired a second time on that tree.
- **The classified read decides on `v` first**, so a newer envelope is `Unsupported` even when its other
  members would not parse as a v1 envelope.
- scope record: none — `gate.py scope` clean, 0 recorded (base `2f1efe3a`, changed 12 · listed 12).

## Decisions & corrections
- The operator's word for implement (`inputs#I3`): no live `claude` start, no mutation run, the implementer
  runs the operator pass with the `ci.py conclusion` read.
- The founder's word, relayed by the operator at this wrap (`inputs#I4`): the killed-verify leftover dirs get a
  narrow route entry. `viola verify` handles Ctrl-C and TERM and removes only what it created in that run,
  nothing older and nothing foreign; what a SIGKILL or a crash leaves stays by hand. `viola revive` stays the
  head.
- The operator's direction for curation (`inputs#I4`): the `events.md` sentence that healing "lands with" this
  entry is stale.
- The operator's answers at this wrap's P2 halt (`inputs#I5`): architecture's State Store decision sentence is
  reworded to the appender ("the invariant stands … only the actor differs, as the approved plan fixed it"),
  and the new route entry "Interrupted verify cleanup" goes first in Epoch 5, ahead of "Paste newline ledger
  row", with the text as proposed; Epoch 4 stays at 10.
- A test home is removed when its test ends unless a run keeps homes, so a local G2 or G4 entry reads no line
  a test's own home held. A case whose new log line must be held to the schema asserts it itself.
- A case that asserts an absence cannot read red on a tree without the behaviour. Its red reading is a control
  that forces the behaviour on.
- The `stamp-ahead` PostToolUse hook reads a literal timestamp in a test fixture as a record stamp, and blocks
  one dated later than the clock. A fixture's literal times are dated in the past.
- clippy's `suspicious_map` refuses `.map(..).count()` over an iterator of results; the helper collects and
  takes the length.
- Shell slips of the session, each against `host-linux.md` Paths: a `cd` outside a subshell four times
  (implement twice, this wrap twice). Three targeted the repository root and moved nothing. The fourth moved
  the working directory into `.andromeda/` for one call and was undone in the next.
- Waits on a backgrounded gate call were made by a polling loop on its output file in place of the completion
  notice.

## Outcome
- **Acceptance criteria**, each re-read against the diff:
  - (arch) the healed file is the prior bytes, one LF, the line and its LF; never shorter; an earlier offset
    still starts the same line — met (`torn_tail` unit cases; `state_events` third case).
  - (arch) `append_event_at` returns and hands L + 1 after a heal, L otherwise — met (unit).
  - (obs) one `state-recovered` line per heal with the five facts and no `corr`; none without a heal — met, in
    memory (unit) and in a real wrapper's role file (the chaos case, all three CI runners).
  - (obs) the read-or-replay function's two lines, none for present or absent — met (unit, each record
    asserted whole).
  - (obs) `schema-check` and `g2-zero-panics.sh` exit 0; every new line validates — met: both entries exit 0;
    the chaos case validates its line itself; CI's G2 and G4 steps concluded `success` on all three runners.
    The local entries did not read the chaos home (above).
  - (tests) the three count names and their rules — met (unit `three_counts`).
  - (tests) every line this binary's writers append reads zero — met (the 13-kind table; `path2` and the two
    `path4` cases on product-written logs).
  - (tests) all 13 kinds round-trip the six-key line in `crates/viola-state/tests/` — met.
  - (tests) an unparseable and a `v` 99 snapshot each lead to a replay with the listed fields — met.
  - (security) the replay writes no file; every read is inside `MAX_FRAME` — met (`state_replay`: bytes,
    modification time and the dir's file names unchanged).
  - (security), (design), (a11y) nothing under `src/` changes — met: the diff holds no `src/` path and the
    preservation guard exits 0.
  - (tests) `run --unit` and `pre-push` green; the CI read `verdict: green`, run `ci#38046300968` — met.
  - No capability is claimed — holds: `matrix.py show --chunk` reads 0 claimed.
- **Gates**, the block's entries by `run`, from the second whole firing on the final tree (10:46:27Z) and the
  operator pass:
  - `cargo fmt --all --check` — green, exit 0
  - `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green, exit 0
  - `bash scripts/agent-run.sh run --unit` — green; 1540 of 1540
  - `bash scripts/agent-run.sh run --unit --filter 'test(/torn_tail|three_counts|replay_recovers|snapshot_cause|kind_table/)'`
    — green; 43 of 43
  - `bash scripts/agent-run.sh run --integration --filter 'binary(state_events) | binary(state_replay) | binary(chaos_torn_append) | binary(cli_send) | binary(cli_answer)'`
    — green; 43 of 43
  - `git diff --quiet 2f1efe3a8962 -- src …` (the preservation guard) — green, exit 0
  - `bash scripts/agent-run.sh cleanup --session p-shs-smoke` (pre-clean) — green
  - `bash scripts/agent-run.sh boot --session p-shs-smoke --instance builder` — green
  - `bash scripts/agent-run.sh status --session p-shs-smoke` — green, `state:"ready"`
  - `bash scripts/g2-zero-panics.sh` — green, `g2: clean`
  - `bash scripts/agent-run.sh schema-check` — green; 145 files, 1952 lines, no failure
  - `bash scripts/agent-run.sh cleanup --session p-shs-smoke` — green; `processes_gone:true`,
    `endpoint_gone:true`
  - `bash scripts/agent-run.sh pre-push` — green; coverage 1902 of 1902, playwright 1 of 1, no breaches; read
    green a third time on the uncommitted tree before the commit (10:49:23Z)
  - `… gate.py hygiene` (leg operator, fired as written) — `hygiene: clean`, exit 0, twice before the commit
  - `git diff --quiet && git diff --cached --quiet && git push origin HEAD` (leg operator, fired once through
    `--operator`) — green, exit 0; history moved `2f1efe3a→0af82832` on the two remote-tracking refs, a
    fast-forward
  - `… ci.py conclusion --sha HEAD --wait 1800` (leg operator, fired as written) — exit 0, `verdict: green ·
    checks 15/15`, run `ci#38046300968`, attempt 1
  - The two filter entries also read red on purpose, on the tree without the behaviour and under the three
    controls (`evidence/red-green.md`); no entry read red on the finished tree. Smoke: the session ran as the
    block's own entries.
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran. The verdicts rest on its final state: one commit, `0af8283`, no fix
  commit, and that sha's CI run, recorded in `evidence/operator-pass.md`. Implement's report, given in this
  session's conversation, is the basis for the deviations and the red and green readings.
- **Process hygiene:** every process the runs started is terminated. The smoke's supervisor, wrapper and fake
  agent ended at each cleanup (`processes_gone:true`, `killed:[]`); the census of 11:01:55Z found no process
  whose executable lies under this repository. Processes with a working directory in it were a shell, two
  rust-analyzer processes and the bridge with this session, none started by a run of this chunk.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 2f1efe3a (the parent of the oldest pre-CI commit 0af82832) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### Cargo.lock — added 1 line(s) in 1 range(s)
added: 1710
### crates/viola-core/src/lib.rs — added 141 line(s) in 3 range(s)
added: 66-82 · 93-117 · 414-512
  - 66-81 @67 «pub const ALL: [Self; 13] = [»
  - 93-96 @94 «pub fn from_name(name: &str) -> Option<Self> {»
  - 98-116 @100 «pub const fn data_keys(self) -> &'static [&'static str] {»
  - 414-457 @415 «const KIND_TABLE: [(EventKind, &str, &[&str]); 13] = [»
  - 459-464 @460 «fn kind_table_every_name_parses_back_to_its_variant() {»
  - 466-481 @467 «fn kind_table_a_text_that_is_no_kind_parses_to_nothing() {»
  - 483-504 @484 «fn kind_table_all_is_the_thirteen_kinds_in_declaration_order() {»
  - 506-511 @507 «fn kind_table_data_keys_are_the_contract_rows() {»
### crates/viola-state/Cargo.toml — added 2 line(s) in 1 range(s)
added: 36-37
### crates/viola-state/src/events.rs — added 461 line(s) in 48 range(s)
added: 11-13 · 69-70 · 73-74 · 89-90 · 92 · 112-113 · 129-131 · 134-136 · 139-145 · 200-218 · 228 · 239-245 · 257-266
       268-272 · 297-329 · 331-342 · 349 · 351 · 366-367 · 373-374 · 376-423 · 425-426 · 431-530 · 570 · 585 · 597 · 599
       610 · 612 · 614 · 657-664 · 697-699 · 702 · 708-710 · 716 · 718 · 720-721 · 727 · 735-736 · 740-741 · 744-753
       759-760 · 762-765 · 769-771 · 776-795 · 799-801 · 807-924 · 931-932
- 200-203 @201 «fn known_kind(line: &Map<String, Value>) -> Option<EventKind> {»
- 205-217 @207 «fn has_unknown_field(line: &Map<String, Value>, kind: EventKind) -> bool {»
  - 209-215 «let outside_data = line»
- 297-302 @299 «struct Tail {»
- 304-310 «impl Tail {»
  - 305-309 @307 «fn line_start(&self) -> u64 {»
- 312-325 «fn tail(instance_dir: &Path) -> Result<Tail, StateError> {»
  - 314-316 «let Some(last) = len.checked_sub(1) else {»
  - 321-324 «Ok(Tail {»
  - 331-334 «if !tail.torn {»
  - 336-342 «obs_event!(»
  - 376-381 @377 «const WHEEL_LINE: &str = concat!(»
  - 383-391 «fn session_end() -> EventLine {»
  - 393-398 @394 «const SESSION_END_LINE: &str = concat!(»
  - 403-413 @404 «fn healed_at_48() -> Value {»
  - 415-417 «fn log_bytes(dir: &Path) -> Vec<u8> {»
  - 431-440 @432 «fn events_torn_tail_heal_writes_one_state_recovered_line_and_the_next_append_none() {»
  - 442-460 @446 «fn events_torn_tail_a_log_with_no_fragment_takes_the_line_alone_and_no_record(»
  - 462-484 @463 «fn events_torn_tail_append_at_returns_and_hands_its_builder_the_fresh_line_start() {»
  - 486-503 @487 «fn events_torn_tail_try_append_heals_while_the_lock_is_free_and_writes_nothing_while_held() {»
  - 505-512 @506 «fn events_torn_tail_reader_started_at_the_old_end_returns_the_healed_line() {»
  - 514-529 @515 «fn events_torn_tail_counts_one_torn_line_before_the_heal_and_one_after() {»
  - 660-664 «const NONE_SKIPPED: Skipped = Skipped {»
  - 826-851 @830 «fn events_three_counts_an_unlisted_key_counts_once_for_its_line_and_the_line_is_returned() {»
### crates/viola-state/src/lib.rs — added 63 line(s) in 3 range(s)
added: 2-4 · 11 · 32-90
- 32-89 @35 «mod test_capture {»
  - 47-64 «impl Visit for Fields {»
  - 69-79 «impl<S: Subscriber> Layer<S> for Captured {»
  - 81-88 @82 «pub(crate) fn capture<R>(f: impl FnOnce() -> R) -> (R, Vec<Value>) {»
### crates/viola-state/src/replay.rs — new file · 409 line(s)
- 15-27 @18 «pub struct Replayed {»
- 29-34 @31 «pub enum ReplayCause {»
- 36-46 @38 «pub enum Recovered {»
  - 42-45 «Replayed {»
- 48-73 @51 «pub fn replay(instance_dir: &Path) -> Result<Replayed, StateError> {»
  - 54-70 «for line in lines.by_ref() {»
- 75-79 «fn holder(value: &Value) -> Option<Wheel> {»
  - 76-78 «[Wheel::Driver, Wheel::Human]»
- 81-104 @84 «pub fn read_snapshot_or_replay(instance_dir: &Path) -> Result<Recovered, StateError> {»
  - 85-90 «let cause = match read_snapshot_classified(instance_dir) {»
  - 92-95 «let (detail, v_seen) = match cause {»
  - 96-102 «obs_event!(»
- 106-409 @107 «mod tests {»
  - 117-121 «const NONE_SKIPPED: Skipped = Skipped {»
  - 123-133 «fn log(dir: &Path, lines: &[(EventKind, Value)]) {»
  - 135-149 «fn snapshot() -> InstanceSnapshot {»
  - 151-183 @152 «fn full_log(dir: &Path) {»
  - 185-195 «fn full_state() -> Replayed {»
  - 197-202 @198 «fn replay_recovers_each_field_from_its_last_line() {»
  - 204-229 @205 «fn replay_recovers_the_driver_wheel_and_an_open_gate_from_a_start() {»
  - 231-256 @234 «fn replay_recovers_a_field_as_absent_when_its_last_line_holds_no_value() {»
  - 258-276 @259 «fn replay_recovers_the_earlier_session_id_when_a_later_start_carries_null() {»
  - 278-293 @279 «fn replay_recovers_nothing_from_an_empty_or_an_absent_log() {»
  - 295-327 @296 «fn replay_recovers_the_three_counts_of_its_pass() {»
  - 329-352 @330 «fn replay_recovers_an_unreadable_snapshot_with_one_snapshot_replayed_line() {»
  - 354-382 @355 «fn replay_recovers_a_newer_snapshot_with_one_unsupported_v_line() {»
  - 384-395 @385 «fn replay_recovers_nothing_and_writes_no_line_for_a_present_or_an_absent_snapshot() {»
  - 397-408 @400 «fn replay_recovers_an_error_and_no_line_from_a_log_that_cannot_be_read() {»
### crates/viola-state/src/snapshot.rs — added 140 line(s) in 5 range(s)
added: 6 · 100-144 · 147-150 · 156 · 317-405
- 100-113 @102 «pub enum SnapshotRead {»
  - 109-112 @110 «Unsupported {»
- 115-119 @117 «struct EnvelopeV {»
- 121-143 @123 «pub fn read_snapshot_classified(instance_dir: &Path) -> SnapshotRead {»
  - 124-128 «let file = match File::open(instance_dir.join(SNAPSHOT)) {»
  - 130-132 «if file.take(MAX_FRAME).read_to_end(&mut bytes).is_err() {»
  - 133-135 «let Ok(EnvelopeV { v }) = serde_json::from_slice(&bytes) else {»
  - 136-138 «if v > u64::from(SNAPSHOT_V) {»
  - 139-142 «match serde_json::from_slice::<Envelope<InstanceSnapshot>>(&bytes) {»
  - 147-150 «match read_snapshot_classified(instance_dir) {»
  - 317-319 «fn seed(dir: &Path, bytes: &str) {»
  - 321-329 @322 «fn snapshot_cause_a_written_snapshot_is_present() {»
  - 331-339 @332 «fn snapshot_cause_no_file_is_absent() {»
  - 350-358 @351 «fn snapshot_cause_bytes_that_are_no_v1_envelope_are_unreadable(#[case] bytes: &str) {»
  - 360-369 @361 «fn snapshot_cause_a_v_of_zero_is_unreadable_whatever_its_data() {»
  - 371-385 @374 «fn snapshot_cause_a_file_that_cannot_be_read_is_unreadable() {»
  - 387-404 @388 «fn snapshot_cause_a_newer_v_is_unsupported_with_the_v_it_saw() {»
### crates/viola-state/tests/state_events.rs — new file · 246 line(s)
- 15-19 «const NONE_SKIPPED: Skipped = Skipped {»
- 21-25 «const ONE_TORN: Skipped = Skipped {»
- 27-31 «fn append(dir: &Path, kind: EventKind, data: Value) {»
- 33-38 @34 «fn read(dir: &Path, after: u64) -> (Vec<LoggedLine>, Skipped) {»
- 40-45 «fn kinds(lines: &[LoggedLine]) -> Vec<&str> {»
  - 41-44 «lines»
- 47-49 «fn log(dir: &Path) -> Vec<u8> {»
- 51-53 «fn offset(n: usize) -> u64 {»
- 55-116 @56 «fn every_kind() -> [(EventKind, &'static str, Value); 13] {»
  - 57-115 «[»
- 118-158 @119 «fn state_events_every_kind_round_trips_the_six_key_line() {»
  - 121-123 «for (kind, _, data) in every_kind() {»
  - 127-155 «for (line, (_, name, data)) in lines.iter().zip(every_kind()) {»
- 160-197 @164 «fn state_events_lines_outside_the_contract_are_counted_and_known_kinds_still_returned() {»
  - 166-170 «append(»
  - 172-177 «let raw = [»
  - 180-185 «OpenOptions::new()»
  - 189-196 «assert_eq!(»
- 199-246 @200 «fn state_events_a_log_cut_short_takes_the_next_append_on_a_fresh_line() {»
  - 202-206 «append(»
  - 208-212 «append(»
  - 219-224 «OpenOptions::new()»
### crates/viola-state/tests/state_replay.rs — new file · 173 line(s)
- 14-16 «use viola_state::snapshot::{»
- 18-32 «fn snapshot() -> InstanceSnapshot {»
  - 19-31 «InstanceSnapshot {»
- 34-64 @36 «fn written() -> tempfile::TempDir {»
  - 40-58 «let lines: [(EventKind, Value); 5] = [»
  - 59-62 «for (kind, data) in lines {»
- 66-81 @67 «fn replayed() -> Replayed {»
  - 68-80 «Replayed {»
- 83-101 @84 «fn on_disk(dir: &Path) -> (Vec<u8>, SystemTime, Vec<String>) {»
  - 86-88 «let modified = fs::metadata(&path)»
  - 89-98 «let mut names: Vec<String> = fs::read_dir(dir)»
- 103-114 @104 «fn state_replay_the_written_snapshot_reads_present_and_is_returned() {»
  - 106-109 «assert_eq!(»
  - 110-113 «assert_eq!(»
- 116-150 @117 «fn state_replay_a_snapshot_cut_short_is_replayed_from_the_log_and_left_untouched() {»
  - 121-126 «OpenOptions::new()»
  - 127-130 «assert_eq!(»
  - 133-139 «assert_eq!(»
  - 141-149 «assert_eq!(»
- 152-173 @153 «fn state_replay_a_snapshot_of_v_99_is_replayed_with_the_v_it_saw_and_left_untouched() {»
  - 159-162 «assert_eq!(»
  - 165-171 «assert_eq!(»
### tests/chaos_torn_append.rs — new file · 123 line(s)
- 19-23 «const NONE_SKIPPED: Skipped = Skipped {»
- 25-29 «const ONE_TORN: Skipped = Skipped {»
- 31-36 @32 «fn read(instance_dir: &Path, after: u64) -> (Vec<LoggedLine>, Skipped) {»
- 38-43 «fn recovered(home: &Path) -> Vec<Value> {»
  - 39-42 «support::ndjson::read_lines(&home.join("diagnostics").join("run-builder.ndjson"))»
- 45-123 @46 «fn chaos_torn_tail_left_by_a_stopped_wrapper_is_healed_by_the_next_start() {»
  - 47-52 «let wrapper = Wrapper::boot(»
  - 61-66 «OpenOptions::new()»
  - 78-82 «while recovered(&home).is_empty() {»
  - 95-98 «assert_eq!(»
  - 112-115 «assert!(»
  - 119-122 «assert!(»
### tests/cli_answer.rs — added 21 line(s) in 4 range(s)
added: 30 · 150-165 · 358-359 · 564-565
- 150-164 @152 «fn assert_reads_clean(instance_dir: &Path) {»
  - 155-162 «assert_eq!(»
### tests/cli_send.rs — added 19 line(s) in 3 range(s)
added: 23 · 169-184 · 269-270
- 169-183 @171 «fn assert_reads_clean(instance_dir: &Path) {»
  - 174-181 «assert_eq!(»
