@task_prose part=session
The session added the CLI verb viola up, which re-opens the fleet after a reboot. Its candidates are instances whose heartbeat predates the boot time, that are not live and whose session did not end after its last start. It opens them parents first, skips live ones, and writes one line per instance.
For each candidate, up spawns a foot terminal directly with a closed argv that starts the pinned viola revive <name> --origin <cli|agent> in the recorded cwd. It then waits for the instance to come up live under the bound open already uses.
viola revive gained a hidden --origin flag with two closed values. It conflicts with --list and replays the workspace that a real snapshot's window recorded. Without the flag there is no window step.
viola-state gained a boot-time reader that reads /proc/stat once under MAX_FRAME, a heartbeat-before-boot test, a bounded end-after-last-start events reader and a read accessor on the board scan. The diag schema's process-exit detail gained boot-time-unread.
Test-only work: one shared teardown for a stand-in terminal's process tree, a settle wait in the child-fd guard, the fake foot recording each spawn, and a new live R4 rig script that refuses R4b and a root inside a repository.
No dependency, config key, env var, MCP tool or channel method was added, and nothing was retired.
@task_file n=1 path=src/cmd/up.rs
what: new file. The viola up verb: the strict board scan, boot-time liveness, the selection, parents-first order, the home-tree and pinned-copy checks before the first spawn, the closed terminal argv and the closed outcome words and exits.
@task_file n=2 path=tests/cli_up.rs
what: new file. Integration cases for viola up over the fakes, including the strict-modes refusal with no line, the pinned-hash and terminal-not-found refusals, and that no snapshot or stamps file is written.
@task_file n=3 path=scripts/r4-up.sh
what: new file. Test-only live rig for R4: rehearsal over fakes, a live root it refuses inside a repository, R4b refusing first, and no credential path.
@task_file n=4 path=tests/support/terminal.rs
what: new file. Test-side teardown of a stand-in terminal's process tree that signals only a recorded pid whose start time still matches.
@task_file n=5 path=src/cmd/mod.rs
what: edited. The CLI verb dispatch and help literal; it adds the up command and lists it in the setup row.
@task_file n=6 path=src/cmd/revive.rs
what: edited. viola revive; it adds the hidden closed --origin flag and the window ask built from a real snapshot's recorded workspace.
@task_file n=7 path=src/cmd/open.rs
what: edited. viola open's direct terminal spawn and live wait; the spawn, the live wait and the recorded-window read are now shared with up.
@task_file n=8 path=src/human/verbs.rs
what: edited. Human-mode output writers; it adds the up line and the hint writer, in plain ASCII with one write per line.
@task_file n=9 path=src/human/hints.rs
what: edited. Fixed human error and hint literals; it adds the boot-time-unread pair and the up not-live hint.
@task_file n=10 path=src/human.rs
what: edited. Human output module re-exports for the new up writers.
@task_file n=11 path=src/bin/viola-fake-agent/foot.rs
what: edited. The fake foot terminal test seam under the fake-agent feature; it records each spawn's pid and argv.
@task_file n=12 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: edited. Unit tests of the fake agent's modes; it adds a case for the fake foot's spawn record.
@task_file n=13 path=crates/viola-state/src/liveness.rs
what: edited. Instance liveness classification; it adds the bounded /proc/stat boot-time reader and the heartbeat-before-boot test.
@task_file n=14 path=crates/viola-state/src/replay.rs
what: edited. The bounded events.ndjson replay reader; it adds an end-after-last-start read.
@task_file n=15 path=crates/viola-state/src/board.rs
what: edited. The board scan with its strict home read rule; it adds a read accessor over the scanned instances.
@task_file n=16 path=schemas/diag-line.v1.json
what: edited. The diagnostics line schema; the process-exit detail enum gains boot-time-unread.
@task_file n=17 path=tests/cli_output_plain.rs
what: edited. Guard on plain human output and the served-verb help list; up is now served.
@task_file n=18 path=tests/contract_error_bodies.rs
what: edited. The census of every product error writer (CLI_DOCUMENTS); it gains five up rows.
@task_file n=19 path=tests/contract_bounded_readers.rs
what: edited. The bounded-readers census guard; BOUNDARIES gains /proc/stat and the end-after-last-start events read, and EXCEPTIONS gains up's own exe read.
@task_file n=20 path=tests/cross_exit_causes.rs
what: edited. The exit-cause table guard; it gains the boot-time-unread exit 1 row.
@task_file n=21 path=tests/cli_revive.rs
what: edited. Integration cases for viola revive; it adds the --origin window move and the no-flag case with no window step.
@task_file n=22 path=tests/cli_machine_contract.rs
what: edited. The machine-output (--json) contract guard; it adds up's document of closed words.
@task_file n=23 path=tests/contract_diag_schema.rs
what: edited. The diag schema contract guard; it accepts the new detail word.
@task_file n=24 path=tests/tui_channel_fds.rs
what: edited. The e2e guard that the claude child inherits no viola handle or home file; it adds a settle wait inside the same deadline and leaves the assertions unchanged.
@task_file n=25 path=tests/support/mod.rs
what: edited. Test support module list; it declares the shared terminal teardown.
@task_file n=26 path=tests/cli_open.rs
what: edited. Integration cases for viola open; its terminal guard hands its roots to the shared teardown in place of its own kill loop.
@task_item n=1 part=touches
decision/open-terminal-direct-spawn: up now also spawns foot directly, through open's spawn. Its terminal runs the pinned viola revive <name> --origin <word>, which widens the boundary from open alone (U2, operator's word arm 1).
@task_item n=2 part=touches
decision/open-closed-argument-allowlist: up's terminal argv is a second closed argv: --app-id=viola.<name>, --title=<name>, --working-directory=<recorded cwd>, then the pinned binary, --home <canonical home>, revive, the name and --origin, one element each.
@task_item n=3 part=touches
decision/run-window-flags-checked and decision/launch-flags-checked: revive gains a hidden --origin limited to cli or agent by clap's possible values. It conflicts with --list and has no --place.
@task_item n=4 part=touches
decision/window-step-moves-own-terminal-window, decision/hyprland-recorder-moves-own-window and decision/window-origin-authenticates-nothing: a revive under --origin moves its own terminal's window to the workspace the snapshot recorded. Without --origin it runs no window step.
@task_item n=5 part=touches
decision/replayed-snapshot-inert, decision/replayed-snapshot-yields-nothing and decision/replayed-snapshot-fallbacks: up reads liveness from a replayed snapshot by the heartbeat alone. revive --origin takes a workspace only from a real snapshot, never from a replayed one.
@task_item n=6 part=touches
decision/revive-cwd-spawn-only, decision/revive-recorded-cwd-spawn-dir, decision/cwd-only-in-snapshot and decision/spawn-cwd-snapshot-only: up reads the recorded cwd and passes it to the terminal as its working directory. The cwd now crosses on the foot argv.
@task_item n=7 part=touches
decision/list-home-read-rule, decision/board-home-read-rule, decision/whole-board-refusal and decision/strict-check-before-narrowing: up runs the strict board scan first and refuses the whole board. contract/cli-exit-codes/21: up exits 21 on that refusal and writes no line.
@task_item n=8 part=touches
decision/start-checks-home-tree-before-log-opens, decision/home-tree-strict-modes-before-log and decision/strict-modes-before-read: up scans before it opens the caller's cli log under VIOLA_NAME. It also checks the home tree again before its first spawn, with strict-modes-failed as exit 1 with a document.
@task_item n=9 part=touches
the contract/strict-modes-refusal/* family (list-human, list-json, run-revive, …): up is a new verb with a strict-modes refusal, and no row names it yet.
@task_item n=10 part=touches
decision/pinned-binary-rehashed, decision/rehash-pinned-binary and decision/pinned-copy-content-hash: up re-hashes the pinned copy before any spawn. A mismatch refuses with pinned-hash-mismatch, exit 1.
@task_item n=11 part=touches
decision/bounded-readers-census, decision/bounded-external-readers, decision/max-frame-cap and decision/reads-capped-at-max-frame: there are two new bounded readers, the /proc/stat boot time and the events end-after-last-start, plus one named exception, up's own exe read.
@task_item n=12 part=touches
decision/events-reader-bounded-lines: the new end-after-last-start read goes through the bounded events reader in one pass.
@task_item n=13 part=touches
decision/cli-names-through-name-check and decision/names-through-validated-type: names given to up are parsed by the viola name check. An unscanned name reads not-taken.
@task_item n=14 part=touches
decision/lineage-parent-authenticates-nothing and decision/board-lineage-and-links-name-checked: up uses the recorded parent only to order the spawns, parents first, with a cycle opened once. It joins no path from it.
@task_item n=15 part=touches
decision/claude-agents-read-fail-open and decision/claude-agents-resolved-from-path: up runs one claude agents read for the whole set to find session-live ids.
@task_item n=16 part=touches
decision/child-path-trust and decision/claude-resolved-from-path: up resolves foot on PATH and refuses with terminal-not-found, exit 1, before any spawn.
@task_item n=17 part=touches
decision/refusals-hold-no-paths-pids-ids, decision/no-paths-in-error-output and decision/error-messages-interpolate-nothing: up's lines, hint and --json document hold codes and names only, with no address, pid or workspace.
@task_item n=18 part=touches
decision/every-error-writer-named and decision/fixed-error-display-messages: the error-writer census gains five up documents. boot-time-unread gets a fixed human pair.
@task_item n=19 part=touches
decision/human-output-escapes-controls and decision/escape-terminal-controls: up's lines are plain ASCII with no SGR and one write per line.
@task_item n=20 part=touches
decision/state-readers-never-write, decision/snapshot-written-only-by-wrapper and decision/stamps-written-only-by-verify: up writes no snapshot, event line or stamp, and does not call the board read.
@task_item n=21 part=touches
decision/session-id-argv-closed-shape and decision/session-id-argv-no-shell: up picks the newest session-shaped id from the session chain for the revive it starts.
@task_item n=22 part=touches
decision/test-removal-own-dirs-only, decision/e2e-home-removal-own-dirs-only and decision/sysinfo-process-identity: the shared test teardown signals only recorded pids whose start time still matches. It reads parents from one sysinfo table.
@task_item n=23 part=touches
decision/test-seams-fake-agent-only and decision/test-seams-compiled-out: the fake foot under the fake-agent feature now records each spawn's pid and argv.
@task_item n=24 part=touches
decision/live-rig-credentials-copy, decision/live-rig-isolated-child and decision/tests-use-own-home: the new R4 live rig refuses a root inside a repository and refuses R4b until granted. It holds no credential path and keeps the stand-in agent in its own target dir.
@task_item n=25 part=touches
contract/instance-unreachable-cause/strict-modes-failed and contract/cli-json-error/strict-modes-failed: up emits the same strict-modes-failed document and the instance-unreachable exit 21 on the board refusal.
@task_fact n=1 slug=input-validation
fact: viola up spawns each terminal directly with a closed argv that starts the pinned viola revive <name> --origin <cli|agent> in the recorded cwd, one argv element each and no shell.
@task_fact n=2 slug=input-validation
fact: revive's hidden --origin takes the two closed words cli and agent only and conflicts with --list. It places a window only at the workspace a real snapshot recorded, and has none otherwise.
@task_fact n=3 slug=input-validation
fact: The boot time is read once from /proc/stat under MAX_FRAME. If it is unreadable, up refuses with boot-time-unread, exit 1, before any spawn.
@task_fact n=4 slug=authentication-authorization
fact: viola up runs the strict board scan before its log opens, and checks the home tree and re-hashes the pinned copy before its first spawn. A board refusal exits 21 with no line.
@task_fact n=5 slug=data-protection
fact: viola up writes no snapshot, event line or stamp. Its lines, hint and --json document hold closed words and names only, never an address, pid or workspace.
@task_fact n=6 slug=security-anti-patterns-never-do-these
fact: Test-side teardown signals only a pid it recorded whose start time still reads the recorded one, and kills a leaf first only while its recorded parent runs.
@task_fact n=7 slug=secret-management
fact: The R4 live rig refuses a root inside a repository and refuses its R4b phase until granted, and holds no credential path.
@task_item n=1 part=untouched
MCP tools, channel methods and channel frames: up is CLI-only and adds no tool, method or frame.
@task_item n=2 part=untouched
Paste, send and answer text validation, and dialog free text: no typed-text path changed.
@task_item n=3 part=untouched
Hook payload parsing, hook refusals and the hook decision details.
@task_item n=4 part=untouched
The socket directory, socket modes, peer euid checks and the server verify before any frame: up sends no frame.
@task_item n=5 part=untouched
Dependency security, toolchains, CI workflow permissions, release build and advisory rows: no dependency or workflow changed.
@task_item n=6 part=untouched
The claude child's environment strip, credential handling in the product and the CI artifact canary scan.
@task_item n=7 part=untouched
config.json keys and the rule that config cannot disable controls: no config key was added.
@task_item n=8 part=untouched
viola verify's probes, stamps, fixture scrubbing and refusal codes.
