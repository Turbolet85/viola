# design · Surface: {type} — category read

## Carried
- (.andromeda/design-system-detail/surface.md:14) The `link` / `unlink` result lines must print with no SGR at every colour depth: no blue index, no styled span.
  > The quiz limits CLI colour to amber and dim, so link output (`overseer -> builder`) is plain at every depth. No index is reserved: any blue SGR would break the ban on colouring anything but `DIALOG` and `stale`.
- (.andromeda/design-system-detail/surface.md:21) A refusal of the two verbs must go to stderr in the two-space `unable` form, with its `hint:` line directly after it.
  > - stderr: context lines (`waiting: <name>`, the issue line of a send), refusals (`unable  <name>  <reason>  <detail>`, two-space separated;
- (.andromeda/design-system-detail/surface.md:54) An `instance-unreachable` from `link` / `unlink` (exit 21) must print the hint of its cause from this per-cause set, server verification included, since the verbs write a frame.
  >   - `instance-unreachable` (exit 21) has one hint per cause, because the code is shared (T4):
  >     - no wrapper is running for the name: `<name> is not running; viola list shows the live instances`
  >     - the name is an unwrapped session: `<name> is not a wrapped session; only sessions started with viola run take commands`
  >     - the viola home fails strict-modes: `the viola home is not private to you; <name> is not contacted until that is fixed`
  >     - server verification fails: `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`
- (.andromeda/design-system-detail/surface.md:73) Under `--json` the two verbs must print no hint text, and an exit-21 document must keep arch's `detail:null` shape until arch names detail codes.
  > Hints are human-mode only. Under `--json`, no hint text is printed; the machine view carries the cause as a detail code, never as prose. **Pending an arch amendment:** arch currently fixes `{"v":1,"error":"instance-unreachable","detail":null}` for exit 21,
- (.andromeda/design-system-detail/surface.md:80) The server-verification hint is written for the six frame verbs today. When `link` / `unlink` become frame-writing verbs, they must print this same byte-for-byte line, and this record must name them.
  > - The server-verification hint of `instance-unreachable` is built for `send`, `wait`, `last`, `answer`, `pause` and `release`, the six verbs that write a frame, as the hint cause `server-verify` beside `not-running`: it prints the server-verification line above byte for byte, plain ASCII with no SGR, and it holds no pid, start time, path or endpoint.
- (.andromeda/design-system-detail/surface.md:86) On macOS the two verbs share the recorded server-check gap, so their server-verification hint cannot print there.
  > - On macOS no peer pid is available, so the server check passes with no comparison and the server-verification hint cannot print there (a recorded gap; owner: the route entry "Unix endpoint and home hardening").
- (.andromeda/design-system-detail/surface.md:113) A served `link` / `unlink` must put its result line on stdout at exit 0, and under `--json` print the `ok` document.
  > | 0 | none (TTY context lines only); the result line goes to stdout (`[RB] read back`, `answered`, `wheel human` …) | `{"v":1,"ok":{…}}` |
- (.andromeda/design-system-detail/surface.md:122) The two verbs' exit-21 output must follow this row: the human `unable  instance-unreachable` line plus the cause's hint, or the fixed `--json` document with stderr empty.
  > | 21 | `unable  instance-unreachable` (no detail; the name is the omitted `<name>` field, as in every row), then the cause's hint (not running / unwrapped / strict-modes / server verification); `viola list`'s refused strict-modes check prints instead the fixed home-state or instance-state `unable:` / `hint:` pair (pattern 1, Refusal; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer) | `{"v":1,"error":"instance-unreachable","detail":null}` (a per-cause `detail` code awaits an arch amendment), `list`'s refusal included, with stderr empty |
- (.andromeda/design-system-detail/surface.md:128) The exit-21 server-verification cause is written for six verbs. The two new frame verbs must write no frame on a mismatch and must print the same line, or the same document, as those six.
  > - Exit 21's server-verification cause is built for `send`, `wait`, `last`, `answer`, `pause` and `release`: a process on the endpoint that is not the recorded wrapper (its pid or start time differs, or the check cannot be made) gets no frame, and the verb prints `unable  <name>  instance-unreachable` with the server-verification hint last on stderr, or under `--json` the document `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty.
- (.andromeda/design-system-detail/surface.md:136) When this entry serves `link` and `unlink`, `viola --help` must add the `handoff: link, unlink` row between `wheel:` and `setup:` (the third CARRY).
  > - `viola --help` groups verbs as `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install` (`revive` beside `run`: the operator's answer at the chunk 2026-10-10-viola-revive wrap). As built (chunk 2026-10-17-cli-output-discipline) it lists the served verbs only: `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `setup: run, revive, verify, plugin install`, then `Options:`; the `handoff:` row joins when "Explicit session links" serves `link` and `unlink`.
- (.andromeda/design-system.md:209) The link lines must use the ASCII `->` with two-space column gaps, and `viola list` must get no link column.
  > The CLI is ASCII only: `->` with no inbound form, two spaces between columns, ` - ` inside one field; `viola list` carries no link column in v1, so its six columns stay fixed.
- (.andromeda/design-system.md:242) No handoff blue and no background may appear in the verbs' CLI output.
  > Handoff blue is never used, and backgrounds are never set: the terminal background belongs to the user.
- (.andromeda/design-system.md:244) The `->` in a link line is a word that must print in every human output, piped and `NO_COLOR` included. Only `--json` replaces it.
  > so the ASCII readback mirror `[RB]` / `[  ]` / `[/ ]` and `->` are words that still print in every human output, piped, `NO_COLOR` and `TERM=dumb` included, and only `--json` replaces them with its JSON document):
- (.andromeda/design-system.md:259) The link and unlink result lines are results and must go to stdout.
  > - stdout: results and data (tables, send/wait results, `last` text).
- (.andromeda/design-system.md:263) Under `--json` each verb must print one JSON document on stdout with the typed exit code, refusals included, and no `hint:` line.
  > - For `--json`, the whole result, including refusals and `error` objects, is one JSON document on stdout with the typed exit code. No `hint:` line is printed under `--json`: the typed `refusal` / `error` fields are the whole answer.
- (.andromeda/design-system.md:270) The verbs' human output must be ASCII only.
  > - Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
- (.andromeda/design-system.md:303) A refusal of the verbs must print `unable` plus its reason and detail, then one hint, and no hint may name `viola release`. Under `--json` no hint prints, and faults get none.
  > A refusal prints `unable` plus `reason  detail`, then one `hint:` line per reason, detail or cause; `run`'s and `revive`'s start refusals take a fixed-message `unable:` line and one hint per cause, `revive`'s read in a fixed order that stops at the first. No hint names `viola release`, since a driver's `release` is refused. Under `--json` no hint prints; a per-cause `detail` code is pending, owner: the arch amendment naming them. Faults and `unknown` get no hint.
- (.andromeda/design-system.md:305) A hint of the two verbs must never quote upstream text.
  > - A hint never quotes the sent text or any upstream text.
- (.andromeda/design-system.md:323) The three result lines (the first CARRY's "three result lines") must match these exact forms, the verb word padded so that `link` and `unlink` align.
  > - `viola link overseer builder` → `link    overseer -> builder  since 19:43:58Z` (an existing pair prints `link    overseer -> builder  already linked`)
  > - `viola unlink overseer builder` → `unlink  overseer -> builder`
- (.andromeda/design-system.md:345) `link` and `unlink` must be flat, one-word lower-case verbs in the verb set.
  > - Flat verbs, one lower-case word each: `run · revive · send · wait · last · list · answer · verify · pause · release · link · unlink · plugin install`.
- (.andromeda/design-system.md:348) Today the body marks the `handoff:` help row as pending on this entry. Serving the verbs must clear that pending marker.
  > - `viola --help` lists the served verbs as `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `setup: run, revive, verify, plugin install`, then `Options:`; the `handoff: link, unlink` row is pending, owner: the route entry "Explicit session links".
- (.andromeda/design-system.md:376) The verbs may colour nothing, since colour belongs only to `DIALOG` and `stale`.
  > - **NEVER colour anything except `DIALOG` (amber) and `stale` rows (dim), and never either without its word.** The quiz confirms colour as a second cue only.
- (.andromeda/design-system.md:377) The verbs must emit no colour, glyph or cursor control under `--json`, non-TTY, `NO_COLOR` or `TERM=dumb`.
  > - **NEVER emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`.**
- (.andromeda/design-system.md:379) Results must go to stdout, and refusals, hints and errors to stderr.
  > - **NEVER mix data and messages.** Results go to stdout. `waiting:`, refusals and their `hint:` lines, and errors go to stderr.
- (.andromeda/design-system.md:380) No error or hint of the verbs may print upstream text, a path, a pid or an anyhow chain.
  > - **NEVER print upstream text, paths, pids or anyhow chains with serde sources in errors or hints.**
- (.andromeda/design-system.md:381) A link result must say what was recorded and for which pair (`since` or `already linked`), never a bare "done".
  > - **NEVER print "done" or "success" without context.**

## Bindings
- **link / unlink result lines ↔ layout-templates and test-plan**: the three line forms (.andromeda/design-system.md:323) are the strings that layout-templates' verb output and the `v1-13` assertion (each human `link` line derivable from its machine result) must match. Both sides must change together.
- **`handoff:` help row ↔ layout-templates §Output structure — `viola --help`**: the row's place (.andromeda/design-system-detail/surface.md:136) and the body's pending marker (.andromeda/design-system.md:348) must move with layout-templates' help sample and the `HELP` literal of `help_is_the_grouped_verb_table` / `help_lists_only_served_verbs`.
- **Plain link output ↔ Color Palette**: the palette locks departure blue to `link` / `unlink` names (a 0.2.0 frontend input), but on the CLI that token is "not used" (.andromeda/design-system-detail/surface.md:14). The CLI side wins for this chunk.
- **Exit-21 server-verification cause ↔ architecture §Conventions / obs-plan D-20**: the six-verb lists at (.andromeda/design-system-detail/surface.md:80) and (.andromeda/design-system-detail/surface.md:128) must grow to cover `link` / `unlink`, matching arch's exit-code row and the `server-verify-failed` log code. The `--json` document stays `detail:null` (.andromeda/design-system-detail/surface.md:73).
- **Refusal / hint text ↔ security-plan NEVER-log floor**: no path, pid or upstream text in a hint (.andromeda/design-system.md:380). This binds the second CARRY's `ViolaName::try_new` re-parse of a logged link name before a result line shows it.
