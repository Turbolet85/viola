@history_item n=1 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (Input Validation, 3 rows) added `+ decision/hyprctl-read-deadline` (5 s deadline, null stdin, named env removed, kill and reap, deadline or non-zero exit reads unreachable) as provisional, "applied ahead of its settlement against a measured need"; widened `~ decision/parser-fuzz-scope` to the hyprctl reply parsers; gave `~ decision/shared-validators-in-core` its except for the seam's window Address validator. This chunk settles that provisional deadline at 5 s and must time a read against it. The new `monitors` parse falls under the fuzz and property scope. The move-to-workspace argv may take only a validated Address, which stays in the seam crate.

@history_item n=2 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (Threat Model Summary, 2 rows) `~ decision/child-output-tolerant` and `~ decision/child-output-tolerant-unknown` changed from "a failure becomes unknown" to "unknown or a closed failure word", because the hyprctl read maps each failure to the seam's closed words (oversize, malformed, unreachable). A new request or dispatch reuses those closed words. A new failure word is a fork for the operator (scope: a new closed word).

@history_item n=3 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (Data Protection) added `+ decision/hyprland-reply-text-dropped`: reply types hold no text field, and titles, classes and every other reply string are dropped at the parse and reach no log line or error (guard `contract_hyprland_canary_reaches_no_line_and_no_error`). The `monitors` parse must keep only what its closed keep-list names and drop the rest. The canary witness should cover the new request.

@history_item n=4 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (Data Protection, recorder) added `+ decision/hyprland-recorder-string-allowlist`: `viola-hypr-tap` keeps a reply string only under a closed per-request key allowlist, puts a placeholder everywhere else, keeps a non-JSON reply only as ok, then runs the shared scrub and refuses an unclean set whole. The founder's own P4 answer ratified it, the P5 fold sits inside it, and the operator relayed overseer1's «Подтвердить обе». The `monitors` record and the R1 move record each need their own closed key list. A widened key list goes back to that ratified boundary.

@history_item n=5 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (bounded-reader census) created `tests/contract_bounded_readers.rs`: `BOUNDARIES` names each external boundary's bounded read and its witness, and `product_readers_are_each_bounded_or_named` requires every product reader to be bounded in its statement or named in `EXCEPTIONS` (own-executable reads named as no external input). This is the register the drain-cap carry extends: the `hyprctl reply` Boundary row's witness moves from a parse test to one that drives the read's own stdout drain past `MAX_FRAME`.

@history_item n=6 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (MCP stdin, serde_json, deny-probes) capped each `viola mcp` stdin line at `MAX_FRAME`, with an over-long line refused whole and ending the session, never truncated or dispatched in part (`decision/mcp-frame-cap`). Banned serde_json `unbounded_depth` in `deny.toml`, proven by a `deny-probes.sh` row, and stated the probe count with its counting rule (14/14). These are the precedents for a refuse-whole cap on an external read and for any new reply parse keeping the default depth limit.

@history_item n=7 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (MAX_FRAME consumers) the `--version` child reads drain stdout and stderr through the cap and are killed at 5 s; `MAX_FRAME` joins every new reader. The same entry's `hook --capture` / `verify --record` scrub (home to `~`, user word to `<user>`, the whole recording refused) is the origin of the fixture scrub rows. This is the first precedent for a child read held to a drain cap and a 5 s kill, the shape the hyprctl read takes.

@history_item n=8 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — (Hook stdin) changed the read from `take(MAX_FRAME)` to `take(MAX_FRAME + 1)` so that an over-limit payload is detected and not silently truncated. The witness that drives the hyprctl drain past `MAX_FRAME` must tell exactly-the-cap from one more byte in the same way.

@history_item n=9 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (claude agents --json child) a direct spawn under the R8 strip, stdout through `take(MAX_FRAME + 1)`, killed at 5 s, parsed tolerantly into rows or a closed `AgentsFailure` (`oversize` · `malformed`) with one `parse-rejected` line, and a failed read fails open. This is the sibling child-output precedent behind `decision/child-output-bounded`, the pattern the hyprctl seam mirrors.

@history_item n=10 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — (Repository fixtures) the test-only `viola-mcp-tap` records live exchanges: it spawns directly with no shell, reads each line through `take(MAX_FRAME)`, keeps its raw capture 0600 in a private dir that is never committed, applies the same scrub, and refuses through the closed codes `home-path` · `absolute-path` · `username` · `email`, never the content. This was ratified as a boundary widening by the founder's own answers (a cap of one start). It is the closest precedent for `viola-hypr-tap`'s new `monitors` record and its R1 window recording.

@history_item n=11 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — (Repository fixtures) the refusal changed from a fixed message to `a recorded fixture is not clean: <file> <code>` (`<file> row <n>[ seam] <code>` for a screen), with the closed codes `home-path` · `absolute-path` · `username` · `email`, never the content, on the founder's live ruling. This is the source of `contract/fixture-refusal-line/form` and `contract/fixture-refusal-codes/*`, which the new hyprland fixtures must meet.

@history_item n=12 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** — (Data Protection) reworded `~ decision/fixture-recording-scrubbed`, `~ decision/fixture-scrub-and-refusal` and `~ contract/fixture-refusal-codes/absolute-path` from "any absolute path" to "a home, drive-letter or user-folder path", because the scrub and the hygiene walk were measured to pass a runtime-dir path. Trap: a hyprland reply carrying a runtime path (for example a socket under `$XDG_RUNTIME_DIR`) is not caught by the scrub, so the recorder's allowlist is the real control.

@history_item n=13 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (Data Protection / Anti-Patterns) retired the `workspace.repo` except of `decision/fixture-scrub-and-refusal` and `decision/fixture-hygiene`, so no fixture keeps an owner handle. Removed the statusline reads from `decision/max-frame-shared-cap`'s consumer list. These rows now stand without exceptions that the new fixtures could lean on.

@history_item n=14 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (Data Protection) added `+ decision/viola-format-fixtures-hygiene`, which widens the hygiene walk to the committed viola-format fixtures. Precedent: a new fixture tree (`fixtures/hyprland/<version>/`) must be under the hygiene walk.

@history_item n=15 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — (test seam) `FAKE_AGENT_PUMP_DELAY_MS` was admitted as the first test seam: `fake-agent`-only, capped, compiled out of release builds, configures nothing, disables no control, widens no redaction. The ban says another seam needs its own Decisions Log entry. This is the source of `decision/test-seams-compiled-out` and `decision/test-seams-fake-agent-only`. A new fake-agent hyprctl replay mode or input read must stay inside them or come as its own ruled widening.

@history_item n=16 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — (test seam) `FAKE_AGENT_HOOK_PANIC`, the second seam, landed only after a live founder ratification: a closed value, named in one file, compiled out of release builds. Precedent that adding a seam, or a new env read for the fake agent standing as hyprctl, is a boundary widening that halts for a live answer.

@history_item n=17 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — (founder ratification of standing widenings) set the standing rule that a boundary widening halts the wrap for a live answer, and an earlier delegated direction never ratifies one. This governs any widening this chunk's seam requests or the R1 live reading bring.

@history_item n=18 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — (release-check) recorded release-check's refusal of `test-support` / `fake-agent` artifacts (probe 5/5). This is the precedent behind `decision/release-build-no-test-binaries` and `decision/release-build-without-test-features`, which keep the test-only `viola-hypr-tap` and the fake agent's hyprctl replay out of the release build.

@history_item n=19 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — (CI integration) moved into the body that `scripts/release-check.sh` judges the build's own artifact records and refuses any executable other than `viola` and any `test-support` / `fake-agent` artifact, with `test-support` enabled only by the root package's dev-dependencies. A new test-only binary or feature edge must stay inside that check.

@history_item n=20 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — (supply chain) wired the `fuzz/Cargo.lock` advisory and source audit as the ratified fuzz exemption's own step outside `cargo deny`, and put the release build through `scripts/release-check.sh` in the CI job list. This is the source of `decision/cargo-deny-gate`'s except. A fuzz target over the new parse lands in that audited, exempt workspace.

@history_item n=21 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — (fuzz exemption) `fuzz/` is a separate workspace outside `cargo deny`, a test-only exemption that holds only while it never links into and never joins the root `[workspace]`, with both lockfiles committed and pinned. A fuzz target for the new hyprctl parse must keep that condition.

@history_item n=22 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (tokio ban) the sole-root `deny-sync.toml` tokio ban runs over the crates of `scripts/sync-crates.txt`, with `viola-channel`'s Tokio client covered by `scripts/channel-tokio-free.sh` and its `deny-probes` probe. If `viola-hyprland` or the tap is a sync crate, it must be listed and checked there (`decision/no-tokio-in-sync-crates`, `decision/sync-crates-tokio-free`).

@history_item n=23 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — (deny.toml) placed the tokio ban per sync crate as sole root in `deny-sync.toml` and recorded that `scripts/deny-probes.sh` proves every ban live (the C-building ban included). Any new dependency for the seam is held to those bans.

@history_item n=24 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (licences, deny target) `deny-sync.toml` runs for the one target triple `x86_64-unknown-linux-gnu`, and an exception naming a crate absent from the graph is dropped. The same entry set child resolution to a given path or a bare name on PATH. This is the current form of the sync-crate ban and the child-resolution rule that the hyprctl spawn sits beside.

@history_item n=25 marker=2026-11-17-supply-chain-green-and-booted-tests-end
**2026-11-17-supply-chain-green-and-booted-tests-end** — (Dependency Security) `− decision/rmcp-advisory-ignores → decision/advisory-ignore-list`, plus `+ decision/advisory-ignore-grounds`: the closed ignore list of four ids, admitted only for a disabled feature or for an unmaintained upstream with no safe upgrade, each re-read at each epoch boundary. The founder ratified it on 2026-11-22. A new dependency that brings an advisory cannot widen the list without its own ruling (`decision/cargo-deny-gate`).

@history_item n=26 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (error census) `tests/contract_error_bodies.rs` enumerates every error body with its writer and witness, and `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing` fail on an unnamed writer or an interpolating message. The same entry set the product-enum census at nine `thiserror` derives. A new `HyprlandError` variant or a new error writer for the move dispatch must be named there, with a fixed message (`decision/every-error-writer-named`, `decision/error-messages-interpolate-nothing`).

@history_item n=27 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (thiserror census) the derive census counts product sources only; the test-only harness's derive falls outside it. The five divergences are named exceptions. This is the rule that decides whether a test-only tap's error type changes the count.

@history_item n=28 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — (Authentication & Authorization) `~ decision/explicit-mode-before-write` gained its one except, the trust record writer keeping Claude's config file mode. Every other written file, the tap's fixture files and captures included, must set its mode before any byte, or the write is discarded.

@history_item n=29 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — (write helper) every private write goes through the shared tempfile `persist` helper `viola_state::fs::replace_private`, which sets the mode on the temp file before any byte, then `sync_all` and `persist`. This is the mechanism `decision/explicit-mode-before-write` rests on, and the tap's files should reuse it.

@history_item n=30 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — (live keys on a compositor) an agent typed keys with `wtype` on a compositor the chunk started itself, nested on the locked desktop: one start, a guard read before each key, and the desktop lock read before, during and after. The founder ratified it after the method and price were shown. This is the precedent for the R1 reading, where the tap opens, moves and ends one foot window at the operator's host and moment.

@history_item n=31 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — (scratch reading) one scratch start whose screen rows were scrubbed into `evidence/`, killed and removed, with the standing rule "no product path and no standing permission: another reading needs its own answer". Precedent that R1's one window reading is spent once taken. A reading beyond R1 and R3 is a fork (scope).

@history_item n=32 marker=2026-11-17-first-readings-on-the-standing-rig
**2026-11-17-first-readings-on-the-standing-rig** — (Authentication & Authorization) `~ decision/live-round-rig-keys-own-pty`: the founder's word, given anew for each chunk, binds the live rig's closed argv grant; its stop (Ctrl-C or a no-key group kill) needs no word. Precedent for R1's closed live action set, and for ending only the foot pid the tap itself started.

@history_item n=33 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** — (Secret Management) retired the operator-only uid-0 WSL launch, so that "no other launcher and no uid-0 launch exists". This is the source of `decision/no-uid-0-launch`. The tap's foot window and the hyprctl child launch as the user, never as root.

@history_item n=34 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — (revive --json) refusal documents print closed codes only, and `--list --json` prints only session-shaped ids, never a path or pid. This is the origin of `decision/refusals-hold-no-paths-pids-ids`, which binds any tap or seam refusal: no window address beyond the closed form, no pid, no path.

@history_item n=35 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — (Threat Model Summary) with the GUI gone, the elevations went from three to two; `decision/security-tier` now holds IPC, paste and home-integrity elevations only, and no compositor surface is among them. Seam requests that would add an elevation (for example a dispatch beyond the closed set, or an `exec`) fall outside the Minimal tier.
