# obs · Obs Anti-Patterns — category read

## Carried
- (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) The pinned-copy race fix lands on `run`'s start path (`pin_bytes`) and `verify`'s (`pin_exe`) and in `viola-state`'s `pin.rs` / `fs.rs`: no print macro may enter those code paths; any outcome is a file-sink line or a refusal through the one writer.
  > NEVER use `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths of the product crates (`viola-harness` and the fake agent are exempt, §3 obs-ci-gate-wire). They bypass the file sink and break the Hook contract, the `run` terminal rule and MCP framing. Enforced by `[workspace.lints.clippy]` `print_stdout`, `print_stderr`, `dbg_macro` = `deny` under the §9 clippy step.
- (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) A new or changed start refusal for a concurrent pin (in `run` or `verify`) must go through `src/human.rs` `refuse` with no `#[allow]`, and the per-caller counts (`run` 7 · `verify` 6) move only by re-deriving them by their stated rule, the `human::refuse(` call sites under `src/`.
  > `run`'s and `revive`'s pre-spawn refusals, `revive --list`'s, `verify`'s, `list`'s and `plugin install`'s refusals, and the instance-check refusal of `link` / `unlink` and of `send` / `wait` / `last` / `answer` / `pause` / `release`, write through `src/human.rs` (`refuse` / `write_refusal`, one `write_all` on the locked stderr; callers `run` 7 · `verify` 6 · `list` 2 · `revive` 1 · `plugin install` 1 · `client` 1, the one `refuse_untrusted` site `link` / `unlink` and the six frame verbs share; the rule: the `human::refuse(` call sites under `src/`), which uses no print macro and carries no `#[allow]`
- (.andromeda/obs-plan.md:717) The body's broad line for the same ban binds the pin path and any wait-out loop the fix adds on the `run` start.
  > NEVER use `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths of the product crates: they bypass the file sink and break the Hook contract, the `run` terminal rule and MCP framing; enforced by the clippy `print_stdout`, `print_stderr` and `dbg_macro` lints at `deny`, except `viola-harness`, the fake agent and the output modules that own `--json`, human CLI stderr and the hook decision body (below).
- (.andromeda/obs-plan.md:707) The pin race's handling (a wait, a same-bytes acceptance, a lost race) must write nothing to stdout or the `run` terminal; any telemetry goes through an explicit writer to the file sink.
  > NEVER write telemetry to stdout on any role. stdout carries `--json` results (cli), the decision body or, for `hook statusline`, the user's status-line output (hook) and MCP frames (mcp), and `run`'s terminal carries the child's screen. The default tracing-subscriber writer is stdout, so `with_writer` is always explicit.
- (.andromeda/obs-plan.md:709) Any line the fix adds for a pin outcome is emitted through `obs_event!` only, never a raw tracing macro.
  > NEVER call `tracing::{event,info,warn,error,debug,trace}!` directly outside `viola_core::obs`. Only `obs_event!` guarantees `event`, `process` and `instance`; the caller sets `corr` explicitly as a typed field on the events that carry one.
- (.andromeda/obs-plan.md:715) A new pin-race outcome event value (or a renamed field) requires a Decisions Log entry before it is emitted.
  > NEVER rename tests fields (for example `corr` → `correlation_id`, `event` → `event_type`) or add `event` values without a Decisions Log entry.
- (.andromeda/obs-plan.md:713) A wait-out loop on a concurrent pin write (the `ERROR_SHARING_VIOLATION`-style wait, or any new retry on the exe's arm) logs no line per attempt; a count on the boundary event instead.
  > NEVER log at `info` inside hot loops (PTY pump). Use counts on the boundary event instead.
- (.andromeda/obs-plan.md:691) Any new or changed instrumented function in `pin.rs` / `fs.rs` / the start path carries `#[instrument(skip_all, fields(...))]`, never a bare one (the pinned bytes and paths are arguments).
  > NEVER use a bare `#[instrument]`. It records every argument with `Debug` and leaks send text, `hook.dialog` payloads and tool `input`. Always use `#[instrument(skip_all, fields(...))]`.
- (.andromeda/obs-plan.md:692) No `err` / `ret` option on the pin functions whose error or return could carry a source with content.
  > NEVER use `#[instrument(err)]` or `#[instrument(ret)]` on functions whose error or return can carry a serde / serde_path_to_error source or user content.
- (.andromeda/obs-plan.md:693) A span over the pin write is named statically; the `bin/<key>/` path and the version-hash key never appear in a span name.
  > NEVER use high-cardinality span names: no instance names, dialog ids or paths in span names.
- (.andromeda/obs-plan.md:731) Any field the fix adds to a line (an attempt count, a race outcome) must first be named in the Section 6 catalog.
  > NEVER log any field not named in the Section 6 catalog (default-deny).
- (.andromeda/obs-plan.md:735) A `detail` code for a pin-race outcome or a lost-race exit carries no path or pid (the pinned path, the competing start's pid).
  > NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.
- (.andromeda/obs-plan.md:748) The two-sided race witness (red before, clean after, beside the 0-of-24 single-instance control) may not lean on a retry to pass.
  > NEVER add retry-once policies for panics or flaky telemetry assertions.

## Bindings
- **Print ban and the one refusal writer ↔ body §3 harness contract**: the detail's callers count (`run` 7 · `verify` 6 …) is the same count §3 states as re-derived by `grep -rcE 'human::refuse\(' src`; a refusal the fix adds moves both, by the one rule (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) (.andromeda/obs-plan.md:311)
- **Print ban detail ↔ body §11 Logs**: the body's broad line and its pointer to this file (.andromeda/obs-plan.md:705) (.andromeda/obs-plan.md:717)
- **Default-deny field ban ↔ body §8 / §9 G4**: a new field fails `schemas/diag-line.v1.json`'s top-level `unevaluatedProperties: false` under G4 (.andromeda/obs-plan.md:731) (.andromeda/obs-plan.md:576)
- **Path-free `detail` codes ↔ body §6 `detail` catalog**: a new code joins the closed catalog, never a path or pid (.andromeda/obs-plan.md:735) (.andromeda/obs-plan.md:451)
- **No retry-once ↔ body §10 cross-input parallel**: the witness sits under the tests' `retries = 0` budget (.andromeda/obs-plan.md:748) (.andromeda/obs-plan.md:662)
- **Raw-macro ban ↔ body §8 source allow-list**: enforced by clippy `disallowed-macros` and the `scripts/lint-probes.sh` raw-`event!` grep (.andromeda/obs-plan.md:709) (.andromeda/obs-plan.md:565)
