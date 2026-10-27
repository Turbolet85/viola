# Task — design · 2026-10-27-pinned-copy-race-and-reproducible-builds

## The chunk
Closes the pinned-copy race: two first `viola run`s of one version on a fresh home must not both replace `bin/<key>/viola(.exe)`, and no agent's hook spawn of the pinned path may fail while another start's write of it is in flight.
The fix changes only the concurrency of the absent-copy write in `pin_bytes` / `pin_exe`, reached from `run`'s start and `viola verify`. The hash-before-reuse rule, the per-start plugin rewrite and the home-tree strict-modes check stay unchanged.
It reproduces the race red before the fix and clean after it on the same rig, with the single-instance control alongside. The mechanism (the ETXTBSY hypothesis) is P3's to establish.
It measures, without changing the build, whether two clean release builds of one committed tree give one file, and where they do not, which sections and notes differ. It measures through `scripts/release-check.sh`'s build on the Linux dev host. Any build change is a founder fork at P4.

## Reached files
- crates/viola-state/src/pin.rs · edits pin_bytes, pin_exe (the absent-copy arm)
- crates/viola-state/src/fs.rs · edits replace_private_shared, replace_private (what is held open across the rename)
- src/cmd/run.rs · edits the start's pin_bytes call, pin_companions (Windows x64 conpty/ arm), the plugin/<key>/ rewrite
- src/cmd/verify.rs · edits the pin_exe call
- scripts/release-check.sh · re-reads its release build (profile and CARGO_TARGET_DIR) as the measurement's build
- viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/evidence/unstamped-boot-race.md · re-reads the 6-of-24 / 0-of-24 boot-race measurement
- viola-0.1.0/chunks/2026-10-09-inner-cr-and-crlf-in-a-sent-text/evidence/live-preconditions.md · re-reads §The product build after the whole block

## Needs
- `viola run`'s start output: nothing printed once the child starts. If the pin fix adds a start failure, it is a fixed-message `unable:` line plus its cause's hint on stderr, exit 1, with no path or pid (cli-verify-run)
- `viola verify` output: one static step line per relied row plus the summary line. Any refusal its pin_exe arm gains prints only the fixed `unable:` / `hint:` pair and exits 1, with no step line or summary (cli-verify-run)
- `viola revive`'s start goes through `run`'s start and so reaches the same pin write. A refusal there after a passed preflight prints no `--json` document (cli-verify-run)
- Exit-code phraseology for exit 1: `error: internal error`, or a fixed-message `unable:` refusal with its hint. No new exit code is added (cli-exit-codes)
- Streams: refusals and hints go to stderr, results go to stdout. `viola run`'s terminal belongs to the child's screen, so a pin wait or retry writes no byte to it (cli-output, cli-tokens colour decision order item 2)
- `viola hook`, spawned by the agent from the pinned path, writes nothing to stderr and always exits 0. The race fix must leave this intact (body Surface: cli › Platform-Specific Notes; cli-tokens decision order item 3)
- The Windows pin_companions fallback to the inbox ConPTY stays silent: no refusal, exit change or terminal byte (cli-verify-run, the run start section)

## Standing conventions
- S1 · every new human output line is ASCII only, plain, fixed-message, with no upstream text, path, pid, stack trace or anyhow chain · body Surface: cli › Per-Surface Bans and Platform-Specific Notes
- S2 · every new refusal takes the typed exit code's stderr form and `hint:` line, and under `--json` only its document on stdout · .andromeda/design-system-detail/surface.md (cli-exit-codes)

## Not needed
- The colour palette, surface scale, text hierarchy, border progression and the domain status colours: inputs for a 0.2.0 frontend; the chunk renders nothing
- Typography, spacing, border radius, motion and iconography values, and the token test: inputs for the 0.2.0 frontend
- Brand identity's expression level and design direction: no surface of this chunk has motion or layout
- `viola list`'s strip table, BAY line, unwrapped rows and width truncation (cli-list): the chunk does not touch the board
- `viola send`'s readback mirror and its hints (cli-send): no send path changes
- `viola wait` / `viola last` result lines and the message-mode escaper: no change to wait or last
- The wheel, link, gate and `plugin install` verb lines (cli-verbs): none of these verbs changes
- `viola --help` verb rows and the navigation pattern (cli-help): no verb is added or renamed
- The reproducible-build measurement's findings: they are evidence for the founder and have no design surface
