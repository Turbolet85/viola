@history_item n=1 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — (the distro TMPDIR carve-out, the leg-verdict upload and the download-artifact pin retire) Removed the CI and pre-push mutation legs on the founder's ruling, which became decision/mutation-testing-at-audit. That is why this chunk runs no mutation itself and the next boundary's audit does. The pre-push crossing carries HOME and PATH only. A TMPDIR taken from a host value would be a boundary widening, which matters if the chunk pins the scratch TMPDIR for its socket-length bound.

@history_item n=2 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — Closed the `mutants.out/` CARRY by removing its upload, the origin of decision/no-mutants-out-upload (it held absolute argv paths and test output). Nothing this chunk adds to the `mutants` profile or its runs may bring back an upload of mutation output.

@history_item n=3 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — Recorded the `mutants.out/` upload as unscanned by the artifact canary scan and gave it a CARRY. This is the earlier half of the no-mutants.out-upload history.

@history_item n=4 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — (the tempfile `persist` helper; `TMPDIR` carve-out under `env -i`) Added the mutation leg's `TMPDIR=<distro home>/viola-pre-push-scratch`, a named, constant, distro-derived assignment. It set the precedent that the mutation scratch's TMPDIR is a fixed path and never a host value. That path is what item 1's socket-length bound is computed under.

@history_item n=5 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** — (the native pre-push crossing; the WSL launchers retired) The scratch guard and wipe run only on a Windows host. On the Linux host the same-named NOCOW `TMPDIR` belongs to the operator and the harness never wipes it. This is the scratch (architecture row `mutation-scratch-nocow`, out of scope here) under which the test socket path overflows `sun_path`.

@history_item n=6 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** — (the wiped mutation scratch re-created owner-only) A root-package baseline failed the instance strict-modes check inside a plainly re-created scratch. The fix: re-create it owner-only and NEVER loosen the check for it. Precedent for this chunk: a baseline red under the scratch is fixed on the test side, and the product checks stay as they are.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (CI runs one ubuntu-latest leg; windows-mutants.yml and the mutation scratch retired) Deleted `windows-mutants.yml` and `scratch.rs` and retired the `run --mutants` scratch-wipe ban. CI now has one `ubuntu-latest` leg, which runs the socket-directory negatives. Mutation now runs only at the audit, under the operator's Linux scratch.

@history_item n=8 marker=2026-10-04-windows-boundary-mutation-workflow
**2026-10-04-windows-boundary-mutation-workflow** — Founder ruling C2: the mutation workflow is dispatch-only, run at the epoch-boundary audit, and never a gate. Together with item n=9 this is the precedent for "NO mutation run, local or dispatched" inside a chunk.

@history_item n=9 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** — Recorded the one dispatch made outside the epoch-boundary audit, allowed by the founder for that chunk alone. A trap: a mutation run inside a chunk needs the founder's own word, and this chunk's directive forbids one.

@history_item n=10 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — Added the `fake-agent`-only `FAKE_AGENT_PUMP_DELAY_MS` test seam (capped, compiled out of release builds, configures nothing). It also set the NEVER-another-seam rule, now decision/test-seams-fake-agent-only and decision/new-env-read-needs-log-entry. Any env knob this chunk might add to size test waits or margins falls under that ban.

@history_item n=11 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — (test seam `FAKE_AGENT_HOOK_PANIC` and G2's exact-path exemption, ratified by the founder) A second test seam, ratified live as a boundary widening. The ban now says another seam or env read needs its own Decisions Log entry. It binds any seam the slow-case, receipt-wait or hook_fail_open test changes might want.

@history_item n=12 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — Recorded release-check's refusal of `test-support` / `fake-agent` artifacts (probe 5/5) and the host mutation scratch guard. This is the origin of decision/release-build-no-test-binaries and decision/test-support-dev-only. `viola-hypr-tap`, the target of item 4, is a test-only binary that must stay out of the release build.

@history_item n=13 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — (registry migration: the Decisions Log leaves the body) Moved into the body: `scripts/release-check.sh` refusing any executable other than `viola` and any test-only artifact, and `test-support` enabled only by the root package's dev-dependencies. These are rows decision/release-build-without-test-features and decision/test-support-dev-only. Hypr-tap test changes must not widen either.

@history_item n=14 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (the hidden `hook --capture` arm, verify's probe captures and the fixture scrub) `verify --record` scrubs home to `~` and the user word to `<user>`. It refuses the whole recording when `/home/` or the user word survives. This is the mechanism behind item 5's record-home naming rule: a test home with a `home` path component trips the refusal.

@history_item n=15 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — Turned the fixture refusal into `a recorded fixture is not clean: <file> <code>` with the closed codes `home-path` · `absolute-path` · `username` · `email`, never the content. These are rows contract/fixture-refusal-line/form and contract/fixture-refusal-codes/home-path. Item 5's naming helper exists so a planned home never triggers `home-path`.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (verify's refusal codes) `verify` writes `unclean-fixture` among its closed refusal codes on its `process-exit`, and the fixture's file name enters no code. This is row contract/verify-refusal-codes/unclean-fixture, the code a misnamed record home produces.

@history_item n=17 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the R8 strip and the fixture scrub match names exactly; the registry reader retired) The fixture scrub now matches the home prefix exactly, with no case fold. Item 5's rule must hold under that exact match.

@history_item n=18 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — (the `Mcp.` exchange recorded by the test-only tap) Precedent for a test-only tap recorder, `viola-mcp-tap`: it reads lines through `take(MAX_FRAME)`, writes 0600 captures in a private dir that is never committed, applies the shared scrub, and refuses an unclean fixture with closed codes. `viola-hypr-tap` follows the same pattern, and its `main`/arms are what item 4 kills or rules out of the tier.

@history_item n=19 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (Data Protection: decision/hyprland-recorder-string-allowlist) Added the recorder's closed per-request key allowlist. Any other string becomes the placeholder, a non-JSON reply stays only `ok`, and an unclean set is refused whole. The guards are `record_replaces_every_string_off_the_allowlist` and `planted_hyprland_lastwindowtitle_is_rejected`. The tests that kill item 4's survivors (`record` / `record-dispatch` arms, `version_of`, `texts`'s `Value::Array` arm) must keep this rule intact.

@history_item n=20 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (Data Protection: decision/hyprland-reply-text-dropped) Hyprland reply types hold no text field, and titles reach no line or error. The guard is `contract_hyprland_canary_reaches_no_line_and_no_error`. Hypr-tap tests built for item 4 must not let reply text into an assertion message or a fixture.

@history_item n=21 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (Input Validation: 3 rows) Widened decision/parser-fuzz-scope to the hyprctl reply parsers and added decision/hyprctl-read-deadline and the seam's Address validator exception. The scope row this chunk's hypr-tap tests sit beside.

@history_item n=22 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (Threat Model Summary: 2 rows) Changed child output parsing so a failure becomes unknown or a closed failure word (oversize, malformed, unreachable). These are the hyprctl read's outcomes, which the recorder's `version_of` arm handles.

@history_item n=23 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — (listener hardening as landed: chmod 0600 after the bind) Replaced `ListenerOptionsExt::mode` with chmod 0600 after the bind. These are rows decision/socket-chmod-after-bind, decision/chmod-socket-after-bind and decision/no-listener-mode-option. The test channel in `tests/hook_fail_open.rs` binds through the same server, so a renamed test socket keeps this order.

@history_item n=24 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the socket-directory check, built at the bind and the client seams) Built `check_socket_dir` / `check_endpoint_dir`, one helper for the bind and the three client seams (CLI, hook, MCP). The endpoint's parent must be owned, have no group/other bit and not be a symlink. Item 1's new test socket name or directory must leave a parent that passes this check, or the hook's fail-open path reads `socket-dir-refused` and not the case under test.

@history_item n=25 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`socket-dir-refused`, the sixth cause and the eleventh code) Added `socket-dir-refused` as exit 21's sixth cause and a `revive --json` code, carried by row contract/ipc-client-refusals/socket-dir-refused. A misplaced test socket shows up under this code.

@history_item n=26 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (a refused socket directory at a hook and at a start) A hook on a refused socket directory exits 0 with empty streams and no frame, and writes one `hook-decision` of detail `socket-dir-refused` at WARN. This is row contract/socket-dir-refusal/hook, the fail-open shape `tests/hook_fail_open.rs` asserts.

@history_item n=27 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the client's peer euid check and the order at a seam) Fixed the order on every client seam: socket-directory check, connect, peer euid decision, server verification. This is row decision/client-seam-check-order. A refused euid reads `server-verify-failed`.

@history_item n=28 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the tests holding the directory and peer checks, and no seam to drive them) Named `tests/security_negatives_socket_dir.rs` as the guard of decision/socket-directory-check. Added a ban: NEVER add an env, flag or feature seam to force a refusing peer-euid decision. A trap for the test socket rework: prove with injected values, not a new seam.

@history_item n=29 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the Unix socket's place in the threat model) The socket file is `<dir>/viola-<h12>.sock` in the per-user 0700 directory, never the `/tmp` or `$TMPDIR` root. These are rows decision/no-socket-in-tmp-root, decision/socket-in-owner-only-dir and decision/per-user-socket-directory. The product's 12-hex name is the fixed-length precedent for item 1's pid-free test socket name.

@history_item n=30 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (a killed wrapper's leftover socket file) A killed wrapper's socket file stays until the next bind of that name takes it over, a harmless file in a 0700 directory. A pid-free test socket name means a rerun meets a same-named socket, which matters with decision/no-socket-overwrite and decision/exclusive-endpoint-bind.

@history_item n=31 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the IPC endpoint is a Unix socket alone; the Windows named pipe retired) The endpoint is one Unix socket in `$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`, chmod 0600 after the bind, and the abstract namespace is still never used. This is the current form of rows decision/socket-in-per-user-dir, decision/no-abstract-namespace and decision/no-tls-local-transport. A shortened test socket must not move to the abstract namespace to dodge `sun_path`.

@history_item n=32 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (client server verification is one Linux flow) Every frame-writing client checks the socket directory, the peer euid and the peer pid plus start time against the snapshot before the first frame, with no carve-out. This is the current form of decision/server-verified-before-any-frame and decision/verify-server-before-frame.

@history_item n=33 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — Served `hook.event` and set the hook stdin fail-open details `oversize-stdin` · `malformed-json` · `channel-unreachable`. These are rows contract/wrapper-channel-methods/hook-event and contract/hook-decision-detail/channel-unreachable, which `tests/hook_fail_open.rs` exercises.

@history_item n=34 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (a failed server check: no frame, and what each client reports) The hook fails open with exit 0, no body, and its `hook-decision` carrying `server-verify-failed`. A hook event or dialog whose check fails writes no frame, and a refused SessionEnd is appended directly. These are rows contract/server-verify-refusal/hook and decision/hook-refusals-write-no-frame.

@history_item n=35 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the observed server pid and the verdict recorded as built) Built `ServerVerdict { Matches, Mismatch, Unverifiable }`. Only `Matches` lets a frame out, per row decision/only-matches-verdict-lets-a-frame-out. The test channel in `hook_fail_open.rs` must still verify as the recorded wrapper after its rename.

@history_item n=36 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the dated strict-modes gaps retired: every client checks the instance before the snapshot) Every client reads the snapshot only after `check_instance`, and a refusal sends no frame and writes no line. This is the source of decision/snapshot-strict-checked-before-trust and decision/client-checks-before-frame.

@history_item n=37 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (`viola hook`'s event and dialog paths check the instance before obs init) A refused or absent instance exits 0 at once with nothing read or written, and the capture arm stays ahead of the check. This is the hook fail-open order behind decision/hooks-fail-open and decision/hook-fails-open.

@history_item n=38 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the snapshot readers move onto `read_snapshot_or_replay`) Over a replayed snapshot no endpoint is given, hook connect fails open as `channel-unreachable`, and the collision check judges by the heartbeat. This is the source of decision/replayed-snapshot-fallbacks.

@history_item n=39 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the frame ban met in full by every frame) Every frame meets the frame-before-verification ban in full, `hook.event` and `hook.dialog` included. No test-side change may add a bypass.

@history_item n=40 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (Error Handling: 2 rows) Changed the hidden capture arms from two to one in decision/hook-silent-exit-zero (never stderr, always exit 0) and decision/hook-strict-modes-first. These are the current wordings the hook tests assert against.

@history_item n=41 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (Input Validation: 9 rows, 2 of 2) Dropped the statusline clause from decision/replayed-snapshot-fallbacks. This is the row's current text.

@history_item n=42 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (Threat Model Summary: 4 rows) Dropped `statusline_command` from rows contract/cli-state-root/home-flag and contract/cli-state-root/viola-dir. `--home` / `VIOLA_DIR` choose the state tree, snapshot, endpoint and stamps. The record home and test homes are chosen through these.

@history_item n=43 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — (revive's --json documents; the CLI home resolution as landed) Every verb resolves `--home` as given, else the `VIOLA_NAME` / `VIOLA_DIR` instance home, else `<user home>/.viola`, canonicalised and with nothing created. This is the origin of decision/cli-home-resolution-order. Item 5's naming helper passes its home through this path.

@history_item n=44 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — (the VIOLA_DIR home step ratified) The founder ratified the `VIOLA_DIR` step and canonicalisation, which is the "Ratified" why on decision/cli-home-resolution-order.

@history_item n=45 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — NEVER let a test-side removal take anything but a dir the test side created. `target/e2e-home` links to `/tmp/viola-e2e-home-<uid>`, and the keepers' link checks are fixed. A trap: a mutation run of the harness keeper leaves `backing/` in its copied tree. The home helper item 5 binds (`tests/support/home.rs`) and the test tempdirs sit on this path.

@history_item n=46 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — (the e2e-home keepers refuse a backing below the headroom floor) The keepers also refuse a backing below 9 GiB free, with a fixed message that names no path. A home-naming change in the test support must keep both keepers' refusals.

@history_item n=47 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (`MAX_FRAME` consumers extended, verify's CI split) The real CLI runs only locally, and `verify` runs in CI only against the fake agent, its real-CLI probe and `--record` only locally. These are rows decision/real-cli-local-only and decision/ci-no-secrets-no-real-claude. That is why this chunk has no live `claude` start, and why its `cli_verify` and `--record` tests run on the fake agent.

@history_item n=48 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — (Run E in the verify child set; a third session-id source) Run E resumes only the `session_id` of Run B's first SessionStart capture, and only when it has the session-id shape. With none, Run E spawns nothing. This is the source of decision/verify-run-e-resume-target. Item 3's red case (`verify_stopped_during_the_resume_run…`, "timed out waiting for Run E's start") waits on this run.

@history_item n=49 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — (verify's dirs removed on a Ctrl-C / TERM stop) A stopped verify kills its child with no key and removes only the dirs that run created. Item 3's red case asserts exactly this (the resume dir removed, exit 1), so the receipt-wait change must not weaken it.

@history_item n=50 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's stop and the colour decision lose their Windows arms) verify's stop is `SIGINT` / `SIGTERM` alone. This is the stop that item 3's red case drives.

@history_item n=51 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the clients' screen of a peer's reply) Added `sanitised_fault` and `is_code_word`, so nothing of the peer's but closed values reaches a CLI `--json` document or an MCP result. A code word outside the closed list stays visible by design. This is the source of row decision/client-reads-closed-values-only, which binds what the test channel may answer.

@history_item n=52 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — (Authentication & Authorization: decision/live-round-rig-keys-own-pty) The live round's rig keys only into its own pty, on a per-chunk word. Item 5 names the rig (`live-hold.py`, gitignored chunk evidence) as a candidate for the home-naming helper. This chunk has no live start, so that rig stays outside the chunk.
