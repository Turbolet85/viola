@carried n=1 label=security-plan/security-anti-patterns fact=fct-izf47ccx
why: The Unix parity work must keep every client frame behind the server verification on Linux (euid, pid and start time) and on macOS (euid and the verified 0700 socket directory only, the recorded gap), and no parity test may read macOS's check as a pid comparison.
quote: NEVER write any channel frame (`send`, `answer`, `hook.dialog`, `wait`/`last`, `pause`/`release`, `link`/`unlink`, and `hook.event` notifications, which carry prompt and reply text) before verifying the server's pid (Windows: the connected stream's peer pid, interprocess `peer_creds().pid()`, which calls `GetNamedPipeServerProcessId` on the SQOS-adopted client end) or euid + pid (Linux/FreeBSD `peer_creds()`; the client-side euid check, `peer_admitted(client.server_euid())` at the three client seams, before the server verification) plus sysinfo start time against `snapshot.json` (on macOS, where `peer_creds()` gives no pid: euid + the verified 0700 socket directory only, per the recorded known gap; the euid check is never skipped), and NEVER take those reference values (`endpoint`, `pid`, `started_at`) from a `snapshot.json` that has not passed the strict-modes check in the same process.
@carried n=1 run=2 label=security-plan fact=fct-6cauxi6n
quote: NEVER write any channel frame before verifying the server against a `snapshot.json` that passed the strict-modes check in the same process (the euid check never skipped), except the named carve-out: macOS's pid and start-time check (no peer pid), owned by no route entry, its euid and verified socket-directory checks standing; no other frame or process may borrow it.
@carried n=1 run=3 label=security-plan fact=fct-zshlqykf
quote: NEVER place a Unix socket directly in `/tmp` or at the `$TMPDIR` root (`/tmp/viola-<h12>.sock`). Always use a verified 0700 per-user directory
@carried n=1 run=4 label=security-plan fact=fct-zyth7okk
quote: NEVER set it with `ListenerOptionsExt::mode`: it returns `Unsupported` on macOS, which fails the bind; chmod after the bind instead.
@carried n=1 run=5 label=security-plan fact=fct-2g5b5nut
quote: NEVER use Linux abstract-namespace sockets (no filesystem permissions).
@carried n=1 run=6 label=security-plan fact=fct-3nboftz7
quote: NEVER enable interprocess `try_overwrite` (TOCTOU deletion of an attacker-swapped file).
@carried n=1 run=7 label=security-plan fact=fct-gqlrkxte
quote: NEVER open a Windows client pipe without `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION`
@carried n=1 run=8 label=security-plan fact=fct-vbcb52sx
quote: NEVER write `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any R8-stripped `CLAUDE*` value into any log or stderr, `events.ndjson`, snapshots, `diagnostics/` or fixtures (consolidated floor: Bootstrap phases, `logging-redaction-wire`).
@carried n=1 run=9 label=security-plan fact=fct-6xxq3ubv
quote: NEVER expose absolute paths in any external error.
@carried n=1 run=10 label=security-plan fact=fct-tnu5bfz7
quote: NEVER write to stderr from `viola hook`, and NEVER exit non-zero from it, including on security-check failures.
@carried n=1 run=11 label=security-plan fact=fct-sv5b7ryi
quote: NEVER run `statusline_command` from a `snapshot.json` whose home or instance directory fails the strict-modes check. Write access there equals code execution.
@carried n=1 run=12 label=security-plan fact=fct-fmxdaj54
quote: NEVER create `~/.viola/`, `--home` or any file under it with the default umask. Directories are 0700 and files 0600 (`OpenOptionsExt::mode`); the only exception is the pinned `bin/<version>-<hash>/viola` at 0700, and NEVER make it group- or world-writable. On Windows, NEVER accept a `--home` outside `%USERPROFILE%` without an explicit protected user + SYSTEM DACL, and NEVER skip the Windows owner + DACL strict-modes check at startup (`#[cfg(windows)]` must not return `Ok(())` unchecked)
@carried n=1 run=13 label=security-plan fact=fct-tpzheerr
quote: NEVER spawn a `.cmd`/`.bat` PTY child through portable-pty. Resolve to a real `.exe` (portable-pty 0.8.1 bypasses std's BatBadBut escaping).
@carried n=1 run=14 label=security-plan fact=fct-iftwwapx
quote: NEVER make channel handles inheritable, and NEVER adopt a portable-pty replacement that calls `CreateProcessW` with `bInheritHandles = TRUE`.
@carried n=1 run=15 label=security-plan fact=fct-llbob32q
quote: NEVER print `wait`/`last` human-mode output, or `list` rows taken from `claude agents --json`, to a terminal without escaping C0/C1 controls (the same class as RUSTSEC-2025-0055).
@carried n=1 run=16 label=security-plan fact=fct-423bwibm
quote: NEVER create `diagnostics/` files readable by other users.
@carried n=1 run=17 label=security-plan fact=fct-zavgpgbb
quote: NEVER call `read_line` on a channel stream or `read_to_end` on hook stdin or child output without `Read::take(MAX_FRAME)`
@carried n=1 run=18 label=security-plan fact=fct-fmxdaj54
quote: NEVER create `~/.viola/`, `--home` or any file under it with the default umask. Directories are 0700 and files 0600 (`OpenOptionsExt::mode`); the only exception is the pinned `bin/<version>-<hash>/viola` at 0700, and NEVER make it group- or world-writable.
@carried n=2 label=security-plan/security-anti-patterns fact=fct-y573epy2
why: Every fake-agent test the chunk runs on ubuntu-latest and macos-latest must create and remove its home under target/e2e-home only through the keepers, removing nothing it did not itself create.
quote: NEVER let a test-side removal under `target/e2e-home` take anything but a dir the test side itself created: the root fixture chain's owner sweep and a test home's drop (by `owner.json` record, by its own `TempDir`) and harness `cleanup` (by the `viola-session-` name).
@carried n=2 run=2 label=security-plan/security-anti-patterns fact=fct-y573epy2
quote: The two keepers (`prepare_home_base` in `tests/support/home.rs`, `Workspace::ensure_e2e_home` in the harness) NEVER follow a linked base unless its target is absolute and one `lstat` of it reads a real directory with no group or other bit; a target that is itself a link is refused
@carried n=2 run=3 label=security-plan fact=fct-4fkoyw3x
quote: NEVER let a test-side removal under `target/e2e-home` take anything but a dir the test side itself created; its two keepers NEVER follow a linked base unless its target is absolute and an owner-only real directory, create the target 0700 and remove nothing; `viola` itself never reads or writes the backing by that name.
@carried n=3 label=security-plan/security-anti-patterns fact=fct-zvkglgpg
why: A dialog-answering case moved off cfg(windows) must stamp its home over the Unix relied set and assert the per-OS counter as documented, never one OS's value on the other, so no non-null decision flows without a full stamp on any OS.
quote: NEVER answer a `hook.dialog` with a non-`null` decision unless `ledger/stamps.json` holds a `viola verify` stamp for the child's CLI version. NEVER let any process other than `viola verify` write `ledger/stamps.json`.
@carried n=3 run=2 label=security-plan/security-anti-patterns fact=fct-zvkglgpg
quote: and a stamp is read over the relied set (`LedgerRow::relied()`, counter `/24` on Unix; on Windows, where the status line is not wrapped, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`, counter `/21`), so a non-`null` decision flows only when every relied row passes: a home stamped `20 pass  4 fail` on Unix (`17 pass  4 fail` on Windows) records `cli_verified:false`
@carried n=3 run=3 label=security-plan fact=fct-syrndlhx
quote: NEVER answer a `hook.dialog` with a non-`null` decision unless `ledger/stamps.json` holds a `viola verify` stamp for the child's CLI version on which every relied ledger row passes
@carried n=3 run=4 label=security-plan fact=fct-7izg6s4l
quote: NEVER answer a `hook.dialog` with a non-`null` decision unless `viola run`'s version gate read `ledger/stamps.json` through the strict-modes check (`read_stamps_strict`); a refusal is `cli_verified:false`.
@carried n=4 label=security-plan/security-anti-patterns fact=fct-bg5w767t
why: The Unix suite must reach the FAKE_AGENT seams only through the fake-agent feature, the v1-40 witness may set the colour facts only to choose SGR bytes, and any Unix-only test seam the chunk adds needs its own Decisions Log entry, as the scope names.
quote: NEVER let `config.json`, a `VIOLA_*` environment variable or a CLI flag switch off any control in this plan: the IPC DACL or socket-directory check, the peer or server verification, the home strict-modes check or `validate_paste_text`. Environment variables are not a configuration channel (architecture.md Cross-cutting Patterns: Config management), and `VIOLA_DIR` is set by a child any same-session agent can influence. The two carve-outs are the test seams `FAKE_AGENT_PUMP_DELAY_MS` (capped at 5 s; Decisions Log 2026-09-27) and `FAKE_AGENT_HOOK_PANIC` (fires only on the exact value `1`, a forced fail-open `hook` panic; Decisions Log 2026-09-28): each exists only under `cfg(feature = "fake-agent")`, is absent from every release build and switches off nothing (§Input Validation). The terminal colour facts `NO_COLOR`, `TERM` and `COLORTERM` are read in every build (the founder's own word, chunk 2026-10-17-cli-output-discipline), only by `viola list`'s human mode and only to choose whether and how deep SGR bytes are written: they switch off no control (no escaping, no `validate_paste_text`, no check), widen no redaction and are not a configuration channel (§Input Validation). NEVER add another such seam or environment read, let these reach a release build, or let a colour fact change a control or reach a log line, error or refusal, without its own Decisions Log entry.
@carried n=4 run=2 label=security-plan fact=fct-7ackeyph
quote: NEVER let `config.json`, a `VIOLA_*` environment variable or a CLI flag switch off any control in this plan (the IPC DACL or socket-directory check, the peer or server verification, the home strict-modes check, `validate_paste_text`): environment variables are not a configuration channel. The carve-outs, the two test-only seams and the terminal colour facts, switch off nothing; NEVER add another such seam or environment read without its own Decisions Log entry.
@carried n=5 label=security-plan/security-anti-patterns fact=fct-lvhmwli7
why: The Unix legs' peer-euid refusals must be proven only over injected values, never through a seam that forces a refusing decision, and each client seam's wiring stays the named weakest proof.
quote: NEVER add an env, flag or feature seam to force a refusing peer-euid decision: one account can make no foreign peer, so the decisions are proven over injected values (`peer_euid_admitted(observed, own, reported)`, and the wrapper's private `serve_admitting(dispatch, admitted)` over an injected `Admission`), and each client seam's `peer_admitted` wiring (`cmd::client::connect`, `cmd::hook::connect`, `viola-mcp`'s `frame`) stays the named weakest proof, read only through a run's forced refusal, which no committed test repeats.
@binding n=1
**security-anti-patterns (frames before server verification) ↔ authentication-authorization**: the Unix parity suite must hold the client-side verification exactly as the Auth row states it, Linux comparing the peer pid and start time, macOS the euid and socket directory alone with `server-verify-failed` an euid mismatch, and no parity test reading macOS's check as a comparison (security-plan/security-anti-patterns fct-izf47ccx) (security-plan fct-phspeqis) (security-plan fct-dthgjfbs)
@binding n=2
**security-anti-patterns (seams and colour facts) ↔ input-validation**: the FAKE_AGENT seams the Unix suite now reaches must keep their Input Validation rows (the 5 000 ms cap, panic only on exactly `1`, compiled out of release), and the v1-40 witness's colour facts must stay compared only, never echoed (security-plan/security-anti-patterns fct-bg5w767t) (security-plan fct-ykdescly) (security-plan fct-bbnvgu6s) (security-plan fct-hqz7q7dg)
@binding n=3
**security-anti-patterns (test-only seams) ↔ dependency-security**: the Unix legs' release check must still refuse any test-only feature in the viola artifact, and G2 on ubuntu-latest and macos-latest must keep exactly the one FAKE_AGENT_HOOK_PANIC exemption, with its probe run first (security-plan/security-anti-patterns fct-bg5w767t) (security-plan fct-h3byb6pq) (security-plan fct-wsxv34pw)
@binding n=4
**security-anti-patterns (peer-euid seam ban) ↔ authentication-authorization**: the server-side peer check on both Unix OSes (drop a foreign-euid peer before any frame) must be proven over injected values, with no seam that forces a refusal (security-plan/security-anti-patterns fct-lvhmwli7) (security-plan fct-unwy7exn)
@binding n=5
**security-anti-patterns (e2e-home keepers) ↔ secret-management (Development)**: every Unix fake-agent test must run under its own home made by the keepers, with no host environment value crossing into a child (security-plan/security-anti-patterns fct-y573epy2) (security-plan fct-z4tolcdo)
@binding n=6
**security-anti-patterns (stamp gate) ↔ authentication-authorization (accepted risk)**: the `unverified-cli` gate is viola's only part in the driver-injection risk, so every OS must keep its relied set whole and refuse a non-null decision on a partial stamp (security-plan/security-anti-patterns fct-zvkglgpg) (security-plan fct-ty6c2nnj)
