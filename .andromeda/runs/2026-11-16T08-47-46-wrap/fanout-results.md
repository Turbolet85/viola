# Fan-out results — 2026-11-16-local-command-delivery

a11y-plan: add 0 · set 0 · retire 0 · none 2 · ops-a11y-plan.md
architecture: add 3 · set 9 · retire 0 · none 1 · ops-architecture.md
design-system: add 2 · set 1 · retire 0 · none 1 · ops-design-system.md
layout-templates: add 2 · set 1 · retire 0 · none 1 · ops-layout-templates.md
obs-plan: add 2 · set 8 · retire 0 · none 0 · ops-obs-plan.md
security-plan: add 5 · set 3 · retire 0 · none 0 · ops-security-plan.md
test-plan: add 9 · set 6 · retire 1 · none 2 · ops-test-plan.md
test-plan (main): retire 1 · ops-main-test-plan.md

Q1 — the answer card (`answer-card.md`): the founder's own answers to the P4 forks of this chunk, relayed verbatim by
the overseer — `inputs#I2` Q2 (transcript tag) and Q3 (by command name), `inputs#I4` (Run B's seventh paste);
ratified as standing rows by the wrap directive `inputs#I6` (1).

ops-a11y-plan.md

ops-architecture.md
decision/local-commands-by-ledger · escalate → Q1 — check 1: boundary widening (classification by name), inputs#I2 Q3 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
decision/send-confirmed-after-the-fact · apply — check 1: accurate this-chunk addition (unconfirmable echo send with no transcript, report Symbols / APIs)
decision/unconfirmed-delivery-not-delivered · apply — check 1: accurate this-chunk addition (as above)
decision/every-send-confirmed · apply — check 1: accurate this-chunk addition (the echo confirmer)
contract/send-results/unconfirmable · apply — check 1: accurate this-chunk addition (`echo_from` → `Nothing`)
contract/ok-payload/unconfirmable · apply — check 1: accurate this-chunk addition (as above)
contract/events-ndjson/send-confirmed-order · apply — check 1: accurate this-chunk addition (`Match::Confirmed(<UTC ms now>)`)
contract/channel-methods/hook-event · escalate → Q1 — check 1: boundary widening (the transcript path crosses the channel), inputs#I2 Q2 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
contract/hook-event/notification · escalate → Q1 — check 1: boundary widening, inputs#I2 Q2 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
decision/local-command-echo-confirmation · escalate → Q1 — check 1: boundary widening (the transcript tag as echo), inputs#I2 Q2 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
contract/hook-event/transcript · escalate → Q1 — check 1: boundary widening, inputs#I2 Q2 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
decision/local-command-echo-probe · escalate → Q1 — check 1: boundary widening (verify's child set gains a paste), inputs#I4 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4

ops-design-system.md
contract/send-readback/unconfirmable · apply — check 1: accurate this-chunk addition (six names; `/mcp` with no transcript)
contract/send-readback/mcp · apply — check 1: accurate this-chunk addition (the echo read back, no new word)
decision/local-commands-by-name · reject — check 2: the fact's home is architecture decision/local-commands-by-ledger

ops-layout-templates.md
contract/viola-send-output/unconfirmable · apply — check 1: accurate this-chunk addition (six names; `/mcp` with no transcript)
contract/viola-send-output/read-back-mcp · apply — check 1: accurate this-chunk addition
decision/local-command-by-name · reject — check 2: the fact's home is architecture decision/local-commands-by-ledger

ops-obs-plan.md
decision/claude-internals-not-a-contract · escalate → Q1 — check 1: boundary widening (the transcript tag count read), inputs#I2 Q2 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
decision/transcripts-not-a-contract · escalate → Q1 — check 1: boundary widening, inputs#I2 Q2 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
contract/send-path-log-events/event-order · apply — check 1: accurate this-chunk addition (an echo-confirmed send awaits no hook line)
contract/send-log-fields/hook-invoked · apply — check 1: accurate this-chunk addition (as above)
contract/send-refused-detail/no-prompt-submitted · apply — check 1: accurate this-chunk addition (`send_local_command_echo_without_a_tag_is_no_prompt_submitted_at_the_window`, New text, by line)
decision/refusal-telemetry · escalate → Q1 — check 1: boundary widening (the transcript read's refusals), inputs#I2 Q2 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
decision/deterministic-emission · escalate → Q1 — check 1: boundary widening (confirmation keyed on the transcript tag), inputs#I2 Q2 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
decision/deterministic-event-emission · escalate → Q1 — check 1: boundary widening, inputs#I2 Q2 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
decision/transcript-echo-read-uninstrumented · apply — check 1: accurate this-chunk addition (inside `run.confirm_window`, no new line, report Coverage)
decision/transcript-path-and-bytes-never-kept · reject — check 2: the fact's home is security-plan decision/transcript-path-redacted

ops-security-plan.md
decision/compiled-classification-sources · escalate → Q1 — check 1: boundary widening (classification by name), inputs#I2 Q3 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
decision/shared-validators-in-core · escalate → Q1 — check 1: boundary widening (the transcript path validator), inputs#I2 Q2 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
contract/wrapper-channel-methods/hook-event · escalate → Q1 — check 1: boundary widening, inputs#I2 Q2 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
decision/transcript-path-shape-check · escalate → Q1 — check 1: boundary widening, inputs#I2 Q2 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
decision/transcript-read-bounded · escalate → Q1 — check 1: boundary widening, inputs#I2 Q2 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
decision/transcript-path-redacted · escalate → Q1 — check 1: boundary widening, inputs#I2 Q2 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
decision/local-command-echo-confirmation · reject — check 2: the fact's home is architecture decision/local-command-echo-confirmation
decision/verify-echo-probe-residuals · escalate → Q1 — check 1: boundary widening (the paste's residuals, named in the founder's answer), inputs#I4 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4

ops-test-plan.md
contract/driver-send/local-command · escalate → Q1 — check 1: boundary widening (classification by name), inputs#I2 Q3 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
decision/local-command-post-condition · escalate → Q1 — check 1: boundary widening (`/mcp` by the transcript tag), inputs#I2 Q2 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
decision/local-command-unconfirmable · reject — check 7: not_master type 5 — the command list is the code's inventory; the standing row is incomplete, so main sets it shorter (ops-main-test-plan.md)
contract/send-result/unconfirmable · apply — check 1: accurate this-chunk addition
decision/fake-agent-replays-recorded-bytes · apply — check 1: accurate this-chunk addition (`--transcript` on the launch SessionStart, Deviations 2)
contract/fake-agent-hooks/stdin-payload · apply — check 1: accurate this-chunk addition (as above)
decision/local-command-exact-match · reject — check 2: its heir decision/local-command-by-name is rejected; main retires the row with no heir (ops-main-test-plan.md)
decision/local-command-by-name · reject — check 2: the fact's home is architecture decision/local-commands-by-ledger
decision/transcript-path-shape-check · reject — check 2: the fact's home is security-plan decision/transcript-path-shape-check
contract/hook-event-method/transcript · reject — check 2: the fact's home is architecture contract/hook-event/transcript
decision/transcript-read-bounded-count-only · reject — check 2: the fact's home is security-plan decision/transcript-read-bounded
decision/local-command-echo-row-check · reject — check 7: not_master type 4 — how one ledger row's check is built, the code holds it
decision/verify-echo-probe-paste · reject — check 2: the fact's home is architecture decision/local-command-echo-probe
contract/fake-agent-cli/transcript · apply — check 1: accurate this-chunk addition
contract/fake-agent-input/echo-command · apply — check 1: accurate this-chunk addition
decision/stamping-verify-passes-transcript · apply — check 1: accurate this-chunk addition (report Harness / gate surface)

ops-main-test-plan.md
decision/local-command-unconfirmable · apply — check 7: the correction of a row left incomplete, shorter than the proposer's not_master set
decision/local-command-exact-match · escalate → Q1 — check 1: boundary widening (classification by name), inputs#I2 Q3 · resolved: apply (Q1) — delegate overseer, 2026-11-16: «Confirm all 21 (Recommended)», answer-card.md; the ratification the founder's own answers, relayed verbatim, inputs#I2 Q2 · Q3 and inputs#I4
