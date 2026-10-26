## 2026-10-26-explicit-session-links — `link` / `unlink`'s exit-21 strict-modes refusal takes the fixed instance-state pair
**Section:** Surface: cli › Tokens (platform-specific) and Component Patterns (surface.md: the cli-output stderr bullet, the cli-list Refusal bullet, the cli-exit-codes table's row 21); design-system.md: the Streams stderr line, the Exit-code phraseology
**Change:**
- surface.md stderr: the fixed `unable: <text>` form is used by the exit-1 refusals of `viola run`, `viola revive` and `viola verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusal `viola link` / `viola unlink` make of the driven instance (was: without the two verbs).
- Refusal: the instance pair `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only` stays `revive`'s, shared; `viola link` / `viola unlink` print it, naming the driven instance, at their exit-21 strict-modes refusal.
- Row 21: `viola list`'s refused strict-modes check, and `viola link` / `viola unlink`'s of the driven instance, print the fixed home-state or instance-state pair in place of `unable  instance-unreachable` and a hint; the `--json` cell reads "the `list`, `link` and `unlink` refusals included" (was "`list`'s refusal included"), stderr empty.
- Streams: the fixed form names "the exit-21 strict-modes refusals of `list`, `link` and `unlink`" (was "`list`'s exit-21 refusal").
- Exit-code phraseology, new: `viola link` / `viola unlink`'s refused strict-modes check of the driven instance (exit 21) prints the instance-state pair naming that instance, as `viola list`'s does, or under `--json` the `instance-unreachable` document, stderr empty.
**Why:** the verbs check the home and the driven instance before any log or snapshot read and print the instance-state pair on a refusal. The form is the founder's own answer to this chunk's P4 fork (inputs#I3, relayed verbatim by the operator); exit 21, as `list` and `plugin install`, not `revive`'s exit 1, is his option's text. Row 21's extension of the closed fixed-form list lands beside the stderr rule it extends, on the overseer's founder-delegated answer at the halt (inputs#I6), a delegate's.
**Kept:** `viola plugin install`'s exit-21 home-state pair is still absent from the fixed-form lists of the stderr bullet and the Streams line: a standing gap, not this chunk's.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the server-verification cause and hint cover eight frame verbs
**Section:** Surface: cli › Component Patterns (surface.md: the cli-send hints bullet on the D-20 log detail codes, the cli-send server-verification hint group, the cli-exit-codes server-verification group)
**Change:**
- `server-verify-failed` is written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 server-verification refusal (was the six, per "2026-10-26-server-verification-before-any-frame — `server-verify-failed` written by the six frame verbs"); the `--json` document stays `detail:null`.
- The server-verification hint of `instance-unreachable` is built for the eight verbs that write a frame (was "the six verbs", per "2026-10-26-server-verification-before-any-frame — the server-verification hint as built, and its macOS gap"); its cause `server-verify`, its byte-for-byte line, plain ASCII with no SGR and no pid, start time, path or endpoint unchanged.
- Exit 21's server-verification cause is built for the same eight (was six, per "2026-10-26-server-verification-before-any-frame — exit 21's server-verification cause as built"); its stderr line and `--json` document unchanged.
**Why:** `link` / `unlink` write their frame after the liveness pre-check through the same server identity check; a server that is not the recorded wrapper gets no frame, exit 21 with the server-verify hint, and `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}`. The CLI frame verbs are now eight.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — `link` / `unlink`'s result lines as built
**Section:** Surface: cli › Component Patterns (surface.md: a new cli-verbs group); design-system.md: pattern 3's message-mode item
**Change:**
- New: `viola link` / `viola unlink` print their one result line on stdout in one write, plain ASCII with no SGR whatever the terminal, and exit 0; the names printed are the verb's own parsed arguments, never names read back from the reply or the log.
- New: the `link` line's `since` is the reply's `since` as UTC clock time in whole seconds with `Z` (`HH:MM:SSZ`); a `since` that is no RFC 3339 instant (the restore's `unknown`) prints through pattern 3's message-mode escaper.
- New: a second `viola link` of a held pair prints the `already linked` form, the wrapper keeping the first `since`; `viola unlink` of a pair not held prints the same `unlink  <driver> -> <driven>` line; both exit 0.
- Pattern 3: message mode is the escaper `wait` and `last` share and `viola link` uses for a `since` that is no RFC 3339 instant (was "the escaper both verbs share"); its rule unchanged.
**Why:** the chunk built the three result lines, each one write, plain in every mode, from the parsed arguments; a repeated `link` answers the same `since` with `already_linked: true`, and `unlink` answers alike whether or not a pair was removed.
**Kept:** pattern 4's `link` / `already linked` / `unlink` lines stand as worded: they are the built lines.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — `link` / `unlink`'s `--json` documents and their exit 20
**Section:** Surface: cli › Component Patterns (surface.md: a new cli-exit-codes group)
**Change:**
- New: exit 0 under `--json`: `viola link`'s document is `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`, `<ts>` the time the wrapper recorded the link, and `viola unlink`'s is `{"v":1,"ok":{}}`.
- New: exit 20 is also reached by `viola link` / `viola unlink` on a pair the wrapper refuses (`-32602`; a `driver` equal to `driven`, which the CLI never refuses itself): `error: wrapper fault  <code>`, or under `--json` the `wrapper-fault` document.
**Why:** the wrapper now serves `link` / `unlink` (the founder's own answer to this chunk's P4 fork, inputs#I3, relayed verbatim by the operator) and the verb prints its `ok` as `{"v":1,"ok":<ok>}`; a wrapper error, `-32602` included, is exit 20. A `driver` equal to `driven` refused by the wrapper only is the operator's answer of chunk 2026-10-10-session-links.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the `handoff:` help row built
**Section:** Surface: cli › Navigation Pattern (surface.md: the cli-help bullet; design-system.md: the `viola --help` line)
**Change:**
- surface.md As built: `--help` lists the served verbs `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install`, then `Options:`; was without the `handoff:` row, which "joins when "Explicit session links" serves `link` and `unlink`" (that clause gone); the planned grouping and its provenance unchanged.
- design-system.md: the same five rows; was the four, "the `handoff: link, unlink` row is pending, owner: the route entry "Explicit session links"" (retired).
- Partial retirement: the `handoff:` waiting clause of "2026-10-17-cli-output-discipline — the colour layer and the width cut built; --help groups the served verbs" and of "2026-10-17-optional-plugin-install — plugin install served in setup:, its lines and its exit-21 pair" no longer stands.
**Why:** the chunk serves `link` and `unlink`, and `viola --help` gains the `handoff:` row between `wheel:` and `setup:`, held by the help tests.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/
