# Task — design · 2026-10-26-explicit-session-links

## The chunk
Serves the explicit session links. The wrapper answers the channel methods `link` (records the pair, answers `{since, already_linked}`) and `unlink` (removes it, answers the empty object, appends nothing for an absent pair), where today it answers `-32601`. A `driven` that is not the serving wrapper's name, or a `driver` equal to `driven`, is `-32602`.
Adds the two human verbs `viola link <driver> <driven>` and `viola unlink <driver> <driven>`, each with `--json`. The verb's instance is the driven one. They print three result lines, and `viola --help` gains its `handoff:  link, unlink` row.
Gives `LinkSet::remove` its first product caller through `unlink`. A logged link name is parsed through `ViolaName::try_new` again before any result line shows it.
Adds root tests in `tests/channel_links.rs` (test-plan Scenario E3 steps 1 and 3, plus the raw-channel name negative). Works toward `v1-13`'s `link` half: every human `link` line can be derived from its machine result.
The founder picks the boundary widening's form at P4. Each verb step follows the form he names, and no master says either verb is served until he does.

## Reached files
- src/run/links.rs · edits `LinkSet::remove` (its `#[cfg_attr(not(test), allow(dead_code))]` ends)
- src/cmd/run.rs · edits the channel method dispatch that answers `link` / `unlink` `-32601`
- src/cmd/mod.rs · edits the `viola --help` grouped verb rows (the `handoff:` row between `wheel:` and `setup:`)
- tests/channel_links.rs
- tests/cli_output_plain.rs · edits the `HELP` literal of `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs`
- viola-0.1.0/verification-matrix.json · edits `v1-13` (its `link` half)

## Needs
- The `link` / `unlink` result lines under Surface: cli › Component Patterns item 4 (wheel, link and gate verbs). A new pair prints `link    <driver> -> <driven>  since <HH:MM:SS>Z`, an existing pair prints `link    <driver> -> <driven>  already linked`, and `unlink` prints `unlink  <driver> -> <driven>`. Each is one stdout line, and the padding is exactly as shown.
- The `since` value is printed at whole-second UTC precision with a trailing `Z`, not the millisecond form that `send` / `wait` use.
- The `--json` result of `link` carries the same facts as its human line (`since`, `already_linked`), and `unlink`'s result is the empty object. Every human `link` line must be derivable from that machine result (`v1-13`).
- `viola --help`: the `handoff:  link, unlink` row sits between `wheel:` and `setup:` once the verbs are served, and the Navigation Pattern's pending note (owner: "Explicit session links") retires with it.
- The flat verb list under Navigation Pattern names `link · unlink`: each is one lower-case word acting on one strip, with no interactive prompt.
- The CLI handoff glyph: ASCII `->` with no inbound form, two spaces between columns, and ` - ` inside one field (Iconography).
- `viola list` stays at its six fixed columns. v1 has no link column, so the explicit verbs add nothing to the board (Iconography).
- On the CLI, link and unlink names are never shown in handoff blue. The departure blue that Core Colors gives to `link` / `unlink` names is a 0.2.0 frontend input, and the CLI colour tokens never use handoff blue. The result lines are therefore unstyled.
- A logged link name shown in these result lines is first parsed through `ViolaName::try_new` again. It prints exactly as the name was typed, never case-transformed or truncated.
- How the verbs refuse: the two-space `unable  <name>  <reason>  <detail>` line on stderr, then one `hint:` line per cause. The name is the driven instance's. No hint names `viola release`, and no hint quotes upstream text (the refusal and hint rules under the `viola send` component).
- Exit-code phraseology for the verbs. Exit 21 prints `unable  instance-unreachable` and then the cause's hint. Its `--json` form is one document with stderr empty, covering a failed liveness pre-check or a failed server identity check. Exit 2 is clap usage for a bad argv. Exit 1 is the fixed `error: internal error`. Exit 20 is `error: wrapper fault`. The wrapper's `-32602` refusals (a wrong `driven`, or `driver` equal to `driven`) must land on one of these typed codes.
- Fixed phraseology words only (`link`, `unlink`, `already linked`, `since`). No bare "done" or "success": the line says which pair and since when.

## Standing conventions
- S1 · Every new human output line is ASCII only, uses two spaces between columns, is never wrapped, and is never truncated where the terminal width is unknown · Surface: cli › Width (.andromeda/design-system-detail/surface.md, cli-output)
- S2 · Results go to stdout. Refusals, `hint:` lines and errors go to stderr. Under `--json` the whole result, refusals and `error` objects included, is one JSON document on stdout with the typed exit code, and no `hint:` line is printed · Surface: cli › Streams (.andromeda/design-system-detail/surface.md, cli-output)
- S3 · The colour decision order applies. There is no SGR, glyph or cursor control under `--json`, on a non-TTY, under `NO_COLOR` or under `TERM=dumb`, and colour is spent only on `DIALOG` (amber) and `stale` (dim) · Surface: cli › Tokens (.andromeda/design-system-detail/surface.md, cli-tokens) and Per-Surface Bans › cli
- S4 · Every refusal, error and hint is a fixed message. None carries a path, pid, endpoint, start time, upstream text, anyhow chain or stack trace, and full detail goes only to `diagnostics/` · Per-Surface Bans › cli

## Not needed
- Typography's web type roles, its per-OS font stacks and the Linux fallback render assertions: a 0.2.0 frontend input, since 0.1.0 renders no page.
- The spacing tokens, the border radius tokens and the depth strategy values: all frontend geometry.
- Surface Scale, Text Hierarchy and Border Progression, including the departure-blue 2px transfer-marker tick: 0.2.0 frontend inputs, and the CLI draws no transfer marker.
- The Semantic Colors domain status rows, including the tape treatment of the `link` / `unlink` kinds: a 0.2.0 frontend input, and this chunk adds no event kind (both kinds already landed).
- Motion and the expression level (brand-identity and motion values): the verbs print one static line, and the CLI has no motion.
- The readback box's drawn states, its `[RB]` / `[  ]` / `[/ ]` mirror, `unconfirmable`, and the `/clear` / `/compact` post-conditions: a link is not a send and carries no readback.
- `viola wait` / `viola last` result lines and their message escaper: untouched.
- The `viola list` board's BAY line, STATUS join, unwrapped rows and width truncation: the board is not edited.
- The `viola verify` step counter, and the output of `viola run`, `viola revive` and `viola plugin install`: none of these verbs is changed, and no verify child or ledger row is added.
- The Token Test of the Self-Validation Protocol, and the Swap and Squint tests: page-design checks for a 0.2.0 frontend.
