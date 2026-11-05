@carried n=1 label=design-system/surface fact=fct-vocxcyzi
why: The typed exit set 0, 1, 2, 10–14, 20, 21 must keep each row's human line and --json form exactly as it is, since the scope re-asserts that the removal adds no exit code and moves none.
quote: | 12 | `unable  unverified-cli` | refusal object |
@carried n=1 run=2 label=design-system/surface fact=fct-k7c3yar5
quote: | 13 | `unable  not-delivered  input-not-ready` / `no-prompt-submitted` / `turn-running` / `unknown-dialog` / `control-character` / `empty-text` | refusal object |
@carried n=1 run=3 label=design-system/surface fact=fct-f6euya2e
quote: | 14 | `unable  unknown  <detail>`, the detail a code word or nothing | refusal object |
@carried n=1 run=4 label=design-system/surface fact=fct-adve5pgh
quote: | 20 | `error: wrapper fault  <code>` | `{"v":1,"error":"wrapper-fault","detail":{…}}` |
@carried n=1 run=5 label=design-system/surface fact=fct-arozinj6
quote: Under `--json` the `cli` catch site prints row 1's `internal-error` document on stdout in place of `error: internal error`, stderr then empty; without `--json` it prints that line, byte for byte.
@carried n=1 run=6 label=design-system fact=fct-tj4jq2zv
quote: Each typed exit code has its human stderr form and, where it has one, its `--json` document:
@carried n=1 run=7 label=design-system fact=fct-2t54hjhs
quote: under `--json` every refusal of the start it shares with `run` prints one stdout document and no hint, stderr empty, its code one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`
@carried n=2 label=design-system/surface fact=fct-hgef4h2n
why: Where a cfg(unix) strict-modes, socket or connect arm loses its gate because its Windows sibling left, exit 21's fixed unable: / hint: pairs, its documents and its six closed causes must stay exactly as they read on Linux.
quote: | 21 | `unable  instance-unreachable` (no detail; the name is the omitted `<name>` field, as in every row), then the cause's hint (not running / unwrapped / endpoint lost / server verification / socket directory); `viola list`'s refused strict-modes check, and `viola link` / `viola unlink`'s of the driven instance, print instead the fixed home-state or instance-state `unable:` / `hint:` pair (pattern 1, Refusal; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer), and so does the instance check `viola send`, `wait`, `last`, `answer`, `pause` and `release` make of their instance before any frame, printing the instance-state pair naming it, with no frame and no line written | `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`; the `list`, `link`, `unlink` and `plugin install` refusals and the six frame verbs' strict-modes refusals read `strict-modes-failed`; stderr empty |
@carried n=2 run=2 label=design-system/surface fact=fct-zp4y2mcv
quote: With no live endpoint a frame verb reads its instance directory after the strict-modes check: a directory is `instance-dead`; absent, a regular file or a symlink is `unwrapped-name`.
@carried n=2 run=3 label=design-system/surface fact=fct-aej2ym2d
quote: A failed connect is `instance-dead`; a wrapper lost with the call open is `endpoint-lost`.
@carried n=2 run=4 label=design-system/surface fact=fct-ihyxsaec
quote: A frame verb against a stale instance reads `instance-dead`: stale heartbeat is exit 1's cause alone.
@carried n=3 label=design-system/surface fact=fct-pxdkcmfq
why: The server-verification cause must keep its Linux checks (pid, start time, and the peer euid refused when it is not this user's or cannot be read) through the removal, while only its macOS clause leaves with the macOS arm, and no env, flag or feature seam may force the peer-euid decision.
quote: Exit 21's server-verification cause is built for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`: a process on the endpoint that is not the recorded wrapper (its pid or start time differs, or the check cannot be made; on Linux and macOS also a peer whose euid is not this user's or cannot be read, on macOS the euid alone) gets no frame, and the verb prints `unable  <name>  instance-unreachable` with the server-verification hint last on stderr, or under `--json` the document `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` with stderr empty.
@carried n=4 label=design-system/surface fact=fct-ue5y52fu
why: hook must keep its contract (exit 0, no body, no stderr) and mcp its stdout of MCP frames only, silent at the catch site, through any arm removal in their paths, while the colour order's Windows VT step and 256-colour Windows clause leave with the cfg(windows) arm.
quote: `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.
@carried n=4 run=2 label=design-system fact=fct-cj5433yq
quote: **`hook`** writes nothing to stderr and always exits 0. **`mcp`** writes only MCP frames on stdout.
@carried n=4 run=3 label=design-system fact=fct-gu2srphh
quote: 3. `viola hook` (never writes stderr) and `viola mcp` (stdout carries MCP frames only)
  4. `NO_COLOR` set (present, an empty value included)
  5. `TERM=dumb`
  6. stdout is not a terminal
  7. Windows VT enable failed
@carried n=4 run=4 label=design-system fact=fct-vzhczpnh
quote: 256-colour if `TERM` contains `256color`, or on Windows once VT enabling has succeeded;
@carried n=5 label=design-system/surface fact=fct-kqtzijw6
why: tests/cli_error_bodies.rs is the check that witnesses the re-asserted exit codes and --json forms, and it must keep passing on Linux with every document free of ESC, paths and upstream text.
quote: Each CLI `--json` failure `tests/cli_error_bodies.rs` drives prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text.
@carried n=5 run=2 label=design-system fact=fct-b7z6jyxk
quote: **NEVER print upstream text, paths, pids or anyhow chains with serde sources in errors or hints.** This is the security plan's NEVER-log floor
@binding n=1
**surface exit-code rows ↔ architecture's typed exit codes**: the CLI's human and `--json` forms of 0, 1, 2, 10–14, 20, 21 are the design-side face of architecture's typed set (design-system fct-nrsgugtu); the removal of the Windows and macOS arms adds no code and changes no form on either side, and `tests/cli_error_bodies.rs` keeps witnessing them on Linux (design-system/surface fct-kqtzijw6) (design-system/surface fct-vocxcyzi).
@binding n=2
**surface exit-21 causes ↔ architecture's cross-platform discipline and the security plan's peer check**: when a `cfg(unix)` socket, connect or strict-modes arm loses its gate, the six closed causes and their fixed pairs stay (design-system/surface fct-hgef4h2n); the server-verification text's macOS clause is re-worded by the wrap's expected amendment to the Linux-only OS target, while the Linux peer-euid refusal stays, forced by no seam (design-system/surface fct-pxdkcmfq).
@binding n=3
**surface hook / mcp silence ↔ the body's colour decision order and Toolkit**: hook and mcp keep their output contract through the arm removal (design-system/surface fct-ue5y52fu); the order's step 7 and the 256-colour Windows clause go with the `cfg(windows)` SGR arm and its windows-sys dependency (design-system fct-2wyyc34q), while steps 1–6 and the truecolor / 256color / 16-colour choice stay as they behave on Linux (design-system fct-gu2srphh), and design-system names the Windows terminal behaviour as an expected amendment the wrap writes.
@binding n=4
**surface error bodies ↔ the security plan's NEVER-log floor**: no error, hint or refusal the chunk re-words or keeps prints upstream text, paths, pids or anyhow chains (design-system fct-b7z6jyxk), and the `--json` failures stay free of ESC, path and upstream text (design-system/surface fct-kqtzijw6).
