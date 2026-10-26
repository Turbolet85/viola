# Report — 2026-10-26-explicit-session-links

**Chunk:** the `viola link` / `unlink` verbs and their channel methods, in the form the founder names for the boundary
widening
**Date:** 2026-10-26
**Commits:** `8fc3d1df` chore(records): remove six misnamed listing dirs from the tokio wrap's run dir · `743057f9`
chore(2026-10-26-explicit-session-links): operator pre-CI commit, for the run this chunk's verdict reads (basis:
`git log --format='%h %s' 8fc3d1df^..HEAD`; the chunk's code is all in `743057f9`, no fix commit above it)

## Changes (structured — detectors read this)
- **Files:** new `src/cmd/link.rs`, `src/cmd/unlink.rs`, `tests/channel_links.rs`; modified `src/run/links.rs`,
  `src/cmd/run.rs`, `src/cmd/mod.rs`, `src/human/verbs.rs`, `src/human.rs`, `tests/channel_endpoint.rs`,
  `tests/cli_output_plain.rs`, `tests/security_negatives_server_identity.rs`, `tests/cli_links.rs`,
  `.config/nextest.toml` (13 files; basis `gate.py scope`: `changed 13 · listed 13 · recorded 0`). Chunk records:
  `viola-0.1.0/chunks/2026-10-26-explicit-session-links/` (scope, research, plan, inputs I1–I5, evidence),
  `viola-0.1.0/verification-matrix.json` (two `notes` lines, P5), `viola-0.1.0/working-route.md` (the take-up stamp).
- **Symbols / APIs:**
  - **Channel methods `link` and `unlink` are served** by the driven instance's wrapper (before: both answered
    `-32601`). `LinkSet::link(&self, params)` (`src/run/links.rs:70-76`) and `LinkSet::unlink(&self, params)`
    (`src/run/links.rs:78-83`) share `driver_of` (`src/run/links.rs:85-99`): `params.driver` and `params.driven` read
    as strings through `ViolaName::try_new`; a missing, non-string or invalid name, a `driven` that is not the
    serving wrapper's own name, and a `driver` equal to `driven` are each `ProtocolError::InvalidParams` (`-32602`,
    `data: null`), no line written. Neither method reads `from` (a `from` of any shape changes nothing). `link` calls
    `record(driver)` and answers `{"ok":{"since":<the link line's ts>,"already_linked":<bool>}}` — a second `link` of a
    held pair appends nothing and answers the same `since` with `already_linked: true`; `unlink` calls
    `remove(driver)` and answers `{"ok":{}}` whether or not a pair was removed (an absent pair appends nothing). A
    `record` / `remove` error is `ProtocolError::Internal` (`-32603`). Each answers after its line and the snapshot
    write. Neither opens a span nor writes a process-log line; the wrapper's existing `channel-request` /
    `channel-response` pair carries `method:"link"|"unlink"`.
  - **`LinkSet::remove` has its first product caller** (the `unlink` method); its
    `#[cfg_attr(not(test), allow(dead_code))]` and its "not served yet" doc sentence are gone. A product path now
    appends an `unlink` event line (`source: wrapper`, `{driver, driven}`) — the first.
  - **`src/cmd/run.rs`**: `Methods` holds `links: Arc<LinkSet>` (handed the restored set at start, as `send` and
    `dialogs` already were); `dispatch_call` routes `"link"` and `"unlink"` to it (`src/cmd/run.rs:144-145`); the
    `Methods` doc names the served set — `send`, `wait`, `last`, `hook.dialog`, `answer`, `pause`, `release`, `link`,
    `unlink`, and the `hook.event` notification — nine methods (`src/cmd/run.rs:68-70`). New unit case
    `methods_route_link_and_unlink_to_the_link_set` (`src/cmd/run.rs:851-870`).
  - **New verbs `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]`**
    (`src/cmd/link.rs`, `src/cmd/unlink.rs`; `LinkArgs` `src/cmd/link.rs:21-32` shared by both). The verb's instance
    is the driven one. The shared call `link_call` (`src/cmd/link.rs:85-136`), in order:
    1. `viola_state::strict::check_instance(<absolute home>, <home>/instances/<driven>)`
       (`src/cmd/link.rs:98-106`) BEFORE `client::start` — so before any role file or log opens and before any
       snapshot value is read (the founder's form, `inputs#I3`); refused → exit 21, human
       `human::instance_state_pair(driven)` through `human::refuse` (the fixed instance-state `unable` / `hint:` pair),
       `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty; no line in
       any log, the directory's mode as found;
    2. `client::start(home, &driven)` (the `cli-<driven>` role file, `process-start`);
    3. the liveness-only pre-check `live_endpoint` (`src/cmd/link.rs:113-115`) — none: exit 21 through
       `client::unreachable` / `client::unable_unreachable` (the not-running hint last);
    4. params `{driver, driven}` plus `from` from `client::own_name()` when `VIOLA_NAME` is set;
    5. one request through `cmd::client::connect` (the server identity check: peer pid and its start time against the
       snapshot's `pid` / `started_at`, Linux and Windows; macOS the recorded gap) under
       `#[instrument(skip_all, name = "cli.link")]` / `name = "cli.unlink"`; a server that is not the recorded wrapper
       gets no frame, exit 21 `instance-unreachable` with the server-verify hint, `process-exit{… exit_code:21,
       detail:"server-verify-failed", during:"connect"}`;
    6. the reply through `client::answer_of`: a `link` `ok` with a string `since` and a boolean `already_linked`, or
       an `unlink` `ok` that is an object → the result line (or `{"v":1,"ok":<ok>}` under `--json`), exit 0; a wrapper
       error (`-32602` included, e.g. `driver` equal to `driven`) → `client::fault`, exit 20; `Dead` /
       `ServerUnverified` → exit 21; a `Refused` or an `ok` of another shape → internal error, exit 1.
  - **Three result lines** (`src/human/verbs.rs:57-89`, re-exported `src/human.rs:25-27`): `write_linked`,
    `write_already_linked`, `write_unlinked`, each one `write_all`, plain in every mode:
    `link    <driver> -> <driven>  since <HH:MM:SSZ>` · `link    <driver> -> <driven>  already linked` ·
    `unlink  <driver> -> <driven>`. The time is the reply's `since` as UTC clock time in whole seconds with `Z`
    (`link_since`, `src/human/verbs.rs:82-89`); a `since` that is no RFC 3339 instant (the restore's `"unknown"`)
    prints through `escape_message`. Names printed are the verb's own parsed `ViolaName` arguments, never a name read
    back from the reply or the log.
  - **`viola --help`** gains the row `handoff:  link, unlink` between `wheel:` and `setup:`; `Command` gains `Link` /
    `Unlink` ("Link a driver to the driven instance: its wrapper records the pair", "Remove a driver's link to the
    driven instance"), dispatched with `resolved_home` and `cli_sink(&home, &args.driven)` (`src/cmd/mod.rs:79-82`,
    `180-189`). `role_of` already read both words as `Role::Cli` (no change).
  - **Remaining-caller facts:** `check_instance(` under `src/` (outside tests): 2 → 3 product call sites
    (`src/cmd/hook/statusline.rs:45`, `src/cmd/revive.rs:137`, + `src/cmd/link.rs:98`, the one site both verbs share;
    basis `grep -rn 'check_instance(' src` at the work tree and `git grep` at `8fc3d1df`). `cmd::client::connect(`:
    6 → 8 callers (+ `src/cmd/link.rs:140`, `src/cmd/unlink.rs:21`). `live_endpoint(` product callers under `src/`:
    5 → 6 (+ `src/cmd/link.rs:113`), plus `crates/viola-mcp/src/tools.rs:334`. `human::refuse(` census: 14 → 15
    (`link.rs` 1; basis `grep -rcE 'human::refuse\(' src`: verify 5 · run 5 · list 2 · revive 1 · plugin 1 · link 1).
  - **The CLI frame verbs are now eight**: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink` —
    each takes the liveness pre-check and the server check through `cmd::client::connect` before its frame, and each
    writes `server-verify-failed` on a mismatch. Only `link` / `unlink` also run `check_instance` first.
- **Crates / modules:** root bin gains modules `cmd::link`, `cmd::unlink`; no crate added, removed or changed.
- **Dependencies:** none.
- **Schema / config:** no config key, schema or scrub shape. The product `unlink` event line (kind already in the
  closed set, `data: {driver, driven}`, `source: wrapper`) is now written. `.config/nextest.toml:24`:
  `binary(channel_links)` joins the `ci` profile's 20 s stamped-home kill list.
- **Spec-master edits:** none (no master edited by this chunk; the masters move at this wrap's P2).
- **Retired:** the hold itself — "the explicit `viola link` / `viola unlink` are not built / held for the founder's word
  on the boundary widening" (the working-route CARRY text on this entry, and every master line stating the hold);
  the `-32601` answer for `link` / `unlink`; `LinkSet::remove`'s `dead_code` allow; `link` / `unlink` as the unserved
  example of `channel_wrapper_logs_each_call_with_corr_and_conn` (now `anything`). No census row (no `census.md`).
- **Counts / qualifiers moved:**
  - Served channel methods: seven + `hook.event` → nine (`link`, `unlink` added) — pattern `methods`: control UNFIRED
    (`methods · control UNFIRED over the seven masters (bodies and sets) at 743057f9`): no master states the served
    count by these words; the served list's other wordings are under `notserved` and `linkverbs`.
  - CLI frame verbs: six → eight — pattern `frameverbs`: standing 20 · stale 6 (the six enumerated:
    `architecture-detail/occupied-resources.md:14`, `security-plan-detail/threat-model-summary.md:55`,
    `test-plan-detail/test-scope-summary.md:12`, `obs-plan-detail/log-coverage.md:29`,
    `obs-plan-detail/span-trace-coverage.md:28`, `a11y-plan-detail/cognitive-accessibility.md:26`) · 14 true claim
    sharing the token (a generic "CLI frame verbs" / "every frame-writing client" that now includes the two verbs by
    class). With `srvverify` (standing 33): stale 2 (`design-system-detail/surface.md:73` lists the six writers of
    `server-verify-failed`; `obs-plan-detail/log-coverage.md:29`, counted above) · 31 true (the code's other writers
    and its shape).
  - `check_instance(` product call sites 2 → 3 — pattern `checkinst`: standing 39 · stale 2
    (`architecture-detail/occupied-resources.md:106` "these two product call sites"; the strict-modes entry-point list
    `security-plan-detail/authentication-authorization.md:34`, which names `run`, `revive`, `list`, `hook statusline`,
    … and not the two verbs) · 37 true claim sharing the token (another verb's check: `revive`, `hook statusline`,
    `list`, `plugin install`; a detector extends a list of refusing verbs where it finds one).
  - `human::refuse` census 14 → 15 — pattern `refuse`: standing 1 · stale 1 (`obs-plan.md:311`: "run (5 sites),
    verify (5), list (2), revive (1) and plugin install (1)").
  - `cmd::client::connect` callers 6 → 8 — pattern `connect`: standing 4 · stale 1
    (`obs-plan-detail/span-trace-coverage.md:28`, the six frame verbs inside their dispatch span; also under
    `frameverbs`) · 3 true (`security-plan-detail/authentication-authorization.md:25`, `:26`, `:28`: other verbs'
    dated gaps).
  - `live_endpoint` readers +1 — pattern `liveep`: standing 16 · 0 stale · 16 true (each describes the read and its
    readers by class — "the CLI frame verbs").
  - `LinkSet::remove` callers 0 → 1 product (the `unlink` method) — pattern `remove`: standing 2 · stale 2
    (`architecture.md:220` "No product path appends `unlink` yet: its one writer, `LinkSet::remove`, has no caller
    until the `unlink` method is served"; `architecture-detail/established-decisions.md:150`).
  - `-32602` causes behind `viola.compat.v_rejected`'s over-count +2 (a bad `link` / `unlink` name or pair) —
    pattern `vrejected`: standing 2 · stale 2 (`obs-plan.md:418`, `obs-plan-detail/metric-coverage.md:28`: "also
    answers `send`'s invalid params and `release` carrying `from`").
  - `tests/security_negatives_server_identity.rs`: 19 → 25 cases (six frame verbs × two modes + control → eight × two +
    the controls) — pattern `idtable`: standing 14 · stale 2 (`test-plan-detail/test-scope-summary.md:12`,
    `a11y-plan-detail/cognitive-accessibility.md:26`, each naming "the six" verbs) · 12 true (the file named for its
    other content).
  - Sweep reach: 18 patterns · 11 moved facts, 11 with a pattern (the report's own count) · owner pattern 10 standing ·
    by line 4 lines · 5 offsets — every by-line pair (`security-plan-detail/authentication-authorization.md:25` @c933
    connect, @c1044 liveep, @c1985 connect; `:27` @c813 liveep, @c1256 frameverbs, @c1394 / @c1593 srvverify;
    `security-plan-detail/input-validation.md:52` @c396 liveep, @c432 frameverbs;
    `registries/contracts/test-plan/5-command-implementation.md:35` @c3505 / @c3764 checkinst;
    `registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10` @c3704 / @c4089 / @c4492
    killline) read `true` (another verb's gap, another test's kill line); the generic `owner` phrase's control did
    not fire and was dropped.
  - Owner pattern `title` ("Explicit session links", standing 10) — every row stale (each names this entry as the
    pending owner of the hold or of the `handoff:` row): `architecture-detail/established-decisions.md:150`,
    `design-system.md:348`, `design-system-detail/surface.md:136`, `layout-templates.md:61`, `layout-templates.md:219`,
    `test-plan.md:570`, `test-plan-detail/e2e-test-strategy.md:226`, `a11y-plan.md:104`,
    `a11y-plan-detail/a11y-scope-summary.md:76`, `a11y-plan-detail/aria-patterns-roles.md:10`.
- **Dev-tool versions:** none — no host tool installed, upgraded or read changed.
- **Harness / gate surface:** `.config/nextest.toml` `ci` profile kill-line filter gains `binary(channel_links)`;
  no agent-run verb, CI step or status shape changed.
- **Cross-project / external claims:**
  - CI run `ci#40347680974` on `743057f925ffe7a22694fcd1435c625896b31d65` (push, `run_attempt` 1): `verdict: green ·
    checks 15/15 · wall 941 s` (`evidence/operator-pass.md` §5); the per-OS PASS witnesses: ubuntu 77, windows 76
    (every case but the `cfg(unix)` one), macOS 69 (77 less the six identity rows and two span cases compiled out).
  - `inputs.py verify` (this wrap): `inputs: 5 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
    unreachable 0 · n/a 3 · uncited 2 · unparsed 0`:
    - I1 · message (operator, 2026-10-26T12:53Z) · copy · n/a — cited scope.md:9, research.md:7
    - I2 · `../additional/viola-overseer/phase-session-links-directive.md` · copy no-repo · unchanged
    - I3 · message (the founder, live, 14:41Z, relayed verbatim) · copy · n/a — «Сначала instance check
      (Рекомендую)»: cited plan.md:9, :21, :398, :447, :478
    - I4 · message (the founder, live, 14:57Z, relayed verbatim) · copy · n/a — cited here: inputs#I4, the P5 word
      «Да (Рекомендую)» on the card `.andromeda/runs/2026-10-26T12-53-39-phase/p5-card-1.md` (the newest and only
      card; none superseded). Its `## Beyond the plan` rows, verbatim, the founder's own word (relayed verbatim):

      | # | proposal | owner |
      |---|---|---|
      | 1 | `v1-13` and `v1-36` each need the founder's disposition of the clause no chunk can prove (the MCP result; the board showing links): a deferral with its reason, or the acceptance re-worded. Their CLI halves are built by this chunk. | a CARRY onto the existing working-route entry "Version done-check" (Epoch 8), which judges every cap `verified` or `deferred` |
    - I5 · `../additional/viola-overseer/wrap-session-links-directive.md` · copy no-repo · unchanged — cited here:
      inputs#I5, the operator's wrap directive (forced sample at P2; keep every transcript and the three run dirs; stop
      at a hard cascade failure or agents doing the orchestrator's work; the P2-validated report items; carry the
      card's Beyond-the-plan row as approved; run the wrap whole).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - The plan's Provenance claims `check_instance` "3 product callers → 4"; measured 2 → 3 under the master's own rule
    (`check_instance(` under `src/`: `src/cmd/hook/statusline.rs:45`, `src/cmd/revive.rs:137` at `8fc3d1df`, +
    `src/cmd/link.rs:98`; `list`'s check runs inside `viola_state::board::scan`, outside `src/`). The master
    (`architecture-detail/occupied-resources.md:106`, "these two product call sites") was right; it moves to three.
    Pattern `checkinst`. Plan-only claim; no master states three.
  - The plan's step 6 assumed the conn-less response line of an unserved method logs its `method`; measured: the
    channel names only the closed method list (`crates/viola-channel/src/server.rs` `method_label`), so `anything`
    logs no `method` key (implement deviation 2; the test asserts the key's absence). Plan-only claim; no master site
    — the masters' Wrapper-channel text names no unserved method's `method` value (pattern `methods`, control
    unfired).
- **Expected amendments (from plan):**
  - security-plan §Security Anti-Patterns / Decisions Log — the served `link` / `unlink` methods mapped to the IPC
    controls; the strict-modes entry-point list, the frame ban and the CLI frame-verb refusal row naming
    `viola link` / `viola unlink` under the instance check, not a dated gap (founder's word `inputs#I3`) — carried:
    Symbols / APIs (new verbs, steps 1 and 5) and Counts (`checkinst` stale
    `security-plan-detail/authentication-authorization.md:34`). Search: `checkinst` security 6 standing;
    `frameverbs` security 4 (`threat-model-summary.md:55` stale); `linkverbs` security 4
    (`security-plan.md:328`, `:448`, `input-validation.md:17`, `:50`: the params' validation, true); `idtable`
    security 1 (`authentication-authorization.md:73`, true).
  - architecture §Established Decisions [Links] (the explicit verbs and methods built in the instance-check form,
    `{since, already_linked}`, `LinkSet::remove`'s caller); §Standard Contracts Channel methods (served, their
    `-32602` causes, the client server-check list), Event data (a product `unlink` line); §Conventions CLI exit codes
    (exit 21 for the two verbs' server mismatch and strict-modes refusal); occupied-resources (checking processes, the
    `cli-<name>` producers, the `check_instance(` count) — carried: Symbols / APIs, Counts. Search: `linkverbs`
    architecture 6 (`architecture.md:51` stale, `architecture-detail/established-decisions.md:149` true, `:150`
    stale, `architecture.md:164`, `:212`, `architecture-detail/standard-contracts.md:41` true); `notserved` 1
    (`architecture.md:193` stale); `remove` 2 stale; `checkinst` architecture 8 (`occupied-resources.md:106` stale);
    `frameverbs` architecture 9 (`occupied-resources.md:14` stale); `title` architecture 1 stale; `srvverify`
    architecture 4 true; `instpair` 1 (`registries/contracts/architecture/project-directory-structure.md:23`, true).
    The exit-21 conventions row has no pattern here — `no pattern fired for it`; the architecture detector reads
    §Conventions whole.
  - design-system §Surface: cli — the server-verify hint's verb list and the exit-21 row (the verbs' strict-modes
    refusal takes the fixed instance-state pair), the help row built (the pending note retired); layout-templates
    §Output structure — the `handoff:` row built, the refusal block's verb lists — carried: Symbols / APIs (result
    lines, help row). Search: `handoff` design 2 stale (`design-system.md:348`, `surface.md:136`), layout 3
    (`layout-templates.md:61`, `:219` stale, `:215` the wireframe row, true); `already` design 1, layout 1 (true: the
    lines as built); `srvverify` design 1 stale (`surface.md:73`); `linkverbs` design 4 · layout 2 (true:
    the tape lines and `layout-templates.md:322` "visible only as `link` / `unlink` verb output").
  - test-plan §6 Scenario E3 (steps 1 and 3 landed; the driver is a self-reported name and is not booted), Security
    control negatives (raw-channel name negative landed; the identity table's eight verbs), §5 CLI and Wrapper channel
    (the help pin, `channel_links` on the kill line), §7 (the link line's `since` read outside the clock seam,
    `src/run/links.rs:92`) — carried: Symbols / APIs, Counts, Harness. Search: `e3` tests 4 (`test-plan.md:556`,
    `e2e-test-strategy.md:222` headings — label; `test-strategy.md:69`, `5-command-implementation.md:26` true);
    `title` tests 2 stale (`test-plan.md:570`, `e2e-test-strategy.md:226`); `linkverbs` tests 12
    (`e2e-test-strategy.md:234` stale "no `viola link` verb exists"; `test-plan.md:606`, `e2e-test-strategy.md:298`
    the raw-channel negative's text, now built; others true); `idtable` tests 10 (`test-scope-summary.md:12` stale);
    `killline` tests 5 (`e2e-test-strategy.md:226` stale, 4 true — no master lists the kill-line's binaries:
    `binary(channel_links)` 0 rows). The clock-seam fact: no pattern — the test-plan detector reads §7.
  - obs-plan §4 Edge flows E3 (explicit methods served, the product `unlink` line), the cli server check's verb list
    and CLIENT spans (`cli.link`, `cli.unlink`), §6 the "six CLI frame verbs" of `server-verify-failed`, the Vector 7
    record and §10 exemption 7 / the pre-init window (the verbs' refused check), §3 / §11 the `human::refuse` census,
    §5 the two new `-32602` causes in `v_rejected`'s over-count — carried: Symbols / APIs, Counts. Search: `e3` obs 6
    (`obs-plan.md:392`, `span-trace-coverage.md:177` stale; `obs-plan.md:183`, `:387`, `span-trace-coverage.md:68`,
    `:112` true); `spans` obs 2 (`obs-plan.md:293`, `span-trace-coverage.md:18`: `cli.<verb>` names the rule, true —
    the detector judges whether `cli.link` / `cli.unlink` join a list); `frameverbs` obs 3 (`log-coverage.md:29`,
    `span-trace-coverage.md:28` stale); `refuse` obs 1 stale; `vrejected` obs 2 stale; `checkinst` obs 20 (the
    refusing-verb enumerations are the obs detector's to extend: `obs-scope-summary.md:370`, `:397`,
    `slo-invariants-telemetry-budgets.md:17`, `telemetry-strategy.md:20` name `revive` / `statusline` /
    `list` checks; read true for those verbs). Vector 7 / exemption 7: no pattern fired for them by these tokens —
    the obs detector reads §10.
  - a11y-plan §1 the CLI verbs entity, §8 Error recovery (the verbs' server-verify and strict-modes refusals), §8 Plain
    language (the result lines among the literal-pinned strings), §4 the `handoff:` row built — carried: Symbols /
    APIs. Search: `handoff` a11y 3 stale (`a11y-plan.md:104`, `a11y-scope-summary.md:76`, `aria-patterns-roles.md:10`);
    `frameverbs` / `idtable` a11y (`cognitive-accessibility.md:26` stale "the six frame verbs' exit-21 server-verify
    refusal"; `:36` true).
  - `v1-13` / `v1-36` `notes` — written at the phase's P5 step 4 (`.andromeda/runs/2026-10-26T12-53-39-phase/
    note-v1-13.md`, `note-v1-36.md`; the matrix diff holds both): ledger-note — already written, no master site
    (a matrix entry); no P7.3 write owed. Their disposition is the card's Beyond-the-plan row (I4), owned by P5.
- **Coverage of new surfaces:**
  - `viola link` / `viola unlink` (CLI verbs) → validation names via `parse_name` (`ViolaName`) + `check_instance`
    before any log + liveness + server check✓ · instrumentation span `cli.link` / `cli.unlink`, `process-start` /
    `process-exit`, `server-verify-failed`✓ · PII n/a (names only; no driver name in the wrapper's process log —
    asserted) · tests unit (`link_reply_is_read_by_its_verbs_shape`, `link_line_is_built_from_the_names_and_the_reply`,
    two `request_refuses_an_unverified_server_inside_its_dispatch_span`) / integ (`tests/cli_links.rs` six new cases,
    `tests/security_negatives_server_identity.rs` six rows) · a11y plain ASCII, no SGR, hint last✓ · tokens n/a (no
    colour).
  - channel methods `link` / `unlink` → validation `ViolaName::try_new` + own-name + self-pair, `-32602`✓ ·
    instrumentation the wrapper's `channel-request` / `channel-response` pair✓ (no span by design, obs §4 E3) · PII
    n/a · tests unit (`links_set_method_*` 6 functions, `methods_route_link_and_unlink_to_the_link_set`) / integ
    (`channel_link_names_are_validated_by_the_wrapper`, E3) · a11y n/a · tokens n/a.
  - the three result lines and the `handoff:` help row → validation n/a · instrumentation n/a · PII n/a · tests unit
    (`link_verb_lines_are_the_design_lines_in_one_write`, `link_since_is_the_utc_time_alone`) / integ
    (`link_verbs_print_the_design_lines`, `help_is_the_grouped_verb_table`, `help_lists_only_served_verbs`) · a11y
    plain ASCII✓ · tokens n/a.

## Deviations from intent
- (implement 1) Step 4's whole-module macOS gate: only `link.rs`'s server-check case is gated off macOS (its module
  also holds the reply-shape table and the line case, which the plan says run there); `unlink.rs`'s module, whose one
  case is gated, is gated whole. Justification: testing.md 2026-10-26 asks a whole-module gate only where every case
  is gated.
- (implement 2) Step 6: `channel_wrapper_logs_each_call_with_corr_and_conn`'s assertion that the conn-less response
  line's `method` is `unlink` became that key's absence — the channel logs no `method` for an unserved name.
- (implement 3) `viola unlink` takes `link::LinkArgs`; the thin verb holds only its request under `cli.unlink`.
- (implement 4) Step 10's raw frames go through `viola_channel::Client::request` (adds `v` and `sender`, no server
  check), reaching the wrapper past every CLI check, as the step asks.
- (implement 5) Step 1's inline refusal table also carries the absent case for both params; the wire table keeps the
  step's list exactly.
- scope record: none — gate.py scope clean, 0 recorded (`scope: clean — changed 13 · listed 13 · recorded 0`).

## Decisions & corrections
- The founder's own word at P4 (`inputs#I3`), «Сначала instance check (Рекомендую)»: the wrapper serves `link` /
  `unlink`, and both verbs run `check_instance` before any log or snapshot read, then the liveness pre-check and the
  server check — the boundary widening ratified, no dated gap taken.
- The founder's own word at P5 (`inputs#I4`), «Да (Рекомендую)», on `p5-card-1.md`: its leans as written and its
  Beyond-the-plan row (`v1-13`, `v1-36` → a CARRY onto "Version done-check"); the operator's wrap directive
  (`inputs#I5`) carries that row as approved.
- The verbs' strict-modes refusal is exit 21 (as `list` / `plugin install`), not `revive`'s exit 1 — the founder's
  option text (card lean 5).
- The order trap held by a test: opening a role file narrows an existing home to 0700, so a check placed after
  `client::start` passes a 0770 home; `link_verbs_refuse_a_widened_instance_before_any_log` reads the mode after the
  refusal (remove-the-guard control A: 1 of 1 red when the check is moved).
- `driver` equal to `driven` is refused by the wrapper only (`-32602`, exit 20 at the CLI), never client-side — the
  operator's answer of chunk 2026-10-10-session-links (`I2-relay-2`), card lean 2.
- `LinkSet::line` stamps `since` with `Utc::now()` directly (`src/run/links.rs:92`), outside the clock seam; root
  cases assert `since` by shape only (`is_clock_time`).
- Count sweep hazard: the plan's research counted `check_instance` product callers as 3 by including `list`'s
  board scan (in `viola-state`), against the master's own rule (`check_instance(` under `src/`) which reads 2 —
  a call-site count must be taken by the rule its master states.
- Hazard: an unserved method's conn-less response line carries no `method` key (the closed `method_label` list); a
  test asserting a method value on it for a name outside the list is wrong by construction.
- Testing-rule refinement met (implement deviation 1): a `cfg(not(target_os = "macos"))` whole-module gate fits only
  a module whose every case is gated; otherwise gate the cases.
- Observation (implement, Surfaced): a `since` that is no RFC 3339 instant prints through `escape_message`, which keeps
  `\n`, `\t` and non-ASCII; the wrapper's own `since` is a line's `ts` or `unknown`, so such a byte reaches a line only
  from a hand-altered log.
- Guard refusals (implement): a `cat` heredoc with a scratchpad file target (re-issued through the Edit tool); a `cd`
  into the gate's log dir (re-issued with absolute paths).
- Standing learning met: testing.md 2026-09-25 remove-the-guard, each neutralising edit and each restore read back
  (`evidence/remove-the-guard.md`: A 1/1, B 4/18, C 6/8 red on exactly their guarded cases).
- The operator's word for implement: "please run implement per the skill on 2026-10-26-explicit-session-links,
  through its operator pass and the CI read. Journal each long run in the run dir before you wait on it."

## Outcome
- (arch) served `link` / `unlink` semantics, snapshot follows, restart restores — met:
  `links_set_method_*`, `e3_two_links_and_a_send_leave_one_link_record`, `links_an_unlink_survives_a_wrapper_restart`
  (cli_links 12 passed; CI green on three OSes).
- (security) raw bad-name / foreign-driven / self-pair frames → `-32602`, no line — met:
  `channel_link_names_are_validated_by_the_wrapper` (14 passed).
- (security) impostor server → no frame, exit 21 in both modes on Linux and Windows; control delivers — met: the two
  tables (25 passed; CI ubuntu and windows witnesses).
- (security) group-writable driven instance (Unix) → refused before any log, mode as found — met:
  `link_verbs_refuse_a_widened_instance_before_any_log` (ubuntu, macOS).
- (obs) refusal inside `cli.link` / `cli.unlink`, channel pair holds no names — met: the two
  `request_refuses_an_unverified_server_inside_its_dispatch_span` (Linux and Windows; compiled out on macOS, the
  recorded gap), `link_verbs_log_the_channel_pair_and_no_names`.
- (design) the three lines byte for byte, `--json` `{"v":1,"ok":…}` — met: the verbs.rs cases and
  `link_verbs_print_the_design_lines`; stopped instance → exit 21 not-running hint — met:
  `link_verbs_against_a_stopped_instance_exit_21`.
- (layouts) `handoff:  link, unlink` between `wheel:` and `setup:` — met: `help_is_the_grouped_verb_table`,
  `help_lists_only_served_verbs`.
- (tests) the unserved-method example reads `-32601` — met (with deviation 2).
- (tests) pre-push `"ok":true` at `linux-tests`; CI `verdict: green` — met (implement entry 23; ci#40347680974).
- No capability claimed — met: `claimed by … : 0` (implement's `matrix.py show`).
- Gates (implement's firing, trail `.andromeda/runs/2026-10-26T14-59-26-implement/gate-2026-10-26-explicit-session-links.json`:
  `entries 26 · green 22 · red 0 · recorded 1 · timeout 0 · not-run 3`): `df -h /tmp` recorded (tmpfs 32G, 6.8G used,
  25G avail) · `cargo fmt --all --check` green · `cargo clippy … -D warnings` green · Windows-target clippy green ·
  the G1 `rg` grep green (exit 1, no output) · `bash scripts/lint-probes.sh` green · orphans-check green ·
  `agent-run.sh run --unit` green (2495 passed) · nextest `binary(cli_links)` green (12) · `binary(channel_links)`
  green (14) · `binary(channel_endpoint)` green (5) · `binary(cli_output_plain)` green (6) ·
  `binary(security_negatives_server_identity)` green (25) · `cargo deny check` green · the preservation guard green
  (`guard: clean`, allowed 13) · cleanup / boot / status green · G2 green · G4 schema-check green · cleanup green
  (`processes_gone`, `endpoint_gone`) · `run --perf` green (7 passed; max 2.6 ms pre-tool-use) · `pre-push` green
  (3133 passed) · operator `gate.py hygiene` green (`hygiene: clean`, `evidence/operator-pass.md` §2) · operator push
  green (`8fc3d1df→743057f9`, §4) · operator CI read green (`743057f925ff verdict: green · checks 15/15`, §5). No
  `defer`, no void. Smoke: green (P3, session `p-esl-p3`, the verbs against a live wrapper).
- watches: none.
- Outcome basis: the operator pass ran — Setup 4's commit list is `743057f9` alone (no fix commit), and the final
  HEAD's CI run `ci#40347680974` is recorded in `evidence/operator-pass.md` §5; implement's
  `outcome-2026-10-26-explicit-session-links.md` (`.andromeda/runs/2026-10-26T14-59-26-implement/`, the only
  implement dir for this marker) is the basis for the gate counts, deviations and smoke; no operator directive moved
  code between implement and this report.
- Process hygiene: implement's census `census: 15 running`, none holding `projects/viola/target`; re-measured at this
  wrap (`gate.py census … --args projects/viola/target`, trail
  `.andromeda/runs/2026-10-26T15-35-13-wrap/gate-2026-10-26-explicit-session-links.json`, its census record):
  `census: 15 running`,
  none holding `projects/viola/target` — the overseer pair's bridge builds, the product `viola wait`, the host's
  `claude` sessions and this session's `viola mcp`: not this chunk's.
## New text, by line
Generated by `cites.py added` (cites v1.7); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 8fc3d1df (the parent of the oldest pre-CI commit 743057f9) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .config/nextest.toml — added 1 line(s) in 1 range(s)
added: 24
### src/cmd/link.rs — new file · 216 line(s)
- 21-32 @22 «pub(crate) struct LinkArgs {»
- 34-37 «pub(crate) fn link(home: &Path, args: &LinkArgs) -> anyhow::Result<ExitCode> {»
- 39-43 @40 «pub(super) enum Verb {»
- 45-54 @47 «fn checked(verb: Verb, ok: Value) -> anyhow::Result<Value> {»
  - 48-51 «let known = match verb {»
- 56-64 @58 «fn answered(verb: Verb, reply: Reply) -> anyhow::Result<Result<Value, Value>> {»
  - 59-63 «match reply {»
- 66-83 @68 «fn write_line(»
  - 75-82 «match (verb, ok["already_linked"].as_bool()) {»
- 85-136 @88 «pub(super) fn link_call(»
  - 98-106 «if check_instance(&absolute, &absolute.join("instances").join(driven)).is_err() {»
  - 108-112 «let unreachable = |cause| {»
  - 113-115 «let Some(live) = live_endpoint(&home.join("instances").join(driven)) else {»
  - 119-121 «if let Some(from) = own_name() {»
  - 122-128 «let ok = match client::answer_of(request(&live, params))? {»
  - 129-133 «if args.json {»
- 138-141 @139 «fn request(live: &LiveEndpoint, params: Map<String, Value>) -> Result<Value, CallError> {»
- 143-216 @144 «mod tests {»
  - 148-154 @152 «fn request_refuses_an_unverified_server_inside_its_dispatch_span() {»
  - 156-161 @157 «enum Read {»
  - 163-191 @180 «fn link_reply_is_read_by_its_verbs_shape(»
  - 193-197 «fn line(verb: Verb, ok: &Value) -> String {»
  - 199-215 @200 «fn link_line_is_built_from_the_names_and_the_reply() {»
### src/cmd/mod.rs — added 17 line(s) in 5 range(s)
added: 5 · 14 · 37 · 79-82 · 180-189
### src/cmd/run.rs — added 40 line(s) in 9 range(s)
added: 68-70 · 79 · 144-145 · 319 · 801 · 805-809 · 818 · 833-837 · 851-871
  - 851-870 @853 «fn methods_route_link_and_unlink_to_the_link_set() {»
### src/cmd/unlink.rs — new file · 37 line(s)
- 14-17 «pub(crate) fn unlink(home: &Path, args: &LinkArgs) -> anyhow::Result<ExitCode> {»
- 19-22 @20 «fn request(live: &LiveEndpoint, params: Map<String, Value>) -> Result<Value, CallError> {»
- 24-37 @26 «mod tests {»
  - 29-36 @32 «fn request_refuses_an_unverified_server_inside_its_dispatch_span() {»
### src/human.rs — added 3 line(s) in 1 range(s)
added: 25-27
### src/human/verbs.rs — added 71 line(s) in 3 range(s)
added: 1-2 · 57-90 · 168-202
- 57-66 @58 «pub(crate) fn write_linked(»
- 68-75 @69 «pub(crate) fn write_already_linked(»
- 77-80 @78 «pub(crate) fn write_unlinked(out: &mut impl Write, driver: &str, driven: &str) -> io::Result<()> {»
- 82-89 @84 «fn link_since(since: &str) -> String {»
  - 85-88 «match DateTime::parse_from_rfc3339(since) {»
  - 168-185 @169 «fn link_verb_lines_are_the_design_lines_in_one_write() {»
  - 187-201 @194 «fn link_since_is_the_utc_time_alone(#[case] since: &str, #[case] shown: &str) {»
### src/run/links.rs — added 183 line(s) in 5 range(s)
added: 5-6 · 13 · 70-100 · 102 · 380-527
  - 70-76 @72 «pub(crate) fn link(&self, params: &Value) -> Result<Value, ProtocolError> {»
  - 78-83 @79 «pub(crate) fn unlink(&self, params: &Value) -> Result<Value, ProtocolError> {»
  - 85-99 @87 «fn driver_of(&self, params: &Value) -> Result<ViolaName, ProtocolError> {»
  - 380-382 «fn params(driver: Value, driven: Value) -> Value {»
  - 384-386 «fn overseer_to_builder() -> Value {»
  - 388-414 @389 «fn links_set_method_link_twice_writes_one_line_and_answers_the_same_since() {»
  - 416-441 @417 «fn links_set_method_unlink_then_link_takes_a_later_since() {»
  - 443-454 @444 «fn links_set_method_unlink_of_an_absent_pair_writes_nothing() {»
  - 456-490 @475 «fn links_set_method_refuses_a_bad_pair_with_invalid_params(»
  - 492-512 @499 «fn links_set_method_a_from_changes_nothing(#[case] from: Value) {»
  - 514-526 @515 «fn links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged() {»
### tests/channel_endpoint.rs — added 6 line(s) in 3 range(s)
added: 217 · 223 · 249-252
  - 249-252 «assert!(»
### tests/channel_links.rs — new file · 88 line(s)
- 17-23 @18 «fn link_records(wrapper: &Wrapper) -> Vec<Value> {»
  - 19-22 «events(&wrapper.instance_dir())»
- 25-27 «fn snapshot_links(wrapper: &Wrapper) -> Value {»
- 29-37 «fn params(driver: &Value, driven: &Value) -> Map<String, Value> {»
  - 31-35 «for (key, value) in [("driver", driver), ("driven", driven)] {»
- 39-88 @54 «fn channel_link_names_are_validated_by_the_wrapper(»
  - 63-70 «for method in ["link", "unlink"] {»
  - 74-76 «let control = client»
  - 77-80 «assert_eq!(»
  - 83-86 «assert_eq!(»
### tests/cli_links.rs — added 308 line(s) in 2 range(s)
added: 6-9 · 285-588
- 288-295 @289 «fn link_verb(home: &Path, verb: &str, json: bool) -> Ran {»
  - 291-293 «if json {»
- 297-305 @298 «fn linked(home: &Path) -> Value {»
- 307-309 «fn unlinks(lines: &[Value]) -> Vec<&Value> {»
- 311-319 @312 «fn is_clock_time(text: &str) -> bool {»
  - 314-318 «b.len() == 9»
- 321-381 @326 «fn e3_two_links_and_a_send_leave_one_link_record(stamped_home: StampedHome) {»
  - 352-355 «assert_eq!(»
  - 365-368 «let (later_since, first_since) = (»
  - 369-372 «assert!(»
  - 375-378 «assert_eq!(»
- 383-409 @386 «fn link_verbs_print_the_design_lines(stamped_home: StampedHome) {»
  - 392-396 «let time = new»
  - 404-407 «for ran in [&new, &held, &unlinked] {»
- 411-418 @412 «fn is_clock_time_reads_the_shape_only() {»
- 420-451 @423 «fn link_verbs_against_a_stopped_instance_exit_21(stamped_home: StampedHome) {»
  - 428-450 «for verb in ["link", "unlink"] {»
- 453-459 @454 «fn channel_lines<'a>(lines: &'a [Value], event: &str, method: &str) -> Vec<&'a Value> {»
  - 455-458 «lines»
- 461-508 @465 «fn link_verbs_log_the_channel_pair_and_no_names(stamped_home: StampedHome) {»
  - 468-471 «for verb in ["link", "unlink"] {»
  - 473-474 «let run_lines =»
  - 475-492 «for method in ["link", "unlink"] {»
  - 494-495 «let cli_lines =»
  - 496-497 «let of =»
  - 501-505 «assert!(»
- 510-514 @511 «fn set_mode(path: &Path, mode: u32) {»
- 516-520 @517 «fn mode_of(path: &Path) -> u32 {»
- 522-564 @528 «fn link_verbs_refuse_a_widened_instance_before_any_log(stamped_home: StampedHome) {»
  - 534-539 «let requests = || {»
  - 543-557 «for verb in ["link", "unlink"] {»
- 566-588 @569 «fn links_an_unlink_survives_a_wrapper_restart(stamped_home: StampedHome) {»
### tests/cli_output_plain.rs — added 4 line(s) in 3 range(s)
added: 62 · 91-92 · 138
### tests/security_negatives_server_identity.rs — added 12 line(s) in 4 range(s)
added: 70-73 · 164-165 · 199-202 · 284-285
