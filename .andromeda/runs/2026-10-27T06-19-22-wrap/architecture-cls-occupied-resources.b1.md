# Classified — architecture · occupied-resources.b1

## Answers
occupied-resources#1.1 · no · the socket path, the 0700 socket dir, the start lock and the recorded endpoint are untouched, since the report says no endpoint or channel transport changed and the stdin cap sits on stdio, not the socket
occupied-resources#2.1 · no · the server check before a frame and its exit 21 or fail-open paths are untouched, since the report leaves channel transport and session liveness unchanged
occupied-resources#3.1 · no · the three test-only binaries (fake-agent, harness, tap) and their argv are unchanged, and the new contract_ file is a root integration test, not one of them
occupied-resources#4.1 · yes · add-to · the mcp verb now also ends at a stdin line past MAX_FRAME, as a closed stdin does, exiting 0 once initialized and 11 when the cut lands before initialize
occupied-resources#5.1 · no · link and unlink, their instance checks and their exit codes are untouched, since the report names links and the exit-code table as unchanged
occupied-resources#6.1 · no · send, wait, last, answer, pause and release keep their instance checks and their exit 21 and exit 1 paths, since the session changes no verb and no exit code
occupied-resources#7.1 · no · the hidden hook capture flags and their bounds are untouched, since the report names hook transport and the hook contract as unchanged
occupied-resources#8.1 · no · the child's flags, the resume argument shape and the plugin dir path are unchanged, since the session adds no flag and no spawn
occupied-resources#9.1 · no · verify's probe plugin, its five runs and its ledger/probes paths are unchanged, since no ledger row was added or re-stamped
occupied-resources#10.1 · no · run still rewrites settings.json through replace_private at 0600, and the report re-reads replace_private as unchanged, only the compare in front of the replace being bounded
occupied-resources#11.1 · no · VIOLA_DIR and its readers are unchanged, since the session adds no env var and no CLI flag
occupied-resources#12.1 · no · hook's instance check before any read and its two capture arms are untouched, since the report names hook transport and the hook contract as unchanged
occupied-resources#13.1 · no · the R8 strip, the identity floor and the names-only registry reader are unchanged, since no env var was added
occupied-resources#14.1 · no · the harness env vars and their named readers (tests/support/home.rs, the harness_lifecycle test, viola-e2e's harness::session) are untouched, since the session adds no env var and names none of the reached test files as a reader
occupied-resources#15.1 · no · the two fake-agent test seams and their read sites are unchanged, since the session adds no seam and no env var
occupied-resources#16.1 · no · the config.json keys, caps and defaults are unchanged, since the report adds no config key
occupied-resources#17.1 · yes · stale · fs.rs is reached whole, the [Snapshot writer] decision it cites was amended, and the report does not show place_private_new's no-clobber placement still holds on the changed file
occupied-resources#18.1 · yes · add-to · mcp.ndjson can now carry one parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1} WARN line for a cut session, and the schema's parser enum grows to hold mcp-stdin
occupied-resources#19.1 · no · the state-recovered lines and the snapshot replay readers are unchanged, since the report names no snapshot field or reader change
occupied-resources#20.1 · no · open_role_file and write_detail in obs.rs and their check_diagnostics are unchanged, since the new mcp line only uses the same writer
occupied-resources#21.1 · no · the instance files and the entry points that check them are unchanged, since the report leaves the snapshot contracts and session liveness unchanged
occupied-resources#22.1 · no · the strict read and write rules, which live in strict.rs and in no reached file, are unchanged, since the session adds no check
occupied-resources#23.1 · no · check_home_tree, checked_exe and their call order at run and revive starts are unchanged, since the session moves no home-tree call site
occupied-resources#24.1 · no · the instance checks before frames and starts are unchanged, and the mcp frame tools' check lives in tools.rs, which is not reached
occupied-resources#25.1 · no · the strict-modes reading of a home under a regular file is unchanged, since the session changes no strict-modes check
