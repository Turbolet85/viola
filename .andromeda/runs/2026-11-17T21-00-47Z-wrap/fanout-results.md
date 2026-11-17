# Fan-out results — 2026-11-17-trust-record-ledger-row (wrap 2026-11-17T21-00-47Z-wrap)

architecture: add 17 · set 15 · retire 0 · none 1 · ops-architecture.md
security-plan: add 3 · set 8 · retire 0 · none 1 · ops-security-plan.md
design-system: add 2 · set 0 · retire 0 · none 0 · ops-design-system.md
layout-templates: add 1 · set 0 · retire 0 · none 2 · ops-layout-templates.md
test-plan: add 5 · set 1 · retire 0 · none 0 · ops-test-plan.md
obs-plan: add 1 · set 7 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: add 1 · set 0 · retire 0 · none 1 · ops-a11y-plan.md

The answer card's authority: the founder's own answers R1–R4, his option labels verbatim, read 2026-11-17T18:59:41Z,
relayed by the Viola overseer (inputs#I4); the wrap directive (inputs#I2 item 2) names them ruled.

ops-architecture.md
decision/verify-child-set · escalate → Q1 — check 1: boundary widening (the ratified child set five → six, Run F), the founder's own answer R4 «Шестой запуск, запись убирается» (inputs#I4); answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/permission-request-question-stays-null · apply — check 1: routine count, the relied rows 22 → 23 (report Counts / qualifiers moved)
decision/verify-run-e-resumes-run-b-first-session · apply — check 1: routine count, the runs A–E → A–F (report Counts)
decision/verify-removes-only-own-dirs · escalate → Q1 — check 1: widening R4, verify's own record removed on every exit (inputs#I4); answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/claude-shapes-in-one-crate · escalate → Q1 — check 1: widening of the one-crate invariant for test-only code, the founder's R1 «Сначала строка реестра» (the new entry holds the fake agent reading the record, inputs#I4); answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/claude-first-no-adapter-trait · escalate → Q1 — check 1: the same test-only exception as claude-shapes-in-one-crate, R1 (inputs#I4); answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/claude-shapes-isolated · escalate → Q1 — check 1: the same test-only exception, R1 (inputs#I4); answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/env-strip-names-only · escalate → Q1 — check 1: widening, the one CLAUDE_CONFIG_DIR value read, R2 «Файл ребёнка, ключ как у CLI» (viola reads that ONE environment value, for that one purpose) with R4 «into the config its child reads» (inputs#I4); answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/env-strip-reads-names-only · escalate → Q1 — check 1: the same env value read, R2 with R4 (inputs#I4); answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/env-var-viola-prefix · reject — the proposer's value drops that PATH is read for the child; main's own set in ops-main-architecture.md carries the change
decision/no-environment-configuration · escalate → Q1 — check 1: the same env value read, R2 with R4 (inputs#I4); answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
contract/config-json/claude-env-keep · escalate → Q1 — check 1: the same env value read, R2 with R4 (inputs#I4); answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
contract/ledger-stamps/rows · apply — check 5: expected amendment (a row's value may be the writer's word), report Schema / config
contract/ledger-stamps/content · apply — check 2: the same stamps fact as contract/ledger-stamps/rows, stated in the envelope row that already lists the values; consistent
contract/ledger-stamps/measured · reject — check 7: not_master type 5 — the set lengthens an inventory of probe readings the code and the stamps hold; the row stays true without it
decision/verify-own-trust-record · escalate → Q1 — check 1: widening R4, verify's own temp-dir record written into the config its child reads and removed (inputs#I4); answer card; its home is architecture (check 2: layout-templates' add rejected) · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/trust-record-ledger-row · escalate → Q1 — check 1: the 23rd relied row and its sixth run, R1 with R4 (inputs#I4); answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/trust-writer-takes-config-lock · reject — check 7: not_master type 4 — a retelling of how the writer takes the lock and retries, which the code holds; escalated (Q1), not asked
contract/trust-writer-words/written · apply — check 1: a closed set of code words reaching the stamps (report Symbols / APIs `TrustWord`, 14 words); no widening
contract/trust-writer-words/removed · apply — check 1: the closed set, as above
contract/trust-writer-words/config-absent · apply — check 1: the closed set, as above
contract/trust-writer-words/config-unreadable · apply — check 1: the closed set, as above
contract/trust-writer-words/config-link · apply — check 1: the closed set, as above
contract/trust-writer-words/config-not-regular · apply — check 1: the closed set, as above
contract/trust-writer-words/config-foreign · apply — check 1: the closed set, as above
contract/trust-writer-words/config-oversize · apply — check 1: the closed set, as above
contract/trust-writer-words/config-malformed · apply — check 1: the closed set, as above
contract/trust-writer-words/config-dir-unusable · apply — check 1: the closed set, as above
contract/trust-writer-words/home-unknown · apply — check 1: the closed set, as above
contract/trust-writer-words/key-present · apply — check 1: the closed set, as above
contract/trust-writer-words/lock-held · apply — check 1: the closed set, as above
contract/trust-writer-words/lost · apply — check 1: the closed set, as above

ops-main-architecture.md
decision/env-var-viola-prefix · escalate → Q1 — check 1: the CLAUDE_CONFIG_DIR value read, R2 with R4 (inputs#I4); answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»

ops-security-plan.md
decision/no-writes-under-claude-home · escalate → Q1 — check 1: widening R4, verify's own record the bounded exception; the product's write not amended in (inputs#I4, inputs#I2 item 2); answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/viola-writes-nothing-in-claude-dir · escalate → Q1 — check 1: widening R4, as above; answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/nothing-written-under-claude-home · escalate → Q1 — check 1: widening R4, as above; answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/explicit-mode-before-write · apply — check 1: no rule; no widening — the writer sets the replaced Claude config's own mode explicitly before any byte (report Outcome: the mode kept), so the row stays true with the exception named
decision/r8-claude-env-strip · escalate → Q1 — check 1: the CLAUDE_CONFIG_DIR value read, R2 with R4 (inputs#I4); answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/credential-strip-by-name-only · escalate → Q1 — check 1: the same env value read, R2 with R4; answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/env-not-config-channel · escalate → Q1 — check 1: the same env value read, R2 with R4; answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/no-stored-secrets · escalate → Q1 — check 1: the same env value read, R2 with R4; answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/trust-record-writer-guarded · escalate → Q1 — check 1: the write's bound, R2 (file, refusals, nothing created) and R3 (lock when free, three attempts) — narrower where it adds the owner and regular-file refusals (inputs#I4); answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/trust-record-closed-words · apply — check 1: the closed outcome words and their redaction, report Symbols / APIs and Coverage (PII redacted); no widening
decision/test-verify-under-test-home · reject — check 2: the fact's home is test-plan decision/verify-under-test-owned-user-home

ops-main-security-plan.md
decision/verify-probe-dirs-cleanup · escalate → Q1 — check 5: expected amendment, verify's own record on the cleanup's exits, R4 (inputs#I4); answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/probe-artifacts-cleanup · escalate → Q1 — check 5: expected amendment, as above; answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»

ops-design-system.md
contract/viola-verify-output/trust-record-names-nothing · apply — check 1: no widening; the report's Coverage (no path, key or value in any line, stamp or summary)
decision/session-folder-trusted-word-on-pass · reject — check 3: the report carries no display word; «Session folder trusted by viola» is a route entry's title, never a CLI word

ops-layout-templates.md
decision/verify-writes-own-trust-record · reject — check 2: the fact's home is architecture decision/verify-own-trust-record

ops-test-plan.md
contract/verify-stop/signal · apply — check 5: expected amendment family (verify removes its own record on a SIGTERM stop), report Outcome
decision/verify-under-test-owned-user-home · apply — check 5: expected amendment decision/stamping-verify-passes-transcript and §3 boot step 4, carried as their own row; Deviation 1 its except
decision/no-verify-start-outside-test-home · apply — check 1: the new census binary, report Harness / gate surface
contract/fake-agent-cli/trust-config · apply — check 5: expected amendment (the fake agent's --trust-config)
contract/fake-agent-cli/trust-record-blank · apply — check 1: the test-only hold, report Deviations 2
decision/live-failing-reading-not-verification · apply — check 3: the wrap directive's ruling (inputs#I2 item 1), stated as the live tier's rule

ops-obs-plan.md
contract/verify-spawn-subject/verify-pty-probe · apply — check 1: routine count five → six
contract/diag-subject/verify-pty-probe · apply — check 1: routine count, Run A to Run F
contract/verify-process-exit/child-exit-status-and-duration · apply — check 1: routine count eight → nine
decision/claude-internals-not-a-contract · escalate → Q1 — check 1: widening R4, verify writes into the config (inputs#I4); answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/refusal-telemetry · apply — check 1: no widening; the writer's refusals ride the stamps, report Coverage (no log line)
decision/r8-strip-logged-as-names-only · escalate → Q1 — check 1: the CLAUDE_CONFIG_DIR value read, R2 with R4; answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/env-vars-not-configuration · escalate → Q1 — check 1: the same env value read, R2 with R4; answer card · resolved: apply (Q1) — delegate viola-overseer, 2026-11-17: «Confirmed by the overseer at 2026-11-17T21:33:27Z … each lies inside R1–R4, and none amends the product's write in.»
decision/trust-record-write-uninstrumented · apply — check 1: no new log event (report Outcome, obs criterion)

ops-a11y-plan.md
contract/verify-stamps/trust-record-value · reject — check 2: the fact's home is architecture contract/ledger-stamps/rows and contract/trust-writer-words/*

Not proposed and judged here (check 5): security-plan decision/verify-children-direct-spawn — its rule names no count, stays true; decision/new-env-read-needs-log-entry — the Decisions Log entry is the security-plan sidecar entry the landing of decision/r8-claude-env-strip writes; test-plan decision/fake-agent-argv-only and contract/fake-agent-cli/argv-only-options — `--trust-config` is argv, both stay true; contract/fake-agent-screens/trust-dialog — the record read rides contract/fake-agent-cli/trust-config («any failure reads untrusted»); decision/stamping-verify-passes-transcript and §3 boot step 4 (decision/harness-boot-creates-stamps-and-starts) — carried by decision/verify-under-test-owned-user-home, one fact one home.

ops-main-architecture.2.md
decision/trust-record-ledger-row · apply — check 7 condition: the delegate's confirm of Q1 (viola-overseer, 2026-11-17) set the wording «no row words it as a measured fact»; the row now asks per version whether the CLI honours a record, never states it

ops-main-architecture.3.md
decision/permission-request-question-stays-null · apply — check 1: routine count, the relied rows 22 → 23 (report Counts); the proposer's set read «landed before: 1 (tx 1193)» at its landing and the row still reads «22nd» (rec.py get dec-ucvfrmxd, rev 3); reworded with no count, «a further relied row»
