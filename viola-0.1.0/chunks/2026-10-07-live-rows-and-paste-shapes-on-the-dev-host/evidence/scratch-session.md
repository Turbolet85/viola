# Live session 1 — the scratch session, `claude` 2.1.287 (start 1 of the cap of 10)

The readings below are the lines of `scratch-session.ndjson`; the texts sent and the raw prompts are whole in
`scratch-prompts.json`.

**Outcome: no STOP of the live work. STOP 6 (a report) fires once, and step 7 meets STOP 5.**
- Three of the five paste shapes do not normalise to the bytes sent under the base commit's `prompt_text`: a paste
  followed by typed text, and both texts whose last byte is a newline.
- The cross-session message arrived in the plain compiled form, under three attributes.
- A `<task-notification>` typed at a prompt's very start arrives as typed and is filed `harness`.
- The 2.1.287 hook's environment holds 12 `CLAUDE*` names; 4 are outside the eleven.

## How it ran
- One session, started 2026-10-07T10:05:36Z (the driver's own `date -u`), ended by Ctrl-C twice at 10:06:24Z, exit
  code 0. The ledger line was written at 10:05:27Z, before the start (`live-sessions.ndjson`, `n` 1).
- A scratch probe outside the repository (the session scratchpad), in the form of the prior chunk's step 0: a Python
  PTY driver at 80×24 and a scratch plugin whose four spine hooks (SessionStart, UserPromptSubmit, Stop, SessionEnd)
  run one scratch script in exec form. The script claims `<Event>.<k>.json` exclusively at 0600, prints nothing,
  exits 0. The earlier chunk's driver was read whole before its parts were reused.
- The founder's second answer (inputs#I3): on its first invocation the same script wrote one file holding the sorted
  names of its environment that start with `CLAUDE`. It reads no value into any file.
- The CLI: the 2.1.287 binary by path (its `--version` answered `2.1.287 (Claude Code)` at 10:05:27Z) with `--model
  haiku --plugin-dir <scratch plugin>`. The 10 inherited `CLAUDE*` names and `VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`
  were removed from its environment, names only. cwd a fresh 0700 `<repo root>/.viola-verify-<pid>/`, removed by the
  driver: `dir_removed` true.
- Screens and normalisation were read through the tree's own code, by a scratch helper built outside the repository
  against `crates/viola-agent-claude` by path at the base commit `e7e5bf75db8a`: `Screen` over the raw PTY bytes for
  the rows, `hook::normalise(UserPromptSubmit, <capture>)` for `text` and `origin`.
- **The wait before every shape:** the previous turn's Stop is captured and the screen has been quiet 300 ms,
  counted from the later of its last output and the driver's sighting of the Stop, with the input-box literal
  (`for agents`) on a row and no modal literal on any. A quiet screen with no literal kept waiting, 60 s at most.
- No key was typed into any dialog; none was raised. No modal literal was on any row at any chunk of the drained
  bytes, and the screen model never poisoned.
- The driver was first run once against a stand-in script (no CLI, no session) at 10:03:34Z: a stand-in that holds a
  hint for 4 s after a long paste and fires one more UserPromptSubmit by itself 2 s after its sixth turn. The waits,
  the hold and the record read as built (four hint timings of 4.000 s to 4.301 s, the hold's capture 1.7 s in).

## Every settle (how long it took, what it settled on)
| settle | counted from | took | settled on |
|---|---|---|---|
| ready | the spawn | 1.306 s | `for agents` |
| after typed-then-paste | its Stop | 5.982 s | `for agents` |
| after paste-then-typed | its Stop | 6.046 s | `for agents` |
| after two-pastes | its Stop | 6.919 s | `for agents` |
| after long-ending-newline | its Stop | 6.678 s | `for agents` |
| after short-ending-newline | its Stop | 0.302 s | `for agents` |
| after typed-task-notification-at-start | its Stop | 0.558 s | `for agents` |
| after the peer message's turn | its Stop | 0.308 s | `for agents` |

Each bound was 60 s (120 s for the first). The four long shapes waited out the paste hint.

## The six shapes
A long text is 1 500 ASCII bytes on one line; "typed" bytes went in one at a time, 10 ms apart, outside any bracketed
paste; separate writes of one shape were 300 ms apart. Every pair of the session carried the one id `eec9`.

| id | what was written | the raw `prompt` | wrapped | normalised `text` against the bytes sent | `origin` |
|---|---|---|---|---|---|
| typed-then-paste | 40 typed bytes, then one paste of a long text with CR in the same write | 1 598 chars: the typed bytes, two newlines, the pair, one newline | yes, 1 pair | **equal** (1 540 = 1 540) | `human` |
| paste-then-typed | one paste of a long text, 39 typed bytes, CR | 1 598 chars: two newlines, the pair, **two** newlines, the typed bytes | yes, 1 pair | **not equal**: 1 540 against 1 539, first difference at index 1 500 (one newline kept between the paste and the typed bytes) | `human` |
| two-pastes | two pastes of two different long texts, CR with the second | 3 116 chars: two newlines, pair, three newlines, pair, one newline; both pairs under the same id | yes, 2 pairs | **equal** (3 000 = 3 000) | `human` |
| long-ending-newline | one paste of a long text whose last byte is LF, CR in the same write | 1 557 chars: two newlines, the open tag and a newline, the 1 500 bytes, the close tag and a newline. No newline was added before the close tag | yes, 1 pair | **not equal**: 1 499 against 1 500 (the text's own last newline is gone) | `human` |
| short-ending-newline | one paste of a 96-byte text whose last byte is LF, CR in the same write | 95 chars: the text without its last newline | no | **not equal**: 95 against 96 | `human` |
| typed-task-notification-at-start | one paste of a 151-byte text that opens `<task-notification>`, CR in the same write | 151 chars, equal to the bytes sent: the tag is not escaped | no | equal | **`harness`** |

- UserPromptSubmit came 23 ms to 47 ms after the last write of each shape; each turn's Stop 1.0 s to 2.4 s later.
- The UserPromptSubmit key set, every capture: `cwd`, `hook_event_name`, `permission_mode`, `prompt`, `prompt_id`,
  `scratchpad_dir`, `session_id`, `transcript_path`.

### What the three unequal readings mean
- **paste-then-typed.** After a pair that is followed by typed text the CLI writes two newlines, not one. The base
  commit's unwrap takes one. This is fixed inside `unwrap_pastes` in step 7 (`live-shape-red-green.md`).
- **long-ending-newline.** The research hypothesis (M6) holds on the live CLI: the CLI adds no newline before the
  close tag when the pasted text already ends in one. The raw prompt is then byte for byte the prompt a paste of
  the same text **without** its last newline produces, so nothing inside the hook can tell the two apart.
- **short-ending-newline.** The loss is not the wrapper's: an unwrapped text loses its last newline too, before the
  hook is called.
- For both newline shapes the claim compares the normalised text with the sent text exactly, so a `send` of a text
  ending in a newline goes unclaimed. Run end to end on both hint homes (`hint-window.md`): exit 13
  `not-delivered` / `no-prompt-submitted`, the prompt filed `human`, the wheel moved to the human, the turn run.
  A fix is outside `hook.rs` (the send side or the claim): **STOP 5**, reported, nothing built.

## The paste hint in this session (four more timings)
The footer lost the input-box literal within 5 ms to 15 ms of each long paste and showed `paste again to expand`.
"Stop" is the driver's sighting of the Stop capture.

| shape | paste → Stop | paste → the literal's return | Stop → the literal's return |
|---|---|---|---|
| typed-then-paste | 2.512 s | 8.023 s | 5.511 s |
| paste-then-typed | 2.259 s | 8.005 s | 5.746 s |
| two-pastes | 1.687 s | 8.306 s from the first paste, 8.006 s from the second | 6.619 s |
| long-ending-newline | 1.627 s | 8.005 s | 6.378 s |

- The hint is a timer of 8.0 s from the last long paste; a second long paste restarts it.
- Typed bytes alone also take the literal off the footer: it was gone 9 ms after the first typed byte of
  typed-then-paste, before any paste.
- A short paste blanks the literal for about 15 ms and draws no hint.

## The peer message (the founder's first answer, inputs#I3)
- This session's peer listing held 9 peers before the scratch start (read at 10:05:27Z) and 10 at the hold. Exactly
  one was new, and it read `idle`: its name carried the probe dir's own name.
- One message was sent to it, once, with the fixed text and nothing else:
  `viola probe: a synthetic cross-session message and nothing else. Reply with the single word ok`.
- The scratch session's UserPromptSubmit fired 6.7 s into the hold; its turn's Stop 1.6 s later.
- **The raw start, every attribute value replaced:**
  `<cross-session-message from="<value>" from-name="<value>" from-mode="<value>">`, then a newline, the sent text, a
  newline and `</cross-session-message>`. 239 chars in all.
- Attribute names: `from`, `from-name`, `from-mode`. The static read of the binary (research M3) named the first
  two; `from-mode` is new here.
- It matched the **plain** compiled form (`<cross-session-message`), not the escaped one. `hook::normalise` files it
  `harness`.
- So on 2.1.287 on this host the hook-layer prompt is unescaped. The escaped form (`<\cross-session-message`, the
  relayed measurement) was not seen; both prefixes stay compiled, unchanged.
- Messages that session sent back to this one: 0 arrived. Nothing was acted on.

## The names (the founder's second answer, inputs#I3)
The scratch hook's first invocation was SessionStart. The child was started with every inherited `CLAUDE*` name
removed, so these are the names 2.1.287 itself hands a hook on this host. Names only.

- **12 names:** `CLAUDECODE`, `CLAUDE_CODE_CHILD_SESSION`, `CLAUDE_CODE_ENTRYPOINT`, `CLAUDE_CODE_MESSAGING_SOCKET`,
  `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_SESSION_ATTENDED`, `CLAUDE_CODE_SESSION_ID`, `CLAUDE_ENV_FILE`,
  `CLAUDE_PID`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PROJECT_DIR`.
- **Outside the eleven (STOP 6, a report): 4** — `CLAUDE_ENV_FILE`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PLUGIN_ROOT`,
  `CLAUDE_PROJECT_DIR`. Read against a literal copy of the eleven typed into the reading script, never the product's
  constant.
- Of the eleven, absent from this hook's environment: `CLAUDE_CODE_BRIDGE_SESSION_ID`, `CLAUDE_CODE_EXECPATH`,
  `CLAUDE_EFFORT`.
- Beside it, the one path already ruled (research M5): `viola run`'s `process-start{subject:"claude-child"}` line of
  both hint runs lists the names it removed from its own inherited environment, which are the names PATH `claude`
  hands this builder session: 10 names, all on the eleven (`CLAUDECODE`, `CLAUDE_CODE_CHILD_SESSION`,
  `CLAUDE_CODE_ENTRYPOINT`, `CLAUDE_CODE_EXECPATH`, `CLAUDE_CODE_MESSAGING_SOCKET`, `CLAUDE_CODE_MESSAGING_TOKEN`,
  `CLAUDE_CODE_SESSION_ATTENDED`, `CLAUDE_CODE_SESSION_ID`, `CLAUDE_EFFORT`, `CLAUDE_PID`). That is a tool's
  environment on PATH `claude`, not a 2.1.287 hook's: the two sets differ.
- `IDENTITY_FLOOR` and `HARNESS_PREFIXES` are unchanged by this chunk, whatever was measured.

## After the session (2026-10-07T10:06:34Z)
- The driver's probe dir is gone; the eleven dirs standing before are the same eleven (`probe-dir-census.md`).
- No `claude` process has a probe-dir cwd; no driver, hook or helper process is running.
- Residual: the CLI wrote 1 transcript under the user's Claude projects dir, in one new directory for the probe
  dir. This is the accepted class.
- The raw PTY bytes, the captures and the driver's log stay in the session scratchpad, outside the repository. Only
  `prompt` fields, key names and bare `CLAUDE*` names were copied here.
