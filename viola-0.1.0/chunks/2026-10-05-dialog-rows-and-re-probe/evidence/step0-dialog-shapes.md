# Step 0 — the live dialog shape probe, `claude` 2.1.288 (sessions 1-2 of the cap)

**Outcome: STOP 7 at step 0, no product code written.** Run D's bare plan approve (PreToolUse `allow`, no
`updatedInput`, the body `dialog::decision_body` builds today) was followed by a second ExitPlanMode
PermissionRequest and by no ExitPlanMode PostToolUse. Every other STOP condition reads clear.

## How it ran
- A scratch probe outside the repository (session scratchpad, `step0`): a Python PTY driver at 80×24 and a scratch
  plugin whose hooks run a scratch script in exec form (`/usr/bin/python3 <script> <Event> <captures> <run>`). The
  script claims `<Event>.<k>.json` exclusively (create-new, the next `k` on a lost claim), writes the payload, and for
  PreToolUse / PermissionRequest prints the body `dialog::decision_body` builds for the compiled answer of that event's
  ordinal (the byte forms of the six insta snapshots under `crates/viola-agent-claude/src/snapshots/`). Exit 0 on
  every path, nothing on stderr.
- The CLI: `~/.local/share/mise/installs/claude/2.1.288/claude --model haiku --plugin-dir <scratch plugin>`, plus
  Run C `--settings '{"permissions":{"ask":["Bash(touch viola-probe-permission)"]}}'` and Run D
  `--permission-mode plan`. The 10 inherited `CLAUDE*` names removed (names only). Hooks registered: SessionStart,
  UserPromptSubmit, Stop, SessionEnd, PermissionRequest, PostToolUse, and PreToolUse with the matcher
  `AskUserQuestion|ExitPlanMode`.
- Prompts as one bracketed paste + CR each (the bytes `PasteHandle::paste` writes); never a key into a dialog; both
  runs ended by SIGKILL to the child's process group; both dirs (0700) removed by the driver's cleanup.
- Run C answers: PreToolUse 1 → first label of question 1 + annotation note on it + free text for question 2;
  PreToolUse 2 and 3 → first label of every question; PermissionRequest 1 → `allow`.
  Run D answers: PreToolUse 1 → nothing; PermissionRequest 1 → `deny` + the compiled revise message; PreToolUse 2 →
  the bare approve.

## Run C — `<repo root>/.viola-verify-<pid>-dialogs/`, started 2026-10-05T11:52:29Z
No modal at start; ready 2.2 s after spawn. Captures in claim order (`k`, event, tool, `tool_use_id`):

| k | event | tool | tool_use_id | note |
|---|---|---|---|---|
| 1 | SessionStart | | | |
| 2 | UserPromptSubmit | | | prompt 1 (questions) |
| 3 | PreToolUse | AskUserQuestion | toolu_01VHHfb6xzdvbV6rHomC9kyH | 2 questions × 2 options; body printed (answers + annotations) |
| 4 | PostToolUse | AskUserQuestion | toolu_01VHHfb6xzdvbV6rHomC9kyH | 30 ms after k 3; `tool_response` keys `annotations, answers, questions` |
| 5 | Stop | | | 7.1 s after the paste |
| 6 | UserPromptSubmit | | | prompt 2 (parallel) |
| 7 | PreToolUse | AskUserQuestion | toolu_012rtmRUEzHWGmyYXdnKgPXT | 1 question; body printed |
| 8 | PostToolUse | AskUserQuestion | toolu_012rtmRUEzHWGmyYXdnKgPXT | 26 ms after k 7; `tool_response` keys `answers, questions` |
| 9 | PreToolUse | AskUserQuestion | toolu_016juTFsuRBXJKxppDkG3UnZ | 1 question; body printed; 504 ms after k 8 |
| 10 | PostToolUse | AskUserQuestion | toolu_016juTFsuRBXJKxppDkG3UnZ | `tool_response` keys `answers, questions` |
| 11 | Stop | | | 5.2 s after the paste |
| 12 | UserPromptSubmit | | | prompt 3 (permission) |
| 13 | PermissionRequest | Bash | (none: a PermissionRequest carries no `tool_use_id`) | body printed (`allow`) |
| 14 | PostToolUse | Bash | toolu_01H1c8zACsGj8byHz7MT1mkV | `tool_response` keys `interrupted, isImage, noOutputExpected, stderr, stdout` |
| 15 | Stop | | | 2.7 s after the paste |

Where the answers land in the answered question's PostToolUse (k 4), field paths only:
- the chosen label (question 1's first option): `tool_response.answers.<question 1 text>` (and the same under
  `tool_input.answers`; the label also stands in `…questions[0].options[0].label` of both);
- the free text: `tool_response.answers.<question 2 text>` (and `tool_input.answers`);
- the note: `tool_response.annotations.<question 1 text>.notes` (and `tool_input.annotations`).
- PermissionRequest key set (k 13): `cwd, hook_event_name, permission_mode, prompt_id, scratchpad_dir, session_id,
  tool_input, tool_name, transcript_path` — no `tool_use_id`, no `permission_suggestions`.

## Run D — `<repo root>/.viola-verify-<pid>-plan/`, `--permission-mode plan`, started 2026-10-05T11:53:15Z
No modal at start; ready 1.8 s after spawn.

| k | event | tool | tool_use_id | note |
|---|---|---|---|---|
| 1 | SessionStart | | | |
| 2 | UserPromptSubmit | | | `permission_mode` `plan` |
| 3 | PostToolUse | ToolSearch | toolu_01CfVdDj9hwpsMZLCRDhCg78 | |
| 4 | PostToolUse | Write | toolu_01NNY6j4L1fxN6Wg9BuzPXJQ | the CLI's plan file, under `~/.claude/plans/` |
| 5 | PreToolUse | ExitPlanMode | toolu_01De5xavRABQii9ZF69fKDb7 | `tool_input` keys `plan, planFilePath`; no body (the table's "nothing") |
| 6 | PermissionRequest | ExitPlanMode | (none) | 35 ms after k 5; body printed (`deny` + message) |
| 7 | PostToolUse | Write | toolu_01Q4Cz28Mthyv7J6ZvN8g4gt | the revised plan file |
| 8 | PreToolUse | ExitPlanMode | toolu_01LAdi4nb8hLbmav79gn9FbV | the re-plan: revise took effect; bare approve printed |
| 9 | PermissionRequest | ExitPlanMode | (none) | **35 ms after k 8: the approve did not take effect** |

No ExitPlanMode PostToolUse came; the run was ended by a kill at 11:54:08Z (about 27 s after k 9, the plan dialog
presumably rendered and left untouched).

## The STOP conditions
| # | Condition | Reading |
|---|---|---|
| 1 | a start shows a modal | clear (both runs) |
| 2 | a run's dialog is not raised | clear (Run C: 3 AskUserQuestion + 1 Bash PermissionRequest; Run D: 2 ExitPlanMode) |
| 3 | a hook-answered AskUserQuestion still followed by a PermissionRequest for it, or by no PostToolUse | clear (3 of 3 answered, 3 of 3 followed by their own PostToolUse, no AskUserQuestion PermissionRequest) |
| 4 | the answered question's PostToolUse carries neither the label nor the free text | clear (both, `tool_response.answers`) |
| 5 | the annotation text in no field of that PostToolUse | clear (`tool_response.annotations.<q>.notes`) |
| 6 | the revise not followed by a fresh ExitPlanMode PreToolUse | clear (k 6 → k 8) |
| 7 | the bare approve not followed by the ExitPlanMode PostToolUse | **FIRED** (k 8 → a PermissionRequest, k 9; no PostToolUse) |
| 8 | the `touch` raises no PermissionRequest under the `--settings` ask rule | clear (k 13, over the host's `Bash(*)` allow) |
| 9 | the parallel prompt yields fewer than two AskUserQuestion PreToolUse captures | clear (k 7, k 9, distinct ids) |

## Findings beyond the STOP (for the decision)
- **The approve body.** The 2.1.288 binary, read statically: on a hook `allow` the CLI runs the tool's own permission
  check with `{hookUpdatedInput: <the hook's updatedInput>}`. An `ask` from that check sends the call to the full
  permission pipeline (the dialog). A user-interaction tool is reported satisfied only "via updatedInput". The
  prototype's approve, which carried `updatedInput {plan, planFilePath}`, took effect 4 of 4 times on 2.1.287
  (inputs#I2). viola's bare approve is the measured failure. architecture [CLI Version Compatibility] says only "S7:
  ExitPlanMode approves only through PreToolUse", so an approve body that echoes the tool input as `updatedInput`
  stays inside it. The plan lists `dialog.rs` and its snapshots for "a body step 0 forces". STOP 7 bars writing it here.
- **Concurrency.** The two "parallel" calls ran one after the other: k 9's PreToolUse came 504 ms after k 8's
  PostToolUse. `measured.dialog_probe.parallel_both_before_first_post` would read `false`. The row as planned
  (two PreToolUse with distinct ids, each followed by its own PostToolUse) passes.
- **No `tool_use_id` on a PermissionRequest.** The plan-approve-revise check ("no PermissionRequest(ExitPlanMode)
  after PreToolUse 2") can only be read by claim order, never by id.
- **A new residual class under `~/.claude`.** Run D's model wrote its plan through the Write tool into
  `~/.claude/plans/` twice (the CLI's own plan file: k 4, k 7). That is the CLI's write, not viola's, but it is beyond
  the +2 transcripts the founder accepted (inputs#I3), and each verify run would add one per version. The 2.1.288
  binary has a `plansDirectory` setting ("Custom directory for plan files, relative to project root", refused outside
  it), which a `--settings` literal could point into Run D's own 0700 dir. Not measured.
- **The Run C `touch`** ran in the 0700 dir, which was removed after. No `viola-probe-permission` file exists at the
  repository root.
- **Text classes.** The captures hold the cwd (absolute, under the user's home), the transcript path, the scratchpad
  path and the plan file path. These are the classes `ledger::scrub` rewrites and `unclean` refuses.

## Re-run — Run D, the founder's ruling on STOP 7 (session 3, started 2026-10-05T12:45:15Z)
The ruling (live, 2026-10-05, via the overseer's AskUserQuestion, relayed by the operator; `live-sessions.md`): the
approve body becomes PreToolUse `allow` + `updatedInput` = the tool's own input, unchanged. Run D's `--settings` carries
`{"plansDirectory": "<Run D dir>/plans"}` (absolute, a 0700 `plans/` created inside the run's 0700 dir). One session.

| k | event | tool | tool_use_id | note |
|---|---|---|---|---|
| 1 | SessionStart | | | ready 1.9 s after spawn, no modal |
| 2 | UserPromptSubmit | | | |
| 3 | PostToolUse | Write | toolu_01FuguqyAC1yuw251tS2z5xT | the plan file, in the run's own `plans/` |
| 4 | PostToolUse | ToolSearch | toolu_019wxDH41hemgLGiAhbeo36G | |
| 5 | PreToolUse | ExitPlanMode | toolu_01Ve1GcWpv9ncoSV5k3Kg26Q | `planFilePath` in the run's own `plans/`; no body |
| 6 | PermissionRequest | ExitPlanMode | (none) | body printed (`deny` + message) |
| 7 | PostToolUse | Write | toolu_01UC7XS2bZ53aUfiDo9rJ9Vq | the revised plan file, same dir |
| 8 | PreToolUse | ExitPlanMode | toolu_01XjHh7nHLeEGkAb3hcRzarA | the re-plan; echoed approve printed |
| 9 | PostToolUse | ExitPlanMode | toolu_01XjHh7nHLeEGkAb3hcRzarA | **34 ms after k 8: the approve took effect**; `tool_response` keys `filePath, isAgent, plan`; `tool_input` an empty object |

- STOP 6 clear (k 6 → k 8); STOP 7 clear (k 8 → k 9, no PermissionRequest after k 8).
- `~/.claude/plans/`: 17 files before the spawn, 17 after the kill, 0 new, 0 changed (names and mtimes compared). The
  run's own `plans/` held 1 file at the kill and went with the dir. `plansDirectory` keeps the CLI's plan file
  inside the probe dir.
- Ended by kill (exit -9) at 12:45:24Z; the dir removed.
