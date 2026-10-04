# Relayed dialog fixtures — the relay leg's record (plan step 8)

Founder ruling, live, 2026-10-04 ~11:55Z (relayed by the overseer): the dialog fixtures are sourced from the
viola-lab prototype's live interactive `claude` 2.1.287 captures, scrubbed and marked as relayed.

## Run 1 (operator entry, fired once by /implement, 2026-10-04 ~14:21Z)
- Run: `python3 viola-0.1.0/chunks/2026-10-04-dialog-answers-by-dialog-id/evidence/relay_fixtures.py` (no `--exclude`).
- Exit 0. Atoms: `exit 0` held · `contains PreToolUse.ask-user-question.json` held ·
  `contains PermissionRequest.exit-plan-mode.json` held. Output kept as
  `.andromeda/runs/2026-10-04T14-20-29-implement/relay.out`.
- Selection: per tool, the smallest PreToolUse -> PermissionRequest pair of one session with equal `tool_input`.

| fixture | source session | line | bytes | sha256 |
|---|---|---|---|---|
| `PreToolUse.ask-user-question.json` | viola-builder | 884 | 1677 | `e027445a974cdb5cc89c66693c90c780b4a1ae7d5edc3f9e82b717b97e3f2c0d` |
| `PermissionRequest.ask-user-question.json` | viola-builder | 888 | 1637 | `ecc4fa94135dbe96a8db358d526aea8765494addd57c6ae124ec79637428727f` |
| `PreToolUse.exit-plan-mode.json` | andromeda-worker | 124 | 14176 | `376b38f8af306c3e28bf8e7853086f4fde55eca7289077f44a871b6deeefb2d6` |
| `PermissionRequest.exit-plan-mode.json` | andromeda-worker | 129 | 14136 | `a424432bbb3a4f3abf06f5f50772b67af7a48465bc1630d3deaaf42a0143b20a` |

## The overseer's review of run 1 (2026-10-04)
- Question pair (viola-builder 884 / 888): approved as is.
- Plan pair (andromeda-worker 124 / 129): not approved as is — viola is PUBLIC (`gh`: visibility PUBLIC) and
  `tool_input.plan` was 12 971 characters of a private Andromeda repository plan (paths, shas, brief names). Directed:
  redact inside the relay script, not by hand — `tool_input.plan` in BOTH files replaced with one identical neutral
  markdown string (a heading plus two lines, no project content), `planFilePath` and every key kept, the two
  `tool_input` values kept equal; the redaction, the original sha256s and the reason recorded in `RELAYED.md`.

## Run 2 (the redacting script, re-fired once on the overseer's direction, 2026-10-04 ~14:30Z)
- The script gained `NEUTRAL_PLAN` and `redact_plan` (ExitPlanMode payloads only; refuses one without a string
  `tool_input.plan`). Same run form, exit 0, the same three atoms held. Output kept as
  `.andromeda/runs/2026-10-04T14-20-29-implement/relay-2.out`.
- The same four sources were picked (the store is live; the selection did not move). The question pair's bytes are
  identical to run 1's (same sha256s, as approved).
- Checked after the run: both plan files' `tool_input.plan` equal the neutral string, the two `tool_input` values are
  equal, `tool_input` keeps `plan` + `planFilePath`, and every top-level key is kept (PreToolUse 11 keys,
  PermissionRequest 10 — no `tool_use_id`, as captured).
- Residual, kept by the directive ("keep every key"): `cwd`, `scratchpad_dir` and `transcript_path` still name the
  source project directory (`andromeda-worker`, scrubbed to `~` / `<user>`) and the session uuid.

| fixture | bytes | sha256 |
|---|---|---|
| `PreToolUse.ask-user-question.json` | 1677 | `e027445a974cdb5cc89c66693c90c780b4a1ae7d5edc3f9e82b717b97e3f2c0d` |
| `PermissionRequest.ask-user-question.json` | 1637 | `ecc4fa94135dbe96a8db358d526aea8765494addd57c6ae124ec79637428727f` |
| `PreToolUse.exit-plan-mode.json` | 753 | `897df36b77ee61719f576fa7487f5378ecc4605263f0c1c787391fe29ed740b2` |
| `PermissionRequest.exit-plan-mode.json` | 713 | `a84ee62d04986345df055bf9e7ff42e617346842e9af003026868dabb86973d4` |

- Plan-text reads: `grep -rnE 'exit-plan-mode|ExitPlanMode|tool_input|"plan"|planFilePath' tests src crates --include=*.rs`
  finds no test reading plan text (the `tool_input` hits are Bash `command` fields; the `plan` hits are the kind name).

## Overseer review
overseer review: approved (plan text redacted, 2026-10-04)
