proposals:
  - detector: D-layout-surface
    severity: warning
    section: 'Surface: cli → Output structure — `viola --help`'
    change: 'Under the wireframe add an As built note (chunk 2026-10-17-cli-output-discipline): `viola --help` (and `viola help`) prints exactly `Usage: viola [--home <dir>] <verb> [args]`, a blank line, the four rows `board:    list` · `traffic:  send, wait, last, answer` · `wheel:    pause, release` · `setup:    run, revive, verify`, a blank line, `Options:` and the clap options (`--home <HOME>`, `-h, --help`, `-V, --version`); no about line, no clap `Commands:` list; served verbs only, so the wireframe `handoff:  link, unlink` row is added by the route entry "Explicit session links" and `plugin install` in `setup:` by "Optional plugin install" when each serves its verb (the served-verbs rule is the Viola overseer answer `inputs#I1` Q3, a delegate, PROVISIONAL); the usage line is the override literal on every OS, while each verb `--help` stays the clap per-verb usage, unchanged, whose usage line names the program `viola.exe` on Windows.'
    sidecar: 'layout-templates §Output structure — `viola --help`: As built note for the grouped verb table (four served-verb rows plus Options:, no handoff row, no plugin install; per-verb usage names viola.exe on Windows) — chunk 2026-10-17-cli-output-discipline, inputs#I1 Q3 PROVISIONAL.'
    rationale: 'The report Changes → Symbols / APIs (`viola --help`) states the built output: four rows, no `handoff:` row and no `plugin install` (served verbs only, `inputs#I1` Q3, PROVISIONAL), plus `Options:`; the wireframe (layout-templates.md:202-210, report pattern `help` hits :202 / :205) shows five rows and no Options block, so the built surface does not map to its wireframe as drawn. Expected amendments names layout-templates §Output structure — `viola --help` (layout-templates.md:202-210). Owners of the absent rows are the P5 card rows in `inputs#I3`.'
    basis: 'src/cmd/mod.rs:25-37'
  - detector: D-layout-surface
    severity: warning
    section: 'Surface: cli → Primary screens (commands)'
    change: '`viola --help` is the grouped verb table, built (chunk 2026-10-17-cli-output-discipline) with the served verbs only: board, traffic, wheel and setup rows; the `handoff:` row and `plugin install` join when "Explicit session links" and "Optional plugin install" serve those verbs (served verbs only is `inputs#I1` Q3, PROVISIONAL) — retire "not built: the built help is clap ordered list; owed to CLI output discipline".'
    sidecar: 'layout-templates §Primary screens: the `viola --help` bullet reads built (served verbs only), its owed-to note discharged — chunk 2026-10-17-cli-output-discipline.'
    rationale: 'Report Counts [help] and [owner-title] list layout-templates.md:59 as changed ("not built" / "owed to CLI output discipline" now discharged); Symbols / APIs states the grouped table is built.'
    basis: 'src/cmd/mod.rs:25-37'
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: 'Surface: cli → Component — Primary navigation (verb structure)'
    change: 'Discoverability: `--help` groups the served verbs as board / traffic / wheel / setup (as built, chunk 2026-10-17-cli-output-discipline); the handoff group and `plugin install` join when their route entries serve those verbs (`inputs#I1` Q3, PROVISIONAL); the rest of the bullet unchanged.'
    sidecar: 'layout-templates §Primary navigation Discoverability: the built help groups are board / traffic / wheel / setup; handoff joins with its verbs — chunk 2026-10-17-cli-output-discipline.'
    rationale: 'Same retired claim (the help shows a handoff group) restated in the Discoverability bullet, which reads "`--help` groups verbs as board / traffic / wheel / handoff / setup"; the report Symbols / APIs states no `handoff:` row is printed.'
    basis: 'src/cmd/mod.rs:25-37'
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: 'Surface: cli → Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`) → As built'
    change: 'Amend the As built paragraph (add chunk 2026-10-17-cli-output-discipline): the styled human-TTY form is built — on a terminal with colour (none under `--json`, `NO_COLOR` present, an empty value included, `TERM=dumb`, a non-terminal stdout, or a failed Windows VT enable) every NAME value is bold, wrapped and unwrapped; the `DIALOG` word of a wrapped row with a pending dialog takes the attention token, its kind word plain; a wrapped `stale` row is dim from first byte to last, NAME dim and bold, and on a stale row with a dialog only `DIALOG` is amber and not dim; the BAY line, caption, empty-board line and separator carry no SGR; still one stdout write; depth truecolor / 256 / 16 from `COLORTERM` / `TERM`. Width: with a known terminal width (`viola_pty::host_size()`, no `COLUMNS` read; a piped stdout reads none; the source is `inputs#I1` Q2, PROVISIONAL) NAME is padded to the largest of 4, the longest wrapped NAME and the longest unwrapped NAME capped at width − 65, and an unwrapped NAME longer than the column is cut to column − 3 characters plus `...`, never splitting a `\xHH` escape; a wrapped NAME is never cut; with no known width the rows are as before, uncut. Retire "uncoloured form on every stream, a TTY included", "not built, owed to CLI output discipline", "never truncated (the wireframe c1f9e2a4... clipping is not built)" and the closing "nothing is truncated".'
    sidecar: 'layout-templates `viola list` As built: the colour layer (bold NAME, amber DIALOG, dim stale row; off under --json / NO_COLOR / TERM=dumb / pipe) and the width cut of unwrapped NAMEs from host_size() built — chunk 2026-10-17-cli-output-discipline; width source inputs#I1 Q2 PROVISIONAL.'
    rationale: 'Report Symbols / APIs (`src/human/board.rs`, `src/cmd/list.rs`, `src/human/style.rs`) states the colour spans and the width cut are built; Counts [colour-layer], [owner-title] and [width] list layout-templates.md:91 as changed (uncoloured, owed to CLI output discipline, never truncated); Expected amendments names §Output structure — `viola list` As built (layout-templates.md:91).'
    basis: 'src/cmd/list.rs:60-71'
  - detector: D-layout-surface
    severity: warning
    section: 'Surface: cli → Component — Primary content block 1: `viola list` strip rows'
    change: 'As built: replace "no styling, no truncation" with the built styling and cut (chunk 2026-10-17-cli-output-discipline): NAME bold, the `DIALOG` word amber with its kind word plain, a `stale` row dim end to end with only `DIALOG` amber and not dim, nothing else styled, no SGR when colour is off; only an unwrapped NAME is cut, with `...`, and only on a terminal whose width is known and too narrow for it, never inside an escape; wrapped NAMEs are never cut.'
    sidecar: 'layout-templates Primary content block 1 As built: styling and the unwrapped-NAME cut built — chunk 2026-10-17-cli-output-discipline.'
    rationale: 'Report Counts [width] lists layout-templates.md:298 ("no styling, no truncation") as changed; Expected amendments names §Component — Primary content block 1 (layout-templates.md:298); Symbols / APIs `src/human/board.rs` states the spans and the cut.'
    basis: 'src/human/board.rs:192-195'
    dependent-of: D-layout-surface
note: 'Two primaries share the one detector: the `viola --help` wireframe (dependents :59 and Discoverability) and the `viola list` As built at :91 (dependent :298). Both PROVISIONAL authorities (inputs#I1 Q2 width source, Q3 served verbs only) are a delegate answer carried to the founder next batch; lines :23, :29, :42, :71, :79, :89 are no-change per the report Counts.'
