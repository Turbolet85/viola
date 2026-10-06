# Step 11 — the rehearsal of Run B's sequence, `claude` 2.1.287 (session 7 of the cap of 12)

**Outcome: no STOP. All five conditions read clear.** The four compiled texts were pasted in Run B's order, each
turn was captured, each prompt normalises to its pasted text under the tree's `prompt_text`, and `/clear` opened a
new session with no UserPromptSubmit. After the long paste the paste hint stood in place of the input-box literal
for **8.0 s from the paste**, which was **6.5 s after that turn's Stop**: longer than the 5 s the tree's settle
waits, so the first record round's reading is reproduced with its cause measured. The session was not re-run.

## How it ran
- One session, started 2026-10-06T21:05:45Z (the probe's own `date -u`), ended by Ctrl-C twice at 21:06:00Z (exit
  code 0, 21:06:01Z). The ledger row was written at 21:05:28Z, before the start (`live-sessions.md`, row 7).
- A scratch probe outside the repository (the session scratchpad, `rehearsal`), in step 0's form
  (`step0-shapes.md`, "How it ran"): a Python PTY driver at 80×24 and a scratch plugin whose four spine hooks
  (SessionStart, UserPromptSubmit, Stop, SessionEnd) run step 0's scratch script in exec form. The script claims
  `<Event>.<k>.json` exclusively, writes the payload at 0600, prints nothing, exits 0.
- The CLI: `~/.local/share/mise/installs/claude/2.1.287/claude --model haiku --plugin-dir <scratch plugin>`; its
  `--version` answered `2.1.287 (Claude Code)` at 21:05:28Z. The 10 inherited `CLAUDE*` names were removed (names
  only). cwd `<repo root>/.viola-verify-<pid>/` (0700), removed by the driver; `dir removed: True`.
- The four texts are the tree's own constants, printed by a scratch helper built outside the repository against
  `crates/viola-agent-claude` by path: `PROBE_PROMPT` (49 bytes), `PROBE_LONG_PASTE` (1 500), `PROBE_TAG_PASTE`
  (200), `PROBE_LOCAL_COMMAND` (6). The long and the tag-like text are byte for byte step 0's
  (`step0-prompts.json`).
- Each paste was one write of `ESC[200~` + text + `ESC[201~` + CR. No key was typed into any dialog; none was
  raised.
- **The rule each paste after the first waited on** (the rule step 12 gives Run B): the previous turn's Stop is
  captured, and the screen, read through the tree's own `Screen` over the raw PTY bytes, has been quiet for 300 ms
  — counted from the later of its last output and the driver's sighting of the Stop capture — with the compiled
  input-box literal (`for agents`) on a row and no modal literal on any. A quiet screen with no literal kept
  waiting, up to 60 s.
- Screens and normalisation were read through the working tree's code (the same helper): `Screen` for the rows,
  `hook::normalise(UserPromptSubmit, <capture>)` for `prompt_text`. No repository file was built or edited.
- The driver was first run once against a stand-in script (no CLI, no session) that holds a hint for 6 s after a
  long paste, to show its waits and its record before the one live session.
- The end was Run B's own: Ctrl-C into the settled input box, again after 500 ms; the process exited by itself.

## Captures in claim order
`t` is seconds after capture 1 (the hook script's own clock); "session" numbers the distinct `session_id` values
in order of first sight (two in all, each 36 characters).

| k | event | t | session | note |
|---|---|---|---|---|
| 1 | SessionStart | 0.000 | 1 | `source` `startup`; 0.9 s after the spawn; the input box quiet 1.7 s after the spawn, no modal |
| 2 | UserPromptSubmit | 0.847 | 1 | the probe prompt, 38 ms after the write; `prompt` 49 chars, as pasted |
| 3 | Stop | 2.380 | 1 | |
| 4 | UserPromptSubmit | 3.038 | 1 | the long paste, 33 ms after the write; `prompt` 1 558 chars, wrapped, id `dead` |
| 5 | Stop | 4.501 | 1 | |
| 6 | UserPromptSubmit | 11.346 | 1 | the tag-like paste, 27 ms after the write; `prompt` 202 chars, not wrapped |
| 7 | Stop | 12.621 | 1 | |
| 8 | SessionEnd | 12.989 | 1 | `reason` `clear`; 37 ms after the write of `/clear` |
| 9 | SessionStart | 13.017 | 2 | `source` `clear`; 28 ms after k 8; a new `session_id` |
| 10 | SessionEnd | 14.164 | 2 | `reason` `prompt_input_exit`; the Ctrl-C exit, not the `/clear` |

- The key sets are step 0's (`step0-shapes.md`): the `clear` SessionStart has no `model`; the `clear` SessionEnd
  carries `prompt_id`, the exit's does not. Stop carries `background_tasks`, `last_assistant_message`,
  `permission_mode`, `prompt_id`, `session_crons`, `stop_hook_active` beside the common keys.
- The long prompt is exactly `"\n\n<pasted_content id=\"dead\">\n"` + the 1 500 pasted bytes +
  `"\n</pasted_content id=\"dead\">\n"`: the frame of step 0 (`7ccf`) and of the first round (`31a3`), a third id.
- The tag-like prompt is the pasted text with both typed `pasted_content` tags escaped (`<\`, `<\/`) and the
  typed `<task-notification>` as typed: step 0's shape.

## Under the tree's `prompt_text`
| paste | normalised | equal to the pasted text | origin | drift |
|---|---|---|---|---|
| probe | 49 chars | yes | `human` | none |
| long | 1 500 chars | **yes** (the unwrap removes the frame) | `human` | none |
| tag | 200 chars | yes | `human` | none |

## The timings (seconds)
"Stop" below is the driver's sighting of the Stop capture, 5 ms to 17 ms after the hook script's own stamp; that
is the instant Run B counts from.

| turn | paste → UserPromptSubmit | UserPromptSubmit → Stop | the literal at the Stop | Stop → literal drawn again | Stop → literal on a quiet screen | paste → literal on a quiet screen |
|---|---|---|---|---|---|---|
| probe | 0.038 | 1.533 | on screen | — | 0.588 | 2.177 |
| **long** | 0.033 | 1.463 | **absent**, the hint in its place | **6.497** | **6.797** | **8.305** |
| tag | 0.027 | 1.275 | on screen | — | 0.309 | 1.616 |

- **The local command**: SessionEnd 0.037 s and SessionStart 0.065 s after the paste; the driver saw the first of
  them 0.081 s after the paste; the input box was back on a quiet screen 0.596 s after that sighting, 0.677 s
  after the paste.
- Against step 0 and the first round, the same long turn: 3.4 s (step 0), 1.5 s (the round), 1.5 s here. Step 0
  read the input box settled 5.8 s after that Stop with 1 s of quiet; here it is 6.8 s with 300 ms of quiet. The
  turn was shorter, so more of the hint's 8 s fell after the Stop.

## The footer row over the raw bytes
The raw PTY bytes (15 821 in all) were fed chunk by chunk, each with its arrival instant, through the tree's
`Screen`; a line below is a change of the literal reading or of row 23. `t` is seconds after the long paste.

| t | row 23 | input-box literal on a row | what happened |
|---|---|---|---|
| −3.339 | (blank) | no | the first output |
| −2.907 | `⏵⏵ auto mode on (shift+tab to cycle) · ← for agents` | yes | the start screen, for 62 ms |
| −2.845 | `⏸ manual mode on · ← for agents` | yes | |
| −2.196 | | | the probe prompt pasted |
| −2.188 | `⏸ manual mode on` | no | for 30 ms after the paste |
| −2.158 | `⏸ manual mode on · ← for agents` | yes | the probe's UserPromptSubmit |
| −0.624 | | | the probe's Stop |
| 0.000 | | | **the long text pasted** |
| +0.005 | `paste again to expand` | **no** | the hint replaces the footer |
| +0.033 | | | the long UserPromptSubmit |
| +1.496 | | | the long Stop |
| +8.005 | `⏸ manual mode on · ← for agents` | **yes** | the footer is drawn again: the hint stood 8.000 s |
| +8.305 | | | 300 ms of quiet with the literal; the tag-like text pasted 10 ms later |
| +8.321 | `⏸ manual mode on` | no | for 18 ms after the paste |
| +8.339 | `⏸ manual mode on · ← for agents` | yes | the tag UserPromptSubmit at +8.342 |
| +9.616 | | | the tag Stop |
| +9.947 | | | `/clear` pasted |
| +9.965 | `⏸ manual mode on` | no | for 15 ms after the paste |
| +9.980 | `⏸ manual mode on · ← for agents` | yes | |
| +10.638 | `Press Ctrl-C again to exit` | no | the first Ctrl-C |

- **The hint is a timer from the paste**: 8.000 s between the two footer draws, with the turn's Stop 1.5 s into
  it. No byte was drawn between the Stop's last output and the footer's return that held the literal. Step 0's
  inference (about 8 s, from one run) is now measured.
- No modal literal was on any row at any chunk.
- Each paste blanks the `· ← for agents` half of the footer for 15 ms to 30 ms. Run B never reads the screen in
  that window: its waits begin at a Stop or at a capture.

## The five settled screens (80×24, blank rows left out)
Each is the screen at the instant the rule above held. Rows 19-23 are the same on all five:

```
19 ────────────────────────────────────────────────────────────────────────────────
20 ❯ 
21 ────────────────────────────────────────────────────────────────────────────────
22   <the host's statusline>
23   ⏸ manual mode on · ← for agents
```

- **ready**: the banner on rows 1-3 (`Claude Code v2.1.287`, `Haiku 4.5 · <plan>`,
  `<repo root>/.viola-verify-<pid>`); row 20 holds the CLI's placeholder suggestion.
- **after the probe turn**: row 6 `❯ viola verify probe: reply with the single word ok`, row 8 `● ok`, row 10 the
  turn's duration line.
- **after the long turn**: rows 0-13 the pasted text's tail, row 15 `● ok`, row 17 the duration line, row 18
  `Update available! Run: mise upgrade claude` (not a modal; it took no key, as at step 0).
- **after the tag-like turn**: the long text's tail scrolled up to rows 0-9, rows 11 and 15 `● ok`, rows 13 and
  17 the duration lines, row 18 the update line. No row holds the tag-like prompt's text at that instant.
- **after `/clear`**: the banner on rows 1-3, row 6 `❯ /clear`, row 18 the update line.

## The STOP conditions
| # | Condition | Reading |
|---|---|---|
| 1 | a modal or a picker on any settled screen | clear (five settled screens, each the input box; no modal literal at any chunk) |
| 2 | after an added turn the literal is not back on a quiet screen within 60 s of that turn's Stop | clear (6.797 s after the long turn's Stop; 0.309 s after the tag-like turn's) |
| 3 | a captured `prompt` does not normalise to its pasted text under the tree's `prompt_text` | clear (the probe prompt, the long text and the tag-like text each normalise to the pasted text) |
| 4 | the local command fires a UserPromptSubmit, or no SessionStart `clear`, or keeps the `session_id` | clear (no UserPromptSubmit; k 9 `source` `clear`; a different `session_id`) |
| 5 | a pasted text gets no capture within 30 s of its paste | clear (27 ms to 38 ms for the three prompts; 37 ms for the local command's first hook) |

Recorded, not a STOP: the hint stood 8.0 s (step 0: about 8 s, inferred); the paste id is `dead`.

## What it says about step 12
- With the tree's settle, Run B reads its rows 5 s after the long turn's Stop. Here the literal came back 6.5 s
  after it. So the guard would again have pasted nothing more: the first round's `15 pass  2 fail` is this
  reading.
- Under the rule the rehearsal ran, which is step 12's, all four texts went in and every post-condition the three
  rows read held. The wait cost one long turn about 8.3 s from its paste; the other waits ended 0.3 s to 0.6 s
  after their Stop or capture.
- A reading for the wrap to route, measured here for the first time and not end to end (no `send` was run): for
  8.0 s after a long paste this CLI's footer holds no input-box literal, 6.5 s of them after the turn's Stop. The
  product's readiness gate reads the same literal with a 5 s maximum (architecture [Screen Model]).

## After the session (2026-10-06T21:06:30Z)
- No probe dir of this chunk is left at the repository root. The one present, `.viola-verify-2095228/`, predates
  the chunk (the operator desk, the founder's word: leave it).
- No `claude` process has a probe-dir cwd (`pgrep -x claude`, each cwd read); no driver, hook or helper process
  is running.
- Residual: the CLI wrote 2 transcripts under `~/.claude/projects/`, in one new directory for the probe dir: the
  first session and the one `/clear` opened. This is the accepted class (inputs#I2).
- The raw PTY bytes, the chunk index, the captures and the driver's log stay in the session scratchpad, outside
  the repository; nothing of them is copied here but the readings above.
- Sessions: 7 of the cap of 12 are used. The 5 left are the second record round's.
