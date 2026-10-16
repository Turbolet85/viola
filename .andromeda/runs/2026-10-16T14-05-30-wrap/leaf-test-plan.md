group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 95 · «- 2026-10-14: Before a new fixture class's first recording i»
read: .claude/rules/verification-harness.md · 72 · «- 2026-10-14: A side effect added to a fake-agent turn (a sp»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «status line variants recorded by `viola verify`, the dialog tier by its Runs C and D, the framing tier by Run B's four added pastes, the status line variants by Run B's probe status line (Unix);»
  source: .andromeda/test-plan.md:408 «the two status line variants (`StatusLine.default-1.json`, `StatusLine.rate-limits-1.json`), and through Run E the resume variant (`SessionStart.resume-1.json`)»
  replace-with: «status line variants and the `SessionStart.resume-1.json` resume variant recorded by `viola verify`, the dialog tier by its Runs C and D, the framing tier by Run B's four added pastes, the status line variants by Run B's probe status line (Unix), the resume variant by Run E;»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «for verify's four interactive runs»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:8 «so its print probe and all five interactive runs (Run A, Run B with its four added compiled pastes»
  replace-with: «for verify's five interactive runs»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «with each recorded set's `StatusLine.default-1.json` then `StatusLine.rate-limits-1.json` under `--framing` (Unix) (one payload is set, not replayed: under `--resume <id>`,»
  source: .andromeda/test-plan.md:408 «the dialog variants, the framing variants and the resume variant (`fake_agent_resume_replay_matches_every_recorded_resume_set`), including forwarding `annotations` (S8)»
  replace-with: «with each recorded set's `StatusLine.default-1.json` then `StatusLine.rate-limits-1.json` under `--framing` (Unix), and replays the resume variant (`fake_agent_resume_replay_matches_every_recorded_resume_set`) (the resume launch: under `--resume <id>`,»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «the launch SessionStart is the recorded default with `source` `resume` and that `session_id`, or with `--fork-session` the compiled fork id; no recorded fixture of source `resume` exists yet, owed to the route entry "Resume and compact ledger rows", directly after "Owed ledger rows")»
  source: .andromeda/test-plan.md:1077 «under `--resume <id>` the SessionStart fired at launch is the set's recorded `SessionStart.resume-1.json` when the set holds it (2.1.287, 10 key names»
  replace-with: «the launch SessionStart is the set's recorded `SessionStart.resume-1.json` when the set holds it (2.1.287, recorded by Run E since chunk 2026-10-14-resume-and-compact-ledger-rows), else its recorded default (a set without the variant, 2.1.288), with `source` `resume` and that `session_id`, or with `--fork-session` the compiled fork id; the `--resume` shape is the `resume-session` ledger row)»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «five framing variants and two status line variants in 2.1.287 and the superseded relayed fixtures included; 2.1.287 stamped at every relied row, 21 on Unix and 18 on Windows,»
  source: .andromeda/test-plan.md:568 «(the verdict `verified` reads `LedgerRow::relied()`: 22 rows on Unix, 19 on Windows)»
  replace-with: «five framing variants, two status line variants and one resume variant in 2.1.287 and the superseded relayed fixtures included; 2.1.287 stamped at every relied row, 22 on Unix and 19 on Windows,»

- leaf: .claude/docs/tests-summary.md:26   class: omission
  leaf-text: «a ledger-listed local command with no measured post-condition (`/remote-control`, or `/clear` on an unverified CLI) → exit 0 `unconfirmable`, `send-issued` then `send-confirmed{confirmed:false}`, no window (`send_window_local_command_is_not_presumed_delivered`);»
  source: .andromeda/test-plan.md:755 «a ledger-listed command with no measured post-condition: `/remote-control` and `/compact` on any version, `/clear` on an unverified CLI»
  replace-with: «a ledger-listed local command with no measured post-condition (`/remote-control` and `/compact` on any version, or `/clear` on an unverified CLI) → exit 0 `unconfirmable`, `send-issued` then `send-confirmed{confirmed:false}`, no window (`send_window_local_command_is_not_presumed_delivered`; `/compact` on a verified CLI `send_compact_on_a_verified_cli_is_unconfirmable_at_once`);»

- leaf: .claude/rules/verification-harness.md:28   class: stale
  leaf-text: «(print probe + all four interactive runs, Run B with its four added pastes and, on Unix, its one probe `--settings`; every relied row, `LedgerRow::relied()`: twenty-one on Unix, the first eighteen elsewhere)»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:8 «and Run E, the `--resume` start of Run B's first session) stamp every relied ledger row (`LedgerRow::relied()`: all twenty-two on Unix, elsewhere nineteen, the first eighteen and `resume-session`)»
  replace-with: «(print probe + all five interactive runs, Run B with its four added pastes and, on Unix, its one probe `--settings`, and Run E, the `--resume` start of Run B's first session; every relied row, `LedgerRow::relied()`: twenty-two on Unix, elsewhere nineteen, the first eighteen and `resume-session`)»

- leaf: .claude/rules/verification-harness.md:39   class: stale
  leaf-text: «under `--resume <id>` the launch SessionStart is the recorded `SessionStart.default` with `source` `resume` and `session_id` the id given»
  source: .andromeda/test-plan.md:1077 «else its recorded `SessionStart.default` payload (a set without the variant, 2.1.288), with `source` set to `resume` and `session_id` set to the id given»
  replace-with: «under `--resume <id>` the launch SessionStart is the set's recorded `SessionStart.resume-1.json` when the set holds it (2.1.287), else its recorded `SessionStart.default` (a set without the variant, 2.1.288), with `source` `resume` and `session_id` the id given»

- leaf: .claude/rules/verification-harness.md:42   class: omission
  leaf-text: «Built for `viola revive`'s tests: `--resume <id>`»
  source: .andromeda/test-plan.md:1094 «and two for `viola revive`'s tests and, under the fake agent, verify's Run E, `--resume <id>` (the launch SessionStart reports source `resume` and that id)»
  replace-with: «Built for `viola revive`'s tests and, under the fake agent, verify's Run E: `--resume <id>`»

- leaf: .claude/rules/verification-harness.md:46   class: omission
  leaf-text: «the framing variants (`paste-<n>`, `clear-<n>`) and the status line variants (`StatusLine.default-1.json`, `StatusLine.rate-limits-1.json`), which `contract_fake_agent_drift` replays byte for byte»
  source: .andromeda/test-plan.md:408 «the dialog variants, the framing variants and the resume variant (`fake_agent_resume_replay_matches_every_recorded_resume_set`), including forwarding `annotations` (S8)»
  replace-with: «the framing variants (`paste-<n>`, `clear-<n>`), the status line variants (`StatusLine.default-1.json`, `StatusLine.rate-limits-1.json`) and the resume variant (`SessionStart.resume-1.json`, by `fake_agent_resume_replay_matches_every_recorded_resume_set`), which `contract_fake_agent_drift` replays byte for byte»
