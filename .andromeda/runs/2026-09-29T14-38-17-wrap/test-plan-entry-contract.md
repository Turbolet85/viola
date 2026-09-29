
## 2026-09-29-fake-agent-drift-contract — The fake-agent drift contract, byte for byte; change-driven size; matchers and S8 moved
**Section:** §2 test pyramid (Contract row); §6 Contract suite; §7 Seed strategies and Fake agent (payload, `size` and `hook` receipt kinds, matchers); §12 Test Decisions Log (`2026-09-29`)
**Change:**
- The fake agent's hook sequences were "pinned with insta"; now `contract_fake_agent_drift` compares, per recorded `fixtures/claude/<ver>/` set (walked at run time, an empty walk fails), each print-mode hook's receipt `stdin_hex` byte for byte with that event's recorded fixture and the hook order with the spine literal SessionStart → UserPromptSubmit → Stop → SessionEnd, each `ran:true` / `exit_code:0`. insta stays for decision bodies; the seed table splits them from fake-agent transcripts, which have no snapshot.
- The S8 `annotations` assertion moves to "Dialog answers by dialog_id", where the question answer path lands.
- Fake agent: matchers were to land with this chunk; now with "Dialog answers by dialog_id" (no recorded fixture carries a `tool_name`). The UserPromptSubmit payload keeps the fixture's trailing newline. The `hook` receipt gains `stdin_hex` when it ran. The `size` receipt keeps its wording ("at start and whenever the size changed") and now holds by a watcher: written at once, then polled every 10 ms and written on every change, no key needed; interactive mode only.
**Why:** the recorded fixture files are the pinned artifact, and a snapshot copy would move with every `--record` refresh (P4 fork, the overseer agreeing); the contract witnessed the payload's missing newline red before its fix; the key-driven size sampling left a resize with no later key unreceipted (the H2 CARRY), witnessed red by `pty_resize_reaches_a_child_that_reads_no_key`.
**Kept:** insta for decision bodies; the jsonschema fixture walk unchanged.
**Ref:** .andromeda/runs/2026-09-29T14-38-17-wrap/
