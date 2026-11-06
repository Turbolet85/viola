@amendment marker=2026-11-05-linux-only-code-and-ci
title: revive's start refusals without batch-script-child
section: §1 A11y Scope Summary (body: the `viola revive --json` line; a11y-scope-summary.md: the CLI verbs entity, the start's refusals under `viola revive --json`)
change:
  - Under `--json` the exit-1 codes of revive's strict-modes check, preflight and start read ten: `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error` (was eleven, `batch-script-child` among them).
  - The start's refusals under `viola revive --json` read `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused`, one exit-1 document each before the child spawns; the body's `viola revive --json` line drops `batch-script-child` the same way.
why: the `.cmd`/`.bat` refusal left the code with the Windows target (the founder's own rulings of 2026-11-04), and its closed word left with its emitter by the overseer's technical answer of 2026-11-05 11:08Z, a narrowing of the scope's «no closed word changes» boundary.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the tui boundary on openpty alone, the ConPTY and F-W3 readings retired
section: §1 A11y Scope Summary (body: the TUI passthrough entity, the tui trigger line; a11y-scope-summary.md: the TUI passthrough entity, its feed-panic Reason, the Owed Windows reading); §4 ARIA Patterns & Roles (body: P4; aria-patterns-roles.md: the tui cases); Keyboard (body); keyboard-test-harness.md (Tooling)
change:
  - The TUI is hosted in openpty (was ConPTY/openpty); focus, mouse and resize sequences and terminal replies (F-W2's closed list) pass through and do not count as editing, with no Windows arm in the entity, the tui trigger line, P4 or the Keyboard ban (was: the ConPTY host swallows focus reports and under win32-input-mode an injected mouse report takes the wheel, F-W3).
  - The feed-panic boundary reads: no viola literal reaches the outer terminal and the child's own bytes (`e4 b8 ad`) reach it (was: no literal on all three OSes, the child's bytes on Unix); the Owed Windows outer-terminal reading of those bytes is retired, with the body's pointer to «Windows-only live measurements».
  - The focus / mouse / resize case asserts no `wheel` record and the bytes reaching the child unchanged, and the zero-viola-bytes case compares the outer-PTY stream byte for byte with an unwrapped fake-agent run (was: the full assertion on Linux and macOS, the platform fact on `windows-2025`, viola's literals alone absent on ConPTY); the Windows ConPTY host-byte paragraph (the inbox and sideloaded preambles, both backends) leaves keyboard-test-harness, and the statusline start writes the settings override and passes `--settings` with no Windows arm.
  - Each of the three boundary clauses is its own nextest case on the `ubuntu-latest` leg (was all three OS legs).
why: the ConPTY sideload and the Windows target are retired (the founder's own rulings of 2026-11-04), `pty_backend()` always `openpty`; the working-route entry «Windows-only live measurements» was retired at the 2026-11-04T23-11-43-wrap.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the manual screen-reader pass, Orca on Linux alone
section: §1 A11y Scope Summary (body: the Manual pass, the screen-reader trigger; a11y-scope-summary.md: the CLI and TUI screen-reader lines); §4 ARIA Patterns & Roles (body: the tool-reach table's manual column); Per-surface SR test pattern (body); screen-reader-test-pattern.md; bootstrap-phases-derive-for-route-setup-project.md (screen-reader-test-spec-setup)
change:
  - The cli pairing reads Orca + GNOME Terminal alone (was NVDA + Windows Terminal/conhost; VoiceOver + Terminal.app; Orca + GNOME Terminal); the SR table's cli row reads `cli (Linux)` (was Windows / macOS / Linux).
  - The tui pairing reads Orca + foot on Hyprland, the founder's host's one terminal, the `xdg-terminal-exec` default (was NVDA + Windows Terminal, Windows the live target); the SR table's tui row reads `tui (Linux)`.
  - The manual pass on P4 and P6 is Orca alone, founder-owned, nothing gated on it; the CLI screen-reader lines read a founder-owned pass on Linux alone (were the NVDA and VoiceOver lines).
why: the Windows and macOS targets are retired (the founder's own rulings of 2026-11-04); the tui's replacement pairing is the overseer's technical answer, founder-delegated, 2026-11-06 14:49Z.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the a11y cases' CI home, the one ubuntu-latest leg
section: §1 A11y Scope Summary (body: Runner, Gate, CLI output discipline); §4 ARIA Patterns & Roles (body: the tool-reach table's cli row, P4, P6; aria-patterns-roles.md: the tui cases, P6); §9 CI Integration (body: Platform, the artifacts table, Pipeline integration); §10 SLO Invariants & A11y Budgets (body); a11y-testing-tool-pick.md; bootstrap-phases-derive-for-route-setup-project.md (a11y-ci-gate-wire); ci-integration.md
change:
  - §9's matrix reads `[ubuntu-latest]` (was `[windows-2025, macos-latest, ubuntu-latest]`); the cli / tui cases run in the `test` job's single instrumented `coverage` suite on that leg, whose `gate --require coverage,doctest` is the verdict (was the per-OS `test` job, each leg its own verdict).
  - Every «all three OS legs» reach in §1, §4, §10 and the three key files reads `ubuntu-latest`.
  - The runner is `scripts/agent-run.sh run` alone (was `.sh` or `.ps1`), in the body and ci-integration.md.
why: the `windows-2025` and `macos-latest` legs and the `{sh,ps1}` shim pair are retired with the Linux-only target (the founder's own rulings of 2026-11-04); `ci.yml` keeps `strategy.matrix.os: [ubuntu-latest]`, and the tui boundary and P6 cases run inside `coverage` there, none skipped.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: server-verify and socket-directory refusals, the macOS and off-Unix carve-outs retired
section: §1 A11y Scope Summary (a11y-scope-summary.md: the CLI verbs entity, the plugin's server-identity entity, the two socket-directory entities); Cognitive Accessibility › Error recovery (body: Verification; cognitive-accessibility.md: cli, the socket-directory refusal, the client euid refusal, its Verification)
change:
  - The macOS no-peer-pid carve-out is retired from the CLI verbs entity, the plugin entity (with its «on every OS») and Error recovery, whose `cli, macOS` item is now `cli, the socket-directory refusal`, its witness `tests/security_negatives_socket_dir.rs`.
  - The socket-directory refusals no longer read «on Unix», and «off Unix the check reads nothing» is retired; `tests/security_negatives_socket_dir.rs` (`#![cfg(unix)]`) is the witness on `ubuntu-latest`, `check_socket_dir`'s Unix body unconditional (was: no Windows witness, ubuntu-latest and macos-latest, `check_socket_dir_off_unix_reads_nothing`).
  - `tests/security_negatives_server_identity.rs` runs ungated on ubuntu-latest with the other cli cases (was `not(target_os = "macos")`, on windows-2025 and ubuntu-latest); the client euid refusal's real read is on Linux (was Linux and macOS); the body's Verification reads ubuntu-latest, both witnesses among it.
why: the macOS no-peer-pid carve-out and socket dir are retired with the Linux-only target (the founder's own rulings of 2026-11-04) and `check_socket_dir`'s off-Unix arm is removed; the server-identity file was made unconditional because its only sibling left.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: strict-modes refusals, the Windows ACL and NotFound arms retired
section: §1 A11y Scope Summary (a11y-scope-summary.md: the frame verbs' strict-modes entity, the `run` / revive / `verify` home-tree entity); Cognitive Accessibility › Error recovery (cognitive-accessibility.md: the unstattable home, the `link` / `unlink` and strict-modes verifications)
change:
  - The frame verbs' strict-modes refusal leaves the mode as found (was «the mode or ACL», «on all three OSes»).
  - A home under a regular file is a strict-modes refusal with its hint (`verify` the home-state pair, exit 1; the six frame verbs the instance-state pair, exit 21); the Windows `NotFound` arm (the `cli` catch-site fault line, no hint, the `internal-error` document under `--json`) is retired, and `verify_under_a_regular_file_refuses_the_home`'s oracle is the home-state pair (was one per OS).
  - The strict-modes cases run on ubuntu-latest, as the server-verify cases do (was all three OS legs, unlike them); `link_verbs_refuse_a_widened_instance_before_any_log` (`cfg(unix)`) runs on ubuntu-latest (was ubuntu-latest and macos-latest).
why: the Windows home owner/DACL strict-modes arm is retired with the Windows target (the founder's own rulings of 2026-11-04), `strict.rs` losing that arm and `windows_read_verdict`.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the Linux-only platform line
section: §1 A11y Scope Summary (a11y-scope-summary.md: the Platform line, the service identity tagging, the multi-platform trigger)
change:
  - New: «Platform (every surface): Linux only — the core on any current distribution, its CI on `ubuntu-latest`, and the 0.2.0 window features on Hyprland; the Windows and macOS code is recoverable at the tag `multi-os-final` (`ecbe7ab3`)».
  - The CLI a11y checks run on Linux, so `os` is `linux` (was three OSes, `os` tagged per matrix leg); the multi-platform trigger must hold on `[ubuntu-latest]` (was `[windows-2025, macos-latest, ubuntu-latest]`).
why: the OS target «Windows first, macOS and Linux» became «Linux only» by the founder's own rulings of 2026-11-04; the diag-line `os` reads `["linux"]`.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the cli SGR and escape checks without their Windows arms, the stale-row witness ungated
section: §4 ARIA Patterns & Roles (body: the stale-row dim line; aria-patterns-roles.md: P6, the stale-row dim case); §6 Visual Design Verification (visual-design-verification.md: the CLI equivalent, a stale row on a terminal)
change:
  - `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` is ungated and runs on ubuntu-latest (was `cfg(unix)`, on ubuntu-latest and macos-latest, not on windows-2025); the body's line is headed «Stale-row dim» (was «Unix legs only»).
  - P6's span check reads bold NAME and amber only on `DIALOG` (was also, on windows-2025, the parsed attribute set); `viola wait` and `viola --help` carry no ESC byte and no lone CR (was on Unix); `tests/cross_list_rows.rs` holds the no-ESC assertion in each mode (was Unix-only, windows-2025 reading each escaped name as text).
  - The colour decision picks a truecolor / 256 / 16 depth with no Windows VT enable (was after enabling VT processing on stdout); the Windows `viola run` ConPTY host-byte clause is retired; the CLI equivalent is checked on `[ubuntu-latest]` (was three legs, the stale-row witness on two).
why: the Windows VT enable and depth branch and the ConPTY sideload are retired (the founder's own rulings of 2026-11-04), rungs 1–6 and the depth rule unchanged; the stale-row witness was made unconditional because its only sibling left.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
@amendment marker=2026-11-05-linux-only-code-and-ci
title: the three-runner greens re-worded as dated readings
section: Cognitive Accessibility › Timeout extensions (cognitive-accessibility.md: the closed-gate case); keyboard-test-harness.md (Tooling: the revive, statusline and budget readings)
change:
  - `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` reads green as measured at CI run 38098238731 (was green on the three CI runners).
  - keyboard-test-harness's revive, statusline and budget-gate readings read green in CI (was green on the three CI runners), their chunks and runs kept.
why: the CI matrix moved to `[ubuntu-latest]`, and a dated measurement on the retired legs is re-worded by its owner, never left as current.
ref: .andromeda/runs/2026-11-05T13-05-41-wrap/
