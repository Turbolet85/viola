# Classified — tests · e2e-test-strategy.b2

## Answers
e2e-test-strategy#6.4 · no · Path 5's cleanup (Ctrl-C, child wait, TempDir drop) states no rule over the socket, euid or home checks the chunk adds
e2e-test-strategy#7.1 · no · Path 6's budget steps exercise governor and statusline behaviour the chunk does not change; the owning user's frames pass the new euid check
e2e-test-strategy#7.2 · no · Path 6's verdicts (budget.json, budget-gate, send exit 11, release, the statusline strict-modes arm) are governor and statusline rules the chunk leaves unchanged, with no time bound stated
e2e-test-strategy#7.3 · no · the as-landed Path 6 cases witness budget and board-cell readings, not the board's home rule or any socket or euid rule
e2e-test-strategy#7.4 · no · Path 6's cleanup is an ordinary harness cleanup with no force-kill, the only case the chunk measures on cleanup.rs
e2e-test-strategy#8.1 · no · the Path 7 boot and step commands state no rule the chunk's checks reach; the unverified-cli gate is wrapper-side
e2e-test-strategy#8.2 · yes · the hook's exit-0, empty-stdout return "within the spine deadline" is a time bound the added connect checks must stay inside under SPINE_DEADLINE
e2e-test-strategy#8.3 · yes · the as-landed MCP witness asserts two first starts on one fresh home both landing, so the new wrapper-bind directory check must admit the directory the first start made
e2e-test-strategy#8.4 · no · Path 7's cleanup is a harness cleanup and a TempDir drop, with no force-kill and no Unix cleanup rule stated
e2e-test-strategy#9.1 · no · the R8 env-strip run states env names and the child's inputs, which the chunk's bind and connect checks do not change
e2e-test-strategy#9.2 · no · the child-holds-nothing rule is held by the spawn's fd close and the env strip, not by the chunk's checks, which run before any child
e2e-test-strategy#9.3 · no · TempDir drop cleanup, with no rule the chunk touches
e2e-test-strategy#10.1 · no · the E3 link and unlink rules are link-record and snapshot rules the chunk does not change, and a client-side refusal sends no frame and so records no link
e2e-test-strategy#11.1 · yes · the replay leg runs through board::read (the `viola list` home check the chunk edits) and through hook connect under the new connect checks, and its e5 list case is a board case of tests/cli_list.rs named in scope
e2e-test-strategy#12.1 · yes · the hook fail-open matrix gains the refused-directory and euid-mismatch rows and must hold within the spine bound, as the scope's hook bullet names
e2e-test-strategy#13.1 · yes · the new refusals' CLI --json, MCP isError and error.data bodies must carry no absolute path, and bind_endpoint's refusal replaces an internal-error path, so the -32603 and sanitization witnesses bind
e2e-test-strategy#14.1 · yes · the exit-cause table gains the socket-directory and euid-mismatch rows, and its one-negative, shared-hint, no-release and top-comment rules bind them
e2e-test-strategy#15.1 · yes · the Unix socket directory refusal, umask-independent modes, single-writer and server-identity controls, and the macOS carve-out case beside security_negatives_server_identity.rs, are the chunk's own negatives
e2e-test-strategy#16.1 · no · the list-row integrity case checks escaped NAME cells and the caption layout, which the chunk leaves alone; its change to list is the home check, not the rows
