@task_prose part=session
Added the CLI verb viola open <name> [--parent <name>] [--role overseer|builder|worker] [--cwd <dir>] [--workspace N] [--model M] [--effort E] [--json] -- claude: it starts a wrapped session in a new foot terminal window beside its parent, from a closed flag allowlist, claude alone after the dashes, and no shell.
viola open answers exit 0 with the human line <name>  live  <placement> or one --json ok document {"v":1,"ok":{"name","parent","window":{"address","workspace"}|null,"placement"}}; exit 2 for any usage fault (a flag outside the allowlist, any word after -- but claude alone with a fixed usage line, a self parent); exit 1 with strict-modes-failed, already-live, stale-heartbeat, pinned-hash-mismatch, terminal-not-found, not-live or internal-error, each the {"v":1,"error":"<code>","detail":null} document under --json.
viola run gained two hidden flags, --origin <cli|agent> and --place <N> (--place without --origin a usage error); under --origin the wrapper finds its terminal's window after the trust step and before the first snapshot, moves it to the place, and records a window line and a snapshot window field (address, workspace, placement, origin).
viola list --json items and the MCP list result gain optional window {address, workspace} (only an address that re-parses) and origin (cli or agent); the help's setup: row grows from four verbs to five with open first; the two new exit-1 codes terminal-not-found and not-live each carry one hint.
Measured: the real Hyprland placement (matrix v2-07) stays unproven, gated behind R2, which fired once and read red on its rig before ever reaching the open; v2-08 (the allowlist and the closed argv) is met over the fakes and by construction.
@task_file n=1 path=src/cmd/open.rs
what: the viola open verb — its argument allowlist, its usage refusal, the terminal spawn with the closed argv, the live wait, its human line and its --json ok and error documents
@task_file n=2 path=src/run/window.rs
what: the wrapper's window step at run start — find its terminal's window, move it to the place, the placement outcome
@task_file n=3 path=src/bin/viola-fake-agent/foot.rs
what: the test stand-in for the foot terminal, recording the argv it was handed
@task_file n=4 path=tests/cli_open.rs
what: the integration cases of viola open over the fakes — the closed argv, each placement, the ok document, the refusals
@task_file n=5 path=scripts/r2-open.sh
what: edited — the operator's one-shot live check of viola open on a real Hyprland desktop, never run by a gate, with a rehearse form
@task_file n=6 path=Cargo.toml
what: the root crate's manifest — the Hyprland seam crate moved into its release dependencies
@task_file n=7 path=crates/viola-core/src/lib.rs
what: the shared core vocabulary — the event kinds (a new window kind), the closed placement words and origin words
@task_file n=8 path=crates/viola-state/Cargo.toml
what: the state crate's manifest — a new edge to the Hyprland seam crate for the window address check
@task_file n=9 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot — a new optional window record (address, workspace, placement, origin)
@task_file n=10 path=crates/viola-state/src/replay.rs
what: the snapshot replay from the event log — recovers the window record, the last window line after the last launch winning
@task_file n=11 path=crates/viola-state/src/board.rs
what: the board read's item behind viola list --json and the MCP list result — gains optional window and origin
@task_file n=12 path=crates/viola-state/src/liveness.rs
what: the process liveness reads — a new ancestor-chain read of a pid
@task_file n=13 path=src/cmd/mod.rs
what: the CLI verb table and dispatch — the open verb, its cli log, and the help's setup: row
@task_file n=14 path=src/cmd/run.rs
what: the viola run verb — the hidden --origin and --place flags, the window step in the start order, the window line after launch
@task_file n=15 path=src/run/mod.rs
what: the wrapper's run module — registers the window step
@task_file n=16 path=src/human/hints.rs
what: the hint lines for refusal causes — the two new hints for terminal-not-found and not-live
@task_file n=17 path=src/bin/viola-fake-agent.rs
what: the test fake agent's entry — dispatches to the foot stand-in
@task_file n=18 path=src/bin/viola-fake-agent/hyprctl.rs
what: the test fake hyprctl — a terminal mode listing the stand-in's window
@task_file n=19 path=crates/viola-e2e/tests/hypr_tap.rs
what: the Hyprland tap recording check — the move refusals tested over a planted stub
@task_file n=20 path=schemas/diag-line.v1.json
what: the diagnostic line schema — the subject terminal and the process-exit details terminal-not-found and not-live
@task_file n=21 path=tests/cli_output_plain.rs
what: the check of plain CLI output — the help's setup: row with open
@task_file n=22 path=tests/cross_exit_causes.rs
what: the cross check that every exit cause has its code and hint — two exit-1 causes added
@task_file n=23 path=tests/contract_error_bodies.rs
what: the census of CLI error documents and the documents that are not error bodies — open's documents added
@task_file n=24 path=tests/contract_diag_schema.rs
what: the check that diagnostic lines meet the schema — open's exit details
@task_file n=25 path=crates/viola-state/src/strict.rs
what: the strict-modes checks — a test snapshot gains the window field
@task_file n=26 path=crates/viola-mcp/tests/handlers.rs
what: the MCP server's handler tests — a snapshot gains the window field
@task_file n=27 path=src/cmd/hook.rs
what: the hook verb — its test snapshots gain the window field
@task_file n=28 path=src/run/dialog.rs
what: the wrapper's dialog handling — a test snapshot gains the window field
@task_file n=29 path=src/run/wheel.rs
what: the wrapper's wheel handling — a test snapshot gains the window field
@task_file n=30 path=src/run/send/slot.rs
what: the wrapper's send slot — a test snapshot gains the window field
@task_file n=31 path=tests/hook_fail_open.rs
what: the hook fail-open check — a snapshot gains the window field
@task_file n=32 path=tests/security_negatives_server_identity.rs
what: the server identity negative checks — a snapshot gains the window field
@task_file n=33 path=crates/viola-state/tests/state_replay.rs
what: the replay tests — the boxed replay result
@task_file n=34 path=src/cmd/revive.rs
what: the viola revive verb — hands no window ask to the start
@task_file n=35 path=src/cmd/list.rs
what: the viola list verb — its items carry the window and origin fields
@task_file n=36 path=src/human/board/test_support.rs
what: the board rendering test support — items gain the window and origin fields
@task_file n=37 path=src/human.rs
what: the human output module — exports the new hint pairs
@task_file n=38 path=Cargo.lock
what: the dependency lock — the state crate's new edge
@task_file n=39 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the fake agent's mode tests — the fake hyprctl's terminal mode case
@task_item n=1 part=touches
contract/cli-verbs/help — the setup: row now holds open, run, revive, verify, plugin install (four verbs to five, open first).
@task_item n=2 part=touches
contract/viola-help/verb-groups — the setup: group row gains open; its member list and count are stale.
@task_item n=3 part=touches
contract/viola-list-json/document — each item gains optional window {address, workspace} (only an address that re-parses) and optional origin (cli or agent only).
@task_item n=4 part=touches
contract/mcp-tools/list — the MCP list result's items gain the same optional window and origin through the shared board item; no new MCP tool.
@task_item n=5 part=touches
contract/cli-verbs/run — viola run takes two hidden flags --origin <cli|agent> and --place <N>; --place without --origin is a usage error; under --origin the start places its terminal's window and records it.
@task_item n=6 part=touches
contract/viola-run/start-output and the run start order — under --origin a window step runs after the trust step and before the first snapshot, and one window line follows the launch line; the human start output itself is unchanged.
@task_item n=7 part=touches
contract/cli-exit-codes/json-form — the {"v":1,"error":"<code>","detail":null} form now also carries open's codes, two of them new: terminal-not-found and not-live.
@task_item n=8 part=touches
decision/hint-for-every-cause — two new refusal causes, terminal-not-found and not-live, each with its one hint.
@task_item n=9 part=touches
decision/hints-name-next-verb and decision/hints-never-quote-or-locate — the two new hint lines fall under them.
@task_item n=10 part=touches
decision/flat-verbs and decision/targets-are-wrapped-names — a new flat verb open whose target and --parent are ViolaNames; a self parent is a usage refusal.
@task_item n=11 part=touches
contract/run-revive-usage/pair-after-dashes — open shares the after-dashes rule but closes it further: any word after -- but claude alone is a usage exit 2 with a fixed usage line, before any home read, log, pin or spawn.
@task_item n=12 part=touches
contract/viola-run/already-live, contract/run-revive-start-json/pinned-hash-mismatch, contract/viola-revive-json/stale-heartbeat and contract/viola-revive-json/strict-modes-failed — open now refuses with these same codes and documents.
@task_item n=13 part=touches
contract/cli-catch-site/internal-error, contract/cli-internal-error/human and contract/cli-internal-error/json — open is one more verb whose uncaught fault is internal-error.
@task_item n=14 part=touches
decision/json-on-every-data-verb and contract/cli-json-mode/one-document — open takes --json and prints exactly one ok document or one error document.
@task_item n=15 part=touches
decision/stdout-results-stderr-rest and decision/cli-field-separators — open's result line <name>  live  <placement> goes to stdout with the two-space field separator.
@task_item n=16 part=touches
decision/words-carry-every-state — the placement outcome is a closed word (placed, unplaced, window-unseen, unsupported, unreachable, oversize, malformed, refused), the origin a word (cli, agent).
@task_item n=17 part=touches
contract/viola-verb-help/usage — viola open --help is a new per-verb usage.
@task_item n=18 part=touches
decision/cli-holds-every-action and decision/mcp-hides-wheel-and-handoff-verbs — open is a CLI-only action with no MCP tool.
@task_item n=19 part=touches
contract/viola-list-board/agents-read-failure — measured at R2: viola list under a scratch --home still runs the claude agents --json read found on PATH under the operator's real config; no code changed, the rig emptied its PATH.
@task_fact n=1 slug=surface-cli
fact: viola open <name> starts a wrapped claude session in a new foot terminal window beside its parent; its flags are a closed allowlist (--parent, --role overseer|builder|worker, --cwd, --workspace, --model, --effort, --json) and the only words allowed after -- are claude alone; no shell runs.
@task_fact n=2 slug=surface-cli
fact: viola open's usage refusals are exit 2 before any home read, log, pin or spawn: a flag outside the allowlist, any program after -- other than claude alone (a fixed usage line that echoes none of the words), or a parent naming itself.
@task_fact n=3 slug=surface-cli
fact: viola open exit 0 prints <name>  live  <placement> on stdout; under --json the one ok document is {"v":1,"ok":{"name","parent","window":{"address","workspace"}|null,"placement"}}.
@task_fact n=4 slug=surface-cli
fact: viola open's exit-1 codes are strict-modes-failed, already-live, stale-heartbeat, pinned-hash-mismatch, terminal-not-found, not-live and internal-error, each the {"v":1,"error":"<code>","detail":null} document under --json.
@task_fact n=5 slug=surface-cli
fact: terminal-not-found (no foot terminal found to spawn) and not-live (the opened session never came live within its 30 s bound) are new exit-1 refusal causes, each with one fixed hint.
@task_fact n=6 slug=surface-cli
fact: viola open's parent defaults to the calling agent's own ViolaName, recorded with origin agent; from a plain shell there is no parent and the origin is cli.
@task_fact n=7 slug=surface-cli
fact: viola run's --origin <cli|agent> and --place <N> are hidden flags absent from help; --place without --origin is a usage error; viola revive never places a window.
@task_fact n=8 slug=surface-cli
fact: the placement words are a closed set of eight (placed, unplaced, window-unseen, unsupported, unreachable, oversize, malformed, refused) and the origin words a closed set of two (cli, agent).
@task_fact n=9 slug=surface-cli
fact: viola open has no MCP tool and no IPC method: opening a window is a CLI-only action.
@task_item n=1 part=untouched
The send path and its output rows — the readback mirror, the issue and outcome lines, the send refusals, viola send --json — no send behaviour changed.
@task_item n=2 part=untouched
viola wait, viola last and viola answer output rows and their MCP tools — unchanged.
@task_item n=3 part=untouched
viola verify output and refusal rows — unchanged; verify only keeps its place in the setup: row.
@task_item n=4 part=untouched
viola link / unlink output and the wheel verb output (pause, release) — unchanged.
@task_item n=5 part=untouched
viola plugin install output and refusal rows — unchanged, beyond sharing the setup: row.
@task_item n=6 part=untouched
viola revive's --list, refusal and start-output rows — revive's behaviour is unchanged; it just places no window.
@task_item n=7 part=untouched
The CLI token rendering rows (colour, spacing, radius, motion, roles) and the styling decisions (SGR module, no spinner, no banner, no pager, no progress glyphs) — open adds no styling.
@task_item n=8 part=untouched
The frame verb exit-21 hints and the exit-21 --json detail rows, and exit codes 10 to 14 and 20 — no exit code other than 1 and 2 moved.
@task_item n=9 part=untouched
The web surface absence and the CLI deep-links absence — still true.
