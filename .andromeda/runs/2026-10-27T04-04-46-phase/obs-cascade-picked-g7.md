# obs · picked accounting · group 7 of 10

## Picked
span-trace-coverage#7.1 · not in scope · the `wait` / `last` span nesting and the server check's place in it, owned by "Unix endpoint and home hardening"; the chunk opens no span and moves no check on this path
span-trace-coverage#7.2 · not in scope · span attributes of `wait.block` / `run.wait_dispatch`; the chunk adds or alters no span attribute, and its typed reading of `wait`'s params reads the request, not the span
span-trace-coverage#7.3 · carried · the closed-type reading of channel params keeps `channel-request` holding `after` / `timeout_ms` only as `u64` values and no params or result body
span-trace-coverage#7.4 · not in scope · `wait.block`'s close paths and the mid-call disconnect line, unchanged by any bound the chunk adds
span-trace-coverage#8.1 · carried · the instance check ahead of every dialog-hook and `answer` frame is what the names-before-path-joins witness reads: a refused or absent instance writes no line, frame or decision
span-trace-coverage#8.2 · carried · `dialog_kind` is a closed hook-event decision and `deadline_ms` reads the bounded `dialog_hold_s` config value, range and default
span-trace-coverage#8.3 · carried · the dialog hook's bounded stdin, its fail-open lines, the refused-instance silence and the closed `unknown-dialog` refusal, with no question, answer or tool `input`
span-trace-coverage#8.4 · carried · the hook's backstop read bound and the fail-open pair before `dialog_id` that an oversize or malformed dialog stdin ends in
span-trace-coverage#9.3 · carried · `release`'s `from` / `budget` param type checks are closed-type channel param decisions, a `-32602` with no line, `from` logged only when a valid name
span-trace-coverage#10.1 · not in scope · the budget path's span tree; the chunk adds no span, and the bounded reads under it carry none
span-trace-coverage#10.2 · carried · the `budget.json` checked read, over-cap included, is no reading with no span and no line, and `statusline_command` never appears
span-trace-coverage#10.3 · carried · the statusline arm's oversize / malformed stdin lines and fail-open `detail`, and no line without a valid `VIOLA_NAME`
span-trace-coverage#10.4 · carried · the status line command's stdout passes through under its cap unlogged, and oversize stdin prints nothing and runs no shell
span-trace-coverage#11.1 · not in scope · the R8 strip's name-only fields on the `claude-child` `process-start`; the chunk caps the spawned readers' output and changes no strip, env read or `claude-child` field
span-trace-coverage#12.1 · not in scope · the implicit link's no-line, no-span trace inside `send` / `answer`, unchanged; the chunk's name checks on `driver` / `driven` are the explicit methods'
span-trace-coverage#13.1 · carried · the explicit `link` / `unlink` methods read `driver` / `driven` through `ViolaName::try_new`, an invalid name `-32602` with no line, a channel name the scope names
span-trace-coverage#14.1 · carried · the bounded `snapshot.json` read keeps `read_snapshot_or_replay`'s one `state-recovered` line per call and no product `read_snapshot` caller
span-trace-coverage#15.1 · carried · the bounded `events.ndjson` reader keeps its three skip counts with no process-log carrier and neither heals nor logs
span-trace-coverage#16.1 · carried · the bounded `ledger/stamps.json` read lands an over-cap ledger in a closed `ledger-stamps` outcome read as `cli_verified:false`, no path or contents
span-trace-coverage#17.1 · carried · the hidden capture arms' stdin caps and verify's `agents-probe` read with no row text are external readers the enumeration names
