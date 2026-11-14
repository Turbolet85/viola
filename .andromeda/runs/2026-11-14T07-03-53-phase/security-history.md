@history_item n=1 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** (trust boundary, CI jobs, deny.toml additions) — placed the tokio ban in `deny-sync.toml` per sync crate as sole root and made `scripts/deny-probes.sh` prove every ban live. If the seam lands as a new workspace crate it must take its sync/Tokio-free standing in this gate (rows decision/sync-crates-tokio-free, decision/no-tokio-in-sync-crates).

@history_item n=2 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** (fuzz lockfile exemption) — put `fuzz/` outside `cargo deny` as a ratified test-only exemption, both lockfiles committed. This is the `except` of decision/cargo-deny-gate and decision/exact-dependency-pins; a fuzz target for the Hyprland reply parser (decision/parser-fuzz-scope) lands under that exemption, never the root graph.

@history_item n=3 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — wired the fuzz lockfile's own advisory/source audit and put `scripts/release-check.sh` (refuses any test-only binary) in the CI job list. A fake-compositor binary or test-only feature for the replay must stay out of the release build (decision/release-build-no-test-binaries).

@history_item n=4 marker=2026-09-25-security-prerequisites
**2026-09-25-security-prerequisites** (0BSD per-crate exceptions) — the licence `allow` list stays closed and 0BSD is admitted only per crate, each further exception needing its own Decisions Log entry (decision/licence-allow-list). A Hyprland client crate pulled in by the seam is bound by this.

@history_item n=5 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** (`FAKE_AGENT_PUMP_DELAY_MS` carve-out) — the first env-read admission: a `fake-agent`-only seam, compiled out of release, needing its own Decisions Log entry and recorded as a boundary widening. This is the precedent for the `HYPRLAND_INSTANCE_SIGNATURE` read the scope flags (decision/new-env-read-needs-log-entry, decision/test-seams-fake-agent-only, decision/env-not-config-channel).

@history_item n=6 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — two entries. (a) Listener hardening: the socket is chmod 0600 after the bind, `ListenerOptionsExt::mode` is banned, and the socket mode is never the only control (decision/socket-mode-not-sole-control). (b) Founder ratification of three standing widenings, setting the standing rule that a boundary widening halts the wrap for a live answer. A new env read or a new socket crossing for the Hyprland seam falls under (b).

@history_item n=7 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — recorded release-check refusing `test-support` / `fake-agent` artifacts (probe 5/5). A fake compositor gated behind a test-only feature has to stay inside that refusal (decision/release-build-without-test-features, decision/test-seams-compiled-out).

@history_item n=8 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** (test seam `FAKE_AGENT_HOOK_PANIC` and G2's exact-path exemption) — the second test seam, plus the rule that another seam or another G2 exemption needs its own entry (decision/g2-single-panic-exemption, decision/g2-zero-panics-single-exemption, decision/new-env-read-needs-log-entry). The seam's tests must add no panic exemption, and the seam must add no env seam without an entry.

@history_item n=9 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (MAX_FRAME consumers extended, verify's CI split) — extended the `MAX_FRAME` consumer list to child `--version` reads (drained through the cap, killed at 5 s), and split CI from local: the real CLI runs only locally, and in CI verify runs only against the fake. This is the precedent for the seam's capped reply reads joining the consumer list (decision/max-frame-shared-cap) and for the fake compositor replaying in CI while live recording stays local (decision/real-cli-local-only, decision/ci-no-secrets-no-real-claude). The fixture scrub of the same chunk's capture-arm entry is the scrub that live Hyprland recordings follow.

@history_item n=10 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — removed the CI and pre-push mutation legs, so mutation testing belongs to the epoch-boundary audit (decision/mutation-testing-at-audit). This matches the scope's "no mutation run" boundary.

@history_item n=11 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** (registry migration) — moved release-check's judging of its own artifact records (`--probe` 5/5) and G2's one exact-path exemption into the body as in-force rules (decision/release-build-without-test-features, decision/g2-single-panic-exemption). This governs any new test-only feature or panic path the seam's tests add.

@history_item n=12 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's interactive children are direct spawns under the R8 strip with output capped at `MAX_FRAME` (decision/verify-children-direct-spawn). The recorded-fixture refusal became `a recorded fixture is not clean: <file> <code>` with the closed codes home-path · absolute-path · username · email (contract/fixture-refusal-*). Hyprland fixtures recorded live on the founder's host must pass this same scrub and refusal.

@history_item n=13 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — two scratch dev-host measurements across a boundary, each ratified live. Standing rule: answers are spent, and another probe needs its own showing. This is the trap for L1–L3: each live compositor step needs its own per-chunk word.

@history_item n=14 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the agent typed keys with `wtype` on a compositor the chunk started itself, nested on the locked desktop. The desktop lock was untouched and read locked before each live step and after the end, and this was ratified by the founder. This is the direct precedent for L2/L3 compositor dispatches (`hyprctl locked` read first, never unlock).

@history_item n=15 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (statusline source and shell-out) — made `/bin/sh -c` in `hook statusline` the only shell-out, never to be joined by a second (decision/single-shell-out, decision/statusline-only-shell-out). A `hyprctl` transport must be a direct spawn, never through a shell.

@history_item n=16 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — the loopback GUI left 0.1.0, leaving no HTTP surface. The listener ban stands (decision/new-listener-needs-log-entry). The seam must open no listener; it is a client of the compositor only.

@history_item n=17 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — the `claude agents --json` child is a direct spawn under the R8 strip, stdout through `take(MAX_FRAME + 1)`, killed at 5 s, parsed tolerantly into rows or the closed `AgentsFailure` (oversize · malformed), and failing open, with probe log pairs holding no row text, id, cwd or path. This is the nearest template for a `hyprctl -j` child and its codes-only outcomes (decision/claude-agents-read-fail-open, decision/child-output-bounded, decision/child-output-tolerant-unknown).

@history_item n=18 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — admitted `NO_COLOR` / `TERM` / `COLORTERM` as a ratified widening: compared only, never echoed, switching off no control, with NEVER another environment read without its own Decisions Log entry (decision/no-stored-secrets except, decision/controls-not-switchable). This is the closest precedent for admitting `HYPRLAND_INSTANCE_SIGNATURE`.

@history_item n=19 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — two entries. (a) The mcp process spawning `claude agents --json`, a new process spawning a child, was recorded as a widening ratified by the founder (decision/claude-agents-resolved-from-path). (b) The tokio ban's tree form: `viola-channel`'s Tokio client sits behind a feature, checked by `scripts/channel-tokio-free.sh` (decision/sync-crates-tokio-free). Both bear on where the seam lives and who may call it.

@history_item n=20 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — widened R1's upstream-text list (plan, question and option text, tool input) as content that is never a command (decision/upstream-text-is-never-a-command). Window titles and classes in Hyprland replies join that class and reach no log, error or refusal.

@history_item n=21 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (scratch footer reading) — one scratch start whose screen rows went into the repository, scrubbed, marked as "no product path and no standing permission: another reading needs its own answer". This is the precedent for recording Hyprland replies live into fixtures.

@history_item n=22 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — the live rig passes dialogs by keys into its own pty only, "never a desktop key", ratified per fork. This bounds what a live Hyprland dispatch step may do on the founder's desktop.

@history_item n=23 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — `Mcp.` exchanges were recorded by a test-only tap: lines read through `take(MAX_FRAME)`, 0600 private capture never committed, then scrubbed and refused through `ledger::screen_fault` with the closed codes, all recorded as a ratified widening. This is the template for a recorder of live compositor replies feeding the fake (decision/fixture-scrub-and-refusal).

@history_item n=24 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (served link/unlink) — this entry was the Security Decisions Log record that the Universal ban requires for a new channel method. It is the precedent for the log entry a new crossing or env read needs (decision/new-listener-needs-log-entry, decision/new-env-read-needs-log-entry).

@history_item n=25 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — four entries. (a) serde_json `unbounded_depth` was banned in `deny.toml` with a deny-probes row (decision/serde-json-unbounded-depth-banned). (b) The deny-probes verdict went to 14/14 with its counting rule named, so a new crate ban moves the count. (c) The `viola mcp` stdin cap refuses an over-long line whole, never truncated: the precedent for "refused past MAX_FRAME". (d) The bounded-reader census `product_readers_are_each_bounded_or_named` requires each new product reader, such as the seam's reply read, to be bounded in its statement or named (decision/bounded-readers-census).

@history_item n=26 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (a) clients screen a peer's reply to closed values via `sanitised_fault` / `is_code_word` (decision/client-reads-closed-values-only); (b) the error-body census `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing` (decision/every-error-writer-named, decision/error-messages-interpolate-nothing); (c) the product error enums counted by census as nine. A new seam error enum or writer must join these censuses with fixed messages.

@history_item n=27 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (a) built `check_socket_dir` / `check_endpoint_dir` with one `symlink_metadata` read that never chmods (decision/socket-dir-check-never-chmods, decision/one-socket-dir-check-for-bind-and-clients); (b) set the client seam order: directory check, connect, peer euid, then server verification (decision/client-seam-check-order); (c) banned any seam to drive a foreign euid or owner (decision/no-seam-for-foreign-peer); (d) placed the socket in a per-user 0700 dir, never the /tmp root. If the seam talks to Hyprland's request socket under `$XDG_RUNTIME_DIR/hypr/`, it needs these checks or a recorded reason, as the scope states.

@history_item n=28 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — made viola Linux-only and settled several rules the seam inherits. CI runs one `ubuntu-latest` leg. The endpoint is a Unix socket in `$XDG_RUNTIME_DIR/viola/` (the env read named in decision/no-stored-secrets' except). Child program resolution is a given path or a bare name on PATH, resolved to an absolute path (decision/run-resolves-absolute-program). Deny-sync runs one target triple, a single 0BSD exception stands, and a stale licence exception is dropped (decision/no-stale-license-exceptions). The fixture scrub matches names exactly.

@history_item n=29 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0) — 0.2.0 has no web or graphical surface and no Windows or macOS target, and the enforcement mechanism is enforced on Linux, the one target. This frames the Hyprland-only, Linux-only window seam as a non-listener local client.

@history_item n=30 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master into rows, each verified against the code. Every picked row's handle dates from this conversion, so a row the seam contradicts is amended by handle (`~` / `+` / `−`).

@history_item n=31 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (Dependency Security: 5 rows) — changed decision/ci-cargo-deny-gate to fail on any advisory not on its reasoned ignore list. It added decision/rmcp-advisory-ignores and decision/rustix-advisory-floor, and retired the no-ignored-advisories row. A new dependency for the seam must clear `cargo deny` by a floor bump, not by widening the ignore list without its own ruling.
