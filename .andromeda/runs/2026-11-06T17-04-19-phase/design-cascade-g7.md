@carried n=1 label=design-system/surface fact=fct-vocxcyzi
why: Item 6's fold or ratification of `StampsError` / `StampsReadError` must leave exit 12's human form and refusal object exactly as stated.
quote: | 12 | `unable  unverified-cli` | refusal object |
@carried n=2 label=design-system/surface fact=fct-k7c3yar5
why: Item 6's `Refused` fold must keep exit 13's closed reasons, and the live driver session's send evidence reads them whenever a send is not read back.
quote: | 13 | `unable  not-delivered  input-not-ready` / `no-prompt-submitted` / `turn-running` / `unknown-dialog` / `control-character` / `empty-text` | refusal object |
@carried n=2 run=2 label=design-system fact=fct-5xeughsz
quote: A refusal prints `unable` plus `reason  detail`, then one `hint:` line per reason, detail or cause;
@carried n=3 label=design-system/surface fact=fct-f6euya2e
why: Item 6's `Refused` fold must keep exit 14's form and its closed detail exactly.
quote: | 14 | `unable  unknown  <detail>`, the detail a code word or nothing | refusal object |
@carried n=4 label=design-system/surface fact=fct-adve5pgh
why: Item 6's `ServeError` / `McpError` fold or ratification must keep exit 20's human line, its `wrapper-fault` document and its closed `detail` exactly.
quote: | 20 | `error: wrapper fault  <code>` | `{"v":1,"error":"wrapper-fault","detail":{…}}` |
@carried n=4 run=2 label=design-system/surface fact=fct-3b32n36m
quote: Exit 20's `--json` `detail` is closed for the five known codes (`viola_channel::sanitised_fault`): `message` is each code's fixed text; `-32602` with an integer `data.supported` and a version-text `data.wrapper` reads the unsupported-version body with those two values, `data.reason` exactly `release-from-driver` reads that body, and any other `-32602` reads `invalid params` with null `data`; every other known code has null `data`.
@carried n=4 run=3 label=design-system/surface fact=fct-utjcf3wu
quote: A code outside the five prints the same human line, `error: wrapper fault  <code>`, and under `--json` the document `{"v":1,"error":"wrapper-fault","detail":{"code":<code>,"message":"wrapper fault","data":null}}`; MCP carries the same fixed `message`.
@carried n=5 label=design-system/surface fact=fct-hgef4h2n
why: Item 7's founder word on the socket-directory pair's wording lands on exit 21's `socket-dir-refused` cause and its fixed `unable:` / `hint:` pair, and the enum folds must keep the six closed causes.
quote: | 21 | `unable  instance-unreachable` (no detail; the name is the omitted `<name>` field, as in every row), then the cause's hint (not running / unwrapped / endpoint lost / server verification / socket directory); `viola list`'s refused strict-modes check, and `viola link` / `viola unlink`'s of the driven instance, print instead the fixed home-state or instance-state `unable:` / `hint:` pair (pattern 1, Refusal; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer), and so does the instance check `viola send`, `wait`, `last`, `answer`, `pause` and `release` make of their instance before any frame, printing the instance-state pair naming it, with no frame and no line written | `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`; the `list`, `link`, `unlink` and `plugin install` refusals and the six frame verbs' strict-modes refusals read `strict-modes-failed`; stderr empty |
@carried n=5 run=2 label=design-system fact=fct-tj4jq2zv
quote: 21 `unable  instance-unreachable` then the cause's hint, or for a refused strict-modes check its fixed home-state or instance-state `unable:` / `hint:` pair instead (pattern 1 and the paragraphs below), its `--json` document `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, one per cause, the cause one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`, with stderr empty.
@carried n=5 run=3 label=design-system fact=fct-2t54hjhs
quote: under `--json` every refusal of the start it shares with `run` prints one stdout document and no hint, stderr empty, its code one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`;
@carried n=6 label=design-system/surface fact=fct-7mkzemsx
why: The founder's disposition of v1-13's clause on the MCP result for `link` is deferred or re-worded against the built CLI half, whose `--json` result documents are these.
quote: Exit 0 under `--json`: `viola link`'s document is `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`, `<ts>` the time the wrapper recorded the link, and `viola unlink`'s is `{"v":1,"ok":{}}`.
@carried n=6 run=2 label=design-system/surface fact=fct-3vwzogmp
quote: Exit 20 is also reached by `viola link` / `viola unlink` on a pair the wrapper refuses (`-32602`; a `driver` equal to `driven`, which the CLI never refuses itself): `error: wrapper fault  <code>`, or under `--json` the `wrapper-fault` document.
@carried n=7 label=design-system/surface fact=fct-kqtzijw6
why: `tests/cli_error_bodies.rs` is the check that witnesses every `--json` failure document the enum folds must keep, and it reads green at this chunk's wrap.
quote: Each CLI `--json` failure `tests/cli_error_bodies.rs` drives prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text.
@carried n=8 label=design-system/surface fact=fct-arozinj6
why: A fold that routes any variant to the `cli` catch site must print exactly its `internal-error` document or line, and no stack trace.
quote: Under `--json` the `cli` catch site prints row 1's `internal-error` document on stdout in place of `error: internal error`, stderr then empty; without `--json` it prints that line, byte for byte.
@carried n=8 run=2 label=design-system fact=fct-zumogwyu
quote: **Stack traces** never print. Errors are fixed messages, and full detail goes only to `instances/<name>/diagnostics/`.
@carried n=9 label=design-system/surface fact=fct-ue5y52fu
why: The `portable-pty` move must keep `run` and `revive`'s start silent and their terminal the child's, and the `ServeError` / `McpError` fold must keep `mcp` silent, at the catch site too.
quote: `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.
@carried n=9 run=2 label=design-system fact=fct-w4pl2fni
quote: **`run`:** prints nothing once the child starts.
@carried n=9 run=3 label=design-system fact=fct-jphnedpy
quote: **NEVER emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`.** LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen; a passed `viola revive` is the same passthrough.
@carried n=9 run=4 label=design-system fact=fct-cj5433yq
quote: **`mcp`** writes only MCP frames on stdout.
@carried n=10 label=design-system/surface fact=fct-pxdkcmfq
why: Item 8's mutation survivor is an always-admit `peer_admitted` read at the client seams, the euid check this cause states, and the chunk must record it as the untestable zone with its dated note rather than claim it tested.
quote: Exit 21's server-verification cause is built for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`: a process on the endpoint that is not the recorded wrapper (its pid or start time differs, or the check cannot be made; also a peer whose euid is not this user's or cannot be read) gets no frame, and the verb prints `unable  <name>  instance-unreachable` with the server-verification hint last on stderr, or under `--json` the document `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` with stderr empty.
@binding n=1
**surface exit-code phraseology ↔ architecture's typed exit codes and one-enum-per-crate rule**: every fold or ratification of `ServeError` / `McpError`, `StampsError` / `StateError`, `PinError`, `StampsReadError` and `Refused` must leave each typed code, its human stderr form, its `--json` document and its closed `detail` words exactly as the surface rows state, and the masters-agree check must find both masters naming the same codes (design-system/surface fct-vocxcyzi) (design-system/surface fct-k7c3yar5) (design-system/surface fct-f6euya2e) (design-system/surface fct-adve5pgh) (design-system/surface fct-hgef4h2n) (design-system fct-tj4jq2zv)
@binding n=2
**surface exit 21 `socket-dir-refused` ↔ the founder's word on the socket-directory pair (item 7) and architecture's socket directory**: whatever wording the founder rules must land in the cause's `unable:` / `hint:` pair and stay the closed `detail` word of exit 21 and of `viola revive --json`'s start codes, the masters following (design-system/surface fct-hgef4h2n) (design-system fct-2t54hjhs)
@binding n=3
**surface link / unlink results ↔ the verification matrix's v1-13 disposition**: a deferral or re-wording of v1-13's MCP-result-for-`link` clause must agree with the built CLI half's result documents and refused-pair path, and the master text must agree with whichever the founder rules (design-system/surface fct-7mkzemsx) (design-system/surface fct-3vwzogmp)
@binding n=4
**surface server-verification cause ↔ security-plan peer admission, test-plan §1 untestable zone and matrix#v1-44**: the euid peer check stated as the cause's rule is the read whose always-admit mutation survives, so the record must name it untestable by construction, not proved (design-system/surface fct-pxdkcmfq)
@binding n=5
**surface `run` silence ↔ the `portable-pty` / `crates/viola-pty` move**: the new pty release must keep `run` and `revive`'s start printing nothing once the child starts and emitting no colour, glyph or cursor control, silent at the catch site too (design-system/surface fct-ue5y52fu) (design-system fct-w4pl2fni) (design-system fct-jphnedpy)
@binding n=6
**surface `--json` failure bodies ↔ test-plan's `tests/cli_error_bodies.rs`**: the gate «every gate green on Linux» includes this check, which must still witness one document, the literal code, empty stderr and no ESC, path or upstream text after the folds (design-system/surface fct-kqtzijw6) (design-system/surface fct-arozinj6)
