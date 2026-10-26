# security-plan · picked accounting

## Picked
threat-model-summary#2.1 · proposed · F3 belongs beside its per-OS enforcement line: the Windows pipe's remote-client rejection and DACL effect are now witnessed on hosted CI, the cross-user parts still unexercised
authentication-authorization#1.3 · proposed · its accept_remote(false) clause and protected DACL (line 15, the report's remote-rej site) gain their CI witnesses, F1 and F2
input-validation#1.4 · not touched · src/cmd/hook/statusline.rs changed only at its test capture (375); the report says no product file, refusal or output moved, so the hook-stdin rule still holds
input-validation#1.6 · not touched · src/main.rs gained only the cfg(test) helper with_scoped_default and its witness; the clap derive and home resolution the row names did not move
input-validation#1.7 · not touched · src/run/budget.rs, src/run/dialog.rs and src/obs.rs changed only in a scoped test capture each; read_thresholds, read_hold and the config rejection line are unchanged
input-validation#1.9 · not touched · src/cmd/run.rs changed only at test captures 772-775 and 912; hold_pump_start and its 5 000 ms cap are unchanged
input-validation#1.12 · not touched · the edits are inside test modules, so the five read_snapshot( callers outside test modules and every state-file read stay as stated
input-validation#1.15 · not touched · src/run/gate.rs changed only at test capture 465 (its set_global_default at 429 untouched); the tee, feed thread, catch_unwind and FEED_CAPACITY are unchanged
input-validation#1.16 · not touched · no file it names (src/run/mod.rs, src/cmd/revive.rs, viola-agent-claude) changed, and src/cmd/run.rs moved only in test captures, so resolution and the cwd rule hold
input-validation#1.18 · not touched · src/cmd/run.rs moved only in test captures; statusline_source / statusline_command and the default-home rule are unchanged
input-validation#1.19 · not touched · src/cmd/hook/statusline.rs moved only at test capture 375; the /bin/sh -c shell-out, its order and STATUSLINE_DEADLINE are unchanged
input-validation#1.20 · not touched · src/main.rs's additions are a cfg(test) helper and test (273-289, 610-651), not main's statements, so restrict_dll_search stays the second statement
bootstrap-phases#1.1 · proposed · its viola-channel per-OS listener hardening item gains the Windows CI witnesses for remote rejection and the DACL's effect
error-handling#1.1 · not touched · hook's fail-open, no-stderr, exit-0 rule is unchanged; src/cmd/hook/statusline.rs moved only in a test capture
