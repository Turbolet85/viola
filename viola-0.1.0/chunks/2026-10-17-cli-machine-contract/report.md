# Report — 2026-10-17-cli-machine-contract

**Chunk:** CLI machine contract: --json per verb mirroring channel results, typed exits, cause-named hints, global --home with the VIOLA_DIR step, config.json settings
**Date:** 2026-10-17T03:53Z
**Commits:** since `last_wrap` 2026-10-17T02:19:29Z (`git log --format='%h %s' f6bcce5..HEAD`):
`e248533 chore(2026-10-17-cli-machine-contract): operator pre-CI commit, for the run this chunk's verdict reads` ·
`f738cbc fix(2026-10-17-cli-machine-contract): the canonical-home unit's oracle is the plain drive form on Windows`.

## Changes (structured — detectors read this)
- **Files:** (basis: `gate.py scope` at this wrap — `changed 22 · listed 21 · recorded 1`, base `f6bcce5b`, the parent
  of the oldest pre-CI commit `e2485333`; the per-file added ranges are `## New text, by line` below)
  - product: `src/cmd/mod.rs`, `src/cmd/hook.rs`, `src/cmd/run.rs`, `src/cmd/revive.rs`, `src/cmd/client.rs`,
    `src/main.rs`, `src/cmd/verify/typed.rs`, `src/obs.rs` (test module only), `crates/viola-state/src/replay.rs`;
  - schemas: `schemas/diag-line.v1.json` (changed), `schemas/config.v1.json` (new);
  - tests: `tests/cli_machine_contract.rs` (new), `tests/cli_controls_not_disableable.rs`, `tests/cli_revive.rs`,
    `tests/cli_list.rs`, `tests/contract_diag_schema.rs`, `tests/cli_instance_state.rs`, `tests/cli_verify.rs`,
    `tests/support/cli.rs`, `tests/support/home.rs`, `tests/support/start.rs`, `tests/support/outer_pty.rs`;
  - ledger: `viola-0.1.0/verification-matrix.json` (`v1-39` `planned → implemented`, its ref);
  - chunk folder: `scope-record.md`, `evidence/ci-red.md`, `evidence/guards.md`, `evidence/operator-pass.md`.
- **Symbols / APIs:**
  - `src/cmd/mod.rs` — `Instance` and `instance_of(name, dir)` MOVED here from `src/cmd/hook.rs` (`pub(super)`,
    unchanged; `hook` and `hook/statusline.rs` keep calling them through `hook.rs`'s import — the one `VIOLA_NAME` +
    `VIOLA_DIR` shape check). `resolve_home(flag, name, dir, user_home)` now takes its inputs as arguments and resolves
    in the documented order: `--home` as given → the `instance_of` home when both variables pass → `<user home>/.viola`;
    a pair that fails the check falls through, never refuses. `resolved_home(flag)` reads `VIOLA_NAME`, `VIOLA_DIR` and
    `std::env::home_dir()` once and is called at every one of `dispatch`'s ten verb arms (every verb but `hook`, `run`
    and `revive` included). `canonical_home(home)` (`pub(super)`): `std::fs::canonicalize` when the path exists, a
    Windows `\\?\C:\…` result written back as `C:\…` (`plain_drive`, a string-level strip of exactly the verbatim-drive
    prefix; `\\?\UNC\…`, `\\?\Volume{…}\…` and `\\.\…` stay as given); a path that does not exist (or cannot be read) keeps
    `std::path::absolute`'s spelling — nothing is created. `default_home` unchanged, still shared with
    `statusline_source`.
  - `src/cmd/run.rs` — `start` canonicalises the home (`super::canonical_home`) where it made it absolute before
    (after `open_wrapper_log` has created it, before the collision check, the pin, the endpoint bind and every path
    built over it); `statusline_source` compares the home with `canonical_home(default_home(user_home))` (was
    `std::path::absolute`). `collision_check(name, instance_dir, json: bool)` gained `json`: with it a `live`/`stale`
    holder writes the `already-live` document (`client::refused_document`) in place of `run`'s stderr pair; `run`'s
    own call passes `false`; its other caller is `revive`'s preflight.
  - `src/cmd/revive.rs` — `ReviveArgs` gains `--json` (allowed on both arms; `list` is now `pub(super)` for
    `dispatch`'s sink). `Refusal::write(name, json)` writes `{"v":1,"error":"<detail>","detail":null}` under `--json`,
    the unchanged `unable:`/`hint:` pair otherwise; the codes are `Refusal::detail` (`strict-modes-failed`,
    `no-session` for both of its pairs, `session-live`, `cwd-missing`) plus `already-live` from `collision_check`.
    `--list`: after its strict-modes check passes, and only under a parsed `VIOLA_NAME`, it opens the CALLER's
    `cli-<VIOLA_NAME>.ndjson` through `client::start` and logs `process-start` and `process-exit` (`exit_code` 0 with no
    detail; `exit_code` 1 `detail:"no-session"` on an empty chain); under `--json` it prints
    `{"v":1,"ok":{"sessions":[{"ts","cause","id"},…]}}` in log order (fields as the rows: the escaped `ts`, the cause word,
    the session-shaped id), or the `no-session` document with exit 1. New `ListRow {ts, cause, id}` with `line()` (the
    human `<ts>  <cause>  <id>`, byte-identical); `cause_word` now returns `&'static str`. A refused check still opens
    nothing and writes no line, on both arms.
  - `src/cmd/client.rs` — `refused_document(code)`: one `--json` exit-1 document through `document`.
  - `src/cmd/mod.rs` `dispatch` — the `Revive` arm's detail sink: with `--list`, the caller's own (`own_name()`,
    `ObsProcess::Cli`), else the instance's `run` sink as before.
  - `src/main.rs` `role_of` — `revive` followed (after any `--home` pair) by `--list` before a `--` is `Role::Cli`;
    `revive` otherwise `Role::Other`; a `--list` after `--` is the child's word. So a `--list` dispatch `Err` or panic
    reaches the `cli` catch site (`process-exit{detail:"internal-error"}`, exactly `error: internal error`).
  - `src/cmd/verify/typed.rs` `count_of(dir, event)` — counts a capture only when its name parses as
    `<Event>.<k>.json` AND the file is non-empty; `captured` (`count_of > 0`) and every wait over it (Run B's Stop
    counts, Run C/D's waits, Run E's `wait_capture`) take the rule. The capture arm (`src/cmd/hook.rs` `file_from`) is
    unchanged: it still claims the name empty and renames the bytes over it after a `sync_all`.
  - `crates/viola-state/src/replay.rs` — `read_snapshot_or_replay`'s `state-recovered` line gains `scanned`: the
    instance dir's own last component when it parses as a `ViolaName`, absent otherwise (`scanned_name`, private); the
    callers' signatures (board.rs, revive.rs) unchanged; the other `state-recovered` emitter (events.rs, the torn-line
    heal) writes no `scanned`.
  - remaining callers: `instance_of` — `hook`, `hook statusline`, `resolve_home`; `collision_check` — `run::start`,
    `revive::preflight`; `canonical_home` — `resolved_home`, `run::start`, `run::statusline_source`;
    `refused_document` — `revive::Refusal::write`, `run::collision_check`.
- **Crates / modules:** none added or removed (changed: root `viola`, `viola-state`).
- **Dependencies:** none.
- **Schema / config:** `schemas/diag-line.v1.json` `state-recovered` row: `scanned` added — a string matching
  `^[a-z][a-z0-9-]{0,31}$` (the schema's `instance` pattern); absent or a `ViolaName`, never a path or a literal
  `null` (`contract_diag_schema`). New `schemas/config.v1.json`: tolerant (`additionalProperties: true`), properties
  `v` (const 1), `diagnostics_level` (`info`|`debug`), `claude_env_keep` (≤32 strings `^CLAUDE[A-Z0-9_]*$`),
  `five_hour`/`seven_day` (integers 0-100) — the five `CONFIG_KEYS` (`src/obs.rs`, pinned equal as a set by
  `config_keys_are_the_config_schema_properties`). No config key, env var or flag added; the `VIOLA_*` variables a
  build reads stay `VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN` (the completeness case walks the product sources for them).
  `VIOLA_DIR` gains readers: every CLI verb (through `resolved_home`), `run` and `revive` included.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:** (basis: `cascade.py sweep --pre-pass` over `{run_dir}/report-patterns.toml`, trail
  `cascade-2026-10-17-cli-machine-contract-pre-pass.json`; every row below `standing`, the master rows only — the 22
  leaf rows are P2's cascade)
  - `revive` takes no `--json` → takes it on both arms · pattern `revive-nojson`: 11 standing · stale 10
    (architecture.md:98, :300; security-plan.md:203 @c3657; design-system.md:506; layout-templates.md:243, :266;
    test-plan.md:67, :862; a11y-plan.md:27; obs-plan.md:462 — its "`viola revive --list`, which opens no process log"
    clause, read with `list-nolog`) · no-change 1 (architecture.md:69 @c2634 — "a revived one included … restores
    none", the budget override, a true claim sharing the token).
  - revive's exit-1 document `{"v":1,"error":"<code>","detail":null}` (new) · pattern `revive-doc`: 0 — control
    unfired over the seven masters at `f738cbc0` (the refusal line of the first sweep call); the CLI `--json` output
    contract states no such document yet.
  - `revive --list` opens no process log (obs-plan §10's sixth bounded exemption) → it logs as `cli` under a
    `VIOLA_NAME` · pattern `list-nolog`: 8 standing · stale 8 (architecture.md:341 @c337; obs-plan.md:143, :462, :530,
    :822, :988, :989 — "the founder's direction named in exemption 6"; registries/contracts/obs-plan/otel-sdk-init.md:20).
  - obs-plan §10's numbered exemptions · pattern `exempt-count`: 3 standing (obs-plan.md:987 "as in exemption 3",
    :988 "as in exemptions 2 and 5", :989 "exemption 6") — :989 stale with exemption 6's retirement; :987 and :988
    no-change unless P2 renumbers the list (exemption 7 stands, `.claude/rules/observability.md:39`).
  - `role_of` files `revive` (every arm) beside `run` → `revive … --list` is `cli` · pattern `role-of`: 3 standing ·
    stale 2 (architecture.md:129 @c3858/@c3955 ×2; obs-plan.md:822 @c490) · no-change 1 (test-plan.md:839 — the unit
    name `role_of_skips_the_home_flag_and_its_value`, still the table's name).
  - the `cli` role file's producers (architecture.md:341's list `verify, send, wait, last, list, answer, pause,
    release`) gain `revive --list` — the same row as `list-nolog` above, one disposition.
  - `state-recovered` carries the caller's instance, never the scanned one (owed) → `scanned` · pattern `scanned`: 9
    standing · stale 1 (obs-plan.md:656 @c1506) · no-change 8 (architecture.md:47 ×2 and security-plan.md:208 ×2 —
    "summed over every scanned instance", the board's skip counts; security-plan.md:378, obs-plan.md:886, :892,
    test-plan.md:1053, registries/contracts/test-plan/5-command-implementation.md:101 — secret-scan "never scanned",
    "scanned before upload", "secret-scanned homes"; obs-plan.md:965 — G2 "nothing is scanned").
  - the home canonicalised (owed) → canonicalised once it exists · pattern `canonical`: 7 standing · stale 1
    (security-plan.md:203 @c3960/@c4026 — "`viola list` makes the home absolute … and does not canonicalise it, as every
    `cli` verb resolves `--home` today (the canonicalisation is owed …)") · now-true 1 (security-plan.md:203 @c190/@c215 —
    "`--home` and `VIOLA_DIR` are canonicalised (`std::fs::canonicalize`)", the strict-modes half still Epoch 6's) ·
    no-change 5 (security-plan.md:203 @c487/@c585 — `hook <event>`'s ratified gap and `hook statusline`;
    architecture.md:274, :342, security-plan.md:184, :215 — `hook statusline`'s instance directory; architecture.md:299,
    test-plan.md:955 — `verify --record`'s paths).
  - the `VIOLA_DIR`-grandparent step for the CLI verbs (owed) → landed · pattern `viola-dir-step`: 6 standing · stale 2
    (architecture.md:398 @c291 — "As landed … the CLI verbs take `--home`, else the default, and only `hook` takes the
    `VIOLA_DIR` step … owed"; architecture.md:321 @c376 — "As landed, `hook` is its only reader: the CLI verbs do …") ·
    now-true 4 (architecture.md:398 @c110/@c147 the order; obs-plan.md:203;
    registries/contracts/obs-plan/log-file-location.md:4; registries/contracts/obs-plan/otel-sdk-init.md:11) ·
    no-change (architecture.md:398 @c977/@c1253, security-plan.md:214 — `resolve_home`'s `home_dir()` read shared with
    the statusline source: still true, the read is `resolved_home`'s now).
  - the config schema test-plan §5 reads (absent) → `schemas/config.v1.json` · pattern `config-schema`: 2 standing ·
    now-true 1 (test-plan.md:595 "the config schema a key") · no-change 1
    (registries/contracts/obs-plan/otel-sdk-init.md:17 — "`CONFIG_KEYS` holds five names", still five).
  - the completeness case (owed) → landed · pattern `completeness`: 2 standing · stale 1 (test-plan.md:593 "the
    remaining rows and the completeness case are listed in the file's header") · now-true 1 (test-plan.md:595, its
    shape; the landed case also requires every `VIOLA_*` literal the product reads to be a row).
  - the owner title · pattern `owner-title`: 8 standing — every row an "owed to … CLI machine contract" clause, each
    stale with this chunk (architecture.md:98, :398; security-plan.md:203 @c4071; layout-templates.md:266;
    test-plan.md:862; obs-plan.md:143, :656 @c1538, :988); `owner-generic` control unfired (no master holds the phrase).
  - Run E's capture wait (`count_of` counts a non-empty capture) · pattern `capture-wait`: 0 — control unfired; the
    concurrent-start witness · pattern `concurrent`: 0 — control unfired; the per-verb file · pattern `machine-file`:
    0 — control unfired (no master names `cli_machine_contract`).
  - `CONFIG_KEYS` five names, `process-exit` `detail` enum 13 values: unchanged (none — verified: `src/obs.rs:32-38`,
    `schemas/diag-line.v1.json` `process-exit` row, read at this wrap).
  - Sweep reach: 10 patterns (`total (10 patterns) · 81 rows over 18 files`; 5 more refused, control unfired, on the
    first call's refusal line) · 15 moved facts, 15 with a pattern (the report's own count: the 14 pattern bullets above
    and the `cli` producers fact, on `list-nolog`) · owner pattern 8 standing · by line 4 lines · 8 offsets.
- **Dev-tool versions:** none.
- **Harness / gate surface:** none — no harness, status or CI shape changed; the test supports changed only for the
  home: `tests/support/cli.rs` `start` drops `VIOLA_DIR`/`VIOLA_BIN` beside `VIOLA_NAME`; `tests/support/home.rs`
  gains `UNWRAPPED` (the three names), `resolved(home)` (a test oracle: canonical, plain drive form, a home not there
  yet spelt under its parent's resolved spelling), `boot_with` and `revive_command` drop `UNWRAPPED`;
  `tests/support/outer_pty.rs` gains `OuterPty::spawn_without(…, remove, …)` (`spawn_in` calls it with none);
  `tests/support/start.rs` `endpoint_of` hashes over `resolved(home)`.
- **Cross-project / external claims:**
  - CI runs read: `ci#38998933306` on `e24853337527` (the pre-CI commit) — **red**, `failed 1: test (windows-2025)`
    (one unit's oracle, below), `test (ubuntu-latest)` completed success; `ci#38999642102` on `f738cbc0f307` (HEAD) —
    `verdict: green · checks 15/15 · wall 542 s`. The red wrap CI this chunk folded: `ci#38994749835` on `f6bcce5b`,
    `test (ubuntu-latest)` red, its artifacts (`harness-ubuntu-latest`, `diag-ubuntu-latest`, `junit-ubuntu-latest`)
    downloaded into `target/ci-38994749835/` and read (evidence/ci-red.md §1).
  - `inputs.py verify` (this wrap): `inputs: 2 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
    unreachable 0 · n/a 2 · uncited 0 · unparsed 0`.
    - `I1 · message: the operator, in this phase session, 2026-10-17T02:33Z · copy message · n/a` — fold the red CI
      `ci#38994749835` into this chunk (cited scope.md:11, :22, :117 · research.md:7 · plan.md:20, :328, :380).
    - `I2 · message: the Viola overseer, technical, founder-delegated, answering the P4 dialog, 2026-10-17T02:40Z · copy
      message · n/a` — revive's exit-1 document `{"v":1,"error":"<code>","detail":null}`; `--home` and the
      `VIOLA_DIR` home canonicalised now, plain drive form on Windows (cited plan.md:16, :61, :88, :332, :380, :396,
      :413, :424). PROVISIONAL per the delegate form (the founder's Epoch 5 boundary batch).
  - No input names a P5 review card (`.andromeda/runs/2026-10-17T02-25-51-phase/p5-card-1.md` exists, unnamed by an
    entry).
- **Reverted / negative API facts:** none in the shipped tree. (Measured and restored, not shipped: the three
  remove-the-guard neutralisations, evidence/guards.md, and the `count_of` revert, evidence/ci-red.md §5.)
- **Insufficient fixes (written, kept, not the remedy):** `tests/cli_verify.rs`
  `stamped_home_concurrent_verify_runs_each_stamp_the_home` — written as the plan's forced concurrent start (step 2),
  kept as a guard, but it does NOT reproduce the red on the dev host (40/40 green before and after the fix): it guards
  only where a runner's disk opens the window. The remedy is the `count_of` change; its witness is the unit
  `captured_reads_no_capture_while_the_claim_is_empty`.
- **Spec claims disproved by measurement:**
  - **The plan's Part A premise** — plan.md step 3 ("which shared input two simultaneous verify runs read, at which
    `file:line`") and the scope §7 / research `hypothesis:` ("contend for something Run E reads"): no shared input
    decides it. Measured from the code and forced at the decision (evidence/ci-red.md §2, §4-5): the deciding equality
    is `count_of(dir, HookEvent::SessionStart) > 0` (`src/cmd/verify/typed.rs:467-468` before the fix) reading TRUE on
    the EMPTY claim `create_private_new` made (`src/cmd/hook.rs:390`), before the bytes land through `replace_private`'s
    write, `sync_all` (`crates/viola-state/src/fs.rs:292`) and rename; Run E then settles, kills, and verify reads the
    empty capture (`src/cmd/verify.rs:253`) → `payload: None` → `session_resumed` false
    (`crates/viola-agent-claude/src/ledger.rs:872-878`). The resource the concurrent runs share is the disk's write
    latency. Plan/scope claim, no master site (pattern `capture-wait`: control unfired).
  - **`v1-39`'s and the plan's "every exit-1 refusal" of `revive --json` (THE GAP, owner to be decided at P2/P7):**
    `v1-39` acceptance ("revive's exit-1 `{"v":1,"error":"<code>","detail":null}` with the code one of
    strict-modes-failed, already-live, no-session, session-live, cwd-missing … Stderr is empty") and plan.md
    Acceptance (arch) (`viola revive --json`'s every exit-1 refusal prints exactly … with `<code>` one of the five, with
    empty stderr and exit 1) and step 10. Measured at `src/cmd/run.rs` (read at implement P2): after a PASSED
    preflight `revive` calls `run::start`, which can still refuse — `batch-script-child` (`refuse_batch_script`, the
    stderr pair), `pinned-hash-mismatch` (`refuse_tampered_pin`), `squatted-name` (`refuse_squatted`), and a `claude`
    not on `PATH` → `Err(Refusal::NotFound) => refused("internal-error")` (`run.rs:229`, no output) — each exit 1 with
    NO `--json` document (the human pair on stderr, or nothing), and none of the four codes is in the closed five. Not
    fixed in the impl: a document for them would carry a code outside the acceptance's set. No master site states the
    five-code set (pattern `revive-doc`: control unfired) — the claim lives in `v1-39`'s acceptance (`matrix#v1-39`), the
    plan, and `inputs#I2`'s Q1 option text. `implemented` was written for `v1-39` with this gap surfaced beside it
    (implement's outcome, "Surfaced"); the P7 flip of `v1-39` and its owner (this chunk's escalation, the route entry
    "Exit-cause code catalogue", or a refine of the acceptance) are the wrap's — named here on the operator's word at
    this wrap's invocation.
- **Expected amendments (from plan):** (each entry's sites from the site sweep above, pattern ids named)
  - architecture §Established Decisions [CLI Conventions] revive's "takes no `--json`" → carried: **Symbols / APIs**
    (`src/cmd/revive.rs`) · `revive-nojson` architecture.md:98 (+ :300 in §Conventions CLI flags), `owner-title` :98.
  - architecture §Standard Contracts "CLI `--json` output" — revive's exit-1 documents with the five codes and
    `--list`'s `ok` payload → carried: **Symbols / APIs** (`revive.rs`, `client.rs`) · `revive-doc` 0 sites (control
    unfired) — new text, and the gap in **Spec claims disproved** bears on its wording.
  - architecture §Conventions "CLI exit codes" — the revive lines name their documents → carried: **Symbols / APIs** ·
    `revive-nojson` architecture.md:300.
  - architecture §Cross-cutting Config management — the `VIOLA_DIR` step and canonicalisation as landed, the owed
    clause retired → carried: **Symbols / APIs** (`src/cmd/mod.rs`, `run.rs`) · `viola-dir-step` architecture.md:398
    @c291, :321 @c376; `owner-title` :398.
  - architecture §Occupied Resources — `schemas/config.v1.json`, `VIOLA_DIR`'s reader list, `cli-<name>.ndjson`'s
    producer `revive --list` → carried: **Schema / config**, **Symbols / APIs** · `config-schema` 0 architecture sites;
    `viola-dir-step` architecture.md:321; `list-nolog` architecture.md:341.
  - security-plan §Input Validation CLI arguments row — `revive` takes `--json`; `--home` / `VIOLA_DIR` canonicalised
    as landed, the owed note retired → carried · `revive-nojson` security-plan.md:203 @c3657; `canonical`
    security-plan.md:203 @c3960/@c4026; `owner-title` :203 @c4071.
  - design-system §Surface: cli → Exit-code phraseology (exit 1's revive document) and pattern 5 ("the verb takes no
    `--json`") → carried · `revive-nojson` design-system.md:506; Exit-code phraseology: no `revive-doc` site (new text).
  - layout-templates §Output structure — `viola revive` ("takes no `--json`") and §Component — Primary navigation (the
    owed clause) → carried · `revive-nojson` layout-templates.md:243, :266; `owner-title` :266.
  - test-plan §1 Coverage scope / Surfaces under test (revive's machine form), §5 Cross-module patterns (CLI: the
    completeness case and its config schema as landed), §6 Exit-cause matrix (the revive document's owner done) →
    carried: **Schema / config**, **Symbols / APIs**, the tests in **Files** · `revive-nojson` test-plan.md:67, :862;
    `completeness` :593, :595; `config-schema` :595; `owner-title` :862.
  - obs-plan §10 (exemption 6 retired), §4 cli row (the `--list` exception), §7 Panic hooks (`--list`), §1 / §2
    restatements, §6 `state-recovered` row (`scanned`) → carried: **Symbols / APIs** (`revive.rs`, `main.rs`,
    `replay.rs`), **Schema / config** · `list-nolog` obs-plan.md:143, :462, :530, :822, :988, :989; `exempt-count`
    :987-:989; `role-of` :822; `scanned` :656; `owner-title` :143, :656, :988; registry
    `contracts/obs-plan/otel-sdk-init.md:20` (`list-nolog`). The §6 catalogue row for `state-recovered` names its fields
    without the `scanned` token, so the `scanned` pattern does not reach it (a listing reaches its wording only); its
    fact is in **Schema / config**.
  - a11y-plan §1 CLI verbs entity ("revive takes no `--json`") → carried · `revive-nojson` a11y-plan.md:27.
  - Capabilities — `matrix#v1-39` claimed at P5: `ledger — owner P7.3` (implemented with the surfaced gap; its flip or
    refine is P7.3's, with the gap's owner). `v1-18` stays pooled (unchanged, no master site).
- **Coverage of new surfaces:**
  - `viola revive --json` (both arms) → validation clap (`--list` conflicts unchanged; `--json` free) ✓ ·
    instrumentation `process-exit` per refusal as without `--json` ✓ · PII codes only, no path/id ✓ · tests integ
    (`cli_revive.rs` `revive_json_*`, `revive_list_json_*`; `cli_machine_contract.rs`) · a11y n/a (no TTY change) ·
    tokens n/a.
  - `viola revive --list` `cli` log → validation the strict-modes check first ✓ · instrumentation
    `process-start`/`process-exit` in the caller's `cli-<name>.ndjson` ✓ · PII codes only ✓ · tests integ
    (`revive_list_under_a_name_logs_its_start_and_exit`, schema-validated) · a11y n/a · tokens n/a.
  - home resolution through `VIOLA_DIR` + canonicalisation → validation `instance_of` (name + absolute
    `instances/<name>` dir) ✓, canonicalise only an existing path ✓ · instrumentation n/a (no line) · PII n/a · tests
    unit (`resolve_home_*`, `canonical_home_*`, `plain_drive_*`) + integ (`home_resolution_…`, `two_homes_are_isolated`,
    `a_home_reached_through_a_link_is_canonicalised`, `cli_instance_state`) · a11y n/a · tokens n/a.
  - `state-recovered.scanned` → validation `ViolaName::try_new` on the dir's last component ✓ · instrumentation the
    line ✓ · PII a name, never a path or pid ✓ · tests unit (`state_recovered_names_the_scanned_instance`) + integ
    (`e5_list_reads_a_live_wrapper_through_the_replay`) + contract (`contract_diag_schema`) · a11y n/a · tokens n/a.
  - `schemas/config.v1.json` → validation n/a (a test input; readers unchanged) · tests unit
    (`config_keys_are_the_config_schema_properties`) + integ (the completeness case) · others n/a.

## Deviations from intent
- **Part A, step 3's premise** — the cause is not a shared input (see **Spec claims disproved**); the fix site,
  `src/cmd/verify/typed.rs`, is one the plan listed.
- **Part A, step 4's revert reading** — "with the fix reverted in place, the same case reads red" holds for the forced
  unit case (red before / green / red reverted / green restored, evidence/ci-red.md §5), NOT for the concurrent root
  case, green on this host either way (evidence/ci-red.md §3, §5: 40 firings). Justification: testing.md 2026-09-27 —
  a timing window is forced, never sampled; the window's state (the empty claim) was planted at the decision. The
  STOP rule did not fire: a cause was named at file:line. The runner leg is CI: `test (ubuntu-latest)` green on
  `e2485333` and `f738cbc0`.
- **Step 12's `cli-<name>`** — the plan leaves which name open; the log is the CALLER's `VIOLA_NAME`, as `list` and
  `verify` log under a name (obs-plan §4 cli row), pinned by `revive_list_under_a_name_logs_its_start_and_exit`
  (`VIOLA_NAME=driver` beside the instance `builder`).
- **Step 15** — `setting_does_not_disable_the_budget_gate` gained `seven_day_over_one_hundred` and `v_two` so the
  `seven_day` and `v` rows are each tried (the first three case labels and order kept); `VIOLA_*` is enumerated by
  walking the product sources for `"VIOLA_…"` literals (`viola-e2e` left out), marked `andromeda:walks-tree`; new rows
  `viola_dir_does_not_disable_the_home_scan` (Unix), `viola_variable_does_not_disable_the_send_paste_control`,
  `config_key_does_not_disable_the_identity_strip`.
- **Step 16** — outcomes over one live wrapper's life are a labelled case table per world (each assertion names its
  label), not independent rstest rows: the outcomes are sequential (pause → refused send → release → stopped). The
  exits read side by side there: 0, 1, 2, 10, 13, 20, 21; `session-live` / `cwd-missing` / `--id` `no-session` in
  `tests/cli_revive.rs`; 11, 12, 14 cited by name.
- **The `revive --json` gap** — not built beyond the five codes (see **Spec claims disproved**).
- **Test oracles corrected in the fix-loop and the operator pass** (test code only, no product change):
  `cli_instance_state::run_rewrites_the_settings_override_each_start` compared an `absolute` home with the canonical
  `pinned_bin` (→ `resolved`); `resolved()` of a home not created yet (`path1_run_start_one_home_in_order`); the
  completeness case read `viola --help`'s command descriptions as options (`--file`) (→ the `Options:` section only);
  the link case's `cfg(unix)`-only imports failed the Windows-target lint (→ moved inside it);
  `canonical_home_resolves_an_existing_home_and_keeps_a_new_one_absolute` took Windows' raw `\\?\` canonical form as its
  oracle (CI `ci#38998933306` `test (windows-2025)`, → `plain_drive`; the fix commit `f738cbc`).
- scope record (`gate.py scope` at this wrap: `scope: clean — changed 22 · listed 21 · recorded 1 (companion 0 ·
  mechanical 0 · in-intent 1 · widening 0) · absorbed 0 · excluded 62`):
  - in-intent: `tests/support/outer_pty.rs` · serves step 8 · self.

## Decisions & corrections
- The operator's invocation of implement (2026-10-17, in this session): "No live claude start in this chunk. The
  folded CI red goes first: name its cause at a file:line or stop and report after the plan's forced runs — never
  raise a timeout, retry or serialise to get green. Run each tool bare and read it as printed (the G1 guard). Journal
  each long run before you wait. Run the operator pass with the ci.py conclusion read (leg=operator); stop ready for
  the wrap." — followed: no live start, the cause at file:line, a journal per long run
  (`.andromeda/runs/2026-10-17T02-50-26-implement/journal.md`), the operator pass run.
- The operator's invocation of this wrap: "Run P1 only, then stop (the two-window form: 58.9 % with a several-master
  reconcile ahead). … Name in the report the gap you found: revive --json after a passed preflight
  (batch-script-child, pinned-hash-mismatch, squatted-name, internal-error) prints no document, against v1-39 and the
  plan — the wrap decides its owner at P2/P7." — named in **Spec claims disproved**; the owner is P2/P7's.
- Curation candidate — **a capture-then-write file is not a capture until its bytes land**: a waiter that tests a
  file's NAME over a writer that claims the name empty and renames bytes over it after a sync reads an empty file on a
  slow disk; test for content (non-empty), never for the name. Evidence: evidence/ci-red.md §2, §5;
  `ci#38994749835` (the red), the unit witness both ways.
- Curation candidate — **a timing window a host cannot open is forced at the decision**: the plan's 20-firing
  concurrent start stayed green on the tmpfs home base (memory-backed, a sync costs nothing); planting the window's
  state (an empty claim) in a unit case gave the red-before / green-after / red-reverted reading. Recurrence of
  testing.md 2026-09-27 (forcing a timing window); evidence/ci-red.md §3-5.
- Curation candidate — **a test oracle for a canonical path on Windows takes the plain drive form**: `std::fs::canonicalize`
  returns `\\?\C:\…`; the product writes `C:\…`; an oracle built from raw `canonicalize` reds only on the Windows
  runner (`ci#38998933306`, job 117297543004, `src\cmd\mod.rs:347`). Pairs with testing.md 2026-10-03 (the dev host's
  Windows-target clippy sees a cfg-only import but runs no test).
- Curation candidate — **a test-started `viola` from a wrapped session strips `VIOLA_DIR` and `VIOLA_BIN` too**: once
  the CLI verbs read `VIOLA_DIR`, a child that keeps the builder session's `VIOLA_*` (this host: `VIOLA_NAME=viola-builder`,
  `VIOLA_DIR`, `VIOLA_BIN`) resolves the bridge's home; `UNWRAPPED` in `tests/support/home.rs` is the list. Extends
  testing.md 2026-10-07 (a product command started from the bridge-wrapped session).
- Curation candidate — **a path comparison against a test home compares the resolved home**: on the dev host
  `target/e2e-home` is a link to tmpfs, so `TestHome::path()` and the canonical spelling differ there and agree on CI;
  `cli_instance_state.rs:337` and the four canonical guard runs read red only through the link (evidence/guards.md).
- Curation candidate — **under a `VIOLA_NAME`, `viola list` (and now `revive --list`) creates the home it resolved**:
  the `cli` log opens there; an oracle "the default home was not made" is false with a name set (the first
  `home_resolution_…` run). The log's presence is the better witness of which home resolved.
- Curation candidate — **`viola --help`'s command descriptions hold `--file`**: a flag census reads only the lines
  after `Options:`.
- Sweep hazard — the `scanned` token: 8 of its 9 master sites are other facts (the board's "every scanned instance",
  the secret scan's "never scanned"); a `scanned` sweep is read per row.
- Sweep hazard — a sed or Edit anchored on pre-format text misses after rustfmt reflows a tuple over lines: the first
  remove-the-guard `sed` on the `SETTINGS` row matched nothing and its green proved nothing (evidence/guards.md) —
  recurrence of testing.md 2026-09-25's extension.
- Guard / hook refusals (implement + this wrap): a `cat` heredoc with a file target (re-issued as an Edit); a `cd`
  that would move the cwd ×2 (re-issued in a subshell); a foreground `sleep` chained to a check ×2 (waited on the
  completion notice); the stamp-ahead hook on a journal stamp 15 min ahead (corrected from `date -u`); `evolve.py`
  `REFUSED ts` on a record stamped 4 s ahead (re-fired with the clock's reading); `gate.py hygiene` refused a runner
  drive path quoted in `evidence/operator-pass.md` (respelt by hand, then clean); a TOML pattern written through
  `printf` landed a doubled backslash (fixed through Edit) — host-linux.md 2026-09-28's doubled-backslash rule,
  recurring through a shell append.
- Process note — implement wrote `v1-39` `implemented` with the gap SURFACED beside it rather than withholding the
  ref; P7.3 reads both.

## Outcome
- (tests, Part A) **MET in part / owner P7**: the cause is named at file:line (evidence/ci-red.md, **Spec claims
  disproved**); the unit witness reads red with the fix reverted and green with it; `stamped_home_concurrent_…` reads
  green over 20 firings with the fix (and 20 without — it cannot read red on this host, **Deviations**); CI on the
  final HEAD `f738cbc0f307` reads `verdict: green · checks 15/15`, `test (ubuntu-latest)` among them. No bound raised,
  no retry, no test serialised (the diff: `count_of`'s condition only). The criterion's "reads red on the tree with
  the fix reverted" is UNMET for the concurrent case as written — routed to P2 (an unlinked plan criterion).
- (arch) `viola revive --json`'s exit-1 documents for the five codes, `--list --json`, and the human pairs and rows
  byte-identical → MET for the preflight's refusals (`cli_revive.rs` `revive_json_*`, `revive_list_json_*`, the
  unchanged human-pair cases green); **UNMET as written for "every exit-1 refusal"** — `run::start`'s own refusals
  after a passed preflight print no document (the gap) → `matrix#v1-39` → P7's post-claim-disproof escalation, its
  owner P2/P7's.
- (obs) `revive <name> --list` under a `VIOLA_NAME` writes one `process-start` and one `process-exit` into the
  caller's `cli-<name>.ndjson`; a refused check writes no line and leaves the mode → MET
  (`revive_list_under_a_name_logs_its_start_and_exit`, `revive_json_refuses_state_another_user_could_write_on_both_arms`).
  The file is named by the caller's `VIOLA_NAME` (**Deviations**).
- (obs) `state-recovered` under a `VIOLA_NAME` carries `instance` the caller and `scanned` the scanned name, valid
  against the schema → MET (`e5_list_reads_a_live_wrapper_through_the_replay`, `contract_diag_schema`).
- (arch, security) resolution order + canonicalisation (plain drive form on Windows) → MET (`resolve_home_*`,
  `canonical_home_*`, `plain_drive_*`, `cli_machine_contract.rs`'s three home cases; Windows leg green on `f738cbc0`).
- (tests) the completeness case → MET (`every_flag_variable_and_config_key_is_a_row_of_the_table`, its remove-the-guard
  run red; the `VIOLA_DIR` row over a 0770 home reads `list` exit 21; `CONFIG_KEYS` = the schema's keys).
- (tests, layouts, design) `tests/cli_machine_contract.rs` reads each driver-facing verb's documents and literal exits
  → MET for the outcomes listed in **Deviations** (step 16).
- `verification-matrix.json#v1-39` → `implemented` (ref: the cases above); its flip at P7.3 carries the gap.
- (tests) `pre-push` with the coverage gate → MET (green ×3: 74-76 s; `linux-tests` stage, the coverage gate read inside it).
- gates (implement's second whole-block firing, `.andromeda/runs/2026-10-17T02-50-26-implement/`, then the operator
  pass): `cargo fmt --all --check` green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`
  green · `CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc -- -D warnings` green (red
  at the first firing: cfg(unix)-only imports, fixed) · the G1 `rg` green (`exit 1`, `no output`) · `bash
  scripts/lint-probes.sh` green · `run --unit` green · `run --unit --filter 'test(/resolve_home_|role_of_|config_keys_|state_recovered_names_/)'`
  green (48 selected) · `run --integration --filter 'test(/stamped_home_concurrent_/)'` green (firings 1-42, every one) ·
  the binaries filter green (139) · `run --integration` green · `run --e2e` green (red at the first firing,
  `path1_run_start_one_home_in_order`, fixed) · `cleanup --session p-cli-smoke` green · `boot --session p-cli-smoke`
  green · `status --session p-cli-smoke` green (`state ready`, `last_error null`) · `g2-zero-panics.sh` green ·
  `schema-check` green · `cleanup` green (`processes_gone`, `endpoint_gone`) · `pre-push` green · `gate.py hygiene`
  (leg operator) — `hygiene: clean` before each commit (evidence/operator-pass.md) · `git push` (leg operator) —
  `history moved … f6bcce5b→e2485333`, then `e2485333→f738cbc0` · `ci.py conclusion --sha HEAD --wait 1800` (leg
  operator) — on `e2485333`: `verdict: red`, `failed 1: test (windows-2025)` (a test oracle, this chunk's, fixed by
  `f738cbc`); on `f738cbc0`: `verdict: green · checks 15/15 · runs ci#38999642102 completed/success`. No deferral.
- smoke: the boot path changed (`run`'s canonical home); the plan's smoke entries ran as P2 gates — boot, status
  ready, cleanup with processes and endpoint gone.
- watches: none folded.
- outcome basis: the operator pass ran (`e248533`, `f738cbc` — Setup 4's commit list); the gate verdicts rest on the
  final HEAD's CI run `ci#38999642102` (evidence/operator-pass.md) and the pass's `pre-push`/hygiene lines; implement's
  report `.andromeda/runs/2026-10-17T02-50-26-implement/outcome-2026-10-17-cli-machine-contract.md` (the one implement
  dir) and this window's conversation, which holds the implement run whole.
- process hygiene: the smoke session `p-cli-smoke` — started by a leg, terminated (entry 17 `processes_gone:true`,
  `endpoint_gone:true`); every test-started `viola` / fake agent — terminated (re-measured 2026-10-17T03:52:56Z: the
  only `viola` processes are 6 whose `/proc/<pid>/exe` is `viola-lab/prototype/target/debug/viola`, the overseer pair's
  build, not this run's); the host's `claude` processes — not this run's.
## Addendum — resumed 2026-10-17
- inputs#I3 · inputs/ · copy message · n/a — the Viola overseer's answers to this wrap's P2 escalation dialog (2026-10-17T04:05:25Z; `relay-1.md`): Q1 the CLI verbs' `VIOLA_DIR` home step and the canonicalisation are written as landed, NOT ratified, the founder's own word owed at his next batch; Q2 `matrix#v1-39` is un-claimed (back to the pool with a dated note naming the gap) and the post-preflight `revive --json` documents are a CARRY on the route entry "Exit-cause code catalogue", which then claims `v1-39` with the gap closed; Q3 `revive --json` on both arms and `--list`'s `cli` log are applied as routine (overseer, technical)
  - bears on **Spec claims disproved by measurement**: adds to it — the gap's owner is "Exit-cause code catalogue" (a CARRY), and `v1-39`'s disposition is un-claim, not a flip or a refine
  - bears on **Outcome**: supersedes «its flip at P7.3 carries the gap» — `v1-39` is un-claimed at P7.3, `chunk:null`, status kept
- inputs.py verify, re-fired: inputs: 3 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 3 · uncited 1 · unparsed 0; moved since the report: uncited 0 → 1 (I3, cited by this addendum)

## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: f6bcce5b (the parent of the oldest pre-CI commit e2485333) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### crates/viola-state/src/replay.rs — added 43 line(s) in 6 range(s)
added: 9 · 162 · 169 · 174-182 · 190 · 465-494
- 174-181 @176 «fn scanned_name(instance_dir: &Path) -> Option<&str> {»
  - 177-180 «instance_dir»
  - 465-493 @471 «fn state_recovered_names_the_scanned_instance(»
### schemas/config.v1.json — new file · 18 line(s)
- 1-18 «{»
  - 6-16 «"properties": {»
### schemas/diag-line.v1.json — added 2 line(s) in 1 range(s)
added: 84-85
### src/cmd/client.rs — added 6 line(s) in 1 range(s)
added: 120-125
- 120-124 @122 «pub(crate) fn refused_document(code: &str) {»
### src/cmd/hook.rs — added 3 line(s) in 3 range(s)
added: 26 · 31 · 548
### src/cmd/mod.rs — added 217 line(s) in 17 range(s)
added: 14 · 81 · 92-93 · 95-107 · 109 · 112 · 117 · 122 · 127 · 133 · 142 · 147 · 152 · 181-195 · 197-262 · 268
       275-383
- 181-187 @183 «pub(super) struct Instance {»
  - 198-200 «let instances = dir»
  - 201-204 «let home = instances»
- 208-219 @210 «fn resolved_home(flag: Option<PathBuf>) -> Result<PathBuf, Failure> {»
  - 211-216 «resolve_home(»
- 221-239 @224 «fn resolve_home(»
  - 230-232 «if let Some(home) = flag {»
  - 233-235 «if let Some(instance) = instance_of(name, dir) {»
  - 236-238 «user_home»
- 241-249 @244 «pub(super) fn canonical_home(home: &Path) -> PathBuf {»
  - 245-248 «std::fs::canonicalize(home)»
  - 253-261 «let plain = path.to_str().and_then(|text| {»
  - 276-278 «fn os(text: &str) -> Option<OsString> {»
  - 288-317 @295 «fn resolve_home_takes_the_flag_then_the_session_then_the_default(»
  - 319-329 @320 «fn resolve_home_needs_only_one_of_the_two_variables_missing_to_fall_through() {»
  - 331-339 @332 «fn resolve_home_with_no_user_home_and_no_session_is_an_error() {»
  - 341-356 @342 «fn canonical_home_resolves_an_existing_home_and_keeps_a_new_one_absolute() {»
  - 358-370 @360 «fn canonical_home_resolves_a_link_to_its_target() {»
  - 372-383 @381 «fn plain_drive_strips_only_the_verbatim_drive_prefix(#[case] given: &str, #[case] want: &str) {»
### src/cmd/revive.rs — added 76 line(s) in 20 range(s)
added: 11 · 17 · 34-37 · 101-108 · 114 · 119 · 175 · 177 · 183 · 235-238 · 240 · 244-246 · 250-251 · 254-263 · 265
       269-285 · 289-292 · 298 · 300-303 · 594-603
  - 244-246 «if let Some(own) = client::own_name() {»
- 269-275 @271 «struct ListRow {»
- 277-282 «impl ListRow {»
  - 278-281 @279 «fn line(&self) -> String {»
### src/cmd/run.rs — added 17 line(s) in 7 range(s)
added: 232-234 · 236 · 364-365 · 367-371 · 383 · 490-493 · 500
### src/cmd/verify/typed.rs — added 18 line(s) in 3 range(s)
added: 315-317 · 329 · 1099-1112
  - 1099-1111 @1102 «fn captured_reads_no_capture_while_the_claim_is_empty() {»
### src/main.rs — added 19 line(s) in 3 range(s)
added: 89-91 · 100-107 · 326-333
### src/obs.rs — added 20 line(s) in 1 range(s)
added: 335-354
  - 335-353 @338 «fn config_keys_are_the_config_schema_properties() {»
### tests/cli_controls_not_disableable.rs — added 254 line(s) in 8 range(s)
added: 3-9 · 23 · 348-352 · 354-364 · 366 · 368 · 373 · 385-611
- 386-413 @388 «const SETTINGS: [(&str, &str); 9] = [»
  - 389-392 «(»
  - 393-396 «(»
  - 398-401 «(»
  - 403-406 «(»
  - 407-410 «(»
- 418-420 «fn named(setting: &str) -> bool {»
- 422-434 @423 «fn long_flags(help: &str) -> Vec<String> {»
  - 424-433 «help.lines()»
- 436-468 @439 «fn product_variables() -> Vec<String> {»
  - 441-446 «for crate_dir in std::fs::read_dir(workspace_path("crates")).expect("crates") {»
  - 448-466 «for file in roots»
- 470-502 @473 «fn every_flag_variable_and_config_key_is_a_row_of_the_table() {»
  - 474-478 «let help = Command::new(VIOLA)»
  - 482-487 «for flag in &flags {»
  - 490-492 «for variable in &variables {»
  - 493-495 «let schema: Value = serde_json::from_str(»
  - 499-501 «for key in keys.keys() {»
- 504-540 @509 «fn viola_dir_does_not_disable_the_home_scan() {»
  - 515-517 «for path in [&home, &home.join("instances"), &dir] {»
  - 520-528 «let out = Command::new(VIOLA)»
  - 532-535 «assert_eq!(»
  - 536-539 «assert!(»
- 542-551 @549 «fn viola_variable_does_not_disable_the_send_paste_control(#[case] setting: (&str, &str)) {»
- 553-557 @554 «const FLOOR: [(&str, &str); 2] = [»
- 559-590 @561 «fn child_names_under(config: &str) -> Vec<String> {»
  - 567-576 «let args: Vec<OsString> = vec![»
  - 579-581 «let lines = support::fake::wait_for(&receipt, "env", |l| {»
  - 584-589 «support::fake::of_kind(&lines, "env")[0]["names"]»
- 602-611 «fn config_key_does_not_disable_the_identity_strip(#[case] config: &str) {»
  - 604-607 «assert!(»
  - 608-610 «for (name, _) in FLOOR {»
### tests/cli_instance_state.rs — added 9 line(s) in 3 range(s)
added: 22-23 · 325 · 418-423
  - 420-423 «assert!(»
### tests/cli_list.rs — added 6 line(s) in 1 range(s)
added: 252-257
### tests/cli_machine_contract.rs — new file · 583 line(s)
- 25-27 «use support::home::{»
- 32-40 @33 «enum Out {»
- 42-51 @44 «struct Row {»
- 53-92 @55 «fn viola_in(»
  - 63-65 «if let Some(home) = home {»
  - 67-69 «for name in UNWRAPPED {»
  - 71-73 «for (name, value) in env {»
  - 74-81 «command»
  - 83-90 «if let Some(text) = stdin {»
- 94-140 @95 «fn assert_row(home: &Path, bin: &Path, row: &Row) {»
  - 96-100 «let env: Vec<(&str, OsString)> = row»
  - 103-109 «assert_eq!(»
  - 110-139 «match &row.out {»
- 142-330 @145 «fn every_verb_prints_one_document_and_its_literal_exit_over_a_wrapper_s_life(»
  - 151-263 «let live: [Row; 13] = [»
  - 264-266 «for row in &live {»
  - 269-326 «let dead: [Row; 7] = [»
  - 327-329 «for row in &dead {»
- 332-367 @335 «fn verbs_over_a_home_with_no_instance_print_their_documents() {»
  - 338-363 «let rows = [»
  - 364-366 «for row in &rows {»
- 369-417 @373 «fn verbs_over_a_home_another_user_could_write_print_their_documents() {»
  - 379-381 «for path in [home, &home.join("instances"), &dir] {»
  - 385-410 «let rows = [»
  - 411-413 «for row in &rows {»
- 419-436 @420 «fn listed(home: Option<&Path>, env: &[(&str, OsString)], bin: &Path) -> Vec<String> {»
  - 422-428 «assert_eq!(»
  - 430-435 «doc["ok"]["items"]»
- 438-507 @442 «fn home_resolution_takes_the_flag_then_the_session_then_the_default(stamped_home: StampedHome) {»
  - 451-458 @452 «let session = |dir: &Path| -> Vec<(&str, OsString)> {»
  - 464-475 «for malformed in [another_name, relative] {»
  - 476-481 @477 «let named = |dir: &Path| -> Vec<(&str, OsString)> {»
  - 482-486 «let logged_as_builder = |home: &Path| {»
  - 487-491 «assert_eq!(»
  - 492-495 «assert!(»
  - 496-499 «assert!(»
  - 500-504 «assert_eq!(»
- 509-529 @512 «fn two_homes_are_isolated(stamped_home: StampedHome) {»
  - 524-527 «assert_eq!(»
- 531-583 @535 «fn a_home_reached_through_a_link_is_canonicalised() {»
  - 546-555 «let args: Vec<OsString> = vec![»
  - 558-565 «let mut pty = OuterPty::spawn_without(»
  - 566-568 «wait_for(&receipt, "the start receipt", |l| {»
  - 571-572 «let over_target =»
  - 574-577 «assert_ne!(»
### tests/cli_revive.rs — added 218 line(s) in 1 range(s)
added: 559-776
- 560-564 @562 «fn refused_document(code: &str) -> String {»
- 566-571 @567 «fn assert_refused_document(ran: &Ran, code: &str) {»
- 573-586 @575 «fn revive_json_on_a_live_name_is_the_already_live_document(booted_wrapper: Wrapper) {»
  - 578-582 «let ran = revive(»
- 588-594 @589 «fn revive_json_of_a_name_never_started_is_the_no_session_document(stamped_home: StampedHome) {»
- 596-609 @597 «fn revive_json_with_an_id_the_log_does_not_hold_is_the_no_session_document(»
  - 602-606 «let ran = revive(»
- 611-623 @612 «fn revive_json_of_a_session_open_in_another_claude_is_the_session_live_document(»
- 625-640 @626 «fn revive_json_of_a_snapshot_with_no_cwd_is_the_cwd_missing_document(stamped_home: StampedHome) {»
  - 631-636 «OpenOptions::new()»
- 642-668 @646 «fn revive_json_refuses_state_another_user_could_write_on_both_arms(stamped_home: StampedHome) {»
  - 651-667 «for args in [»
- 670-698 @672 «fn revive_list_json_prints_the_logged_chain(stamped_home: StampedHome) {»
  - 683-688 «let fields: Vec<&str> = sessions[0]»
- 700-706 @701 «fn revive_list_json_of_a_name_never_started_is_the_no_session_document(stamped_home: StampedHome) {»
- 708-711 @709 «fn cli_file(home: &Path) -> PathBuf {»
- 713-776 @717 «fn revive_list_under_a_name_logs_its_start_and_exit(stamped_home: StampedHome) {»
  - 720-726 «let listed_as = |args: &[&str], from: Option<&str>| {»
  - 738-747 «let own: Vec<(&str, &str)> = lines»
  - 748-756 «assert_eq!(»
  - 757-760 «let exits: Vec<&Value> = lines»
  - 768-770 «for line in &lines {»
  - 771-775 «assert_eq!(»
### tests/cli_verify.rs — added 47 line(s) in 1 range(s)
added: 1567-1613
- 1568-1613 @1572 «fn stamped_home_concurrent_verify_runs_each_stamp_the_home() {»
  - 1575-1592 «let runs: Vec<_> = (0..RUNS)»
  - 1594-1611 «for (n, run) in runs.into_iter().enumerate() {»
### tests/contract_diag_schema.rs — added 24 line(s) in 1 range(s)
added: 216-239
- 216-238 @224 «fn diag_line_schema_takes_a_state_recovered_line_with_and_without_scanned(»
  - 229-233 «let mut line = json!({»
  - 234-236 «if let Some(scanned) = scanned {»
### tests/support/cli.rs — added 2 line(s) in 1 range(s)
added: 114-115
### tests/support/home.rs — added 27 line(s) in 4 range(s)
added: 21-24 · 315-331 · 598-600 · 776-778
- 315-330 @319 «pub fn resolved(home: &Path) -> PathBuf {»
  - 320-323 «if !home.exists() {»
  - 326-328 «let plain = text»
### tests/support/outer_pty.rs — added 13 line(s) in 2 range(s)
added: 54-65 · 75
### tests/support/start.rs — added 3 line(s) in 2 range(s)
added: 52-53 · 55
