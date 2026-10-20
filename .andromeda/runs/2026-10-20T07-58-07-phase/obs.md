# obs extract

## Relevance
partial — no product telemetry changes; the obs touchpoints are item (10)'s obs-plan §3 log-format key, item (5)'s code moves (spans and `obs_event!` sites travel with the code), item (8)'s killed `viola hook` profile writer, the mutation items (1, 3, 4) where obs code is graded like product code, and the test homes items (7) and (9) boot.

## Constraints
- Item (10): the §3 key "Log format JSON schema" (`.andromeda/registries/contracts/obs-plan/log-format-json-schema.md`) is marked **Binding**, a verbatim copy of test-plan §3 Log format ("obs aligns to tests, not vice versa"). Any reserved mark or note must leave the copy's fields and enums matching test-plan §3, and obs-plan "may add fields but must not rename or remove these" (per obs-plan §3 Log format JSON schema). The key file holds no "reserved" text at HEAD, and obs-plan.md has none in §3. Where the mark goes, and whether the §4 "GUI view: page load, sessions and links, live SSE tape" scenario heading (a body reading only "deferred to 0.2.0") is part of the same leftover, is research's question. The edit is wrap (cascade) work, never phase's (per obs-plan preamble, "Do not edit during phase loop runs").
- Item (5): moving `ledger.rs`'s test module out and splitting `src/cmd/verify/typed.rs` and `src/human/board.rs` must keep every span in the §4 form: `#[instrument(skip_all, name = "...", fields(...))]` with `skip_all` first, static `<area>.<operation>` names, and no `err` / `ret` (per obs-plan §4 Span naming convention).
- Item (5): moved code logs only through `obs_event!`, never through a raw `tracing::{event,info,warn,error,debug,trace}!`. A local print-macro `#[allow]` stays only on the output module that actually uses a print macro, so a split must not carry it to a file that does not (per obs-plan §11 Logs).
- Items (7) and (9): every integration or E2E home the chunk adds or changes (the recorded `tools/list` replay case, any faster replay, any headroom-checked stamped home) lives under `target/e2e-home/`, so G2, G4, the secret scan and the `diag-<os>` upload read it. The only carve-out is `tests/chaos_feed_panic.rs`'s `TestHome::outside_scan()`, which the founder ruled on (per obs-plan §9 Pipeline integration, Integration tests row).
- Item (8): `viola hook` must still exit 0 with empty stdout and stderr on every path, and every Cargo profile keeps `panic = "unwind"` (G3). A probe or fix for the truncated `.profraw` may not change either (per obs-plan §7 Panic hooks). Whether the killed profile writer also tears a home-level role-file line, which G4 skips and counts, is research's question (per obs-plan §9 Gate commands, G4).
- Items (1), (3) and (4): obs code (`obs_event!` call sites, `MillisUtc`, the panic hook) is graded under cargo-mutants like product code. A timed-out obs-code mutant, or a missed one the harness does not leave out as host-excluded, is red at that run (per obs-plan §9 Pipeline integration, Mutation row). Whether any of the 16 Informational survivors or the timed-out baselines sits in obs code is research's question.
- Items (1) to (3): a profile override or a faster replay in `.config/nextest.toml` must keep `[profile.ci.junit] path = "junit.xml"`, the `junit-<os>` artifact's source (per obs-plan §9 Telemetry artifact handling). It must also add no retry, since zero tolerance pairs `retries = 0` with the zero-unlogged-panics invariant (per obs-plan §10 Cross-input parallel).

## Patterns to follow
- Gate scripts fail closed with a `--probe` first. `scripts/g2-zero-panics.sh` refuses an empty scope, prints `tool-missing` when a tool is missing, and passes only on its stated exit. This is the model for item (9)'s headroom read before stamped-home gates (per obs-plan §9 Gate commands, G2).
- Every CI or gate telemetry verdict is a `jq` or shell assertion that sets an exit code, never a reading a human must judge. Item (9)'s headroom check and item (8)'s cause witness should leave a machine-checkable verdict (per obs-plan §11 Universal).
- Item (6): the verify summary count is "`MM` the relied count (`LedgerRow::relied()`, `ALL` filtered in its order)", 24 on Unix and 21 on Windows. `dialogless_home` deriving its summary from `LedgerRow::relied()` matches how the plan states that count (per obs-plan §4 Edge flows, `verify` / `plugin install`).
- Item (7): `mcp` stdout carries MCP frames only, and rmcp's own `tracing` target is OFF in the sink. A replayed `tools/list` exchange is checked on the frames, never on rmcp log records (per obs-plan §6 Per-module log levels).

## Anti-patterns to avoid
- NEVER use a bare `#[instrument]`, `#[instrument(err)]` or `#[instrument(ret)]` on moved or split code. Never put an instance name, dialog id or path in a span name (per obs-plan §11 Spans / Traces).
- NEVER add an `event` value or rename a tests field (`corr`, `event`, …) without a Decisions Log entry. This covers any diagnostic line item (8) might add and item (10)'s edit of the log-format copy (per obs-plan §11 Logs).
- NEVER add retry-once policies to make a slow or flaky baseline pass (per obs-plan §11 SLO).

## Contract bindings
- obs-plan §3 Log format JSON schema ↔ test-plan §3 Log format: a binding verbatim copy, where tests own the text and obs aligns (item 10).
- obs-plan §9 G2 / G4 / secret scan scope ↔ the tests-owned `target/e2e-home/` home layout and `AGENT_RUN_KEEP_HOMES=1` (items 7, 9).
- obs-plan §9 Mutation row ↔ test-plan §10 Mutation gate (items 1, 3, 4).
- obs-plan §4 Edge flows, the verify relied count ↔ design-system §Streams, the step-line `[NN/MM]` and summary wording (item 6).

## Acceptance criteria contributions
- After item (5)'s moves, G1 (`rg -n -U --pcre2 --type rust '#\[(tracing::)?instrument\b(?!\(\s*skip_all\b)' .`) exits exactly 1 over the whole workspace, and clippy `disallowed-macros` / `print_stdout` / `print_stderr` / `dbg_macro` stay green (per obs-plan §9 Gate commands).
- Every gate home the chunk's new or changed tests boot sits under `target/e2e-home/`. Over those homes, `scripts/g2-zero-panics.sh --probe` and then the check pass, and G4 schema conformance reads no failing line (per obs-plan §9 Gate commands).
- Item (10)'s edit leaves the obs-plan §3 log-format key's fenced copy still matching test-plan §3 Log format, with no required field or enum value renamed or removed (per obs-plan §3 Log format JSON schema).
- No item (4) survivor located in obs code (`obs_event!` call sites, `MillisUtc`, the panic hook) is closed as "recorded" alone. Such a survivor is a build-failure condition at the next `run --mutants` (per obs-plan §10 Build / deploy failure conditions).
