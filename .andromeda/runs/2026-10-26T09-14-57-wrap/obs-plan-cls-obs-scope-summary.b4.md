# Classified — obs-plan · obs-scope-summary.b4

## Answers
obs-scope-summary#22.3 · no · the mcp sink, its home resolution and its strict-modes check are unchanged, and a mismatch writes no mcp.ndjson line
obs-scope-summary#22.4 · no · the mcp process and instance identity fields are unchanged by the check
obs-scope-summary#22.5 · no · the isError code set is unchanged, since a mismatch reuses instance-unreachable with no log line
obs-scope-summary#23.1 · no · the process init order and the panic hook are not reached by the check
obs-scope-summary#24.1 · no · the logging stack, the sink rules and the single-write writer are unchanged
obs-scope-summary#25.0 · yes · contradict · a dialog hook's mismatch writes hook-invoked and hook-decision with corr null, against the rule that corr is dialog_id for dialog hooks
obs-scope-summary#26.1 · no · the diagnostics file locations, the modes and the absence of rotation are untouched, and the new lines go to existing files
obs-scope-summary#27.1 · no · the run surfaces are unchanged, since run's collision check still reads the snapshot with no server check
obs-scope-summary#27.2 · no · run's must-trace span chain is unchanged, since the run start sequence is not reached
obs-scope-summary#27.3 · no · run's exit-1 causes and details are unchanged, and server-verify-failed is an exit-21 CLI cause
obs-scope-summary#27.4 · no · the run critical-path sources still hold, since run is not changed
obs-scope-summary#28.1 · no · the send surface chain is unchanged, since the check runs inside the existing client before the channel
obs-scope-summary#28.2 · no · the framed send path keeps send.client, channel.request and its log order, and the check adds no span name
obs-scope-summary#28.3 · no · the send required fields cover send and channel lines, which a pre-frame mismatch never writes
obs-scope-summary#28.4 · no · the send critical-path sources still hold
obs-scope-summary#29.1 · no · the wait surface chain is unchanged
obs-scope-summary#29.2 · no · the framed wait path keeps wait.block and channel.request, and the check sits inside wait.block
obs-scope-summary#29.3 · no · the wait required fields cover channel lines that a pre-frame mismatch never writes, and exit 21 for a killed wait still holds
obs-scope-summary#29.4 · no · the wait critical-path sources still hold
obs-scope-summary#30.1 · no · the dialog surface chain is unchanged, since the check precedes hook.dialog with no new surface
obs-scope-summary#30.2 · no · the framed dialog path keeps its must-trace chain and event set, and no span name is added
obs-scope-summary#30.3 · yes · contradict · the mismatch hook-invoked and hook-decision lines carry corr null, against the required corr dialog_id on dialog lines
obs-scope-summary#30.4 · no · the dialog critical-path sources still hold
obs-scope-summary#31.1 · no · the wheel surface chain is unchanged, since pause and release add only a pre-frame check
obs-scope-summary#31.2 · no · the framed pause and release path keeps its must-trace chain, and the check sits inside pause.client and release.client
