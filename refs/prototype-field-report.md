# Prototype field report — the first dogfood of a session-drives-session bridge

**What this is.** Viola's throwaway prototype (`viola-lab/prototype`, not a repo) drove the Viola builder
session for 18 hours straight, 2026-09-23 19:40 → 2026-09-24 13:41 (host clock, WEDT), under Claude Code
**2.1.280** on **Windows 11 Pro 10.0.26200**, Git Bash as the driver's shell. One `viola run viola-builder`
wrapper (pid 57276) hosted the builder the whole time — through security, design, tests, obs, a11y, route,
setup-project and the first five phase-loop chunks — and one overseer session drove it through the
prototype's CLI (`send`, `wait`, `answer`, `list`). Every number below is measured from the wrapper's
`events.ndjson` or the overseer's transcript; nothing is estimated. Status: **input to Epoch 2–3 chunks**
(readiness gate, confirmed send, wait/last, dialog answers, wheel, CLI contract). Pair it with the brief's
§4.1 S1–S8 and §4.2 M1–M7 (the earlier, shorter measurements).

## 1. Volume and outcomes (from `events.ndjson`)

| Measure | Count | Note |
|---|---|---|
| Wrapper lifetime | 18 h 01 min, 1 run, 0 restarts | pid 57276 from start to report |
| `send` (typed = bracketed paste + Enter) | 100 | sizes 3 B … ~1 KB |
| Driver prompts confirmed by `UserPromptSubmit` | 88 | + 12 `/clear` (fire no `UserPromptSubmit`) = **100 / 100 accounted** |
| `UserPromptSubmit` total | 598 | **510 harness-injected** (`<task-notification>`, `<agent-message from=…>`) |
| `Stop` | 324 | ≈ 3.7 turn ends per driver prompt (background agents re-wake the builder) |
| `Notification` (idle_prompt) | 150 | fires while background agents still run |
| Dialogs opened / answered via hook | 22 / 22 | all `AskUserQuestion`; 0 fell back to the window, 0 rendered |
| `/clear` | 12 | each: `SessionEnd reason=clear` + `SessionStart source=clear`, new session id |
| Wheel moves to `human` | 3 | all 19:47–20:03 on day 1, all FALSE (see §2.3); **0 after the fix, over ~17.5 h and ~490 harness prompts** |

## 2. Behaviours measured (each one a ledger row or a contract input)

### 2.1 Typing and delivery
- **Turn-boundary send works; delivery is provable.** Every typed text reappeared verbatim as the next
  `UserPromptSubmit.prompt` (checked after each send). The only exception class is local commands (`/clear`),
  which fire no `UserPromptSubmit` — their proof is the post-condition in 2.4.
- **`send` into a busy session refuses after its 120 s wait** ("not at a turn boundary"). Correct: it never
  typed into a running turn by accident.
- **`send --force` into a running turn is QUEUED by the CLI and delivered mid-turn.** Measured 6× (a
  "directions for this run" note right after a skill command): each arrived as a `UserPromptSubmit` while the
  skill was working, and the model folded it into the running work. This is a real, useful capability
  (steering a long turn) — and a hazard if it is ever the default.
- **Long paste wrapping:** above ≈ 800 B the CLI wraps the paste in `<pasted_content id=…>`; the model still
  treats it as the user's own text (18 h of sends up to ~1 KB, all obeyed). Tag-like text a user types
  arrives escaped (`<\pasted_content`). The classifier must normalise both.
- **Git Bash rewrites a leading `/`** in an argument into a Windows path (`/clear` → `C:/Program Files/Git/clear`).
  `MSYS_NO_PATHCONV=1` fixed it every time. Product consequence already in intent F-29: text comes from stdin
  or a file, a leading-slash argument is warned.

### 2.2 Dialogs through hooks
- 22 / 22 `AskUserQuestion` dialogs answered through the `PreToolUse` hook; the builder never saw the dialog
  render. Up to **3 questions in one dialog**, each answered in order, by option label or by free text
  (the "Other" path), with an optional note carried into `annotations`.
- The dialog record carries the full `tool_input` (questions, option labels, descriptions, previews). That is
  enough for a driver to decide without reading the screen.
- **Not exercised in this run:** `PermissionRequest` (auto mode was on), plan approval (`ExitPlanMode`),
  `multiSelect`. Those rows stay "measured once" from the brief (S3/S7/S8).

### 2.3 The wheel and harness-injected prompts
- Harness turns fire `UserPromptSubmit` exactly like a human prompt: 510 of 598 in this run. Before the
  classifier existed they flipped the wheel to `human` (2× on day 1, pausing automation); a long paste's
  `<pasted_content>` wrapper did it once more. After the prefix classifier (`<agent-message from=`,
  `<task-notification>`) plus paste/escape normalisation: **0 false takes in ~17.5 h**.
- No real human typing happened after 20:03, so the true-positive path was not re-measured in this run
  (it was on day 1, brief §6).

### 2.4 `/clear`
- 12 / 12: `SessionEnd reason=clear` then `SessionStart source=clear` with a NEW `session_id`; no
  `UserPromptSubmit`. The prototype's `sent/*.txt` record therefore never matches and must be dropped once the
  post-condition is seen. This is the confirmation rule for local commands (intent F-29 "unconfirmable only
  without a measured post-condition").

### 2.5 Turn ends are noisy — `wait` needs a relevance filter
- 324 `Stop` for 88 driver prompts. Most extra turn ends are progress notes ("research is running",
  "waiting on the other five distillers") produced when a background agent re-wakes the builder.
- The overseer needed a filter (`waitm.sh`) that skips short progress turns and returns on a dialog, a long
  turn end, or completion; it skipped 0–8 progress turns per return. **Implication for `wait`:** "first
  driver-relevant event" must not mean "any `Stop`"; the product needs either a relevance rule or a
  documented pattern for drivers to filter.
- `Notification idle_prompt` fires while background agents are still running: **idle ≠ done.** `list`
  showing `idle` is not a completion signal.

### 2.6 The final text is enough to drive
- `Stop.last_assistant_message` carried the whole final turn text (routinely 3–7 KB: review cards, gate
  tables, CI verdicts). Every decision in 18 h was taken from it plus the dialog record — the screen tail was
  needed only to spot CLI-native modals (§2.7). Confirms `last` as a first-class verb.

### 2.7 What hooks cannot see
- **CLI-native modals bypass every hook** (brief M1, day 1): "Teach auto mode about your environment?"
  appeared after a `Stop`, status read `idle`, and a paste answered the modal. Not repeated in this run,
  but still the reason the readiness gate needs a screen check, not only hook state.

### 2.8 Encoding on Windows
- Dialog text contains non-cp1252 characters (`→`, `—`). A Python reader with the default Windows console
  encoding crashed (`UnicodeEncodeError: 'charmap'`); `PYTHONIOENCODING=utf-8` fixed it. **Product
  consequence:** every viola output path (CLI human mode, `--json`, MCP) must write UTF-8 regardless of the
  console code page — a CLI-machine-contract test on Windows.

## 3. Driver-side practice that made 18 h unattended possible
- A background `wait` (with the relevance filter) that re-wakes the driver's harness, plus a 20-minute
  scheduled heartbeat that re-checks state if a wait dies. No wait died in this window.
- Every send is followed by a delivery check (`UserPromptSubmit` text match); every `/clear` by the
  post-condition check.
- Answers carry provenance ("overseer, founder-delegated") in the note; the builder recorded them as such.

## 4. Where each finding lands (route entries / intent)
| Finding | Route entry | Intent |
|---|---|---|
| 2.1 delivery proof, busy refusal, `--force` queueing, paste wrapper | Readiness gate; Confirmed send | F-29 |
| 2.2 multi-question dialogs, free text + notes | Dialog answers by dialog_id | F-31 |
| 2.3 harness-prompt classifier, 0 false takes | The wheel | F-32 |
| 2.4 `/clear` post-condition | Confirmed send | F-29 |
| 2.5 noisy `Stop`, idle ≠ done | wait and last | F-30 |
| 2.6 final text suffices | wait and last | F-30 |
| 2.7 native modals | Readiness gate | F-29 |
| 2.8 UTF-8 on Windows | CLI machine contract; CLI output discipline | F-39, F-40 |
