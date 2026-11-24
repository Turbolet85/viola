@task_prose part=chunk
Adds a new CLI verb `viola focus <name>` to the root bin: it resolves the viola home, validates the name, reads the instance's state under the strict-modes checks, finds the session's window and brings it forward on its workspace on Hyprland.
The window is found two ways: from the window record the wrapper keeps in the snapshot for a session started with `--origin`, or, for a hand-started `viola run`, by matching the wrapper's process ancestry against the compositor's client list.
Every outcome with no window found ends in a named refusal, and outside Hyprland the verb refuses as unsupported. Window dispatches go only through the Hyprland seam's closed request set.
It re-measures at HEAD that the seam holds a focus-window and a workspace-focus dispatch recorded for Hyprland 0.56.2 and that the fake compositor replays them. It adds tests on the fake compositor and the fake agent, plus the help, exit-cause and verb table entries the verb needs.
@task_file n=1 path=src/cmd/focus.rs
what: the new `focus` CLI verb in the root bin: it resolves the home, validates the name, does the strict-modes read of the instance state, resolves the window identity and sends the Hyprland dispatches
@task_file n=2 path=src/cmd/mod.rs mark=edits
what: the root bin's CLI verb set and its dispatch — edits: the verb table entry and the help entry for `focus`
@task_file n=3 path=tests/cli_focus.rs
what: end-to-end tests of `viola focus` against the fake compositor's recorded Hyprland 0.56.2 replies and the fake agent, covering a session viola opened, a hand-started session, each no-window refusal and the unsupported refusal outside Hyprland
@task_item n=1 part=needs
A new CLI verb `focus` joins the CLI verbs boundary and the command name register. It is added as a subcommand in the existing clap flag and subcommand style, with its argument shape among the viola CLI arguments.
@task_item n=2 part=needs
The `<name>` argument is validated through ViolaName as the instance key, so control characters are refused and never stripped. Identity comes from the instance, never from a location.
@task_item n=3 part=needs
The viola home is resolved in the viola home resolution order. The strict-modes read rule applies before any read of instance state or logs. A failed check writes no line and refuses with exit 21 strict-modes-failed and its exit-21 human hint.
@task_item n=4 part=needs
The verb reads the instance snapshot: the snapshot envelope and its version, the `window` field (address, workspace, placement, origin) the wrapper records only under `--origin`, and the pid fields. Reads are versioned-tolerant, a missing value and a null read the same, and a replayed or unreadable snapshot is handled as the snapshot rules say. The verb writes no file.
@task_item n=5 part=needs
First identity source: a recorded window address is re-parsed through the Hyprland seam's Address validator before use, and recorded text is treated as untrusted upstream text.
@task_item n=6 part=needs
Second identity source, for a hand-started session with no recorded window: the wrapper's process ancestry is matched against the compositor's client list. This goes through viola-state's ancestry walk if it can be reused (at most 8 parent pids, held in memory), starting from a pid the snapshot holds. Lineage stays a label only.
@task_item n=7 part=needs
Every compositor call goes only through viola-hyprland, the one compositor seam, and its closed request set: a workspace-focus dispatch and a focus-window dispatch. hyprctl is spawned directly with a closed argv and nothing shells out. Window features target Hyprland.
@task_item n=8 part=needs
The Hyprland seam's errors (unsupported, unreachable, refused, malformed, oversize) each map to a named refusal of the verb. Outside Hyprland the verb refuses as unsupported.
@task_item n=9 part=needs
Every no-window outcome gets a named refusal: the recorded window is gone, no client matches, or the name holds no instance. Each one adds an entry to the closed sets beside the existing tables: the CLI exit codes, the process-exit detail enum (next to terminal-not-found), the --json error document and the human refusal line. The usual rules hold: refusal details come from a closed set, only the first refusal is returned, wire enum values are kebab-case, and retired exit codes are never reused.
@task_item n=10 part=needs
Output follows the CLI conventions: human text by default and a `--json` document for agents. Refusals go on human stderr through the one human writer, with no print macros. Error surfaces are sanitised, and the role's codes-only process log (the cli-<name> diagnostics role file) holds codes only.
@task_item n=11 part=needs
Errors follow the crate rule: thiserror in the crates, with messages that interpolate nothing, and anyhow only at the root bin edge.
@task_item n=12 part=needs
Focus stays CLI-only, a human control. No driver-session surface may move the operator's focus unless a ruling widens the boundary.
@task_item n=13 part=needs
The verb grows no new time bound and no new full time-bound list cell. That question waits on the operator's open fork.
@task_item n=14 part=needs
Tests prove the verb against the fake compositor (viola-hypr-tap's recorded replies for Hyprland 0.56.2) and the fake agent, with no live claude, no live compositor, no real window moved and no read of the operator's real claude home. Any clause a live focus reading alone could prove is planned as gated on the operator's grant at the host.
@task_item n=15 part=needs
The verb adds `focus` entries to the help table, the exit-cause table and the verb table the masters pin, and the cross-verb exit-cause checks then cover it.
@task_standing n=1
convention: each new end-to-end test runs in its own viola home, which is removed by its owner record, and the test's evidence survives if the test is killed
home: occupied-resources
@task_standing n=2
convention: each new test that only runs on Linux or under Hyprland is OS-gated and names its reason
home: cross-cutting-patterns
@task_standing n=3
convention: each new test's waits share the one root test wait bound
home: occupied-resources
@task_standing n=4
convention: each new end-to-end run counts toward the zero-panic-lines gate, and any insta snapshot runs in check mode only
home: project-directory-structure
@task_standing n=5
convention: each new module is reachable from its crate root (no orphan modules), and new code stays within the print and dbg bans with clippy warnings denied
home: build-system
@task_standing n=6
convention: new tests run in CI on ubuntu against the fake agent and count toward the coverage floors
home: ci-cd-approach
@task_item n=1 part=not_needed
The wheel, send, paste, confirmation and dialog-hold rows: focus types nothing into a session and does not move the wheel.
@task_item n=2 part=not_needed
Hook rows (hook tiers, hook-to-kind map, hook contract, hook exit and body, hook capture arm): focus adds or changes no hook.
@task_item n=3 part=not_needed
Start-path rows (child argv, child environment strip, pinned copy placement and rehash, run start order, revive, default launch pair): focus starts or revives no session.
@task_item n=4 part=not_needed
Verify, ledger stamps, trust writer code words, probe capture and unclean fixture refusal rows: focus runs no CLI probe and writes no stamp.
@task_item n=5 part=not_needed
Plugin install, plugin marketplace and plugin precedence rows: no change to the plugin.
@task_item n=6 part=not_needed
The `viola open` exit codes and its gated real-window clause (v2-07): those belong to the gated open chunk, not to focus.
@task_item n=7 part=not_needed
Design-philosophy absences such as the usage-budget gate and model API calls, and the deployment and release rows (dist, crates.io, containers): focus touches none of them.
@task_item n=8 part=not_needed
Channel link and unlink, link set and links-as-events rows: focus records no driver-to-driven link.
