@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — only `viola-e2e` omits `[lints] workspace = true` and carries its own `[lints.clippy]`; the fake agent (a root-package bin) is exempted from the print ban by a crate-level allow. Matters if the rig lands as a test-only binary of `viola-e2e` (or extends the fake agent's trust/modal fidelity): its print and lint posture follows this carve-out, not the workspace ban.

@history_item n=2 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — raw `tracing` level macros banned by path in `clippy.toml`, raw `event!` caught by the fail-closed grep in `scripts/lint-probes.sh`, a lint-probes failure a build failure (rows decision/no-raw-tracing-macros, decision/lint-probes-failure-fails). Any rig code in the workspace must log, if at all, only through `obs_event!` or not at all.

@history_item n=3 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — D-34: every stripped and kept `CLAUDE*` name logged as names only (`env_kept`), values never read (rows decision/r8-strip-logged-as-names-only, contract/claude-child-process-start/env-fields). The rig's isolation (private config dir, credentials copy) must keep values out of every record; only names may surface.

@history_item n=4 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — G2 (`scripts/g2-zero-panics.sh`, `--probe` first) exempts only a panic at exactly `src/cmd/hook/seam.rs:<digits>`; the perf job's synthetic-input upload basis (row decision/zero-panics-gate, decision/detail-upload-synthetic-inputs-only). A rig home or rig-driven test under `target/e2e-home/` falls under G2/G4/secret scan with no new exemption; non-synthetic (live) inputs must drop detail uploads.

@history_item n=5 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — in CI `viola verify` runs only against the fake agent; the real-`claude` verify runs only through the local `run --local-live`, refused under `CI` (rows decision/ci-verify-fake-agent-only, decision/real-cli-verify-never-in-ci). The standing live rig is local-only by the same rule: no live start in CI, its fake-agent checks the only CI part.

@history_item n=6 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** — no home is deleted (harness `cleanup` or a test home's drop through `remove_owned`) before G2, G4, the secret scan and uploads; CI's `AGENT_RUN_KEEP_HOMES=1` keeps every home (rows decision/homes-kept-until-gates-run, decision/test-homes-kept-until-gates). The rig's "end what it starts on every exit path" must remove processes, not CI-kept homes before the gates read them.

@history_item n=7 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — vt100 panics caught per feed/resize on run's feed thread, poisoning the model until a size change, one `parse-rejected{vt100-feed, panicked}` per episode (rows decision/vt100-panics-contained, decision/vt100-feed-panic-contained). If the rig reuses the vt100 `=0.16.2` screen model, this is the product's precedent for containing its panics; the screen serves gate outcomes only, never telemetry.

@history_item n=8 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (F4) — `tests/chaos_feed_panic.rs` keeps its one home outside the scan root (`TestHome::outside_scan()`), the founder's ruling as a named test-data carve-out rather than a second G2 exemption (rows decision/test-homes-under-e2e-home, decision/test-homes-under-scan-root). Trap: a rig scratch root outside every repository is a second such carve-out only by an explicit ruling; otherwise rig-booted test homes must live under `target/e2e-home/`.

@history_item n=9 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — `run.wheel_transition.cause` is `human-input|manual-pause|release`; the wheel product event is the audit record and typed keys are never logged (rows decision/typed-keys-never-logged, decision/wheel-event-is-audit-record). Live starts by typed keys (trust dialog keys) leave only wheel lines, never the keys; the rig's own records must not keep typed key bytes beyond what the founder's keys define.

@history_item n=10 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — `turn-running` covers a running turn (in-memory, marked by any `prompt-submitted`) or another send in flight, no new field, no `send-issued`, client exit 13 (row contract/send-refused-detail/turn-running). The carried hook-latency-window reading (cry-iugxdgc4) measures the gap this state cannot see.

@history_item n=11 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's interactive PTY runs logged as `verify-pty-probe` pairs; CI drives the fake agent with `--screens --turn-stop --trusted-root`; no line holds row text, a prompt or a path. Precedent for a pty-driven CLI run the rig generalises, and the fake agent's screen/trust fidelity the rig is to raise.

@history_item n=12 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — Runs C and D and the `--answers` capture arm; CI's fake agent adds `--dialogs` (rows contract/hook-stdout/capture-arms, decision/capture-arms-uninstrumented). The fake agent's modal-dialog fidelity rides this rig; capture arms stay uninstrumented.

@history_item n=13 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — on the Linux dev host `target/e2e-home` is a link to an owner-only tmpfs dir; a kept home's diagnostics end at reboot and `/tmp` ages them out after ten days. Trap: `find` does not descend a linked start point without the trailing slash. Bears on the rig's scratch root and its process census by path, and on keeping screens as evidence.

@history_item n=14 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — the identity floor (11 names) is always removed whatever the persistent set says; a differing Linux dev-host reading on `claude` 2.1.287 stands beside it (rows decision/identity-floor-always-stripped, decision/claude-env-strip). The rig's private config dir and credentials copy must be built knowing which `CLAUDE*` names the strip removes from the child.

@history_item n=15 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `send-confirmed.duration_ms` runs from the send's arrival and includes the readiness gate's wait (up to `GATE_MAX_WAIT`, 8.5 s on a delivered send), from a live ruling (row contract/send-log-fields/send-confirmed). Live readings taken on the rig must not read that field as the open-to-readback interval.

@history_item n=16 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — `text_bytes` counts the typed text, trailing LF removed, on a live founder ruling. First step of the typed-text rule the carried whitespace-trim reading (cry-vdpx2jqz) extends.

@history_item n=17 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — `empty-text` refusal added, one line per deciding side, never the text; `text_bytes` strips trailing CR and LF (rows contract/send-refused-client/empty-text, decision/send-refused-by-each-deciding-side). Part of the typed-text and validation rules the trim reading touches.

@history_item n=18 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — `text_bytes` is the typed text with CR LF and lone CR as LF, trailing CR/LF removed, read live on 2.1.287 (row contract/send-span-attributes/text-bytes). Precedent for a live reading of how the CLI treats whitespace inside a paste; the rig's trim reading must hold to this normalisation.

@history_item n=19 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a verify stopped by SIGINT/SIGTERM starts no further spawn, closes every pair it opened, ends `process-exit{detail:"interrupted"}`; its probe dir is removed on pass, fail and stop, while SIGKILL or a crash leave it (rows contract/verify-process-exit/interrupted, contract/process-exit-detail/interrupted). Direct precedent and limit for the rig's "end what it starts on every exit path" (credentials copy, scratch root, processes).

@history_item n=20 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — a child of an interactive PTY run killed at a stop carries `child_exit_status` (portable-pty reports a code for a signalled child) and `duration_ms`, as measured; a print-mode or `--version` child killed at the stop is not measured (row contract/verify-process-exit/child-exit-status-and-duration). Bears on how the rig observes the PTY children it kills on its deadline.

@history_item n=21 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — home-tree, instance and `check_diagnostics` strict-modes checks run before any log opens, a refusal writes no line; `verify` checks the home tree before logging; a refused `update_stamps` ends in verify's own `process-exit` (rows decision/home-tree-check-before-log-init, decision/strict-modes-failure-lines, contract/viola-verify-refusal/home-tree-refused). A rig-built home or stamped-home copy must pass these checks (0700/0600, owner) or every start refuses silently.

@history_item n=22 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — the forced feed-panic test also witnesses the passthrough: no `panic`/`parse-rejected`/`vt100`/`unable`/`hint:` reaches the outer terminal and the child's bytes `e4 b8 ad` do (row decision/run-silent-on-terminal). Precedent for reading the outer terminal of a wrapped run as evidence, the rig's kept screens' contract.

@history_item n=23 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (v1-47's secret-scan reading) — the scan runs after each job's E2E run with `AGENT_RUN_KEEP_HOMES=1`, `if: always()`, the `viola-chaos-*` home out (rows decision/secret-scan-scope, decision/secret-scan-before-upload). Rig-booted homes in CI fall under this one scan; nothing of the host's config may be planted where it is scanned or uploaded.

@history_item n=24 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — a refused socket directory (symlink, no dir, foreign owner, mode bit) is `socket-dir-refused`: run/revive exit 1 before any snapshot, child or bind; frame verbs exit 21; hook a `hook-decision` (rows contract/exit-cause-detail/socket-dir-refused and its twins). Trap for the rig's "test socket under the short `/tmp` root": that directory must be owner-only or every start refuses.

@history_item n=25 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — Linux-only OS target, CI on `ubuntu-latest`, openpty the one PTY backend, `cargo deny` on `x86_64-unknown-linux-gnu`, OS-gated test census faulting any windows/macos gate (rows decision/linux-only-os-target, decision/obs-deps-pass-cargo-deny, decision/os-gated-test-census). The rig is Linux-only, any gated rig test must be named in the census, and a new dependency must pass deny.

@history_item n=26 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — portable-pty pinned at `=0.9.0`, its `log` target OFF and failures captured at the seam (row decision/portable-pty-log-target-off). If the rig owns its pty through portable-pty it takes this pin.

@history_item n=27 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** — the secret scan's classes are `CLAUDE*` values, the content canary, mode 0600 and an unreadable file; the Cookie class retired (rows decision/secret-scan-before-upload, decision/no-secrets-in-log-lines, decision/claude-env-and-cookie-never-logged). The rig's credentials copy and config must never reach a scanned file; an unreadable or non-0600 diagnostics file is itself a hit.

@history_item n=28 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — run-start order became `wheel{cause:"start"}` then `session-start` (no `budget-gate`); the additive-only schema rules narrowed to "a field the product still emits" (rows contract/run-start-events/order, decision/diag-schema-additive-only, decision/log-fields-additive-only, decision/log-schema-additive-only). The start order the rig reads, and the rule any reading-driven schema change must keep.

@history_item n=29 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — viola no longer reads or runs the status line; one hidden capture arm remains (`hook <event> --capture`), uninstrumented (rows contract/hook-stdout/capture-arms, decision/capture-arms-uninstrumented). The founder's status line is no product input, so the rig's isolation is what keeps it out of any record.

@history_item n=30 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — the live round on 2.1.325 measured folder-trust keys writing `wheel{human-input}` before `session-start`; the start order is read from that start's own lines because a reused instance name keeps earlier starts' lines in `events.ndjson`, never rewritten (rows contract/run-start-events/order, decision/events-stream-never-rotated). Direct precedent for the rig's trust-dialog reading and for reading a reused rig instance's events.

@history_item n=31 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — `hook_event` gains `stop-failure` (ten names); StopFailure stands with no ledger row (ratified 2026-11-16), its live reading owned by «The 2.1.325 recorded fixture set» (row contract/diag-line-fields/hook-events). Keeps that reading off this rig's carried list.

@history_item n=32 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — the confirm window closes on a `prompt-submitted{origin:"driver"}` equal to the typed text or equal with a trailing ASCII space/TAB/CR/LF run removed; `turn-running` bounded to a turn the wrapper knows of before the paste, a turn whose `UserPromptSubmit` arrives after the paste staying undetected (rows contract/confirm-window-close/ordinary-send, contract/send-refused-detail/turn-running). The two carried readings (cry-vdpx2jqz, cry-iugxdgc4) measure exactly these bounds.

@history_item n=33 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only tolerance; a leading trim («either end») waits for the live reading carried on «Standing live rig with a screen model»; StopFailure's no-ledger-row ratified. The tolerance row stays trailing-only until this chunk's reading lands and the founder places any change.

@history_item n=34 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — named exceptions: a local command's transcript stdout tag counted past the pre-paste end confirms its send, no transcript byte kept; the echo read is uninstrumented and refuses symlinked/foreign transcripts with no line (rows decision/claude-internals-not-a-contract, decision/transcripts-not-a-contract, decision/transcript-echo-read-uninstrumented, decision/deterministic-emission, contract/send-path-log-events/event-order). The rig reading the CLI's screens and dialogs is test evidence, not a product contract; any product reliance on CLI internals takes a named exception like this one.

@history_item n=35 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — `parse-rejected{hook-stdin, unknown-value}` for a StopFailure error word outside the CLI's set, read as cause `api-error`, never `unknown` (rows contract/parse-rejected-detail/unknown-value, decision/parse-failures-surface-as-unknown, decision/parse-failures-unknown-counter). Precedent if the scheduled-wake `source` reading (cry-skycqoql) moves the hook to read a new closed hook-stdin field.

@history_item n=36 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — the founder ratified the local-command-delivery exceptions (claude-internals, refusal-telemetry, event order, deterministic emission, transcripts-not-a-contract) on his own word, unchanged. Those exceptions now stand final; this chunk's readings do not reopen them.
