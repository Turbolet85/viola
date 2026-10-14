proposals:
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy → CLI (`tests/cli_verify.rs`)
    change: >-
      `tests/cli_verify.rs` pins one `[NN/MM] <row id> <row words>  pass|fail` step line per relied ledger row, MM the
      relied count (`LedgerRow::relied()`: 21 on Unix, the last `[21/21] statusline-rate-limits`; 18 on Windows, the
      last `[18/18] paste-newline`; `STEPS_PASS` / `ROW_IDS` are `[&str; 21]`, `STATUSLINE_ROWS` the three statusline
      ids), the no-framing-replay and tag-turn-modal summaries are the per-OS literals the test states under
      `cfg!(unix)` (replacing the single `stamped 2.1.0  17 pass  1 fail` / `16 pass  2 fail` readings), Run B carries
      the probe `--settings` on Unix only, and the identity-strip case asserts two `statusline` receipts on Unix.
    sidecar: >-
      §5 CLI `cli_verify`: step lines at the relied count (21 Unix / 18 Windows) with per-OS summaries; Run B's probe
      `--settings` and the two `statusline` receipts (chunk 2026-10-13-owed-ledger-rows).
    rationale: >-
      Report Symbols / APIs (`check_rows` walks `LedgerRow::relied()` and the step line's total is its length), Counts
      (rows 18 → 21 in `ALL`, MM now the relied count, 21 Unix / 18 Windows), Coverage of new surfaces (the step lines
      at the relied count → integ `cli_verify` per OS), Deviations 8 (identity-strip case asserting two `statusline`
      receipts on Unix), Expected amendments (test-plan §5 site `test-plan.md:656`, rows18 and step-counter).
    basis: tests/cli_verify.rs:27-50, tests/cli_verify.rs:72-95, tests/cli_verify.rs:122-127, tests/cli_verify.rs:1265-1269
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy → CLI (`contract_ledger_probes.rs`)
    change: >-
      `contract_ledger_probes.rs` must list each relied literal ledger row id exactly once (`ROW_IDS: [&str; 21]`, the
      three statusline ids relied on Unix only) and end `stamped <ver>  <n> pass  0 fail` with `<n>` the relied count
      (21 on Unix, 18 on Windows); the sub-bullet "the `<n>` must equal the literal row count" becomes "must equal the
      relied count (`LedgerRow::relied()`'s length on that OS)".
    sidecar: >-
      §5 `contract_ledger_probes`: eighteen → the relied ids (`ROW_IDS` 21, relied 21 Unix / 18 Windows) and the
      relied-count summary.
    rationale: >-
      Report Symbols / APIs (`LedgerRow::ALL` is `[Self; 21]`; `relied()`; the verdict over `relied()`), Counts (sweep
      rows18 site `test-plan.md:666` ×2), Outcome (`contract_ledger_probes` green), Expected amendments (test-plan §5).
    basis: tests/contract_ledger_probes.rs:16-39
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation (`boot` step 4)
    change: >-
      Unless `--unstamped`, `viola verify` against the fake agent stamps every relied ledger row (`LedgerRow::relied()`:
      21 on Unix, the first 18 of `ALL`'s 21 elsewhere), Run B also carrying on Unix the probe `--settings`
      (`typed/statusline.json`) whose status line is filed by the hidden `hook statusline --capture` arm; the rest of
      the step (exit 0, the `^stamped \S+  \d+ pass  0 fail$` last line, the only writer of `ledger/stamps.json`)
      unchanged.
    sidecar: >-
      §3 → 5-command implementation boot step 4: "all eighteen ledger rows (`LedgerRow::ALL`)" → the relied set
      (21 Unix / 18 Windows) and Run B's probe `--settings` on Unix.
    rationale: >-
      Report Harness / gate surface and Counts (relied set per OS; sweep rows18 site
      `registries/contracts/test-plan/5-command-implementation.md:8`), Symbols / APIs (`typed.rs` `trusted_args`
      appends `--settings` for Run B on Unix), Outcome smoke (boot stamps 21 rows against the fake agent). obs-plan §3
      states no row count, so only test-plan §3 moves.
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation (`run --local-live`)
    change: >-
      Suite `local-live` checks each relied literal row id: `LEDGER_ROWS` is `[&str; 21]` under `cfg(unix)` (the
      eighteen named, then `statusline-override` · `statusline-shell` · `statusline-rate-limits`) and the 18-id form
      under `cfg(not(unix))` (not `cfg(windows)`, so the harness is not enrolled in the Windows mutation scope); the
      literals are unit-proven by `LIVE_PASS` (the same split) and `run_local_live_reads_the_relied_rows`. The "until
      its next firing re-stamps it at eighteen / the tmpfs home stamped at seventeen reads `row-missing`" clause is
      replaced: the live firing at twenty-one rows was made at chunk 2026-10-13-owed-ledger-rows (`evidence/live-round.md`:
      five starts, `"suite":"local-live","passed":1,"failed":0`, census empty); the seventeen-row firing stays as
      history.
    sidecar: >-
      §3 → 5-command implementation `run --local-live`: `LEDGER_ROWS` per OS (21 `cfg(unix)` / 18 `cfg(not(unix))`),
      `LIVE_PASS` split, the twenty-one-row live firing recorded.
    rationale: >-
      Report Harness / gate surface (`LEDGER_ROWS` per OS, which `run --local-live` asserts), Symbols / APIs
      (`harness/run.rs` `LEDGER_ROWS`, `LIVE_PASS`, `run_local_live_reads_the_relied_rows`), Deviations 4 (the
      `cfg(not(unix))` spelling), Outcome (the `run --local-live` round 5 starts, passed 1, census empty), Counts (sweep
      rows18 site `registries/contracts/test-plan/5-command-implementation.md:53` ×4). obs-plan §3 holds no row count:
      the change is one-sided only if test-plan §3 is left at eighteen.
    basis: crates/viola-e2e/src/harness/run.rs:941-963, crates/viola-e2e/src/harness/run.rs:1026-1040
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy → viola-agent-claude (the ledger's arms)
    change: >-
      Add after the four framing arms: the ledger's three statusline arms — `statusline-override` (passes on ≥ 1
      capture), `statusline-shell` (≥ 1 capture, every one `BinSh`) and `statusline-rate-limits` (`whole_rate_limits`:
      some capture one JSON object whose `rate_limits.five_hour` and `.seven_day` each hold an integer
      `used_percentage` 0–100 and an integer `resets_at`) — in nine `check_statusline_*` cases (each arm's pass, broken
      shape and no-capture fail; `test(/check_statusline_/)` 9 passed), with `shell_verdict_reads_only_the_exact_word`,
      `statusline_variants_name_the_first_without_and_the_first_with_rate_limits`,
      `statusline_capture_file_names_round_trip`, and the relied-set cases
      `relied_rows_drop_the_three_statusline_rows_where_the_status_line_is_not_wrapped` and
      `verified_reads_a_failed_statusline_row_by_the_relied_set` (the verdict `verified` iterates `relied()`), since
      chunk 2026-10-13-owed-ledger-rows.
    sidecar: >-
      §4 viola-agent-claude: the three statusline ledger arms and their nine `check_statusline_*` cases, the relied-set
      cases.
    rationale: >-
      Report Symbols / APIs (the three rows, check arms, `ShellVerdict`, `statusline_variants`, `relied()` and the
      verdict over it), Outcome (`test(/check_statusline_/)` 9 passed), Deviations 2 (the override row's third case).
      The new paths carry unit tests; test-plan's unit inventory of the ledger arms stops at the four framing arms.
    basis: crates/viola-agent-claude/src/ledger.rs:2850-3043, crates/viola-agent-claude/src/ledger.rs:1432-1451, crates/viola-agent-claude/src/ledger.rs:2039-2057
  - detector: D-tests-coverage
    severity: warning
    section: §2 Test Strategy → Contract row
    change: >-
      `contract_fake_agent_drift` also compares the fake agent's `statusline` receipts' `stdin_hex` byte for byte, in
      order and count, with each recorded set's `StatusLine.default-1.json` then `StatusLine.rate-limits-1.json` under
      `--framing` (`fake_agent_statusline_replay_matches_every_recorded_statusline_set`, with the synthetic
      `statusline_drift_names_a_changed_byte_a_swapped_order_and_a_short_replay`); and jsonschema checks every
      `fixtures/claude/*/StatusLine.*.json` against `schemas/claude-statusline-fixture.v1.json`, every other hook
      payload against `schemas/claude-fixture.v1.json`.
    sidecar: >-
      §2 Contract row: the status line replay tier in `contract_fake_agent_drift` and the `StatusLine.*.json` schema.
    rationale: >-
      Report Files (new `schemas/claude-statusline-fixture.v1.json` and the two `StatusLine` variants), Schema / config
      (`contract_fixture_hygiene` validates `StatusLine.*.json` against the new schema), Coverage of new surfaces (the
      fake agent's replay → drift contract), Expected amendments (test-plan fixture-schema site `test-plan.md:438`).
    basis: tests/contract_fake_agent_drift.rs:417-451, tests/contract_fake_agent_drift.rs:453-485
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Fixture hygiene (the walk's coverage)
    change: >-
      The walk also covers `2.1.287`'s two status line variants, `StatusLine.default-1.json` and
      `StatusLine.rate-limits-1.json`, against `schemas/claude-statusline-fixture.v1.json` (a tolerant object,
      `session_id` required as a string; `rate_limits`, when present, an object requiring `five_hour` and `seven_day`
      objects), every other hook payload against `schemas/claude-fixture.v1.json` as before, with the same path /
      username / email walk; planted status line cases prove each class red, and
      `statusline_variants_are_walked_for_every_statusline_set` proves the variant walk.
    sidecar: >-
      §7 Fixture hygiene: the walk covers the two `StatusLine` variants against
      `schemas/claude-statusline-fixture.v1.json`.
    rationale: >-
      Report Files, Schema / config (the new schema and the hygiene walk's split), Counts (2.1.287 fixture set +2
      files), Expected amendments (fixture-schema site `test-plan.md:1101`).
    basis: tests/contract_fixture_hygiene.rs:110-205, schemas/claude-statusline-fixture.v1.json:1-19
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Fixture hygiene (the recorder's scrub)
    change: >-
      The `viola verify` recorder's rewrite (home paths to `~`, the OS user as a whole word to `<user>`) and its
      `unclean` refusal cover the two recorded `StatusLine` variants as well as the hook payloads; they are written only
      at 0 failing rows.
    sidecar: >-
      §7 Fixture hygiene: the recorder's scrub and refusal also cover the `StatusLine` variants.
    rationale: >-
      Report Symbols / APIs (`src/cmd/verify.rs` `named_payloads` adds the two `StatusLine` variants to what `--record`
      scrubs, refuses on `unclean` and writes only at 0 failing rows), Schema / config (the scrub unchanged).
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Fixture hygiene
    change: >-
      New bullet: a `workspace.repo` naming this repository's own remote is kept in a fixture (the founder's word,
      `inputs#I3`, worded by `inputs#I4`): the `StatusLine` variants' `workspace.repo.owner`, the remote owner's handle,
      is neither rewritten by the scrub's whole-word OS-user rule nor refused by the hygiene walk.
    sidecar: >-
      §7 Fixture hygiene: a `workspace.repo` naming this repository's own remote is kept in a fixture (the founder,
      inputs#I3 / inputs#I4, chunk 2026-10-13-owed-ledger-rows).
    rationale: >-
      Report Spec claims disproved by measurement (test-plan §7 states fixtures are scrubbed of paths and usernames; the
      two variants hold the remote owner's handle, the walk green), Schema / config, Decisions & corrections (the
      founder's word), Expected amendments (added by the founder after the plan; sites `test-plan.md:1096`, `:1103`,
      `:1310`).
  - detector: D-tests-coverage
    severity: warning
    section: §11 Test Anti-Patterns → Test Data
    change: >-
      "Fixtures come only from `viola verify` probe text and are scrubbed of paths and usernames; a `workspace.repo`
      naming this repository's own remote is kept (§7 Fixture hygiene)."
    sidecar: >-
      §11 Test Data: the fixture scrub ban names the kept `workspace.repo` exception.
    rationale: >-
      Same claim as the §7 Fixture hygiene exception (report Spec claims disproved, site `test-plan.md:1310`, sweep
      fx-hygiene).
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Fixture hygiene (the walk's failure classes)
    change: >-
      "it fails on absolute paths …, on non-placeholder usernames (a `workspace.repo` naming this repository's own
      remote is kept), on an email-shaped token, …"
    sidecar: >-
      §7 Fixture hygiene: the walk's username class states the kept `workspace.repo` exception.
    rationale: >-
      The walk's "non-placeholder usernames" class reads the remote owner's handle green (report Spec claims disproved;
      Outcome: "The hygiene walk's 'usernames' reach is the founder-kept exception"); same claim as the §7 exception
      proposal.
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Fake agent (the `--settings` bullet)
    change: >-
      Add: under `--framing`, with `--settings <file>` naming an absolute `statusLine.command` (no `--statusline-stdin`),
      the fake agent replays the set's `StatusLine.default-1.json` after the launch SessionStart's hooks and
      `StatusLine.rate-limits-1.json` once after the first Stop, at the end of the turn after the turn screen is
      written; the command is split on ASCII white space, the one word exactly `"$0"` handed over as `/bin/sh`, spawned
      directly (never through a shell); each writes a `statusline` receipt with `stdin_hex`; a relative command is
      receipted `command_absolute:false, ran:false` and never run; a set without the variants fires nothing; no option
      is added.
    sidecar: >-
      §7 Fake agent: the `--framing` status line replay of the two `StatusLine` variants with the `"$0"` hand-over.
    rationale: >-
      Report Symbols / APIs (`src/bin/viola-fake-agent.rs` `replay_statusline`, `shell_expanded`; the first-Stop
      replay moved after the turn screen by the fix commit), Deviations 7 and 10, Coverage of new surfaces (the fake
      agent's replay, unit ×4 + drift contract), Expected amendments (site `test-plan.md:1076`).
    basis: src/bin/viola-fake-agent.rs:518-535, src/bin/viola-fake-agent.rs:352-364
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Fake agent (Modes: `--settings` / `--statusline-stdin`)
    change: >-
      The parenthetical "(the settings command runs only with both options and only when it is absolute, …)" now also
      names the `--framing` replay of the set's `StatusLine` variants, which runs the absolute settings command without
      `--statusline-stdin`.
    sidecar: >-
      §7 Fake agent Modes: the settings command also runs under `--framing` with the recorded variants.
    rationale: >-
      Same claim as the `--settings` bullet proposal (report Symbols / APIs, `replay_statusline`; site
      `test-plan.md:1093`).
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Fake agent (Modes: `--framing`)
    change: >-
      The `--framing` (Run B) clause adds: and, with an absolute `--settings` status line command, it replays the set's
      `StatusLine.default-1.json` after the launch SessionStart and `StatusLine.rate-limits-1.json` once after the first
      Stop.
    sidecar: >-
      §7 Fake agent Modes: `--framing` names the status line replay.
    rationale: >-
      Same claim as the `--settings` bullet proposal (report Symbols / APIs, `replay_statusline`; site
      `test-plan.md:1093`).
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Fake agent (the rewritten resume payload note)
    change: >-
      The recorded `SessionStart` resume payload is owed to the route entry "Resume and compact ledger rows" (minted
      directly after "Owed ledger rows", carrying `claude --resume <id>` and `/compact`, by the founder's fork 1,
      `inputs#I1`, card `inputs#I2`), no longer to "Owed ledger rows".
    sidecar: >-
      §7 Fake agent: the resume payload note re-owed to "Resume and compact ledger rows".
    rationale: >-
      Report Chunk line (`--resume` / `/compact` moved off this entry by the founder's forks, `inputs#I1`), Decisions &
      corrections (the founder's route moves), Expected amendments (test-plan "the rewritten-resume-payload note re-owed
      to the new entry", site `test-plan.md:1093`).
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Recorded hook payloads row
    change: >-
      Add: on Unix, Run B's probe status line records `StatusLine.default-1.json` and `StatusLine.rate-limits-1.json`
      (two under `2.1.287`), scrubbed and `unclean`-checked like the rest and validated against
      `schemas/claude-statusline-fixture.v1.json`; CI replays them.
    sidecar: >-
      §7 Recorded payloads: the two `StatusLine` variants recorded by Run B's probe status line.
    rationale: >-
      Report Files (the two variants recorded live and copied alone), Counts (2.1.287 set +2 files), Symbols / APIs
      (`named_payloads`; Run B's probe `--settings`).
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → contract-test-against-sandbox trigger
    change: >-
      "Live `viola verify` refreshes … through Run B's four added pastes the framing variants (…), and through Run B's
      probe status line (Unix) the two `StatusLine` variants, locally."
    sidecar: >-
      §1 contract trigger: verify also refreshes the `StatusLine` variants.
    rationale: >-
      Same claim as the Recorded hook payloads row proposal (what live verify records; report Files, Symbols / APIs).
    dependent-of: D-tests-coverage
note: >-
  Kept, no proposal: `test-plan.md:752`'s `stamped 2.1.287  18 pass  0 fail` is the paste-newline chunk's dated live
  reading (the report keeps it as history); `bootstrap-phases-derive-for-route-setup-project.md:10`'s "eighteen
  verify-driven binaries" counts nextest binaries, not ledger rows (a rows18 sweep false positive); `test-plan.md:219`
  quotes the security excerpt ("checked for absolute paths and usernames"), still true and a quote, so left; the
  hidden `hook statusline --capture` arm is tested (hook.rs ×4, hook_fail_open ×2) and test-plan documents no hidden
  capture arm, so no inventory line is proposed. D-tests-framework: no drift (nextest via agent-run, rstest, jsonschema,
  no new runner or dependency).
