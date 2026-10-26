# obs · Obs Anti-Patterns — category read

## Carried
- (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) The wrapper's new `link` / `unlink` channel-method handlers sit in the `run` code path, so they must carry no print macro; the lint gate must stay at `deny` with no new `#[allow]` outside the output modules.
  > NEVER use `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths of the product crates (`viola-harness` and the fake agent are exempt, §3 obs-ci-gate-wire). They bypass the file sink and break the Hook contract, the `run` terminal rule and MCP framing. Enforced by `[workspace.lints.clippy]` `print_stdout`, `print_stderr`, `dbg_macro` = `deny` under the §9 clippy step.
- (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) The `viola link` / `viola unlink` verbs' three result lines, their `--json` document and any refusal pair must be written only by an output module that owns `--json` or human CLI stderr. Refusals go through `src/human.rs` `refuse` / `write_refusal`. If the verbs add a `human::refuse` caller, the per-verb caller counts quoted here must be re-derived and amended.
  > Only the output modules that own `--json`, human CLI stderr and the hook decision body may carry a local `#[allow]`, and only where they use a print macro — `run`'s and `revive`'s pre-spawn refusals, `revive --list`'s, `verify`'s, `list`'s and `plugin install`'s refusals write through `src/human.rs` (`refuse` / `write_refusal`, one `write_all` on the locked stderr; callers `run` 5 · `verify` 5 · `list` 2 · `revive` 1 · `plugin install` 1, chunk 2026-10-17-optional-plugin-install), which uses no print macro and carries no `#[allow]`
- (.andromeda/obs-plan.md:707) The verbs' `--json` result goes to stdout, so no telemetry may share stdout. Each role's writer stays explicit.
  > NEVER write telemetry to stdout on any role. stdout carries `--json` results (cli), the decision body or, for `hook statusline`, the user's status-line output (hook) and MCP frames (mcp), and `run`'s terminal carries the child's screen. The default tracing-subscriber writer is stdout, so `with_writer` is always explicit.
- (.andromeda/obs-plan.md:709) Every line the new methods or verbs log must be emitted through `obs_event!`, never through a raw tracing level macro or `event!`.
  > NEVER call `tracing::{event,info,warn,error,debug,trace}!` directly outside `viola_core::obs`. Only `obs_event!` guarantees `event`, `process` and `instance`; the caller sets `corr` explicitly as a typed field on the events that carry one.
- (.andromeda/obs-plan.md:710) The `corr` of a `link` / `unlink` channel call must be recorded unchanged as a typed field.
  > NEVER record `corr` with `?corr` / `%corr`. That stringifies numbers, and `corr` must be copied unchanged as a JSON number or string.
- (.andromeda/obs-plan.md:715) The chunk must not add any `event` value, such as a link-recorded line, without a Decisions Log entry. This holds even though the explicit methods are served.
  > NEVER rename tests fields (for example `corr` → `correlation_id`, `event` → `event_type`) or add `event` values without a Decisions Log entry.
- (.andromeda/obs-plan.md:684) Any line the link path writes must carry the closed `event` enum and `corr`, never free text.
  > NEVER let the signal pyramid collapse into free text. Every line carries the closed `event` enum and `corr`.
- (.andromeda/obs-plan.md:691) Any span on the new handlers or verb clients must be declared with `skip_all` and an explicit field allow-list, so the driver and driven names in the params are not recorded by `Debug`.
  > NEVER use a bare `#[instrument]`. It records every argument with `Debug` and leaks send text, `hook.dialog` payloads and tool `input`. Always use `#[instrument(skip_all, fields(...))]`.
- (.andromeda/obs-plan.md:692) The new handlers must not put `err` or `ret` on their spans.
  > NEVER use `#[instrument(err)]` or `#[instrument(ret)]` on functions whose error or return can carry a serde / serde_path_to_error source or user content.
- (.andromeda/obs-plan.md:693) A span for the new verbs and methods must have a static name, never one built from the driver or driven instance name.
  > NEVER use high-cardinality span names: no instance names, dialog ids or paths in span names.
- (.andromeda/obs-plan.md:694) If the link set is updated on another thread than the channel dispatch, `corr`, `conn` and `instance` must be passed as explicit values.
  > NEVER rely on span context to carry `corr` / `conn` / `instance` across threads or tasks. Spans are not serialized, so pass these as explicit values.
- (.andromeda/obs-plan.md:700) Any derived metric over the new `link` / `unlink` channel lines groups by closed enums such as `method`, never by `instance`.
  > NEVER group derived metrics by unbounded keys (`path`, `corr`, `conn`, `pid`, `instance`). Group by the closed enums only.
- (.andromeda/obs-plan.md:724) A link verb's fault output, whether stderr, `--json` or channel `error.data`, must not carry a serde-sourced anyhow chain.
  > NEVER print an anyhow chain that still contains a serde_json / serde_path_to_error source to stderr, `--json`, MCP `isError` or channel `error.data`.
- (.andromeda/obs-plan.md:731) A new field on any link line, such as a driver or driven name, must first be named in the §6 catalog. The default-deny schema rejects any field it does not name.
  > NEVER log any field not named in the Section 6 catalog (default-deny).
- (.andromeda/obs-plan.md:733) Neither verb may take a flag that widens what is logged.
  > NEVER let `config.json`, a `VIOLA_*` env var or a CLI flag widen the field allow-list or disable redaction.
- (.andromeda/obs-plan.md:735) A `-32602` cause or refusal code on the new methods, such as a `driven` that is not the serving wrapper, must be a closed code with no path or pid in it.
  > NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.

## Bindings
- **Print-macro ban and the refusal writer count ↔ body §3**: the detail's caller counts `run` 5 · `verify` 5 · `list` 2 · `revive` 1 · `plugin install` 1 (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) are repeated in the body's harness contract line (.andromeda/obs-plan.md:311), which re-derived them with `grep -rcE 'human::refuse\(' src`. If the link verbs add a caller, both sides move together.
- **Print-macro ban ↔ body §11 Logs**: the detail (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) expands the body's broad line (.andromeda/obs-plan.md:717). The exemptions named in the two must stay the same.
- **No new `event` value ↔ body §4 E3**: the edge flow (.andromeda/obs-plan.md:392) says the link has "no process-log line, no `event` value and no span, and the explicit `link` / `unlink` methods are not built". Serving the methods must amend that line. Any new line or value also needs a Decisions Log entry (.andromeda/obs-plan.md:715) and a §6 catalog entry (.andromeda/obs-plan.md:731).
- **Default-deny ↔ tests (schema)**: a new link field or event value must enter `schemas/diag-line.v1.json` (G4) together with the §6 catalog (.andromeda/obs-plan.md:576, .andromeda/obs-plan.md:731).
- **Static span names ↔ body §2 naming**: the area list (.andromeda/obs-plan.md:293) names no `link` / `unlink` area. A verb with no dedicated client span takes `cli.<verb>`. Static names follow (.andromeda/obs-plan.md:693).
- **Closed `detail` codes ↔ body §5 metrics**: `viola.compat.v_rejected` already over-counts `-32602` (.andromeda/obs-plan.md:418), and the new methods' `-32602` causes add to that over-count. Codes stay closed and path-free (.andromeda/obs-plan.md:735).
- **Stdout reservation ↔ body §1 cli surface**: `link · unlink` are listed as short-lived cli verbs whose stdout carries `--json` results (.andromeda/obs-plan.md:109, .andromeda/obs-plan.md:707).
