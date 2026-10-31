# Classified — architecture · conventions.b1

## Answers
conventions#1.1 · yes · add-to · F2: `viola_core::UnreachableCause` is a closed set with no serde path and is no RefusalReason, beside the RefusalReason values and the unconfirmable note
conventions#2.1 · no · is_code_word is unchanged and the new cause words pass it (F17); viola-core gains only UnreachableCause, so the claim holds on the changed file
conventions#3.1 · no · the refusal-detail reading of reply_of and of_reply is unchanged; RefusalReason and the crate error enums gain nothing, so the claim holds on the changed file
conventions#4.1 · yes · contradict · exit 21's --json detail is now a cause word, not null; revive --json documents every refusal of its start, so a post-preflight run start refusal no longer prints none; a stale holder reads stale-heartbeat, not already-live
conventions#5.1 · no · the home-tree refusal of run's and revive's start keeps its home-state pair, no log, and the strict-modes-failed document under revive --json, all unchanged by the session
conventions#6.1 · no · verify's home-tree refusal stays lineless with the same pair; F13's codes land only on logged refusals under a parsed VIOLA_NAME
conventions#7.1 · yes · contradict · the no-endpoint server-not-recorded-wrapper case now carries server-verify-failed as its --json detail, not null, and has a cause word of its own (it does not arise on macOS, F19)
conventions#8.1 · yes · contradict · the refused strict-modes check's --json detail is now strict-modes-failed or an instance cause word, not null, written through client::unreachable_document
conventions#9.1 · yes · contradict · the MCP unreachable result's structuredContent detail is now a cause word, not null, the CLI --json body less v, with text still refused: instance-unreachable
conventions#10.1 · yes · add-to · F5: the frame tool's isError result keeps its text refused: instance-unreachable and no cause writes a frame or a line, and its structuredContent now carries the cause word
conventions#11.1 · yes · contradict · a refused check_instance returns a strict-modes cause word as the isError detail, not null
