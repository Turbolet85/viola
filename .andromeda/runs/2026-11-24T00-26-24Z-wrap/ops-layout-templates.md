@row_set n=1 row=contract/cli-verbs/help cell=promise seen=1 detector=D-design-cli-wording
value: The grouped verb table with the served verbs only: the board, traffic, wheel, handoff and setup rows, `handoff:` holding link, unlink and `setup:` holding open, run, revive, verify and plugin install.
basis: fct-xxzbm355
because: The help's `setup:` row gains `open`, so the row naming what `setup:` holds must name it.
@row_set n=2 row=contract/viola-help/verb-groups cell=promise seen=1 detector=D-design-cli-wording
value: Five rows `board:    list`, `traffic:  send, wait, last, answer`, `wheel:    pause, release`, `handoff:  link, unlink`, `setup:    open, run, revive, verify, plugin install`, served verbs only, plain ASCII, then a blank line, `Options:` and clap's options; no about line and no clap `Commands:` list.
basis: fct-xxzbm355
because: The help's `setup:` row now reads `open, run, revive, verify, plugin install`, so the stated row wording is stale.
@row_add n=1 row=contract/cli-verbs/open area=surface-cli fact=F1 detector=D-layout-surface
boundary: CLI verbs
item: viola open <name>
promise: Starts a wrapped claude session in a new foot terminal window beside its parent; its flags are a closed allowlist (--parent, --role overseer|builder|worker, --cwd, --workspace, --model, --effort, --json), the only words allowed after `--` are `claude` alone, and no shell runs.
basis: fct-xxzbm355
because: The chunk adds the viola open verb, a new CLI surface the verb rows must describe.
@row_add n=2 row=contract/viola-open/usage-refusals area=surface-cli fact=F2 detector=none
boundary: viola open
item: usage refusals
promise: A flag outside the allowlist, any program after `--` other than `claude` alone (a fixed usage line that echoes none of the words), or a parent naming itself is a usage refusal, exit 2, before any home read, log, pin or spawn.
basis: fct-xxzbm355
because: viola open's usage refusals are a new exit-2 contract of the CLI surface.
@row_add n=3 row=contract/viola-open-output/live area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola open output
item: live
promise: Exit 0 prints `<name>  live  <placement>` on stdout.
basis: fct-xxzbm355
because: viola open adds a new printed human line that the masters must state.
@row_add n=4 row=contract/viola-open-output/json-ok area=surface-cli fact=F3 detector=none
boundary: viola open output
item: --json ok
promise: Under `--json` exit 0 prints the one ok document `{"v":1,"ok":{"name","parent","window":{"address","workspace"}|null,"placement"}}`.
basis: fct-xxzbm355
because: viola open adds a new --json ok document at the CLI boundary.
@row_add n=5 row=contract/viola-open-json/refusal-document area=surface-cli fact=F4 detector=none
boundary: viola open --json
item: refusal document
promise: Each exit-1 refusal (strict-modes-failed, already-live, stale-heartbeat, pinned-hash-mismatch, terminal-not-found, not-live, internal-error) is the one document `{"v":1,"error":"<code>","detail":null}` under `--json`.
basis: fct-xxzbm355
because: viola open's exit-1 codes and their --json document are a new contract of the CLI surface.
@row_add n=6 row=contract/viola-open-refusals/terminal-not-found area=surface-cli fact=F5 detector=D-design-cli-wording
boundary: viola open refusals
item: terminal-not-found
promise: With no foot terminal found to spawn, viola open exits 1 with an `unable:` line and its one fixed hint, spawning nothing; under `--json` the `terminal-not-found` document.
basis: fct-xxzbm355
because: terminal-not-found is a new exit-1 refusal cause with its own hint.
@row_add n=7 row=contract/viola-open-refusals/not-live area=surface-cli fact=F5 detector=D-design-cli-wording
boundary: viola open refusals
item: not-live
promise: An opened session that never comes live within its 30 s bound makes viola open exit 1 with an `unable:` line and its one fixed hint; under `--json` the `not-live` document.
basis: fct-xxzbm355
because: not-live is a new exit-1 refusal cause with its own hint.
@row_add n=8 row=contract/viola-open/parent-default area=surface-cli fact=F6 detector=none
boundary: viola open
item: --parent default
promise: The parent defaults to the calling agent's own ViolaName, recorded with origin `agent`; from a plain shell there is no parent and the origin is `cli`.
basis: fct-xxzbm355
because: The parent default and its recorded origin are a new promise of the open verb.
@row_add n=9 row=contract/viola-run/hidden-window-flags area=surface-cli fact=F7 detector=none
boundary: viola run
item: --origin and --place
promise: `--origin <cli|agent>` and `--place <N>` are hidden flags absent from help, and `--place` without `--origin` is a usage error.
basis: fct-xxzbm355
because: viola run gains two hidden flags with a pairing rule at its argument boundary.
@row_add n=10 row=contract/viola-revive/no-window area=surface-cli fact=F7 detector=none
boundary: viola revive
item: window placement
promise: viola revive never places a window.
basis: fct-xxzbm355
because: The window step reaches only a start that asks for it, and revive hands none.
@row_add n=11 row=decision/closed-placement-and-origin-words area=surface-cli fact=F8 detector=none
name: Closed placement and origin words
rule: The placement word is one of a closed set of eight (placed, unplaced, window-unseen, unsupported, unreachable, oversize, malformed, refused) and the origin word one of a closed set of two (cli, agent).
basis: fct-xxzbm355
because: viola open prints a placement word and list --json carries an origin word, both from closed vocabularies the CLI now promises.
@row_add n=12 row=decision/open-is-cli-only area=surface-cli fact=F9 detector=none
name: Opening a window is CLI-only
rule: viola open has no MCP tool and no IPC method, so opening a window is a CLI-only action.
basis: fct-xxzbm355
because: The chunk adds no MCP tool or IPC method for open, a deliberate absence future work must respect.
@row_add n=13 row=contract/viola-list-json/window-origin area=surface-cli detector=none
boundary: viola list --json
item: window and origin
promise: An item may carry `window` `{address, workspace}`, only for an address that re-parses, and `origin`, only for an origin word.
basis: fct-wrqnldxo
because: The list --json item gains optional window and origin fields that the list document row does not state.
