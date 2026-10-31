# a11y · ARIA Patterns & Roles — category read

## Carried
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:18) A sanitised `send` refusal must keep the P4 cli shape: the `human-typing` code on stderr with its typed exit code, held by assert_cmd.
  > - **cli:** `send` refused with `human-typing` on stderr, with a typed exit code (assert_cmd).
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:19) The fixed-code work must leave this `human-typing` value and its exit code as they are, and the wheel case's outcome unchanged.
  > the send exits 10 `human-typing` with nothing typed and the key reaches the child
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:30) Every `--json` error document the chunk reshapes must carry no SGR, in the same P6 assertion set as the non-TTY modes.
  > - no SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json`;
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:36) Any a11y acceptance the chunk writes on its CLI witnesses must be a stand-in with no conformance claim.
  > These stand in for SC 1.3.2, 1.4.1 and 3.3.1, with no conformance claim.
- (.andromeda/a11y-plan.md:192) A `thiserror` message made fixed must leave the P2 human-mode refusal layout intact, reason and detail then the `hint:` line, as trycmd pins it.
  > - **cli:** `[RB]` mirror and `unable <reason> <detail>` + `hint:` via trycmd.
- (.andromeda/a11y-plan.md:194) The `unverified-cli` refusal must stay readable text on the CLI after the chunk narrows the error bodies to codes.
  > - **cli:** `viola answer` refusal `unverified-cli` text on the CLI.
- (.andromeda/a11y-plan.md:94) A refusal whose message source the chunk changes must still reach stderr in the `unable … hint:` form with a typed exit code.
  > the refusal written to stderr as `unable … hint:` with a typed exit code.
- (.andromeda/a11y-plan.md:107) Stripping paths and upstream text from a shared `Display` must not strip the human reader's refusal reason and detail, which stay given in text.
  > Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text.
- (.andromeda/a11y-plan.md:53) The CLI `--json` surface must hold this output discipline, and any upstream text still shown to a human must stay escaped.
  > no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`; output is static and linear; C0/C1 control characters are escaped except `\n` and `\t`, which `list` rows escape too.
- (.andromeda/a11y-plan.md:55) A fault text the chunk takes off an error surface must not move to the terminal on the `run` path, whose diagnostics stay in `diagnostics/`.
  > viola's diagnostics for `run` go to `diagnostics/`, never to the terminal
- (.andromeda/a11y-plan.md:29) The chunk's CLI `--json` witnesses must be output-discipline assertions on text and streams.
  > output-discipline assertions on text and streams only; no automated a11y tool reaches a terminal.
- (.andromeda/a11y-plan.md:186) The CLI witnesses must use this reach, on all three OS legs, and no other a11y tool.
  > | cli | assertable (output discipline, no conformance claim) | assert_cmd 2.2.2 + predicates 3.1.4 + trycmd 1.2.1; portable-pty outer-PTY SGR checks; all three OS legs |
- (.andromeda/a11y-plan.md:182) The MCP and channel error surfaces owe no a11y assertion, because the plugin and IPC surfaces are skipped in the tool-reach inventory.
  > (surfaces flagged not-assertable in a11y-scope Section 1 are skipped: plugin/IPC, internal crates)
- (.andromeda/a11y-plan.md:40) The chunk's MCP and channel witnesses must stand as machine-contract proof, with no a11y claim resting on them.
  > **Reason:** ipc-internal with no UI surface. Machine-to-machine traffic only.
- (.andromeda/a11y-plan.md:31) The revive start-refusal document must stay with its named owner, and this chunk must not build it.
  > - **Owed:** under `viola revive --json`, a refusal `run`'s start makes after a passed preflight prints no document yet — owed to "Exit-cause code catalogue".
- (.andromeda/a11y-plan.md:73) The per-surface witnesses and the enumeration must run inside the existing suites, adding no `suite` value without a Decisions Log entry.
  > **No new suite:** no new `suite` enum value (such as `a11y`) is added without a Decisions Log entry.
- (.andromeda/a11y-plan.md:312) P4 and P6 output-discipline invariants must still pass on all three OS legs after the error-surface changes.
  > The CLI output-discipline invariants (P4, P6) pass on all three OS legs inside `nextest-integration` / `nextest-e2e` (CI: the `coverage` suite)
- (.andromeda/a11y-plan.md:330) The chunk's proof must not be presented as WCAG conformance or as a11y-tool evidence.
  > NEVER claim WCAG conformance for the cli or the `viola run` TUI (the wrapper-hosted TUI, whether `viola run` or a passed `viola revive` started it). assert_cmd / trycmd / portable-pty results are output-discipline stand-ins for SC 1.3.2 / 1.4.1 / 3.3.1, not a11y-tool evidence.

## Bindings
- **P2 and P4 refusal shape ↔ body §8 Error recovery**: the path scenarios require the refusal on stderr with its code and exit code, and Error recovery requires the word, plain detail and final `hint:` in human mode and one hint-free document under `--json`; the chunk's fixed `--json` body must satisfy both modes (.andromeda/a11y-plan.md:192), (.andromeda/a11y-plan.md:257).
- **P2 trycmd pin ↔ body §8 Plain language verification**: the refusal layout is pinned by trycmd, and every fixed CLI string is pinned by trycmd or literal, so a copy change the chunk makes must update its pin on purpose (.andromeda/a11y-plan.md:192), (.andromeda/a11y-plan.md:273).
- **P6 no SGR under `--json` ↔ body §6 Visual Design Verification and §11 Visual**: the P6 assertion and the visual ban state the same requirement, and the chunk's `--json` error documents must meet it (.andromeda/a11y-plan-detail/aria-patterns-roles.md:30), (.andromeda/a11y-plan.md:221), (.andromeda/a11y-plan.md:340).
- **Witness placement ↔ body §9 CI Integration**: the new witnesses must report through the harness `run` JSON and the per-OS `gate --require coverage,doctest`, with no second driver (.andromeda/a11y-plan.md:73), (.andromeda/a11y-plan.md:301).
- **Revive owed item ↔ the route entry "Exit-cause code catalogue"**: the body names that entry the owner and the chunk's scope lists it as a boundary, so both sides keep the document out of this chunk (.andromeda/a11y-plan.md:31).
