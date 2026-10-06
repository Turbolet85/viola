# Step 6 — the wrapped send's one-shot control

Both readings are the same command, run from the repository root on the Linux dev host, after the second record
round put `fixtures/claude/2.1.287/UserPromptSubmit.paste-1.json` in place (`record-round-green.md`):

```
bash scripts/agent-run.sh run --integration --filter 'test(/send_long_text_wrapped_by_the_cli_is_confirmed/)'
```

The case (`tests/cli_send.rs`) takes the 1 500-byte text out of the recorded `paste-1` prompt by its own literal
frame, boots a wrapper over a fake agent with `--framing`, and sends that text. The agent answers with the recorded
prompt, its bytes unchanged: two newlines, the pair (id `7602`), one newline.

| reading | started (UTC) | `unwrap_pastes`, the matched-pair arm (`crates/viola-agent-claude/src/hook.rs:205-210`) | result |
|---|---|---|---|
| red | 2026-10-06T21:16:26Z | HEAD's body: `out.push_str(&rest[..start]); out.push_str(inner); rest = tail;` (re-read after the edit; the whole function compared equal to `git show HEAD:` by `diff`) | exit 1, `"passed":0,"failed":1`; `FAIL [  10.500s]`; the assertion at `tests/cli_send.rs:516`: left `Some(13)`, right `Some(0)`, stdout `{"v":1,"refusal":"not-delivered","detail":"no-prompt-submitted"}` |
| green | 2026-10-06T21:16:50Z | the change: the lead loses a final two-newline run, the tail a first newline (re-read after the restore; the file's diff against HEAD is 62 added and 14 deleted lines again, as before the control) | exit 0, `"passed":1,"failed":0`; `PASS [   0.452s]` |

- With the unwrap at its HEAD body the wrapper's normalised text is two newlines, the text and one newline, so the
  in-flight send is not claimed: it waits out the 10 s confirmation window and ends `not-delivered` /
  `no-prompt-submitted`, exit 13. That is step 0's inference (`step0-shapes.md`, "What STOP 3 means for a driver
  today") measured through the fake agent's replay of the live shape.
- The red is a failed assertion at 10.5 s, under the 20 s kill of the `cli_send` binary: not a kill.
- Only the one integration case was run on the neutralised tree. The neutralising edit was made and reverted
  through the editor, each time re-read at the site before the run.
