# Classified — security · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · yes · the link and unlink lines append to events.ndjson, which is never rotated or truncated, and carry the two names and a timestamp only, never prompt or reply text
threat-model-summary#1.2 · no · the chunk reads and writes no permission input, plan, question or option text, and hook.dialog stays unchanged
threat-model-summary#1.3 · yes · the wrapper's snapshot.json rewrite that adds the links items must keep statusline_command, endpoint and pinned_bin unchanged and go through replace_private
threat-model-summary#1.4 · no · standing S1 · the NEVER-log floor, no R8-stripped CLAUDE* value in any log, diagnostic, event, snapshot, fixture or error body, a convention the task lists
threat-model-summary#1.5 · yes · the verbs read pid and started_at for the server check, and the exit-21 unreachable line holds no pid, start time, path or endpoint
threat-model-summary#2.1 · yes · the chunk serves link and unlink on the one endpoint, their from is self-reported and never an authenticated identity, and both verbs take the server check before their first frame
threat-model-summary#3.1 · no · a link carries no typed text, so the send path with its paste normalisation and delivery confirmation stays unchanged
threat-model-summary#4.1 · no · the chunk changes neither the human's stdin classifier nor the wheel, and link and unlink are not sends
threat-model-summary#5.1 · no · the chunk changes no hook arm or statusline path, so hook stdin parsing, the fail-open path and the statusline shell-out stay as they are
threat-model-summary#6.1 · yes · link and unlink stay off the MCP tool list, and v1-13's MCP wording must call that an affordance, not an enforcement
threat-model-summary#7.1 · yes · the new verbs take --home and VIOLA_DIR, and their driver and driven arguments pass clap and ViolaName::try_new before any path join
threat-model-summary#8.1 · yes · the link and unlink lines append to events.ndjson through its append that heals a torn last line, under the home's 0600 rule, and the verbs take no log replay, which only revive's preflight and list may take
threat-model-summary#9.1 · no · the verbs spawn no child, and no spawn path, PATH resolution or MCP tool list changes
threat-model-summary#10.1 · no · the chunk adds no crate, lockfile entry, action or toolchain, so the cargo deny gates are a re-check every change owes
threat-model-summary#11.1 · no · the chunk adds no CI job, workflow or OS-only check, so the matrix runs its new tests as it runs every test
