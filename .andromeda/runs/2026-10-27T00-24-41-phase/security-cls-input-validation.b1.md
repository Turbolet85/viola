# Classified — security · input-validation.b1

## Answers
input-validation#1.1 · no · the paste-text rules live in send.rs, run/send.rs and viola-core, and the task changes no send path
input-validation#1.2 · no · the dialog free-text rules sit in viola answer and the answer handler, neither of which the pin fix reaches
input-validation#1.3 · no · the channel frame rules are untouched and the chunk adds no frame path
input-validation#1.4 · yes · the scope names viola hook's fail-open stance (exit 0, no body, no stderr) and the fix may add no hook-side refusal, while the hook's stdin read rules stay untouched
input-validation#1.5 · no · MCP tool-input validation in viola-mcp is untouched and no viola mcp tool path changes
input-validation#1.6 · yes · the home resolution and the strict-modes check before any read or log bind, since the scope re-asserts check_home_tree before the pin and before any log for run, revive's start and verify
input-validation#1.7 · no · the config.json reads sit in src/run/budget.rs, src/run/dialog.rs, src/run/env.rs and viola::obs, units the chunk does not edit
input-validation#1.8 · no · the Windows registry name read is src/run/env.rs under the R8 rule, which the task leaves untouched
input-validation#1.9 · no · the pump-delay seam is hold_pump_start in src/cmd/run.rs, a unit the chunk does not edit, and it acts only after the child starts, past the absent-copy write
input-validation#1.10 · no · the panic seam is fake-agent only in seam.rs, the chunk forces no hook panic, and the fix does not reach hook()
input-validation#1.11 · no · the colour facts are read only by src/human/style.rs for viola list's human mode, which the chunk does not touch
input-validation#1.12 · yes · the strict-modes check before any read of ledger/stamps.json (read strictly) and of snapshot.json (pinned_bin, keyed by the build hash) binds, while the events, budget and link readers stay untouched
input-validation#1.13 · no · notify tailing of events.ndjson is not reached by the pin fix or its witness
input-validation#1.14 · no · the version probe, claude agents --json and verify's PTY output sit in units the chunk does not edit, since verify edits only its pin_exe call
input-validation#1.15 · no · the vt100 tee and feed thread in src/run/gate.rs are not among the edited units and act on the pump after the pin write
input-validation#1.16 · no · claude program resolution and the .cmd/.bat refusal are untouched, and the pinned exe is spawned from its absolute hooks.json path rather than resolved here
input-validation#1.17 · no · the session-id argv of revive and verify's Run E is untouched and the chunk changes no argv
input-validation#1.18 · no · the statusline source read and its statusline_command record sit in run.rs units the chunk does not edit, and the task names the statusline untouched
input-validation#1.19 · no · the statusline shell-out is the hook statusline arm, which the chunk does not reach, and the task names it untouched
