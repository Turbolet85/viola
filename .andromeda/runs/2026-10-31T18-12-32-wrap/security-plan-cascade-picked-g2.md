# security-plan · picked accounting · group 2 of 2

## Picked
input-validation#1.1 · not touched · `src/cmd/send.rs` moved only at its no-endpoint arm; the report moves no `validate_paste_text`, `empty-text` or `not-delivered` rule, and the row still holds
input-validation#1.2 · not touched · `src/cmd/answer.rs` moved only at its no-endpoint arm, so `control-character` checked first on both sides still holds
input-validation#1.5 · not touched · `tools.rs` and `result.rs` moved the unreachable result only; the report leaves the raw-argument checks and `invalid-params` as they stood
input-validation#1.7 · proposed · the frame verbs' and MCP's null `detail` and the `revive --json` post-preflight gap are retired, and F10 and F11 sit beside the gap's line
input-validation#1.8 · not touched · the report adds no config key and `src/cmd/list.rs` moved only its strict-modes document, so a refused `list` still reads no `config.json`
input-validation#1.10 · not touched · `src/cmd/run.rs` moved `collision_check` and refusals that return before `spawn_child`; the seam, which acts after the child starts, is not among them
input-validation#1.13 · not touched · a fresh replayed beat still reads `already-live`, `without_endpoint` opens nothing, and `live_endpoint` stays the one read, as the report states
input-validation#1.17 · proposed · under `revive --json` a `batch-script-child` refusal prints a document in place of the row's two stderr lines, and a `claude` not found prints `internal-error`
input-validation#1.18 · not touched · `no-session`, exit 1 before any spawn, still holds and the new hints hold no id; the closed code list it joins is F10, carried once beside input-validation#1.7
input-validation#1.19 · not touched · the report moves no statusline read; `run.rs` and `mod.rs` changed only refusal paths and `internal_error_document`
input-validation#1.21 · not touched · `src/main.rs` gained the catch site's `--json` branch and `json_asked`, and `pinned-hash-mismatch` is the pinned binary's, so no sideload rule moved
input-validation#2.0 · not touched · no read or cap moved; `version-unreadable` is a new log code on the same capped read, and `without_endpoint` reads metadata only
data-protection#9.1 · proposed · F12: a refused recording now reaches `process-exit` as `unclean-fixture`, with the fixture's `<file>` in no code
bootstrap-phases#1.1 · not touched · the strict-modes check stays first and writes no line, so its client line holds; `without_endpoint` is F7, an authentication-authorization fact
error-handling#1.5 · proposed · a failed server check now reads `server-verify-failed` in `detail`, and its null `detail` is retired
error-handling#1.6 · proposed · a refused check reads `strict-modes-failed` and no live endpoint reads `without_endpoint`'s cause, so one null result no longer holds
error-handling#1.15 · proposed · the CLI's `--json` refusal now reads `strict-modes-failed`, and F1, F2 and F5 sit beside it
error-handling#1.17 · proposed · `update_stamps`'s refusal now carries `strict-modes-failed` on `process-exit`, so its `no line` holds for the refused home tree alone, and F3 and F4 sit beside it
error-handling#1.21 · not touched · the two refusals stay fixed messages and the report leaves the types as they stood; their new codes stand on input-validation#1.17 and in F10 and F12
error-handling#1.22 · proposed · F6: the new cause table stands beside the error-body inventory, whose named exclusions still hold with one row gone
security-anti-patterns#2.1 · not touched · no harness verb moved, and the report's `create_private_dir` is `bind_endpoint`'s, not the mutation scratch's
security-anti-patterns#4.1 · not touched · the report reads the ledger's 24 rows as a true claim sharing the kill line's token, and it moves no stamp rule
security-anti-patterns#4.2 · not touched · `verify`'s new codes are log codes; the home-tree check stays before the `cli` log and a stamps refusal still reads and writes no stamps file
security-anti-patterns#5.1 · not touched · `json_asked` only reads `--json` to choose a document, and the new `fake-agent` `[[test]]` adds no seam or environment read
