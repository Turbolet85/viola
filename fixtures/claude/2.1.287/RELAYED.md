# Relayed fixtures — `fixtures/claude/2.1.287/`

Four files in this directory are **relayed captures**, not `viola verify --record` recordings:

- `PreToolUse.ask-user-question.json`
- `PermissionRequest.ask-user-question.json`
- `PreToolUse.exit-plan-mode.json`
- `PermissionRequest.exit-plan-mode.json`

## Why relayed
On `claude` 2.1.287 a print-mode turn exposes neither `AskUserQuestion` nor `ExitPlanMode` and fires no dialog hook,
so `viola verify`'s probe cannot record them (chunk 2026-10-04-dialog-answers-by-dialog-id,
`evidence/print-mode-dialog-probe.md`). The founder ruled live (2026-10-04 ~11:55Z, relayed by the overseer): the dialog
fixtures are sourced from the viola-lab prototype's live interactive 2.1.287 captures, scrubbed and marked as relayed.
Working-route `:82`'s own `viola verify` re-probe supersedes them.

## Source and form
- Source: `~/.viola/sessions/*/events.ndjson` (the prototype's per-session log; each line's `data` is the hook
  payload), interactive `claude` 2.1.287. Picked by
  `viola-0.1.0/chunks/2026-10-04-dialog-answers-by-dialog-id/evidence/relay_fixtures.py`: per tool, the smallest
  PreToolUse -> PermissionRequest pair of one session whose `tool_input` is equal.
  - `*.ask-user-question.json`: session `viola-builder`, lines 884 / 888.
  - `*.exit-plan-mode.json`: session `andromeda-worker`, lines 124 / 129.
- Re-serialized: the prototype stored each payload with sorted keys, and the relay writes one compact sorted-key
  object plus a newline (the form `viola verify --record` writes) — not the CLI's own byte order.
- Scrubbed with `ledger::scrub`'s rule (the user home to `~`, the username to `<user>`) and checked with
  `ledger::is_clean`'s checks before any file was written.

## Redaction (the overseer's review, 2026-10-04)
viola is a public repository, and the captured `ExitPlanMode` `tool_input.plan` (12 971 characters) was a private
Andromeda repository's plan (paths, shas, brief names). The relay replaces `tool_input.plan` in BOTH plan files with one
identical neutral markdown string (a heading and two lines, no project content); `planFilePath` and every other key are
kept, and the two `tool_input` values stay equal. The question pair is relayed unredacted, as approved.

| file | sha256 before the redaction | sha256 as committed |
|---|---|---|
| `PreToolUse.exit-plan-mode.json` | `376b38f8af306c3e28bf8e7853086f4fde55eca7289077f44a871b6deeefb2d6` | `897df36b77ee61719f576fa7487f5378ecc4605263f0c1c787391fe29ed740b2` |
| `PermissionRequest.exit-plan-mode.json` | `a424432bbb3a4f3abf06f5f50772b67af7a48465bc1630d3deaaf42a0143b20a` | `a84ee62d04986345df055bf9e7ff42e617346842e9af003026868dabb86973d4` |
| `PreToolUse.ask-user-question.json` | — | `e027445a974cdb5cc89c66693c90c780b4a1ae7d5edc3f9e82b717b97e3f2c0d` |
| `PermissionRequest.ask-user-question.json` | — | `ecc4fa94135dbe96a8db358d526aea8765494addd57c6ae124ec79637428727f` |

No test reads plan text content: the tests compare a plan dialog's `data.plan` only with the fixture's own value.

## Superseded (2026-10-05, chunk 2026-10-05-dialog-rows-and-re-probe)
`viola verify`'s own dialog re-probe recorded this version's dialog tier (Run C and Run D, each dialog answered by the
probe's capture hook with the product's own decision body; `stamped 2.1.287  14 pass  0 fail`). The recorded files are the
measured truth; the relayed four stay only for Path 4's existing cases (`fixtures/fake-scripts/path4.json`).

| relayed file | recorded counterpart |
|---|---|
| `PreToolUse.ask-user-question.json` | `PreToolUse.questions-1.json` (and the one-question calls `PreToolUse.parallel-1.json`, `PreToolUse.parallel-2.json`) |
| `PermissionRequest.ask-user-question.json` | none: a hook-answered AskUserQuestion raises no PermissionRequest (each answered question went straight to its own PostToolUse, `PostToolUse.questions-1.json`) |
| `PreToolUse.exit-plan-mode.json` | `PreToolUse.plan-1.json` (the unanswered first plan) and `PreToolUse.plan-2.json` (the re-plan after the revise) |
| `PermissionRequest.exit-plan-mode.json` | `PermissionRequest.plan-1.json` (answered `deny` + `message`, the revise) |
