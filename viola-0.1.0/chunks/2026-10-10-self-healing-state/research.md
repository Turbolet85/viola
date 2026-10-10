# Codebase Research — 2026-10-10-self-healing-state

## Scope
- **Depth:** deep on `viola-state`, targeted elsewhere · **Reads:** 21 · **Globs/Greps:** 19
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole, its 14 Session Additions
  included (the dated bullets under that heading); applied: never pipe `boot` (2026-09-25), a whole root test file is selected with `binary(<stem>)` and
  an inline unit test by `test(/…/)` (2026-09-27, extended 2026-10-04), a stalled-start red is first a reading of
  the backing (2026-09-29, extended 2026-10-07). No live `claude` leg is in this chunk.
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg.
- **External inputs:** `inputs#I1` — the operator's take-up direction (no mutation run, no dispatch, no witness
  run, no live start, the gate block, the CI read, the split at the review card).

## Files inspected
- `crates/viola-state/src/events.rs` (1-262 whole, the test names of 263-624) — the three append functions share
  `write_line` (257-261); `append_event_at` reads the length under the lock and hands it to its builder (81-91);
  `read_within` skips the line `after` falls inside (140-168); `next_line` breaks on an unterminated tail without
  counting it (208-213), counts an over-long line (214-219) and a non-object line (231).
- `crates/viola-state/src/snapshot.rs` (1-170) — `read_snapshot` is `Option`, `None` for four different causes
  (100-110); `InstanceSnapshot` holds `wheel`, `budget_paused`, `links` and no `agent_session_id` or
  `budget_override_until` (38-58); `SNAPSHOT_V` is 1.
- `crates/viola-state/src/lib.rs` (whole) — eight modules, one error enum with fixed messages.
- `crates/viola-state/src/liveness.rs` (1-75) — `process_start_time`, `same_process`: the pid plus start time read.
- `crates/viola-state/Cargo.toml` (whole) — `tracing` and `tempfile` are product dependencies; the only
  dev-dependency is `rstest`.
- `crates/viola-core/src/lib.rs` (44-100, 336-362) — `EventKind`: 13 variants, `WAIT_WAKE`, `as_str`; no list of
  all variants, no parse from a name, no `data` key list.
- `crates/viola-core/src/obs.rs` (8-60) — `ObsEvent::StateRecovered` is one of 19 values.
- `schemas/diag-line.v1.json` (33, 87, 94-96, 125-128) — `state-recovered` admits `detail` (three closed values),
  `file` (string), `offset` (a count), `v_seen` (integer).
- `src/run/wait.rs` (60-210) — `rebuild` and `scan`, the two product callers of `read_from`; neither reads
  `skipped()`, and `scan` wakes only on `EventKind::WAIT_WAKE`.
- `src/run/snapshot.rs` (1-50) — `Snapshots`, the wrapper's one snapshot writer.
- `src/cmd/run.rs` (177-234, 286-309, 404-443) — the start order; `collision_check` reads "free" for an absent or
  unreadable snapshot; `start_state` writes a fresh snapshot and appends `wheel{driver, start}` and
  `budget-gate{paused:false}` on every start.
- `src/cmd/client.rs` (80-128) — `live_endpoint`: no snapshot means no endpoint, so exit 21.
- `src/cmd/verify.rs` (150-310) and `src/cmd/verify/typed.rs` (probe-dir lines by search) — the probe dirs and
  their drop guards.
- `crates/viola-channel/src/lib.rs` and `server.rs` (by search) — the newer-peer refusal and its tests.
- `tests/support/ndjson.rs` (whole), `tests/support/events.rs` (1-60), `tests/support/home.rs` (486-520) — the
  test-side log reader fails a test on a complete line that is not JSON; `events::boot` waits through that reader;
  `Wrapper::stop_keep` hands the home back for a second start.
- `.config/nextest.toml` (section lines) and `crates/viola-e2e/src/harness/run/nextest.rs` (1-40) — the
  integration layer is `kind(test)`: every test binary, crate-level ones included.
- `.andromeda/architecture.md` (50, 214-221, 237-247, 290-306, 348-351) and `.andromeda/test-plan.md` (544-555,
  937-948, 1010-1019) — the contract passages the scope closure rests on.

## Graph impact (from the code-graph query; rust plane, trail `tree-query-2026-10-10-self-healing-state.json`)
- **read_from** — 2 product callers: `rebuild` @ `src/run/wait.rs:115`, `scan` @ `src/run/wait.rs:197`. Its
  signature and `LoggedLine` stay as they are, so neither caller changes. Both stop receiving lines of a kind
  this binary does not know; neither used them (`scan` filters to the wake kinds, `rebuild` reads `turn-ended`
  and dialog ids).
- **skipped** (the method) — 1 caller, the test helper `read` @ `crates/viola-state/src/events.rs:473`. No
  product code reads the count, so its field names are free to take the contract's.
- **write_line** — 3 callers, all in `events.rs`: `append_event` @ `:68`, `append_event_at` @ `:89`,
  `try_append_event` @ `:109`. One heal there covers every writer.
- **append_event / append_event_at / try_append_event** — product callers `start_state` @ `src/cmd/run.rs:440`,
  `append_hook_event` @ `src/run/send.rs:246`, `record` @ `src/run/wheel.rs:279`, `register` @
  `src/run/dialog.rs:238`, `record` @ `src/run/send.rs:390`, `issue` @ `src/run/send.rs:399`,
  `append_session_end` @ `src/cmd/hook.rs:494`. No signature changes, so none is edited.
- **read_snapshot** — 4 product callers: `live_endpoint` @ `src/cmd/client.rs:96`, `ask` @ `src/cmd/hook.rs:254`,
  `deliver` @ `src/cmd/hook.rs:469`, `collision_check` @ `src/cmd/run.rs:294`. It stays as it is; the classified
  read is added beside it and none of the four moves to it.
- **end_offset** — product callers in `src/run/send.rs` and `src/run/wait.rs`, and root tests
  (`tests/cli_send.rs`, `tests/cli_wait_last.rs`). Unchanged: it is the file's length, and a reader started there
  on a torn log already skips to the next line start (`read_within`, 153-164).

## Patterns detected
- **One shared write path** (`crates/viola-state/src/events.rs:257`): every append is `line_bytes` then one
  `write_all` under `events.ndjson.lock`. The heal joins that write.
- **A lib crate's own log capture** (`crates/viola-channel/src/lib.rs:103-200`): a `#[cfg(test)] mod test_capture`
  with `tracing-subscriber` as a dev-dependency; unit tests read the lines an `obs_event!` call made. `viola-state`
  has none yet.
- **A crate-level suite beside inline tests** (`crates/viola-channel/tests/channel_frames.rs`): the one
  crate-level `tests/` dir outside `viola-e2e` today.
- **A second start on one home** (`tests/cli_wait_last.rs:283`, `last_survives_a_wrapper_restart`): `stop_keep`,
  then a boot on the returned home.
- **The writers' `data` keys, read against architecture's per-kind list** (`.andromeda/architecture.md:290-304`):
  `session-start` `{cause, agent_session_id}`, `prompt-submitted` `{text, origin}`, `turn-ended`
  `{last_assistant_message}`, `activity` `{}` or `{tool}` (`crates/viola-agent-claude/src/hook.rs:156-167`);
  `wheel` `{holder, cause}` (`src/run/wheel.rs:276`, `src/cmd/run.rs:435`); `budget-gate` `{paused}`
  (`src/cmd/run.rs:437`); `send-issued` `{cursor, from?}` (`src/run/send.rs:400`); `send-confirmed` `{cursor}` or
  `{cursor, confirmed}` (`:433`, `:441`); `send-refused` `{refusal, detail, cursor?}` (`:480`). Each is inside the
  list. The three dialog kinds' bodies come from the hook's `hook.dialog` and were not read key by key: the plan's
  product-log test is what pins them.

## Conventions to follow
- **Fixed error messages**: a new failure joins `StateError` (`crates/viola-state/src/lib.rs:19-25`), no path
  and no payload in a `Display`.
- **Spans**: `#[tracing::instrument(skip_all, name = "state.<operation>")]`, as `state.snapshot_write`
  (`crates/viola-state/src/snapshot.rs:75`); never `err` or `ret` on the snapshot parse.
- **Lines**: only through `viola_core::obs_event!`, typed fields, no `corr` passed for `state-recovered`.
- **Test names**: `<subject>_<condition>_<expected>`, table cases as rstest `#[case]` rows; expected values are
  literals in the test.
- **A torn log and the test-side reader**: a root test that reads a healed log goes through the product reader or
  the raw bytes, never `tests/support/events.rs`, whose reader fails on the fragment line; the second boot on such
  a home is `Wrapper::boot`, not `events::boot`.

## Sweeps
- `read_from(` over `src` and `crates` outside `events.rs`: 2 hits · 0 changed · 2 no-change (the signature
  stays).
- `read_snapshot(` over `src` and `crates`, product code: 4 hits · 0 changed · 4 no-change (no reader takes the
  fallback in this chunk).
- `Skipped` and `.skipped()` over `src` and `crates` outside `events.rs`: 0 hits.
- `StateRecovered|state-recovered` over `*.rs`, `*.json`, `*.sh`, `*.toml` outside `target`, `.andromeda` and the
  version dir: 5 files (`crates/viola-core/src/obs.rs`, the two schemas, `src/obs.rs`,
  `tests/contract_diag_schema.rs`) · 0 changed · 5 no-change (the event and its fields already stand).
- `signal|ctrlc|SIGTERM|SIGINT` over `src/cmd/verify.rs`, `src/cmd/verify/typed.rs`, `src/main.rs`: 0 files.
- `ls -d .viola-verify-*` at the repository root: 0 dirs; under `crates/viola-e2e/`: 1.

## Scope premise closure
- **The heal's form** (scope 1): corrected. Architecture states that readers heal and names no form (the arch
  extract, Constraints, first item). Test-plan §6 Chaos suite gives the observable, "the next append starts on a
  fresh line". The shared write path is where a torn tail does harm today, so the heal is one LF at the head of the
  next append, in that append's single write.
- **Logging the reader's counts** (scope 1): closed as not logged. Obs-plan names `list --json` and the
  `/api/sessions` line as the only carriers.
- **Which reader replays** (scope 2): corrected. None in this chunk; the four callers and their reasons are in the
  scope. The replay and the classified read land as library code with their record.
- **`links` in the replay** (scope 2): corrected. No `link` or `unlink` kind exists at HEAD (`EventKind`, 13
  variants; architecture lists fifteen).
- **The surface for the counts** (scope 3): corrected. No surface exists; the reader returns them.
- **The newer-peer refusal** (scope 4): corrected. Landed and tested at unit level and over a real endpoint.
- **"Every event kind"** (scope 5): verified as the 13 variants; the CARRY's "lands the reader" corrected.
- **The verify mechanism claim** (scope 6): verified at HEAD at the three named files, with one addition (three
  classes of leftover dir).
- **The remover's trust rule** (scope 6): corrected. No rule exists; not built here.
- **The chaos cases** (scope Boundaries): closed. The fresh-line half is built here; the surface readings and E5
  are owed to the entries that land the surfaces.
- No extract leaned on a falsified premise: each extract left these points as research's questions.

## Mechanisms the plan rests on
- **The offset equality**: for a log of length L whose last byte is not LF, the next appended line starts at
  L + 1; for an absent, empty or LF-ended log, at L. The frame that produces it is `append_event_at`
  (`events.rs:81-91`), which today hands `current_len` to the builder; with the heal it hands the line's own start.
- **A reader started at a torn log's end**: `read_within(after = L)` reads byte L - 1, finds it is not LF and
  skips through the next LF (`events.rs:153-164`), which after the heal is the heal's own LF at L. So a `wait`
  that took the log's end before the heal returns the healed append's line. Verified by reading the code; the plan
  pins it with a test.
- **A torn tail counts once before and once after the heal**: before, as the unterminated last line of the read;
  after, as a terminated line that is not one JSON object.
- **Zero counts on product-written logs**: the per-kind key lists in `viola-core` are architecture's lists, and the
  writers read at P3 stay inside them.

## New files to create
- `crates/viola-state/src/replay.rs` — the log replay and the read-or-replay function with its record
- `crates/viola-state/tests/state_events.rs` — every event kind round-trips; the three counts; the heal on a real dir
- `crates/viola-state/tests/state_replay.rs` — the classified snapshot read and the replay on a real dir
- `tests/chaos_torn_append.rs` — the root chaos case: a stopped wrapper, a shortened log, a second start

## Files to modify
- `crates/viola-core/src/lib.rs` — the list of all kinds, the parse from a kind's name, each kind's `data` keys
- `crates/viola-state/src/events.rs` — the three counts, the unknown-kind and unknown-field reading, the heal
- `crates/viola-state/src/snapshot.rs` — the read that tells the four cases apart
- `crates/viola-state/src/lib.rs` — the new module and the crate's test log capture
- `crates/viola-state/Cargo.toml` — the `tracing-subscriber` dev-dependency
- `Cargo.lock` — the dev-dependency edge
- `tests/cli_send.rs` — the product-written log of the send path reads zero on all three counts
- `tests/cli_answer.rs` — the product-written log of the dialog path reads zero on all three counts

## Open questions
- none that block a plan step. Three points go to the review card, each named in the scope: the cut itself; the
  gate block, where the crate-level suite and the chaos file are integration-layer binaries that `run --unit` does
  not select; and test-plan E5's `wheel:"human"` after a second start, which disagrees with the landed start.
