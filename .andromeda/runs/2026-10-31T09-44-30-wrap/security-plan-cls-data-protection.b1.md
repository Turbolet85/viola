# Classified — security-plan · data-protection.b1

## Answers
data-protection#1.1 · yes · stale · its edits in reached src/cmd/run.rs, src/run/mod.rs, src/cmd/verify.rs and src/cmd/hook/statusline.rs reach its run-start rewrite, probe-dir and statusline-capture rules, which no touch clears
data-protection#2.1 · no · the pinned copy's placement and replace_private_shared sit in viola-state, which the session did not reach, and no caller of the pinned copy moved
data-protection#3.1 · no · the AlreadyExists placement rule sits in viola-state, unreached, and the pinned-copy handling is listed untouched
data-protection#4.1 · no · the re-hash on Placed::Present is viola-state's and listed untouched with its callers, so run.pin_copy still reads ok or pinned-hash-mismatch
data-protection#5.1 · no · the release-build reading is architecture's, and Cargo.toml gained only the two test-binary tables with no profile or dependency moved
data-protection#6.1 · no · the events.ndjson link lines and the success replies are unchanged, the wire form did not move, and no touch names LinkSet or events.ndjson
data-protection#7.1 · no · the -32602 channel-response detail is one of three fixed words with no param name or value on the line, so no driver name reaches the process log
data-protection#8.1 · no · its open refusal and sink rules run through viola_state::strict::check_diagnostics, which the session did not reach, and the session changed only the channel-response detail value
data-protection#9.1 · yes · stale · its BatchScriptChild match edit in reached src/cmd/verify.rs reaches the --record scrub and refusal path, which no touch clears
data-protection#10.0 · yes · stale · its BatchScriptChild match edit in reached src/cmd/verify.rs reaches the Run B to E probe residuals, which no touch clears
