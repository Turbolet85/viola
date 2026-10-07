# Harness prompts at the hook layer, read from the driver's own log (counts only)

Research M2's read, run again at implement. It is a reading of the CLI this builder session runs (PATH `claude`),
not of 2.1.287: the log's records carry no CLI version.

## How it was read
- The file: the bridge's log of this builder session, `events.ndjson` under the bridge's own home (outside this
  repository; another tree's build wraps this builder). 503 lines, none torn.
- A scratchpad script, run at 2026-10-07T09:59:08Z (the script's own `date -u`). It keeps no prompt text and prints
  none: only counts and key names.
- **The classification rule.** For every record whose `event` is `UserPromptSubmit`, the string at `data.prompt` is
  tested with `str.startswith` against seven literals, in this order, and counted under the first that matches:
  1. `<agent-message from=`
  2. `<task-notification>`
  3. `<cross-session-message`
  4. `<\cross-session-message`
  5. `Another Claude session sent a message` (the model-layer preface)
  6. `[SYSTEM NOTIFICATION`
  7. `<system-reminder`
- A second count: prompts that hold `<task-notification>` anywhere but at the start.

## The counts
| reading | research M2 (09:03Z) | this run (09:59:08Z) |
|---|---|---|
| `UserPromptSubmit` records | 136 | 140 |
| start with `<agent-message from=` | 75 | 75 |
| start with `<task-notification>` | 28 | 29 |
| start with `<cross-session-message` | 0 | 0 |
| start with `<\cross-session-message` | 0 | 0 |
| start with the model-layer preface | 0 | 0 |
| start with `[SYSTEM NOTIFICATION` | 0 | 0 |
| start with `<system-reminder` | 0 | 0 |
| start with none of the seven | 33 | 36 |
| hold `<task-notification>` anywhere but the start | 0 | 0 |

The four records added since research are this session's own turns: one more task notification and three prompts
that match none of the seven.

## Key names (no values)
- Every prompt sat at `data.prompt` (140 of 140).
- A `UserPromptSubmit` record's keys: `data`, `event`, `name`, `tool`, `ts_ms`; its `data` keys: `cwd`,
  `hook_event_name`, `permission_mode`, `prompt`, `prompt_id`, `scratchpad_dir`, `session_id`, `session_title`,
  `transcript_path`.

## What it settles, and what it does not
- The `<task-notification>` form reaches UserPromptSubmit starting at the tag, unescaped, with no preface, on the
  CLI this session runs. The compiled prefix expects that.
- No cross-session message has reached this session through the hook in the log's life: both cross-session counts
  are 0. The cross-session form is measured by the scratch session's peer message, not here.
- It is the CLI's own injection. Whether a typed `<task-notification>` at a prompt's start arrives escaped is the
  scratch session's sixth reading.
