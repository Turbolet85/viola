## 5. Creator Brief Excerpt

Sources: `.andromeda/input.md` (which folds in `refs/`) plus the current `refs/viola-brief.md` (§4.2 M1–M7 are newer than the folded copy). Quotes are verbatim.

### Must-Work Scenarios

- R2: "Exactly one driver at a time. A submitted prompt the bridge did not send means the human took the wheel, and automation pauses by itself — no button needed."
- R3: "Type only at a turn boundary, as one bracketed paste followed by Enter. Never send Esc". S1: "A bracketed multi-line paste plus Enter arrived as ONE prompt, newlines inside."
- R5: "a session started without the wrapper makes every hook a silent no-op." R8: "A wrapper strips its parent's session identity" (S6: 14 `CLAUDE*` variables removed; the child registered as a session of its own).
- S3/S7/S8 dialogs: AskUserQuestion answered by "`PreToolUse` … `permissionDecision: allow` with `updatedInput.answers`" (free text and `annotations` too); permission by "`PermissionRequest` … `decision.behavior: allow`"; plan "APPROVE: only `PreToolUse` … `PermissionRequest` `allow` is IGNORED for this tool … REVISE: `PermissionRequest` `deny` with a `message`".
- S5: "A background `wait` that exits on the next `Stop` completed when the driven turn ended".
- Windows: "`claude` on PATH is an npm shim … the wrapper must resolve it. ConPTY did not close the output stream when the child exited, so exit is detected on the process, never on EOF."
- §6 first live test: "the overseer sends `/andromeda-new-session` to the builder, waits for `Stop`, and reads the dashboard" — passed run: "the founder typing into the window took the wheel, `send` was refused, `release` returned it."
- §6: "`/andromeda-new-session` arrived as `C:/Program Files/Git/andromeda-new-session` … take the text from stdin or a file, and warn when an argument carries a rewritten-path prefix."
- M1: "CLI-native modals bypass every hook … A turn boundary per the hooks therefore does not prove the input box is ready." M2: "Harness-injected turns fire UserPromptSubmit" (`<agent-message from=`, `<task-notification>`; the prototype "paused automation twice"). M3/M4: long pastes arrive wrapped in `<pasted_content id="…">`; "A literal `<pasted_content` arrived as `<\pasted_content` … Prompt matching must normalise both." M5: "Local commands fire no UserPromptSubmit." M6: "Hooks found through PATH run a different binary from the wrapper after a rebuild."
- O8 / §7: budget from statusline `rate_limits.five_hour|seven_day.used_percentage`, "then hand over to the user's script unchanged"; "a budget governor belongs in the first version".

### Risk Tolerance Hints

- §7: "Security is a first-class concern: the bridge types into sessions that can run commands. Localhost only by default … the event log as the audit trail."
- §4: "The docs lag the installed build. … Design against the installed CLI, measured." Timing: "Hooks are the source of truth; the screen is an eventually consistent view."
- D3: "cross-platform from the start: Windows, macOS, Linux. Windows is … the first target and the hardest one." O5: "every measurement in §4.1 is Windows-only so far."
- prior-art §4: "viola's on-disk state parses defensively and heals itself from day one." / "The GUI shows only what the hooks and the wrapper observed."

### Test Anti-Patterns (creator's explicit asks)

- §3.4: "CI on all three OSes against a FAKE AGENT — a small program that behaves like Claude Code (prints, waits for input, calls the hook commands). Tests spend no tokens and do not flake; the real CLI runs only in local live tests."
- §3.4: "an agent can verify it through a headless browser, which is how the founder's projects verify UI by default."
- §3.2 / prior-art §4: "Nobody parses the rendered screen for content." / "Viola parses no screen for content (R7)."
- S8: "a first run that seemed to show otherwise was an instrument defect — the spike's hook did not forward `annotations`; fixed and re-measured".
- §4.2: "A long real session surfaced what the fake agent and the short demos could not."
- prior-art §4: "Blanket approval. Auto-yes flags and skip-permissions defaults remove the one checkpoint a human relied on."

### Founder Directions for this tests run (2026-09-24, verbatim; binding from Phase 1 on)

Delivered by the overseer on the founder's behalf, during this run:

1. "Mutation testing is part of the headless contract from chunk 1: cargo-mutants scoped to each chunk diff, surviving mutants are red; include it in the P2 research catalog so the plan can cite it."
2. "Every layer agent-runnable; the fake agent + `viola verify` fixtures are the contract."
3. "Structured JSON logs from every process (run/hook/mcp/ui) in the §3 log format, a `logs` command in the harness, hook trace in diagnostics/ (hook never writes stderr)."
4. "3-OS CI (windows/ubuntu/macos)."
5. "Design cross-lane CL-1: send-issued (cursor, from) and send-refused (refusal + detail) events are expected; cover them in critical paths."
