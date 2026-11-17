# Answer card — 2026-11-17-first-readings-on-the-standing-rig (Q1)

The founder's own answers, each relayed verbatim by the overseer (his option's label verbatim), on record before this
Validate:

- **A1 — the rig's own capture**, inputs#I2 item 3 («Разрешить захват в риге», read 2026-11-17T16:05:02Z): the rig
  may register a UserPromptSubmit hook of its own in the started child's private config dir, recording only the
  payload's key names and `source` as a closed word (`schedule_wakeup` · `loop_wakeup` · `other` · `absent`) — no
  prompt text, no path, no value of any other key; nothing written to his real config; the product's hook unchanged.
- **A2 — the user-config witness**, inputs#I6 item 1 («Искать в файле следы рига», read 2026-11-17T16:50:18Z): when
  the whole-file stamp moves after a start, the rig reads the file for exactly two marks of its own (its scratch
  root's path, its child's session id) and prints one closed word (`unchanged` · `changed-by-others` ·
  `changed-by-rig`, RED); nothing of the content kept, copied, logged or written.

The escalated operations lying wholly inside them (each with fit's verdict):

1. `ops-security-plan.md · decision/live-rig-prompt-capture-closed` — area input-validation — `fit: fits type 1` —
   the capture reads the payload under MAX_FRAME, default JSON depth, writes one owner-only line of key names
   `[a-z_]{1,40}`, others counted, one closed `source` word, never a value; non-object or over-long → `unread`. (A1)
2. `ops-security-plan.md · decision/live-rig-isolated-child` — area secret-management — `fit: fits type 1` — the rule
   set: private CLAUDE_CONFIG_DIR, closed kept env list, the user's `~/.claude.json` read only after a moved stamp,
   searched for rig-made needles, the bytes dropped, one closed word kept (was: a sha256 prefix and mtime only). (A2)
3. `ops-architecture.md · decision/claude-shapes-in-one-crate` — area crate-dependency-direction — `fit: fits type 2`
   — `except` set: the test-only live rig in viola-e2e reads the key names and the source word of its own captured
   UserPromptSubmit payload, never a value. (A1's consequence)
4. `ops-architecture.md · decision/claude-first-no-adapter-trait` — area established-decisions — `fit: fits type 2` —
   the same `except`. (A1's consequence)

Named beside the card, not asked (fit read `not_master` before the question):
`ops-security-plan.md · decision/live-rig-user-config-read-bounded` — `not_master type 4`.

Not on this card: `ops-security-plan.md · decision/live-round-rig-keys-own-pty` (Q2 — the overseer's relay of the
founder's answer on whether «anew for each chunk» binds the stop; inputs#I8 item 2).

The card's count: 4 lines.
