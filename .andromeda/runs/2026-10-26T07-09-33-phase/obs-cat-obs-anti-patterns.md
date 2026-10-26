# obs · Obs Anti-Patterns — category read

## Carried
- (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) The hook client's fail-open on a server mismatch or an unmakeable check, and the MCP tools' `instance-unreachable` path, must write nothing through a print macro: no byte on the hook's stdout or stderr and no stray byte between MCP frames.
  > NEVER use `print!` / `println!` / `eprint!` / `eprintln!` / `dbg!` in the `hook`, `run` or `mcp` code paths of the product crates (`viola-harness` and the fake agent are exempt, §3 obs-ci-gate-wire). They bypass the file sink and break the Hook contract, the `run` terminal rule and MCP framing. Enforced by `[workspace.lints.clippy]` `print_stdout`, `print_stderr`, `dbg_macro` = `deny` under the §9 clippy step.
- (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) A CLI verb's unreachable refusal on a failed verification must go through an output module that owns human CLI stderr or `--json`, never a new local `#[allow]` on a print macro; and if the chunk adds a `human::refuse` call site, the caller counts this line names are re-derived and amended.
  > Only the output modules that own `--json`, human CLI stderr and the hook decision body may carry a local `#[allow]`, and only where they use a print macro — `run`'s and `revive`'s pre-spawn refusals, `revive --list`'s, `verify`'s, `list`'s and `plugin install`'s refusals write through `src/human.rs` (`refuse` / `write_refusal`, one `write_all` on the locked stderr; callers `run` 5 · `verify` 5 · `list` 2 · `revive` 1 · `plugin install` 1, chunk 2026-10-17-optional-plugin-install), which uses no print macro and carries no `#[allow]`
- (.andromeda/obs-plan.md:707) No telemetry about the verification (outcome, mismatch, cause) may reach stdout on the cli, hook or mcp role; any logger the verification path touches keeps an explicit writer.
  > NEVER write telemetry to stdout on any role. stdout carries `--json` results (cli), the decision body or, for `hook statusline`, the user's status-line output (hook) and MCP frames (mcp), and `run`'s terminal carries the child's screen. The default tracing-subscriber writer is stdout, so `with_writer` is always explicit.
- (.andromeda/obs-plan.md:709) Any line the verification writes (a refusal, a fail-open `hook-decision`, a `process-exit` exit 21) must be emitted through `obs_event!`, never a raw tracing macro, with `corr` set explicitly where the event carries one.
  > NEVER call `tracing::{event,info,warn,error,debug,trace}!` directly outside `viola_core::obs`. Only `obs_event!` guarantees `event`, `process` and `instance`; the caller sets `corr` explicitly as a typed field on the events that carry one.
- (.andromeda/obs-plan.md:715) A new `event` value for a failed server verification must not be added without an obs Decisions Log entry; prefer a closed `detail` code on an existing event.
  > NEVER rename tests fields (for example `corr` → `correlation_id`, `event` → `event_type`) or add `event` values without a Decisions Log entry.
- (.andromeda/obs-plan.md:735) The mismatch the check detects is a pid / start-time mismatch, but neither the recorded pid, the peer's pid, the start time nor the endpoint path may appear inside a `detail` code.
  > NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.
- (.andromeda/obs-plan.md:731) A field the verification adds to a line (for example a server pid or a verification cause) must be named in the §6 catalog first; anything else is a schema failure under default-deny.
  > NEVER log any field not named in the Section 6 catalog (default-deny).
- (.andromeda/obs-plan.md:724) The verification reads `snapshot.json` through serde, so a failed read or parse surfaced as a CLI refusal, an MCP `isError` or a channel error must not carry an anyhow chain holding that serde source.
  > NEVER print an anyhow chain that still contains a serde_json / serde_path_to_error source to stderr, `--json`, MCP `isError` or channel `error.data`.
- (.andromeda/obs-plan.md:691) A verification function given a span must declare it `skip_all` with an explicit field allow-list, so the snapshot, the endpoint and the peer credentials are never recorded by `Debug`.
  > NEVER use a bare `#[instrument]`. It records every argument with `Debug` and leaks send text, `hook.dialog` payloads and tool `input`. Always use `#[instrument(skip_all, fields(...))]`.
- (.andromeda/obs-plan.md:692) The verification's error and return can carry a serde source from the snapshot read, so its span takes neither `err` nor `ret`.
  > NEVER use `#[instrument(err)]` or `#[instrument(ret)]` on functions whose error or return can carry a serde / serde_path_to_error source or user content.
- (.andromeda/obs-plan.md:693) A verification span is named statically, with no instance name, endpoint or path in its name.
  > NEVER use high-cardinality span names: no instance names, dialog ids or paths in span names.
- (.andromeda/obs-plan.md:694) The MCP tools' verification runs in the Tokio client, so `corr` / `conn` / `instance` cross into it as explicit values, not through span context.
  > NEVER rely on span context to carry `corr` / `conn` / `instance` across threads or tasks. Spans are not serialized, so pass these as explicit values.
- (.andromeda/obs-plan.md:683) The verification sits at the front of the send, wait / last, dialog → answer and wheel must-trace paths, and must not drop or bypass their existing spans and events.
  > NEVER skip spans or events on the 6 must-trace paths (Section 4). The agent loses its ability to debug.
- (.andromeda/obs-plan.md:700) Any derived count of verification failures groups by a closed `detail`, never by the pid it compared.
  > NEVER group derived metrics by unbounded keys (`path`, `corr`, `conn`, `pid`, `instance`). Group by the closed enums only.

## Bindings
- **Print ban and fail-open ↔ the body's §3 harness and the universal invariant**: the hook's no-byte fail-open on mismatch (scope "What it builds") is the same rule as §3's "`hook` writes only its decision body to stdout and nothing to stderr" (.andromeda/obs-plan.md:310), enforced by the print ban (.andromeda/obs-plan-detail/obs-anti-patterns.md:8).
- **`event` ban ↔ §4's wait / last scenario**: §4 already names `strict-modes-failed` / `server-verify-failed` as the codes that cannot occur "until client-side server verification lands", owned by this entry and "Home and code-bearing file integrity" (.andromeda/obs-plan.md:361); landing one is a `detail` code under the ban on new `event` values (.andromeda/obs-plan.md:715) and the no-pid rule (.andromeda/obs-plan.md:735).
- **detail no-pid rule ↔ §6 log levels**: a verification refusal ending in exit 21 logs as `process-exit` exit 21 at `warn` (.andromeda/obs-plan.md:460), its `detail` a closed code with no pid or path (.andromeda/obs-plan.md:735).
- **Default-deny ↔ §8 / G4**: a new field fails G4 against `schemas/diag-line.v1.json` (.andromeda/obs-plan.md:576) unless the §6 catalog names it first (.andromeda/obs-plan.md:731).
- **Span bans ↔ §10 perf budget**: the hook's added verification stays inside the `pre-tool-use` / spine hook `max` budget gated by `gate --require perf` (.andromeda/obs-plan.md:649).
