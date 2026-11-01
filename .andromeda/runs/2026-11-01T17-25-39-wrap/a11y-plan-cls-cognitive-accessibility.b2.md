# Classified — a11y-plan · cognitive-accessibility.b2

## Answers
cognitive-accessibility#26.1 · yes · add-to · the exit-cause table in `tests/cross_exit_causes.rs` (reached) gains the exit-21 cause `socket-dir-refused` and its hint literal (F2), reached through `send_hint` cause `socket-dir` at exit 21 (F3)
cognitive-accessibility#27.1 · no · `tests/support/events.rs` is not in the reached files, the body check's four classes are unchanged, and the new refusal's clean `--json` adds no class to it
cognitive-accessibility#28.1 · yes · add-to · the new socket-dir hint (F2, reached through `send_hint` cause `socket-dir` per F3) joins the literal-pinned hints beside `unwrapped-name` and `endpoint-lost`, and the new fixed planning-form pair (F1) joins the planning-form strings pinned by literal
cognitive-accessibility#29.1 · no · the `viola link` and `viola unlink` lines and their pinning tests (`tests/cli_links.rs`, `src/human/verbs.rs`) are not reached and the session changes no link output
cognitive-accessibility#30.1 · no · the `link` time and name rules are unchanged, and the session touches no link output or its files
cognitive-accessibility#31.1 · no · the `CAPTION` pin in `tests/cli_list.rs` was not edited and the new `viola list` case is a refusal, not a board row, so the caption order still holds on the changed file
