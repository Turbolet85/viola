@history_item n=1 marker=2026-11-23-seam-requests-for-viola-open
**2026-11-23-seam-requests-for-viola-open** (Input Validation) — `~ decision/hyprctl-read-deadline` moved from provisional to standing at its unchanged 5 s (the operator's F8, delegate confirm PROVISIONAL). This is the settled bound carry `cry-2oejf2md` turns into an a11y time-bound row. The open's 5 s window-appearance poll and 30 s liveness wait come on top of it, and each new bound needs a row of its own.

@history_item n=2 marker=2026-11-23-seam-requests-for-viola-open
**2026-11-23-seam-requests-for-viola-open** (Data Protection, two entries) — `~ decision/hyprland-recorder-string-allowlist`: the rule now covers the monitors query (keeps no string) and the move dispatch (kept only as `ok`). The guard became `record_keeps_no_monitors_string` in place of the planted title case. The chunk's tests for the recorder's `move-refused` / `move-unseen` / `window-left` codes (carry `cry-pu3g6nb7`) sit under this allowlist and scrub.

@history_item n=3 marker=2026-11-23-seam-requests-for-viola-open
**2026-11-23-seam-requests-for-viola-open** (Authentication & Authorization) — `+ decision/hyprland-recorder-moves-own-window`, PROVISIONAL on the overseer's delegated host rule: the test-only recorder moves only the window of a terminal it started, then reaps that terminal. It is the only precedent for acting on a live desktop window. The product's new move of the window of a terminal that `open` started (and R2's no-start probe under the lock) has no row yet. The planner must say how the product's move is bounded to the window it opened and never a user window.

@history_item n=4 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Threat Model Summary) — `~ decision/child-output-tolerant` and `~ decision/child-output-tolerant-unknown`: a child output failure became "unknown or a closed failure word" because the hyprctl reply read gives the seam's closed words (oversize, malformed, unreachable). The open's placement record (`unplaced`, the seam's `unsupported` / `unreachable`) must stay within those closed words.

@history_item n=5 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Input Validation) — `~ decision/shared-validators-in-core` gained the exception that the window Address validator lives in the seam crate. `~ decision/parser-fuzz-scope` added the hyprctl reply parsers. `+ decision/hyprctl-read-deadline` (null stdin, named env removed, 5 s kill and reap). This chunk is the seam's first product caller, so these rows now bind product code. A new validator for the hidden origin or workspace flags must either live in core or name its exception.

@history_item n=6 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Data Protection, two entries) — `+ decision/hyprland-reply-text-dropped`: window titles, classes and every other reply string are dropped at the parse and reach no line or error. `+ decision/hyprland-recorder-string-allowlist`. The new `window {address?, workspace?, placement, origin}` log kind and snapshot field must carry only addresses, numbers and closed words, never reply text.

@history_item n=7 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (Threat Model Summary) — `~ decision/cli-input-trust-boundary` now names the closed model-word and effort-word checks. `open` adds `--role`, `--workspace N`, `--cwd` and two hidden checked flags (origin, target workspace) on the started `run`'s argv, so this row must name their checks as well.

@history_item n=8 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (Input Validation, two entries) — `+ decision/launch-flags-checked` (exit 2 before anything is read), `+ decision/launch-pair-argv-crossing` (separate argv elements of a direct spawn, never a shell, ratified as a crossing), `+ decision/launch-flags-refused-after-dashes` (one fixed usage line, echoing none of the caller's words; it is the `PAIR_AFTER_DASHES` the scope puts beside the new refusal of any word after `--` other than `claude`), and `+ decision/board-launch-pair-re-parsed`. `~ contract/revive-arguments/id`. `+ decision/effort-word-check-in-agent-crate`. This is the direct precedent for the open's allowlist and its hidden flags crossing onto `run`'s argv, which W1 widens.

@history_item n=9 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (Data Protection and Authentication & Authorization) — `+ decision/launch-pair-kept-to-snapshot-and-event-log`: a log-only event kind plus a snapshot field that leaves only through `list --json` and the MCP list item. `+ decision/launch-pair-authenticates-nothing`. Both are PROVISIONAL. They are the template for the window record's log-only `window` kind, its snapshot `window?`, the `list --json` / MCP `window?` / `origin?` (W3), and the rule that `origin` authenticates nothing.

@history_item n=10 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Input Validation) — `+ decision/run-lineage-flags-checked` (`--parent` through the name check, `--role` a code word, a self-parent exit 2) and `+ decision/board-lineage-and-links-name-checked`, both PROVISIONAL. The open's `--parent` / `--role overseer|builder|worker` pass these through to `run`, so the closed role set and the parent check must hold at `open` before any spawn.

@history_item n=11 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Authentication & Authorization and Data Protection) — `+ decision/lineage-parent-authenticates-nothing`: the parent comes from `--parent` or the inherited `VIOLA_NAME`, is never an identity and is never joined into a path. `+ decision/lineage-kept-to-snapshot-and-event-log`. The scope's parent default (the caller's `VIOLA_NAME`) and the parent-window lookup (its recorded window, else an ancestry walk) rest on this. The parent name must stay out of every path except a validated instance path, and must decide no authority.

@history_item n=12 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Threat Model Summary, Data Protection, Secret Management) — `~ decision/spawn-cwd-snapshot-only`, `~ decision/cwd-only-in-snapshot`, `~ decision/nothing-written-under-claude-home`, `+ decision/start-writes-trust-record-flag-only`, `+ decision/start-trust-skips-trusted-folders`, and the `CLAUDE_CONFIG_DIR` value read at `run`'s start. The open's `--cwd` becomes foot's `--working-directory` and then `run`'s spawn directory, so it keys a trust record in the Claude config. Because it can carry the OS username, it must stay off log lines, error bodies and stdout, and off the `window` log line too.

@history_item n=13 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (Threat Model Summary, Data Protection: ratifications of the session-folder rows) — the founder ratified `decision/cwd-only-in-snapshot`, `decision/spawn-cwd-snapshot-only` and `decision/nothing-written-under-claude-home` (and the start-trust Data Protection rows). These standing rows bind the cwd that `open` hands foot and `run`.

@history_item n=14 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** (Threat Model Summary, Data Protection) — set the first exception to `decision/nothing-written-under-claude-home` / `decision/viola-writes-nothing-in-claude-dir` (verify's own trust record) and `+ decision/trust-record-writer-guarded`. `open` must add no writer under `~/.claude`. The only trust write stays `run`'s start.

@history_item n=15 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Threat Model Summary, Input Validation: shell-out rows) — `− decision/statusline-only-shell-out`, `− decision/single-shell-out` and `− decision/statusline-shell-out`: viola now runs no shell at all, and the ban stands in architecture `decision/no-shell-out`. The open's foot spawn (F2: no shell line built from the arguments, whatever characters they hold, v2-08) must be a direct argv spawn and must not reopen a shell exception.

@history_item n=16 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Threat Model Summary: replay and state root) — `~ decision/replayed-snapshot-yields-nothing`, `~ contract/cli-state-root/home-flag` and `~ contract/cli-state-root/viola-dir` dropped `statusline_command`. A replayed snapshot carries no endpoint, pid or start time. A replayed `window?` must likewise give no frame and no move.

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home tree checked at `run` and `revive`'s start, before the wrapper log) — `check_home_tree` runs before any log opens, and a refusal is exit 1 with no log, line or child. This underlies `decision/start-checks-home-tree-before-log-opens` and `decision/state-tree-strict-modes-before-use`. The scope's exit 1 `strict-modes-failed` before any log follows this order, and `open` must place its own check before its own log or role file.

@history_item n=18 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`ledger/stamps.json` checked before `ledger/` is made) — the trap: a check placed after a create, a mode set or a log open that narrows the path passes a widened tree. This became the ban `decision/strict-check-before-narrowing`. Any directory that `open` creates or opens must come after its check.

@history_item n=19 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (every client checks the instance before the snapshot; snapshot readers on `read_snapshot_or_replay`; collision check through the replay) — `check_instance` comes before any snapshot read, and only a parsed snapshot gives reference values. The collision check judges a parsed holder by pid, start time and heartbeat, and a replayed one by the beat alone. This applies to the open's liveness wait through `live_endpoint` and to its pre-spawn `already-live` / `stale-heartbeat` read.

@history_item n=20 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`check_instance`'s set and the read rule as built) — the read rule covers the home, the instance and `diagnostics/`, with the fixed `Readable` / `Symlink` refusal messages (`contract/strict-refusal-message/readable` and `/symlink`). These are the refusals `open` reuses.

@history_item n=21 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`viola link` / `viola unlink`'s instance check before any log) — a trap recorded for later verbs: opening a role file narrows an existing home to 0700, so a check placed after `client::start` passes a 0770 home. A new verb's strict-modes check goes before its role file. This applies directly to the new `open` verb.

@history_item n=22 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the two instance checks ahead of any log) — a log opened before or after a refused check narrowed the home and erased the state the check refuses, so a refused check writes no line and leaves the mode as found. This is the origin of `decision/strict-refusal-leaves-no-trace`, which `open`'s refusal must meet.

@history_item n=23 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the home strict-modes check is owner and mode alone) — the strict-modes check reads owner and mode, and an unstattable home is a strict-modes refusal (`decision/unstattable-home-refused`). The Linux-only rule applies to the `open` verb as to every other.

@history_item n=24 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (child program resolution) — a given path is resolved as given and a bare name on `PATH`, with portable-pty handed an absolute path and `ProgramNotFound` as the one refusal. This is the precedent for finding `foot` on PATH (`decision/child-path-trust`) and for the open's `terminal-not-found` exit 1.

@history_item n=25 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the IPC endpoint is a Unix socket alone) — one per-instance socket in a verified per-user 0700 directory (`$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`). This underlies `decision/socket-in-per-user-dir`, which the instance that `open` starts uses.

@history_item n=26 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at a hook and at a start) — `socket_dir_ready` checks, creates the directory only when absent, and checks again, before any snapshot, child or bind, with `socket-dir-refused` exit 1. This underlies `decision/start-checks-socket-dir-before-bind`. A start launched by `open` refuses the same way. The planner must say what `open` reports when its started `run` refuses this way: the window is left standing and the open reads `not-live`.

@history_item n=27 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the Unix socket's place in the threat model) — the socket stands in a per-user 0700 directory, never the world-writable tmp root, and a foreign, loosened or symlinked directory is refused with no bind and no frame. This underlies `decision/socket-in-owner-only-dir`.

@history_item n=28 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the tests holding the directory and peer checks, and no seam to drive them) — the ban `decision/no-seam-forcing-peer-refusal`: no env, flag or feature seam forces a refusing peer decision. The open's fake-agent stand-in `foot` and its `clients` reply must not become such a seam.

@history_item n=29 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (`run`'s collision check: the server check retired as moot) — the collision check opens no connection and so needs no server verification (`decision/collision-check-needs-no-server-verify`). The open's `already-live` / `stale-heartbeat` read before any spawn is the same frameless judgement.

@history_item n=30 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder's code at the collision check) — `stale-heartbeat` for a stale holder, `already-live` for a live one or a fresh replayed beat (`contract/collision-check/*`). These are the words `open` reads before any spawn.

@history_item n=31 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's document under `--json`) — `json_asked` reads `--json` before any `--` and prints `{"v":1,"error":"internal-error","detail":null}` with stderr empty. `run`, `revive`'s start, `mcp` and `hook` stay silent there (`contract/internal-error-output/json`). `open --json` falls under this rule, with a `--` and `claude` after it.

@history_item n=32 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`revive --json` documents every refusal of its start) — closed exit-1 codes and the `{"v":1,"error":"<code>","detail":null}` document (`contract/revive-json/*`). This is the precedent for the open's closed error document and codes (`terminal-not-found`, `not-live`, `already-live`, `stale-heartbeat`, `strict-modes-failed`), each holding no path or pid.

@history_item n=33 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `revive`'s closed-code `--json` documents, and CLI home resolution (`--home`, else the `VIOLA_NAME` / `VIOLA_DIR` instance home, else the default, canonicalised, nothing created; `decision/cli-home-resolution-order`). `open` resolves its home this way and hands it to the pinned `run` as `--home <home>`. An agent caller's `VIOLA_DIR` choosing the home also feeds the parent default.

@history_item n=34 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** (the `VIOLA_DIR` home step ratified) — the founder ratified the `VIOLA_DIR` step and canonicalisation of CLI home resolution as a widening. `open` reuses it and widens it no further.

@history_item n=35 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the clients' screen of a peer's reply) — `sanitised_fault` and `is_code_word`: only closed values of a peer reach a CLI document or MCP result (`decision/client-reads-closed-values-only`, `decision/client-reply-code-words-only`). Any value that `open` reads back (liveness, the window record) and prints in `--json` must be closed words or re-parsed values.

@history_item n=36 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error bodies' census) — `product_error_writers_are_each_named` fails on an unnamed product error writer, and `thiserror_messages_interpolate_nothing` on an interpolating message (`decision/every-error-writer-named`, `decision/error-messages-interpolate-nothing`). The open's new error documents and any new error enum must join the census.

@history_item n=37 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader census) — `product_readers_are_each_bounded_or_named` requires every product reader to be bounded or named (`decision/bounded-readers-census`). The seam's first product caller and any new read in `open` or the wrapper (the `clients` poll, the ancestry walk) must pass it.

@history_item n=38 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (serde_json's `unbounded_depth` banned in `deny.toml`) — every parse of external bytes keeps the default recursion limit (`decision/serde-json-unbounded-depth-banned`, `decision/serde-json-bounded-depth`). This now applies to the hyprctl reply parse on a product path.

@history_item n=39 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — readers count torn lines, unknown kinds and unknown fields without logging, an unknown kind is not returned, and the next append heals (`decision/readers-tolerate-torn-lines`, `decision/events-reader-bounded-lines`). The new log-only `window` kind, replayed with the last line winning, must go through this reader. An older build must read it as an unknown kind.

@history_item n=40 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the recorded cwd ratified) — the recorded cwd used as a spawn directory is a ratified crossing, and a host path that can carry the OS username stands only in the 0600 snapshot. The `--cwd` that `open` takes is the same class of value.

@history_item n=41 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `list` is a strict-modes entry point that checks before any open, and its rows escape every control character. `list --json` gains `window?` / `origin?`, while the six-column board stays unchanged. Both new fields must come from viola's own state and re-parse before they show.

@history_item n=42 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the first-start race on the pinned copy closed) — the pinned copy is placed once, never replaced, and re-hashed before reuse. The foot argv names `<pinned viola>` by its absolute path (`decision/exec-commands-absolute-pinned-path`), so `open` must pin, or take the pin, before the spawn and never name viola through PATH.

@history_item n=43 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the precedent for agent-driven desktop interaction. Agent-typed keys went to a compositor the chunk started itself, nested on the locked desktop, with the lock untouched and read before and after each step, ratified by the founder. R2 runs on the operator's locked or unlocked desktop and must hold the same care.

@history_item n=44 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the scratch footer reading of a newer CLI accepted) — one scratch `viola run` start of the user's CLI is no standing permission, and another reading needs its own answer. R2's single live `claude` start (no spare, `scripts/r2-open.sh`) needs its own word in the same way.

@history_item n=45 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Secret Management, Authentication & Authorization) — `+ decision/live-rig-isolated-child` (a private `CLAUDE_CONFIG_DIR`, a closed kept environment), `+ decision/live-rig-credentials-copy`, and `~ decision/live-round-rig-keys-own-pty`. R2 runs with a private `CLAUDE_CONFIG_DIR` under a scratch home outside every repository, so these rows bound it.

@history_item n=46 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** (Secret Management) — `~ decision/live-rig-isolated-child`: the stamp-only witness was disproved by a host writer of the shared config and replaced by the rig-made needle search. R2's check that nothing touched the operator's real `~/.claude*` must use the witness as it now stands.

@history_item n=47 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** (Secret Management) — the VH-arm exceptions to `decision/live-rig-isolated-child` and `decision/live-rig-credentials-copy`. A run under the real home needs an explicit home, Claude dir and user config. R2's script must decide which of these isolation shapes it takes.

@history_item n=48 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — the first test seam carve-out (fake-agent only, compiled out of release). Another seam or env read needs its own Decisions Log entry (`decision/test-seams-fake-agent-only`, `decision/new-env-read-needs-log-entry`, `decision/test-seams-compiled-out`). The fake agent's stand-in `foot` and its pid-keyed `clients` reply must stay under the fake-agent feature and add no env read without an entry.

@history_item n=49 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the colour facts joined the admitted env set, restating "NEVER another seam or environment read … without its own Decisions Log entry" (`decision/controls-not-switchable`, `decision/no-control-switch-off`). The scope says placement takes no environment read. Any `HYPRLAND_*` or `WAYLAND_*` read by `open` or the wrapper would need such an entry.

@history_item n=50 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — `scripts/release-check.sh` refuses any executable other than viola and any `test-support` / `fake-agent` artifact (`decision/release-build-no-test-binaries`, `decision/release-build-without-test-features`). `viola-hyprland` moves from `[dev-dependencies]` (`Cargo.toml:174`) to a product dependency. Its test-only recorder and any stand-in must stay out of the release graph.

@history_item n=51 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — the fuzz workspace exemption from `cargo deny` (`decision/cargo-deny-gate` except). The hyprctl reply parsers' fuzz target sits there. Making the seam a product dependency puts its graph under the root deny gate.

@history_item n=52 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** (Dependency Security) — `+ decision/advisory-ignore-grounds` and `+ decision/advisory-ignore-list` (a closed list, two grounds, re-read at each epoch boundary). The root deny gate that the seam now joins as a product dependency admits no new ignore outside these grounds.

@history_item n=53 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Input Validation) — `~ decision/shared-validators-in-core` except now names the Hyprland Address validator and the transcript path check. The cell is near its bound, which is why the effort check took a sibling row. A new validator for the origin or workspace flags outside core needs the same treatment.
