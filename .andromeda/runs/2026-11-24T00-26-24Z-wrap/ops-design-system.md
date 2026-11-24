@row_set n=1 row=contract/help-output/verb-groups cell=promise seen=1 detector=D-design-cli-wording
value: `viola --help` lists the served verbs as `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, verify, plugin install`, then `Options:`.
basis: fct-xxzbm355, fct-uspycncu
because: The help's `setup:` row now opens with `open`, so the row's listing of its members no longer holds.
@row_set n=2 row=contract/viola-help/verb-groups cell=promise seen=1 detector=D-design-cli-wording
value: `viola --help` lists the served verbs only, grouped `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, verify, plugin install`, then `Options:`.
basis: fct-xxzbm355, fct-uspycncu
because: The help's `setup:` row now opens with `open`, so the row's listing of its members no longer holds.
@row_set n=3 row=contract/cli-exit-codes/1 cell=promise seen=1 fact=F2 detector=D-design-cli-wording
value: `error: internal error` (fixed; no chain, paths or hint), or a fixed `unable:` refusal of `run`, `revive`, `open` or `verify` with its hint; `--json` only for `revive`'s shared-start refusals, `open`'s refusals and the catch site's `{"v":1,"error":"internal-error","detail":null}`, stderr empty.
basis: fct-xxzbm355
because: `viola open` exits 1 with its own refusals, each a `{"v":1,"error":"<code>","detail":null}` document under `--json`, which the row's verb list and `--json` clause leave out.
@row_set n=4 row=contract/cli-stderr/fixed-message-refusal cell=promise seen=1 detector=D-design-cli-wording
value: The exit-1 refusals of `viola run`, `revive`, `open` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` use `unable: <text>`.
basis: fct-xxzbm355, fct-4kn64u2g
because: `viola open` adds exit-1 refusals printed as fixed `unable:` / `hint:` pairs, so the row's list of verbs using the fixed form is incomplete.
@row_set n=5 row=contract/cli-refusal-line/fixed-unable-form cell=promise seen=1 detector=D-design-cli-wording
value: The exit-1 refusals of `run`, `revive`, `open` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.
basis: fct-xxzbm355, fct-4kn64u2g
because: `viola open` adds exit-1 refusals printed as fixed `unable:` / `hint:` pairs, so the row's list of verbs using the fixed form is incomplete.
@row_add n=1 row=name/command/open area=surface-cli fact=F1 detector=none
class: command
name: open
basis: fct-xxzbm355
because: The chunk adds the CLI verb `viola open`, an occupied command name the register must hold.
@row_add n=2 row=decision/open-closed-argument-surface area=surface-cli detector=none
name: open takes a closed allowlist and claude alone
rule: `viola open` takes only `--parent`, `--role` (overseer, builder or worker), `--cwd`, `--workspace`, `--model`, `--effort` and `--json`, and after `--` the one program `claude` alone; it runs no shell and hands the terminal a closed argv, one element per value.
basis: fct-xxzbm355, fct-4kn64u2g
because: The new setup verb opens one terminal window per session beside its parent on a closed argument surface that future verbs and flags must respect.
@row_add n=3 row=contract/viola-open-output/usage-refusal area=surface-cli detector=D-design-cli-wording
boundary: viola open output
item: usage refusal
promise: A flag outside the allowlist, a word after `--` other than `claude` alone, or a self parent exits 2 with clap's usage text or the fixed `OPEN_PROGRAM` line, before any home read, log or spawn; no document.
basis: fct-xxzbm355, fct-4kn64u2g
because: `viola open` adds an exit-2 usage line of its own that the CLI exit-code rows do not state.
@row_add n=4 row=contract/viola-open-output/json-refusal area=surface-cli detector=D-design-cli-wording
boundary: viola open output
item: --json refusal
promise: Under `--json` every exit-1 refusal of `viola open` (`strict-modes-failed`, `already-live`, `stale-heartbeat`, `pinned-hash-mismatch`, `terminal-not-found`, `not-live`, `internal-error`) prints one document `{"v":1,"error":"<code>","detail":null}`.
basis: fct-xxzbm355
because: The open verb's exit-1 codes and their `--json` document are a new contract of the CLI surface.
@row_add n=5 row=contract/viola-open-output/live-line area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola open output
item: live line
promise: Exit 0 prints the human line `<name>  live  <placement>`, two spaces between fields.
basis: fct-xxzbm355
because: The open verb's success line is a new printed wording of the CLI surface.
@row_add n=6 row=contract/viola-open-json/ok area=surface-cli detector=D-design-cli-wording
boundary: viola open --json
item: ok
promise: Exit 0 prints one document `{"v":1,"ok":{"name","parent","window","placement"}}`, `window` being `{"address","workspace"}` or null.
basis: fct-xxzbm355
because: The open verb's ok document is a new `--json` contract of the CLI surface.
@row_add n=7 row=contract/viola-open-refusals/strict-modes area=surface-cli fact=F6 detector=none
boundary: viola open refusals
item: strict-modes-failed
promise: A refused strict-modes check of `viola open` is made before any log and exits 1, not the instance strict-modes refusal's 21; under `--json` it prints the `strict-modes-failed` document with `detail` null.
basis: fct-xxzbm355
because: The open verb's strict-modes refusal departs from the exit-21 rule the standing strict-modes rows state for other verbs.
@row_add n=8 row=contract/placement-word/placed area=surface-cli fact=F4 detector=none
boundary: placement word
item: placed
promise: One of the eight closed placement words `viola open` prints and the window record carries: the window was moved to the asked workspace.
basis: fct-xxzbm355, fct-wrqnldxo
because: The chunk adds a closed list of eight placement words to the open's output, one row an item.
@row_add n=9 row=contract/placement-word/unplaced area=surface-cli detector=none
boundary: placement word
item: unplaced
promise: One of the eight closed placement words `viola open` prints and the window record carries: no workspace was asked.
basis: fct-xxzbm355, fct-4kn64u2g
because: The chunk adds a closed list of eight placement words to the open's output, one row an item.
@row_add n=10 row=contract/placement-word/window-unseen area=surface-cli detector=none
boundary: placement word
item: window-unseen
promise: One of the eight closed placement words `viola open` prints and the window record carries: no window of the wrapper's own terminal was seen.
basis: fct-xxzbm355, fct-4kn64u2g
because: The chunk adds a closed list of eight placement words to the open's output, one row an item.
@row_add n=11 row=contract/placement-word/unsupported area=surface-cli detector=none
boundary: placement word
item: unsupported
promise: One of the eight closed placement words `viola open` prints and the window record carries: off Hyprland, the open still live.
basis: fct-xxzbm355, fct-4kn64u2g
because: The chunk adds a closed list of eight placement words to the open's output, one row an item.
@row_add n=12 row=contract/placement-word/unreachable area=surface-cli detector=none
boundary: placement word
item: unreachable
promise: One of the eight closed placement words `viola open` prints and the window record carries.
basis: fct-xxzbm355
because: The chunk adds a closed list of eight placement words to the open's output, one row an item.
@row_add n=13 row=contract/placement-word/oversize area=surface-cli detector=none
boundary: placement word
item: oversize
promise: One of the eight closed placement words `viola open` prints and the window record carries.
basis: fct-xxzbm355
because: The chunk adds a closed list of eight placement words to the open's output, one row an item.
@row_add n=14 row=contract/placement-word/malformed area=surface-cli detector=none
boundary: placement word
item: malformed
promise: One of the eight closed placement words `viola open` prints and the window record carries.
basis: fct-xxzbm355
because: The chunk adds a closed list of eight placement words to the open's output, one row an item.
@row_add n=15 row=contract/placement-word/refused area=surface-cli detector=none
boundary: placement word
item: refused
promise: One of the eight closed placement words `viola open` prints and the window record carries: the move was refused.
basis: fct-xxzbm355, fct-4kn64u2g
because: The chunk adds a closed list of eight placement words to the open's output, one row an item.
@row_add n=16 row=contract/origin-word/cli area=surface-cli detector=none
boundary: origin word
item: cli
promise: One of the two closed origin words: an open made from a human shell.
basis: fct-xxzbm355
because: The chunk adds a closed list of two origin words carried by the window record and the list items, one row an item.
@row_add n=17 row=contract/origin-word/agent area=surface-cli detector=none
boundary: origin word
item: agent
promise: One of the two closed origin words: an open made from inside a wrapped session.
basis: fct-xxzbm355
because: The chunk adds a closed list of two origin words carried by the window record and the list items, one row an item.
@row_add n=18 row=decision/open-parent-defaults-to-caller area=surface-cli fact=F9 detector=none
name: open's parent defaults to the calling session
rule: An open made from inside a wrapped session names that session as its parent by default, with origin `agent`; an open from a human shell has origin `cli`.
basis: fct-xxzbm355, fct-4kn64u2g
because: The open verb reads the calling session's name for its parent default and origin, a convention future verbs launched by agents must respect.
@row_add n=19 row=contract/viola-open-refusals/terminal-not-found area=surface fact=F5 detector=D-design-cli-wording
boundary: viola open refusals
item: terminal-not-found
promise: `terminal-not-found` is an exit-1 refusal of `viola open` with one fixed `unable:` / `hint:` pair, spawning nothing.
basis: fct-xxzbm355, fct-4kn64u2g
because: The chunk adds the refusal code `terminal-not-found` with its own fixed hint, which no refusal row states.
@row_add n=20 row=contract/viola-open-refusals/not-live area=surface detector=D-design-cli-wording
boundary: viola open refusals
item: not-live
promise: `not-live` is an exit-1 refusal of `viola open` with one fixed `unable:` / `hint:` pair naming the instance.
basis: fct-xxzbm355, fct-4kn64u2g
because: The chunk adds the refusal code `not-live` with its own fixed hint, which no refusal row states.
@row_add n=21 row=decision/run-hidden-origin-and-place area=surface-cli fact=F7 detector=none
name: run's hidden origin and place flags
rule: `viola run` carries two hidden flags kept off `--help`, `--origin` (`cli` or `agent`) and `--place N`; `--place` without `--origin` is a usage error.
basis: fct-xxzbm355
because: The run verb gains flags deliberately hidden from its help, a choice future help and flag work must respect.
@row_add n=22 row=contract/viola-list-json/window area=surface-cli fact=F8 detector=none
boundary: viola list --json item
item: window
promise: An optional `window` `{address, workspace}`, present only for an address that re-parses; MCP `list` items carry the same.
basis: fct-wrqnldxo, fct-xxzbm355
because: The board's `--json` items gain the window record, a new field at the CLI and MCP boundary.
@row_add n=23 row=contract/viola-list-json/origin area=surface-cli detector=none
boundary: viola list --json item
item: origin
promise: An optional `origin`, present only as an origin word (`cli` or `agent`); MCP `list` items carry the same.
basis: fct-wrqnldxo, fct-xxzbm355
because: The board's `--json` items gain the origin word, a new field at the CLI and MCP boundary.
