# obs · Obs Anti-Patterns — category read

## Carried
- (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) Any change to `run`'s squatted-name start refusal, or to `viola-channel`'s Windows listener, must use no print macro in the `run` code path. A start refusal must go only through `src/human.rs` `refuse` / `write_refusal`, with no new `#[allow]`. The recorded caller count (`run` 5) must still hold, or must be amended where it is written.
  > NEVER use `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths of the product crates (`viola-harness` and the fake agent are exempt, §3 obs-ci-gate-wire). They bypass the file sink and break the Hook contract, the `run` terminal rule and MCP framing. Enforced by `[workspace.lints.clippy]` `print_stdout`, `print_stderr`, `dbg_macro` = `deny` under the §9 clippy step. Only the output modules that own `--json`, human CLI stderr and the hook decision body may carry a local `#[allow]`, and only where they use a print macro — `run`'s and `revive`'s pre-spawn refusals, `revive --list`'s, `verify`'s, `list`'s and `plugin install`'s refusals write through `src/human.rs` (`refuse` / `write_refusal`, one `write_all` on the locked stderr; callers `run` 5 · `verify` 5 · `list` 2 · `revive` 1 · `plugin install` 1, chunk 2026-10-17-optional-plugin-install), which uses no print macro and carries no `#[allow]` — plus the fake agent's crate-level `#![allow]` in `src/bin/viola-fake-agent.rs` (a root-package bin, §3 obs-ci-gate-wire).
- (.andromeda/obs-plan.md:717) The body states the same print-macro ban broadly, for every `run` path the chunk's listener and start-refusal work touches.
  > NEVER use `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths of the product crates: they bypass the file sink and break the Hook contract, the `run` terminal rule and MCP framing; enforced by the clippy `print_stdout`, `print_stderr` and `dbg_macro` lints at `deny`, except `viola-harness`, the fake agent and the output modules that own `--json`, human CLI stderr and the hook decision body (below).
- (.andromeda/obs-plan.md:707) No telemetry may reach stdout from the listener, the start refusal, or the CARRY's capture fix. Any writer must be set explicitly.
  > NEVER write telemetry to stdout on any role. stdout carries `--json` results (cli), the decision body or, for `hook statusline`, the user's status-line output (hook) and MCP frames (mcp), and `run`'s terminal carries the child's screen. The default tracing-subscriber writer is stdout, so `with_writer` is always explicit.
- (.andromeda/obs-plan.md:709) Any new log line in `viola-channel`'s server, for example a rejected remote client or a failed first-instance bind, must go through `obs_event!` only, never a raw tracing macro.
  > NEVER call `tracing::{event,info,warn,error,debug,trace}!` directly outside `viola_core::obs`. Only `obs_event!` guarantees `event`, `process` and `instance`; the caller sets `corr` explicitly as a typed field on the events that carry one. Enforcement: clippy `disallowed-macros` for the level macros, and the fail-closed raw-`event!` grep in `scripts/lint-probes.sh` for `event!` (§3 logger-stack-install).
- (.andromeda/obs-plan.md:715) Logging a remote-client rejection or a first-instance failure must not add an `event` value without a Decisions Log entry. The scope says no refusal word changes.
  > NEVER rename tests fields (for example `corr` → `correlation_id`, `event` → `event_type`) or add `event` values without a Decisions Log entry.
- (.andromeda/obs-plan.md:731) Any field logged for the listener's admission outcome must already be named in the §6 catalog (default-deny).
  > NEVER log any field not named in the Section 6 catalog (default-deny).
- (.andromeda/obs-plan.md:735) The squatted-name refusal's `detail` code, and any admission-failure code, must carry no pipe name, path or pid.
  > NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.
- (.andromeda/obs-plan.md:690) A span on the Windows listener's bind or accept must carry no content in its fields.
  > NEVER include PII or content in span fields.
- (.andromeda/obs-plan.md:691) Any span added to `server/win.rs` or `endpoint.rs` must use `#[instrument(skip_all, fields(...))]`, never a bare `#[instrument]`.
  > NEVER use a bare `#[instrument]`. It records every argument with `Debug` and leaks send text, `hook.dialog` payloads and tool `input`. Always use `#[instrument(skip_all, fields(...))]`.
- (.andromeda/obs-plan.md:693) No pipe name or endpoint path may appear in a span name.
  > NEVER use high-cardinality span names: no instance names, dialog ids or paths in span names.
- (.andromeda/obs-plan.md:748) The CARRY on the flaky `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 must be closed by three plain `cargo test` runs or by fixing its log capture. A retry policy must never mask it.
  > NEVER add retry-once policies for panics or flaky telemetry assertions.

## Bindings
- **print-macro ban ↔ body §3 (the `run` terminal rule)**: the squatted-endpoint start refusal is one of `run`'s `src/human.rs` `refuse` sites, so the caller count `run` 5 binds both sides (.andromeda/obs-plan-detail/obs-anti-patterns.md:8), (.andromeda/obs-plan.md:717)
- **raw tracing macro ban ↔ §3 logger-stack-install contract and `scripts/lint-probes.sh`**: a new channel-server line uses `obs_event!`, and the fail-closed grep proves it (.andromeda/obs-plan.md:709)
- **no new `event` value ↔ §6 Log Coverage catalog and §12 Decisions Log**: an admission-outcome line uses an existing `event` and catalog fields, or it needs a logged decision (.andromeda/obs-plan.md:715), (.andromeda/obs-plan.md:731)
- **no path or pid in `detail` ↔ §6 `detail` code catalog and security-plan §Authentication (`squatted-name`, exit 1 stand)**: the refusal code stays closed and path-free (.andromeda/obs-plan.md:735)
- **span bans ↔ §4 Scenario `viola run` start sequence to child spawn**: the endpoint name never reaches a span field or a span name (.andromeda/obs-plan.md:690), (.andromeda/obs-plan.md:693)
- **no retry-once ↔ test-plan zero-flakiness (`retries = 0`), the body's §10 cross-input parallel**: the CARRY's red case is fixed or measured green three times, never retried away (.andromeda/obs-plan.md:748)
