# New groups — architecture · conventions

## Verdicts
- fanout-architecture--cascade-g1.md 8 · contradiction · lines 64–64 · «the cause has no exit code, `--json` code or `RefusalReason` of its own» · the file gives the server-verify cause no `--json` code of its own, the proposal gives it a word through `UnreachableCause::ServerVerify` that the two clients write
- fanout-architecture--cascade-g1.md 14 · contradiction · lines 38–38 · «prints exactly `error: internal error` on stderr once» · the file has every `cli` verb print that human line on stderr with no `--json` exception, the proposal has `--json` print a stdout document in its place with stderr empty
- fanout-architecture--cascade-g1.md 15 · distinct · no group names the socket-directory refusal, `bind_endpoint`'s `create_private_dir` failure or its owner
- fanout-architecture--cascade-g1.md 17 · distinct · line 64 names only the server-verify hint, and no group holds the unwrapped-name or endpoint-lost hints or the four hint cause keys
- fanout-architecture--cascade-g1.md 18 · contradiction · lines 64–64 · «the cause has no exit code, `--json` code or `RefusalReason` of its own» · the file says this cause has no `--json` code and gives it no platform exception, the proposal names the code `server-verify-failed` and says it does not arise on macOS
- fanout-architecture--cascade-g1.md 20 · distinct · line 38 gives only the exit a failed connect or closed connection reaches, and no group holds `answer_of`'s mapping to causes or the no-endpoint arms
- fanout-architecture--cascade-g1.md 22 · duplicate · lines 76–76 · «the same as CLI `--json`» · line 76 already holds the matching body, the `refused: <code>` text and `-32603`'s fixed body, and lines 83 and 89 hold no frame and no line for their causes
- cascade-compare-architecture.md 1 · contradiction · lines 36–36 · «a refusal `run`'s start makes after a passed preflight prints no document» · the file says those start refusals print no document and lists five codes, the proposal has each print one and lists ten
- cascade-compare-architecture.md 2 · distinct · line 37 gives verify's refusals as stderr pairs and only `interrupted` as a `process-exit` detail, and no group holds the refusal codes the proposal adds to that line
