# Classified — architecture · conventions.b1

## Answers
conventions#1.1 · yes · add-to · F2: `viola_core::UnreachableCause` is a closed detail-word set in viola-core with no serde path and no RefusalReason, joining the closed sets this entry describes; RefusalReason's own values gain nothing
conventions#2.1 · no · is_code_word's definition is unchanged and the new cause words pass it, so the code-word rule and its callers reach nothing here
conventions#3.1 · no · the refusal-detail reading (code-word string as is, non-code string as null, non-string as no known reply) is unchanged; the session's new mapping covers channel errors, not refusal details
conventions#4.1 · yes · contradict · exit 21's document `detail` is one of five cause words, not null; a stale holder now refuses `stale-heartbeat` not `already-live`; `revive --json` documents every start refusal, retiring the post-preflight gap sentence
conventions#5.1 · no · the home-tree refusal still prints the home-state pair, opens no log and writes no line, and under `revive --json` stays the strict-modes-failed document; `refused_document` keeps its signature and exit-1 callers
conventions#6.1 · no · the verify home-tree refusal stays lineless with the same pair, and the update_stamps strict-modes refusal still prints the same pair; the new log codes do not change either claim
conventions#7.1 · yes · contradict · the exit-21 document on these verbs now carries a cause word (`server-verify-failed` for a server that is not the recorded wrapper) instead of detail null, so the cause has a `--json` code
conventions#8.1 · yes · contradict · the strict-modes check's `--json` document for these verbs is `strict-modes-failed` (`refuse_untrusted` writes through `unreachable_document`), not detail null
conventions#9.1 · yes · contradict · the MCP unreachable `isError` body is now the CLI document less `v`, with a cause word in place of detail null
conventions#10.1 · yes · add-to · F5: the server-verify refusal's `isError` body now carries the cause word `server-verify-failed`, while its text stays `refused: instance-unreachable` with no frame and no log line
conventions#11.1 · yes · contradict · a refused `check_instance` now reads `StrictModes` in `tools.rs` and its result carries detail `strict-modes-failed`, not detail null
