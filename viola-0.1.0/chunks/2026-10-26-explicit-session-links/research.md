# Codebase Research — 2026-10-26-explicit-session-links

## Scope
- **Depth:** deep · **Reads:** 19 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole (18 Session Additions); applied: never pipe `boot` (2026-09-25), `schema-check` reads kept homes only (2026-10-11), `test(/…/)` vs `binary(<stem>)` selection (2026-09-27 ext. 2026-10-04), the stamped-home kill line by named list (`.config/nextest.toml:24`)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the founder's word «Взять, жду P4», relayed by the operator; `inputs#I2` — the operator's phase directive (forced sample, the P4 dialog, the standing `df -h /tmp` read)

## Files inspected
- `src/run/links.rs` (full) — `LinkSet { name, instance_dir, snapshots, set }`: `restore` (41), `current` (51), `record(driver) -> (since, already)` (58), `remove(driver) -> bool` (71, `#[cfg_attr(not(test), allow(dead_code))]` at 70, its doc "Called only by the `unlink` channel method, which is not served yet"), `change` appends then applies then `Snapshots::update` (98–104). Both methods take only the driver: the driven is always the set's own `name`.
- `src/cmd/run.rs` (60–146, 270–340, 778–830) — `Methods { name, instance_dir, send, wait, dialogs, wheel, budget }` (71–79) holds no `LinkSet`; `dispatch_call` (134–145) routes seven methods and falls through to `dispatch`, which answers every method but `hook.event` `-32601` (115–118). `start` builds `links` (281–286), `restore`s it before `server.serve` (287), hands it to `send` and `dialogs` (288, 303) and builds `Methods` without it (308–316). Unit cases build `Methods` at 778–809; `methods_answer_method_not_found_for_every_other_method` (821–830) calls `dispatch` (the notification path) with `link`, `unlink`, `anything`.
- `src/run/send.rs:142-146`, `src/run/dialog.rs:452-456` — the implicit link: `links.record(from)` when `from != name`, `-32603` (`ProtocolError::Internal`) if it cannot land.
- `src/cmd/client.rs` (full) — `start` (19: absolute home, `cli` role file opened, `process-start`), `connect` (127: `Client::connect` then `verify_server` on the snapshot's `pid` / `started_at`, `CallError::ServerUnverified` before any frame), `answer_of` (143), `unreachable` (197), `unable_unreachable` (217), `fault` (184, exit 20), `document` (173), `refused_document` (179), `own_name` (162), and the test helper `refused_inside::assert_refused_inside` (232–323, `#[cfg(test)] #[cfg(not(target_os = "macos"))]`).
- `src/cmd/pause.rs` (full) — the nearest verb shape: `client::start` → `wheel_call` (`live_endpoint` → `from` → `answer_of(request)` → `Ok` / `Fault` / unreachable; a `Refused` is an internal error) → one line or one document → `run::log_self_exit(0, None)`; `request` under `#[instrument(skip_all, name = "pause.client")]` (88); its test module gated `#[cfg(test)] #[cfg(not(target_os = "macos"))]` (93–94).
- `src/cmd/send.rs` (full) — `parse_name` (43) is the shared CLI name parser (Git Bash rewrite warning, `ViolaName::try_new`).
- `src/cmd/revive.rs` (100–138) — `trusted(home, name)`: `std::path::absolute(home)`, `home/instances/<name>`, `check_instance(&home, &instance_dir).is_ok()`, run BEFORE `open_wrapper_log` (118–122); the refusal is `instance_state_pair` (78) through `human::refuse`, or `client::refused_document("strict-modes-failed")` under `--json`.
- `src/cmd/list.rs:100-108`, `src/cmd/plugin.rs:56` — the exit-21 strict-modes refusals: `home_state_pair` / `instance_state_pair` through `human::refuse`, the `instance-unreachable` document under `--json`, no log opened.
- `crates/viola-state/src/strict.rs` (1–75) — `check_instance(home, instance_dir)`: the home, `instances/`, the instance dir, `snapshot.json`, `events.ndjson`, each that exists, a folder before what it holds; `Refused` carries fixed messages only.
- `src/cmd/mod.rs` (full) — `HELP_TEMPLATE` (29–38, rows `board:` · `traffic:` · `wheel:` · `setup:`), `Command` (57–90), `dispatch` (98–206, each frame verb `resolved_home` → `cli_sink(&home, &args.name)` → its module), `cli_sink` (210).
- `src/main.rs:94-119` — `role_of`: every first word but `hook`, `mcp`, `run`, `revive` (without `--list`) and a flag is `Role::Cli`, so `link` / `unlink` are `cli` with no change.
- `src/human.rs` (full), `src/human/verbs.rs` (25–60, 140–175) — the per-verb result writers live in `verbs.rs` and are re-exported from `human.rs` (23–27); `override_end` (28) is the time formatter with the escaped fallback (`escape_message`, 30).
- `src/human/hints.rs` (1–56) — `send_hint` causes `not-running` and `server-verify` (the exit-21 hints `unable_unreachable` prints); no strict-modes cause is built in this table.
- `src/human/board/pairs.rs` (1–17) — `instance_state_pair(name)` / `home_state_pair()`, the fixed strict-modes pairs.
- `tests/channel_endpoint.rs` (200–255) — `channel_wrapper_logs_each_call_with_corr_and_conn` uses `link` (`Map::new()` params) and a raw `unlink` frame against the booted wrapper as its examples of an unserved method, asserting `-32601` (218, 250).
- `tests/cli_output_plain.rs` (56, 135) — the `HELP` literal and the forbidden list `["hook", "mcp", "help", "link", "unlink", "plugin"]` of `help_lists_only_served_verbs`.
- `tests/security_negatives_server_identity.rs` (25–30, 60–70, 149–160, 189–192, 272–273) — the impostor `Dispatch` (`pause` / `release` answers), the `verb(name, json)` argv table, the two rstest tables over the six verbs.
- `tests/cli_links.rs` (29–141) — `boot`, `send`, `links`, `the_link`, `snapshot_links`, `linked_since`, `line_starts`, `assert_logs_clean`; the cases `links_a_send_from_another_instance_links_once`, `links_an_answer_from_another_instance_links`.
- `.config/nextest.toml:24` — the stamped-home kill-line filter, a named list of binaries (`binary(cli_links)` among them).
- `crates/viola-channel/src/server.rs:39-45` (`Call { method, params, id, conn, srv_conn }`), `:362-363` (`method_label` already admits `link` / `unlink`), `schemas/diag-line.v1.json:35,40` (`method` a free string, `error_code` enum holds `-32602`).
- `.andromeda/playbook.md:36-70` — the two "Boundary widening" rules and "the founder answered himself to a P4 fork".

## Graph impact (rust plane, regenerated: 6713 nodes / 37363 edges; trace `.andromeda/runs/2026-10-26T12-53-39-phase/tree-query-2026-10-26-explicit-session-links.json`)
- **`LinkSet::record`** — product callers 2: `run/send/send()` @ `src/run/send.rs:145`, `run/dialog/impl#[DialogSlot]answer()` @ `src/run/dialog.rs:455`; the rest are its own tests.
- **`LinkSet::remove`** — product callers 0; three call sites, all in `src/run/links.rs`'s own tests (302, 307, 317). Its first product caller is this chunk's `unlink` method, which ends the `dead_code` allow.
- **`LinkSet::restore` / `current`** — `cmd/run/start()` @ `src/cmd/run.rs:285` / `:328`; unchanged.
- **`check_instance`** — product callers 3: `board::scan` @ `crates/viola-state/src/board.rs:94`, `cmd/hook/statusline/trusted()` @ `src/cmd/hook/statusline.rs:45`, `cmd/revive/trusted()` @ `src/cmd/revive.rs:137` (re-derived: the `calls` query above, `callee_name = 'check_instance'`, test modules and `use` rows read out). The second P4 form adds two.
- **`live_endpoint`** — product callers 7: `answer`, `last`, `pause` (`wheel_call`), `send`, `wait` and `viola-mcp`'s `tools::frame()` @ `crates/viola-mcp/src/tools.rs:334` (re-derived: the same query, `callee_name = 'live_endpoint'`). Every P4 form that serves the verbs adds two.
- **`cmd::client::connect`** — callers 6, one `request()` per frame verb: `answer.rs:149`, `last.rs:62`, `pause.rs:90`, `release.rs:74`, `send.rs:253`, `wait.rs:90`. The verbs add two.

## Patterns detected
- **The wheel-verb client** (`src/cmd/pause.rs:49-86`): pre-check, `from`, one request, `Ok` → line or document, `Fault` → exit 20, unreachable → exit 21, a `Refused` an internal error; the verbs take this shape with their own `ok` check.
- **The instance check before the log** (`src/cmd/revive.rs:116-121`): `check_instance` on the absolute home before `client::start`/obs init, the refusal opening nothing and writing no line.
- **The served-method arm** (`src/cmd/run.rs:134-145`): one `match` arm per method into a component holding its own state; `-32602` is `ProtocolError::InvalidParams` (used at `src/run/dialog.rs:450`), `-32603` `ProtocolError::Internal`.
- **The server-check unit case** (`src/cmd/pause.rs:93-106`): `request_refuses_an_unverified_server_inside_its_dispatch_span` through `client::refused_inside`, the module gated by the two stacked attributes.

## Conventions to follow
- **Name parsing at the CLI edge**: `#[arg(value_parser = super::send::parse_name)]` (`src/cmd/pause.rs:24`).
- **One write per output line**, through a `human` writer taking `&mut impl Write` (`src/human/verbs.rs:43-50`), re-exported in `src/human.rs:24-27`.
- **The time-only `HH:MM:SSZ` form with the escaped fallback**: `override_end`'s parse and `escape_message` (`src/human/verbs.rs:28-39`), without its other-date branch (the wireframe's link line is time-only).
- **Two stacked test-module gates** for the server-check case (`src/cmd/pause.rs:93-94`; testing.md 2026-10-26).

## New files to create
- `src/cmd/link.rs` — `viola link <driver> <driven> [--json]`: the args, the verb, its `request` under a static span, its unit cases
- `src/cmd/unlink.rs` — `viola unlink <driver> <driven> [--json]`: the same shape
- `tests/channel_links.rs` — the raw-channel `link` / `unlink` name negative over a booted wrapper (test-plan §6 Security control negatives)

## Files to modify
- `src/run/links.rs` — the `link` / `unlink` method handlers over `record` / `remove`, the param checks, the `dead_code` allow removed
- `src/cmd/run.rs` — `Methods` holds the `LinkSet`; `dispatch_call` routes `link` / `unlink`; the unit cases' `Methods` builder and the not-found case
- `src/cmd/mod.rs` — the two modules, the two `Command` variants, their dispatch arms, the `handoff:  link, unlink` help row
- `src/human/verbs.rs` — the three result-line writers and their cases
- `src/human.rs` — their re-export
- `tests/cli_links.rs` — test-plan §6 Scenario E3 steps 1 and 3 and the verbs' output cases
- `tests/channel_endpoint.rs` — its unserved-method examples move off `link` / `unlink`
- `tests/cli_output_plain.rs` — the `HELP` literal and the forbidden list
- `tests/security_negatives_server_identity.rs` — the two verbs in both tables and the impostor's answers
- `.config/nextest.toml` — `binary(channel_links)` joins the stamped-home kill-line list

## Sweeps
- `"link"` / `"unlink"` as string literals over `src`, `crates`, `tests` (`*.rs`, `src/run/links.rs` left out): 52 hits · 5 changed (`src/cmd/run.rs:823`; `tests/channel_endpoint.rs:217`, `:223`, `:249`, the unserved-method examples; `tests/cli_output_plain.rs:135`) · 47 no-change (`symlink` test-fixture labels ×13; the event-kind and replay tables in `viola-core`, `viola-state` and their tests ×20; the implicit-link kind assertions `src/cmd/run.rs:1167`, `src/run/send.rs:705`, `:734`, `src/run/dialog.rs:1516`, `:1539` ×5; `viola-channel`'s `method_label` table and its case `server.rs:362`, `:363`, `:940`, and `tokio_client.rs:170`, whose server is the crate's own test `Dispatch`, not the wrapper ×4; the "never an MCP tool" guards `crates/viola-mcp/src/tools.rs:471`, `:640` and `crates/viola-e2e/tests/mcp_tools.rs:297`, which stay true ×3; the `tests/cli_links.rs:40`, `:46` kind filters ×2). Re-derived: `grep -rn '"link"\|"unlink"' src crates tests --include=*.rs`, then `src/run/links.rs` read out.
- `check_instance(` / `live_endpoint(` / `client::connect` call sites: the graph queries above.

## Open questions
- **The form of the widening** (the founder's, at P4, per `inputs#I2` item 5): which of the three recorded options, re-stated against HEAD → blocks: plan-decision (whether the verbs are built at all, and whether `src/cmd/link.rs` / `unlink.rs` run `check_instance` before `client::start`). The file lists above are the "verbs served" branch; under "no explicit verbs" none of them is written and the chunk plans nothing but its own close.
- **`v1-13`'s claim**: the cap's acceptance names "the machine-readable and MCP results"; no MCP tool returns a `link` or a `release` result by design ([MCP]: both CLI-only, `crates/viola-mcp/src/tools.rs:471`) → blocks: plan-decision (claim with that clause concretized, or leave pooled) — a P4 lean, previewed at P5.

## Scope premise closure
- "That is the word the hold names" — VERIFIED: `.andromeda/playbook.md:68-70` (the founder's own answer to a P4 fork, relayed verbatim).
- "`v1-13`… whether this chunk can prove the whole acceptance, the MCP half included" — VERIFIED with a narrowing: the `release --budget` half is built (`release` answers `{wheel, budget_paused, override_until?}`, `.claude/rules/api.md`), the MCP server exists, and no MCP tool carries `link` or `release` by design; the claim is a P4 lean.
- "Crossing (b) is no longer what the first option describes" — VERIFIED: `src/cmd/client.rs:127-139` and both master records complete (`.andromeda/master-route.md`).
- "Whether the second option can be taken for these two verbs alone" — VERIFIED: `check_instance` is a library function with three independent product callers (graph above); calling it in the two verbs before `client::start` takes none of "Home and code-bearing file integrity"'s work for the other eight clients, and moves no snapshot reader off `read_snapshot` (the founder's fork-1 ruling stands untouched).
- "No ledger row, no `viola verify` child…" — VERIFIED: the links code lives in `src/run/links.rs` and reads only viola's own params; no file of `crates/viola-agent-claude`, `fixtures/claude` or `src/cmd/verify` is on the lists.
- "No `link` / `unlink` MCP tool" — VERIFIED: architecture [MCP] (arch extract), `crates/viola-mcp/src/tools.rs:471`.
