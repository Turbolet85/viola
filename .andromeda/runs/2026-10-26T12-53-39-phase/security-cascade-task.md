# Task — security · 2026-10-26-explicit-session-links

## The chunk
Serves the two channel methods `link` and `unlink` in the wrapper (today answered `-32601`): `link` records a driver/driven pair and answers `{since, already_linked}`, `unlink` removes it (an absent pair appends nothing); a `driven` that is not the serving wrapper's name, or a `driver` equal to `driven`, is `-32602`.
Adds the CLI verbs `viola link <driver> <driven>` and `viola unlink <driver> <driven>` with `--json`, the verb's instance being the driven one, their three human result lines, and the `handoff:  link, unlink` row of `viola --help`.
Gives `LinkSet::remove` its first product caller (the `unlink` method), ending its dead-code allowance, and adds the root cases in `tests/channel_links.rs` (Scenario E3 steps 1 and 3, the raw-channel name negative) and `v1-13`'s `link` half.
Is a boundary widening held for the founder's own word at P4: the plan covers every form up to it (the methods served; the verbs' snapshot read under a dated strict-modes gap, or behind the instance check `viola revive` takes; or no verbs), each re-stated against HEAD, where the server identity check before the first frame has already landed on Linux and Windows.
Re-parses every logged link name shown in a result line through `ViolaName::try_new`; the board shows no link.

## Reached files
- src/run/links.rs · edits `LinkSet::remove` (its product caller and the `#[cfg_attr(not(test), allow(dead_code))]` it loses)
- src/cmd/run.rs · edits the channel method dispatch (the `link` / `unlink` arms answered `-32601` today)
- src/cmd/mod.rs · edits the `viola --help` template (the `handoff:  link, unlink` row between `wheel:` and `setup:`)
- tests/cli_output_plain.rs · edits the `HELP` literal of `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs`
- tests/channel_links.rs
- viola-0.1.0/verification-matrix.json · edits `v1-13` (its `link` half)

## Needs
- A new channel method served by the wrapper (`link`, `unlink`): the Universal ban on adding a channel method without a Security Decisions Log entry mapping it to the plan's IPC controls.
- The boundary widening itself: crossing (a) the wrapper serving two more methods and crossing (c) the verbs reading `snapshot.json`'s `endpoint`, `pid` and `started_at` without the strict-modes check; ratified only by the founder's own word, shown the widening as itself, recorded as an input.
- IPC client-side server verification before the first frame: the two verbs' frames go through `client::connect`'s peer pid and start-time check against the snapshot on Linux and Windows, no frame on a mismatch or a check that cannot be made, no fallback to a default connect.
- The Authentication anti-pattern ban on writing any frame before verifying the server against a strict-modes-checked snapshot, and its closed list of dated gaps (`hook.event`, `hook.dialog`, `send`, `wait` / `last`, `answer`, `pause` / `release`, `viola mcp`'s tools) that no other frame may borrow: the verbs need their own dated gap or the strict-modes check.
- The instance strict-modes check `viola_state::strict::check_instance` run before the snapshot read, as `viola revive` takes it (home, `instances/`, instance dir, `snapshot.json`, `events.ndjson`), as the second option for the verbs; the strict-modes work of the eight other clients stays "Home and code-bearing file integrity"'s.
- The liveness-only pre-check (`viola_state::liveness::live_endpoint`: snapshot pid and start time alive, heartbeat live, an `endpoint` present) the verbs take before connecting, else exit 21 `instance-unreachable`.
- The dated gaps still owed to "Unix endpoint and home hardening": the client euid check and macOS, where the peer reports no pid.
- CLI home resolution for the new verbs: `--home`, else a valid `VIOLA_NAME` / `VIOLA_DIR` pair, else the user home's `.viola`, canonicalised before any snapshot read.
- Names on both sides through `ViolaName::try_new`: the verbs' `driver` / `driven` arguments in clap, and the channel `link` / `unlink` params re-checked by the wrapper (the input-validation bootstrap wiring names these params), never joining `instances/<name>` with an unparsed string.
- The wrapper's own param check on `link` / `unlink`: `driven` not the serving wrapper's name and `driver` equal to `driven` refused `-32602` before any line is appended; channel frames under `MAX_FRAME`, serde_json default depth.
- The second CARRY: a logged link name shown on a surface (the result lines `link … since …Z`, `link … already linked`, `unlink …`) parsed again through `ViolaName::try_new`.
- The channel `from` / driver name as self-reported, never an authenticated identity: it serves link records only.
- The `link` / `unlink` event lines appended to `events.ndjson`: one write per line, never truncating the log, 0600 file in 0700 dirs, never the umask.
- The snapshot's `links` items `{driver, driven, since}` written only by the instance's own wrapper, through `replace_private`.
- External error sanitisation for the verbs and methods: CLI `--json` errors and channel `error.data` carry codes and fixed messages only, no absolute path, pid, start time or endpoint; `-32603` exactly `internal error` with `data: null`.
- Refusal results of the verbs: exit 21 `instance-unreachable` on a failed liveness or server check, writing no frame, the unreachable line holding no pid, start time, path or endpoint.
- Leaving `link` and `unlink` off the MCP tool list is an affordance, not an enforcement (MCP boundary), against `v1-13`'s MCP-result wording.
- A security refusal never blocks the human: refusals of `link` / `unlink` go to the calling automation only.

## Standing conventions
- S1 · Every new log line keeps the NEVER-log floor: no R8-stripped `CLAUDE*` value, user content only in `diagnostics/`, no absolute path or internal type name in external error output · Bootstrap phases, `logging-redaction-wire`
- S2 · Every new error type's `Display` uses a fixed message, no path or payload interpolated, and anyhow chains stop at the human-mode stderr without a serde source · Error Handling
- S3 · Every new test runs under its own `--home`, and no local `--home` test directory is committed · Secret Management (Development)
- S4 · A test-side removal under `target/e2e-home` takes only a dir the test side itself created · Security Anti-Patterns › Code Patterns
- S5 · Every file a test run leaves under `target/agent-run/` passes CI's artifact canary scan before any upload · Secret Management (Secret scanning in CI)
- S6 · No new environment read or test seam outside `VIOLA_*` plumbing without its own Decisions Log entry; no flag, env var or `config.json` key switches a control off · Security Anti-Patterns › Universal
- S7 · Every new file under the home is created 0600 (dirs 0700) through `OpenOptionsExt::mode` / `replace_private`, never the umask · Security Anti-Patterns › Data Protection

## Not needed
- Paste text and dialog free-text validation (`validate_paste_text`, `control-character`, `empty-text`): a link carries no typed text.
- The dialog-answer stamp gate and `read_stamps_strict`: no `hook.dialog` decision is made.
- The statusline source read, the `/bin/sh -c` shell-out and `budget.json`: no statusline or budget path changes.
- Hook stdin parsing, the hook's fail-open path and the hidden `--capture` arms: no hook arm changes.
- The `claude agents --json` read, its row `name` rule and the board's `escape_row`: the board shows no link and `list` is not changed.
- `viola revive`'s session-id argv shape, `resume_args`, its `--json` refusal codes and its recorded `cwd`: only its instance-check form is named as an option.
- The Windows ConPTY companions, the vendored ConPTY audit script and the DLL-search restriction.
- The Windows persistent-environment registry read and the R8 strip of the child's environment: the verbs spawn no child.
- PTY output, the vt100 feed under `catch_unwind` and the human's stdin classifier.
- `viola verify`'s trusted runs, the scratch footer residual and the live `plugin install` residual (Data Protection accepted risks).
- The Windows pipe listener DACL and SQOS open, and the Unix socket directory and peer euid check on accept: the listener is unchanged, the methods ride the existing endpoint.
- `run --mutants`' Windows scratch and the dispatch-only `windows-mutants.yml`.
- The v1.x release prerequisites (dist, cargo-auditable, `self_update` signatures, no `rust-cache` in a release workflow).
