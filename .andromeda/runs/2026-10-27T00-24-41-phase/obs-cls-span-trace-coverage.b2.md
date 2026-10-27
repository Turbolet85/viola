# Classified — obs · span-trace-coverage.b2

## Answers
span-trace-coverage#7.1 · no · the Dialog → answer scenario is named not needed (no dialog path changes), and the pin write reaches no dialog frame or instance check
span-trace-coverage#7.2 · no · dialog span attributes; the Dialog → answer scenario is named not needed
span-trace-coverage#7.3 · no · dialog log fields; the Dialog → answer scenario is named not needed, and the chunk changes no hook path, so the fail-open lines it states stay the hook's own
span-trace-coverage#7.4 · no · dialog cleanup at the hold's end; the Dialog → answer scenario is named not needed
span-trace-coverage#8.1 · no · wheel transition span and pause/release client spans; the wheel/release scenario is named not needed (no wheel change)
span-trace-coverage#8.2 · no · run.wheel_transition attributes; the wheel/release scenario is named not needed
span-trace-coverage#8.3 · no · wheel and release log fields, send-refused and release-from-driver included; the wheel/release scenario is named not needed
span-trace-coverage#8.4 · no · run.wheel_transition point-in-time cleanup; the wheel/release scenario is named not needed
span-trace-coverage#9.1 · no · budget governor must-trace spans; the Budget governor scenario and the statusline shell-out are named not needed
span-trace-coverage#9.2 · no · budget span attributes, run.budget_eval at start included; the Budget governor scenario is named not needed, and the chunk touches no budget gate
span-trace-coverage#9.3 · no · statusline and budget.json log fields; the statusline arm and budget gate are named not needed
span-trace-coverage#9.4 · no · statusline.shell_out deadline cleanup; the statusline shell-out is named not needed
span-trace-coverage#10.1 · yes · the R8-strip fields env_stripped_count, env_stripped_known and env_kept ride the process-start{subject:"claude-child"} line the Needs name beside sideload_fallback, inside the start-to-spawn scenario the scope names
span-trace-coverage#11.1 · no · the E3 implicit link record and its channel pair; E3 link/unlink is named not needed (no state appender changes)
span-trace-coverage#12.1 · no · the E3 explicit link and unlink methods; E3 link/unlink is named not needed
span-trace-coverage#13.1 · no · standing S3 · a new span is named `<area>.<operation>` with a static name and an explicit field allow-list, never bare `#[instrument]`, `err` or `ret` (G1)
span-trace-coverage#14.1 · no · torn-line healing on the events.ndjson appender; named not needed, and the chunk changes no appender
span-trace-coverage#15.1 · no · the capability ledger read in run.version_gate and cli_verified; named not needed, the stamps are untouched
span-trace-coverage#16.1 · yes · verify's process-start and process-exit pairs to cli-<name>.ndjson, their detail codes and its home-tree check before the log, which the verify pin_exe call and the Needs name
