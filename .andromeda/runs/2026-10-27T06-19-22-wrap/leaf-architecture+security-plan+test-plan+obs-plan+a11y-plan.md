group: architecture+security-plan+test-plan+obs-plan+a11y-plan
read: CLAUDE.md · 128 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:38   class: omission
  leaf-text: «`Read::take(MAX_FRAME)` on every external reader;»
  source: .andromeda/architecture.md:422 «Every product reader of external bytes is bounded in its own statement (`take`) or named as an exception (the own-executable reads)»
  replace-with: «every product reader of external bytes bounded in its own statement (`Read::take(MAX_FRAME)`) or named as an exception (the own-executable reads), as `tests/contract_bounded_readers.rs` censuses, and each `viola mcp` stdin line capped at `MAX_FRAME` before rmcp reads it, an over-long line refused whole and ending the session, never truncated or skipped;»

- leaf: CLAUDE.md:25   class: omission
  leaf-text: «rmcp 3.4.1 stdio server, tools `send · wait · last · answer · list` (Tokio)»
  source: .andromeda/architecture.md:43 «Its stdin is read under a `MAX_FRAME` line cap, each line's newline counted: a longer line is refused whole, never truncated or skipped, and ends the session as a closed stdin does»
  replace-with: «rmcp 3.4.1 stdio server, tools `send · wait · last · answer · list` (Tokio); its stdin handed to rmcp through the crate-private `stdin::CappedLines` at `MAX_FRAME` per line, the newline counted, an over-long line refused whole and ending the session»
