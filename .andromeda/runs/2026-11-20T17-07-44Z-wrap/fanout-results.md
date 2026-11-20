# Fan-out results — 2026-11-20-a-new-cli-version-stamped-automatically (wrap 2026-11-20T17-07-44Z-wrap)

architecture: add 7 · set 5 · retire 0 · none 2 · ops-architecture.md
security-plan: add 6 · set 2 · retire 0 · none 0 · ops-security-plan.md
design-system: add 10 · set 0 · retire 0 · none 1 · ops-design-system.md
layout-templates: add 4 · set 2 · retire 0 · none 0 · ops-layout-templates.md
test-plan: add 6 · set 5 · retire 0 · none 2 · ops-test-plan.md
obs-plan: add 7 · set 3 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: add 2 · set 9 · retire 0 · none 0 · ops-a11y-plan.md
main: ops-main-architecture.md (add 2 · set 1) · ops-main-a11y-plan.md (set 1)

The licence (inputs#I5 item 3): the rows the phase block's §5 names as reached (`.andromeda/runs/2026-11-20T14-44-21Z-phase/founder-forks.md` §5) and the five rows of the founder's three P4 answers; nothing wider. A reached row outside them goes to `founder-forks.md` of this run dir at its most conservative option (the row unchanged). A §5 row the built change does not reach is left unchanged and said so.

## ops-architecture.md
decision/run-start-order · reject — directive item 5: the set drops the row's qualifying «only» («and only then spawns the child»); main's own set in ops-main-architecture.md carries the order with it
decision/on-disk-file-naming · escalate → Q1 — P4-F2 «Одно именованное исключение»: the named exception `ledger/verify.lock`, the except cell empty before (member added, none dropped) · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
contract/instance-snapshot/cli-verified · escalate → Q1 — §5 architecture (from the second read): the final verdict · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
decision/run-terminal-child-only · reject — §5 lists it but the built change does not reach it: «While the child runs, viola run writes nothing … except the child's own output» still holds, the start's lines and verify's output end before the child starts (inputs#I5 item 3); row unchanged
decision/viola-home-resolution · reject — outside §5 and the five P4 rows: founder-forks.md F1 at its most conservative option, the row unchanged
decision/start-verifies-new-cli-version · escalate → Q1 — «A: verify перед сессией» and «Выключено, включаете один раз»: form (A), key default false, after the bind as built · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
decision/no-automatic-reverify-of-red · escalate → Q1 — «Как любой красный»: a red entry is an attempt, never re-verified automatically · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
decision/stop-during-start-verify · reject — superseded by main's add of the same handle (ops-main-architecture.md), which states the stop handler before the start line and «no stamp» (inputs#I5 item 4)
decision/automatic-verify-attempt-lock · reject — superseded by main's add of the same handle (ops-main-architecture.md), which states the under-lock re-read as built (report Deviations 1), owner-only, after the strict check, and carries test-plan's guard
name/other/ledger-verify-lock · escalate → Q1 — P4-F2: the one named lock file · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
contract/config-json/verify-new-cli · escalate → Q1 — widening (iv) and «Выключено, включаете один раз»: the key's home (config.json key rows stand in architecture §Occupied Resources) · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
contract/diag-spawn-subjects/auto-verify · reject — check 2: the fact's home is obs-plan contract/diag-subject/auto-verify

## ops-main-architecture.md
decision/run-start-order · escalate → Q1 — §5 architecture: the verify after the endpoint bind, the trust record after it (as built, P5 lean «after the endpoint bind») · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
decision/stop-during-start-verify · reject — check 7: not_master type 4 — How the start's stop handler tears down its verify child (order, TERM, 30 s grace, KILL, re-raise) is one mechanism whose basis states no reason for it, so the code and its test hold it; escalated (Q1), not asked (the stop's outward promise stands in obs-plan contract/run-process-log/auto-verify-stop)
decision/automatic-verify-attempt-lock · escalate → Q1 — P4-F2 and the overseer's technical answer 5, as built (report Deviations 1) · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word

## ops-security-plan.md
decision/start-writes-trust-record-flag-only · reject — directive item 5: the set drops «only» and «where none stands»; §5 lists the row, but «On a verified CLI only» holds on the final verdict and the order stands in architecture decision/run-start-order: the built change does not reach it, row unchanged
decision/config-cannot-disable-controls · reject — outside §5 and the five P4 rows: founder-forks.md F2 at its most conservative option, the row unchanged
contract/config-json/verify-new-cli · reject — check 2: the fact's home is architecture contract/config-json/verify-new-cli
decision/start-verifies-new-cli-once · reject — check 2: the fact's home is architecture decision/start-verifies-new-cli-version
decision/verify-attempt-lock · reject — check 2: the fact's home is architecture decision/automatic-verify-attempt-lock
decision/start-verify-child-isolated · reject — check 7: not_master type 4 — The basis is a symbols list that retells how the start's auto-verify spawns its child and states no reason or named threat for it, so the code holds it; escalated (Q1), not asked
decision/start-verify-stop-leaves-nothing · reject — check 2: the fact's home is architecture decision/stop-during-start-verify
decision/start-verify-lines-closed-words · reject — check 2: the fact's home is design-system contract/auto-verify-line/line-form

## ops-design-system.md
decision/verify-new-cli-key · reject — check 2: the fact's home is architecture decision/start-verifies-new-cli-version
contract/auto-verify-line/verifying · escalate → Q1 — §5 design-system: new §Surface: cli rows for the start's lines · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
contract/auto-verify-line/did-not-verify · escalate → Q1 — §5 design-system, as above · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
contract/auto-verify-line/earlier-red · escalate → Q1 — §5 design-system, as above · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
contract/auto-verify-line/verifying-elsewhere · escalate → Q1 — §5 design-system, as above · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
contract/auto-verify-line/no-stamp · escalate → Q1 — §5 design-system, as above (a verify that cannot start writes nothing and is no attempt) · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
contract/auto-verify-line/line-form · escalate → Q1 — §5 design-system, as above · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
decision/auto-verify-never-loops · reject — check 2: the fact's home is architecture decision/no-automatic-reverify-of-red and decision/automatic-verify-attempt-lock
decision/start-verify-uses-start-terminal · reject — check 2: the fact's home is layout-templates contract/viola-run/start-output
decision/stop-during-start-verify-ends-start · reject — check 2: the fact's home is architecture decision/stop-during-start-verify

## ops-layout-templates.md
contract/viola-run/start-output · escalate → Q1 — P4-F1 «Печатать, строки поправить» · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
contract/viola-revive/start-output · reject — P4 read it as not reached and it holds as built: «prints nothing of its own while the resumed child holds the terminal»; row unchanged
contract/run-revive-start/automatic-verify · reject — check 2: the fact's home is architecture decision/start-verifies-new-cli-version
contract/run-revive-start/automatic-verify-line · reject — check 2: the fact's home is design-system contract/auto-verify-line/line-form
contract/run-revive-start/automatic-verify-stop · reject — check 2: the fact's home is architecture decision/stop-during-start-verify
decision/automatic-verify-once-per-version · reject — check 2: the fact's home is architecture decision/no-automatic-reverify-of-red (this text names one clearer of the three)

## ops-test-plan.md
decision/human-never-blocked · reject — outside §5 and the five P4 rows (P4-F3 named the a11y-plan rows alone): founder-forks.md F3 at its most conservative option, the row unchanged
decision/run-checks-before-child-spawn · reject — outside §5 and the five P4 rows: founder-forks.md F4 at its most conservative option, the row unchanged (it reads short of the built order)
decision/single-writer-state-files · escalate → Q1 — §5 test-plan: verify stays the one writer, now also started by a start · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
contract/fake-agent-cli/argv-only-options · reject — outside §5 and the five P4 rows: founder-forks.md F5 at its most conservative option, the row unchanged
decision/fake-agent-argv-only · reject — outside §5 and the five P4 rows: founder-forks.md F5 at its most conservative option, the row unchanged
contract/config-json/verify-new-cli · reject — check 2: the fact's home is architecture contract/config-json/verify-new-cli
decision/one-verify-across-concurrent-starts · reject — check 2: the fact's home is architecture decision/automatic-verify-attempt-lock (main's, carrying this row's guard)
decision/attempted-version-not-reverified · reject — check 2: the fact's home is architecture decision/no-automatic-reverify-of-red
contract/run-start-verify-stop/signal · reject — check 2: the fact's home is architecture decision/stop-during-start-verify
contract/diag-subject/auto-verify · reject — check 2: the fact's home is obs-plan contract/diag-subject/auto-verify
contract/start-verify-lines/format · reject — check 2: the fact's home is design-system contract/auto-verify-line/line-form

## ops-obs-plan.md
contract/config-json/known-keys · escalate → Q1 — §5's contract/config-json/* class, read across masters, and the plan's Expected amendments (obs-plan · contract/config-json/known-keys): four keys → five · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
contract/diag-line-fields/process-start · escalate → Q1 — §5 obs-plan: the subject enum gains auto-verify · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
contract/run-terminal-output/child-output · reject — outside §5 and the five P4 rows (P4 read obs's run-silent rows as not reached): founder-forks.md F6 at its most conservative option, the row unchanged
contract/diag-subject/auto-verify · escalate → Q1 — §5 obs-plan: the closed subject for the start's verify child · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
contract/config-json/verify-new-cli · reject — check 2: the fact's home is architecture contract/config-json/verify-new-cli
contract/run-terminal-output/auto-verify-line · reject — check 2: the fact's home is design-system contract/auto-verify-line/line-form
decision/auto-verify-on-run-start-span · reject — check 7: not_master type 4 — The basis only retells how the new auto-verify module is built and states no reason, trade or ratification for the placement, so the code holds it; escalated (Q1), not asked
contract/run-process-log/auto-verify-stop · escalate → Q1 — the overseer's technical answer 6 under form (A), as built (report Deviations 2): the run log's exit pair on a stop · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
decision/auto-verify-lock-writes-no-line · reject — check 2: the fact's home is architecture decision/automatic-verify-attempt-lock
decision/attempted-cli-never-auto-verified · reject — check 2: the fact's home is architecture decision/no-automatic-reverify-of-red

## ops-a11y-plan.md
contract/viola-run-auto-verify/stderr-lines · reject — check 2: the fact's home is design-system contract/auto-verify-line/line-form
contract/config-json/verify-new-cli · reject — check 2: the fact's home is architecture contract/config-json/verify-new-cli
decision/human-keystrokes-never-blocked · reject — its except names the window's trigger «a claude version with no stamp entry», narrower than built (an entry short of a relied id); main's own set in ops-main-a11y-plan.md
contract/tui-passthrough/human-keystrokes · escalate → Q1 — P4-F3 «Исключение в обеих строках» · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
contract/tui-passthrough/viola-terminal-bytes · escalate → Q1 — P4-F1 «Печатать, строки поправить» · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word
contract/tui-passthrough/own-bytes · reject — outside §5 and the five P4 rows: founder-forks.md F7 at its most conservative option, the row unchanged
decision/zero-viola-bytes-on-the-terminal · reject — outside §5 and the five P4 rows: founder-forks.md F7, the row unchanged
decision/wheel-ignores-terminal-events · reject — outside §5 and the five P4 rows: founder-forks.md F7, the row unchanged
decision/portable-pty-outer-pty-driver · reject — outside §5 and the five P4 rows: founder-forks.md F7, the row unchanged
decision/time-bounds-fall-on-the-driver · reject — outside §5 and the five P4 rows: founder-forks.md F7, the row unchanged (the set also drops «blocking, refusing or»)
decision/time-bounds-never-on-human · reject — outside §5 and the five P4 rows: founder-forks.md F7, the row unchanged (the set also drops «refused or»)

## ops-main-a11y-plan.md
decision/human-keystrokes-never-blocked · escalate → Q1 — P4-F3 «Исключение в обеих строках»: one except naming the start's verify window before any child exists, the except cell empty before · resolved: apply (Q1) — delegate the overseer, 2026-11-20: «Confirm all 23 (Recommended)», PROVISIONAL until the founder's own word

## §5 rows the built change does not reach (no operation; each left unchanged, inputs#I5 item 3)
architecture: decision/verify-child-set (verify's own child set unchanged) · decision/stamps-single-writer · decision/verify-sole-stamps-writer · contract/ledger-stamps/writer (verify stays the one writer, each still true) · decision/unverified-cli-transport-only · decision/partial-gate-on-unverified-cli (an unverified start is as before) · decision/version-gate-strict-stamps-read (the same strict read, `stamps_reading`) · decision/config-keys-match-schema (still equal, guard green) · decision/run-terminal-child-only (above) · decision/real-claude-local-only · decision/no-live-claude-in-ci (off by default in every CI home).
security-plan: decision/stamps-written-only-by-verify · decision/unverified-cli-gate · decision/dialog-answers-need-verify-stamp · decision/version-gate-strict-stamps-read · contract/strict-modes-refusal/version-gate · contract/cli-exit-codes/12 · decision/controls-not-switchable · decision/config-tolerant-parse · decision/verify-real-cli-transcripts-accepted · decision/verify-echo-probe-residuals · decision/verify-context-probe-residual (the residuals read the same, now arising unattended too: widening (ii), ratified) · decision/driver-permission-policy-outside-viola · decision/start-writes-trust-record-flag-only (above) · decision/verify-probe-dirs-cleanup · decision/probe-artifacts-cleanup (a stop reaches verify by TERM; a KILL after the grace is the except's) · decision/strict-modes-check.
design-system: contract/viola-verify-output/* (verify's own lines, unchanged).
test-plan: decision/stamps-written-only-by-verify · decision/stamps-only-from-verify · decision/single-writers · decision/state-files-never-hand-written · contract/verify-stop/signal.
obs-plan: contract/viola-verify-refusal/stamp-write-refused (unchanged words, a new caller).
