# Classified — arch · conventions.b1

## Answers
conventions#1.1 · no · the closed RefusalReason set and the send / answer refusal order; the chunk touches no send or answer path and adds no refusal reason (the wrapper channel is Not needed)
conventions#2.1 · yes · run's start refusals: the pinned-copy re-hash refusal (exit 1, pinned-hash-mismatch, file left as found) that the absent-copy fix must keep, and the exit-1 internal error a new absent-copy failure maps to on run and verify's pin_exe call, the race fix adding no exit code outside this table
conventions#3.1 · yes · run's start home-tree check before any log (checked_exe then refuse_tree over bin/, bin/<key>/, the pinned viola, plugin/<key>/ with hooks/ and .claude-plugin/), named by the scope: run must keep pinning the bytes it checked, and a new .lock or temp entry under bin/<key>/ or plugin/<key>/ must pass that write rule
conventions#4.1 · no · verify's pre-log home-tree refusal (check_home_tree with no name and no key over home, ledger/, bin/, plugin/) and its update_stamps refusal sit in units of verify.rs the change does not edit (only the pin_exe call is edited), so a re-check every change owes
conventions#5.1 · no · exit 21 for a server that is not the recorded wrapper on the frame verbs; the chunk sends no frame, changes no method and leaves the server identity check untouched (Not needed: the wrapper channel)
conventions#6.1 · no · exit 21 for the refused strict-modes instance check of the frame verbs (client::refuse_untrusted); the chunk reaches no frame verb or instance check (Not needed: the wrapper channel)
conventions#7.1 · no · MCP result shapes and their isError refusal documents; the chunk changes no MCP tool or result shape (Not needed: MCP tools and MCP result shapes)
conventions#8.1 · no · a frame tool's instance-unreachable result on a server-identity mismatch; the chunk changes no frame tool and no server identity check (Not needed: the wrapper channel)
conventions#9.1 · no · a frame tool's check_instance before live_endpoint; the chunk touches no frame tool or instance check, and its new lock and temp entries sit under bin/ and plugin/, not instances/
