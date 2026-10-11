# Report — 2026-10-10-session-links

**Chunk:** Session links: link and unlink event kinds, the implicit link on a cross-instance send or answer,
viola link and unlink with a result that carries what they print, the replay's link set with since
**Date:** 2026-10-11
**Commits:** `3cae2ce` `chore(2026-10-10-session-links): operator pre-CI commit, for the run this chunk's verdict
reads` (the one commit since the last wrap's `f02d548`; `git log --format='%h %s' f02d548..HEAD`). Every diff
reading below is `f02d548` (the parent of that pre-CI commit) against the work tree.

**What landed and what did not.** Part A of the plan (steps 1 to 10) is built and is the whole of this report's
Changes. Part B (steps 12 to 15: the two channel methods, the three result lines, the two CLI verbs, their root
cases) is **held by plan step 11** and nothing of it exists in the tree: the founder's own word on the boundary
widening was not recorded, and the operator directed Part A alone (`inputs#I4`). No `src/cmd/link.rs`, no
`src/cmd/unlink.rs`, no `tests/channel_links.rs`, no dispatch arm for `link` or `unlink`, no edit of
`src/human.rs`, `src/cmd/mod.rs` or `src/main.rs` (`ls` of the three new paths: absent; `git diff --stat f02d548`
names none of the three modify paths).

## Changes (structured — detectors read this)

- **Files:** 13 (`gate.py scope`: changed 13, listed 13). Coordinates of added text are copied from the last
  section of this report.
  - new: `src/run/links.rs` (370 lines), `tests/cli_links.rs` (281 lines);
  - modified: `crates/viola-core/src/lib.rs`, `crates/viola-state/src/replay.rs`,
    `crates/viola-state/src/events.rs`, `crates/viola-state/tests/state_events.rs`,
    `crates/viola-state/tests/state_replay.rs`, `src/run/mod.rs`, `src/cmd/run.rs`, `src/run/send.rs`,
    `src/run/dialog.rs`, `tests/cli_budget.rs`, `.config/nextest.toml`;
  - `git diff --stat f02d548 -- src crates tests .config`: 13 files, 1395 insertions, 23 deletions.
- **Symbols / APIs:**
  - **Two event kinds.** `viola_core::EventKind::Link` and `::Unlink` (`crates/viola-core/src/lib.rs:63-64`),
    last in declaration order; `ALL` is `[Self; 15]` (`:69`, `:83-84`); `data_keys` gives both
    `["driver", "driven"]` (`:119`); `as_str` gives `link` and `unlink` (`:138-139`). Neither is in `WAIT_WAKE`
    (unchanged, five kinds). `from_name` is unchanged and now returns the two. No other crate matches on the
    enum exhaustively.
  - **The link rule.** `viola_state::replay::apply_link_line(links: &mut Vec<Value>, line: &Value) -> bool`,
    public (`crates/viola-state/src/replay.rs:76-100`, head at `:80`). A `link` line whose `data.driver` and
    `data.driven` are strings adds `{driver, driven, since}` with `since` = the line's `ts`, unless the set
    holds the pair; an `unlink` line removes the pair; a line of another kind, or one whose `driver` or `driven`
    is no string, changes nothing. It returns whether the set changed, carries no span and writes nothing. A
    `link` line that holds no string `ts` gets `since: "unknown"` (the form `session_chain` in the same file
    takes for a missing `ts`; no writer of this build makes such a line).
  - **The replay's link half.** `replay` calls the rule for every line of its one pass
    (`crates/viola-state/src/replay.rs:56`), so `Replayed.links` holds the set. Its span (`state.replay`), its
    other fields, its skip counts and the rule that it writes no file are unchanged. It still has one product
    caller, `read_snapshot_or_replay`, so a `viola revive` over a replayed snapshot now reads the logged links.
  - **The wrapper's link set.** `LinkSet` (`pub(crate)`, `src/run/links.rs:19-24`, its impl `:26-105`), one per
    wrapper process, holding the instance's name, its directory, the shared `Snapshots` and the set under one
    lock. `restore` (`:40-48`): one pass over `viola_state::events::read_from(instance_dir, 0)` through the
    rule; it appends nothing and logs nothing. `current` (`:50-53`). `record(driver)` (`:55-66`): a pair the
    set holds is answered `(since, true)` with nothing written; a new one appends one `link` line
    `{driver, driven}` with `source: wrapper` through `append_event`, applies the rule with that line's own
    `ts`, writes the set into the snapshot through `Snapshots::update`, and answers `(since, false)`.
    `remove(driver)` (`:68-79`): an absent pair writes nothing; else one `unlink` line the same way. A failed
    append leaves the set as it was; a line that landed is in the set even when the snapshot write then fails
    (`change`, `:95-104`). The module emits no `obs_event!` line and declares no span.
  - **`remove` has no product caller.** Its one caller is step 12's `unlink` method, which is held. It carries
    `#[cfg_attr(not(test), allow(dead_code))]` and is unit-tested. So no product path of this build writes an
    `unlink` line.
  - **The implicit link on a send.** `send` (`src/run/send.rs:395-399`): after the second wheel and
    running-turn reads and directly before `ctx.issue`, when `from` is a name other than the wrapper's own and
    the slot was handed a set, it calls `record`; an error answers `-32603` with nothing issued and nothing
    typed. A send refused at any earlier rung records nothing; a send with no `from`, or with `from` equal to
    the wrapper's name, records nothing. The `link` line stands directly before that send's `send-issued`,
    whose `cursor` is still the offset its own line starts at.
  - **The implicit link on an answer.** `DialogSlot::answer` (`src/run/dialog.rs:366-370`): once every refusal
    has passed and the response's kind matched the pending dialog, and before the `dialog-answered` line and
    the answer being taken, the same call under the same two conditions; an error answers `-32603` and the
    answer is not taken (the dialog stays held). A refused answer records nothing.
  - **Setters.** `SendSlot::links_with` (`src/run/send.rs:131-134`) and `DialogSlot::links_with`
    (`src/run/dialog.rs:174-177`), each a `OnceLock<Arc<LinkSet>>`, first call wins, the form of
    `SendSlot::budget_with`. A slot handed no set records nothing (every earlier unit case builds such a slot).
  - **The start.** `start` builds the set and restores it beside the wait feed's rebuild, before `server.serve`
    (`src/cmd/run.rs:274-280`), hands it to the send slot there and to the dialog slot (`:291`), and passes
    `links.current()` into `Recorded` (`:318`, the field at `:530`); `start_state` writes it as the first
    snapshot's `links` in place of an empty list (`:559`). The start appends no line for links; its start
    records are unchanged. It now reads the log twice before the endpoint serves (`WaitFeed::rebuild`, then
    `LinkSet::restore`). `viola revive` starts through the same function.
  - **No channel method, no CLI verb.** `Methods::dispatch_call` is unchanged: `link` and `unlink` still
    answer `-32601` (the unit case `methods_answer_method_not_found_for_every_other_method` is unchanged and
    green). `viola-channel`'s `METHODS` label list already held both names and is unchanged. No clap
    subcommand was added.
  - **Callers, counted** (`grep -n '\.update(' src/run src/cmd/run.rs`, test modules left out): product callers
    of `Snapshots::update` 4 → 5 (`src/cmd/run.rs:604`, `src/run/budget.rs:333`, `src/run/dialog.rs:313`,
    `src/run/wheel.rs:293`, and the new `src/run/links.rs:103`). The readers of `snapshot.json` are unchanged (no
    `read_snapshot(` site added).
  - No port, socket, pipe, env var, config key or file under the viola home was added.
- **Crates / modules:** added the root-bin module `run::links` (`src/run/links.rs`, declared at
  `src/run/mod.rs:8`). Changed: `viola-core` (the kind table), `viola-state` (`replay`; test code in `events`),
  root `run::send`, `run::dialog`, `cmd::run`. No crate edge added (`Cargo.toml` and `Cargo.lock` are held
  unmoved by the preservation guard).
- **Dependencies:** none.
- **Schema / config:**
  - event `data` per kind: `link` and `unlink` carry `{driver, driven}`; the reader no longer counts such a
    line under `unknown_kinds`, and returns it;
  - snapshot: `links` now holds `{driver, driven, since}` items; its type (`Vec<Value>`) is unchanged, and it
    is still always serialised;
  - `.config/nextest.toml:32`: the CI profile's 20 s kill filter line gains `binary(cli_links)`;
  - no file under `schemas/` moved (the guard), no config key, no redaction shape.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - `EventKind::ALL` 13 → 15 (the array's length, `crates/viola-core/src/lib.rs:69`). Stated as 13 in
    test-plan (`the 13 kinds`: 1 hit, `test-plan.md:644`; pattern `[Tt]hirteen|13 (normalised |event )?kinds|\b13 rows`
    over the seven masters and every file under `.andromeda/registries/`: 1 hit, that one);
  - the binaries on the 20 s kill filter line 14 → 15 (the `binary(` names on `.config/nextest.toml:32`,
    counted by `grep -o`, 14 at `f02d548`). Stated as "the fourteen verify-driven binaries" with the name list
    in the test-plan key file `registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10`
    (pattern `[Ff]ourteen`: 5 hits over masters and key files, this one changed; the other four are other
    subjects: the fourteen older stamp ids at `architecture.md:91`, `security-plan.md:617`, `test-plan.md:562`,
    and fourteen labelled table cases at `test-plan.md:566`);
  - product callers of `Snapshots::update` 4 → 5 (above); pattern `Snapshots::update` over masters and key
    files: 0 hits, so no master states the count;
  - the root waits: 28 sites in 20 files, **unchanged** (`grep -rn -F 'Instant::now() + WITHIN' tests`, 28 at
    `f02d548` too): no new case adds one;
  - the unit suite 1865 → 1899 cases on the Linux dev host (`run --unit`, this run's block against the last
    chunk's recorded block), 34 of them the new prefixed ones;
  - CI's test counts per runner: 2280, 2275, 2279 → 2322, 2317, 2321 (`windows-2025`, `macos-latest`,
    `ubuntu-latest`), 42 more on each.
- **Dev-tool versions:** none. No host tool was installed, upgraded or read changed; `pre-push`'s `tools` stage
  passed against CI's pins twice.
- **Harness / gate surface:** `.config/nextest.toml:32` (above). No `scripts/` file, no workflow, no harness
  subcommand, no status or verdict shape changed (the guard holds `scripts`, `.github`, `crates/viola-e2e`).
- **Cross-project / external claims:**
  - CI on `Turbolet85/viola` (the push remote `origin`): run `ci#38098238731`, event `push`, measured sha
    `3cae2ceca8f0`, `run_attempt` 1, conclusion `success`, 15 of 15 checks (`ci.py conclusion`, then
    `gh api …/actions/runs/38098238731`). It is the only run on the sha. The verdict was taken on the pre-CI
    commit's tree; this wrap's own commit adds no source file to it.
  - Inputs, from `inputs.py verify` at this wrap (5 entries; unchanged 0, drifted 0, vanished 0, broken 0,
    n/a 5; unparsed 0):
    - `I1` · a message, the operator in the phase invocation · copy · n/a (a message has no live source);
    - `I2` · a message, the operator's fork answers at P4 · copy · n/a;
    - `I3` · a message, the operator's approval word at P5 · copy · n/a;
    - `I4` · a message, the arguments of the implement invocation (Part A alone, the operator pass on the
      operator's word, no live start, no mutation run) · copy · n/a; printed `UNCITED` by `verify` before this
      report existed and cited here as `inputs#I4`;
    - `I5` · a message, the arguments of this wrap's invocation (Phase 1 only in this session, a resume point,
      the reconcile in a fresh context) · copy · n/a; snapped at this wrap, cited here as `inputs#I5`;
    - `I6` · a message, the operator's directions given with this wrap's resume · copy · n/a; snapped at the
      resume, after the five above were verified, and cited here as `inputs#I6` (Decisions & corrections).
- **Reverted / negative API facts:** none in the committed tree. Two things were written and taken out before
  the block ran, both by design of step 10 (`evidence/red-green.md`): two stub bodies (the rule returning
  `false`, `LinkSet::change` returning `Ok(())`) that stood only for the red reading; and three scratch edits
  for the forced-on readings (the link record at a call's arrival, the own-name guard removed, a budget
  refusal in `DialogSlot::answer`), each reverted and read back by SHA-256.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - **The plan's note on its schema-check entry** (`plan.md:349`: "G4 over the same homes: every role line
    the new cases wrote validates against the unchanged schema"). Measured false on the Linux dev host: a root
    test's home is removed when the test ends (no keep flag is set there), so the entry read the booted smoke
    home and the kept live homes, 160 files and 2220 lines, the figures the previous chunk's entry read. The
    property itself holds by two other reads: each new root case validates its own role lines against
    `schemas/diag-line.v1.json` in the test (`tests/cli_links.rs:78-97`), and CI keeps its test homes, where
    the G4 step read 346, 356 and 356 files with no failure. No master sentence was read for this claim; the
    one place it is stated that this chunk read is the plan.
- **Expected amendments (from plan):** the plan's list has seven entries. Each search is over the seven masters
  and every file under `.andromeda/registries/`, by a scratch script that counts a pattern's matches per file.
  - *architecture §Standard Contracts: Event `data` per kind and the wrapper-appended kinds gain `link` /
    `unlink`; the Snapshot envelope sentence is retired; the start restores the set.* **Carried**: Symbols /
    APIs (the two kinds, the wrapper's set, the start) and Schema / config. Sites: `replays? empty` 3 hits
    (`architecture.md:247`, `test-plan.md:379`, `test-plan.md:552`); `wrapper appends` 2 hits, both
    architecture (`:71`, the [Links] decision; `:308`, the sentence listing what the wrapper appends);
    `` `links` (is|stays|always) `` 1 hit (`architecture.md:262`, the snapshot's `links` field, true as it
    stands). Note for the amendment: the wrapper appends `link` on an implicit link only; no product path
    appends `unlink` yet.
  - *architecture §Established Decisions: [Links] (a refused call and an own-name `from` record nothing, the
    record precedes `send-issued`); [Budget Governor] (the four verbs' case exists).* **Carried**: Symbols /
    APIs (the two implicit links) and Outcome (the folded case). Sites: `\[Links\]` 1 hit (architecture);
    `no case drives one of them` 2 hits (`architecture.md:72`, `a11y-plan.md:823`).
  - *security-plan §Input Validation, row "Own state files on read": the wrapper's start reads the log for
    its link set through `read_from`.* **Carried**: Symbols / APIs (the start; `LinkSet::restore`). Site:
    `Own state files on read` 1 hit (`security-plan.md:241`).
  - *test-plan §4, §5, §6: the "`links` replays empty" sentences, the kind count with its rule, Scenario E5's
    `link` step as landed, Path 6's owed sentence, and the §3 key file's kill-line count.* **Carried**: Counts
    / qualifiers moved, Symbols / APIs and Outcome. Sites: `replays? empty` in test-plan 2 hits (`:379`,
    `:552`); `the 13 kinds` 1 hit (`:644`); `Session links` in test-plan 3 hits (`:379`, `:552`, and `:951`,
    "The scenario still needs `link`", which is E5's step); `owed to the route` 5 hits in test-plan, the one
    that is Path 6's at `:863` (the other four name other entries); the kill-line count in the key file above
    (`…/bootstrap-phases-derive-for-route-setup-project.md:10`).
  - *a11y-plan §8: the Timeout extensions sentence "no case drives one of them under a closed gate yet", and
    the §3 Keyboard test harness Tooling label.* **Carried**: Outcome (the folded case
    `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone`, `tests/cli_budget.rs:470-541`). Sites:
    `no case drives one of them` in a11y-plan 1 hit (`:823`); `cli_budget` in the a11y key file
    `registries/contracts/a11y-plan/keyboard-test-harness.md` 1 hit (`:8`).
  - *obs-plan §4: Edge flows E3 and E5 as landed (no line and no span for a link).* **Carried**: Symbols /
    APIs (the module emits no line and declares no span) and Coverage. Sites: `\bE[35]\b.{0,60}(link|replay|log)`
    in obs-plan 4 hits on three lines (`:347` twice, `:729` E3, `:731` E5). Only E3's implicit half and E5's
    link step landed; E3's `channel-request` / `channel-response{method:"link"|"unlink"}` pair belongs to the
    explicit methods, which are held.
  - *Only when Part B lands: the `link` result, `unlink` of an absent pair, the `-32602` shapes, the
    subcommand and module lists; security-plan's two methods as served and the dated gap; test-plan's E3 cli
    half and the name negative; obs-plan §3 and §4 for a two-name verb; layout-templates' catch site;
    design-system none.* **Not carried**: Part B is held and this chunk changed none of those values. The
    masters' sentences on the explicit verbs, the `link` result as the empty object, and `link` / `unlink` as
    unserved stand as they are.
- **Coverage of new surfaces:**
  - the `link` line a wrapper appends on an implicit link → validation ✓ (the driver is `from`, already
    through `parse_from` and `ViolaName::try_new`; the driven is the wrapper's own name) · instrumentation n/a
    (by the obs plan: no line and no span; the trace is the call's channel pair, the product line and the
    snapshot) · PII n/a (two instance names in the instance's own `events.ndjson`; no home-level line holds
    `driver`, `driven` or `since`, asserted by the root cases) · tests unit + integration · a11y n/a · tokens
    n/a;
  - `apply_link_line` over logged lines → validation ✓ (a name that is no string changes nothing; unknown
    kinds never reach it through `read_from`) · instrumentation n/a · PII n/a · tests unit + crate
    integration · a11y n/a · tokens n/a;
  - `LinkSet::restore` at a wrapper's start, a read of `events.ndjson` → validation ✓ (through `read_from`:
    the line cap, torn and unknown lines stepped over) · instrumentation n/a (no span, so the start's pinned
    span chain is unchanged) · PII n/a · tests unit + integration (`links_survive_a_wrapper_restart`) · a11y
    n/a · tokens n/a;
  - the snapshot's `links` items → validation n/a (written by the wrapper alone) · instrumentation n/a · PII
    n/a · tests unit + integration · a11y n/a · tokens n/a.

## Deviations from intent

1. **Part B is not built.** Not a deviation from the plan: step 11 holds steps 12 to 15 until the founder's own
   word is recorded, and the plan says implement then completes steps 1 to 10 and the operator pass goes ahead
   for Part A. Where the explicit verbs live on the route is the operator's direction at this wrap (plan,
   Implementation notes).
2. **`LinkSet::remove` carries a dead-code allow.** Step 4 lists it among Part A's four operations; its only
   product caller is step 12's `unlink` method. Built as written it fails
   `cargo clippy … -- -D warnings`, so it has `#[cfg_attr(not(test), allow(dead_code))]` with a one-line
   reason. It has its unit cases. The allow ends when the `unlink` method lands.
3. **`record` returns a tuple `(since, already_linked)`**, not a named type: Part A's two callers read neither
   value (only the error), and a named type's fields would be dead code under the same lint.
4. **A `link` line with no string `ts` gets `since: "unknown"`.** The plan words `since` as "that line's
   `ts`" and says nothing of a line without one. Taken from the same file's `session_chain`. One unit case
   pins it (`link_rule_a_link_line_without_a_ts_has_since_unknown`).
5. **Step 10's forced-on edit read the snapshot.** The plan words it as "a gate evaluation added to
   `DialogSlot::answer`"; the slot holds no gate handle. The scratch edit refused an answer `budget-paused`
   when the instance's snapshot read `budget_paused: true`. The reading is the one the plan asks for: red at
   the case's step 5.
6. **Cases beyond the plan's lists** (eleven): in `links_set_*`, a log that cannot be read, two drivers, a
   logged pair of another driven instance, a remove that appends, a failed append and a failed snapshot write
   (six); `links_set_at_a_start_over_a_log_with_a_link_is_in_the_first_snapshot` in `src/cmd/run.rs`; one
   error-path case each in `send_link_*` and `answer_link_*`; two `state_replay.rs` cases where the plan
   words one sequence. Each observes a branch the new code has.
7. **The absence cases have forced-on readings the plan did not ask for** (the refusal guard and the own-name
   guard, `evidence/red-green.md` section 3), by `.claude/rules/testing.md`'s remove-the-guard rule.
8. **One source edit went through an inline python heredoc** (step 1's edit of `crates/viola-core/src/lib.rs`)
   in place of the Edit tool, against `host-linux.md` Transports. It landed whole; every later edit used the
   Edit tool.

scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 13 · listed 13 · recorded 0
(companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 61`, base `f02d5486`).

## Decisions & corrections

- **The operator's direction for the run** (`inputs#I4`): Part A alone, steps 12 to 15 reported as held, no
  live `claude` start, no mutation run, then the operator pass on the operator's word with the
  `ci.py conclusion` read, a red folded into a fix commit.
- **The operator's direction for this wrap** (`inputs#I5`): Phase 1 only in this session; the reconcile in a
  fresh context from a resume point; the route directions arrive with the resume.
- **The operator's directions with the resume of this wrap** (`inputs#I6`, added at the resume; nothing above
  it in this report was re-authored): the record flips `complete` on Part A with its description rewritten to
  what landed; steps 12 to 15 become one CARRY on the route head "The board: viola list", naming the hold (the
  explicit verbs wait for the founder's word on the widening, its three options quoted); no master says `link`
  or `unlink` is served; the light gate skips `binary(channel_links)` with the reason "step 11 holds Part B";
  and the five shapes that landed on the operator's word stand as built, none ruled by the founder: a refused
  `send` or `answer` records no link (`inputs#I2`), a `from` equal to the wrapper's own name links nothing, a
  `link` line's `source` is `wrapper`, the record sits directly before `send-issued` so an issued send that
  then reads `not-delivered` has linked (`inputs#I3`), and a `link` line with no string `ts` gets
  `since: "unknown"` (Deviations 4).
- **Open for the founder, not decided here:** the boundary widening of the explicit verbs (three places,
  `scope.md`, "Held for the review card, unsettled"). Step 11 names the two forms his word may rule.
- **Decisions taken at implement:** the two kinds stand last in declaration order; the link's `source` is
  `wrapper` (the plan's lean); the record in `answer` sits before the `dialog-answered` line, so a link that
  cannot be recorded writes no such line.
- **A lint finding in new test code:** `clippy::cloned_ref_to_slice_refs` on `[linked.clone()]` in an
  `assert_eq!`; a one-element array bound once reads the same and passes.
- **Sweep hazards met:**
  - the word `link` as a search term reads symlink test messages (`.expect("link")`, 10 sites by `grep -rn -F` over `src`, `crates`, `tests`), two test
    helpers named `link` (`crates/viola-state/src/replay.rs`, `src/cmd/revive.rs`) and the channel's method
    label list; the kind was found by `"link"` with its quotes and by `EventKind::Link`;
  - `[Ff]ourteen` over the masters reads five sites of three unrelated subjects (above);
  - `owed to the route` reads five test-plan sites of five different owners;
  - a pattern `E3 |E5 ` reads a11y-plan lines that are not the edge flows; the edge flows were found with the
    kind word beside the id.
- **On the harness, read at this chunk:** `schema-check` on the dev host reads kept homes only (above); the
  harness's `run` documents name their archive (`target/run-archive/<n>`), which dates a reading without a
  clock.
- **The red-first order worked on two stub bodies**: 27 cases read red before the product and green after,
  in two runs each of the unit and integration filters.

## Outcome

**Acceptance criteria, Part A, each re-asserted against the diff:**
- (arch) kinds in `viola-core`, the rule and the replay's half in `viola-state`, the set in the root bin; no
  crate edge, dependency, env var or file under the home: **met** (the diff's 13 files; the guard green on
  `Cargo.toml`, `Cargo.lock`; clippy green).
- (arch) one issued `send` under `VIOLA_NAME=overseer` leaves exactly one `link` record with `source: wrapper`
  directly before its `send-issued`, whose `cursor` is its own line's start; a second issued send and a taken
  answer add none: **met** (`links_a_send_from_another_instance_links_once`,
  `links_an_answer_from_another_instance_links`, `send_link_*`, `answer_link_*`).
- (arch) a refused `send`, a refused `answer`, a call with no `from` and a call under the wrapper's own name
  record no link: **met** (`links_a_refused_send_and_an_own_name_send_record_none`, the refusal and own-name
  cases of both prefixes, each with a forced-on red).
- (arch) the snapshot's `links` holds one item per pair with `since` = the `link` line's `ts`; a wrapper
  started over a log with links writes that set into its first snapshot and appends no second `link`: **met**
  (`links_survive_a_wrapper_restart`; the unit case in `src/cmd/run.rs`).
- (tests) the replay over a wrapper's log returns the recorded link, equal to the snapshot's, with all three
  skip counts 0; the crate-level suite holds the first-`since`, link-after-unlink and repeated-link cases:
  **met** (`e5_the_replay_reads_the_link_the_wrapper_wrote`, `link_rule_*`, the two `state_replay.rs` cases).
- (tests) the fifteen kinds round-trip with zero on all three counts, and a line of a kind no build knows is
  counted and not returned: **met** (`binary(state_events)`, 4 of 4; the unknown-kind sample is now `bond`).
- (tests) under one closed gate an automated `send` is refused exit 11 `budget-paused` / `five-hour`, and
  `wait`, `last`, `answer` and `pause` each give the exit code and document they give under an open gate; one
  `send-refused` record: **met** (`path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone`; forced-on red
  at its step 5, `evidence/red-green.md` section 2).
- (obs) the implicit link and the restore add no process-log line, no `event` value and no span; no home-level
  line of the new cases holds `driver`, `driven` or `since`; every role line passes the schema and no panic
  line exists: **met**, with one reading corrected: the schema read over the new cases' lines is the cases'
  own and CI's G4 step, not the dev host's `schema-check` entry (Spec claims disproved). `g2: clean` on the
  host and on all three runners; G1 `success` on all three lint jobs.
- (security) no refusal and no permission added by the chunk rests on `from` or on the link set, and a
  `release` carrying a string `from` is still refused: **met** (the diff adds no refusal; the guard holds
  `src/run/wheel.rs` and `src/cmd/release.rs` unmoved; `binary(cli_wheel)` ran green under `pre-push` and CI).
- (tests) `pre-push` green on the Linux dev host and the operator's `ci.py conclusion` read of the pushed HEAD
  prints `verdict: green` for all three OSes: **met** (run `ci#38098238731`, attempt 1).

**Acceptance criteria, Part B:** reported as **held**, as the plan words it for a wrap at which step 11 holds.
Its first criterion's "without it" arm is true of the tree: no product code serves `link` / `unlink` and no
client verb exists. The other six are not met and are not claimed.

**Capabilities:** none claimed (`matrix.py show --chunk`: claimed 0). `v1-36` and `v1-13` are advanced and
proven by neither part.

**Gates** (the block fired once by implement on HEAD `f02d548` plus the uncommitted chunk; then the operator
pass; lines in `evidence/operator-pass.md`):
- `cargo fmt --all --check`: green · exit 0.
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`: green · exit 0.
- `bash scripts/agent-run.sh run --unit`: green · exit 0, `"ok":true` (1899 of 1899).
- `bash scripts/agent-run.sh run --unit --filter 'test(/link_rule_|links_set_|send_link_|answer_link_/)'`:
  green · exit 0, `"ok":true` (34 of 34).
- `bash scripts/agent-run.sh run --integration --filter 'binary(cli_links) | binary(cli_budget) |
  binary(state_replay) | binary(state_events)'`: green · exit 0, `"ok":true` (28 of 28).
- `bash scripts/agent-run.sh run --integration --filter 'binary(channel_links)'`: **red · exit 0 ✗ (exit 1)**,
  `contains "ok":true` ✗. This is the hold of step 11 and nothing else: the entry selects a file of step 15,
  nextest answers `operator didn't match any binary names` and `failed to parse filterset` (the text of the
  entry's baseline), and no test ran. The plan's Test Commands prose says the entry reads red for as long as
  step 11 holds. It is this chunk's own entry, red by construction of a held part, not a red of built code.
  How the light gate treats it is for P7 with the operator.
- `git diff --quiet f02d54865815 -- …` (the preservation guard): green · exit 0.
- `bash scripts/agent-run.sh cleanup --session p-links-smoke` (pre-clean): green, `cleaned:[]`.
- `bash scripts/agent-run.sh boot --session p-links-smoke --instance builder`: green, `"ok":true`.
- `bash scripts/agent-run.sh status --session p-links-smoke`: green, `state:"ready"`.
- `bash scripts/g2-zero-panics.sh`: green, `g2: clean`.
- `bash scripts/agent-run.sh schema-check`: green, `"ok":true` (160 files, 2220 lines; see Spec claims
  disproved for what it read).
- `bash scripts/agent-run.sh cleanup --session p-links-smoke`: green, `processes_gone:true`,
  `endpoint_gone:true`.
- `bash scripts/agent-run.sh pre-push`: green twice · exit 0, `"ok":true`, `"stage":"linux-tests"` (coverage
  2321 of 2321, playwright 1 of 1; lines 98.08, functions 97.99, regions 97.89).
- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` (`leg =
  'operator'`, fired as written): `hygiene: clean`, exit 0, read twice before the pre-CI commit.
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD` (`leg = 'operator'`, through
  `--operator`): green · exit 0, `history moved: … f02d5486→3cae2cec`, a fast-forward.
- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD
  --wait 1800` (`leg = 'operator'`, fired as written): exit 0, `verdict: green`, checks 15/15, run
  `ci#38098238731`.
- Smoke: the plan lists boot, status and cleanup as block entries, so it ran whole in the block and was not
  driven a second time.

**Watches:** none folded (the entry carries no `WATCH`).

**Outcome basis:** the operator pass ran, so the verdicts rest on its final state: the one commit
`3cae2ce` and CI run `ci#38098238731` on it, recorded in `evidence/operator-pass.md`. Implement's report was
given in this same conversation and is the basis for the deviations, the readings of step 10 and the census.
No operator directive arrived between implement's report and this one but `inputs#I5`.

**Process hygiene** (implement's census, re-measured at this wrap by the host process list, 00:37:08Z: no process
under this repository's `target/` or the test-home base):

| process | started by | final state |
|---|---|---|
| the smoke session `p-links-smoke` (supervisor, one wrapper, one fake agent) | the block's boot entry | terminated by the block's cleanup entry (`processes_gone:true`, `killed:[]`) |
| wrappers and fake agents of the unit, integration, forced-on and `pre-push` runs | those runs | terminated with their tests; none left |
| the code-graph refresh | this wrap's Setup | exited 0 (`rust` 5311 nodes / 28724 edges, `ts` 7 / 1) |

Six `viola-test-*` homes dated 2026-10-08 stand on the test-home base; they are earlier chunks' and were left.
No live `claude` was started, no rig home was made, no ledger row and no stamp was written.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: f02d5486 (the parent of the oldest pre-CI commit 3cae2cec) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .config/nextest.toml — added 1 line(s) in 1 range(s)
added: 32
### crates/viola-core/src/lib.rs — added 14 line(s) in 9 range(s)
added: 63-64 · 69 · 83-84 · 119 · 138-139 · 484 · 526-527 · 553 · 571-572
### crates/viola-state/src/events.rs — added 11 line(s) in 3 range(s)
added: 809 · 900-907 · 930-931
### crates/viola-state/src/replay.rs — added 194 line(s) in 4 range(s)
added: 7 · 56 · 76-101 · 482-647
- 76-100 @80 «pub fn apply_link_line(links: &mut Vec<Value>, line: &Value) -> bool {»
  - 82-84 «let (Some(driver), Some(driven)) = (data["driver"].as_str(), data["driven"].as_str()) else {»
  - 85-87 «let held = links»
  - 88-99 «match (line["kind"].as_str().and_then(EventKind::from_name), held) {»
  - 485-488 «fn logged(kind: &str, driver: &str, driven: &str, ts: &str) -> Value {»
  - 490-492 «fn pair(driver: &str, driven: &str, since: &str) -> Value {»
  - 494-502 @495 «fn link_rule_a_link_adds_its_pair_with_the_lines_ts() {»
  - 504-512 @505 «fn link_rule_a_repeated_link_keeps_the_first_since() {»
  - 514-527 @515 «fn link_rule_a_link_after_an_unlink_takes_the_later_ts() {»
  - 529-545 @530 «fn link_rule_an_unlink_of_an_absent_pair_changes_nothing() {»
  - 547-581 @550 «fn link_rule_two_drivers_of_one_driven_are_two_pairs() {»
  - 583-598 @584 «fn link_rule_a_line_with_a_name_that_is_no_string_changes_nothing() {»
  - 600-614 @601 «fn link_rule_a_line_of_another_kind_changes_nothing() {»
  - 616-623 @618 «fn link_rule_a_link_line_without_a_ts_has_since_unknown() {»
  - 625-646 @626 «fn link_rule_replay_holds_the_set_its_one_pass_derives() {»
### crates/viola-state/tests/state_events.rs — added 13 line(s) in 4 range(s)
added: 56 · 115-124 · 136 · 183
### crates/viola-state/tests/state_replay.rs — added 78 line(s) in 2 range(s)
added: 9 · 251-327
- 251-255 «const NONE_SKIPPED: Skipped = Skipped {»
- 257-269 @259 «fn append_pair(dir: &Path, kind: EventKind, second: u32) -> String {»
  - 261-264 «let at = Utc»
- 271-276 @272 «fn replayed_links(dir: &Path) -> Vec<Value> {»
- 278-298 @281 «fn state_replay_links_hold_the_first_since_until_an_unlink_and_a_later_one_after_it() {»
  - 294-297 «assert_eq!(»
- 300-326 @303 «fn state_replay_a_snapshot_cut_short_is_replayed_with_the_logged_link() {»
  - 308-313 «OpenOptions::new()»
  - 315-324 «assert_eq!(»
### src/cmd/run.rs — added 54 line(s) in 12 range(s)
added: 29 · 270-271 · 274-280 · 291 · 318 · 524-525 · 530 · 534-535 · 559 · 1095-1100 · 1112 · 1120-1148
  - 274-278 «let links = Arc::new(LinkSet::new(»
  - 1120-1147 @1123 «fn links_set_at_a_start_over_a_log_with_a_link_is_in_the_first_snapshot() {»
### src/run/dialog.rs — added 155 line(s) in 8 range(s)
added: 11 · 27 · 76 · 165 · 174-178 · 325-327 · 366-370 · 1071-1208
  - 174-177 @175 «pub(crate) fn links_with(&self, links: Arc<LinkSet>) {»
  - 1072-1080 @1073 «fn linking(dir: &Path, links_dir: &Path, wheel: Arc<WheelSlot>) -> DialogSlot {»
  - 1082-1107 @1084 «fn raise_and_send(»
  - 1109-1111 «fn kinds(dir: &Path) -> Vec<Value> {»
  - 1113-1115 «fn links_on_disk(dir: &Path) -> Vec<Value> {»
  - 1117-1138 @1118 «fn answer_link_a_taken_answer_from_another_instance_appends_one_link() {»
  - 1140-1151 @1141 «fn answer_link_a_second_taken_answer_adds_no_second_line() {»
  - 1153-1169 @1157 «fn answer_link_a_refused_answer_appends_none(#[case] human_key: bool, #[case] refused: Value) {»
  - 1171-1189 @1174 «fn answer_link_a_wrong_kind_no_from_and_the_wrappers_own_name_append_none() {»
  - 1191-1208 @1194 «fn answer_link_a_link_that_cannot_be_recorded_is_internal_and_the_answer_is_not_taken() {»
### src/run/links.rs — new file · 370 line(s)
- 19-24 «pub(crate) struct LinkSet {»
- 26-105 «impl LinkSet {»
  - 27-34 «pub(crate) fn new(name: ViolaName, instance_dir: PathBuf, snapshots: Arc<Snapshots>) -> Self {»
  - 36-38 «fn set(&self) -> MutexGuard<'_, Vec<Value>> {»
  - 40-48 @41 «pub(crate) fn restore(&self) -> Result<(), StateError> {»
  - 50-53 @51 «pub(crate) fn current(&self) -> Vec<Value> {»
  - 55-66 @58 «pub(crate) fn record(&self, driver: &ViolaName) -> Result<(String, bool), StateError> {»
  - 68-79 @71 «pub(crate) fn remove(&self, driver: &ViolaName) -> Result<bool, StateError> {»
  - 81-87 @82 «fn since(&self, set: &[Value], driver: &ViolaName) -> Option<String> {»
  - 89-93 «fn line(&self, kind: EventKind, driver: &ViolaName) -> EventLine {»
  - 95-104 @98 «fn change(&self, set: &mut Vec<Value>, line: &EventLine) -> Result<(), StateError> {»
- 107-370 @108 «mod tests {»
  - 116-118 «fn name(text: &str) -> ViolaName {»
  - 120-137 «fn first() -> InstanceSnapshot {»
  - 139-144 @140 «fn set_on(dir: &Path) -> LinkSet {»
  - 146-152 «fn events(dir: &Path) -> Vec<Value> {»
  - 154-156 «fn on_disk(dir: &Path) -> Vec<Value> {»
  - 158-160 «fn pair(driver: &str, driven: &str, since: &str) -> Value {»
  - 162-170 @163 «fn logged(dir: &Path, kind: EventKind, data: Value, second: u32) {»
  - 172-174 «fn overseer_builder() -> Value {»
  - 176-211 @177 «fn links_set_restore_over_a_log_with_link_unlink_link_holds_the_last_since() {»
  - 213-221 @214 «fn links_set_restore_over_an_absent_log_is_empty() {»
  - 223-229 @224 «fn links_set_restore_of_a_log_that_cannot_be_read_is_an_error() {»
  - 231-254 @232 «fn links_set_record_twice_writes_one_line_and_returns_the_same_since() {»
  - 256-270 @257 «fn links_set_two_drivers_are_two_pairs_in_the_order_they_linked() {»
  - 272-295 @275 «fn links_set_a_logged_pair_of_another_driven_is_not_this_instances() {»
  - 297-309 @298 «fn links_set_remove_of_an_absent_pair_writes_nothing() {»
  - 311-333 @312 «fn links_set_remove_appends_one_unlink_and_a_later_record_takes_a_later_since() {»
  - 335-345 @337 «fn links_set_an_append_that_fails_leaves_the_set_without_the_pair() {»
  - 347-369 @350 «fn links_set_a_snapshot_write_that_fails_leaves_the_set_with_the_logged_pair() {»
### src/run/mod.rs — added 1 line(s) in 1 range(s)
added: 8
### src/run/send.rs — added 148 line(s) in 8 range(s)
added: 35 · 89-91 · 98 · 115 · 131-135 · 323-325 · 395-399 · 2286-2414
  - 131-134 @132 «pub(crate) fn links_with(&self, links: Arc<LinkSet>) {»
  - 395-399 «if let (Some(links), Some(from)) = (slot.links.get(), ctx.from.as_ref())»
  - 2287-2294 @2288 «fn linking(links_dir: &Path) -> (SendSlot, Pastes) {»
  - 2296-2303 @2297 «fn line_starts(dir: &Path) -> Vec<usize> {»
  - 2305-2333 @2308 «fn send_link_an_issued_send_from_another_instance_appends_link_before_send_issued() {»
  - 2335-2354 @2336 «fn send_link_a_second_issued_send_adds_no_link() {»
  - 2356-2376 @2360 «fn send_link_a_refused_send_adds_none(»
  - 2378-2388 @2380 «fn send_link_a_send_under_the_wrappers_own_name_adds_none() {»
  - 2390-2399 @2393 «fn send_link_a_send_with_no_from_adds_none(#[case] params: Value) {»
  - 2401-2414 @2403 «fn send_link_a_link_that_cannot_be_recorded_is_internal_and_issues_nothing() {»
### tests/cli_budget.rs — added 75 line(s) in 2 range(s)
added: 30-31 · 470-542
- 470-541 @475 «fn path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone(stamped_home: StampedHome) {»
  - 480-482 «let lines = wait_events(&dir, "the question's line", |l| {»
  - 483-486 «let raised = lines»
  - 487-490 «let dialog_id = raised["data"]["dialog_id"]»
  - 498-501 «assert_eq!(»
  - 510-513 «assert_eq!(»
  - 514-517 «assert_eq!(»
  - 528-531 «let refusals: Vec<&Value> = lines»
  - 534-537 «for ran in [&wait_closed, &last_closed, &answered, &paused] {»
### tests/cli_links.rs — new file · 281 line(s)
- 28-31 @29 «fn boot(stamped: StampedHome, script: Option<&str>) -> Wrapper {»
- 33-37 @34 «fn send(home: &Path, from: Option<&str>) -> Ran {»
- 39-41 «fn links(lines: &[Value]) -> Vec<&Value> {»
- 43-57 @44 «fn the_link(lines: &[Value]) -> (usize, &Value) {»
  - 45-47 «let at: Vec<usize> = (0..lines.len())»
  - 52-55 «assert_eq!(»
- 59-62 @60 «fn snapshot_links(instance_dir: &Path) -> Value {»
- 64-67 @65 «fn linked_since(link: &Value) -> Value {»
- 69-76 @70 «fn line_starts(instance_dir: &Path) -> Vec<usize> {»
  - 73-75 «std::iter::once(0)»
- 78-97 @80 «fn assert_logs_clean(home: &Path) {»
  - 83-96 «for entry in std::fs::read_dir(home.join("diagnostics")).expect("diagnostics") {»
- 99-137 @103 «fn links_a_send_from_another_instance_links_once(stamped_home: StampedHome) {»
  - 115-118 «assert_eq!(»
  - 123-125 «wait_events(&dir, "the scripted Stop's turn-ended", |l| {»
  - 129-132 «assert_eq!(»
- 139-181 @141 «fn links_an_answer_from_another_instance_links(stamped_home: StampedHome) {»
  - 146-148 «let lines = wait_events(&dir, "the question's line", |l| {»
  - 149-152 «let asked = lines»
  - 153-155 «let dialog_id = lines[asked]["data"]["dialog_id"]»
  - 157-161 «let answer = |id: u64| {»
  - 165-168 «assert_eq!(»
- 183-216 @186 «fn links_a_refused_send_and_an_own_name_send_record_none(stamped_home: StampedHome) {»
  - 195-198 «assert_eq!(»
  - 206-209 «let issued: Vec<&Value> = lines»
- 218-251 @221 «fn links_survive_a_wrapper_restart(stamped_home: StampedHome) {»
  - 243-246 «assert_eq!(»
- 253-281 @256 «fn e5_the_replay_reads_the_link_the_wrapper_wrote(stamped_home: StampedHome) {»
  - 267-270 «assert_eq!(»
  - 271-278 «assert_eq!(»
