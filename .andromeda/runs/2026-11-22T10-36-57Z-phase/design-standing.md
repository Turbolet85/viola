# design · standing conventions

- S1 · every new closed refusal or detail word owes its fixed hint line in human mode, none under --json, and its exit code from the fixed table · surface
- S2 · every --json failure the change adds is one clean stdout document, with no hint line and no internals · surface-cli
  contract/viola-list-board/json-refusal · viola list board · --json refusal — Under `--json` the strict-modes refusal, the board's only one, is one stdout document `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
  contract/viola-revive-json-codes/socket-dir-refused · viola revive --json exit-1 codes · socket-dir-refused — A failed create of the socket directory and a refused check both print one stdout document with the code `socket-dir-refused`, no hint, stderr empty.
  contract/wrapper-fault-detail/unknown-code · wrapper fault --json detail · code outside the five — A code outside the five known prints `error: wrapper fault  <code>`, and under `--json` `{"v":1,"error":"wrapper-fault","detail":{"code":<code>,"message":"wrapper fault","data":null}}`; MCP carries the same fixed `message`.
