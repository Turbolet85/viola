# The guard before the local-command paste: its remove-the-guard control

## The guard
`src/cmd/verify/typed.rs`, `framing_turns`, the third check: after the long and the tag-like pastes, the settled
rows must hold the input-box literal and no modal literal, else `PROBE_LOCAL_COMMAND` is not pasted. Its test is
`verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal` (`tests/cli_verify.rs`): the fake agent's
`--tag-turn-screen tag-modal` draws a screen with the trust row beside the input-box row after the tag-like
turn alone, and the test asserts exit 1, `long-paste-wrapper` and `tag-escaping` `pass`, `local-command-clear`
`fail`, and exactly three prompts typed in Run B's span, none of them `/clear`.

Both readings are the same command (the warning case of `rewritten-path-warning-control.md` rode the same two
runs):

```
bash scripts/agent-run.sh run --integration --filter 'test(/verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal|send_rewritten_path_argument_draws_the_warning/)'
```

| reading | started (UTC) | the guard's first line, `src/cmd/verify/typed.rs:278` | result |
|---|---|---|---|
| red | 2026-10-06T23:21:40Z | `if false && !input_box_up(rows.as_deref()) {` (re-read after the edit, before the run) | exit 1, `"ok":false`, `"passed":0,"failed":2`; this case panicked at its first assertion, left `Some(0)`, right `Some(1)`: with the guard off verify pasted `/clear` under the modal screen, all seventeen lines read `pass` and the summary was `stamped 2.1.0  17 pass  0 fail` |
| green | 2026-10-06T23:21:56Z | `if !input_box_up(rows.as_deref()) {` (re-read after the restore, before the run) | exit 0, `"ok":true`, `"passed":2,"failed":0`; this case passed in 3.408 s |

The site was read before the neutralising edit too (2026-10-06T23:21Z): lines 278-280 were
`if !input_box_up(rows.as_deref()) {` / `return;` / `}`, followed by `let starts = count(HookEvent::SessionStart);`.

After the restore `git diff --quiet 11c77f7c9349 -- src/cmd/verify/typed.rs` exits 0: the file ends with no net
change.

The first two checks (inside the loop, before the long and the tag-like pastes) were controlled by the prior
chunk (`2026-10-06-local-command-and-paste-framing-rows/evidence/paste-guard-control.md`). With this one, each of
the three has a red and a green reading.
