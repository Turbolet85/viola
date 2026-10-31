# Classified — a11y-plan · cognitive-accessibility.b1

## Answers

cognitive-accessibility#1.1 · yes · stale · its bounds live in reached src/cmd/wait.rs, send.rs, revive.rs, list.rs and viola-mcp tools.rs, a reached file whose claim the report does not show holding
cognitive-accessibility#2.1 · yes · contradict · the exit-21 `--json` document is now one per cause with a closed `detail` word, not the null `detail` this line gives; revive's start refusals and the internal-error document also move the line (F1, F7)
cognitive-accessibility#3.1 · yes · contradict · `viola link` / `viola unlink` strict-modes exit 21 under `--json` is now the per-cause document with detail `strict-modes-failed`, not the exact null-detail `instance-unreachable` document
cognitive-accessibility#4.1 · yes · contradict · `send`, `wait`, `last`, `answer`, `pause` and `release` strict-modes exit 21 under `--json` now carries detail `strict-modes-failed`, not the exact null-detail document
cognitive-accessibility#5.1 · yes · stale · src/cmd/run.rs and src/cmd/revive.rs are reached, and `revive --json` now prints one exit-1 document for every start refusal before the child spawns
cognitive-accessibility#6.1 · yes · stale · src/cmd/verify.rs is reached and its refusals gained `process-exit` log codes, so the "no line written" reading must be re-read
cognitive-accessibility#7.1 · yes · stale · src/cmd/verify.rs and the cli catch site in src/main.rs are reached; the Windows catch-site line stands without `--json`, but the catch site now prints an internal-error document under `--json` (F6)
cognitive-accessibility#8.1 · yes · stale · src/cmd/link.rs is reached and the no-endpoint exit-21 refusal now reads `instance-dead` or `unwrapped-name` by the instance directory (F3), so "the not-running hint" is only one of two
cognitive-accessibility#9.1 · yes · stale · src/cmd/link.rs is reached and a CLI verb's fault under `--json` now prints a document on stdout, so the fault reading moved
cognitive-accessibility#10.1 · yes · stale · `reply_of` and `of_reply` and `viola_core::is_code_word` sit in reached files; the new code words pass the screen, but the file is reached whole and its claim is not shown holding
cognitive-accessibility#11.1 · yes · stale · the closed refusal list gained `stale-heartbeat`, `endpoint-lost`, `unwrapped-name`, `internal-error` and `server-verify-failed` in reached files, so the list's reading moved
cognitive-accessibility#12.1 · yes · add-to · F6: under `--json` a CLI verb's catch-site fault now prints the `internal-error` document on stdout with stderr empty, a new fault body beside the sanitised one
cognitive-accessibility#13.1 · yes · stale · tests/contract_error_bodies.rs is reached and its `NOT_ERROR_BODIES` lost the post-preflight revive row, so the exception set moved
cognitive-accessibility#14.1 · yes · contradict · the server-verify refusal's `--json` document now carries `server-verify-failed` and `viola list`'s refusal reads `strict-modes-failed`, so the exact documents pinned here moved; the exit-cause table (F11) also witnesses them
cognitive-accessibility#15.1 · yes · stale · tests/cli_links.rs is reached and the stopped-instance refusal reading moved (`instance-dead` or `unwrapped-name` by the instance directory, F3)
cognitive-accessibility#16.1 · yes · contradict · the frame verbs' strict-modes refusal under `--json` now has detail `strict-modes-failed`, not the exact `instance-unreachable` document it pins
cognitive-accessibility#17.1 · yes · stale · tests/cli_error_bodies.rs is reached and the set of `--json` failure documents it holds grew (internal-error, per-cause and revive start documents)
cognitive-accessibility#18.1 · yes · stale · tests/contract_error_bodies.rs is reached; the new code words and the internal-error body become rows and writers the census must name
cognitive-accessibility#19.1 · no · its check functions and body classes live in tests/support/events.rs, which is not a reached file, and the session adds no body class
cognitive-accessibility#20.1 · yes · add-to · F12: the `unwrapped-name` and `endpoint-lost` hints join the literal-pinned strings, through `send_hint_is_the_design_string` and the exit-cause table
cognitive-accessibility#21.1 · yes · stale · tests/cli_links.rs is reached and holds the pinned link-line root case; the file is reached whole and its claim is not shown holding
cognitive-accessibility#22.1 · no · its pinned unit lives in src/human/verbs.rs, which is not a reached file, and the session changes no link `since` time or name
cognitive-accessibility#23.1 · yes · stale · tests/cli_list.rs is reached and holds the `CAPTION` root case; the file is reached whole and its claim is not shown holding
