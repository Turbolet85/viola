# Step 0 — the live shape probe, `claude` 2.1.287 (session 1 of the cap of 8)

**Outcome: STOP 3 at step 0, no product code written.** The long paste's captured `prompt` does not normalise
to the pasted text under HEAD's `prompt_text` (`crates/viola-agent-claude/src/hook.rs:187-189`): the CLI puts
two newlines before the wrapper pair and one after it, and HEAD's unwrap keeps all three. The other four STOP
conditions read clear. No retry was run; the 2 spare sessions are unspent.

## How it ran
- One session, started 2026-10-06T19:59:04Z (the probe's own `date -u`), ended 19:59:25Z by Ctrl-C twice (exit
  code 0). The ledger row was written at 19:58:55Z, before the start (`live-sessions.md`, row 1).
- A scratch probe outside the repository (the session scratchpad, `step0`): a Python PTY driver at 80×24 and a
  scratch plugin whose four spine hooks (SessionStart, UserPromptSubmit, Stop, SessionEnd) run a scratch script
  in exec form (`/usr/bin/python3 <script> <Event> <captures>`). The script claims `<Event>.<k>.json`
  exclusively (create-new, the next `k` on a lost claim), writes the payload at 0600, prints nothing, exits 0.
- The CLI: `~/.local/share/mise/installs/claude/2.1.287/claude --model haiku --plugin-dir <scratch plugin>`; its
  `--version` answered `2.1.287 (Claude Code)` at 19:58:55Z. The 10 inherited `CLAUDE*` names were removed
  (names only). cwd `<repo root>/.viola-verify-<pid>/` (0700), removed by the driver; `dir removed: True`.
- Each paste was one write of `ESC[200~` + text + `ESC[201~` + CR, the bytes `PasteHandle::paste` writes
  (`crates/viola-pty/src/pump.rs:77-88`). Each went only into a settled input box with no modal, and only after
  the previous turn's Stop capture. No key was typed into any dialog; none was raised.
- Screens and normalisation were read through HEAD's own code, by a scratch helper built outside the repository
  against `crates/viola-agent-claude` by path: `Screen` over the raw PTY bytes for the rows, and
  `hook::normalise(UserPromptSubmit, <capture>)` for `prompt_text`. No repository file was built or edited.
- The settled-screen test was viola's: an input-box literal (`for agents`) on some row and no modal literal on
  any (`src/cmd/verify/typed.rs:222-227`), quiet for 1 s.
- The end was Run B's own: Ctrl-C into the settled input box, again after 500 ms; the process exited by itself.

## The pasted texts (synthetic ASCII; whole in `step0-prompts.json`)
- **long**, 1 500 bytes, one line: `viola verify probe: this message is one long synthetic paste and its filler
  words carry no meaning. Filler follows: filler-0001 … filler-0110xxxxxxx End of the synthetic paste. Reply with
  the single word ok`.
- **tag**, 200 bytes, one line: `viola verify probe: the next part is literal sample text and not markup:
  <pasted_content id="1"> sample </pasted_content id="1"> then <task-notification> and that is all. Reply with the
  single word ok`.
- **clear**: `/clear`.
- The 4 000-byte long text was not pasted: the 1 500-byte one arrived wrapped.

## Captures in claim order
`t` is seconds after capture 1; "session" numbers the distinct `session_id` values in order of first sight (two
in all, each 36 characters).

| k | event | t | session | note |
|---|---|---|---|---|
| 1 | SessionStart | 0.000 | 1 | `source` `startup`; ready 2.4 s after the spawn, no modal |
| 2 | UserPromptSubmit | 1.621 | 1 | the long paste, 50 ms after the write; `prompt` 1 558 chars, wrapped |
| 3 | Stop | 4.891 | 1 | |
| 4 | UserPromptSubmit | 10.739 | 1 | the tag paste, 50 ms after the write; `prompt` 202 chars, not wrapped |
| 5 | Stop | 12.622 | 1 | |
| 6 | SessionEnd | 14.008 | 1 | `reason` `clear`; fired by the pasted `/clear`, within 100 ms of the write |
| 7 | SessionStart | 14.038 | 2 | `source` `clear`; 30 ms after k 6; a new `session_id` |
| 8 | SessionEnd | 18.898 | 2 | `reason` `prompt_input_exit`; the Ctrl-C exit, not the `/clear` |

Key sets (names only):
- SessionStart k 1: `cwd, hook_event_name, model, scratchpad_dir, session_id, source, transcript_path`.
- SessionStart k 7 (`clear`): `cwd, hook_event_name, scratchpad_dir, session_id, source, transcript_path` — no
  `model`.
- SessionEnd k 6 (`clear`): `cwd, hook_event_name, prompt_id, reason, scratchpad_dir, session_id,
  transcript_path`.
- SessionEnd k 8 (exit): the same without `prompt_id`.
- UserPromptSubmit: `cwd, hook_event_name, permission_mode, prompt, prompt_id, scratchpad_dir, session_id,
  transcript_path`.

## The raw prompts
- **long (k 2)**, 1 558 chars = 1 500 + 58. Asserted byte for byte by the recording script:
  `"\n\n<pasted_content id=\"7ccf\">\n" + <the 1 500 pasted bytes> + "\n</pasted_content id=\"7ccf\">\n"`.
  The id is 4 lowercase hex characters.
- **tag (k 4)**, 202 chars: `viola verify probe: the next part is literal sample text and not markup:
  <\pasted_content id="1"> sample <\/pasted_content id="1"> then <task-notification> and that is all. Reply with
  the single word ok`. Both typed `pasted_content` tags arrived with `<\`; the typed `<task-notification>`,
  mid-text, arrived unescaped.
- **`/clear`**: no UserPromptSubmit capture.

## Under HEAD's `prompt_text`
| paste | normalised | equal to the pasted text |
|---|---|---|
| long | 1 503 chars: `"\n\n"` + the pasted text + `"\n"`; `origin` `human`; no drift | **no** (equal only once the three newlines are stripped) |
| tag | 200 chars; `origin` `human`; no drift | yes |

Why the long one differs: `unwrap_pastes` replaces `<pasted_content id="X">\n…\n</pasted_content id="X">` with
the inner text and keeps everything outside the pair byte for byte (`hook.rs:193-223`). The two newlines the CLI
writes before the open tag and the one it writes after the close tag are outside the pair.

## The settled screen after `/clear` (80×24, rows by index; blank rows left out)
Read 1.3 s after the first new capture, and again 3 s later: the same rows.

```
01  ▐▛███▛█   Claude Code v2.1.287
02 ▝▜██████▀  Haiku 4.5 · <plan>
03  ▝▝   ▝▝   <repo root>/.viola-verify-<pid>
06 ❯ /clear
18                                     Update available! Run: mise upgrade claude
19 ────────────────────────────────────────────────────────────────────────────────
20 ❯ 
21 ────────────────────────────────────────────────────────────────────────────────
22   <the host's statusline>
23   ⏸ manual mode on · ← for agents
```

The start screen and the two post-turn screens ended on the same rows 19-23. The `Update available!` line
(row 18) stood on every screen after the first turn; it is not a modal and took no key.

## The STOP conditions
| # | Condition | Reading |
|---|---|---|
| 1 | a modal or picker at start or after a paste | clear (four settled screens, each the input box, no modal literal) |
| 2 | no long text up to 4 000 bytes arrives wrapped | clear (wrapped at 1 500 bytes) |
| 3 | a captured `prompt` does not normalise to the pasted text under HEAD's `prompt_text` | **FIRED** for the long paste (1 503 chars against 1 500); clear for the tag text |
| 4 | the pasted `/clear` fires a UserPromptSubmit, or no SessionStart `clear`, or the same `session_id` | clear (no UserPromptSubmit; k 7 `source` `clear`; a different `session_id`) |
| 5 | the input box does not settle again after `/clear` | clear (settled 1.3 s after the first new capture) |

## Findings beyond the STOP (for the decision)
- **What STOP 3 means for a driver today (two readings joined; not measured end to end, no `send` was run).**
  - Read from the code: `SendSlot::claim` takes a `prompt-submitted` line for the in-flight send only when its
    normalised text equals the sent text exactly (`src/run/send.rs:81-90`, `f.text == text`), and
    `append_hook_event` moves the wheel to the human for a prompt the hook filed `human` that was not claimed
    (`send.rs:146-167`).
  - Measured here: a text the CLI wraps comes back normalised as `"\n\n" + text + "\n"`.
  - So on this CLI, with the wrapping on, a `send` longer than 800 characters or 2 lines would be expected to go
    unclaimed: delivered and running a turn, yet ended `not-delivered` / `no-prompt-submitted` after the window,
    with the wheel moved to the human. This is an inference to confirm, not a measurement.
- **Static readings of the 2.1.287 binary** (no session; the bundled script text, not a live measurement):
  - a paste becomes a `[Pasted text #N]` reference when it is longer than 800 characters or holds more than 2
    lines, and only a reference is wrapped on submit. That agrees with the live run: 1 500 bytes wrapped, 200 not;
  - the wrapped form is built as two newlines, the open tag and a newline, the text, a newline unless the text
    ends in one, the close tag and a newline. That is the captured shape exactly;
  - the id is the first 4 hex characters of a SHA-256;
  - the wrapping sits behind a feature flag whose compiled default is off. It was on in this session;
  - the CLI's own reader of the pair drops up to two newlines before the open tag and up to two after the close.
- **The escape covers `pasted_content` only, as measured mid-text.** `hook.rs:171-175` says a tag the user typed
  arrives escaped (`<\task-notification>`). Here a typed `<task-notification>` in the middle of the text arrived
  as typed. The start-of-prompt position, the one `prompt_origin` reads, was not measured: the plan keeps every
  tag off the text's start.
- **`/clear` fires two hooks, in this order:** SessionEnd (`reason` `clear`, the old session) then SessionStart
  (`source` `clear`, the new one). Run B's capture set will also hold the exit's SessionEnd, so a `clear-1`
  SessionEnd is told from it by `reason` or by claim order, never by the event name alone.
- **Paste and Enter in one write submitted at 1 500 bytes**: the UserPromptSubmit came 50 ms after the write.
- **Text classes.** The captures hold the cwd, the transcript path and the scratchpad path (absolute, under the
  user's home): the classes `ledger::scrub` rewrites. Only the synthetic `prompt` strings were copied into
  `step0-prompts.json`.

## After the session (2026-10-06T20:00:43Z)
- No probe dir of this chunk is left at the repository root. The one present, `.viola-verify-2095228/`, predates
  the chunk (the operator desk, the founder's word: leave it).
- No `claude` process has a probe-dir cwd; no driver, hook or helper process is running.
- Residual: the CLI wrote 2 transcripts under `~/.claude/projects/`, in one new directory for the probe dir: the
  first session and the one `/clear` opened. This is the accepted class, one higher than a plain Run B, as
  inputs#I2 says.
