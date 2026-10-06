# The rewritten-path warning: the control of its first case that reads the line

## What is pinned
`src/cmd/send.rs`: `parse_name` and `refuse_text_argument` each call `warn_if_rewritten`, which writes
`warning: argument looks like a Git Bash rewritten path` to stderr when the argument has the shape Git for Windows
gives a leading-slash argument. The behaviour existed before this chunk; only the predicate had a test.

`send_rewritten_path_argument_draws_the_warning` (`tests/cli_send.rs`) runs `viola send` with no wrapper, the
rewritten form of `/clear` (the test's literal `rewritten`: a drive letter, the Git install folder under
Program Files, then `clear`) in the name position and again after `builder` in the text position. Each must
exit 2 with an empty stdout, and stderr's first line must be exactly the warning. The literal is not spelled
here: the hygiene read takes a drive path in an evidence file for a host path.

Both readings are the same command (the guard case of `local-command-guard-control.md` rode the same two runs):

```
bash scripts/agent-run.sh run --integration --filter 'test(/verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal|send_rewritten_path_argument_draws_the_warning/)'
```

| reading | started (UTC) | the two call lines, `src/cmd/send.rs:41` and `:46` | result |
|---|---|---|---|
| red | 2026-10-06T23:21:40Z | `// control: warn_if_rewritten(raw);` at both (re-read after the edits, before the run) | exit 1, `"ok":false`, `"passed":0,"failed":2`; this case panicked at the stderr assertion on its first argument set: left `Some("error: invalid value '<the rewritten literal>' for '<NAME>': invalid instance name")`, right `Some("warning: argument looks like a Git Bash rewritten path")` |
| green | 2026-10-06T23:21:56Z | `warn_if_rewritten(raw);` at both (re-read after the restore; no `control:` left in the file) | exit 0, `"ok":true`, `"passed":2,"failed":0`; this case passed in 0.008 s |

The red reading stopped at the name position, the loop's first pass, so the text position was not read red on
its own. Both calls were neutralised in that run, and the green reading covers both positions.

The file's net change in this chunk is the client's `unconfirmable` arm alone; the two calls are back as they
were.
