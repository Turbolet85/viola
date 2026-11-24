@history_item n=1 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — tempfile `persist` snapshot writer, liveness by process check, start order as landed: pid + start time decide `gone` whatever the beat; same process → `live` (≤ 5 s beat) or `stale`; a `live`/`stale` name refuses `already-live`, a `gone` one is taken over at once. Origin of the live/stale/gone reading `viola up` uses to select and skip instances (decision/liveness-heartbeat-and-pid, decision/session-liveness-heartbeat-and-start-time).

@history_item n=2 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the `revive` verb registered: subcommand, exit-1 refusals, child flags, log producer) — `revive <name> [--id] [--fork] [-- <child args>]` / `--list`, sends no channel frame, preflight refusals exit 1, the child launched by name on PATH with `--plugin-dir` first and `--resume <id>` (newest logged id); `run-<name>.ndjson` has revive's start arm as second producer. The per-instance path `up` re-uses for each re-open.

@history_item n=3 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the snapshot's `cwd` and the child's second cwd source) — the snapshot gains `cwd?`; a revived start spawns in the recorded `cwd`, read only after the strict-modes check and required to exist, else `cwd-missing`; a replayed snapshot gives no cwd, so revive over it is refused. Founder-ratified widening; bears on which instances `up` can actually re-open after a reboot (contract/instance-snapshot/cwd, decision/revive-refused-without-cwd, decision/child-cwd-explicit).

@history_item n=4 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the replay's first product reader, the session chain, revive as the second way a wrapper starts) — revive refuses a `live`/`stale` name through the same collision check (run twice: preflight and start), resumes the `gone` instance in place appending to `events.ndjson`, replays no launch, first record `wheel{holder:"driver", cause:"start"}` (founder's answer). `replay::session_chain` gives the logged session ids `up` would resume.

@history_item n=5 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (four crossings ratified by the founder) — the logged session id as the value of `--resume` on a revived child's command line reads as ratified (decision/logged-session-id-on-child-argv). `up` puts that same crossing on N children at once; no new crossing unless `up` adds one.

@history_item n=6 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the instance check before any log, on both arms) — revive's start arm runs `check_instance` before its wrapper log opens, because opening a log narrows an existing home to 0700 and a later check passes a 0770 home; a refusal writes no line. Trap for `up`: any per-instance log line it writes must come after each instance's check (decision/strict-check-before-log-open, decision/instance-check-before-any-read).

@history_item n=7 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `list` is a caller-side board (`board::scan` then `board::read`), contacts no wrapper; the replay fallback gives `live` for a replayed snapshot only with a beat ≤ `STALE_AFTER`, never `stale`; a failed check on the home, `instances/` or any instance refuses the whole board, exit 21, no log, no line. The board read model `up` reads its candidates from (decision/list-is-caller-side-read, decision/board-refused-whole, decision/replayed-snapshot-liveness).

@history_item n=8 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder's Epoch 4 batch: `viola list`'s whole-board exit-21 refusal with no log and no line is his own ruling (decision/board-refused-whole); precedent for whether `up` refuses the whole fleet on one refused instance or skips it.

@history_item n=9 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (Standard Contracts: the agents read, the join and the unwrapped rack land) — `list`, revive's preflight and verify spawn one bounded `claude agents --json` read (`run::agents`, R8 strip, `MAX_FRAME` cap, 5 s kill), joined by `child_pid` / `agent_session_id`; a failed read degrades, never errors. `up`'s skip/selection must not depend on it (decision/agents-listing-never-errors, decision/agents-join-read-ungated, decision/process-liveness-signals).

@history_item n=10 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (Session Liveness: revive refuses a session id live elsewhere) — revive's preflight gains `session-live`: a `claude agents --json` row holding the logged id refuses (two writers on one transcript measured), `--fork` skips it, a failed read passes (fail open). Each revive `up` drives runs this read (decision/revive-refuses-live-session, contract/viola-revive-json-error-codes/session-live).

@history_item n=11 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** (revive takes --json on both arms; revive --list is a cli producer) — revive `--json` prints `{"v":1,"error":"<code>","detail":null}` for its refusals, codes only; without `--json` the human pairs byte-identical; `role_of` files `revive --list` under `cli`. The machine contract `up --json` sits beside (decision/cli-human-text-json-exit-codes, decision/cli-output-conventions).

@history_item n=12 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** (the CLI verbs take the VIOLA_DIR step and canonicalise the home) — `resolve_home`: `--home`, else the `VIOLA_DIR` grandparent when `VIOLA_NAME`+`VIOLA_DIR` pass the shape check, else `<user home>/.viola`; `canonical_home`. Ratified later at 2026-10-18-plugin-precedence-live-readings. `up`'s home and what it hands each re-opened child follow this rule (decision/session-home-from-viola-dir, decision/viola-home-resolution).

@history_item n=13 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the colour layer reads `NO_COLOR` / `TERM` / `COLORTERM`, never sets or echoes them; they decide only SGR bytes; any further environment read needs its own ratification (decision/colour-env-decides-only-sgr, decision/clap-without-color-feature). Binds `up`'s one human line per instance.

@history_item n=14 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the snapshot's pid and started_at as the server identity every frame-writing client checks) — `live_endpoint` returns `LiveEndpoint {endpoint, pid, started_at}` from one snapshot read; `server_verdict` compares pid and OS start time in whole seconds through sysinfo, a zombie reading gone. The mechanism behind «pid-and-start-verified live ones skipped» (decision/server-start-time-read-through-sysinfo, contract/instance-snapshot/pid-and-started-at, contract/instance-snapshot/started-at).

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the snapshot readers moved onto the replay) — every product snapshot reader goes through `read_snapshot_or_replay`; a replayed snapshot carries no `endpoint`, `pid` or `started_at`, so the collision check judges liveness from the heartbeat alone (decision/snapshots-read-through-replay, decision/replayed-snapshot-gives-no-live-endpoint). After a reboot `up` must read the fleet through this path.

@history_item n=16 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (what each replay reader decides; one read per revive, one state-recovered line) — a replayed snapshot writes one `state-recovered` line into the reading process's role file per replay; revive reads the snapshot once for its holder check and recorded directory. Trap: `up` reading each snapshot and then revive reading it again doubles the replay and its line (decision/revive-reads-snapshot-once, decision/replay-writes-no-file, contract/diag-line/state-recovered).

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree check before the start's and verify's log; the pin key taken once) — `check_home_tree` runs at every start before its wrapper log via `checked_exe`; a refusal is exit 1, no log, no line, no child; `run` pins the very bytes its check hashed (decision/start-pins-the-bytes-it-checked, contract/run-start-refusals/home-tree).

@history_item n=18 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the checked entry points and check_instance's call sites; viola list's two home checks) — every checked entry point runs its strict-modes or instance check ahead of any log, a refusal writing no line, the mode left as found; `check_instance(` call sites counted under `src/`. A new verb like `up` joins this list (decision/strict-modes-check-before-log, decision/refused-check-writes-no-line).

@history_item n=19 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the pinned copy on disk: placed once with no clobber) — two concurrent first starts raced the pinned copy (ETXTBSY 4/24); now `place_private_new` no-clobber, a lost placement re-hashes the copy that stands. Precedent for `up` launching several starts at once (decision/pinned-binary-copy, decision/pinned-copy-rehash).

@history_item n=20 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the sanitised error surfaces pattern and its contract binary) — CLI `--json` documents carry codes and fixed messages only, no path, pid or upstream text; `tests/contract_error_bodies.rs` census enumerates every error body. `up --json` must be codes and closed words only and land its rows in that census (decision/sanitised-error-surfaces).

@history_item n=21 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`stale-heartbeat`, a stale holder's own code) — run's collision refusal is `already-live` for a live holder or fresh replayed beat and `stale-heartbeat` for a stale holder; revive's first preflight reading likewise. The codes an `up` per-instance skip/refusal outcome would surface (contract/viola-revive-json-error-codes/stale-heartbeat, contract/process-exit-detail/stale-heartbeat).

@history_item n=22 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola revive --json` documents every refusal of its start) — `Launch.json` makes refusals after a passed preflight print the refused document; a program not found prints `internal-error`; a `pump_child` error prints no document because the child has held the terminal (decision/no-document-after-child-held-terminal, contract/revive-json/*).

@history_item n=23 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site's `internal-error` document) — a `cli` verb with `--json` before a `--` prints `{"v":1,"error":"internal-error","detail":null}` at the catch site; run, revive's start, mcp and hook stay silent there (decision/silent-roles-at-internal-error-catch, contract/cli-json-output/internal-error).

@history_item n=24 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`without_endpoint`, the no-endpoint reading) — with no live endpoint the instance directory is read: a directory is `instance-dead`, anything else `unwrapped-name`; `live_endpoint` gives none for a stale wrapper. Reading a dead wrapper without opening anything, as `up`'s selection needs.

@history_item n=25 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the start's refused socket directory, exit 1) — `socket_dir_ready` before any snapshot, child or bind; a failed create and a refused check both read `socket-dir-refused`, a bind failure does not (decision/socket-dir-refused-covers-create-and-check, contract/run-start-refusals/socket-dir-refused). After a reboot the runtime socket dir is gone and is re-made by each start.

@history_item n=26 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a force-killed wrapper's socket file) — a killed wrapper's socket file stays until a next bind of that name takes it over; nothing removes it; harness cleanup reads refused/absent as gone. Bears on the scope's premise of what a reboot leaves on disk (decision/stale-socket-file-left, decision/cleanup-reads-refused-or-missing-socket-gone).

@history_item n=27 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`viola list`'s one home rule) — `board::scan` holds the home to the read rule with or without an instance, so a 0750 home refuses the whole board; mcp's start and plugin install keep the write rule (decision/list-home-read-rule, decision/strict-read-rule).

@history_item n=28 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the shared Frozen guard) — root tests make a wrapper read stale by SIGSTOP through one shared `tests/support/frozen.rs` guard waiting for state `T`; the one sanctioned way for `up`'s tests to plant a stale-but-matching wrapper (decision/root-tests-freeze-wrappers-with-one-guard).

@history_item n=29 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS target: Linux only) — the core on any current Linux, CI on `ubuntu-latest` against the fake agent, the 0.2.0 window features on Hyprland, no Windows or macOS code; no design may depend on tmux or a shell (decision/linux-only, decision/window-features-target-hyprland, decision/no-tmux-or-shell-dependency).

@history_item n=30 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the wrapper endpoint is a Unix socket only) — the socket lives in `$XDG_RUNTIME_DIR/viola/`, `/tmp/viola-<uid>/` fallback, never the `/tmp` root; the runtime dir is a tmpfs on this host, so a reboot clears every endpoint while `~/.viola` snapshots and logs stay — central to telling «was live before the host went down» (decision/socket-in-per-user-dir, decision/wrapper-ipc-local-socket).

@history_item n=31 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (child program resolution) — `resolve_program`: a given path as given, a bare name on `PATH`; revive resolves from its own PATH and current directory, never the recorded one (decision/child-program-resolution, decision/revive-program-resolution).

@history_item n=32 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the `.cmd`/`.bat` refusal `batch-script-child` retired) — revive's closed `--json` refusal codes are ten (strict-modes-failed · already-live · no-session · session-live · cwd-missing · stale-heartbeat · pinned-hash-mismatch · squatted-name · socket-dir-refused · internal-error); a closed word no build can emit leaves with its emitter (contract/cli-json-output/revive-refusal-codes).

@history_item n=33 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the R8 strip: exact names, `claude_env_keep` the one pass-list) — every inherited `CLAUDE*` name removed from the child except `claude_env_keep`, the identity floor always; names only. Applies to every child `up` causes to start (decision/child-env-claude-strip, decision/r8-identity-floor).

@history_item n=34 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0: no web or graphical frontend; the view beyond `viola list` is the keyboard-driven terminal fleet console tiled on Hyprland, owed by Epoch 6 entries. Keeps `up` a CLI verb, not a console feature (absence/graphical-frontend).

@history_item n=35 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Standard Contracts / Infrastructure Patterns / Conventions / Occupied Resources) — `viola-hyprland` is the one compositor seam, sync, tokio-free, depending on viola-core alone; hyprctl spawned directly with a closed argv; closed errors unsupported · unreachable · oversize · malformed · refused; an unparseable reply is `malformed`, not `unknown`; test-only `viola-hypr-tap`; the root bin's «depends on all members» dropped. Any workspace placement by `up` goes only through this seam (decision/hyprland-seam-crate, decision/hyprctl-direct-spawn, contract/hyprland-seam-errors/*, decision/root-bin-consumes-normalised-events).

@history_item n=36 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Conventions) — retired exit code 11 is never reused or renumbered (decision/retired-exit-codes-never-reused, contract/cli-exit-codes/11); removing fields and a kind kept every own-format `v` at 1. Binds any exit code `up` assigns.

@history_item n=37 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Established Decisions 1 of 2) — the budget evaluation left the start order (decision/run-start-order) and test homes no longer share `budget.json` (decision/test-isolated-viola-home).

@history_item n=38 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Established Decisions / Conventions) — no `--settings` at start, snapshot loses `statusline_command`; protocol-version-bump-rule's exception widened (ratified at 2026-11-15T18-01-44-wrap); snapshots read through the replay give no endpoint (decision/snapshots-read-through-replay, decision/protocol-version-bump-rule, absence/status-line-interception).

@history_item n=39 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — root test waits share the one 12 s `WITHIN` bound (was 7 s); test channel sockets bind under a short `/tmp` root (decision/root-waits-share-one-bound). `up`'s tests of N sequential starts sit inside this bound.

@history_item n=40 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — test homes live on the dev host's tmpfs through the `target/e2e-home` link; standing rule: a stalled-start red is a finding about the backing, reported and never re-run for green (the scope's `hostwatch.py` / `df -m /tmp` read); test-side removals outside the work dir ratified (decision/test-side-removal-outside-workdir, decision/test-home-owner-sweep).

@history_item n=41 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Established Decisions / Standard Contracts / Occupied Resources) — a start on a verified CLI writes the trust record of an untrusted spawn folder; a revive keys it by the recorded `cwd`; the spawn directory lives only in the 0600 snapshot and the child's Claude config entry; `CLAUDE_CONFIG_DIR`'s value read by run and revive when kept (decision/run-start-order, contract/instance-snapshot/cwd, decision/spawn-directory-confined-to-snapshot, decision/env-strip-names-only). `up`'s revives each take this step.

@history_item n=42 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (Established Decisions / Occupied Resources) — with `verify_new_cli` on, a start meeting an unstamped claude version runs verify once after the bind and gates on the final verdict, under `ledger/verify.lock` (a held lock means no verify and an unverified start); start order amended. A fleet of N starts after a CLI update can hit this lock (decision/start-verifies-new-cli-version, decision/automatic-verify-attempt-lock, contract/config-json/verify-new-cli).

@history_item n=43 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (Cross-cutting Patterns) — viola run passes the home to its claude child only through `VIOLA_DIR` and to its automatic verify child only through `--home` (decision/viola-home-resolution); precedent for how `up` must hand the home to each child it spawns.

@history_item n=44 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Standard Contracts / Conventions / Infrastructure Patterns) — the wrapper appends a log-only `lineage {parent, role}` line at every start; the snapshot gains `lineage?`, recovered by replay from the last line; list items gain `parent?` (only when it parses as another instance's name) and `role?`, and the `ok` payload an optional `links`; the board may hold links. `up`'s parents-first order comes from this record (contract/event-data/lineage, contract/instance-snapshot/lineage, contract/list-item/parent, decision/snapshot-replay-log-fields-only).

@history_item n=45 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Established Decisions / Occupied Resources) — lineage is a label only: no wheel, link, send or answer path reads it, held by a source census (decision/lineage-label-only); revive keeps the parent and role its real snapshot records; `run --parent` (self-parent a usage error, inherited `VIOLA_NAME` fallback) and `--role`. Ordering by lineage in `up` is a new reader the census must admit.

@history_item n=46 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — every start hands `--model M --effort E` after `--plugin-dir`, never read back or verified; a log-only `launch` line after `lineage`; snapshot `launch?` replayed; revive keeps the recorded pair (override, else record, else default; provisional) and the pair on the revived argv is a ratified crossing; revive gains `--model` / `--effort`; config `default_model` / `default_effort`; model word format (decision/launch-pair-handed-not-verified, decision/revive-pair-from-record, decision/recorded-pair-on-child-argv, contract/child-argv/launch-pair, contract/cli-verbs/revive, decision/revive-in-place, decision/revive-resumes-in-place).

@history_item n=47 marker=2026-11-23-seam-requests-for-viola-open
**2026-11-23-seam-requests-for-viola-open** — the seam's closed dispatch set gains `hl.dsp.window.move` (move to workspace) beside `hl.dsp.focus`, each one Lua element whose inserted values are only a closed window address or an i32 workspace id, measured on Hyprland 0.56.2 (decision/hyprctl-direct-spawn). The request `up` would need to place a window in its recorded workspace.

@history_item n=48 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Standard Contracts) — the wrapper appends a log-only `window {address, workspace, placement, origin}` line after `launch` only under `--origin`; snapshot `window?` replayed (last `window` after last `launch`, a later `launch` clears it); list items gain `window?` / `origin?`; `viola open` exit codes 0/1/2. A session started by hand records no window, and window addresses die with the compositor — the scope's premise on «recorded workspace and placement» (contract/wrapper-event-appends/window, contract/instance-snapshot/window, contract/event-data/window).

@history_item n=49 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Conventions / Established Decisions / Occupied Resources) — `viola open <name> [--parent] [--role] [--cwd] [--workspace N] [--model] [--effort] [--json] -- claude`; exit 1 covers open's refusals (terminal-not-found, not-live); open refuses a `live`/`stale` name like run and revive; hidden `run --origin <WORD>` and `run --place <N>` (usage error without `--origin`); process-exit details `terminal-not-found`, `not-live`. The window-spawning path `up` is to reuse (contract/cli-verbs/open, decision/no-start-over-live-instance, contract/viola-cli-arguments/run-origin, contract/viola-cli-arguments/run-place).

@history_item n=50 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Established Decisions) — focus has no driver-facing form: CLI only, adding an MCP tool is a further boundary widening, provisional on fork F-A's conservative arm (decision/focus-has-no-driver-form). Direct precedent for the scope's ruling that `up` is CLI-only, no MCP tool.

@history_item n=51 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Conventions / Occupied Resources) — exit 1 names focus among the refusing verbs and exit 21 strict-modes-failed covers focus; focus logs as `cli` into `cli-<name>.ndjson` with the target instance's name; the process-exit detail enum gains the seam's words (unsupported · unreachable · oversize · malformed · refused · window-not-found); `name/command/focus` registered. The pattern `up` follows to add its verb, exit causes and diagnostics role file (contract/cli-exit-codes/1, contract/cli-exit-21/strict-modes-failed, contract/diagnostics-role-files/cli-name, contract/process-exit-detail/*).

@history_item n=52 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** (send and human split by phase) — the root bin's only human writer is the `human` module (`src/human.rs` + `src/human/` phase files: mirror, escape, board, wait, verbs, hints); `up`'s per-instance lines land there (decision/single-human-writer).
