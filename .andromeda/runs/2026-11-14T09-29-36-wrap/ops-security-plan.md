@row_set n=1 row=decision/child-output-tolerant cell=rule seen=1 detector=D-security-input
value: Output of spawned children is parsed tolerantly, and a failure becomes unknown or a closed failure word.
basis: fct-woso52s6, fct-breo2vsc
because: The hyprctl reply read parses tolerantly but reads each failure as one of the seam's closed words (oversize, malformed, unreachable), so a failure that only ever becomes unknown no longer holds.
@row_set n=2 row=decision/child-output-tolerant-unknown cell=rule seen=1 detector=D-security-input
value: Child output parsing is tolerant, and a failure becomes unknown or a closed failure word.
basis: fct-woso52s6, fct-breo2vsc
because: The hyprctl reply read parses tolerantly but reads each failure as one of the seam's closed words, so a failure that only ever becomes unknown no longer holds.
@row_set n=3 row=decision/shared-validators-in-core cell=except seen=1 detector=D-security-input
value: The Hyprland seam's window Address validator lives in the seam crate.
basis: fct-woso52s6
because: The new Address newtype and its predicate stand in viola-hyprland rather than viola-core, so the row's rule needs its named exception to stay true.
@row_set n=4 row=decision/parser-fuzz-scope cell=rule seen=1 detector=D-security-input
value: Paste text validation, channel ndjson framing, the hook stdin parser, the claude agents output parser, the hyprctl reply parsers and the vt100 feed are in scope for fuzz and property coverage.
basis: fct-wnem37fs, fct-arnp5nr7
because: The hyprctl reply parsers are a new external-input parser surface brought under a fuzz target and per-parser property tests, so the row's scope of parser surfaces must name them.
@row_add n=1 row=decision/hyprctl-caller-resolved-program area=input-validation fact=F1 detector=D-security-input
name: hyprctl spawned from a caller-resolved path
rule: The Hyprland seam spawns hyprctl directly, never through a shell, from a program path its caller resolves; the seam itself does no PATH lookup and reads no environment variable.
basis: fct-woso52s6, fct-w4fzcahg
because: The new seam is an external-input boundary whose program resolution is left to its caller, unlike the claude rows that resolve from PATH, and future callers must respect it.
@row_add n=2 row=decision/hyprctl-read-deadline area=input-validation fact=F3 detector=D-security-input
name: hyprctl child bounded by a deadline
rule: The hyprctl child runs with null stdin and the named environment variables removed, under a 5 s deadline at which it is killed and reaped; a deadline kill or a non-zero exit reads as unreachable.
guard: read_past_the_deadline_kills_and_reaps_its_child
state: provisional
basis: fct-woso52s6, fct-cfg2bcyo
because: The seam adds a new spawned child whose deadline is applied ahead of its settlement against a measured need, so the bound and its provisional state must stand for the planner.
@row_add n=3 row=decision/hyprctl-dispatch-closed-values area=security-anti-patterns-never-do-these fact=F4 detector=D-security-input
name: hyprctl dispatch text holds closed values only
rule: hyprctl dispatch text, a Lua expression, never holds an inserted value other than a closed Address (0x plus 1 to 16 lower-case hex digits) or an i32 workspace id, never reply text, and no shell line is built.
why: Hyprland 0.56.2 runs dispatch as Lua.
guard: request_args_are_the_closed_argv_one_element_each
basis: fct-woso52s6, fct-oh77jo2w
because: The measured Lua dispatch form makes the dispatch argument code another program runs, so only closed newtypes may enter it.
@row_add n=4 row=decision/hyprland-fixtures-closed-keep-list area=data-protection fact=F5 detector=none
name: Hyprland fixtures recorded through a closed keep-list
rule: Hyprland fixtures are recorded only through a test-only tap with a closed per-request string keep-list (addresses and the digit version tag); every other string at any depth becomes a placeholder, a non-JSON reply is kept only as the success text ok, then the shared scrub and whole refusal follow.
guard: record_replaces_every_string_off_the_allowlist; hyprland_fixtures_pass_hygiene
basis: fct-w4fzcahg
because: A new recorded fixture class arrives with its own closed keep-list, which the hygiene walk checks too, and no standing row states it.
@row_add n=5 row=decision/hyprland-reply-text-dropped area=data-protection fact=F6 detector=D-security-input
name: Hyprland reply text dropped at the parse
rule: Hyprland reply types hold no text field: window titles, classes and every other reply string are dropped at the parse and reach no log line and no error.
guard: contract_hyprland_canary_reaches_no_line_and_no_error
basis: fct-woso52s6, fct-bwodkdui
because: The new seam reads compositor replies carrying window titles, and the choice to keep none of that text must bind every later caller.
@row_none n=1 fact=F2 type=4
relied: an over-cap hyprctl reply is refused whole as oversize, never read as unreachable; the cap itself already stands in the shared MAX_FRAME rows as child output.
@row_none n=2 fact=F7 type=4
relied: the seam's failures read only as the closed words unsupported, unreachable, oversize, malformed and refused; the fixed-message rule already stands in the error-handling rows.
