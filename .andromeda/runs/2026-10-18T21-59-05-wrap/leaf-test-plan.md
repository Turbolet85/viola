group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 101 · «- 2026-10-17: A harness-session case never assumes the fake »
read: .claude/rules/verification-harness.md · 71 · «- 2026-10-14: A side effect added to a fake-agent turn (a sp»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «the `paste-<n>` / `clear-<n>` framing variants»
  source: .andromeda/test-plan.md:920 «Recorded `viola verify` spine fixtures, dialog variants and framing variants (`paste-<n>`, `compact-<n>`, `clear-<n>`)»
  replace-with: «the `paste-<n>` / `compact-<n>` / `clear-<n>` framing variants»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «the framing tier by Run B's four added pastes»
  source: .andromeda/test-plan.md:379 «The framing-tier fixtures are recorded the same way by Run B's five added pastes as `UserPromptSubmit.paste-<n>.json`, `SessionStart.compact-<n>.json`»
  replace-with: «the framing tier by Run B's five added pastes»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «`--framing` replays Run B's recorded paste and `/clear` variants»
  source: .andromeda/test-plan.md:957 «and the probed `/compact` the recorded `SessionStart.compact-1` with the last fired SessionStart's `session_id` and no UserPromptSubmit»
  replace-with: «`--framing` replays Run B's recorded paste, `/compact` and `/clear` variants (the probed `/compact` its recorded `SessionStart.compact-1` with the last fired SessionStart's `session_id`)»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «five framing variants»
  source: .andromeda/test-plan.md:965 «`2.1.287`'s six recorded framing variants, `UserPromptSubmit.paste-1.json`, `UserPromptSubmit.paste-2.json`, `UserPromptSubmit.paste-3.json`, `SessionStart.compact-1.json`»
  replace-with: «six framing variants»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «23 on Unix and 20 on Windows»
  source: .andromeda/test-plan.md:598 «`<n>` the relied count (`LedgerRow::relied()`'s length: 24 on Unix, 21 on Windows)»
  replace-with: «24 on Unix and 21 on Windows»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «and replays the resume variant (`fake_agent_resume_replay_matches_every_recorded_resume_set`)»
  source: .andromeda/test-plan.md:379 «`contract_fake_agent_drift` also replays the compact variant byte for byte once resumed into its own session (`fake_agent_compact_replay_matches_every_recorded_compact_set`)»
  replace-with: «and replays the resume variant (`fake_agent_resume_replay_matches_every_recorded_resume_set`) and, once resumed into its own session, the compact variant (`fake_agent_compact_replay_matches_every_recorded_compact_set`)»

- leaf: .claude/docs/tests-summary.md:26   class: stale
  leaf-text: «(`/remote-control` and `/compact` on any version, or `/clear` on an unverified CLI)»
  source: .andromeda/test-plan.md:666 «`local` (a ledger-listed command with no measured post-condition: `/remote-control` on any version, `/compact` and `/clear` on an unverified CLI)»
  replace-with: «(`/remote-control` on any version, or `/compact` and `/clear` on an unverified CLI)»

- leaf: .claude/docs/tests-summary.md:26   class: stale
  leaf-text: «`/compact` on a verified CLI `send_compact_on_a_verified_cli_is_unconfirmable_at_once`)»
  source: .andromeda/test-plan.md:666 «`/compact` on a verified CLI is confirmed by its post-condition alone, a `session-start` with cause `compact` and the session's own (unchanged) `agent_session_id`»
  replace-with: «`/compact` on an unverified CLI `send_compact_on_an_unverified_cli_is_unconfirmable_at_once`); `/compact` on a verified CLI is confirmed by a `session-start` with cause `compact` and the session's own unchanged id inside its own `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL), else `not-delivered`/`no-prompt-submitted` at it (`send_compact_on_a_verified_cli_is_confirmed_by_its_same_session`, on the recorded 2.1.287 `compact-1` variant)»

- leaf: .claude/docs/tests-summary.md:61   class: omission
  leaf-text: «the confirmation window is the compiled `CONFIRM_WINDOW_FALLBACK` = 10 s»
  source: .andromeda/test-plan.md:515 «its expiry at `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL; `CONFIRM_WINDOW_FALLBACK` stays 10 s for every other confirmed send)»
  replace-with: «the confirmation window is the compiled `CONFIRM_WINDOW_FALLBACK` = 10 s for every confirmed send but `/compact` on a verified CLI, which waits its own `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL)»

- leaf: .claude/rules/verification-harness.md:28   class: stale
  leaf-text: «Run B with its four added pastes»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:8 «Run B with its five added compiled pastes — the long, the tag-like and the newline texts, then `/compact`, then `/clear` —»
  replace-with: «Run B with its five added pastes (the long, tag-like and newline texts, then `/compact`, then `/clear`)»

- leaf: .claude/rules/verification-harness.md:28   class: stale
  leaf-text: «twenty-three on Unix, elsewhere twenty, the first eighteen, `resume-session` and `agents-join`»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:8 «(`LedgerRow::relied()`: all twenty-four on Unix, elsewhere twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`)»
  replace-with: «twenty-four on Unix, elsewhere twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`»

- leaf: .claude/rules/verification-harness.md:38   class: omission
  leaf-text: «and no other payload is built.»
  source: .andromeda/test-plan.md:940 «no other payload is built save the compact replay's `SessionStart.compact-1` under `--framing`, whose `session_id` is set to the last SessionStart the agent fired»
  replace-with: «and no other payload is built save the compact replay's `SessionStart.compact-1` under `--framing`, whose `session_id` is set to the last SessionStart the agent fired.»

- leaf: .claude/rules/verification-harness.md:41   class: omission
  leaf-text: «the probed local command its recorded `clear-<n>` SessionEnd then SessionStart;»
  source: .andromeda/test-plan.md:957 «the probed `/clear` fires the recorded `clear-1` SessionEnd then SessionStart and no UserPromptSubmit, and the probed `/compact` the recorded `SessionStart.compact-1` with the last fired SessionStart's»
  replace-with: «the probed `/clear` its recorded `clear-<n>` SessionEnd then SessionStart and no UserPromptSubmit, and the probed `/compact` the recorded `SessionStart.compact-1` with the last fired SessionStart's `session_id` and no UserPromptSubmit;»

- leaf: .claude/rules/verification-harness.md:45   class: omission
  leaf-text: «the framing variants (`paste-<n>`, `clear-<n>`)»
  source: .andromeda/test-plan.md:920 «Recorded `viola verify` spine fixtures, dialog variants and framing variants (`paste-<n>`, `compact-<n>`, `clear-<n>`)»
  replace-with: «the framing variants (`paste-<n>`, `compact-<n>`, `clear-<n>`)»

- leaf: .claude/rules/verification-harness.md:45   class: omission
  leaf-text: «and the resume variant (`SessionStart.resume-1.json`, by `fake_agent_resume_replay_matches_every_recorded_resume_set`),»
  source: .andromeda/test-plan.md:379 «`contract_fake_agent_drift` also replays the compact variant byte for byte once resumed into its own session (`fake_agent_compact_replay_matches_every_recorded_compact_set`)»
  replace-with: «and the resume variant (`SessionStart.resume-1.json`, by `fake_agent_resume_replay_matches_every_recorded_resume_set`) and the compact variant (`SessionStart.compact-1.json`, once resumed into its own session, by `fake_agent_compact_replay_matches_every_recorded_compact_set`),»
