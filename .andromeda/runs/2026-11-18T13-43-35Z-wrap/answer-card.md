# Answer card — Q1 · 2026-11-18-session-folder-trusted-by-viola

## The founder's own answers this card stands inside
- **R2 «Файл ребёнка, ключ как у CLI»** — the founder's own answer, given in the overseer's dialog and read at
  2026-11-17T18:59:41Z, relayed by the overseer (`~/dev/projects/additional/viola-overseer/split-trust-route-adaptation.md`,
  snapped by run 2026-11-17T19-03-40Z-wrap, cited by the plan's Expected amendments): viola writes ONLY
  `projects[<key>].hasTrustDialogAccepted` for one folder, `<key>` the key the CLI itself would write; the file the
  started child reads (`<CLAUDE_CONFIG_DIR>/.claude.json` when `claude_env_keep` keeps that name, else
  `~/.claude.json`), so viola reads that ONE environment value for that one purpose; only at `run` and `revive`, after
  the version gate and before the spawn, on a verified CLI, only when no record already trusts the folder; nothing for
  the home folder; nothing written when the file is absent, unreadable, a link, not the JSON expected, or oversize; one
  closed word per start in the run log, nothing of the file anywhere.
- **R3 «Блокировка, замена, проверка»** — same record: the CLI's own lock when free, an atomic replace, a read-back
  before the spawn, at most 3 attempts; the residual (a concurrent write of another session lost in the window)
  accepted by his word and named in the row.
- **F-T1 «Ставит флаг, остальное не трогает»** and the `S1` shape with its sentinel seed — the founder's own answers,
  read at 2026-11-18T11:39:23Z, relayed by the overseer (inputs#I2 items 1, 4): where an entry stands without a true
  flag viola sets ONLY `hasTrustDialogAccepted` and keeps every other key byte for byte; his price — a human's earlier
  «no» for the folder cannot be told from a closed dialog, and a start through viola overrides either — worded in the
  row it amends.

## The escalated operations main reads as lying wholly inside them (26)
- ops-architecture.md · decision/verify-own-trust-record · established-decisions · fit: fits type 1 — R2 (verify's own record only; the start's write is another row)
- ops-architecture.md · contract/instance-snapshot/cwd · standard-contracts · fit: fits type 3 — R2 (the revive's start keys its record by the recorded cwd)
- ops-architecture.md · decision/spawn-directory-confined-to-snapshot · established-decisions · fit: fits type 1 — R2 (the folder as the record's key in the child's config)
- ops-architecture.md · decision/env-strip-names-only · occupied-resources · fit: fits type 2 — R2 (the one env value at the start)
- ops-architecture.md · decision/env-strip-reads-names-only · occupied-resources · fit: fits type 2 — R2
- ops-architecture.md · decision/no-environment-configuration · cross-cutting-patterns · fit: fits type 2 — R2
- ops-architecture.md · decision/env-var-viola-prefix · conventions · fit: fits type 2 — R2
- ops-architecture.md · contract/config-json/claude-env-keep · occupied-resources · fit: fits type 3 — R2
- ops-security-plan.md · decision/start-trust-skips-trusted-folders · data-protection · fit: fits type 1 — R2 (home folder, an already-trusting record)
- ops-security-plan.md · decision/trust-record-writer-guarded · input-validation · fit: fits type 1 — R3, F-T1 (the one writer, the flag-only set)
- ops-security-plan.md · decision/revive-recorded-cwd-spawn-dir · input-validation · fit: fits type 1 — R2 («only at run and revive»)
- ops-security-plan.md · decision/revive-cwd-spawn-only · input-validation · fit: fits type 1 — R2
- ops-security-plan.md · decision/cwd-only-in-snapshot · data-protection · fit: fits type 1 — R2
- ops-security-plan.md · decision/spawn-cwd-snapshot-only · data-protection · fit: fits type 1 — R2
- ops-security-plan.md · decision/nothing-written-under-claude-home · threat-model-summary · fit: fits type 2 — R2, F-T1 (the start's flag set, which stays)
- ops-security-plan.md · decision/no-writes-under-claude-home · data-protection · fit: fits type 2 — R2, F-T1
- ops-security-plan.md · decision/viola-writes-nothing-in-claude-dir · data-protection · fit: fits type 2 — R2, F-T1
- ops-security-plan.md · decision/credential-strip-by-name-only · secret-management · fit: fits type 2 — R2
- ops-security-plan.md · decision/env-not-config-channel · secret-management · fit: fits type 2 — R2
- ops-security-plan.md · decision/no-stored-secrets · secret-management · fit: fits type 2 — R2
- ops-security-plan.md · decision/r8-claude-env-strip · secret-management · fit: fits type 2 — R2
- ops-main-security-plan.md · decision/start-writes-trust-record-flag-only · data-protection · fit: fits type 1 — R2, R3 (residual), F-T1 (its price worded)
- ops-main-security-plan.2.md · decision/live-rig-seed-from-bundle-logic · secret-management · fit: fits type 1 — the S1 shape with its sentinel seed (inputs#I2)
- ops-obs-plan.md · decision/r8-strip-logged-as-names-only · obs-scope-summary · fit: fits type 1 — R2
- ops-obs-plan.md · decision/env-vars-not-configuration · service-identity · fit: fits type 1 — R2
- ops-obs-plan.md · decision/claude-internals-not-a-contract · 1-obs-scope-summary · fit: fits type 2 — R2, F-T1

## Named beside the card, not asked (fit `not_master`)
- ops-architecture.md · decision/start-trust-walk — not_master type 4 (the code holds how the walk is built)
- ops-main-security-plan.md · decision/live-rig-seed-from-bundle-logic — not_master type 4 (its lengthening; the shorter set above replaces it)

## The answer
Asked as a dialog to the overseer (the operator away, inputs#I7). Chosen: «Confirm all 26 (Recommended)». Its note,
verbatim: «Overseer (founder-delegated), 2026-11-18T14:17:37Z: confirmed after reading the card and the text of the
operations in ops-main-security-plan.md, ops-security-plan.md, ops-architecture.md and ops-obs-plan.md against R2, R3
(2026-11-17T18:59:41Z) and F-T1 (2026-11-18T11:39:23Z); every one lies inside those answers. PROVISIONAL until the
founder's own word at the epoch boundary's batch.»
Authority: delegate overseer, 2026-11-18 — PROVISIONAL; the ratification stays the founder's own R2, R3 and F-T1.
