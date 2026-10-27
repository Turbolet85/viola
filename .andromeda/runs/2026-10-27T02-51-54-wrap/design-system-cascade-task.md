# Task — design-system · 2026-10-27-pinned-copy-race-and-reproducible-builds

## The session
The pinned copy's absent-copy write (`bin/<version>-<hash>/viola`) is now a no-clobber placement of a closed temp file (`viola_state::fs::place_private_new`, new public enum `Placed { Written, Present }`), never a replace; a concurrent first start that loses the placement re-hashes the copy that stands, reusing it when equal and refusing `PinError::HashMismatch` when not.
Measured: the two-instance first-start race (`ETXTBSY`, raw errno 26) failed 4 of 24 boots before and 0 of 24 after; two clean release builds of one tree gave two different files whose loaded sections are all equal (cause not explained, a founder item).
Test-only: the fake agent's `hook` receipt gains `spawn_os_error` on a failed absolute spawn; Path 7's MCP-leg session now boots the default two-instance pair unstamped (the race workaround retired); a Windows plain-home strict-modes oracle now reads the inherited untrusted write grant by its right, whichever SID holds it.
No config key, env var, flag, `detail` code, log line, field, span, refusal, exit code or human output was added or changed; `run.pin_copy` keeps `ok|pinned-hash-mismatch`, a lost placement reading `ok`; `src/cmd/run.rs` and `src/cmd/verify.rs` are unchanged.

## Reached files
- crates/viola-state/src/fs.rs
- crates/viola-state/src/pin.rs
- crates/viola-state/src/strict.rs
- src/bin/viola-fake-agent.rs
- crates/viola-e2e/tests/mcp_tools.rs
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/preconditions.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/repro-unstamped.py
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/race-before.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/race-after.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/red-green.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/repro-builds.sh
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/reproducible-builds.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/path1-phases.md
- viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/operator-pass.md

## Touches
- The tampered pinned-copy refusal's text and output (surface, the `run` start refusals under cli-verify-run and the per-cause lines it points to): re-read as true and unchanged; indirectly, the same `HashMismatch` refusal now also covers a copy a concurrent first start placed with different bytes, with no new line, word or hint.
- `run`'s start refusal set (exit 1, fixed-message `unable:` line plus one hint per cause): no cause added or removed; a lost placement is not a refusal and prints nothing.
- `viola run` prints nothing once the child starts (cli-verify-run, and the colour decision order's `viola run` row): unchanged; the race fix removes failed agent boots, not any terminal byte.
- The exit-code phraseology (exit 1, exit 21, the `--json` documents): no code, document or `detail` code moved.
- `viola verify`'s step-counter output and summary line: `pin_exe`'s caller in verify is unchanged, so no step line, count or relied-row figure moves.
- The home-state and instance-state `unable:` / `hint:` pairs of a refused strict-modes check: the Windows plain-home change is a test oracle only; the product verdict and the printed pair are unchanged and name no SID.
- `viola hook` writes nothing to stderr and always exits 0: the new `spawn_os_error` is on the test-only fake agent's receipt, not on `viola hook` or any human surface.
- The `verified` / `unverified-cli` CLI words (Brand Identity's verified aircraft type, `list`'s CLI column): stamps are keyed by the `claude` CLI version, so measurement B's two `bin/<key>/` keys for one tree reach no printed word.
- The never-print-paths-or-pids floor in errors and hints: re-read as true; nothing new is printed.

## New facts
- (none)

## Untouched
- Color Palette (core colours, surface scale, text hierarchy, semantic and domain status colours, border progression): no colour, contrast rule or status row changed.
- Typography, Spacing, Depth Strategy, Border Radius, Motion and Iconography: no token, face, length, duration or glyph changed.
- The `viola list` strip table, its BAY line, columns, escaping and the unwrapped section: the board's output is unchanged.
- The `viola send` readback mirror and its hints, `[RB]` / `[  ]` / `[/ ]`, and the `unconfirmable` word: no send path changed.
- `viola wait` / `viola last` result lines and the message-mode escaper: unchanged.
- The wheel, link and gate verbs (`pause`, `release`, `release --budget`, `link`, `unlink`, `answer`, `plugin install`) and their lines: unchanged.
- The Navigation Pattern and `viola --help` grouping, and the hidden `mcp` verb: no verb added or moved.
- The Self-Validation Protocol, the Token Test included: no token or value added.
- The Design Decisions Log pointer: this chunk records no design decision.
