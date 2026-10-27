# obs · picked accounting · group 2 of 10

## Picked
obs-scope-summary#12.3 · carried · the chunk bounds the `claude agents --json` / `--version` readers and closes the row-status type, whose parse failure must stay `"unknown"` plus a counter over untrusted rows
obs-scope-summary#13.2 · carried · the statusline shell-out stays boundary-only while the chunk bounds its stdout
obs-scope-summary#13.3 · carried · the chunk bounds the status line command's stdout, which must log duration and exit status only and never the command string outside `diagnostics/`
obs-scope-summary#14.2 · not in scope · the OS-API instrumentability label; the chunk changes no OS call site
obs-scope-summary#14.3 · not in scope · bind, connect, lock, rename, kill-fallback, the server check and the strict-modes call sites are unchanged; server identity and home hardening belong to "Unix endpoint and home hardening"
obs-scope-summary#15.2 · carried · the bounded reads of `snapshot.json`, `events.ndjson`, `budget.json` and `ledger/stamps.json` are `viola-state` call sites
obs-scope-summary#15.3 · carried · the per-store scrub rules and the replayed-snapshot rule must hold over the chunk's bounded store reads
obs-scope-summary#16.2 · carried · `config.json` stays observable only at its parse, which the chunk closes
obs-scope-summary#16.3 · carried · the closed-type `config.json` read is the load-time parse whose only telemetry is the rejected-key lines
obs-scope-summary#18.1 · not in scope · standing S1 · every new log line goes through `obs_event!` under `#[instrument(skip_all, fields(..))]`
obs-scope-summary#18.3 · carried · CLI `send`'s client-side paste refusal and a refused name must log only into a resolved instance's `cli` file, none when no instance resolves
obs-scope-summary#18.4 · carried · the log `instance` is a `ViolaName` from an argument or `VIOLA_NAME`, the names-before-path-joins rule the chunk re-verifies
obs-scope-summary#18.5 · carried · its instance note (never from the working directory) is the names rule the chunk re-verifies
obs-scope-summary#19.1 · carried · the vt100 readiness-gate outcome and the channel server handlers are instrumented sync, and the degrade witness reads that outcome
obs-scope-summary#19.2 · carried · the terminal carries the child's screen, the passthrough the vt100-panic degrade must leave intact
obs-scope-summary#19.3 · carried · `run` writes nothing to the terminal while the child runs, which the forced vt100 panic must keep
obs-scope-summary#19.4 · not in scope · `run`'s service identity is unchanged (the task's Not needed: service identity)
obs-scope-summary#19.5 · carried · the replaced panic hook keeps a forced vt100 panic off the child's terminal
obs-scope-summary#20.1 · carried · the hook's stdin parse is the reader the chunk bounds, its fail-open `hook-decision` and caught-panic line required
obs-scope-summary#20.3 · carried · hook stdout carries only the decision body or the statusline output, so an oversize stdin prints nothing and the bounded statusline stdout passes through
obs-scope-summary#20.4 · carried · the hook's instance check precedes its stdin read, and a missing or refused name is a silent no-op
obs-scope-summary#20.5 · carried · a `VIOLA_DIR` naming no existing instance directory resolves none and writes nothing, the names rule at the hook
obs-scope-summary#21.1 · carried · channel params readers skip unknown fields, which the closed-type channel params reading must keep
