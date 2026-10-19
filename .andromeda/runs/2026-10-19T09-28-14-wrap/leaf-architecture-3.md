group: architecture-3
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»
read: .claude/rules/events.md · 40 · «- 2026-10-16: A snapshot field that a reader expects once an»

- leaf: .claude/docs/stack.md:21   class: stale
  leaf-text: «| Domain newtypes | nutype 0.8.0 | `ViolaName` (charset + length cap), `Percent` (0–100) |»
  source: .andromeda/architecture.md:25 «| Domain newtypes | nutype 0.8.0 | `ViolaName` (charset + length cap), `Percent` (0–100), `DialogHold` (whole seconds 60–3600) |»
  replace-with: «| Domain newtypes | nutype 0.8.0 | `ViolaName` (charset + length cap), `Percent` (0–100), `DialogHold` (whole seconds 60–3600) |»

- leaf: .claude/rules/events.md:16   class: omission
  leaf-text: «`send-refused` `{refusal, detail, cursor?}` (the CL-1 send records, `source: wrapper`; `cursor` absent for a refusal before `send-issued`).»
  source: .andromeda/architecture.md:267 «- `dialog-expired`: `{dialog_id, dialog_kind}` (`source: wrapper`), appended once when a held dialog's hold ends; it carries no question, plan or tool text.»
  replace-with: «`send-refused` `{refusal, detail, cursor?}` (the CL-1 send records, `source: wrapper`; `cursor` absent for a refusal before `send-issued`), `dialog-expired` `{dialog_id, dialog_kind}` (`source: wrapper`, appended once when a held dialog's hold ends; it carries no question, plan or tool text).»

- leaf: .claude/rules/events.md:17   class: stale
  leaf-text: «- `wait` wakes only on `turn-ended`, `question`, `permission`, `plan`, `session-end`; `activity`, `link`, `unlink`, `wheel`, `budget-gate` and the send records are log-only.»
  source: .andromeda/architecture.md:237 «Only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake (`viola_core::EventKind::WAIT_WAKE`)»
  replace-with: «- `wait` wakes only on `turn-ended`, `question`, `permission`, `plan`, `session-end`, `dialog-expired` (the one wrapper-appended kind that wakes it, appended through `WaitFeed::appending`); `activity`, `link`, `unlink`, `wheel`, `budget-gate` and the send records are log-only.»

- leaf: .claude/rules/events.md:29   class: omission
  leaf-text: «and it rewrites the snapshot's `agent_session_id` on each `session-start`, so the `claude agents --json` session-id join survives a `/clear`.»
  source: .andromeda/architecture.md:220 «It is written before the dialog's event line is appended, and written back to none when that append fails, so a reader that sees the line already sees it.»
  replace-with: «and it rewrites the snapshot's `agent_session_id` on each `session-start`, so the `claude agents --json` session-id join survives a `/clear`; it writes `pending_dialog` `{dialog_id, kind}` before the dialog's event line is appended and back to none when that append fails, so a reader that sees the line already sees it.»
