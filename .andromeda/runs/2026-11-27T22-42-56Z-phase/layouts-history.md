@history_item n=1 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — changed contract/cli-verbs/run, contract/viola-run/start-output and contract/cli-verbs/revive: the start writes the claude CLI's trust record for an untrusted session folder, keyed by revive's recorded directory, and the step adds no refusal, exit code, hint, `--json` code or terminal byte. This is the trust step that move 3 takes out of `src/cmd/run.rs`. A pure move must keep its silence and keep revive's key on the recorded directory.

@history_item n=2 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — changed contract/viola-run/start-output: before the child, only a key-on start's automatic verify line and verify's own output reach the terminal, and both are discarded under `revive --json`. This is the start's output order inside `run::start`. Moving the window and trust steps must not reorder them around the verify step, or the bytes before the child change.

@history_item n=3 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (ratification of 2026-11-20-a-new-cli-version-stamped-automatically, contract/viola-run/start-output) — the founder ratified the start-output promise that the run.rs split must leave byte-identical. The row is no longer provisional, so it is a fixed constraint and not a point to reopen.

@history_item n=4 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — added contract/cli-verbs/open and contract/viola-revive/no-window (the window step reaches only a start that asks for it, and revive hands it none), and the `setup:` help row gained open. This is where the window step that move 3 lifts out of `run::start` (`open_window`) comes from. The gating, where only an asking start places a window, must survive the move.

@history_item n=5 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Surface: cli: 3 row(s), 1 of 2) — changed contract/viola-revive/no-window: under the hidden `--origin <cli|agent>` the revived window moves to the workspace a snapshot recorded, or reads unplaced. It also changed contract/viola-revive/usage (`--list` conflicts with `--origin`). This is the window path that `up` drives through revive. Both the up.rs split (move 1) and the run.rs window-step move (move 3) must keep it.

@history_item n=6 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Surface: cli: 3 row(s), 2 of 2) — added contract/cli-verbs/up, and changed contract/cli-exit-codes/21 (the board refusal of `viola up` prints the fixed state pair) and decision/board-vs-strip (`up` re-opens the whole fleet or the named instances). These rows state the behaviour of `upped`'s plan: refusal, order (parents before children), and skipping live instances. Move 1's plan/act split must keep all of it, exit 21 and its pair included.

@history_item n=7 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** (ratifications of 2026-11-24-viola-up 1 of 2 and 2 of 2, 2026-11-23-model-and-effort-at-launch, 2026-11-24-switching-a-session-s-model-and-effort 1 of 2 and 2 of 2, 2026-11-25-placing-without-moving-the-view) — the operator's word ratified the up, revive-window, run/revive usage, switch and verify rows that this chunk's moves rest on. They now stand as firm constraints for a pure move.

@history_item n=8 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (Surface: cli: 4 row(s), 1 of 2) — changed contract/cli-verbs/run so viola writes nothing at a switch's respawn of the child, and added switch to contract/frame-verbs/strict-modes-refused. The respawn is the restart code that move 2 takes out of `crates/viola-pty/src/pump.rs`. The moved module must keep the respawn silent on the terminal.

@history_item n=9 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (Surface: cli: 3 row(s), 2 of 2) — added contract/cli-verbs/switch, contract/viola-switch-output/switched and contract/viola-switch-output/json-ok: the exit set 0/10/12/13/14/20/21, the `<name>  switched  <model> <effort>` line and the `ok` document with the new pair. These are the outputs of a switch whose respawn path move 2 relocates. They must read byte-identical after the move.

@history_item n=10 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — changed contract/viola-revive/usage (`--model`/`--effort`) and added contract/run-revive-usage/pair-after-dashes, a refusal with its own exit code. Both live on the shared run/revive start in `src/cmd/run.rs`. Any re-pointed test or census that names them must follow the move.

@history_item n=11 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Surface: cli: 3 row(s); Surface: cli: 2 row(s)) — added contract/cli-verbs/adopt and retired contract/cli-catch-site/internal-error, because adopt's start is a silent role at the catch site just like the revive start. Adopt runs through the shared `run.rs` start that move 3 splits, so its silent-start behaviour and catch-site role must not shift.

@history_item n=12 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `viola revive --json` post-preflight gap closed) — a refusal of the start that revive shares with run, after a passed preflight, prints one exit-1 document. A `pump_child` error prints none, because the child has already held the terminal, and a document from `main` was rejected for that reason. This is a trap for both the run.rs move and the pump.rs move: where an error surfaces relative to the child holding the terminal decides whether a document prints.

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree refusals of verify and the run and revive starts) — the run and revive starts check the home tree before the wrapper log opens: no log, no line and no child on a refusal. This is a precedent on ordering inside `run::start`. Moving the trust and window steps out must not move them ahead of these checks.

@history_item n=14 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory refusal of the `run` and `revive` starts) — `refuse_socket_dir` runs at run's start before any snapshot, child or bind, and revive shares it. This is a second ordering constraint in the start that move 3 reshapes. The steps must keep their place relative to the bind and the child.

@history_item n=15 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the strict-modes refusal of the driven instance) — recorded the trap that opening a role file narrows an existing home to 0700, so a check placed after `client::start` passes a 0770 home. `link_verbs_refuse_a_widened_instance_before_any_log` holds that order by reading the mode after the refusal. This is a precedent that a moved call's position against role-file opening is behaviour, which matters to a pure move of start code.

@history_item n=16 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — revive takes `--json`: each refusal is one stdout document with stderr empty, and decision/json-on-every-data-verb now includes revive. This governs the start's refusal documents, which must stay unchanged when the start's steps move out of `run.rs`.
