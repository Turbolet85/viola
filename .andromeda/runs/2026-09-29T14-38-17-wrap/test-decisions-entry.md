
`2026-09-29`: The Fake-agent drift contract chunk pins the fake agent against the recorded fixtures byte for byte, makes the resize oracle change-driven, and moves matcher evaluation and S8 `annotations` to their consumer
- **Decision:**
  - The contract suite (`contract_fake_agent_drift`) compares each print-mode hook's receipt `stdin_hex` with the recorded `<Event>.default.json` bytes and the hook order with a spine literal — no insta snapshot (P4 fork, the overseer agreeing): the recorded fixture files are the pinned artifact, and a snapshot copy would duplicate them and move with every `--record` refresh. insta stays for decision bodies.
  - The contract witnessed one live drift before its fix: the UserPromptSubmit payload dropped the fixture's trailing newline (now kept).
  - The receipt `size` is written by a watcher at start and on every change (P4 fork, the overseer agreeing), the one size mechanism; `pty_resize_reaches_a_child_that_reads_no_key` was red before it.
  - Hook `matcher` evaluation and the S8 `annotations` assertion move to "Dialog answers by dialog_id" (P4 forks, the overseer agreeing): no recorded fixture carries a `tool_name`, and the question answer path does not exist yet.
- **Rationale:** the fake agent is the one component whose job is not to drift from the measured CLI, so its pin is the measured bytes themselves; a matcher implemented from the docs alone would be unmeasured behaviour inside it.
- **Impact:** §2 (Contract row), §4 (viola-agent-claude M2), §6 (Path 5, Contract suite), §7 (Seed strategies, Fake agent).
- **By:** `/andromeda-wrap-session`, chunk `2026-09-29-fake-agent-drift-contract`.
