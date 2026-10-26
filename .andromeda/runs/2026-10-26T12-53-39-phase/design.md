# design extract

## Relevance
partial — the chunk's only design surface is the CLI human output of the two new verbs (`link` / `unlink` result lines, their refusals) and the `viola --help` `handoff:` row; the channel methods, the trust-boundary widening and the founder's P4 form are outside this domain, and every web-surface link treatment (blue names, `→` tape lines) is a 0.2.0 frontend input that binds nothing here.

## Constraints
- design-system §Surface: cli › Component Patterns (pattern 4, "Wheel, link and gate verbs") requires the result lines in their exact phraseology: `viola link overseer builder` → `link    overseer -> builder  since 19:43:58Z`, an existing pair → `link    overseer -> builder  already linked`, `viola unlink overseer builder` → `unlink  overseer -> builder` (the word column padded so `link` and `unlink` align, two-space gutters, `since` time in the `HH:MM:SSZ` form shown). The plan states no form for a `since` that falls on another UTC date (the `release --budget` line's date rule in `.andromeda/design-system-detail/surface.md` cli-verbs is that verb's own); whether the link line borrows it is research's / P4's question, not a mandate.
- design-system §Surface: cli › Tokens (platform-specific) requires link output (`overseer -> builder`) plain at every colour depth: handoff blue #5B8DB8 is "not used" on the CLI and no SGR index is reserved for it (detail: `.andromeda/design-system-detail/surface.md` cli-tokens, handoff blue row).
- design-system §Typography (**cli** paragraph) requires bold only for the board's NAME column, so the driver and driven names in a link / unlink line carry no bold.
- design-system §Iconography requires ASCII `->` for the CLI transfer arrow with no inbound form; `viola list` keeps its six columns with no link column in v1 (detail: `.andromeda/design-system-detail/iconography.md`, Transfer arrows).
- design-system §Surface: cli › Streams requires the result line on stdout, refusals and their `hint:` lines on stderr, and under `--json` one JSON document on stdout carrying the whole result (refusals and `error` objects included) with the typed exit code and no `hint:` line.
- design-system §Surface: cli › Exit-code phraseology requires each typed exit to take its human stderr form: exit 21 `unable  <name>  instance-unreachable` then the cause's hint (`.andromeda/design-system-detail/surface.md` cli-send lists the four causes, server verification among them), under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty. The server-verification hint is specified as built for six verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`); whether `link` / `unlink` join that set is an amendment this chunk owes if the verbs write a frame through the same check. Which exit row a `-32602` link refusal (`driven` not the serving wrapper, `driver` equal to `driven`) takes is arch's; the plan names no hint for it.
- design-system §Surface: cli › Navigation Pattern requires `viola --help` to list the served verbs only, the `handoff: link, unlink` row pending on this entry, placed between `wheel:` and `setup:` (detail: `.andromeda/design-system-detail/surface.md` cli-help).

## Patterns to follow
- The other one-line verb results of §Surface: cli › Component Patterns pattern 4 (`viola pause` / `viola release` / `viola release --budget` / `viola answer`): one unstyled stdout line, two-space gutters, fixed phraseology words, no colour in any mode.
- The colour decision order of §Surface: cli › Tokens (platform-specific): with no styled span in the link lines, the output is the same bytes on a TTY, a pipe, `NO_COLOR` and `TERM=dumb`; only `--json` replaces it with its document.
- The exit-21 per-cause hint table in `.andromeda/design-system-detail/surface.md` cli-send (`not-running`, unwrapped name, strict-modes, `server-verify`): a link verb's unreachable refusal reuses those lines byte for byte rather than adding verb-specific prose.
- The served-verbs-only help as built (§Surface: cli › Navigation Pattern): the `handoff:` row joins the grouped table in the same shape as `wheel: pause, release`.

## Anti-patterns to avoid
- design-system §Anti-Patterns › Per-Surface Bans (cli): never colour anything except `DIALOG` (amber) and `stale` rows (dim) — no blue or any SGR on link names, the arrow or the `since` time.
- design-system §Anti-Patterns › Per-Surface Bans (cli): never print "done" / "success" without context, and never green `✓` / red `✗` or emoji — the line says `link` / `unlink`, the pair, and `since <time>` or `already linked`.
- design-system §Anti-Patterns › Per-Surface Bans (cli): never print upstream text, paths, pids or anyhow chains in errors or hints — no refusal or hint of either verb names the endpoint, a pid, a start time or the home path.

## Contract bindings
- design ↔ security: the names printed in a link line are logged link names, which the second CARRY requires re-parsed through `ViolaName::try_new` before display; design-system §Surface: cli › Width requires human output ASCII only, which that re-parse is what guarantees for these fields.
- design ↔ arch: the human line must be derivable from the machine result `{since, already_linked}` (and `unlink`'s empty object) — `v1-13`'s link half (`viola-0.1.0/verification-matrix.json#v1-13`); the `--json` document shape is arch §Standard Contracts', the human words design-system §Surface: cli › Component Patterns'.
- design ↔ layouts: the `handoff:` help row is layout-templates §Output structure — `viola --help`'s as well; the two must name the same row and position.
- design ↔ tests: the `HELP` literal of `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs` moves with the row; the three result-line forms are root-case assertions in the verbs' tests.

## Acceptance criteria contributions
- `viola link a b` prints exactly `link    a -> b  since <HH:MM:SSZ>` on stdout for a new pair and `link    a -> b  already linked` for an existing one; `viola unlink a b` prints exactly `unlink  a -> b`; each is one ASCII line with no SGR byte on a TTY at every colour depth (per design-system §Surface: cli › Component Patterns; per design-system §Surface: cli › Tokens (platform-specific)).
- Under `--json` each verb prints one JSON document on stdout and no `hint:` line, with stderr empty on its refusal documents (per design-system §Surface: cli › Streams).
- An unreachable driven instance prints `unable  <name>  instance-unreachable` then the matching cause hint on stderr (exit 21), holding no pid, start time, path or endpoint (per design-system §Surface: cli › Exit-code phraseology).
- `viola --help` lists `handoff: link, unlink` between the `wheel:` and `setup:` rows once both verbs are served, and not before (per design-system §Surface: cli › Navigation Pattern).
