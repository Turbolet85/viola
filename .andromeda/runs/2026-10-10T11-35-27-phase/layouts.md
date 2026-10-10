# layouts extract

## Relevance
partial — the chunk adds one verb to the cli surface (its start path, three new refusal lines, a `--list`
output); the web-spa surface gains no element, and layout-templates names no `revive` anywhere, so every
binding below is a general cli rule applied to the new verb, with the unplaced parts named as gaps.

## Constraints
- Verb shape: layout-templates §Surface: cli · Component — Primary navigation (verb structure) requires flat
  one-word verbs in the order `viola <verb> <target>`, `<target>` always a `ViolaName` (an unwrapped name is
  never a valid target), with no interactive prompt, no TUI mode and no pager. The id choice is therefore made
  by flags (`--id`, `--fork`, `--list`), never by a question on screen, and `--list` is never paged.
- Start path: layout-templates §Surface: cli · Output structure — `viola run` requires that the wrapper prints
  nothing while the child holds the terminal, and layout-templates §Surface: cli · Expression level puts
  passthrough at 0.0 (no colour, no non-ASCII glyph, no cursor control). The plan names only `viola run`; that
  this binds `revive` rests on the scope's statement that revive runs the ordinary `run` start order.
- `instance-live`: layout-templates §Surface: cli · Output structure — `viola run` fixes the collision pair
  (the `unable: <name> is already live` line, then `hint: viola list`), on stderr, exit 1, with no path and no
  pid. The scope keeps it unchanged for revive; whether `run` already prints exactly that pair is research's
  question.
- One hint per cause: layout-templates §Surface: cli · Component — Primary content block 2 (Hint line) requires
  a `hint:` line directly under each refusal, keyed by reason, and a hint of its own for every cause of a
  multi-cause exit so an agent tells the causes apart from the last stderr line. `cwd-missing`, `session-live`
  and `no-session` each need a distinct hint.
- What a refusal and a hint may carry: layout-templates §Surface: cli · Component — Primary content block 2
  (Hint line) requires that a hint never quotes sent or upstream text and never names a path or a pid. This
  bounds `cwd-missing` (the recorded cwd is not printed) and collides with the scope's study wording that
  `session-live` "names the pid": that collision is a P4 question, not settled here.
- No `release` hint: layout-templates §Surface: cli · Component — Primary navigation (verb structure,
  Discoverability) requires that no hint names `viola release`. It applies whichever way the founder rules on
  the revived wheel's holder.
- Streams: layout-templates §Surface: cli · Component — Primary content block 2 (Streams) requires results on
  stdout and refusals, hints and errors on stderr, never mixed. The `--list` chain is a result (stdout); the
  four refusals are stderr.

## Patterns to follow
- Two refusal line forms exist, per layout-templates §Surface: cli · Component — Primary content block 2 (Line
  form): the typed `unable  <name>  <reason>  <detail>` line, and the fixed-message `unable: <text>` form of
  `viola run`'s exit-1 start refusals "and its sibling causes". The plan does not say which form the three new
  causes take, and its typed tail (same section, Exit codes) holds no number for them: a gap for P4, with the
  numbers owned by architecture.
- Row output: layout-templates §Surface: cli · Token names on this surface gives the two-space column gutter,
  Data-role timestamps and plain default-foreground ink, and layout-templates §Surface: cli · Component —
  Primary content block 1 (Columns) gives fixed-width padding with no box-drawing borders. These are the
  nearest pattern for the `--list` chain rows; the plan holds no output structure for `revive --list` (the
  scope's `(ts, cause, id)` order and whether a caption row is drawn are unplaced).
- Appended lines: layout-templates §Surface: cli · Output structure — `viola send` requires lines appended and
  never redrawn; layout-templates §Surface: cli · Component — Footer / terminator requires that the last
  printed line says what happened and to whom, with the exit code as the terminator.
- Next-verb breadcrumbs: layout-templates §Surface: cli · Component — Primary navigation (verb structure,
  Discoverability) has hints point to the next verb by name. The one start command the plan prints anywhere is
  the empty-board line of layout-templates §Surface: cli · Output structure — `viola list` (human TTY).
- Machine view: layout-templates §Surface: cli · Component — Primary navigation (verb structure, Global flag)
  makes `--json` the machine view of every data verb, one document on stdout with no stderr line and no hint
  (layout-templates §Surface: cli · Component — Primary content block 2, Exit codes). Whether `revive --list`
  counts as a data verb and takes `--json` is not stated by the plan.

## Anti-patterns to avoid
- No colour, banner or version line on any revive output: layout-templates §Surface: cli · Component — Hero /
  signature output line (Colour) reserves cli colour for `DIALOG` and `stale`, and layout-templates §Surface:
  cli · Component — Header / banner allows one header on any verb, the BAY line of `viola list`.
- No spinner, progress line, `done` or `success`: layout-templates §Surface: cli · Component — Footer /
  terminator bans a terminator word, and layout-templates §Surface: cli · Primary screens (commands) names
  `viola verify`'s step counter as the project's only progress pattern.
- No new human column for the recorded cwd or the session id on the board: layout-templates §Surface: cli · IA
  notes (Output as a contract) makes a seventh `viola list` column a design change, never an implementation
  choice.

## Contract bindings
- architecture §Conventions (refusals, exit codes) and the route entry "Exit-cause code catalogue": the layout
  plan's typed exit tail has no row for `cwd-missing`, `session-live` or `no-session`, and layout-templates
  §Surface: cli · Component — Primary content block 2 (Exit codes) leaves the `--json` cause as a detail code
  only "once arch names those codes".
- design-system cli pattern 2: layout-templates §Surface: cli · Component — Primary content block 2 (Line form)
  points there for the list of `viola run`'s sibling start-refusal causes and their hints; the design extractor
  owns it.
- security-plan §Error Handling: the no-path, no-pid hint rule is the layout face of the fixed-message rule; the
  recorded cwd and the `session-live` pid are the two values it bears on.
- layout-templates itself, owed at wrap: layout-templates §Surface: cli · Output structure — `viola --help`
  lists no `revive` in its grouped verb table (the `setup` group holds `run`), and layout-templates §Surface:
  cli · Primary screens (commands) has no revive line. The group and the screen line are an amendment, and the
  group choice is not stated by the plan.
- The one catch site: layout-templates §Surface: cli · Component — Primary content block 2 (Line form) lists
  the `cli` verbs whose failure prints `error: internal error` once; the list names neither `run` nor `revive`.
  Whether revive joins it is research's question against the code and architecture's against the contract.
- web-spa, read-only: no element is added. Layout-templates §Surface: web-spa · Component — Primary content
  block 1 (Ordering, the stable-slot rule) keeps a revived name in its slot, and layout-templates §Surface:
  web-spa · IA notes (Multi-surface coordination) keeps the strip at `viola list`'s six fields, so the recorded
  cwd has no slot there either.
- The founder's open wheel question: either answer prints a WHEEL word the board already has (`driver` or
  `human`), per layout-templates §Surface: cli · Component — Primary content block 1 (Columns); no new word and
  no new column follows from it.
- a11y: (none) — the chunk adds no focusable element and no announced region.

## Acceptance criteria contributions
- (layouts) `viola revive <name>` on a live name prints the `viola run` collision pair on stderr, the `hint:`
  line last, exit 1, with no path and no pid, and nothing on stdout (per layout-templates §Surface: cli ·
  Output structure — `viola run`)
- (layouts) each of `cwd-missing`, `session-live` and `no-session` prints one refusal line with its own
  distinct `hint:` as the last stderr line, and no hint names a path (the recorded cwd included), a pid, any
  upstream text or `viola release` (per layout-templates §Surface: cli · Component — Primary content block 2,
  Hint line)
- (layouts) a successful revive prints no viola line of its own, on either stream, before or while the child
  holds the terminal: no colour, no non-ASCII glyph, no cursor control (per layout-templates §Surface: cli ·
  Output structure — `viola run`)
- (layouts) `viola revive <name> --list` writes its chain rows to stdout only, plain ASCII with no SGR, each
  line appended once, with no banner and no terminator word, and `viola list`'s human output keeps exactly its
  six columns after the chunk (per layout-templates §Surface: cli · IA notes, Output as a contract)
