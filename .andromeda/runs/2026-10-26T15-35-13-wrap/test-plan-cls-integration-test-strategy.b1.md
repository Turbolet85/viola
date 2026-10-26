# Classified — test-plan · integration-test-strategy.b1

## Answers
integration-test-strategy#1.1 · no · its direct crate-API calls and the cargo-nextest and rstest 0.27.0 tools still hold on .config/nextest.toml, which gains only one kill-line binary entry
integration-test-strategy#1.2 · no · the concurrent-append files and their writers are unchanged, and CI is green on three OSes
integration-test-strategy#1.3 · no · the external boundary is still child processes served by the unchanged fake agent, and the link and unlink checks read no claude agents --json
integration-test-strategy#1.4 · no · the endpoint, socket and pipe claims hold on tests/channel_endpoint.rs, whose only change is the unserved-method case, and link and unlink frames stay under the same schema assertions
integration-test-strategy#1.5 · no · the PTY spawn and pump claims hold on src/cmd/run.rs, whose only change is link and unlink method routing, and CI is green on three OSes
integration-test-strategy#1.6 · no · the hook path, its trace and budget.json are unchanged, and src/cmd/run.rs gains only link and unlink routing beside hook.event
integration-test-strategy#2.0 · no · the fresh-home and stamped_home setup rules hold for the new link cases (CI green on Windows), and the nextest kill-line filter is not named here
integration-test-strategy#3.1 · yes · add-to · F10 joins this group, because the unserved-method case in tests/channel_endpoint.rs now uses anything and asserts no method key on the conn-less line, and link and unlink are served and covered by tests/channel_links.rs
integration-test-strategy#3.2 · no · strict-modes refusals already say CLI 21 for every other verb and link and unlink take exit 21, so the re-read agrees, while the revive and event-kind claims are not reached
integration-test-strategy#4.1 · yes · contradict · the session's handoff row link, unlink now sits in the HELP table, so the claim of no link or unlink there is false, and F5 adds tests/cli_links.rs to the CLI file list
