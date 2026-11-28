@history_item n=1 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (Occupied Resources, Cross-cutting Patterns, Conventions) — added `name/command/console`, and widened `decision/codes-only-process-logs` to five role files, `console` among them (PROVISIONAL, delegate overseer). It also put console's two refusals under `contract/cli-exit-codes/1`. This is the scaffold the view builds on. The view's `console-frame` `shown` / `hidden` fields and the board and agents lines it writes go to that `console` role file, which must stay codes-only.

@history_item n=2 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (Established Decisions, Project Intent) — added the console exception to `decision/cli-human-text-json-exit-codes`, `decision/cli-output-conventions` and `decision/new-capabilities-as-verbs`: viola console takes no `--json` and draws its own screen (PROVISIONAL, delegate overseer). The view keeps that exception. It adds no `--json` and no machine document, and `LAST` reaches no serialised surface.

@history_item n=3 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0) — [Real-time Strategy] became the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14), owed by «Console scaffold», «Fleet console view» and «Console actions». [GUI Control Scope] became no page, «what the console shows being «Fleet console view»'s to name». This is where `decision/terminal-fleet-console` comes from and why this chunk names the view's content. `absence/graphical-frontend` and `absence/graphical-interface` hold: the view is terminal-only.

@history_item n=4 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — retired the GUI. Its page's starting intent was session rows, liveness, status, wheel holder, pending dialog, link set, `skipped` counts and a live feed. The item, `budget` and `skipped` shapes moved to the `list` payload contract (CLI `list --json`, MCP `list`). This is the precedent for what a fleet view shows. It also explains why the board's payload is one shared contract that `LAST` must not enter.

@history_item n=5 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — landed the board as the view now reuses it:
- `viola_state::board::scan` then `board::read`, caller-side, contacting no wrapper.
- The `board::log_pass` skip counts, summed over every instance, gone ones included.
- The scan's strict check before any open, the whole-board exit-21 refusal with no line, and `list` as a `cli` producer under `VIOLA_NAME` only (`state-recovered`, `config-json`).
- `escape_row` and the board writers.

This is the read the console calls once a second. Its whole-board refusal and line rules decide what the console does on a refused scan.

@history_item n=6 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — made the whole-board exit-21 refusal with no log and no line the founder's own (`decision/board-refused-whole`). A console that re-reads the board inherits a refusal the founder ruled. How the view shows or survives a refused scan must not widen it.

@history_item n=7 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (Standard Contracts) — CLI `list` makes one `claude agents --json` read (`run::agents::read_here`) and joins it with `board::join`.
- A matched row sets `idle` / `busy`. Unjoined rows become read-only unwrapped items.
- A failed, oversize or malformed read leaves every status `unknown`.
- The read writes one `agents-probe` pair and `parse-rejected{claude-agents-json}` under an open log.
- `list` gates the read on no stamp (`decision/agents-join-read-ungated`).

STATUS comes only from this spawn. The console calls it as it is, at most every 5 s off the loop, and its lines land in the console role file.

@history_item n=8 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (MCP list builds the board and reads agents) — MCP `list` builds the same board through `list::board_document` and makes its own agents read on each call, its `agents-probe` pair in `mcp.ndjson`. This is the precedent of a second process driving the board read plus the agents spawn. Any last-activity field added to the board row must stay out of `board_document`'s document, which MCP `list` serialises.

@history_item n=9 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — registered `src/human/style.rs` and the colour reads, `NO_COLOR` / `TERM` / `COLORTERM` (`decision/colour-env-decides-only-sgr`). `COLUMNS` is never read. The `board.rs` gloss names its width cut and spans. Any further environment read needs its own ratification. The view gets its width from the terminal, not `COLUMNS`. The scaffold is plain ASCII with no SGR. The board's existing width cut is the code nearest the new `...` row cut.

@history_item n=10 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** (send and human split by phase) — split `src/human.rs` into `src/human/` (`escape.rs` with `escape_row`, `board.rs`, ...). The root bin's only human writer is the `human` module (`decision/single-human-writer`). The view's frame writer `src/human/console.rs` and the escaping it reuses live under that one-writer rule.

@history_item n=11 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the split modules) — split `src/human/board.rs` into `src/human/board/` (`pairs.rs`, `bay.rs`, `cells.rs`, `rows.rs`). The row builder, the lineage `forest`, the six captions and the wheel words the view must reuse (`rows.rs`, `cells.rs`) sit there, not in a second tree builder.

@history_item n=12 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (the next append heals a torn last line) — appenders heal a torn last line. `events::read_from` counts `unknown_kinds` / `unknown_fields` / `torn_lines` per read and logs nothing, and showing those counts was owed to the board. The last-activity age comes from the newest `events.ndjson` line in this same pass. A torn last line must not be taken as the newest activity, and readers never rewrite the log (`decision/append-heals-torn-line`, `decision/reader-counts-skipped-lines`).

@history_item n=13 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (bounded readers) — every product reader of external bytes is bounded in its own statement or named as an exception, held by the census in `tests/contract_bounded_readers.rs` (`decision/bounded-readers`). The board read re-run once a second reads every events log whole (research M2). A newest-line or age read added to that pass must stay bounded or the census goes red.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (snapshot readers moved onto the replay; one state-recovered line) — every product snapshot reader goes through `read_snapshot_or_replay`. A replayed snapshot gives no endpoint and is live or gone from the heartbeat alone. Each replay writes one `state-recovered` line into the reading process's role file. Each re-read can therefore write lines into the console role file. A console re-reading once a second can repeat those lines, which matters for the "on change only" skip-count line (F7).

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (read rule, check_instance set, diagnostics opens, checked entry points) — established:
- The strict read rule and `check_instance`'s widened set.
- `check_diagnostics` before every role-file open (a refusal is a failed open).
- The list of checked entry points, each check before any log, a refused check writing no line.

The console's board re-read and role file inherit these: no line on a refused check, the mode left as found (`decision/strict-check-before-log-open`, `decision/refused-check-writes-no-line`).

@history_item n=16 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`viola list`'s one home rule) — `board::scan` holds the home to the read rule with or without an instance, so a 0750 home refuses the whole board. `mcp`'s start and `plugin install` keep the write rule (`decision/list-home-read-rule` and its except). The console's scan inherits the read rule on the home.

@history_item n=17 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Standard Contracts, Occupied Resources) — the `list` payload went to `{items, skipped}`. `decision/list-reads-state-directly` reads instances only, and the snapshot replay recovers log-derived fields only. `budget_paused` left the item, and the `list-home-read-rule` except dropped the budget read. There is no budget column to show. The board row's fields and the payload contract the view must leave untouched are those that remain.

@history_item n=18 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Infrastructure Patterns) — `decision/board-read-model-shape` lost `statusline_command` from the fields the board never holds. The board read model stays sync, std-only and Claude-free, holding no endpoint, pinned_bin, cwd, pid or started_at. The new unserialised last-activity time on the row must keep that shape.

@history_item n=19 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — added lineage:
- List items gained `parent?` / `role?` and the payload gained `links?`, with no `v` move.
- `contract/list-item/parent` is kept only under a name check, and `role` only when it is a code word.
- The snapshot and replay gained `lineage`, and `decision/board-read-model-shape` dropped its links ban.
- `decision/lineage-label-only` was added (no wheel, link, send or answer path reads parent or role).

The view's lineage trees read this, through the board's existing `forest`, as a label only.

@history_item n=20 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Established Decisions) — `decision/lineage-label-only` gained «viola up reads the parent only to order its re-opens». This is the precedent for an exception to the label-only rule. The console reads parent only to draw trees in the board's order, which is display, and needs no further exception unless a control path reads it.

@history_item n=21 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (Standard Contracts) — list items gained `model?` / `effort?` and the snapshot gained `launch`, additive with no `v` move. This is part of the `contract/list-payload/items` shape the view's board row sits beside. It is also the precedent for adding fields to the board item. `LAST`, by contrast, must stay unserialised.

@history_item n=22 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Standard Contracts) — list items gained `window?` / `origin?`, and the snapshot and replay gained `window`. The current `contract/list-payload/items` and `contract/instance-snapshot/data` shapes, and the replay's log-derived field list, stand as this entry left them (`decision/snapshot-replay-log-fields-only`).

@history_item n=23 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — [Session Liveness]: pid plus start time decide `gone` whatever the beat. The same process is `live` (≤ 5 s) or `stale`. This is the root of the board's LIVE words (`decision/session-liveness-heartbeat-and-start-time`, `contract/session-liveness/*`). The view's «stalled» is the board's `stale` (F3), and gone instances are omitted.

@history_item n=24 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `pending_dialog` is written before the dialog's event line and written back to none on a failed append. This is the ordering behind `contract/instance-snapshot/pending-dialog`. `dialog_pending` comes only from the snapshot, so a replayed snapshot reads `false` (`contract/list-payload/dialog-pending`). The view's `DIALOG <kind>` cell inherits that.

@history_item n=25 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (the wheel as landed) — the wheel lives in `src/run/wheel.rs`, in memory, with the `wheel` record and snapshot written on a worker thread (`decision/wheel-in-wrapper-memory`, `contract/wrapper-event-appends/wheel`). The wheel cell the view reads from the snapshot mirrors memory, can lag a move briefly, and is absent for unwrapped rows.

@history_item n=26 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (Established Decisions) — `decision/wheel-starts-with-driver` gained «a key the human types into the folder-trust dialog before the session starts included». This is the current text of a picked wheel row, which the view's WHEEL cell reflects.

@history_item n=27 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — set every role's codes-only log to go to its home-level role file under `diagnostics/`, with content-bearing detail only in the instance detail files. Process-log lines carry no `v` (the `decision/versioned-records-no-migrations` except). The console role file and its `console-frame` / `console-key` lines follow this plane. `shown` / `hidden` are counts only, and the schema pinning their fields lives in `schemas/diag-line.v1.json`.

@history_item n=28 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — made clap `default-features = false` without `color`. This closed the anstream route that read `CLICOLOR` / `NO_COLOR` / `TERM` / `CI`, because a dependency's env read breaks the env rule like a direct one. It made `src/human.rs` the one refusal writer. This is a trap for the view: no crate pulled in for terminal drawing may read colour or size variables.

@history_item n=29 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — set the sole-root `deny-sync.toml` tokio ban per crate in `scripts/sync-crates.txt` (`decision/sync-crates-tokio-free`, `decision/no-tokio-in-sync-crates`). The board read model is in `viola-state`, a sync crate. The console's 1 s re-read and its 5 s off-loop agents read must use std threads (`decision/hybrid-concurrency`), with no async runtime.

@history_item n=30 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** (the CLI verbs take the VIOLA_DIR step) — every CLI verb resolves its home through `resolve_home` / `resolved_home` (`--home`, then the grandparent of `VIOLA_DIR`, then the default) and canonicalises it. This was later ratified at 2026-10-18-plugin-precedence-live-readings. The console's board reads resolve the home the same way (`decision/viola-home-resolution`).

@history_item n=31 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the shared Frozen guard) — `tests/support/frozen.rs` `Frozen` is the one way a root test freezes a wrapper so that it reads stale. It was made ungated at 2026-11-05-linux-only-code-and-ci (`decision/root-tests-freeze-wrappers-with-one-guard`). The view's stale-row cases (T4, booted fake-agent wrappers) must use this guard rather than a new one.

@history_item n=32 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (Occupied Resources) — moved the root wait bound `WITHIN` to 12 s. Added `decision/test-sockets-under-short-tmp-root`: test channel sockets bind under a short `/tmp` root, never under `TMPDIR`. The view's tests with booted wrappers and planted instance dirs bind their sockets under that root and wait on that bound.

@history_item n=33 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS target: Linux only) — made viola Linux only, with the 0.2.0 window features on Hyprland and CI on `ubuntu-latest` against the fake agent (`decision/ci-ubuntu-fake-agent`, `decision/ci-against-fake-agent`). The view's code half is proven on CI under the fake agent. The tiled quarter-screen half stays a live reading on the operator's host.

@history_item n=34 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — added `name/crate/viola-hyprland` and `decision/hyprland-seam-crate`: viola-hyprland is the one compositor seam, with a closed request set. Any compositor call the view's tiled clause might want goes only through that seam. This chunk dispatches nothing to the compositor.
