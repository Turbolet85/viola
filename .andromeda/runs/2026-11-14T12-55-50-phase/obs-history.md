@history_item n=1 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (6. Log Coverage: 5 rows) `~ contract/parse-rejected-detail/oversize` gains «hyprctl-json past MAX_FRAME», `+ contract/parse-rejected/hyprctl-json`, plus `hyprctl-json` in the parse-rejected parser list and `hyprctl-probe` in the subject enum. These are the rows this chunk's exact-frame tests pin: one byte past `MAX_FRAME` is `oversize` (one parse-rejected line, codes only), and exactly `MAX_FRAME` must not take that path. The tests must keep the line's shape unchanged.

@history_item n=2 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (1. Obs Scope Summary: 3 rows) `~ decision/per-parser-parse-failure-counts` adds «the hyprctl replies» to the counting parsers, and `~ decision/parse-failures-surface-as-unknown` gains its except: a hyprctl reply that fails its parse reads the closed error `oversize` or `malformed` with one parse-rejected line, never `unknown`. This is why the `MAX_FRAME + 1` case must assert `Err(HyprlandError::Oversize)` and not some unknown or fallback value.

@history_item n=3 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (1. Obs Scope Summary: 2 rows) `~ decision/parse-failures-unknown-counter` gets the same hyprctl except (closed `oversize`/`malformed`, never `unknown`). `~ decision/os-apis-at-call-sites` names the hyprctl spawn as instrumented at viola's call site, so the read through the fake `hyprctl` goes through that instrumented spawn.

@history_item n=4 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (2. Telemetry Strategy: 1 row) `~ decision/span-naming` adds the `hyprland` area for the manual `skip_all` span `hyprland.read`. The bounded read the chunk tests (`read_within`, `crates/viola-hyprland/src/read.rs`) runs under it. Test-only work must not rename or move that span.

@history_item n=5 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the `viola mcp` stdin line cap) Precedent for MAX_FRAME boundary semantics: a line of at most `MAX_FRAME` bytes passes whole, and a longer one is refused whole, never truncated or skipped. It has one `parse-rejected{…, detail:"oversize"}` record and no byte of the input in any line. This is the same inclusive bound (`> MAX_FRAME` refuses) that the Hyprland exact-frame tests pin.

@history_item n=6 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (D-05 `parse-rejected` gains `mcp-stdin`) Precedent for extending the oversize register: a new closed `parser` word for a `MAX_FRAME` cut is a Decisions Log record, with `oversize` as its one detail. hyprctl-json followed this pattern in the seam chunk. This chunk adds no parser word and must not.

@history_item n=7 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — `parse-rejected` `oversize` became a `claude-agents-json` detail (a child answer past `MAX_FRAME`). This is the nearest precedent for a child-process reply bounded at `MAX_FRAME` (as the hyprctl replies are), and it is part of the `contract/parse-rejected-detail/oversize` row's history.

@history_item n=8 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — `oversize` was extended to `vt100-feed` (one line per poisoning episode). That clause still stands in the `contract/parse-rejected-detail/oversize` row this chunk relies on, and per-parser oversize semantics differ (episode vs per-reply).

@history_item n=9 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — Mutation testing left CI for the epoch-boundary `/andromeda-code-audit` (`run --mutants`), and a surviving mutant in obs code is a failure at that run (§10). This explains why the two `>` → `>=` survivors come from the Epoch 1 audit and why this chunk only pins them by tests with remove-the-guard controls and runs no mutation itself.

@history_item n=10 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** — §10's «surviving cargo-mutants mutant in obs code» reads the harness document's `survived`, and red is a missed mutant not host-excluded. This defines how the next audit will read the two `viola-hyprland` survivors this chunk closes (96.00 %, 2 missed).

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the OS target, Linux only) Created what is now `decision/linux-only-os-target`: core on Linux, CI on `ubuntu-latest` alone. A companion entry (one CI leg, two workflows) dropped the `windows-mutants.yml` leg from the Mutation row. The exact-frame tests and their fake `hyprctl` need no other-OS gate (the OS-gated census faults a `windows`/`macos` predicate).

@history_item n=12 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — Established that `mutants.out/` is never uploaded (the root of `decision/mutants-out-never-uploaded` and `decision/local-run-outputs-never-uploaded`). This is relevant if the chunk's `gate.py control` runs leave mutation-style outputs: nothing they write may reach an upload.

@history_item n=13 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — `target/run-archive/<n>/` (JUnit and `outcomes.json` with absolute argv paths) was named never-uploaded, alongside `mutants.out/`. This is the stated «why» of the picked `decision/mutants-out-never-uploaded` row.

@history_item n=14 marker=2026-09-26-ci-chunk-base-and-union-verdict
**2026-09-26-ci-chunk-base-and-union-verdict** — `target/agent-run/chunk.diff` was excluded from both the secret scan and the `harness-<os>` upload, because it is repository source text that trips Critical-class patterns. This is the origin of the picked `decision/mutation-diff-never-uploaded` row.

@history_item n=15 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** — (pre-push document and mutation scratch wording) On the Linux dev host `mutants.out/` stays at the repository root and the pre-push document carries no absolute path. This governs where any local control output lands and is relevant to `decision/local-run-outputs-never-uploaded`.

@history_item n=16 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — The raw-tracing ban moved to a fail-closed grep in `scripts/lint-probes.sh`, proven both ways, and a probe failure is a build failure. This is a precedent for `decision/ci-gates-fail-closed`: guard tests added here must be able to read red (their remove-the-guard control) and never pass vacuously.

@history_item n=17 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — (G2's exact-path seam exemption) G2 runs as `scripts/g2-zero-panics.sh` with `--probe` first. This is the guard named by `decision/ci-gates-fail-closed` and the 0-panic-lines error budget behind `decision/nonzero-count-blocks-chunk`, which this chunk's tests must keep at 0.

@history_item n=18 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — G2's start point became `target/e2e-home/` with a trailing slash, because over a linked scope the old form read the fail-closed `empty scope`. Trap: on this Linux host `find` does not descend a symlinked start point without the slash. This matters to `decision/ci-gates-fail-closed` if the chunk's tests create homes.

@history_item n=19 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — `PtyError`'s fixed `Display` is hand-written because `viola-pty` takes no thiserror. This is the source of the except on `decision/typed-errors-fixed-display`. `HyprlandError` (whose `Oversize` the tests assert) falls under the same fixed-message rule.

@history_item n=20 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the nine thiserror derives, CoreError retired) A census of `thiserror::Error` derives with fixed messages. With the error-body census entry (`thiserror_messages_interpolate_nothing`, `assert_error_body_clean`), it backs `decision/typed-errors-fixed-display`, `decision/fixed-error-messages` and `decision/stderr-fixed-strings`. The chunk is test-only and must not move an error's Display.

@history_item n=21 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (channel-response's detail closed to three words) Closed the `-32602` detail to kebab-case words naming no param or value. It is guarded by `answer_invalid_params_names_its_cause_and_no_param`, the guard of the picked `decision/closed-protocol-codes` row. `oversize`/`malformed` on the hyprctl line are such closed codes.

@history_item n=22 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the thiserror derive census on the product sources) The census was re-measured at 9 over `src` and `crates` outside `crates/viola-e2e`. Count rules are re-measured, never copied. The Hyprland seam's error type may have moved the count, which is a P3 measurement if this chunk restates it.

@history_item n=23 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — The master became rows (decision/contract/absence), each verified against the code. Every picked row's handle, including the twin-section pairs (`1-obs-scope-summary`/`obs-scope-summary`), dates from this migration. Later edits name rows by handle (`~`/`+`/`−`), and this chunk's amendments, if any, must do the same.
