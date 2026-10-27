# Classified — obs-plan · obs-scope-summary.b4

## Answers
obs-scope-summary#22.3 · yes · stale · src/obs.rs now runs the diagnostics/ read-rule check on the mcp.ndjson open before the append, so the claim's strict-modes reading moved from owed to built and a refused open is a failed open.
obs-scope-summary#22.4 · no · the claim still holds on the changed tools.rs and obs.rs: process mcp and instance from VIOLA_NAME are unchanged, and the new check adds a refusal, not an identity field.
obs-scope-summary#22.5 · yes · add-to · instance-unreachable now also covers a frame tool's refused check_instance with no line, and F12 adds that a replayed snapshot has no endpoint, so live_endpoint is None and the MCP result is instance-unreachable with detail null.
obs-scope-summary#23.1 · yes · add-to · F4 and F9 put the six verbs' and verify's home checks before the step 4 diagnostics open, F5 puts the hook's canonical then trusted check before read_diagnostics_level and obs init, and F6 makes a VIOLA_DIR naming no existing instance directory resolve to no session at step 3.
obs-scope-summary#24.1 · yes · stale · open_role_file now runs the diagnostics/ check before the append open, so a refused mcp.ndjson is a failed open that the stderr-only-when-cannot-be-opened rule now reaches.
obs-scope-summary#25.0 · no · the claim still holds on the changed diag-line.v1.json: the report's §8 line says no log field and no event value moved, and the one schema change is a sideload_fallback enum word.
obs-scope-summary#26.1 · yes · add-to · the 0600-in-0700 rule must now carry the read rule as built (no group or other bit on Unix, no read grant to another SID on Windows) and the symlink refusal on diagnostics/ and its files.
obs-scope-summary#27.1 · yes · add-to · run's start now reads the home tree (ledger/, bin/, plugin/, the pinned exe) under strict-modes before its log opens, a persistent store this surfaces list does not carry.
obs-scope-summary#27.2 · yes · add-to · the run span list must now carry state.snapshot_recover under run.start before run.collision_check, and the home-tree check ahead of open_wrapper_log.
obs-scope-summary#27.3 · yes · add-to · F4 (the six verbs), F5 (the hook event and dialog paths), F9 (verify) and F13 (every new refused check writes no line) join this no-line exemption list, and run and revive's refused home tree exits 1 before the log with no line and no detail code.
obs-scope-summary#27.4 · no · a source citation (tests Critical Path 1, arch hint 1, Founder Direction 4) that no reached file carries and that the session moves no text of.
obs-scope-summary#28.1 · yes · add-to · F4 (send first runs refuse_untrusted on the home and instances/<name> before the channel) and F5 (the UserPromptSubmit and SessionStart hook hops take canonical then trusted before obs init) extend the surfaces chain.
obs-scope-summary#28.2 · no · the claim still holds on the changed send.rs: the refused check runs before the channel request and adds no span or event to the accepted chain.
obs-scope-summary#28.3 · no · the claim still holds on the changed send.rs: F4's refusal writes no line, so the send line's required fields and detail list are unchanged.
obs-scope-summary#28.4 · no · a source citation (tests Critical Path 2, arch hint 2, Founder Direction 5) that no reached file carries and that the session moves no text of.
obs-scope-summary#29.1 · yes · add-to · F4 (wait, and last which shares this surface, first runs refuse_untrusted on the home and instances/<name> before the channel and exits 21 with no line) extends the surfaces chain.
obs-scope-summary#29.2 · no · the claim still holds on the changed wait.rs: the refused check precedes the channel request and adds no span, so wait.block › channel.request › run.wait_dispatch holds on the sent path.
obs-scope-summary#29.3 · no · the claim still holds on the changed wait.rs: outcome and exit_code are line fields and F4's refusal writes no line, so the wait line's required fields are unchanged.
obs-scope-summary#29.4 · no · a source citation (tests Critical Path 3, arch hint 3, design excerpt, creator must-work) that no reached file carries and that the session moves no text of.
obs-scope-summary#30.1 · yes · add-to · F4 (answer first runs refuse_untrusted before the channel, exiting 21 with no line) and F5 (the dialog hook takes canonical then trusted before obs init) extend the answer and hook surfaces.
obs-scope-summary#30.2 · no · the claim still holds on the changed hook.rs and answer.rs: the refused hook check runs before obs init so no hook.handle span opens for it, and the accepted chain is unchanged.
obs-scope-summary#30.3 · yes · add-to · F12 adds that a replayed snapshot's dialog hook connect is NotSent::Unreachable, failing open with detail channel-unreachable and decision_emitted false, beside the server-verify-failed fail-open.
obs-scope-summary#30.4 · no · a source citation (tests Critical Paths 4 and 7, arch hint 4, creator must-work) that no reached file carries and that the session moves no text of.
obs-scope-summary#31.1 · yes · add-to · F4 (pause and release first run refuse_untrusted on the home and instances/<name> before the channel, exiting 21 with no line) extends the pause and release surfaces.
obs-scope-summary#31.2 · no · the claim still holds on the changed pause.rs, release.rs and diag-line.v1.json: refusals write no line and no channel request, and the schema change is an enum word that leaves release-from-driver's admission alone.
