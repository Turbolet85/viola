@history_item n=1 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Threat Model Summary: 2 rows) — ~ decision/child-output-tolerant and ~ decision/child-output-tolerant-unknown: a failure becomes "unknown or a closed failure word", because the hyprctl reply read turns each failure into one of the seam's closed words (oversize, malformed, unreachable). This chunk's two exact-frame tests assert `oversize` at `MAX_FRAME + 1` and a non-oversize outcome at exactly `MAX_FRAME` (`Refused` on the dispatch parse). They pin the closed word these rows now promise.

@history_item n=2 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Input Validation: 3 rows) — + decision/hyprctl-read-deadline (null stdin, named env vars removed, a 5 s deadline that kills and reaps, provisional), ~ decision/parser-fuzz-scope (the hyprctl reply parsers join fuzz and property coverage), ~ decision/shared-validators-in-core (the Address validator stays in the seam crate). The bounded read at `crates/viola-hyprland/src/read.rs:77` runs inside this deadline-bound child, driven by the fake `hyprctl`. The new read test must keep the deadline guard (`read_past_the_deadline_kills_and_reaps_its_child`) intact. Any property or fuzz case it touches is in this row's scope.

@history_item n=3 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Data Protection: hyprland-reply-text-dropped) — + decision/hyprland-reply-text-dropped: reply types hold no text field, so titles, classes and every other string are dropped at the parse and reach no log line and no error (guard `contract_hyprland_canary_reaches_no_line_and_no_error`). A `MAX_FRAME`-byte reply built for the new tests must not make an `Oversize` or `Refused` error carry or echo reply bytes.

@history_item n=4 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Data Protection: hyprland-recorder-string-allowlist) — + decision/hyprland-recorder-string-allowlist: the recorder keeps strings only by a closed allowlist (the founder's answer to the P4 recorder fork). The fake `hyprctl` replays recorded replies. A synthetic `MAX_FRAME` or `MAX_FRAME + 1` reply fed to the fake should be generated in the test, not recorded or committed as a fixture outside that allowlist.

@history_item n=5 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — the `claude agents --json` child's stdout is read through `take(MAX_FRAME + 1)`, killed at 5 s, and parsed tolerantly into the closed `AgentsFailure` (`oversize` · `malformed`). This is the nearest precedent for the hyprctl read: drain through `MAX_FRAME + 1`, so the one extra byte is what tells oversize apart from exactly the cap. That is the boundary this chunk pins at `read.rs:77`/`:121`.

@history_item n=6 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — hook stdin moved from `take(MAX_FRAME)` to `take(MAX_FRAME + 1)` "so an over-limit payload is detected". This is the origin of the `+ 1` drain idiom whose `>` comparison this chunk's tests pin (`>` → `>=` is the surviving mutant at both sites).

@history_item n=7 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (`viola mcp`'s stdin capped at `MAX_FRAME` per line) — the precedent for exact-cap semantics: "a line of exactly the cap passes whole; a longer line releases no byte, is refused whole". This chunk asserts the same two-sided boundary (exactly `MAX_FRAME` accepted, one more `oversize`) on the Hyprland seam.

@history_item n=8 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader census) — `tests/contract_bounded_readers.rs` names each external boundary's bounded read and its witness in `BOUNDARIES`. Its census `product_readers_are_each_bounded_or_named` (the guard of decision/bounded-readers-census) requires every product reader to be bounded or named. The new exact-frame tests are natural witnesses for the Hyprland read's bound. If `BOUNDARIES` names a witness for the hyprctl read or `parse_dispatch`, check whether to point it at the new tests. No product reader may change shape.

@history_item n=9 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (`MAX_FRAME` consumers extended, verify's CI split) — child `--version` reads were drained through the cap and killed at 5 s. This entry also set "the real CLI runs only locally; verify runs in CI only against the fake agent" (decision/real-cli-local-only, decision/ci-no-secrets-no-real-claude). Together these are the precedent for this chunk's rule: a bounded child read tested only through a fake, never the real tool, here the fake `hyprctl` with no live desktop step.

@history_item n=10 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the founder's ruling took mutation legs out of CI and pre-push and put them in the epoch-boundary audit (decision/mutation-testing-at-audit). This is why the chunk runs no mutation itself. It must catch the audit's two `>` → `>=` survivors with remove-the-guard controls instead, and leave the next audit to confirm.

@history_item n=11 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** — the one mutation dispatch made outside the epoch-boundary audit needed the founder's own word, for that chunk alone. This is the precedent showing that no mutation run, local or dispatched, may happen inside this chunk without such a word. The operator's directive forbids it here.

@history_item n=12 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the tests holding the directory and peer checks, and no seam to drive them) — the guard decisions are proven over injected values, and remove-the-guard runs are made once without being committed. A new Universal ban says: NEVER add an env, flag or feature seam to force a refusing decision. This chunk is the same pattern. Each guard test's one-off remove-the-guard control mutates the site through `gate.py control`, and is never a committed seam or a product line change.

@history_item n=13 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — the first test seam (`FAKE_AGENT_PUMP_DELAY_MS`): fake-agent-only, compiled out of release builds, configures nothing, disables no control. The Universal ban says another seam or env read needs its own Decisions Log entry. This bears on decision/test-seams-compiled-out and decision/test-seams-fake-agent-only. If driving a `MAX_FRAME`-sized reply through the fake `hyprctl` would need a new env knob or seam, it is a boundary widening, not test-only work.

@history_item n=14 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (test seam `FAKE_AGENT_HOOK_PANIC` and G2's exact-path exemption) — the second seam and G2's single whole-string panic exemption (decision/g2-single-panic-exemption, decision/g2-zero-panics-single-exemption, founder-ratified). Any other G2 exemption needs its own entry. The new tests and their remove-the-guard controls must not add a panic location G2 would need to exempt.

@history_item n=15 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** (founder ratification of three standing boundary widenings) — sets the standing rule that a boundary widening halts the wrap for a live founder answer. The scope is test-only, with no requirement and no capability. Anything found at P3 that would widen (a new seam, a changed bound) must escalate, not land as an `[inferred]` fold.

@history_item n=16 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — release-check refuses `test-support` / `fake-agent` artifacts (probe `5/5`). This is the origin of decision/release-build-without-test-features and decision/release-build-no-test-binaries. The fake `hyprctl` and any test-only helper the new tests use must stay out of the release graph.

@history_item n=17 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** (registry migration, the Decisions Log leaves the body) — moved the in-force items into the body: `scripts/release-check.sh` judging artifact records and refusing test-only features (`--probe` 5/5), and G2's one exact-path exemption with its look-alike `--probe`. These are the guards behind rows the picked set holds (release-build-*, g2-*).

@history_item n=18 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error bodies' census; the product error enums, by census) — `tests/contract_error_bodies.rs` holds `thiserror_messages_interpolate_nothing` (the guard of decision/error-messages-interpolate-nothing) and the `thiserror` derive census, nine at the time. `HyprlandError::Oversize` and `Refused` keep fixed messages with no interpolation (decision/fixed-error-display-messages, decision/sanitised-external-errors). The tests should assert the variant, never message text holding reply bytes.

@history_item n=19 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the error-enum exceptions and the thiserror census) — the derive census counts product sources only, and test-only harness derives sit outside the count. The chunk adds tests only, so the count must not move. A test-only error type belongs outside the product census.

@history_item n=20 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (serde_json's `unbounded_depth` banned in `deny.toml`; the deny-probes verdict at 14/14) — decision/no-serde-unbounded-depth is enforced by a `deny.toml` ban and the probe row `serde_json-unbounded-depth`. A `MAX_FRAME`-sized JSON payload built for `parse_dispatch` must not reach for that feature or any depth override.

@history_item n=21 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Dependency Security: 5 rows) — ~ decision/ci-cargo-deny-gate: the gate fails on any advisory not on its reasoned ignore list. − cargo-deny-gate-clean, − no-ignored-advisories; + rmcp-advisory-ignores, + rustix-advisory-floor. This is the current form of a picked row. A test-only chunk adds no dependency. Any dev-dependency added for the tests would meet this gate and the exact-pin rows.

@history_item n=22 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (cargo-deny advisories green, the ignore list empty) — the prior state of the advisory gate: ignore list empty, RUSTSEC-2026-0441 closed by portable-pty `=0.9.0`. It was later superseded by the three reasoned rmcp ignores (n=21), and explains the history of decision/ci-cargo-deny-gate's ignore-list clause.

@history_item n=23 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the tokio ban's tree form and the rmcp release-graph gate) — the sole-root `deny-sync.toml` tokio ban over `scripts/sync-crates.txt`, plus `viola-channel`'s Tokio client behind its feature, checked by `scripts/channel-tokio-free.sh`. This is the source of decision/no-tokio-in-sync-crates and decision/sync-crates-tokio-free. If `viola-hyprland` is a sync crate, its new tests must not pull tokio, even as a dev-dependency, into its graph.

@history_item n=24 marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
**2026-10-26-tokio-rustsec-2026-0386-lock-bump** (viola-channel's Tokio client pin at tokio 1.53.3) — re-pins the same tokio-ban item: the four sole-root bans pass, `channel-tokio-free: ok`, `deny-probes 13/13`, later 14/14. These are the standing checks any graph change from the tests would meet.

@history_item n=25 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — the `fuzz/` workspace lockfile sits outside `cargo deny` as a ratified test-only exemption. This is the source of the except-clause of decision/cargo-deny-gate and of decision/exact-dependency-pins' fuzz pins. The same entry closed the `mutants.out/` upload CARRY by removal (decision/no-mutants-out-upload). If a hyprctl fuzz target needs a corpus seed for the exact-frame case, it lives under that exemption.

@history_item n=26 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — wired the `fuzz/Cargo.lock` advisory and source audit, and added the per-OS release build through `scripts/release-check.sh` to CI. Together these are the guards of decision/ci-cargo-deny-gate's fuzz step and decision/release-build-no-test-binaries.

@history_item n=27 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — CI runs the artifact canary scan before every test-home upload, and the `mutants.out/` upload was noted as unscanned (its CARRY was closed in n=25). This explains decision/no-mutants-out-upload. The chunk produces no mutation output to upload.

@history_item n=28 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (child program resolution is a given path or a bare name on PATH) — a bare name resolves on `PATH`, and a program not found is `ProgramNotFound`. This bears on decision/child-path-trust: the fake `hyprctl` stands in for the real one by PATH resolution, so the read's exact-frame test must take that fake from the test's own PATH, never the host's real `hyprctl`.
