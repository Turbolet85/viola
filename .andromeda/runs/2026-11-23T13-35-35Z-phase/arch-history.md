@history_item n=1 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Standard Contracts: 6 rows) — added `decision/hyprctl-direct-spawn` (hyprctl spawned directly from a caller-resolved path, no shell, no PATH lookup, closed argv; a dispatch only the measured Lua `hl.dsp.focus` form whose only inserted value is a closed window address or i32 workspace id) and the five closed `contract/hyprland-seam-errors/*` words (unsupported · unreachable · oversize · malformed · refused, fixed messages interpolating nothing). Why it matters: the new `monitors` query and the move-to-workspace dispatch must fit this closed-argv rule; the row names `hl.dsp.focus` as the only accepted dispatch form, so a `hl.dsp.window.move_to_workspace` dispatch widens it and needs the row amended (a fork per the scope's boundaries); no new error word may be added without a fork.

@history_item n=2 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Infrastructure Patterns: `+ decision/hyprland-seam-crate`) — viola-hyprland is the one compositor seam: a closed request set, sync and tokio-free, depending on viola-core alone among viola crates, guarded by the cargo tree check and the per-sync-crate tokio ban. Why it matters: the chunk extends that closed request set by two requests and must keep the crate's dependency direction and guard green.

@history_item n=3 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Infrastructure Patterns: `~ decision/root-bin-consumes-normalised-events`) — dropped «depends on all members» because viola-hyprland is a root dev-dependency only while no verb calls the seam. Why it matters: the scope keeps the seam with no product caller in this chunk; adding one would make the row and this reasoning move.

@history_item n=4 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Occupied Resources: `+ name/crate/viola-hyprland`, `+ name/binary/viola-hypr-tap`) — registered the seam crate and the test-only tap binary. Why it matters: the chunk adds a `monitors` record and an R1 window verb to `viola-hypr-tap`; the tap stays a test-only binary under the never-released rule.

@history_item n=5 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Conventions: `~ decision/unparseable-external-values-unknown`) — added the except that a hyprctl reply the seam cannot parse is the seam's `malformed` error, not `"unknown"`. Why it matters: the new `monitors` parse must answer an unparseable reply with `malformed`, per this exception.

@history_item n=6 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** (Conventions) — extended the same `decision/unparseable-external-values-unknown` except with a second clause (a StopFailure word outside the known set reads `api-error`). Why it matters: the except now holds two clauses; any wording change for the seam's parse must keep both.

@history_item n=7 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (bounded readers pattern) — introduced the `decision/bounded-readers` rule held by `tests/contract_bounded_readers.rs`: a `BOUNDARIES` table, one `Boundary {` block per bounded read naming its witness test, checked by `boundaries_each_name_a_bounded_read_that_stands`, and the census `product_readers_are_each_bounded_or_named`. Why it matters: the chunk's drain-cap carry re-points the `hyprctl reply` Boundary row from a parse test to a witness that drives the read's own stdout drain past `MAX_FRAME`; a new reader for `monitors` must stand bounded in its statement or the census goes red.

@history_item n=8 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (serde_json unbounded_depth ban) — banned serde_json's `unbounded_depth` in `deny.toml` with a probe row `serde_json-unbounded-depth`, so every external parse keeps the default recursion limit. Why it matters: the `monitors` reply parse is a parse of external bytes bound by `decision/no-unbounded-json-depth` / `decision/serde-json-recursion-limit`.

@history_item n=9 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** ([MCP] stdin line cap) — a line past `MAX_FRAME` is refused whole, never truncated or skipped. Why it matters: the precedent for the seam's `oversize` (reply refused whole before the exit is read) and for the drain-cap witness the chunk adds.

@history_item n=10 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — set the frame rule: one line ≤ `MAX_FRAME` (`\n` counted), longer answered `-32600` and closed (`contract/wrapper-channel-frames/max-frame`). Why it matters: `MAX_FRAME` is the cap the hyprctl read drains to and the bound the new drain witness must cross.

@history_item n=11 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the pattern and its contract binary) — added `decision/sanitised-error-surfaces` held by `tests/contract_error_bodies.rs` and the guard `thiserror_messages_interpolate_nothing`. Why it matters: `HyprlandError` messages stay fixed and interpolate nothing; reply text never reaches an error body, so the new parse drops reply text save the closed keep-list.

@history_item n=12 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the enum census) — `Refusal` folded, `CoreError` off the list, the `thiserror::Error` derive census stated by a grep over `src crates`. Why it matters: one error enum per crate; extending `HyprlandError` keeps one enum in viola-hyprland, and a second enum there would be a divergence the census counts.

@history_item n=13 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the one-enum divergences ratified as named exceptions) — ratified `ServeError`, `StampsError`, `PinError`, `StampsReadError`, `Refused` beside `PtyError`; the derive census reads the product sources outside `crates/viola-e2e`. Why it matters: these are the exceptions in `decision/thiserror-one-enum-per-crate` and siblings; the tap's crate is outside the count, the seam crate is inside it.

@history_item n=14 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the recorded live MCP exchange: the tap) — added `viola-mcp-tap` with `record` (direct spawn, no shell, no PATH lookup, each line through `take(MAX_FRAME)`, capture 0600) and `fixture` (through `ledger::scrub` then `ledger::screen_fault`, refusing with the file name and one closed code `home-path` · `absolute-path` · `username` · `email`, never the content), a fixture class with its own schema replayed by a test; a live crossing ratified by the founder. Why it matters: the direct precedent for `viola-hypr-tap` recording `monitors` and the R1 move, for scrubbing and refusing an unclean recording, and for live readings needing the founder's word.

@history_item n=15 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (registry rows) — the `--record` refusal names the file and the closed code, never the content. Why it matters: the origin of `contract/unclean-fixture-refusal/message`, which binds any refusal of a recorded hyprland fixture.

@history_item n=16 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** (Occupied Resources) — `~ contract/unclean-fixture-refusal/absolute-path`: fires only on a home, drive-letter or user-folder path, not any absolute path. Why it matters: a recorded `monitors` or window reply may hold runtime paths or names; the refusal's real reach is this narrower shape.

@history_item n=17 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** (Occupied Resources) — added the except on `decision/recorded-fixtures-scrubbed` (the rig-only `--record-first-screens`). Why it matters: the row the tap's fixture writes rest on; its only exception is that one, so the hyprland fixtures stay scrubbed and hygiene-walked, refused whole when unclean.

@history_item n=18 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (Occupied Resources: the fake agent's agents mode) — the fake agent answers `agents --json` from files beside its own executable (a mode file `recorded` · `oversize` · `malformed`, planted rows), no option and no env added: the founder's own answer, no third fake-agent env seam. Why it matters: the precedent for the fake agent standing as hyprctl replaying by exact argv in its modes; a new replay case for `monitors` and the move must not add an env seam.

@history_item n=19 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (the second fake-agent test seam) — `FAKE_AGENT_HOOK_PANIC` joined `FAKE_AGENT_PUMP_DELAY_MS` as the two ratified fake-agent seams, read only under `cfg(feature = "fake-agent")`, configuring nothing; a boundary widening ratified live. Why it matters: `decision/fake-agent-test-seams` and `decision/test-seams-not-configuration` cap the seam set at two; any new hyprctl replay knob must not be a third env seam without a ruling.

@history_item n=20 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the test-only child marker) — `MCP_SERVE_TEST_CHILD` registered as a test-private self-spawn marker outside the `VIOLA_*` namespace. Why it matters: the naming rule in `decision/harness-env-var-prefix` / `decision/viola-env-namespace-product-only` for any marker the seam's tests or the tap need.

@history_item n=21 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (Occupied Resources) — added `viola-live-rig` to `decision/test-binaries-never-released` and `AGENT_RUN_RIG` to the harness variables viola never reads. Why it matters: the row's binary list and the harness env rule the tap's R1 window verb must stay inside; a new test-only binary or env name gets registered the same way.

@history_item n=22 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — the `test-support` feature enabled only by root dev-dependencies, and `release-check.sh` refusing a release artifact carrying `test-support` or `fake-agent`. Why it matters: the guard behind `decision/test-support-never-in-product`, `decision/release-build-viola-only` and `decision/release-carries-viola-only`; seam test helpers or fake-agent replay code must not reach a release build.

@history_item n=23 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — the tokio ban is `deny-sync.toml` run once per crate in `scripts/sync-crates.txt` as the sole root. Why it matters: the per-sync-crate tokio ban that guards `decision/hyprland-seam-crate`; the seam's new read and its deadline must stay on std threads.

@history_item n=24 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the Tokio client and viola-channel's tree-form ban) — Tokio enabled only by `viola-mcp` among product crates, the channel's no-feature graph proven by a tree-form check. Why it matters: `decision/hybrid-concurrency` — the seam's timed read (`HYPRCTL_DEADLINE`) is built with std threads, never Tokio.

@history_item n=25 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (Occupied Resources) — `decision/root-waits-share-one-bound` moved from 7 s to 12 s. Why it matters: a root test that waits on a child (the fake agent standing as hyprctl, the deadline test) shares that one 12 s `WITHIN` bound; the read deadline settled at 5 s sits inside it.

@history_item n=26 marker=2026-09-29-h2-conpty-resize-probe
**2026-09-29-h2-conpty-resize-probe** (the viola-pty-watch report) — real-process test rigs write report files outside the viola home, kept on a panic or kill and removed on a pass. Why it matters: the origin of `decision/test-evidence-survives-kill`; a real-process tap or replay test streams its evidence the same way.

@history_item n=27 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (Established Decisions) — `decision/dialog-hold` and `decision/dialog-hold-bound` moved from `provisional` to `standing` once measured live and ruled. Why it matters: the precedent for settling a PROVISIONAL time constant; `HYPRCTL_DEADLINE` (5 s, PROVISIONAL) is settled on the founder's answer F8 with a test that times a read against it.

@history_item n=28 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS target: Linux only) — Linux only, the window features on Hyprland, no design choice depending on tmux or a shell, Windows and macOS code out of the tree. Why it matters: binds `decision/linux-only`, `decision/no-tmux-or-shell-dependency` and `decision/window-features-target-hyprland`; the seam's new requests are Hyprland-only with no other-OS gate.

@history_item n=29 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS-gate census faults a gate naming another OS) — nothing shells out through `sh` or `bash`; every OS-gated test names its reason in `tests/contract_os_gated_tests.rs`, a gate naming another OS a fault. Why it matters: `decision/no-shell-out` for the hyprctl spawn and the tap's foot window, and the census any new gated seam test must satisfy.

@history_item n=30 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (child program resolution) — `resolve_program`: a given path as given, a bare name searched on `PATH`. Why it matters: `decision/child-program-resolution` is the product's own resolution; the seam takes a caller-resolved hyprctl path and does no PATH lookup itself, so the tap and its foot spawn resolve before handing a path.

@history_item n=31 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Standard Contracts) — emptied `decision/no-shell-out`'s except (the statusline `/bin/sh -c` shell-out removed), so viola runs no shell at all. Why it matters: the rule has no exception left; hyprctl, a dispatch and the tap's foot window must be direct spawns.

@history_item n=32 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Established Decisions) — `~ decision/tolerant-external-payloads`: hook JSON and `claude agents --json` parsed tolerantly, agents rows keeping the raw value for the fixture recorder. Why it matters: the tolerant-parse row the `monitors` reply parse sits beside; the seam's own parse instead keeps only a closed keep-list and answers an unparseable reply `malformed`.

@history_item n=33 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0) — no web or graphical frontend; the view beyond `viola list` is the keyboard-driven terminal fleet console tiled on Hyprland, owed by the console entries. Why it matters: the absences `absence/graphical-frontend` and `absence/web-or-graphical-frontend`, and why `viola open` (its verb, flags, window kind and R2) belongs to a later chunk, not this one.
