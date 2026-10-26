# design · cascade read

## Carried
- (.andromeda/design-system-detail/iconography.md:8) The `link` / `unlink` result lines must print the pair as `<driver> -> <driven>` in ASCII `->` with no inbound form, and a link must reach the human only as this verb output, never as a `viola list` column.
  > The CLI uses ASCII `->` and has no inbound form. `viola list` carries no link column in v1, so the six columns shared with the web strip stay fixed. Links print only as `link` / `unlink` verb output (`overseer -> builder`).
  (.andromeda/design-system.md:209)
  > The CLI is ASCII only: `->` with no inbound form, two spaces between columns, ` - ` inside one field; `viola list` carries no link column in v1, so its six columns stay fixed.
  (.andromeda/design-system.md:323)
  > - `viola link overseer builder` → `link    overseer -> builder  since 19:43:58Z` (an existing pair prints `link    overseer -> builder  already linked`)
  > - `viola unlink overseer builder` → `unlink  overseer -> builder`
- (.andromeda/design-system-detail/surface.md:14) The two verbs' result lines must be plain at every colour depth: no handoff-blue SGR on the names or the arrow, whatever the terminal takes.
  > | handoff blue #5B8DB8 | not used | not used | not used | The quiz limits CLI colour to amber and dim, so link output (`overseer -> builder`) is plain at every depth. No index is reserved: any blue SGR would break the ban on colouring anything but `DIALOG` and `stale`. |
  (.andromeda/design-system-detail/color-palette.md:13)
  > | Role (handoff) | #5B8DB8 | Transfer-marker arrows and names (outbound `→ builder`, inbound `← overseer`), the marker's 2px left tick (`--rule-strong`, the only blue rule), and the names in `link` / `unlink` tape lines only. Never a fill, never text on holder and never a status. | Departure-holder blue: outbound traffic being handed to the next sector (`driver → driven`). |
  (.andromeda/design-system.md:242)
  > Handoff blue is never used, and backgrounds are never set: the terminal background belongs to the user.
- (.andromeda/design-system-detail/typography.md:12) A logged link name, re-parsed through `ViolaName::try_new` before a result line shows it, must print exactly as it was typed: never case-transformed or truncated.
  > Never text-transformed: a `ViolaName` is lower-case `[a-z0-9-]` and must read exactly as it is typed as a `target`.
- (.andromeda/design-system-detail/surface.md:21) The verbs' refusals must take the two-space `unable  <name>  <reason>  <detail>` form on stderr with the driven instance's name, since neither verb is among the verbs the fixed `unable: <text>` form is reserved to.
  > refusals (`unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column, as in its sample; the exit-code table omits the `<name>` field; only the exit-1 refusals of `viola run`, `viola revive` and `viola verify`, and `viola list`'s exit-21 strict-modes refusal (chunk 2026-10-11-the-board-viola-list, the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer; the operator's answer superseded 2026-10-13), use the fixed message form, `unable: <text>`, as in `unable: <name> is already live`)
  (.andromeda/design-system.md:261)
  > - stderr: context lines, refusals (two-space `unable  <name>  <reason>  <detail>`, or the fixed `unable: <text>` form for the exit-1 refusals of `run`, `revive` and `verify` and `list`'s exit-21 refusal), each refusal's `hint:` line directly after it, and errors.
- (.andromeda/design-system-detail/surface.md:54) The verbs' exit-21 refusal must print the one hint of its cause (not running, unwrapped, strict-modes, server verification) as the shared code requires, each hint holding no pid, path or endpoint.
  >   - `instance-unreachable` (exit 21) has one hint per cause, because the code is shared (T4):
  >     - no wrapper is running for the name: `<name> is not running; viola list shows the live instances`
  >     - the name is an unwrapped session: `<name> is not a wrapped session; only sessions started with viola run take commands`
  >     - the viola home fails strict-modes: `the viola home is not private to you; <name> is not contacted until that is fixed`
  >     - server verification fails: `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`
  (.andromeda/design-system-detail/surface.md:46)
  >   - These two refusals reach drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20). So no hint ever names `viola release` (T3).
  (.andromeda/design-system-detail/surface.md:73)
  > Of them, `server-verify-failed` is written to the log by `send`, `wait`, `last`, `answer`, `pause` and `release` at their exit-21 server-verification refusal, whose `--json` document stays `detail:null`, as the not-running cause's does.
  (.andromeda/design-system-detail/surface.md:74)
  >   - No hint line for `unknown` (exit 14), because its detail is opaque and a hint would have to guess. No hint line for `error: wrapper fault` (exit 20) or `error: internal error` (exit 1), because they are faults, not refusals, and their detail goes only to `diagnostics/`.
  (.andromeda/design-system.md:303)
  > A refusal prints `unable` plus `reason  detail`, then one `hint:` line per reason, detail or cause; `run`'s and `revive`'s start refusals take a fixed-message `unable:` line and one hint per cause, `revive`'s read in a fixed order that stops at the first. No hint names `viola release`, since a driver's `release` is refused.
- (.andromeda/design-system-detail/surface.md:80) Where the form the founder names at P4 has the two verbs write their frame after the server identity check, the server-verification hint cause must be built for `link` and `unlink` too, and the list of frame-writing verbs it names must grow by them.
  > - The server-verification hint of `instance-unreachable` is built for `send`, `wait`, `last`, `answer`, `pause` and `release`, the six verbs that write a frame, as the hint cause `server-verify` beside `not-running`: it prints the server-verification line above byte for byte, plain ASCII with no SGR, and it holds no pid, start time, path or endpoint.
  (.andromeda/design-system-detail/surface.md:128)
  > - Exit 21's server-verification cause is built for `send`, `wait`, `last`, `answer`, `pause` and `release`: a process on the endpoint that is not the recorded wrapper (its pid or start time differs, or the check cannot be made) gets no frame, and the verb prints `unable  <name>  instance-unreachable` with the server-verification hint last on stderr, or under `--json` the document `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty.
- (.andromeda/design-system-detail/surface.md:86) The verbs' server check through `client::connect` must keep the macOS recorded gap: no comparison there and no server-verification hint printed on macOS.
  > - On macOS no peer pid is available, so the server check passes with no comparison and the server-verification hint cannot print there (a recorded gap; owner: the route entry "Unix endpoint and home hardening").
- (.andromeda/design-system-detail/surface.md:113) A served `link` / `unlink` must print its result line on stdout with no stderr word, and under `--json` one `{"v":1,"ok":{…}}` document (`since`, `already_linked` for `link`; the empty object for `unlink`).
  > | 0 | none (TTY context lines only); the result line goes to stdout (`[RB] read back`, `answered`, `wheel human` …) | `{"v":1,"ok":{…}}` |
- (.andromeda/design-system-detail/surface.md:114) The verbs' exit 1 must be only the fixed `error: internal error`, with no chain, path or hint and no `--json` document; they add no fixed-message refusal to this row.
  > | 1 | `error: internal error` (fixed message; no chain with serde sources, no paths, no hint)
  (.andromeda/design-system-detail/surface.md:114)
  > every other exit 1 has no `--json` document
- (.andromeda/design-system-detail/surface.md:115) A bad `link` / `unlink` argv must exit 2 with clap's usage text.
  > | 2 | clap usage text (never produced by `viola hook`) | — |
- (.andromeda/design-system-detail/surface.md:121) A wrapper fault on a `link` / `unlink` frame must print `error: wrapper fault  <code>` (exit 20) with no hint, and the wrapper's `-32602` refusals (a wrong `driven`, `driver` equal to `driven`) must land on one typed code of this table.
  > | 20 | `error: wrapper fault  <code>` | `{"v":1,"error":"wrapper-fault","detail":{…}}` |
- (.andromeda/design-system-detail/surface.md:122) A failed liveness pre-check or server identity check of either verb must print `unable  instance-unreachable` then the cause's hint, or under `--json` exactly the `instance-unreachable` document with stderr empty.
  > | 21 | `unable  instance-unreachable` (no detail; the name is the omitted `<name>` field, as in every row), then the cause's hint (not running / unwrapped / strict-modes / server verification); `viola list`'s refused strict-modes check prints instead the fixed home-state or instance-state `unable:` / `hint:` pair (pattern 1, Refusal; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer) | `{"v":1,"error":"instance-unreachable","detail":null}` (a per-cause `detail` code awaits an arch amendment), `list`'s refusal included, with stderr empty |
  (.andromeda/design-system.md:338)
  > Each typed exit code has its human stderr form and, where it has one, its `--json` document: 0 the result on stdout, 1 `error: internal error` or a fixed-message `unable:` refusal with its hint (a `--json` document only for `viola revive`'s strict-modes and preflight refusals), 2 clap usage, 10 to 14 an `unable` refusal with its reason (a refusal object under `--json`), 20 `error: wrapper fault`, 21 `unable  instance-unreachable` then the cause's hint, its `--json` document with stderr empty.
- (.andromeda/design-system-detail/surface.md:136) Once the verbs are served, `viola --help` must carry the `handoff:  link, unlink` row between `wheel:` and `setup:`, the as-built served-verbs note and the body's pending note retiring with it, and the flat verb list naming `link · unlink`.
  > - `viola --help` groups verbs as `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install` (`revive` beside `run`: the operator's answer at the chunk 2026-10-10-viola-revive wrap). As built (chunk 2026-10-17-cli-output-discipline) it lists the served verbs only: `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `setup: run, revive, verify, plugin install`, then `Options:`; the `handoff:` row joins when "Explicit session links" serves `link` and `unlink`.
  (.andromeda/design-system.md:345)
  > - Flat verbs, one lower-case word each: `run · revive · send · wait · last · list · answer · verify · pause · release · link · unlink · plugin install`.
  > - `viola list` is the board and the other verbs act on one strip. There is no interactive prompt, no TUI mode and no pager.
  (.andromeda/design-system.md:348)
  > - `viola --help` lists the served verbs as `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `setup: run, revive, verify, plugin install`, then `Options:`; the `handoff: link, unlink` row is pending, owner: the route entry "Explicit session links".
- (.andromeda/design-system.md:381) Each result line must name its pair and, for a new `link`, its `since` time: no bare "done" or "success" for a recorded or removed link.
  > - **NEVER print "done" or "success" without context.** Say what was read back, by whom, and at which cursor (`[RB] read back      builder  19:43:59.004Z  cursor 48213`, padded as in the `viola send` sample).

## Bindings
- **Link result lines ↔ the machine result (`v1-13`, arch's `link` reply)**: each human line — `link    <driver> -> <driven>  since <HH:MM:SS>Z`, `... already linked`, `unlink  <driver> -> <driven>` — is derived only from the reply's `since` / `already_linked` (and the empty `unlink` object) plus the two re-parsed names, and the `--json` exit-0 document carries the same facts, (.andromeda/design-system.md:323) ↔ (.andromeda/design-system-detail/surface.md:113).
- **Exit-code table ↔ the wrapper's `-32602` refusals (arch §Conventions, typed codes)**: a wrong `driven` and `driver` equal to `driven` reach the human and `--json` only through a typed row (20 `wrapper fault` or a refusal row), never a new untyped form, (.andromeda/design-system-detail/surface.md:121) ↔ (.andromeda/design-system-detail/surface.md:122).
- **Server-verification hint and cause lists ↔ security (the boundary widening's form at P4)**: whether `link` / `unlink` join the six frame-writing verbs of the hint and the exit-21 cause, and whether the strict-modes hint can print for them, follows the form the founder names, (.andromeda/design-system-detail/surface.md:80) ↔ (.andromeda/design-system-detail/surface.md:55), with the macOS gap kept, (.andromeda/design-system-detail/surface.md:86).
- **Handoff glyph ↔ CLI colour tokens**: `->` prints in every human mode as a word, while the handoff blue the palette gives link names stays off the CLI, (.andromeda/design-system-detail/iconography.md:8) ↔ (.andromeda/design-system-detail/surface.md:14) ↔ (.andromeda/design-system-detail/color-palette.md:13).
- **Help row ↔ test-plan (`tests/cli_output_plain.rs`)**: the `handoff:` row and the `HELP` literal of `help_is_the_grouped_verb_table` / `help_lists_only_served_verbs` move together, only once the verbs are served, (.andromeda/design-system-detail/surface.md:136) ↔ (.andromeda/design-system.md:348).
- **Name display ↔ input validation (`ViolaName::try_new`)**: a logged name printed in a result line is re-parsed and printed as typed, (.andromeda/design-system-detail/typography.md:12) ↔ (.andromeda/design-system-detail/iconography.md:8).
