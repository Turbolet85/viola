@task_prose part=chunk
`viola verify`'s typed run (the probe paste, then the long, tag-like, newline, `/compact` and local-command pastes) decides the claude input box is up from the input box itself, no longer from a footer literal that a transient footer notice («Update available!», a usage warning) displaces after a turn.
The input-box readiness signature in the viola-agent-claude screen signatures changes, with its capability-ledger row(s) and a `viola verify` probe for it; the fake agent's screens and recorded fixtures gain a footer-notice replay.
It re-tests the measured diagnosis (footer-literal readiness displaced by a footer notice) rather than assuming it.
It re-measures by one live `viola verify` on the newest installed claude CLI, every ledger row pass but the account-dependent statusline-rate-limits, pastes landing while a footer notice stands, stamps written by verify itself; each live start is a named operator step waiting on the founder's cap.
It turns CI's `supply-chain` job green again: the cargo deny advisories against rmcp 3.4.1 (bump or reasoned per-advisory ignore) and rustix 1.1.5 (lockfile bump).
@task_file n=1 path=src/cmd/verify/typed/turns.rs mark=edits
what: viola verify's typed run, its paste turns and its input-box readiness wait — edits: `input_box_up`, its callers `framing_turns`, `turn`'s settle, `Run`'s `box_wait`
@task_file n=2 path=crates/viola-agent-claude/src/screen.rs mark=edits
what: viola-agent-claude's screen signatures, the readiness literals the screen is matched against — edits: `SIGNATURES.input_box`
@task_file n=3 path=ledger/stamps.json
what: viola verify's stamps for each claude CLI version, written only by verify
@task_file n=4 path=Cargo.toml mark=edits
what: the workspace manifest and its third-party pins — edits: `[workspace.dependencies]` rmcp's ratified range
@task_file n=5 path=Cargo.lock mark=edits
what: the workspace lockfile — edits: the rmcp and rustix entries
@task_file n=6 path=deny.toml mark=edits
what: the cargo deny supply-chain policy — edits: the advisories `ignore` list
@task_item n=1 part=needs
viola verify's output: its step line per probe and its interrupted pair and exit, which the typed run's readiness change and the new probe print through.
@task_item n=2 part=needs
The screen signatures as the only screen content viola reads: a readiness literal is a signature, and the chunk may read nothing of the screen beyond signatures nor inspect claude's content otherwise.
@task_item n=3 part=needs
Indirect, through the shared input-box signature: send's input-not-ready refusal and the run pump's readiness gate read the same signature, so a changed literal reaches them unless research keeps them apart.
@task_item n=4 part=needs
The fake agent and its recorded screens per claude CLI version, replayed in CI: the new footer-notice fixtures are records the boundary tests are sequenced on.
@task_item n=5 part=needs
Live readings never gate: the proof run on the live claude CLI is an operator step, never a CI gate; CI replays the recorded screens only.
@task_item n=6 part=needs
Time bounds fall on the driver only: the typed run's box wait and turn settle are driver-side bounds, never a time limit on the human.
@task_item n=7 part=needs
Wrapped TUI reads as unwrapped and the TUI terminal belongs to the child: verify reads the screen and changes no Claude-facing behaviour.
@task_item n=8 part=needs
Fixed CLI strings pinned by literal: any new or changed verify step label or ledger row name keeps its literal pin.
@task_item n=9 part=needs
CI's test gate: the supply-chain job and the coverage suite must pass with no skips, no retries and zero flakiness after the chunk.
@task_standing n=1
convention: each new test's a11y-relevant assertions live inside the existing test suites, with no separate a11y driver or suite
home: 1-a11y-scope-summary · 9-ci-integration
@task_standing n=2
convention: each new test fails as a nextest failure; a test may not be skipped and the CI test gate allows no skips
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=3
convention: each new test is deterministic, never retried, and a breach fails the build
home: 10-slo-invariants-a11y-budgets
@task_standing n=4
convention: each new boundary test runs on recorded fixtures, sequenced on records, never on a live claude start
home: keyboard-test-harness
@task_item n=1 part=not_needed
The viola list board's colour, SGR, dim, bold, amber and NAME truncation rows: the chunk prints nothing on the board.
@task_item n=2 part=not_needed
The revive --json refusal documents, revive --list rows and start-against-holder refusals: the chunk touches no revive or start path.
@task_item n=3 part=not_needed
link and unlink, plugin install, hook event and dialog calls, and MCP frame tool refusals: no such verb or tool changes.
@task_item n=4 part=not_needed
The --help group labels, usage line and handoff row: the chunk adds no verb.
@task_item n=5 part=not_needed
The budget gate, config.json budget keys and dialog hold: the chunk sends nothing under the budget.
@task_item n=6 part=not_needed
The manual Orca and keyboard passes and their screen-reader test setups: they gate nothing and the chunk changes no human-facing output layout.
@task_item n=7 part=not_needed
The absences (web or graphical frontend, touch surface, internationalization, rendered page): the chunk adds none.
