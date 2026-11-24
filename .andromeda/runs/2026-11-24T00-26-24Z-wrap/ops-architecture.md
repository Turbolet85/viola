@row_set n=1 row=decision/snapshot-replay-log-fields-only cell=rule seen=4 fact=F10 detector=none
value: A snapshot rebuilt by replay recovers only links, agent_session_id, wheel, lineage, launch and window from the log; a field no line gave is absent, never a default.
basis: fct-wrqnldxo
because: The replay now recovers the window record from the log, so the closed list of recovered fields gains window.
@row_set n=2 row=contract/snapshot-envelope/unreadable-snapshot cell=promise seen=4 detector=none
value: A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by replaying `events.ndjson`, recovering only `links`, `agent_session_id`, `wheel`, `lineage`, `launch`, `window`; the replay writes no file.
basis: fct-wrqnldxo
because: The replay now recovers `window`, so the list of fields an unreadable snapshot's rebuild recovers gains it.
@row_set n=3 row=contract/instance-snapshot/data cell=promise seen=5 detector=D-arch-resources
value: `data` is `{endpoint?, pid, started_at, pinned_bin, cli_version?, cli_verified, wheel, pending_dialog?, links, child_pid?, agent_session_id?, cwd?, lineage?, launch?, window?}`.
basis: fct-wrqnldxo
because: The instance snapshot gained an optional `window` field, so the promised shape of `data` gains it.
@row_set n=4 row=contract/list-payload/items cell=promise seen=4 detector=D-arch-resources
value: Each `items` entry is `{name, wrapped, liveness, status, wheel?, dialog_pending, cli_version?, cli_verified?, parent?, role?, model?, effort?, window?, origin?}`.
basis: fct-wrqnldxo
because: A list item on CLI list and MCP list now carries optional `window` and `origin`, so the promised item shape gains both.
@row_set n=5 row=decision/wrapper-events-log-only cell=rule seen=5 detector=none
value: Event kinds the wrapper appends (`wheel`, `lineage`, `launch`, `window`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`.
basis: fct-xxzbm355
because: The wrapper now appends a log-only `window` line not in the wait wake set, so the list of wrapper-appended log-only kinds gains it.
@row_set n=6 row=contract/cli-exit-codes/1 cell=promise seen=1 detector=none
value: Internal error, and the start, revive-preflight, open and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document.
basis: fct-xxzbm355
because: viola open now exits 1 on its own refusals, terminal-not-found and not-live among them, each with a hint pair or one error document, so exit 1 covers the open's refusals too.
@row_set n=7 row=decision/no-start-over-live-instance cell=rule seen=1 detector=none
value: A `viola run`, `viola revive` or `viola open` whose `ViolaName` is `live` or `stale` refuses to start.
basis: fct-xxzbm355
because: viola open refuses already-live and stale-heartbeat before any spawn, so the rule that no start goes over a live or stale instance now holds for open as well.
@row_add n=1 row=name/command/open area=occupied-resources fact=F1 detector=D-arch-resources
class: command
name: open
basis: fct-xxzbm355
because: The chunk lands the new CLI verb viola open, an occupied command name the register must hold.
@row_add n=2 row=decision/open-closed-terminal-spawn area=established-decisions fact=F2 detector=D-arch-resources
name: viola open spawns its terminal with a closed argv
rule: viola open starts a session only in a foot terminal it spawns directly with a closed argv, one element per value and no shell; it takes only its allowlisted flags and claude alone after --, and waits for the session to go live within the compiled bound OPEN_LIVE_BOUND.
basis: fct-xxzbm355
because: The new open verb fixes how a session is launched in a new window, a decision later launch work must respect.
@row_add n=3 row=decision/open-window-target area=established-decisions fact=F3 detector=none
name: Where viola open places its window
rule: viola open targets the workspace of the parent's recorded window first, otherwise that of the window of the nearest process ancestor within the compiled ANCESTRY depth; with neither, the window is left unplaced.
basis: fct-xxzbm355
because: The open verb chooses its window's workspace by a fixed order, a placement decision future window work must follow.
@row_add n=4 row=decision/wrapper-window-step area=established-decisions fact=F4 detector=none
name: Wrapper window step bounded with one move
rule: The wrapper's window step waits for its own terminal's window within the compiled WINDOW_WAIT bound, polling at WINDOW_POLL, and makes at most one move to the asked workspace.
basis: fct-xxzbm355
because: The wrapper gained a bounded window step through the Hyprland seam, whose bound and single move later window work must respect.
@row_add n=5 row=decision/window-origin-self-reported area=established-decisions fact=F5 detector=none
name: Window origin self-reported, never verified
rule: A window's origin is self-reported through viola run's hidden --origin flag and is never verified.
basis: fct-xxzbm355
because: The window record's origin comes from a hidden flag the starter sets, a trust limit any reader of origin must know.
@row_add n=6 row=contract/normalised-event-kinds/window area=conventions fact=F6 detector=D-arch-resources
boundary: normalised event kinds
item: window
promise: `window` is a normalised event kind, appended by the wrapper, log-only, that never wakes `wait`.
basis: fct-xxzbm355
because: viola-core gained the `window` event kind, a new member of the closed normalised kind set.
@row_add n=7 row=contract/window-placement/word-set area=conventions fact=F7 detector=D-arch-resources
boundary: Window placement words
item: word set
promise: A window's placement is one of a closed set defined in viola-core: placed, unplaced, window-unseen, unsupported, unreachable, oversize, malformed, refused.
basis: fct-xxzbm355
because: viola-core now defines the closed placement words the event line, snapshot and open document carry.
@row_add n=8 row=contract/window-origin/word-set area=conventions fact=F7 detector=D-arch-resources
boundary: Window origin words
item: word set
promise: A window's origin is one of a closed set defined in viola-core: cli, agent.
basis: fct-xxzbm355
because: viola-core now defines the closed origin words the hidden run flag, event line, snapshot and list item carry.
@row_add n=9 row=contract/wrapper-event-appends/window area=standard-contracts fact=F8 detector=D-arch-resources
boundary: wrapper event appends
item: `window`
promise: The wrapper appends one `window` line with `source: wrapper` after the `launch` line, only when started under --origin; it is log-only and never wakes `wait`.
basis: fct-wrqnldxo
because: The wrapper now appends a window line at start, a new wrapper append the registry must hold.
@row_add n=10 row=contract/event-data/window area=standard-contracts detector=D-arch-resources
boundary: event data
item: `window`
promise: `{address, workspace, placement, origin}`, with `source: wrapper`; placement and origin are closed words.
basis: fct-wrqnldxo
because: The new window event carries its own data shape, a contract its readers rely on.
@row_add n=11 row=contract/instance-snapshot/window area=standard-contracts fact=F9 detector=D-arch-resources
boundary: instance snapshot
item: window
promise: `window` is `{address?, workspace?, placement, origin}`, the wrapper's window record, omitted when absent; a replay rebuilds it from the log, the last `window` line after the last `launch` line winning and a later `launch` line clearing it.
basis: fct-wrqnldxo
because: The instance snapshot gained an optional window record with its own replay rule, a contract its readers rely on.
@row_add n=12 row=contract/list-item/window area=standard-contracts fact=F11 detector=D-arch-resources
boundary: list payload item
item: window
promise: `{address, workspace}`, present only when the recorded window's address re-parses, on CLI list and MCP list alike.
basis: fct-wrqnldxo
because: The board item now carries an optional window on both list surfaces, a new item field.
@row_add n=13 row=contract/list-item/origin area=standard-contracts fact=F11 detector=D-arch-resources
boundary: list payload item
item: origin
promise: Present only when the recorded origin is an origin word, on CLI list and MCP list alike.
basis: fct-wrqnldxo
because: The board item now carries an optional origin on both list surfaces, a new item field.
@row_add n=14 row=contract/viola-open-exit-codes/0 area=standard-contracts fact=F12 detector=D-arch-resources
boundary: viola open exit codes
item: 0
promise: The session went live: the human line `<name>  live  <placement>`, or under --json `{"v":1,"ok":{"name","parent","window","placement"}}`, window `{address, workspace}` or null.
basis: fct-xxzbm355
because: The new open verb promises its exit 0 and its one ok document to callers.
@row_add n=15 row=contract/viola-open-exit-codes/1 area=standard-contracts fact=F12 detector=D-arch-resources
boundary: viola open exit codes
item: 1
promise: A refusal: strict-modes-failed before any log, already-live, stale-heartbeat, pinned-hash-mismatch, terminal-not-found, not-live or internal-error, each under --json the `{"v":1,"error":"<code>","detail":null}` document.
basis: fct-xxzbm355
because: The new open verb promises its exit-1 refusal codes and their document to callers.
@row_add n=16 row=contract/viola-open-exit-codes/2 area=standard-contracts fact=F12 detector=D-arch-resources
boundary: viola open exit codes
item: 2
promise: A usage error: a clap error, any word after -- but claude alone, or a parent equal to the opened name.
basis: fct-xxzbm355
because: The new open verb promises its usage exit to callers.
@row_add n=17 row=contract/process-exit-detail/terminal-not-found area=occupied-resources fact=F13 detector=D-arch-resources
boundary: process-exit detail enum
item: terminal-not-found
promise: A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola open.
basis: fct-wrqnldxo
because: The diag-line schema gained the process-exit detail terminal-not-found for viola open.
@row_add n=18 row=contract/process-exit-detail/not-live area=occupied-resources fact=F13 detector=D-arch-resources
boundary: process-exit detail enum
item: not-live
promise: A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola open.
basis: fct-wrqnldxo
because: The diag-line schema gained the process-exit detail not-live for viola open.
@row_add n=19 row=contract/diag-subject/terminal area=occupied-resources fact=F13 detector=D-arch-resources
boundary: diag-line subject enum
item: terminal
promise: A word of the subject enum in schemas/diag-line.v1.json, the subject of viola open's terminal spawn.
basis: fct-wrqnldxo
because: The diag-line schema gained the subject terminal for viola open's spawn.
@row_add n=20 row=contract/viola-cli-arguments/run-origin area=occupied-resources fact=F14 detector=D-arch-resources
boundary: viola CLI arguments
item: run --origin
promise: viola run --origin <WORD> is a hidden flag naming the window's origin, an origin word; any other word is a usage error, exit 2.
basis: fct-xxzbm355
because: viola run gained the hidden --origin flag, an occupied argument name.
@row_add n=21 row=contract/viola-cli-arguments/run-place area=occupied-resources fact=F14 detector=D-arch-resources
boundary: viola CLI arguments
item: run --place
promise: viola run --place <N> is a hidden flag naming the workspace the window is moved to; --place without --origin is a usage error, exit 2.
basis: fct-xxzbm355
because: viola run gained the hidden --place flag, an occupied argument name bound to --origin.
@row_add n=22 row=decision/state-depends-on-hyprland area=crate-dependency-direction fact=F15 detector=D-arch-resources
name: viola-state and viola depend on viola-hyprland
rule: viola-state depends on viola-hyprland, whose address check is the one window-address validator, and the viola bin takes viola-hyprland as a normal dependency in its release graph.
basis: fct-xvpboldf
because: The chunk added the viola-state to viola-hyprland edge and moved viola's edge out of dev-dependencies, a new crate direction future work must respect.
@row_add n=23 row=contract/cli-verbs/open area=conventions detector=D-arch-resources
boundary: CLI verbs
item: open
promise: `viola open <name> [--parent <name>] [--role overseer|builder|worker] [--cwd <dir>] [--workspace N] [--model M] [--effort E] [--json] -- claude`.
basis: fct-xxzbm355
because: The new open verb's argument surface joins the CLI verbs the conventions list.
@row_add n=24 row=decision/window-step-in-start-order area=established-decisions detector=none
name: Window step placed in the start order
rule: Under --origin a start places its terminal's window after the spawn folder's trust record and before the first snapshot, so the window record stands in the first snapshot.
basis: fct-xxzbm355
because: The wrapper's window step now runs inside the start order, and the standing start-order row cannot take that step within its bound.
