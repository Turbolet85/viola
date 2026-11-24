@row_set n=1 row=decision/lineage-label-only cell=rule seen=1 detector=none
value: Lineage is a label only: no wheel, link, send or answer path reads the declared parent or role; viola up reads the parent only to order its re-opens.
basis: fct-vdfhytdg
because: viola up now reads the declared parent to order its re-opens, so the label-only rule must name that one reader while staying true for every other path.
@row_set n=2 row=decision/revive-resumes-in-place cell=rule seen=2 detector=none
value: viola revive appends to the gone instance's events log with the earlier bytes unchanged, replays no launch spec but the recorded model and effort, and under the hidden --origin the recorded workspace, and its first record is wheel with holder driver and cause start, as every start's.
basis: fct-ykd4mawd
because: Under revive's new hidden --origin the revive replays the workspace a real snapshot recorded, so the replay clause must name it.
@row_set n=3 row=contract/cli-verbs/revive cell=promise seen=2 detector=none
value: `viola revive <name> [--id <ID>] [--fork] [--model <MODEL>] [--effort <EFFORT>] [-- <child args>]` and `viola revive <name> --list` send no channel frame; `--list` conflicts with `--id`, `--fork`, `--model`, `--effort`, the hidden `--origin` and the trailing arguments.
basis: fct-ykd4mawd
because: revive gained a hidden --origin that conflicts with --list, so the conflict list is incomplete without it.
@row_set n=4 row=contract/cli-exit-codes/1 cell=promise seen=3 detector=none
value: Internal error, and the start, revive-preflight, open, focus, up and verify refusals, a refused home-tree check or socket directory of the start among them, each an unable and hint pair on stderr with no path, or under --json one refusal document; also an up whose lines are not all successes.
basis: fct-ykd4mawd
because: viola up exits 1 on its own refusals and when any line is not live, already-live, stale-heartbeat or would-open, so the exit-1 list must name up.
@row_set n=5 row=contract/instance-snapshot/cwd cell=promise seen=2 detector=none
value: `cwd` is the directory the start spawned its child in, written into the first snapshot when valid UTF-8 and omitted otherwise; its readers are `viola revive`, which uses it as the revived child's spawn directory and its trust record's folder, and `viola up`, opening each terminal there.
basis: fct-ykd4mawd
because: viola up now reads the recorded cwd for each terminal's working directory, so revive is no longer its one reader.
@row_set n=6 row=contract/diagnostics-role-files/cli-name cell=promise seen=2 detector=none
value: The CLI verbs' lines, <name> being the target instance for the frame verbs and focus and the caller's VIOLA_NAME for list, verify, revive --list, up and plugin install.
basis: fct-ykd4mawd
because: viola up writes the caller's cli log under a parsed VIOLA_NAME, so the role file's list of verbs must name it.
@row_set n=7 row=contract/process-exit-detail/not-live cell=promise seen=1 detector=D-arch-resources
value: A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola open and viola up.
basis: fct-uiucoyys
because: viola up writes not-live as its exit-1 process-exit detail when any instance reads it, so the word is now occupied by up too.
@row_set n=8 row=contract/process-exit-detail/terminal-not-found cell=promise seen=1 detector=D-arch-resources
value: A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola open and viola up.
basis: fct-uiucoyys
because: viola up refuses with terminal-not-found as its own process-exit word, so the word is now occupied by up too.
@row_add n=1 row=name/command/up area=occupied-resources fact=F1 detector=D-arch-resources
class: command
name: up
basis: fct-ykd4mawd
because: The chunk lands the CLI subcommand viola up, an occupied command name no row registers.
@row_add n=2 row=contract/cli-verbs/up area=conventions fact=F2 detector=D-arch-resources
boundary: CLI verbs
item: up
promise: `viola up [<NAME>...] [--dry-run] [--json]` re-opens, after a reboot, each instance gone before the boot through viola revive in a new terminal; names given narrow the fleet and a name not scanned reads not-taken; CLI-only, no MCP tool and no channel method.
basis: fct-ykd4mawd
because: A new CLI verb lands with its arguments and its CLI-only surface, and the verb list holds no row for it.
@row_add n=3 row=decision/up-selection-rule area=established-decisions fact=F3 detector=none
name: up re-opens only instances gone before the boot
rule: viola up re-opens an instance only when it is neither live nor stale, its heartbeat is older than the host boot time, and its events log has no session-end after its last session-start, so an instance ended on purpose or ended this boot stays down.
state: provisional
basis: fct-orhmfpak
because: The chunk fixes which instances up re-opens, a provisional decision future fleet work must respect.
@row_add n=4 row=decision/up-parents-first area=established-decisions fact=F4 detector=none
name: up re-opens parents before children
rule: viola up re-opens parents before children; a child whose parent is missing is a root, and a cycle is visited once.
basis: fct-o5x32fkq
because: The chunk orders up's re-opens by declared parent, a decision no row states.
@row_add n=5 row=contract/viola-up-output/human-line area=conventions fact=F5 detector=none
boundary: viola up output
item: human line
promise: One plain ASCII line per instance, `<name>  <word>` or `<name>  live  <placement>`, the word from up's closed outcome set; lines not a spawn come first in name order, then spawned or would-open lines in spawn order; one stderr hint: line when any reads not-live, none under --json.
basis: fct-tpkehh2e
because: viola up's per-instance line is a new output contract at the CLI boundary.
@row_add n=6 row=contract/viola-up-outcome/live area=conventions fact=F5 detector=none
boundary: viola up outcome words
item: live
promise: A success word: the instance came up live in its new terminal, its line `<name>  live  <placement>`.
basis: fct-ykd4mawd
because: live is one item of up's new closed outcome set.
@row_add n=7 row=contract/viola-up-outcome/already-live area=conventions fact=F5 detector=none
boundary: viola up outcome words
item: already-live
promise: A success word: the instance was live and is not re-opened.
basis: fct-ykd4mawd
because: already-live is one item of up's new closed outcome set.
@row_add n=8 row=contract/viola-up-outcome/stale-heartbeat area=conventions fact=F5 detector=none
boundary: viola up outcome words
item: stale-heartbeat
promise: A success word: the instance was stale and is not re-opened.
basis: fct-ykd4mawd
because: stale-heartbeat is one item of up's new closed outcome set.
@row_add n=9 row=contract/viola-up-outcome/would-open area=conventions fact=F5 detector=none
boundary: viola up outcome words
item: would-open
promise: A success word under --dry-run: an instance the plan would re-open; nothing is spawned.
basis: fct-ykd4mawd
because: would-open is one item of up's new closed outcome set.
@row_add n=10 row=contract/viola-up-outcome/no-session area=conventions fact=F5 detector=none
boundary: viola up outcome words
item: no-session
promise: A failure word: a line reading it makes up exit 1, with no process-exit detail of its own.
basis: fct-uiucoyys
because: no-session is one item of up's new closed outcome set.
@row_add n=11 row=contract/viola-up-outcome/cwd-missing area=conventions fact=F5 detector=none
boundary: viola up outcome words
item: cwd-missing
promise: A failure word: a line reading it makes up exit 1, with no process-exit detail of its own.
basis: fct-uiucoyys
because: cwd-missing is one item of up's new closed outcome set.
@row_add n=12 row=contract/viola-up-outcome/session-live area=conventions fact=F5 detector=none
boundary: viola up outcome words
item: session-live
promise: A failure word: a line reading it makes up exit 1, with no process-exit detail of its own.
basis: fct-uiucoyys
because: session-live is one item of up's new closed outcome set.
@row_add n=13 row=contract/viola-up-outcome/not-live area=conventions fact=F5 detector=none
boundary: viola up outcome words
item: not-live
promise: A failure word: the spawned instance did not come up live within open's live bound; up exits 1 with process-exit detail not-live and one stderr hint: line.
basis: fct-uiucoyys
because: not-live is one item of up's new closed outcome set.
@row_add n=14 row=contract/viola-up-outcome/not-taken area=conventions fact=F5 detector=none
boundary: viola up outcome words
item: not-taken
promise: A failure word: a name given on the command line that the scan did not find; a line reading it makes up exit 1.
basis: fct-urp643y2
because: not-taken is one item of up's new closed outcome set.
@row_add n=15 row=contract/viola-up-exit-codes/0 area=conventions fact=F6 detector=none
boundary: viola up exit codes
item: 0
promise: Every line reads live, already-live, stale-heartbeat or would-open.
basis: fct-ykd4mawd
because: viola up is a new verb with its own exit codes.
@row_add n=16 row=contract/viola-up-exit-codes/1 area=conventions fact=F6 detector=none
boundary: viola up exit codes
item: 1
promise: Any other line; or a refusal with a document under --json: boot-time-unread, strict-modes-failed, pinned-hash-mismatch or terminal-not-found.
basis: fct-ykd4mawd
because: viola up is a new verb with its own exit codes.
@row_add n=17 row=contract/viola-up-exit-codes/21 area=conventions fact=F6 detector=none
boundary: viola up exit codes
item: 21
promise: The board's strict refusal (instance-unreachable, strict-modes-failed), with no line printed.
basis: fct-ykd4mawd
because: viola up is a new verb with its own exit codes.
@row_add n=18 row=contract/cli-json-output/up area=standard-contracts fact=F7 detector=none
boundary: CLI `--json` output
item: `viola up --json`
promise: Prints one document for the whole fleet, `{"v":1,"ok":{"instances":[{"name","outcome","placement"}]}}`, with no address, pid or workspace on it.
basis: fct-m4vsf7nm
because: viola up's machine document is a new contract at the CLI boundary.
@row_add n=19 row=decision/up-writes-no-state area=standard-contracts fact=F8 detector=none
name: up writes no state of its own
rule: viola up writes no snapshot, no event line and no stamp; under --dry-run it spawns nothing and prints the plan as would-open lines.
basis: fct-lwvzzjno
because: The chunk fixes that up leaves every state file to the revives it spawns, a contract future verbs must keep.
@row_add n=20 row=decision/up-terminal-direct-spawn area=standard-contracts fact=F9 detector=none
name: up spawns each terminal under a closed argv
rule: viola up spawns each terminal directly under a closed argv running the pinned viola revive <name> --origin <cli|agent>, and waits for it under open's existing live bound; it mints no new time bound.
basis: fct-ykd4mawd
because: The chunk widens the terminal spawn to up with a fixed argv and reuses open's bound, a decision future work must respect.
@row_add n=21 row=contract/viola-cli-arguments/revive-origin area=occupied-resources fact=F10 detector=D-arch-resources
boundary: viola CLI arguments
item: revive --origin
promise: viola revive --origin <cli|agent> is a hidden flag, conflicting with --list and with no --place: under it the revive places its window on the workspace a real snapshot recorded, else none; without it there is no window step.
basis: fct-ykd4mawd
because: revive gains a hidden flag, an occupied argument no row registers.
@row_add n=22 row=contract/process-exit-detail/boot-time-unread area=occupied-resources fact=F11 detector=D-arch-resources
boundary: process-exit detail enum
item: boot-time-unread
promise: A word of the process-exit detail enum in schemas/diag-line.v1.json, occupied by viola up: its exit 1 when the host boot time cannot be read.
basis: fct-xgdyo5mg
because: The schema's process-exit detail enum gains boot-time-unread, which architecture's enum set does not hold.
@row_add n=23 row=decision/terminal-tree-one-teardown area=occupied-resources fact=F13 detector=none
name: Stand-in terminal trees ended by one teardown
rule: Tests end a stand-in terminal's process tree through one shared teardown: only recorded pids whose start time still matches are signalled, a leaf under a running parent is killed first, and every other member is waited on under one deadline and killed only at it.
why: Killing a fake agent already in its own exit truncated its coverage profile.
basis: fct-z7bnkfyd
because: The chunk replaces each terminal guard's own kill loop with one shared teardown, a test convention future guards must use.
@row_add n=24 row=name/other/foot-spawns-ndjson area=occupied-resources fact=F14 detector=none
class: other
name: foot-spawns.ndjson
basis: fct-ykd4mawd
because: The fake foot now records each spawn in a file of this name beside itself, an occupied name.
@row_none n=1 fact=F12 type=5
what: The list of bounded sources (the /proc/stat boot time, the events reader's end-after-start read) is the guard's BOUNDARIES inventory; decision/bounded-readers already states the rule and stays true.
