# Apply — obs-plan · part 2 of 2

## Verdicts
- fanout-obs-plan--cascade.md 17 · applied · .andromeda/obs-plan-detail/log-coverage.md:13#1
- fanout-obs-plan--cascade.md 18 · applied · .andromeda/obs-plan-detail/log-coverage.md:29#1
- fanout-obs-plan--cascade.md 19 · applied · .andromeda/obs-plan-detail/log-coverage.md:35#1
- fanout-obs-plan--cascade.md 20 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:23#1
- fanout-obs-plan--cascade.md 21 · applied · .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:11#1
- fanout-obs-plan--cascade.md 22 · applied · .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:17#1, .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:17#2
- fanout-obs-plan--cascade.md 23 · applied · .andromeda/obs-plan-detail/obs-anti-patterns.md:8#1
- fanout-obs-plan.md 4 · applied · .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:22+#1
- cascade-compare-obs-plan.md 2 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:20+#1

## Edits
- line: .andromeda/obs-plan-detail/log-coverage.md:13
  old: «| `channel-response` | `method`, `conn` / `srv_conn`,»
  new: «| `channel-response` | `method` (only a name on the channel's closed method list, `link` and `unlink` among them; any other name logs no `method` key), `conn` / `srv_conn`,»
- line: .andromeda/obs-plan-detail/log-coverage.md:29
  old: «written by the six CLI frame verbs on a server that is not the recorded wrapper, or a check that cannot be made (Linux and Windows): `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN into `cli-<name>.ndjson`,»
  new: «written by the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) on a server that is not the recorded wrapper, or a check that cannot be made (Linux and Windows): `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN into `cli-<name>.ndjson` (`link` / `unlink`: the driven instance's),»
- line: .andromeda/obs-plan-detail/log-coverage.md:35
  old: «the `instance-unreachable` document (chunk 2026-10-17-optional-plugin-install))»
  new: «the `instance-unreachable` document (chunk 2026-10-17-optional-plugin-install); `viola link` / `viola unlink`'s refused `check_instance` of the driven instance exits 21 and writes no line either — no `process-start`, no `process-exit` — the fixed instance-state pair on stderr, or under `--json` the `instance-unreachable` document)»
- line: .andromeda/obs-plan-detail/error-capture-reporting.md:23
  old: «also run before any role file is opened (chunk 2026-10-17-mcp-server-for-drivers).»
  new: «also run before any role file is opened (chunk 2026-10-17-mcp-server-for-drivers), and the instance check that `viola link` / `viola unlink` run on the driven instance before their `cli` role file opens, whose refusal (exit 21 `instance-unreachable`: the fixed instance-state pair, or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}`) writes no line in any log and leaves the directory's mode as found.»
- line: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:11
  old: «`marketplace/` tree dirs, which runs before its `cli` role file opens);»
  new: «`marketplace/` tree dirs, which runs before its `cli` role file opens, and the instance check of `viola link` / `viola unlink` on the driven instance, which runs before their `cli` role file opens);»
- line: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:17
  old: «through `check_path` (chunk 2026-10-17-optional-plugin-install): it opens no role file»
  new: «through `check_path` (chunk 2026-10-17-optional-plugin-install), or a refused `check_instance` of the driven instance on `viola link` / `viola unlink`: it opens no role file»
- line: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:17
  old: «list and plugin install: exit 21, the `--json` error document or the stderr pair;»
  new: «list and plugin install: exit 21, the `--json` error document or the stderr pair; link and unlink: exit 21, the fixed instance-state pair or under `--json` the `instance-unreachable` document;»
- line: .andromeda/obs-plan-detail/obs-anti-patterns.md:8
  old: «`list`'s and `plugin install`'s refusals write through `src/human.rs` (`refuse` / `write_refusal`, one `write_all` on the locked stderr; callers `run` 5 · `verify` 5 · `list` 2 · `revive` 1 · `plugin install` 1, chunk 2026-10-17-optional-plugin-install)»
  new: «`list`'s and `plugin install`'s refusals, and `link`'s / `unlink`'s instance-check refusal, write through `src/human.rs` (`refuse` / `write_refusal`, one `write_all` on the locked stderr; callers `run` 5 · `verify` 5 · `list` 2 · `revive` 1 · `plugin install` 1 · `link` 1, the one `link_call` site both verbs share; the rule: the `human::refuse(` call sites under `src/`)»
- after: .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:22
  anchor: «  - `hook`: one `config.json` read plus exactly one log `open`, then appends, which fits the `max < 1.0 s` gate. `viola hook statusline` runs its instance check before this init, because the role-file open itself sets an existing home to 0700 (`open_role_file` → `create_private_dir(home)`) and a check made after it passes a home at 0770 (as measured at chunk 2026-10-10-statusline-pass-through, `evidence/red-green.md` §3). A refused instance returns exit 0 at once, before anything under the home is read or opened (chunk 2026-10-10-budget-governor): no `config.json` read, no role file opened, no `hook-invoked`, no `hook-decision`, and the home's mode left as found, so step 4's "a refused home gets no diagnostics line" holds for this arm. Before that chunk the arm opened the role file after a refused check and wrote `hook-decision{detail:"strict-modes-failed"}`, and that opening narrowed the refused home, so the refusal lasted one invocation (as measured at that chunk's `evidence/red-green.md` (b)).»
  add: «  - `cli`: `viola link` / `viola unlink` (instance: the driven name from their own argument, log `cli-<driven>.ndjson`) run `check_instance` on the home and `instances/<driven>` BEFORE the log init (`client::start`), because the role-file open narrows an existing home to 0700 and a check made after it passes a home at 0770: a refusal opens no role file, writes no `process-start`, no `process-exit` and no line in any log, and leaves the directory's mode as found (exit 21, the fixed instance-state pair, or `{"v":1,"error":"instance-unreachable","detail":null}` under `--json`). Only after a passed check does the log open with `process-start`, the liveness pre-check and the server check following.»
- after: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:20
  anchor: «| `claude agents --json` rows / unwrapped session ids | Low, untrusted | OK structured; C0/C1 escaped in human terminal output | JSON serialization escapes C0 controls in files; rows never logged verbatim, only `parse-rejected` counts |»
  add: «| `link` / `unlink` pair names (`driver`, `driven`, each a `ViolaName`) | Low | OK in the contract payload: the wrapper's `link` / `unlink` event lines in `events.ndjson` (`data: {driver, driven}`, `source: wrapper`); never in a process-log line | the served methods open no span and write no process-log line of their own, and the wrapper's `channel-request` / `channel-response` pair logs `method`, never the `driver` / `driven` params; the verbs' result lines print their own parsed `ViolaName` arguments, never a name read back from the reply or the log; asserted by `link_verbs_log_the_channel_pair_and_no_names` (`tests/cli_links.rs`) |»

## Patterns
[[pattern]]
id = "obs2-01"
fixed = "six CLI frame verbs"

[[pattern]]
id = "obs2-02"
fixed = "`plugin install` 1, chunk 2026-10-17-optional-plugin-install"
