# Classified — tests · e2e-test-strategy.b1

## Answers
e2e-test-strategy#1.1 · yes · the cli driver, assert_cmd with predicates, that the task names for the CLI --json witnesses; its trycmd and TTY human-layout parts are dropped
e2e-test-strategy#1.2 · yes · the status-line witness runs through viola hook statusline under assert_cmd with an Instant deadline, the verb and driver the task names; its env_clear sentence stays under S3
e2e-test-strategy#1.3 · no · the outer-PTY passthrough driver serves none of the three surfaces the chunk reaches; the Path 1 watch it also drives is a timing read only
e2e-test-strategy#1.4 · yes · the wrapper-channel driver, a raw interprocess stream with jsonschema whose endpoint is read from snapshot.json, which the task names for the channel witnesses
e2e-test-strategy#1.5 · yes · the MCP stdio driver, an rmcp client and an owned viola mcp child with its raw stdin writer, which the task names for the stdio path and the stdin-before-initialize test
e2e-test-strategy#1.6 · no · a re-check every change owes: the library-only runner (cargo-nextest, cargo test --doc), whose cases §4 covers
e2e-test-strategy#2.1 · no · the Path 1 flow steps, which the chunk does not change; the watch needs only the wall read in 2.3
e2e-test-strategy#2.2 · no · Path 1 outcome signals and run exit codes the chunk does not reach, none a machine error body on the three surfaces
e2e-test-strategy#2.3 · yes · the Path 1 watch: path1_run_start_one_home_in_order, its wall read against the ci profile's 20 s kill line
e2e-test-strategy#2.4 · no · standing S2 · a test owns a fresh home, its started wrappers in a Drop guard, nothing shared between tests
e2e-test-strategy#3.1 · yes · the MCP send result compared with the CLI --json document, the cross-surface parity the chunk needs
e2e-test-strategy#3.2 · yes · step 4's send mute --json refusal document and step 5's rmcp send call, the send error surfaces; the confirmation steps are not needed
e2e-test-strategy#3.3 · yes · the mute refusal at exit 13 (not-delivered, no-prompt-submitted), a CLI --json error body; the confirmation and human-mode lines are not needed
e2e-test-strategy#3.4 · no · the harness's cleanup command, which the task does not need
e2e-test-strategy#4.1 · yes · the MCP wait and last compared with the CLI --json document, and chaos_wait_parked_wrapper_killed_exits_21, the instance-unreachable body of a killed wrapper
e2e-test-strategy#4.2 · yes · step 7, the wait on a killed wrapper exiting 21 and the rmcp wait on the dead instance (instance-unreachable)
e2e-test-strategy#4.3 · yes · step 7's instance-unreachable body, exit 21 with detail null, and MCP is_error:true with text refused: instance-unreachable
e2e-test-strategy#4.4 · no · standing S2 · a test owns a fresh home, its started wrappers in a Drop guard, nothing shared between tests
e2e-test-strategy#5.1 · yes · the unknown-dialog exit 13 and unverified-cli exit 12 refusals on the CLI, and the MCP answer compared with the CLI --json document
e2e-test-strategy#5.2 · yes · step 4's answer builder 999999 --json (unknown-dialog), the rmcp answer call, and step 6's pause, whose answer is refused at exit 10
e2e-test-strategy#5.3 · yes · the unknown id's exit 13 (not-delivered, unknown-dialog), the MCP is_error:false answer, and step 6's exit 10 refusal
e2e-test-strategy#5.4 · no · the harness's cleanup command, which the task does not need
e2e-test-strategy#6.1 · yes · the cli pause and release refusals (human-typing with detail null or manual-pause, release-from-driver at exit 20) and the MCP refusal compared with the CLI document
e2e-test-strategy#6.2 · yes · steps 3 and 5 to 8: the send refusals (human-typing, turn-running) and release from a driver, which exits 20 with release-from-driver
e2e-test-strategy#6.3 · yes · send exit 10 with detail null, MCP is_error:true with text refused: human-typing, the manual-pause and release-from-driver bodies, and no MCP text naming release
