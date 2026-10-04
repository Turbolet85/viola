# security extract

## Relevance
partial — the chunk adds wrapper state behind an existing channel method (`send`) and an existing verb (`release`). It opens no new boundary, method, param, listener or dependency. Security binds through the refusal order, the `release-from-driver` guard, the human-always-wins rule, the treatment of upstream text and the NEVER-log floor.

## Constraints
- The running-turn refusal must go only to automation (`send`). A human keystroke during a running turn is never refused, blocked or delayed, and the hook path still fails open (exit 0, no body). Per security-plan §Security Anti-Patterns §Universal ("NEVER let a security refusal block the human") and §Error Handling (hook fail-open).
- The new `turn-running` rung sits after the authoritative input checks. The `send` params check (`-32602` `"invalid params"`, `data: null`) runs first, then `validate_paste_text` (`not-delivered`/`control-character`), first in the wrapper's refusal order. A running turn must never let a control-character text through, and must never reorder a `-32602` behind it. Per security-plan §Input Validation (Paste text row; Channel frames row, `send` params).
- A `release` whose `params` carry a string `from` is rejected `-32602` with `data: {"reason":"release-from-driver"}` (other `from` types: `data: null`). Per security-plan §Authentication & Authorization (Driver-originated `release` row) and §Input Validation (Channel frames row, `pause` / `release` params). The rejection must change no state, so the running-turn clear may happen only on a `release` that passes that check. Whether the code validates params before any `WheelSlot` mutation, so the new clear can sit after the check, is research's question.
- `from` is self-reported, never an identity. Neither the turn state nor the clear may key on `from`, and the `release` guard stays an affordance, not enforcement. Per security-plan §Security Anti-Patterns §Authentication ("NEVER treat the channel `from` param as an authenticated identity") and §Threat Model Summary (Local IPC trust boundary).
- Turn start and turn end follow the event kind of a re-validated `hook.event`. The kind must be one of the five hook kinds and `data` an object; for `prompt-submitted`, `text` must be a string and `origin` `harness` · `human`. An invalid event is not appended and so must not move the turn state. The state never parses prompt text: `origin` comes only from the four compiled harness prefixes, and upstream text is content, never a command. Per security-plan §Input Validation (Channel frames row, `hook.event` re-validation) and §Security Anti-Patterns §Code Patterns (R1; compiled harness prefixes). Whether `turn-ended` / `session-start` / `session-end` reach the wrapper through the same re-validated path as `prompt-submitted` is research's question.
- Model the running-turn state as a closed type (enum or bool), not a free `String`, the same way the wheel holder is modelled. Per security-plan §Security Anti-Patterns §Input ("NEVER model `behavior`, `dialog_id`, the wheel holder or the dialog kind as free `String`s").
- If the state rides `snapshot.json`, only the instance's own wrapper writes it, through `viola_state::fs::replace_private` (0600, mode set before any byte is written). It carries no user content: the snapshot is operational metadata, and prompt text belongs only to `events.ndjson`. Per security-plan §Security Anti-Patterns §Universal (snapshot writer), §Authentication & Authorization (`~/.viola/` access control) and §Threat Model Summary (Data classification).

## Patterns to follow
- `validate_paste_text` stays first and authoritative in the wrapper's `send` refusal order, and the client-side check stays advisory (security-plan §Input Validation, Paste text row; §Security Anti-Patterns §Input "NEVER trust … client-side CLI checks alone").
- `viola_channel::ProtocolError::ReleaseFromDriver` stays the single rejection for a driver-originated `release`, with its one `release-from-driver` log line (`from_trust:"self-reported"`) (security-plan §Authentication & Authorization, Driver-originated `release`).
- Wire refusals are fixed codes: channel `result.refusal` `not-delivered` with detail `turn-running`, MCP `structuredContent` with only `refusal` / `detail` (security-plan §Error Handling).
- The interim liveness-only pre-check gaps for CLI `send` / `release` stay as dated. The chunk adds no frame and borrows no exception (security-plan §Authentication & Authorization, IPC client-side server verification row).

## Anti-patterns to avoid
- Typing, queueing or partly writing anything into the PTY for a `send` refused `turn-running`, or letting the turn gate touch human input or `answer` / `hook.dialog` (security-plan §Security Anti-Patterns §Universal, refusals only to automation, and §Input, no write of refused text).
- Putting prompt text, `last_assistant_message`, paths or upstream values into any refusal, log or diagnostic line other than `diagnostics/detail-*.ndjson`. Carry codes only (security-plan §Bootstrap phases `logging-redaction-wire`; §Security Anti-Patterns §Logging).
- Adding a new channel method, param or listener to carry or clear the turn state without a Security Decisions Log entry (security-plan §Security Anti-Patterns §Universal).

## Contract bindings
- security ↔ architecture §Conventions (`send` refusal order): `control-character` comes first (security-owned), and `turn-running` keeps its documented rung.
- security ↔ obs: any `send-refused` or turn-state line is held to the NEVER-log floor, codes only (security-plan §Bootstrap phases `logging-redaction-wire`).
- security ↔ tests: the witness asserts zero PTY bytes written for the refused send, and a `release-from-driver` case asserts that the state is not cleared.

## Acceptance criteria contributions
- During a running turn, a `send` whose text holds a C0 control other than LF / CR / TAB is refused `control-character`, not `turn-running`, and nothing is typed (per security-plan §Input Validation, Paste text row).
- During a running turn, a `release` carrying a string `from` returns `-32602` with `data: {"reason":"release-from-driver"}`, and a following `send` is still refused `turn-running`, so the turn state is not cleared (per security-plan §Authentication & Authorization, Driver-originated `release`).
- During a running turn, human keystrokes still reach the child unblocked; only the automated `send` is refused (per security-plan §Security Anti-Patterns §Universal).
- The `turn-running` refusal result, MCP `structuredContent` and every log line it causes carry only fixed codes: no prompt text, no absolute path (per security-plan §Error Handling and §Bootstrap phases `logging-redaction-wire`).
