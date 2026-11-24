@row_add n=1 row=decision/open-live-wait-bounds-the-driver area=cognitive-accessibility fact=F1 detector=none
name: viola open's live wait bounds the driver
rule: viola open waits at most 30 s for the session it opened to go live; the bound falls on the open verb's own run and ends in the not-live refusal, exit 1, with its hint, never on the human.
basis: fct-xxzbm355, fct-ofc4imyq, fct-m35lqujh
because: The chunk adds a new time bound, open's live wait, which the report places on the open verb's own run, and a11y-plan holds each time bound as a row of its own.
@row_add n=2 row=decision/window-wait-bounds-the-wrapper-start area=cognitive-accessibility fact=F2 detector=none
name: Window wait bounds the wrapper's own start
rule: The wrapper's window step waits at most 5 s for its terminal's window under --origin; the bound falls on the wrapper's own start, and past it the placement is window-unseen and the start goes on, never blocking, refusing or delaying a human key.
basis: fct-xxzbm355, fct-ofc4imyq, fct-m35lqujh
because: The chunk adds a new time bound, the window step's wait, which the report places on the wrapper's own start, and a11y-plan holds each time bound as a row of its own.
@row_add n=3 row=decision/hyprctl-deadline-bounds-the-driver area=cognitive-accessibility fact=F3 detector=none
name: hyprctl call deadline bounds the driver
rule: Each hyprctl call is bounded by a compiled 5 s deadline; the bound falls on the driver (viola open's parent read and the wrapper's window step), never on the human.
basis: fct-ofc4imyq, fct-4kn64u2g
because: The Hyprland seam gains its first two product callers, so its per-call deadline now bounds product work and a11y-plan holds it as a time-bound row of its own.
@row_add n=4 row=contract/open-refusal/terminal-not-found area=cognitive-accessibility fact=F4 detector=none
boundary: viola open refusals, exit 1
item: terminal-not-found
promise: No foot terminal was found and nothing was spawned: exit 1 with one fixed unable/hint pair, or under --json one {"v":1,"error":"terminal-not-found","detail":null} document.
basis: fct-xxzbm355, fct-4kn64u2g
because: viola open adds a new exit-1 refusal code with its own fixed pair and --json document, a promise at the CLI boundary that a11y-plan's refusal contracts carry.
@row_add n=5 row=contract/open-refusal/not-live area=cognitive-accessibility fact=F4 detector=none
boundary: viola open refusals, exit 1
item: not-live
promise: The opened session did not go live within its bound: exit 1 with one fixed unable/hint pair, or under --json one {"v":1,"error":"not-live","detail":null} document.
basis: fct-xxzbm355, fct-4kn64u2g
because: viola open adds a new exit-1 refusal code with its own fixed pair and --json document, a promise at the CLI boundary that a11y-plan's refusal contracts carry.
@row_add n=6 row=contract/viola-open/live-line area=a11y-scope-summary fact=F6 detector=none
boundary: viola open output
item: live line
promise: On success viola open prints the human line `<name>  live  <placement>`.
basis: fct-xxzbm355
because: viola open is a new verb whose success line is a fixed output promise at the CLI boundary, and no standing row says it.
@row_add n=7 row=contract/viola-open/json-ok area=a11y-scope-summary fact=F6 detector=none
boundary: viola open output
item: --json ok document
promise: Under --json, a successful viola open prints the one document {"v":1,"ok":{"name","parent","window":{"address","workspace"}|null,"placement"}}.
basis: fct-xxzbm355
because: viola open is a new verb whose --json success document is a fixed output promise at the CLI boundary, and no standing row says it.
@row_add n=8 row=decision/placement-carried-by-printed-word area=visual-design-verification fact=F7 detector=none
name: Placement carried by its printed word
rule: A session's window placement is carried by one printed word from a closed list of eight: placed, unplaced, window-unseen, unsupported, unreachable, oversize, malformed, refused.
basis: fct-xxzbm355
because: The chunk adds a placement state printed on viola open's live line from a closed word list, which the board-state row does not cover since placement is not a board state.
@row_set n=1 row=contract/cli-exit-codes/2 cell=promise seen=1 fact=F5 detector=none
value: A usage error, or an unusable answer response or send text, or a viola open word after -- other than claude alone or a parent equal to the name, written as one fixed stderr line with no document that echoes none of the refused words.
basis: fct-xxzbm355, fct-4kn64u2g
because: viola open adds an exit-2 refusal written as one fixed usage line and no error body, a cause the exit-2 promise did not name.
@row_set n=2 row=contract/cli-refusal/json cell=promise seen=1 detector=none
value: Under `--json` a refusal gives one document with no hint, except the `viola answer`, `viola send` and `viola open` exit-2 paths, which print a fixed stderr line and no document.
basis: fct-xxzbm355, fct-uspycncu
because: viola open's exit-2 program refusal is a fixed line counted among the messages that are no error bodies, so the exception list must name it.
@row_set n=3 row=decision/json-one-document-no-hint cell=except seen=1 detector=none
value: the viola answer, viola send and viola open exit-2 paths write a fixed line to stderr whatever --json says, and no document
basis: fct-xxzbm355, fct-uspycncu
because: viola open's exit-2 program refusal is a fixed line counted among the messages that are no error bodies, so the exception must name it.
@row_set n=4 row=decision/screen-reader-target-tui cell=rule seen=1 fact=F8 detector=none
value: Manual screen-reader checks of the tui, a session viola open starts in its own foot window among them, use Orca with foot on Hyprland on Linux.
basis: fct-xxzbm355, fct-4kn64u2g
because: viola open starts each session in its own foot window on Hyprland, the tui's screen-reader target, so the target row must name those sessions.
