# Answer card — Q1 — chunk 2026-11-17-trust-record-ledger-row (wrap 2026-11-17T21-00-47Z-wrap)

## His answers (the founder's own, his option labels verbatim, read 2026-11-17T18:59:41Z; relayed by the Viola overseer — inputs#I4)
- **R1 «Сначала строка реестра»** — a new entry ahead of «Session folder trusted by viola» holds the 23rd relied
  ledger row, its `viola verify` probe, the fake agent reading the record and one live verify S0.
- **R2 «Файл ребёнка, ключ как у CLI»** — the write's bound: `projects[<key>].hasTrustDialogAccepted` alone, the
  CLI's key, the file the child reads (`<CLAUDE_CONFIG_DIR>/.claude.json` when the home keeps that variable, else
  `~/.claude.json`) — viola reads that ONE environment value, for that one purpose; absent / unreadable / a link /
  not the JSON expected / oversize → nothing written.
- **R3 «Блокировка, замена, проверка»** — the CLI's own lock when free, atomic replace, read-back, at most 3 attempts.
- **R4 «Шестой запуск, запись убирается»** — verify gains a SIXTH interactive run; it writes the trust record for its
  own temp dir into the config its child reads and removes that one record of its own after the run.
The wrap directive (inputs#I2 item 2) names R4's two widenings ruled and asks nothing new; the PRODUCT's write (`run` /
`revive`) is NOT amended in by this wrap.

## The operations lying wholly inside them (25 lines; each fit-judged)
- ops-architecture.md · decision/verify-child-set · established-decisions · fit: fits type 1 — five → six interactive runs (R4)
- ops-architecture.md · decision/verify-removes-only-own-dirs · established-decisions · fit: fits type 1 — its own record removed on pass, fail, stop (R4)
- ops-architecture.md · decision/claude-shapes-in-one-crate · established-decisions · fit: fits type 2 — test-only: the fake agent and the rig read a config's trust record (R1)
- ops-architecture.md · decision/claude-first-no-adapter-trait · established-decisions · fit: fits type 2 — the same exception (R1)
- ops-architecture.md · decision/claude-shapes-isolated · established-decisions · fit: fits type 2 — the same exception (R1)
- ops-architecture.md · decision/env-strip-names-only · established-decisions · fit: fits type 2 — verify reads CLAUDE_CONFIG_DIR's value when kept (R2 with R4)
- ops-architecture.md · decision/env-strip-reads-names-only · fit: fits type 2 — the same read (R2 with R4)
- ops-architecture.md · decision/no-environment-configuration · fit: fits type 2 — the same read (R2 with R4)
- ops-architecture.md · contract/config-json/claude-env-keep · fit: fits type 3 — keeping the name makes verify read its value (R2 with R4)
- ops-architecture.md · decision/verify-own-trust-record · established-decisions · fit: fits type 1 — verify writes and removes its own temp-dir record; run and revive write none (R4)
- ops-architecture.md · decision/trust-record-ledger-row · established-decisions · fit: fits type 4 — the 23rd relied row and Run F's check (R1, R4)
- ops-main-architecture.md · decision/env-var-viola-prefix · conventions · fit: fits type 2 — the exception list names verify's kept CLAUDE_CONFIG_DIR (R2 with R4)
- ops-security-plan.md · decision/no-writes-under-claude-home · fit: fits type 2 — verify's own record the bounded exception, not a rule for run or revive (R4)
- ops-security-plan.md · decision/viola-writes-nothing-in-claude-dir · fit: fits type 2 — the same bounded exception (R4)
- ops-security-plan.md · decision/nothing-written-under-claude-home · fit: fits type 2 — the same bounded exception (R4)
- ops-security-plan.md · decision/r8-claude-env-strip · fit: fits type 2 — the one CLAUDE_CONFIG_DIR value read (R2 with R4)
- ops-security-plan.md · decision/credential-strip-by-name-only · fit: fits type 2 — the same read (R2 with R4)
- ops-security-plan.md · decision/env-not-config-channel · fit: fits type 2 — the same read (R2 with R4)
- ops-security-plan.md · decision/no-stored-secrets · fit: fits type 2 — the same read in the env-read list (R2 with R4)
- ops-security-plan.md · decision/trust-record-writer-guarded · data-protection · fit: fits type 1 — the writer's bound: no create, the refusals, lock when free, three tries, mode and other keys kept (R2, R3)
- ops-main-security-plan.md · decision/verify-probe-dirs-cleanup · fit: fits type 2 — a SIGKILL leaves verify's own record too (R4)
- ops-main-security-plan.md · decision/probe-artifacts-cleanup · fit: fits type 2 — the same (R4)
- ops-obs-plan.md · decision/claude-internals-not-a-contract · fit: fits type 2 — verify writes and removes its own record in the config (R4)
- ops-obs-plan.md · decision/r8-strip-logged-as-names-only · fit: fits type 1 — the one value read, never logged (R2 with R4)
- ops-obs-plan.md · decision/env-vars-not-configuration · fit: fits type 1 — the one value read configures nothing of viola (R2 with R4)

## Named beside the card, not asked
- ops-architecture.md · decision/trust-writer-takes-config-lock — fit: not_master type 4 (the code holds how the
  writer takes the lock); rejected before the question.

## The answer

Answered 2026-11-17 by the Viola overseer through the wrap's dialog (option «Confirm (Recommended)»), its notes verbatim:
«Confirmed by the overseer at 2026-11-17T21:33:27Z, the card's 25 lines read whole against his four answers of 18:59:41Z: each lies inside R1–R4, and none amends the product's write in. One condition on the wording, the directive's item 1: `decision/trust-record-ledger-row` and `decision/verify-own-trust-record` state the row and its probe as BUILT — that the real CLI honours such a record is what the probe measures per version, and on 2.1.325 the one live reading (S0) read `fail`; no row words it as a measured fact. If a landed text does, set it before the land.»
The confirm is a delegate's: PROVISIONAL, superseded by the founder's own word at the epoch boundary; the ratification stays his answers R1–R4 (inputs#I4).
