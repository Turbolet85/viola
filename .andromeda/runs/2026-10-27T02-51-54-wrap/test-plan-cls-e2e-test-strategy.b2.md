# Classified — test-plan · e2e-test-strategy.b2

## Answers
e2e-test-strategy#6.4 · no · the Path 5 cleanup line names nothing the session moved, and the report keeps Paths 2 to 6 unchanged
e2e-test-strategy#7.1 · no · the Path 6 steps rest on budget behaviour the report keeps unchanged for Paths 2 to 6
e2e-test-strategy#7.2 · no · the Path 6 verification signals, the step 6 strict-modes and replayed-snapshot refusals included, hold: the report keeps Path 6 budget behaviour unchanged and CI is green 15/15
e2e-test-strategy#7.3 · no · the as-landed Path 6 cases, the mcp_tools.rs budget-paused MCP case included, hold on the changed tree: the report keeps Path 6 budget behaviour unchanged and CI is green 15/15
e2e-test-strategy#7.4 · no · the Path 6 cleanup line (harness cleanup) names nothing the session moved
e2e-test-strategy#8.1 · yes · add-to · F4: Path 7's MCP leg (the rmcp answer of step 3) now boots the default overseer + builder pair unstamped, not builder alone; step 1's CLI boot is unchanged
e2e-test-strategy#8.2 · yes · add-to · F4: the MCP leg's green run also witnesses two first starts on one fresh unstamped home both landing (the pinned-copy race), green on all three CI OSes
e2e-test-strategy#8.3 · no · the Path 7 cleanup line (harness cleanup and TempDir drop for H3) names nothing the session moved
e2e-test-strategy#9.1 · no · the E2 steps hold: the report keeps env-strip behaviour unchanged, and the env-name receipt is not the hook receipt the session extended
e2e-test-strategy#9.2 · no · the E2 verification signals hold: the report keeps env-strip behaviour unchanged, and CI is green 15/15
e2e-test-strategy#9.3 · no · the E2 cleanup line (TempDir drop) names nothing the session moved
e2e-test-strategy#10.1 · no · E3 link and unlink behaviour is unchanged per the report's untouched list, and CI is green 15/15
e2e-test-strategy#11.1 · no · E5 replay behaviour is unchanged per the report's untouched list, and CI is green 15/15
e2e-test-strategy#12.1 · no · the sweep's MCP list and violation cases hold on the changed tree per CI green 15/15 on all three CI OSes (ci#40439188053), and the report names no change to the cases it cites
e2e-test-strategy#13.1 · no · the error-sanitization suite is untouched per the report: no case added or moved, and no log line, field or detail code added
e2e-test-strategy#14.1 · no · the exit-cause rows hold: the pinned-exe hash is unchanged by the new arm and the §1 trigger line stays true, the run.pin_copy detail stays ok or pinned-hash-mismatch, and the exit-cause suites are untouched
e2e-test-strategy#15.1 · no · the protected-DACL, strict-modes and create_private_dir claims hold per CI green 15/15 on all three CI OSes, and its BUILTIN\Users example of an untrusted SID is one the report leaves unmoved
e2e-test-strategy#16.1 · no · the list-row integrity suite is untouched per the report: no case added or moved
e2e-test-strategy#17.1 · no · the chaos suite is untouched per the report: no case added or moved, and CI is green 15/15
