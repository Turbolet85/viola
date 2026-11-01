# security-plan · picked accounting · group 2 of 3

## Picked
input-validation#1.1 · not touched · `crates/viola-core/src/lib.rs` is reached, but the report moves only `UnreachableCause` there and names no change to the paste rule or `validate_paste_text`
input-validation#1.2 · not touched · the frame verbs' `connect` gained checks with its callers unedited (report.md:15), so the free-text rule in `viola answer` and the wrapper's handler still holds
input-validation#1.3 · not touched · `server.rs` is reached, but the drop acts before any frame (report.md:13) and `channel-response`'s `detail` stays closed (report.md:75), so the frame rules still hold
input-validation#1.4 · not touched · `src/cmd/hook.rs` is reached, but its change is in `connect` (report.md:16), after the stdin and instance rules this row states, which still hold
input-validation#1.5 · not touched · `tools.rs` is reached, but its change is in `frame` after `live_endpoint` (report.md:17), so the argument checks this row states still hold
input-validation#1.7 · proposed · the frame verbs' and MCP's order gain the directory and euid checks, the `revive --json` codes go ten to eleven, and the board's home takes the read rule
input-validation#1.8 · not touched · `viola list`'s scan keeps its order and no-open (report.md:18), so a refused `list` still reads no `config.json`, and the report names no config read moved
input-validation#1.13 · not touched · `board.rs` is reached, but `read_budget_stamped` keeps the write rule (report.md:18) and hook `connect`'s replayed-snapshot path is not named as moved, so the row holds
input-validation#1.19 · not touched · `src/cmd/run.rs` is reached, but its change is the socket-directory step of the start (report.md:14), and no statusline source read is named as moved
input-validation#1.20 · not touched · `src/cmd/hook.rs` is reached, but its change is in `connect` (report.md:16), and the statusline arm and its shell-out are not named as moved
input-validation#1.21 · not touched · `src/cmd/run.rs` is reached, but its change is the socket-directory step (report.md:14), and no sideload or DLL step is named as moved
input-validation#2.0 · not touched · the directory check reads metadata only (report.md:11) and the report names no new bounded reader or moved `MAX_FRAME` use, so the constants row holds
data-protection#8.1 · not touched · `strict.rs` is reached, but its additions are the socket-directory checks (report.md:11), and `check_diagnostics` and the diagnostics opens are not named as moved
bootstrap-phases#1.1 · proposed · the owed `peer_creds` decision and directory verification are built, the client reads the peer euid, and the `viola-state` list gains the new checks and refusal
secret-management#1.1 · not touched · the session adds no secret and the wrapper's drop uses no env, flag or feature seam (report.md:13), so the production row holds
secret-management#2.1 · not touched · no pre-push launcher or harness environment is named as moved, and the report's harness change is `cleanup`'s `unconnectable` (report.md:20), so the development row holds
