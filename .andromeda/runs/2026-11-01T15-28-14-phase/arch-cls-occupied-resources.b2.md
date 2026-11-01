# Classified — arch · occupied-resources.b2

## Answers
occupied-resources#26.1 · yes · the instance check runs before any snapshot read in the frame verbs, hook, mcp and the run/revive starts, so the new socket-directory and euid checks must sit after it and before the first frame or any log
occupied-resources#27.1 · no · a Windows stat fact for tests that plant a regular file above a path; the chunk's socket-directory checks are Unix-only and its tests plant modes, owners and symlinks, not a regular file
occupied-resources#28.1 · yes · board::scan's home check is the unit the chunk edits, and the chunk makes it one rule, reconciling the write rule on the home with the read rule per instance
occupied-resources#29.1 · no · budget.json's read order belongs to read_budget_stamped, which the chunk neither edits nor names; it changes board::scan's home check only
occupied-resources#30.1 · no · ledger/stamps.json and run's version gate are not reached or named by the chunk
occupied-resources#31.1 · no · verify's ledger/probes dirs; the chunk does not reach verify
occupied-resources#32.1 · no · verify's interactive probe dirs outside the home; the chunk does not reach verify
occupied-resources#33.1 · yes · the Unix holder_gone connect probe (connect refused or path absent) is the re-read check and the root kill path the chunk re-measures, so its rule binds
occupied-resources#34.1 · yes · the error-body check's classes are the witness for the sanitised surfaces each new refusal must carry (no path, pid or id in a role line, stdout or stderr)
occupied-resources#34.2 · yes · the four checks over cross_exit_causes.rs's table rule over the rows the chunk edits, so each new row keeps no shared hint, no release name and a code word
occupied-resources#35.1 · no · the Linux dev-host tmpfs backing of target/e2e-home; the chunk's fallback is /tmp/viola-<uid>/, the chunk creates no e2e home, and cleanup's session-home removal is outside its re-read check
occupied-resources#36.1 · no · committed hook, screen, dialog and MCP fixture sets; the chunk adds no fixture or external-bytes parser and changes no recorded set
occupied-resources#37.1 · no · the target/agent-run session record and bin files; not reached, and cleanup's rule over them is not the chunk's re-read check
occupied-resources#38.1 · no · standing S1 · each new test's own viola home under target/e2e-home, with its owner record beside it
occupied-resources#39.1 · no · standing S1 · each new test's home kept for the obs gates and secret scan through AGENT_RUN_KEEP_HOMES on each job
occupied-resources#40.1 · no · the Windows seeded-home ACL inheritance; the chunk changes no Windows behaviour
occupied-resources#41.1 · no · the profraw census script's outputs; no part of the chunk reaches it
occupied-resources#42.1 · no · the Windows and dev-host mutation scratch; the chunk runs no mutation pass and reaches no scratch
occupied-resources#43.1 · no · the Windows protected DACL of a home; no Windows behaviour moves
occupied-resources#43.2 · no · the Windows inherited temp-dir ACL per runner; no Windows behaviour moves
