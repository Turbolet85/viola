# Fan-out results — 2026-11-17-standing-live-rig (wrap 2026-11-17T07-48-22Z-wrap)

architecture: add 8 · set 6 · retire 0 · none 0 · ops-architecture.md
security-plan: add 6 · set 2 · retire 0 · none 1 · ops-security-plan.md
design-system: none · ops-design-system.md — reason: the chunk adds no product surface; no viola CLI wording, colour token or output rule moves
layout-templates: none · ops-layout-templates.md — reason: the chunk adds no product surface and changes no viola CLI output wording
test-plan: add 31 · set 10 · retire 1 · none 0 · ops-test-plan.md
obs-plan: add 3 · set 0 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: add 1 · set 1 · retire 0 · none 0 · ops-a11y-plan.md

Nothing was removed by a fix (no file was fixed).

## Dispositions

ops-architecture.md
name/binary/viola-live-rig · apply — check 5: the plan's expected amendment (architecture §Occupied Resources, the binary viola-live-rig), the report's Symbols / APIs bullet; a new binary is a resource the register enumerates
name/other/agent-run-rig · apply — check 5: the plan's expected amendment (architecture §Occupied Resources, the env name AGENT_RUN_RIG), the report's Symbols / APIs bullet
decision/standing-live-rig · reject — check 2: the fact's home is test-plan decision/standing-live-rig (the plan's expected amendment names test-plan · decision/live-readings-not-a-test-layer)
decision/live-rig-private-claude-config · reject — check 2: the fact's home is security-plan decision/live-rig-isolated-child
contract/viola-live-rig-exit-codes/0 · reject — check 1: Registry over-reach (rul-ochhsjmf) — a test-only binary's per-item exit codes are realization inside a registered binary; no other test binary's exit codes are registered
contract/viola-live-rig-exit-codes/1 · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/viola-live-rig-exit-codes/0
contract/viola-live-rig-exit-codes/2 · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/viola-live-rig-exit-codes/0
contract/viola-live-rig-exit-codes/3 · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/viola-live-rig-exit-codes/0
decision/test-binaries-never-released · apply — check 1: Accurate this-chunk addition — viola-live-rig is a test-only binary kept out of release (report: Harness / gate surface, `release-check: viola only`), and the row's list must name it
decision/harness-env-unread-by-product · apply — check 1: Accurate this-chunk addition — AGENT_RUN_RIG is a harness variable set on the rig's child only (report: Symbols / APIs)
decision/no-typing-into-trust-dialogs · reject — check 2: the row says viola types into no trust dialog and stays true (the rig is no viola); the rig's grant has its home in security-plan decision/live-round-rig-keys-own-pty
decision/submitted-form-prompt-match · apply — check 6: R2's measured trim (report Cross-project claims; the close's Sweep reach, inputs#I9 item 4) makes the why's «an assumption not yet read live, either end held as the candidate» false
decision/vt100-screen-model · apply — check 1: Accurate this-chunk addition — vt100 added to viola-e2e as the rig's screen model (report: Dependencies)
decision/vt100-readiness-gate · apply — check 1: Accurate this-chunk addition — the row's «serves only as a pre-send readiness gate» is false once the rig uses vt100 as its screen model (report: Dependencies)

ops-security-plan.md
decision/live-round-rig-keys-own-pty · escalate → Q1 — check 1: Boundary widening the founder answered himself to a P4 fork (rul-bznresq5): the rig's key grant, inputs#I2 item 2 · resolved: apply (Q1) — delegate Viola overseer, 2026-11-17: «Confirm all 7» (answer-card.md); the ratification is the founder's own answer relayed verbatim, inputs#I2 item 2 · as: the rule keeps that each chunk's grant is the founder's anew (his answer granted the keys for THIS chunk)
decision/real-cli-local-only · reject — check 2: the row stays true (in CI viola verify runs only against the fake agent); the rig's CI and cap rule has its home in test-plan decision/standing-live-rig
decision/live-rig-credentials-copy · escalate → Q1 — check 1: Boundary widening the founder answered himself to a P4 fork (rul-bznresq5): a test-only site handling the login copy, inputs#I2 items 2 and 4 · resolved: apply (Q1) — delegate Viola overseer, 2026-11-17: «Confirm all 7» (answer-card.md); the ratification is the founder's own answer relayed verbatim, inputs#I2 items 2 and 4
decision/live-rig-isolated-child · escalate → Q1 — check 1: Boundary widening the founder answered himself to a P4 fork (rul-bznresq5): the rig's child environment and private config, inputs#I2 items 2 and 4 · resolved: apply (Q1) — delegate Viola overseer, 2026-11-17: «Confirm all 7» (answer-card.md); the ratification is the founder's own answer relayed verbatim, inputs#I2 items 2 and 4
decision/live-rig-no-key-into-imports-modal · reject — check 2: the fact's home is security-plan decision/live-round-rig-keys-own-pty (any key outside the grant writes nothing); the imports modal's live reading is a PROVISIONAL carry of «first readings on the standing rig» (inputs#I9 item 3), not this chunk's measurement
decision/live-rig-kills-only-its-own · reject — check 2: the fact's home is test-plan decision/live-rig-reaches-only-its-own
name/binary/viola-live-rig · reject — check 2: the fact's home is architecture name/binary/viola-live-rig
name/other/agent-run-rig · reject — check 2: the fact's home is architecture name/other/agent-run-rig

ops-test-plan.md
decision/live-readings-not-a-test-layer · escalate → Q1 — check 1: Boundary widening the founder answered himself to a P4 fork (rul-bznresq5): the committed rig, inputs#I2 item 4 · resolved: apply (Q1) — delegate Viola overseer, 2026-11-17: «Confirm all 7» (answer-card.md); the ratification is the founder's own answer relayed verbatim, inputs#I2 item 4
decision/standing-live-rig · escalate → Q1 — check 1: Boundary widening the founder answered himself to a P4 fork (rul-bznresq5): the committed rig, inputs#I2 item 4 (the heir of the retired row) · resolved: apply (Q1) — delegate Viola overseer, 2026-11-17: «Confirm all 7» (answer-card.md); the ratification is the founder's own answer relayed verbatim, inputs#I2 items 2 and 4 · as: the live starts run by hand under the founder's cap for each chunk, never a fixed «2 starts plus 1 spare» (his 2 + 1 was for THIS chunk)
decision/live-rig-key-grant · reject — check 2: the fact's home is security-plan decision/live-round-rig-keys-own-pty
decision/live-rig-isolates-each-start · reject — check 2: the fact's home is security-plan decision/live-rig-credentials-copy and decision/live-rig-isolated-child
decision/live-rig-reaches-only-its-own · reject — check 7: not_master type 4 — the code holds how the rig's census marker and stop are built (test-plan-fit.md)
contract/live-rig-exit-codes/0 · reject — check 1: Registry over-reach (rul-ochhsjmf) — a test-only binary's per-item exit codes; the code holds them
contract/live-rig-exit-codes/1 · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-exit-codes/0
contract/live-rig-exit-codes/2 · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-exit-codes/0
contract/live-rig-exit-codes/3 · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-exit-codes/0
contract/live-rig-refusal/ci · reject — check 1: Registry over-reach (rul-ochhsjmf) — per-item refusal words of a test-only binary; the code holds them (the CI refusal itself stands in decision/standing-live-rig)
contract/live-rig-refusal/spent · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-refusal/ci
contract/live-rig-refusal/cap · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-refusal/ci
contract/live-rig-refusal/version · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-refusal/ci
contract/live-rig-refusal/token-window · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-refusal/ci
contract/live-rig-refusal/leftover · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-refusal/ci
contract/live-rig-refusal/ledger · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-refusal/ci
contract/live-rig-refusal/scratch · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-refusal/ci
contract/live-rig-refusal/credentials · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-refusal/ci
contract/live-rig-refusal/spawn · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-refusal/ci
contract/live-rig-refusal/viola · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-refusal/ci
contract/live-rig-refusal/shape · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-refusal/ci
contract/live-rig-refusal/no-session-start · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-refusal/ci
contract/live-rig-refusal/texts · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-refusal/ci
contract/live-rig-refusal/key-not-granted · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-refusal/ci
contract/live-rig-refusal/key-not-ready · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-refusal/ci
contract/live-rig-refusal/key-write-failed · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-refusal/ci
contract/live-rig-refusal/trust-unpassed · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-refusal/ci
contract/live-rig-trust-moves/count · reject — check 1: Registry over-reach (rul-ochhsjmf) — per-item words of a test-only binary's reading; the code holds them (the report's departure `trust-moves: unread` is judged here: built as the fourth no-key word, a code-held detail)
contract/live-rig-trust-moves/needs-up · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-trust-moves/count
contract/live-rig-trust-moves/needs-third-move · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-trust-moves/count
contract/live-rig-trust-moves/unread · reject — check 1: Registry over-reach (rul-ochhsjmf), as contract/live-rig-trust-moves/count
contract/fake-agent-screens/trust-dialog · apply — check 5: the plan's expected amendment (test-plan §Test Data, the fake agent's trust selector under --screens), in 2.1.325's form (report: Symbols / APIs; Deviations 3)
decision/no-screen-parsing · reject — check 7: not_master type 4 — the exception named the rig's vt100 grid size, which the code holds (test-plan-fit.w2.md); the row stays false without an exception, so main writes the shorter set in ops-main-test-plan.md
decision/never-parse-rendered-child-screen · apply — check 1: Accurate this-chunk addition, as decision/no-screen-parsing
decision/no-child-screen-assertions · apply — check 1: Accurate this-chunk addition, as decision/no-screen-parsing
decision/discipline-at-every-layer · apply — check 1: Accurate this-chunk addition, as decision/no-screen-parsing
decision/no-blanket-approval-tests · escalate → Q1 — check 1: Boundary widening the founder answered himself to a P4 fork (rul-bznresq5): the rig passes a read trust dialog under its grant, inputs#I2 item 2 · resolved: apply (Q1) — delegate Viola overseer, 2026-11-17: «Confirm all 7» (answer-card.md); the ratification is the founder's own answer relayed verbatim, inputs#I2 item 2
decision/real-claude-local-only · escalate → Q1 — check 1: Boundary widening the founder answered himself to a P4 fork (rul-bznresq5): live measurement through the committed rig, never in CI, inputs#I2 item 4 · resolved: apply (Q1) — delegate Viola overseer, 2026-11-17: «Confirm all 7» (answer-card.md); the ratification is the founder's own answer relayed verbatim, inputs#I2 item 4
contract/fake-agent-events/session-start · apply — check 5: the plan's expected amendment (test-plan §Test Data), the fake's launch SessionStart behind the trust form (report: Symbols / APIs)
contract/fake-agent-exit/exit-triggers · apply — check 5: the plan's expected amendment (test-plan §Test Data), Enter on «No» exits 1 (report: Symbols / APIs)
contract/fake-agent-input/keystroke · apply — check 5: the plan's expected amendment (test-plan §Test Data, the fake's DEL erase), the box rows and DEL (report: Symbols / APIs; the departures «the fake draws its box rows» and `box-text: absent` judged here)
decision/send-confirmation-trailing-trim-match · apply — check 6: R2's measured trim (inputs#I9 item 4) makes the why's «the trim set is an assumption, not a live reading» false

ops-obs-plan.md
decision/live-rig-records-closed-words · reject — check 2: the fact's home is security-plan decision/live-rig-credentials-copy (no credential, host path or config value in any record) and decision/live-rig-isolated-child (the config hash and mtime)
decision/live-rig-frames-kept-local · reject — check 2: the fact's home is security-plan decision/live-session-screens-kept-local, which already says it
decision/no-real-cli-start-in-ci · reject — check 2: the fact's home is test-plan decision/standing-live-rig

ops-a11y-plan.md
decision/no-screen-content-parsing · apply — check 1: Accurate this-chunk addition — the rig reads the child's screen beyond compiled signatures (the selector's place, box-text) into closed words (report: Symbols / APIs rig::screen)
decision/live-rig-sole-vendor-pty-typing-site · reject — check 2: the fact's home is security-plan decision/live-round-rig-keys-own-pty

ops-main-test-plan.md
decision/no-screen-parsing · apply — check 7: the shorter set the not_master on the proposer's set owes — the row «Tests never parse the rendered child screen» is false once the rig's rehearsal asserts on words read from its child's screen (report: Symbols / APIs rig::screen)
