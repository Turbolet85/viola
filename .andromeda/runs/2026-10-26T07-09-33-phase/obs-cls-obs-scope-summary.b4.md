# Classified — obs · obs-scope-summary.b4

## Answers
obs-scope-summary#22.3 · yes · the mcp instance-unreachable mirror line goes to the <home>/diagnostics/mcp.ndjson file sink, never stdout (MCP frames)
obs-scope-summary#22.4 · yes · the mirror line's process is mcp and its instance the server's own VIOLA_NAME when set, which is also the from a send adds
obs-scope-summary#22.5 · yes · instance-unreachable is an isError code only, its mirror line codes-only with no path, upstream text or type name, as the scope names (Vector 5)
obs-scope-summary#23.1 · yes · viola-channel and viola-state stay sync and Tokio-free at the edited client, endpoint and liveness units, and stdout stays reserved on hook and mcp for the fail-open and refusal paths
obs-scope-summary#24.1 · yes · one file per role with no stdout and no hook stderr, which the hook fail-open line (hook-<name>.ndjson only, stdout and stderr empty) and the mcp mirror line both keep
obs-scope-summary#25.0 · yes · the new channel-request/response, process-exit, hook-decision and send-refused lines carry the corr each event fixes (JSON-RPC id, dialog_id, send cursor or null), never renamed
obs-scope-summary#26.1 · yes · the codes-only mcp.ndjson mirror line sits in the home-level file, so it carries no user content; the hook-<name> and cli-<name> lines land at the basenames it fixes
obs-scope-summary#27.1 · no · surfaces only: the run channel bind is server side and unchanged, and this line states no rule over the collision check's re-read
obs-scope-summary#27.2 · no · run start spans; the chunk changes no run span, and the collision check's read_snapshot re-read is not recorded by any of them
obs-scope-summary#27.3 · yes · already-live is the collision check's exit-1 cause, decided on the read_snapshot outcome the chunk re-reads; its detail carries no path or pid (S4)
obs-scope-summary#27.4 · no · source line for the run start scenario, no rule of its own
obs-scope-summary#28.1 · yes · the cli or mcp send to channel hop the pre-check sits on; the run, hook and events hops are unchanged
obs-scope-summary#28.2 · yes · the send.client span and channel-request order: a refusal before the frame writes no channel-request line
obs-scope-summary#28.3 · yes · send-refused gains the verification detail for a pre-frame refusal, codes-only with no sent text, and its corr (the send cursor) must be decided for a refusal with no cursor
obs-scope-summary#28.4 · no · source line for the send scenario, no rule of its own
obs-scope-summary#29.1 · yes · the cli or mcp wait to channel hop the pre-check sits on; the run and events hops are unchanged
obs-scope-summary#29.2 · yes · the wait.block span and channel-request order: a pre-frame refusal writes no channel-request line
obs-scope-summary#29.3 · yes · outcome instance-unreachable with exit_code 21 on a failed server verification, the wait failure lines the chunk changes, and last_assistant_message never appears
obs-scope-summary#29.4 · no · source line for the wait scenario, no rule of its own
obs-scope-summary#30.1 · yes · the hook.dialog client and the answer pre-check on the hook to channel and mcp answer hops; the run hop is unchanged
obs-scope-summary#30.2 · yes · hook-invoked is written only after the hook.dialog response, which a refused verification never returns, so the fail-open path writes no hook-invoked
obs-scope-summary#30.3 · yes · the dialog hook-decision carries corr = dialog_id on its fail-open line and the detail set gains the verification value; question, answer and input text never appear
obs-scope-summary#30.4 · no · source line for the dialog scenario, no rule of its own
obs-scope-summary#31.1 · yes · the pause and release clients' pre-check and the mcp send refusal on the cli release to channel hop; the run and events hops are unchanged
obs-scope-summary#31.2 · yes · a pre-frame pause or release refusal writes no channel-request or channel-response line
