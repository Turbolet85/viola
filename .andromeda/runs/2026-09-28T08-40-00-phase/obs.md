# obs extract

## Relevance
partial — the chunk adds no telemetry, but it owns the human stdout/stderr channel that obs's per-role channel rules, its print ban and the §7 cli catch-site line govern.

## Constraints
- Per-role output channels: `hook` writes only its decision body to stdout, `run` writes nothing of its own while the child runs, `mcp` writes only MCP frames to stdout, and only `ui` and short-lived CLI verbs may use stderr for human output (per obs-plan §3 Observability Harness Contract, the opening channel list). The layer must be unreachable from the `hook`, `mcp` and in-child `run` paths. Whether the dispatch already makes it unreachable is research's question.
- The print macros are banned by `[workspace.lints.clippy]` `print_stdout` / `print_stderr` / `dbg_macro` = `deny`. Only the output modules that own `--json`, human CLI stderr, the hook decision body and the one-time `ui` launch line may carry a local `#[allow]` (per obs-plan §11 Logs, the print-macro ban; §3 Bootstrap phases → obs-ci-gate-wire). The new SGR/output module is one such owner, and its allow must stay local to it.
- Colour selection is not telemetry. It adds no `event` value and no field, because the `event` enum is closed and fields are default-deny against `schemas/diag-line.v1.json` (per obs-plan §6 Log Coverage, Additive field catalog; §8 Default-deny posture; §11 Logs, "NEVER … add `event` values without a Decisions Log entry"). If a colour or depth decision seems to need logging, raise it as a fork, not a new field.
- Env vars are not a configuration channel, and no env var, flag or config value may disable a control or widen redaction (per obs-plan §3 Logging stack; §11 PII Scrubbing). `NO_COLOR` / `TERM` / `COLORTERM` readings may change human rendering only. They must never reach the subscriber, the level filter, the field allow-list or the diagnostics writer. Whether they need a named admission is security's question, which P3 answers.
- Human stderr lines are fixed strings: refusal and fault lines carry no paths, pids, chains or upstream text (per obs-plan §1 Telemetry surfaces, cli short-lived verbs Notes). Any serde-sourced anyhow chain becomes the fixed `internal error` before it reaches stderr or `--json` (per obs-plan §7, scrubbing layer 2).
- CARRY catch site: a short-lived `cli` role exits 1 with the fixed stderr `error: internal error`. The chain goes only to `instances/<name>/diagnostics/detail-cli.ndjson`, and only when an instance resolves (per obs-plan §7, Panic hooks → per-role behaviour; scrubbing layer 3; D-06). The role is classified from argv before clap runs, and any verb other than `run` / `hook` / `mcp` / `ui` is `cli` (per obs-plan §7 main-thread catch site, D-28). Whether `main.rs`'s `Role` already maps a `cli` role that a test can reach is research's question.
- Zero unlogged panics: a `cli` panic after init step 4 must yield exactly one home-level `event:"panic"` line. A `cli` without a resolved instance is a bounded exemption (per obs-plan §10 Always-required SLO invariant; D-06). The output layer's writes therefore must not panic on a failed write. `println!`-style macros panic on a failed write, such as a closed pipe. How the layer handles write errors is P4's call.

## Patterns to follow
- A local `#[allow(clippy::print_stdout, clippy::print_stderr)]` scoped to the single output-owning fn or module. The precedent is the `run` path's pre-spawn `.cmd`/`.bat` refusal fn with its two fixed stderr lines (per obs-plan §11 Logs; §3 Observability Harness Contract).
- The token-bearing `ui` launch line is written to stderr directly and never through the subscriber (per obs-plan §8 PII Scrubbing table, GUI token row). The layer's stderr path for that line follows the same rule.
- Diagnostics never carry terminal styling. The subscriber is built `.with_ansi(false)`, with the tracing-subscriber `ansi` feature off (per obs-plan §3 OTel SDK init, init body sketch; D-11). SGR lives only in the human-output layer and never feeds the log pipeline.
- C0/C1 controls in upstream-origin rows are escaped in any human terminal rendering (per obs-plan §1 Coverage Triggers, security Vector 8; §8 table, `claude agents --json` row). The layer's `\x1B`-style escaping is that rendering.

## Anti-patterns to avoid
- NEVER use `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths, and never widen the print allow to crate level for a product crate (per obs-plan §11 Logs).
- NEVER send human output through `tracing` / `obs_event!`, and never send telemetry to stdout on any role (per obs-plan §11 Logs, stdout ban). Also never use a raw `tracing::{event,info,…}!` outside `viola_core::obs` (per obs-plan §11 Logs).
- NEVER let `NO_COLOR` / `TERM` / `COLORTERM` / a flag alter redaction, the field allow-list or the diagnostics level (per obs-plan §11 PII Scrubbing).

## Contract bindings
- obs ↔ design-system: obs §7's fixed `error: internal error` (uncoloured, no hint) must equal design-system §Surface: cli → Exit-code phraseology. obs §3's per-role channel list must match design-system §Streams' stdout/stderr split.
- obs ↔ security: C0/C1 escaping in human rendering (obs §1 Vector 8 trigger, §8) binds to security-plan §Input Validation. The env-var reads bind to obs §3 Logging stack's "env vars are not a configuration channel" and to security's two-seam env sentence, which P3 settles.
- obs ↔ tests: the §9 clippy step and obs-ci-gate-wire prove the print ban both ways (a `println!` in a `hook` path fails clippy, and an output module's local `#[allow]` passes). The new output module is a new allow site under that proof. If the CARRY binds, G2's home-level panic counting and the D-06 `cli-<name>.ndjson` / `detail-cli.ndjson` files enter the test surface.

## Acceptance criteria contributions
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` passes. `print_stdout` / `print_stderr` are allowed only locally inside the output layer, and no print macro is reachable from `hook` / `mcp` / in-child `run` code (per obs-plan §11 Logs; §3 Bootstrap phases → obs-ci-gate-wire).
- No new `event` value or field: `schemas/diag-line.v1.json` and `diag-detail.v1.json` are unchanged by the chunk, and G4 schema conformance stays green (per obs-plan §8 Default-deny posture; §6 Log Coverage).
- If the CARRY binds: a test-reachable `cli` catch-site error or panic exits 1 with stderr exactly `error: internal error` (no SGR byte, no `hint:` line) and an empty stdout. With an instance resolved, the chain appears only in `detail-cli.ndjson` and never in a home-level line (per obs-plan §7 Panic hooks / scrubbing layers 2–3).
- No diagnostics line written during a run that exercises the layer contains an ESC (`\x1b`) byte (per obs-plan §3 OTel SDK init, `.with_ansi(false)`).
