# design · Surface: {type} — category read

## Carried
- (.andromeda/design-system-detail/surface.md:54) A CLI verb whose server verification fails must refuse `instance-unreachable` (exit 21) with the server-verification cause's own hint, worded exactly as written, distinct from the not-running, unwrapped and strict-modes hints that share the code.
  > `instance-unreachable` (exit 21) has one hint per cause, because the code is shared (T4):
  >     - no wrapper is running for the name: `<name> is not running; viola list shows the live instances`
  >     - the name is an unwrapped session: `<name> is not a wrapped session; only sessions started with viola run take commands`
  >     - the viola home fails strict-modes: `the viola home is not private to you; <name> is not contacted until that is fixed`
  >     - server verification fails: `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`
- (.andromeda/design-system-detail/surface.md:73) Under `--json` a server-verification refusal must print no hint and keep arch's fixed exit-21 document, with no per-cause `detail` code invented here.
  > Hints are human-mode only. Under `--json`, no hint text is printed; the machine view carries the cause as a detail code, never as prose. **Pending an arch amendment:** arch currently fixes `{"v":1,"error":"instance-unreachable","detail":null}` for exit 21
- (.andromeda/design-system-detail/surface.md:73) The `--json` shape stays arch's until arch names the detail codes, and the log detail code for this cause is already named `server-verify-failed`, which the chunk's log line must use.
  > Until arch names the `detail` codes, `--json` keeps arch's shape. obs-plan D-20 already names the log detail codes: `instance-dead`, `strict-modes-failed`, `server-verify-failed`,
- (.andromeda/design-system-detail/surface.md:110) The exit-21 row binds every frame-writing CLI verb's mismatch outcome: the two-space `unable  instance-unreachable` line with no detail, then the cause's hint (server verification among the four), and under `--json` the fixed document with stderr empty.
  > | 21 | `unable  instance-unreachable` (no detail; the name is the omitted `<name>` field, as in every row), then the cause's hint (not running / unwrapped / strict-modes / server verification); `viola list`'s refused strict-modes check prints instead the fixed home-state or instance-state `unable:` / `hint:` pair (pattern 1, Refusal; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer) | `{"v":1,"error":"instance-unreachable","detail":null}` (a per-cause `detail` code awaits an arch amendment), `list`'s refusal included, with stderr empty |
- (.andromeda/design-system-detail/surface.md:21) The refusal must go to stderr in the two-space form, `viola send`'s with its `[/ ]` mirror prefix and padding, its `hint:` line directly after it.
  > refusals (`unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column, as in its sample; the exit-code table omits the `<name>` field; only the exit-1 refusals of `viola run`, `viola revive` and `viola verify`, and `viola list`'s exit-21 strict-modes refusal (chunk 2026-10-11-the-board-viola-list, the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer; the operator's answer superseded 2026-10-13), use the fixed message form, `unable: <text>`, as in `unable: <name> is already live`), each refusal's `hint:` line directly after it
- (.andromeda/design-system.md:338) The exit-21 phraseology for every verb this chunk touches is the `unable  instance-unreachable` line then the cause's hint, or the `--json` document with stderr empty.
  > 21 `unable  instance-unreachable` then the cause's hint, its `--json` document with stderr empty.
- (.andromeda/design-system.md:303) A server-verification refusal prints `unable` plus its reason, then one hint for its cause; no hint under `--json`, the per-cause detail code staying pending on the arch amendment.
  > A refusal prints `unable` plus `reason  detail`, then one `hint:` line per reason, detail or cause;
- (.andromeda/design-system.md:303) The pending per-cause `detail` code has its own owner, so this chunk must not add one to the `--json` document.
  > Under `--json` no hint prints; a per-cause `detail` code is pending, owner: the arch amendment naming them.
- (.andromeda/design-system.md:263) Under `--json` the whole refusal must be one stdout document with the typed exit code and no `hint:` line.
  > - For `--json`, the whole result, including refusals and `error` objects, is one JSON document on stdout with the typed exit code. No `hint:` line is printed under `--json`: the typed `refusal` / `error` fields are the whole answer.
- (.andromeda/design-system.md:380) The mismatch refusal and its hint must show no pid (recorded or observed), no start time source path and no endpoint path.
  > - **NEVER print upstream text, paths, pids or anyhow chains with serde sources in errors or hints.** This is the security plan's NEVER-log floor, and human output of `last` / `list` escapes C0/C1 controls.
- (.andromeda/design-system.md:354) The verification's full detail (the pids and start times compared) must go only to diagnostics, never to the terminal.
  > - **Stack traces** never print. Errors are fixed messages, and full detail goes only to `instances/<name>/diagnostics/`.
- (.andromeda/design-system.md:379) The refusal and hint stay on stderr, results on stdout, for every verb the check is added to.
  > - **NEVER mix data and messages.** Results go to stdout. `waiting:`, refusals and their `hint:` lines, and errors go to stderr.
- (.andromeda/design-system.md:353) The hook client's fail-open on mismatch must write nothing to stderr and exit 0, and the MCP tools' `instance-unreachable` must travel only as MCP frames on stdout; neither gains a human surface.
  > - **`hook`** writes nothing to stderr and always exits 0. **`mcp`** writes only MCP frames on stdout. Neither has a human design surface.
- (.andromeda/design-system.md:247) `viola hook` and `viola mcp` sit in the colour decision order's no-SGR, no-glyph rank, so nothing this chunk adds to them prints styled bytes or human text.
  > 3. `viola hook` (never writes stderr) and `viola mcp` (stdout carries MCP frames only)

## Bindings
- **server-verification hint ↔ obs-plan D-20**: the human cause line at (.andromeda/design-system-detail/surface.md:58) pairs with the log detail code `server-verify-failed` named at (.andromeda/design-system-detail/surface.md:73); the chunk's refusal path emits both, the code in the log, the prose only on the TTY.
- **exit-21 `--json` document ↔ architecture §Standard Contracts / §Conventions**: `{"v":1,"error":"instance-unreachable","detail":null}` at (.andromeda/design-system-detail/surface.md:110) is arch's fixed shape; a per-cause `detail` is the arch amendment's (.andromeda/design-system.md:303), not this chunk's.
- **hook fail-open silence ↔ security universal invariant (`viola hook` always exits 0, never writes stderr)**: (.andromeda/design-system.md:353) and (.andromeda/design-system.md:247).
- **no pid or path in a refusal ↔ security-plan §Error Handling (fixed messages, detail to `diagnostics/` only)**: (.andromeda/design-system.md:380) and (.andromeda/design-system.md:354).
- **strict-modes cause hint ↔ the boundary with "Home and code-bearing file integrity"**: if this chunk takes the snapshot's strict-modes check with the verification, its refusal uses the strict-modes cause's hint at (.andromeda/design-system-detail/surface.md:57), not the server-verification one.
- **refusal strings ↔ test-plan assertions**: an E2E or unit test matching the exit-21 stderr line or the hint matches the text at (.andromeda/design-system-detail/surface.md:58) verbatim.
