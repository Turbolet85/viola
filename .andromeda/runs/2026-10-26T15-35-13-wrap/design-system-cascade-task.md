# Task — design-system · 2026-10-26-explicit-session-links

## The session
Built the human verbs `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]`, and the driven wrapper now serves the channel methods `link` and `unlink` (before: answered `-32601`).
Retired the hold: every line stating that the explicit link / unlink verbs are not built, held for the founder's word, or pending on the route entry "Explicit session links".
Each verb runs the instance strict-modes check on the driven instance before any log or snapshot read, then the liveness pre-check, then the server identity check before its one frame (the founder's own word, the instance-check form).
Added three plain result lines, the `--json` success documents, and the `viola --help` row `handoff:  link, unlink` between `wheel:` and `setup:`.
Moved counts: CLI frame verbs six to eight (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`), each a writer of `server-verify-failed`; served channel methods seven plus `hook.event` to nine.
No colour, glyph, token, length or duration was added or changed; every new line is ASCII with no SGR in every mode.

## Reached files
- src/cmd/link.rs
- src/cmd/unlink.rs
- src/run/links.rs
- src/cmd/run.rs
- src/cmd/mod.rs
- src/human/verbs.rs
- src/human.rs
- tests/channel_links.rs
- tests/channel_endpoint.rs
- tests/cli_output_plain.rs
- tests/security_negatives_server_identity.rs
- tests/cli_links.rs
- .config/nextest.toml
- viola-0.1.0/verification-matrix.json
- viola-0.1.0/working-route.md

## Touches
- surface, cli-send group (the line the report cites as `surface.md:73`): the list of verbs that write `server-verify-failed` and take the server-verify hint names six CLI frame verbs; it is now eight, `link` and `unlink` added.
- surface, cli-help group (the line cited as `surface.md:136`) and the body's Navigation Pattern (`design-system.md:348`): the `handoff: link, unlink` row is no longer pending and no longer owned by the route entry "Explicit session links"; it is built as `handoff:  link, unlink` between `wheel:` and `setup:`, among the served rows `viola --help` prints.
- surface, cli-verbs group and the body's Component Patterns item 4: the `link` and `unlink` sample lines are now built and tested byte for byte (`link    <driver> -> <driven>  since <HH:MM:SSZ>`, `link    <driver> -> <driven>  already linked`, `unlink  <driver> -> <driven>`); any wording that treats them as design-only or unbuilt is stale.
- surface, cli-exit-codes groups and the body's Exit-code phraseology: exit 21 gains the two verbs' strict-modes refusal of the driven instance, printed as the fixed instance-state `unable:` / `hint:` pair (as `viola list` and `viola plugin install` print their home-state pair), and under `--json` the `instance-unreachable` document with stderr empty.
- surface, cli-exit-codes: exit 21 also covers the two verbs' stopped instance (the not-running hint, printed last) and an impostor server (the server-verify hint), as for the other frame verbs.
- surface, cli-exit-codes: exit 20 `error: wrapper fault` is now reachable from `link` / `unlink` when the wrapper refuses the pair (`-32602`, e.g. driver equal to driven); exit 1 internal error for a reply of the wrong shape.
- surface, cli-output groups and the body's Streams: the list of refusals that take the fixed `unable:` form (the exit-1 refusals of `run`, `revive` and `verify`, and `list`'s exit-21 refusal) must now include the `link` / `unlink` exit-21 strict-modes refusal.
- The body's Component Patterns item 3, message mode (the escaper `wait` and `last` share): now also used by `link` when the reply's `since` is no RFC 3339 instant; its rule is unchanged, its users grow by one.
- Any count of CLI frame verbs, or any enumeration of verbs that run the instance strict-modes check, wherever design-system states one: six frame verbs to eight; the strict-modes refusing verbs gain `link` and `unlink`.
- Any line in surface or the body stating that link / unlink are held, unserved, or answered `-32601`: retired.
- The body's Navigation Pattern flat verb list (`… · link · unlink · plugin install`): re-read, claim unmoved; both words are now served verbs.
- The body's Core Colors role line (handoff blue for transfer markers and `link` / `unlink` names only) and the cli Tokens line (handoff blue never used on the CLI): re-read, claims unmoved; the built CLI lines carry no SGR in any mode.
- surface, cli-tokens: the new result lines are plain in every mode, so the colour decision order and the "colour only `DIALOG` and `stale`" rule hold without a new case.
- The `--json` envelope `{"v":1,"ok":…}` described for verb results gains two document shapes (see New facts).

## New facts
- F1 · surface · `viola link` / `viola unlink` print one result line on stdout in one write, plain ASCII with no SGR in every mode, exit 0; the names printed are the verb's own parsed arguments, never names read back from the reply or the log.
- F2 · surface · the `since` in the `link` line is the reply's time as UTC clock time in whole seconds with `Z`; a `since` that is no RFC 3339 instant (the restore's `unknown`) prints through the message-mode escaper.
- F3 · surface · `--json` success documents: `link` prints `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`, `unlink` prints `{"v":1,"ok":{}}`.
- F4 · surface · `unlink` of a pair that is not held prints the same `unlink  <driver> -> <driven>` line and exits 0; a second `link` of a held pair prints the `already linked` form with the first `since` kept.
- F5 · surface · the verb's instance is the driven one: its refusals and hints name the driven instance, and its strict-modes refusal is exit 21 (as `list` and `plugin install`), not `revive`'s exit 1.
- F6 · surface · a pair the wrapper refuses (driver equal to driven, a bad or foreign name) is never refused client-side; the CLI shows it as exit 20 `error: wrapper fault`.

## Untouched
- Typography, Spacing, Border Radius, Motion and Depth Strategy: 0.2.0 frontend inputs; no value, face or length moved.
- color-palette Surface Scale, Text Hierarchy, Semantic Colors and Border Progression: no page is drawn; the session adds no status row, no hue and no rule.
- Brand Identity anchors (the handoff / transfer-of-control anchor included), the signature readback box and the expression level `0.2`: unchanged; the built CLI lines realise existing samples, not a frontend marker.
- Iconography: the ASCII `->` with no inbound form is unchanged, and `viola list` still carries no link column, its six columns fixed.
- The `viola list` strip table (cli-list): the board, its columns, the BAY line and the unwrapped section are unchanged; the board does not show pairs.
- The `viola send` readback mirror samples, the send hints and the `unconfirmable` row: unchanged; only the server-verify verb list in that group moves.
- The `viola wait` / `viola last` result lines and the `waiting:` line: unchanged.
- `viola verify`, `viola run` and `viola revive` output (cli-verify-run): unchanged, including the revive `--json` refusal documents and the pending "Exit-cause code catalogue" note.
- `viola pause`, `viola release` (including `--budget`), `viola answer` and `viola plugin install` lines: unchanged.
- Self-Validation Protocol and its Token Test: no colour, length or duration was added.
- Anti-Patterns: no ban changed; the new lines obey the cli bans as they stand.
- Platform-Specific Notes (Git Bash, conhost, `hook`, `mcp` hidden verb, stack traces): unchanged.
