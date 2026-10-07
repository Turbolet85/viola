## 2026-10-06-local-command-send-outcomes — Path 2's local verdicts as landed
**Section:** §1 Critical paths → Path: confirmed `send` · §4 root bin (the send-confirmation matcher bullet) · §5 CLI (`tests/cli_verify.rs`) · §6 Path 2 → Verification signal (the `local`, Playwright and Human mode bullets) · §7 Fake agent (the Modes bullet)
**Change:**
- §1 and §6: was "`send` does not consume them yet … until then it is `not-delivered`" and "`local`: exit 13 … exit 0 … is owed"; now `local` is exit 0 `{confirmed:false, detail:"unconfirmable", cursor}` with `send-issued` then `send-confirmed{cursor, confirmed:false}`, no `send-refused`, no window, the command's text in no role log line (`send_window_local_command_is_not_presumed_delivered`, `/clear` and `/remote-control` each under `--json`).
- `/clear` on a verified CLI is confirmed by a `session-start` with cause `clear` and a new `agent_session_id` (`send_clear_on_a_verified_cli_is_confirmed_by_its_new_session`), proven on the recorded 2.1.287 `clear-1` variants; the live proof is owed to "First live test and self-drive".
- A slash text off the compiled list stays exit 13 `no-prompt-submitted` (`send_window_slash_text_off_the_list_is_not_delivered`).
- The forced-window pair `send_under_the_paste_hint_on_a_verified_cli` (two cases): `input-not-ready` with nothing typed under the fake agent's paste-hint hold, delivered without it.
- Playwright bullet: the "once "Local-command send outcomes" makes `send` return it" condition is retired; the web half stays owed to `:147`.
- Human mode bullet: `local` prints `[  ] unconfirmable  <name>  local command, no measured post-condition` on stdout, exit 0, nothing on stderr; a post-condition-confirmed `/clear` prints `[RB] read back`.
- §4: the unit cases `send_local_command_` (five functions, 13 cases) and the mirror writer `write_send_unconfirmable`.
- §5: `verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal` beside the turn-screen modal case.
- §7: eight argv options (was seven); `--tag-turn-screen <phase>` added.
**Why:** the chunk landed `send`'s local-command outcomes, the measured readiness-gate reading and the control of the guard before verify's local-command paste.
**Ref:** .andromeda/runs/2026-10-06T23-49-33-wrap/
