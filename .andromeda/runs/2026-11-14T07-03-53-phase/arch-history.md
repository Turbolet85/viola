@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — (test-only crate, bins, env vars, paths) Module Boundaries gained "each product crate is created by its first consumer" and the `AGENT_RUN_` prefix for harness-only variables; matters because the seam's placement (a new `viola-<area>` crate vs a root-bin module) is this chunk's inferred P3 item and a new crate is born only by its first consumer (`decision/cargo-workspace-flat-crates`, `decision/crate-naming`).

@history_item n=2 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — the tokio ban became `deny-sync.toml` run once per crate in `scripts/sync-crates.txt` as the sole root (an excluded-graph check false-fails under feature unification); a new seam crate must join that list and its deny/ban checks (`decision/sync-crates-tokio-free`, `decision/no-tokio-in-sync-crates`).

@history_item n=3 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — fixed every role's codes-only log to its home-level role file and content-bearing detail only to the instance detail file, process-log lines carrying no `v`; the seam's closed outcome codes must land in the role file of whichever verb calls it, never reply text (`decision/codes-only-process-logs`, `decision/content-detail-per-instance`, `decision/versioned-records-no-migrations`).

@history_item n=4 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — banned the tracing level macros via `clippy.toml` disallowed-macros and a raw `event!` grep (`scripts/lint-probes.sh`), print bans workspace-wide; the seam logs only through `obs_event!` (`decision/obs-event-only-emitter`).

@history_item n=5 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — added the per-lib/bin `cargo modules orphans --deny` CI gate, the rust code-graph plane covering every member, and `scripts/release-check.sh`; a new crate or module must pass the orphans gate and be indexed, and any fake-compositor binary must stay out of the release artifact (`decision/release-build-viola-only`, `decision/release-carries-viola-only`).

@history_item n=6 marker=2026-09-24-fake-agent-and-test-data-fixtures
**2026-09-24-fake-agent-and-test-data-fixtures** — registered test-side fixture dirs and `schemas/` (fake-script schema) in the registry and tree; precedent for registering the fake compositor's recorded-reply fixture directory and its schema as test-side paths.

@history_item n=7 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — viola-side program resolution (viola resolves the child to an absolute path itself) and "environment variables are not a configuration channel" kept unqualified; precedent if the transport spawns `hyprctl`: resolved by viola, spawned directly (`decision/child-program-resolution`, `decision/no-environment-configuration`).

@history_item n=8 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — `FAKE_AGENT_PUMP_DELAY_MS` ratified as a boundary widening of [Naming], read only under `cfg(feature = "fake-agent")`; any env seam the fake compositor needs is a widening needing ratification, outside `VIOLA_*` (`decision/test-seams-not-configuration`, `decision/env-var-viola-prefix`).

@history_item n=9 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — the endpoint moved into a per-user 0700 socket dir (`$XDG_RUNTIME_DIR/viola/`, fallback `/tmp/viola-<uid>/`), frames one line at most `MAX_FRAME` with `\n` counted, longer refused; the scope names this as the precedent for capping each Hyprland reply and for the 0700/owner checks a Hyprland request socket would need (`decision/socket-dir-owner-only`, `contract/wrapper-channel-frames/max-frame`).

@history_item n=10 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — removed every CI and pre-push mutation leg; mutation runs only through `agent-run run --mutants` at the epoch-boundary audit — matches the chunk's "no mutation run" boundary (`decision/mutation-at-epoch-boundary`).

@history_item n=11 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (the six landed ledger rows, verify's probe, the stamps envelope) a measured-live record: fixtures recorded per CLI version under `fixtures/claude/<v>/`, scrubbed, a `--version` read drained through `MAX_FRAME` and killed at 5 s; the model for the Hyprland "measured live" record (version read with each recording, a fixture dir per version), which must stay apart from the Claude-only ledger (`decision/capability-ledger-gate`, `decision/claude-first-no-adapter-trait`).

@history_item n=12 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — closed clap's `color` route because a dependency's env read breaks the no-env rule like a direct read; any Hyprland client crate or the seam reading `HYPRLAND_INSTANCE_SIGNATURE` / `XDG_RUNTIME_DIR` counts as a new env read (`decision/no-environment-configuration`).

@history_item n=13 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — (registry rows for verify's interactive runs and screen fixtures) `--record`'s unclean-recording refusal names the file and a closed code, never the content, and recorded screens keep signature rows only; the fake compositor's recorded replies must pass the same scrub, refusal and hygiene walk (`decision/recorded-fixtures-scrubbed`, `contract/unclean-fixture-refusal/*`).

@history_item n=14 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — (four crossings ratified by the founder) the user's statusline command through `/bin/sh -c` ratified as the ONLY shell-out in viola; a `hyprctl` transport must be a direct spawn, never a shell (`decision/no-shell-out`, `decision/statusline-only-shell-out`, `decision/no-tmux-or-shell-dependency`).

@history_item n=15 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (Standard Contracts: the agents read) the nearest precedent for querying an external program: resolved via `run::resolve_program`, read through `take(MAX_FRAME + 1)` by `run_capped`, killed at 5 s, one probe start/exit pair logged, parsed by a pure module into rows or a closed failure (`oversize` · `malformed`), fail-open; also (the fake agent's agents mode, the Agents fixture class, the sixth fuzz target) the fake answers from files beside its exe with no new env seam, a fixture class with its own schema, replayed recorded values only, and a fuzz target on the parser — the template for the fake compositor (`decision/bounded-readers`, `decision/tolerant-external-payloads`).

@history_item n=16 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — (the CLI verbs take the VIOLA_DIR step) a new environment read landed as built and recorded NOT ratified, owed the founder's word (ratified later at 2026-10-18-plugin-precedence-live-readings); precedent for the `HYPRLAND_INSTANCE_SIGNATURE` read the scope says needs its own Decisions Log entry (`decision/no-environment-configuration`).

@history_item n=17 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the `NO_COLOR` / `TERM` / `COLORTERM` reads ratified as a boundary widening, with the standing precedent "any further environment read … needs its own ratification"; the instance-signature read is exactly such a further read (`decision/env-var-viola-prefix`, `decision/no-environment-configuration`).

@history_item n=18 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (the Tokio client and viola-channel's tree-form ban) the sync-crate list became four, `viola-channel`'s graph proven tokio-free by `scripts/channel-tokio-free.sh`, rmcp release graph checked; a new seam crate must state its sync/Tokio-free standing and join the list or a tree-form check (`decision/hybrid-concurrency`, `decision/tokio-containment`, `decision/tokio-only-in-mcp-crate`).

@history_item n=19 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — (the recorded live MCP exchange) `viola-mcp-tap` records a live exchange by direct spawn, each line capped, then scrubs and refuses with a closed code, a schema-validated `Mcp.` fixture class hygiene-walked and replayed by a test — the closest precedent for "recorded live, scrubbed, replayed by a fake in CI"; (the test-only child marker) `MCP_SERVE_TEST_CHILD` registered as a test-only marker outside `VIOLA_*` (`decision/test-binaries-never-released`, `decision/viola-env-namespace-product-only`, `decision/harness-env-var-prefix`).

@history_item n=20 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (bounded readers, a cross-cutting pattern held by a census) every product reader of external bytes is bounded in its own statement or named in `EXCEPTIONS`, held by `tests/contract_bounded_readers.rs` (`BOUNDARIES` row + witness per read) — the seam's capped reply read must land as a `Boundary` row or the census goes red; (the viola mcp stdin line cap) a longer line refused whole, never truncated; (the serde_json unbounded_depth ban) parses keep the default recursion limit (`decision/bounded-readers`, `decision/no-unbounded-json-depth`, `decision/serde-json-recursion-limit`).

@history_item n=21 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the sanitised error surfaces pattern and its contract binary) error bodies carry codes and fixed messages only, censused by `tests/contract_error_bodies.rs` with `thiserror_messages_interpolate_nothing`; (`unknown`'s detail a code word) `viola_core::is_code_word` defines a code word; (the enum census) one error enum per crate — the seam's closed outcome codes and any new error enum must fit these (`decision/sanitised-error-surfaces`, `decision/thiserror-messages-interpolate-nothing`, `decision/one-error-enum-per-crate`).

@history_item n=22 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the code-word rule and the exit-cause table) every new code passes `is_code_word` and is a row of `tests/contract_error_bodies.rs`, no path, pid or upstream text; the seam's outcome words (`unsupported`, `unreachable`, `malformed`, …) follow that rule.

@history_item n=23 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the socket-directory check's read) `viola_state::strict::check_socket_dir` / `check_endpoint_dir`: one `symlink_metadata`, absent passes, symlink / non-directory / foreign owner / group-other bit refused, never chmodded — the check a Hyprland request socket under `$XDG_RUNTIME_DIR/hypr/` would reuse or record a reason against; (the peer euid on both sides) same-user peers only via `peer_creds`; (the test-owned socket roots) tests own their `XDG_RUNTIME_DIR` socket root (`decision/socket-dir-strict-check`, `decision/socket-dir-check-verdicts`, `decision/same-user-peers-only`).

@history_item n=24 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (the OS-gated tests census) every OS-gated test names its reason in `GATES` of `tests/contract_os_gated_tests.rs`; a seam test gated on a compositor/platform must carry its row.

@history_item n=25 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the OS target: Linux only) Linux only, window features of 0.2.0 on Hyprland, no tmux/shell dependence; (CI on the one `ubuntu-latest` leg) where no compositor runs, hence the fake; (child program resolution) a given path as given, a bare name on `PATH`; (the wrapper endpoint is a Unix socket only); (the OS-gate census faults a gate naming another OS); (windows-sys out, libc a plain dependency) libc the one OS-binding crate; (cargo-deny: one Linux triple) — all bind the seam's transport and crate (`decision/linux-only`, `decision/window-features-target-hyprland`, `decision/ci-linux-fake-agent`, `decision/libc-only-os-binding`, `decision/cargo-deny-bans`).

@history_item n=26 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the one-enum divergences ratified as named exceptions) `ServeError`, `StampsError`, `PinError`, `StampsReadError`, `Refused` ratified beside `PtyError`, 9 product `thiserror::Error` derives by grep; a new seam crate adds one `<Crate>Error` enum and moves that census (`decision/thiserror-one-enum-per-crate`, `decision/one-error-enum-per-crate`).

@history_item n=27 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — (version census 0.2.0) no web or graphical frontend; the view beyond `viola list` is the keyboard-driven terminal fleet console tiled on Hyprland, owed by «Console scaffold», «Fleet console view», «Console actions» — the later consumers whose acceptance closes the seam's relied-on request set (`decision/terminal-fleet-console`, `absence/graphical-frontend`).

@history_item n=28 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (1062), each verified against the code, guards re-pointed; the picked rows this chunk extends are these rows, and a new crate, env read or transport lands as row edits (`+`/`~` handles), not prose.
