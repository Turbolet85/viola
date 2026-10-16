# design extract

## Relevance
relevant. The chunk changes the CLI surface: it adds the unwrapped rack and its rows to `viola list`, turns the STATUS word into `idle` / `busy`, and adds a new `viola revive` refusal line. Nothing on the web surface applies, because 0.1.0 renders no page.

## Constraints
- The board's strip fields are fixed in the order NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, and a field with no reading prints `unknown`. An unwrapped session is a bare strip ("another facility's traffic, visible but never a `target`"), so the unwrapped rack adds no seventh column (a pid column, for example). Per design-system §Brand Identity (Domain anchors: Flight-progress strip; Strip holder).
- The unwrapped rack sits below the wrapped rows, under the ASCII separator `-- UNWRAPPED - READ-ONLY --`. An unwrapped row prints its name, LIVE and STATUS, and prints `n/a` in WHEEL, DIALOG and CLI, as in the sample row `c1f9e2a4      live   idle     n/a                      n/a                n/a`. Rows are ordered by name. Per design-system §Surface: cli (Component Patterns 1, `viola list`).
- STATUS takes one word from a closed set: `idle`, `busy` or `unknown`. `unknown` covers a `claude agents --json` read that is missing or failed, and a row that joins nothing. `busy` has no spinner, pulse or animation. `n/a` on an unwrapped row means "absent by contract", which is different from a failed reading. Per design-system §Color Palette (Domain status colors: `SessionStatus::*`, `WheelHolder::(absent, unwrapped)`, `DialogPending::(absent, unwrapped)`, `CliVersion::(absent, unwrapped)`, `Wrapped::false`).
- Every field that comes from `claude agents --json` (the unwrapped names, for example) has its C0 and C1 controls escaped as `\xHH` text. In `list` rows `\n` and `\t` are escaped too (`\x0A`, `\x09`), so each row stays one fixed-width line. Human output is ASCII only. Per design-system §Surface: cli (Component Patterns 1, `viola list`).
- Where the terminal width can be detected, human tables truncate only the NAME of unwrapped rows (`...`). Otherwise nothing is truncated, and lines are never wrapped. Per design-system §Surface: cli (Width).
- A `viola revive` refusal uses the fixed exit-1 form: one `unable: <text>` line, then that cause's `hint:` line. The causes are read in a fixed order that stops at the first refusal. No refusal line shows the recorded directory, a pid or a logged id, and no hint names `viola release`. Under `--json` no hint text is printed. Per design-system §Surface: cli (Component Patterns 2, the `viola revive` refusals). The plan has no `session-live` cause line or hint yet. Its wording is a gap for P4, not something to invent here.
- In a refusal, the CLI separator inside one field is ` - `, and two spaces separate fields. Per design-system §Iconography (Separator).

## Patterns to follow
- The `list` "As built" note: the row mode escapes every control character of a row (DEL and C1 included) as `\xHH` through `escape_row`. This chunk adds the call's fields as upstream-derived cells next to the recorded CLI version. Per design-system §Surface: cli (Component Patterns 1, As built).
- The As-built board stays uncoloured. SGR, bold NAME, amber `DIALOG` and the dim row are owed to "CLI output discipline", so the separator and the unwrapped rows print plain too. Per design-system §Surface: cli (Component Patterns 1, As built).
- The unwrapped-name hint already written for `instance-unreachable` (exit 21) reads `<name> is not a wrapped session; only sessions started with viola run take commands`. It is the existing wording for a driver that targets an unwrapped row. Per design-system §Surface: cli (Component Patterns 2, `instance-unreachable` hints).
- New revive refusals follow the shape of the existing pairs (`no-session`, `cwd-missing`): one fixed-message line plus one hint, and a matching log detail code where one exists. Per design-system §Surface: cli (Component Patterns 2; Exit-code phraseology, row 1).

## Anti-patterns to avoid
- No traffic-light status dots and no colour for `idle` / `busy`: the status word in its fixed column is the whole signal. Per design-system §Anti-Patterns (Rejected Defaults: Traffic-light status dots; Per-Surface Bans, cli: colour only `DIALOG` and `stale`).
- No pid, path, upstream text or anyhow chain in the `session-live` refusal or its hint. The study wording "naming the pid" goes against this. Per design-system §Anti-Patterns (Per-Surface Bans, cli: never print upstream text, paths, pids …).
- No re-sorting of rows by status or recency. Every session keeps a stable slot. Per design-system §Anti-Patterns (Rejected Defaults: Auto-sorting).

## Contract bindings
- layout-templates ↔ design: which `claude agents --json` field fills the unwrapped NAME cell, and whether LIVE is printed for an unwrapped row, belong to the layout source. The design sample shows only `c1f9e2a4  live  idle`, and design-system §Surface: cli names no source field for it.
- security-plan ↔ design: the `\xHH` escape of agents-derived fields is stricter than the security floor, which keeps `\n` and `\t` (ratified T5). This binds to the security plan's NEVER-log floor (design-system §Surface: cli, Component Patterns 1).
- obs-plan ↔ design: design-system §Surface: cli (Component Patterns 2) names the revive log detail codes `no-session` and `cwd-missing` only. A `session-live` detail code needs obs-plan §6 / `schemas/diag-line.v1.json`, and the arch `--json` detail-code amendment is still pending.
- test-plan ↔ design: test-plan §6 E1 (an unwrapped name is never a target) binds to the unwrapped-name `instance-unreachable` hint (design-system §Surface: cli, Component Patterns 2).

## Acceptance criteria contributions
- With at least one unjoined agents row, the human `viola list` prints `-- UNWRAPPED - READ-ONLY --` after the wrapped rows, and each unwrapped row prints `n/a` in WHEEL, DIALOG and CLI. The output is ASCII only and has no SGR (per design-system §Surface: cli (Component Patterns 1)).
- Every row's STATUS is exactly `idle`, `busy` or `unknown`. When the agents read is absent, oversize or malformed, every wrapped row reads `unknown`, no unwrapped rack is printed, and the exit code does not change (per design-system §Color Palette (Domain status colors: `SessionStatus::unknown`)).
- An agents-derived field holding ESC, `\n`, `\t`, DEL or a C1 character prints as `\xHH` text, and its row stays one line (per design-system §Surface: cli (Component Patterns 1)).
- The `session-live` revive refusal prints exactly one `unable:` line and one `hint:` line on stderr and exits 1. Neither line holds a pid, path, session id or `viola release`, and the refusal takes its place in the fixed preflight order (per design-system §Surface: cli (Component Patterns 2)).
