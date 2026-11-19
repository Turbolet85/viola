# Answer card — Q1 — 2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin

The operator is AWAY (`inputs#I20`); the card goes to the overseer as a dialog (`inputs#I19`): every row below lies
inside the founder's own recorded answers, so the overseer confirms. A delegate's confirm is PROVISIONAL until the
founder's own word at the epoch boundary; each operation listed reads `fits`, so it lands.

## The founder's own answers (cited by their words)
- «Разрешить ветку VH (Recommended)» — read 2026-11-18T22:44:06Z (`inputs#I4`; `inputs#I3` item 1): the test-only
  live rig gains the `VH` arm for a start under his REAL home and config (no private `CLAUDE_CONFIG_DIR`, no
  credentials copy); the security row «the rig runs under a private config only» takes a NAMED EXCEPTION on this
  word; every start stays under his cap.
- «Да, оба чтения» — 2026-11-18T22:18:16Z (`inputs#I19` item 4): the two reads of his real Claude config — the key
  NAME before `L1`, key names and the needle around it, nothing wider.
- «Под конфигом стенда» (`L2`), «2 + 1 запасной», «Перевести на 2.1.328» (2026-11-19T06:01:53Z), «Строка в stdout»,
  «Остаётся за человеком» (`inputs#I19` item 4) — cited for the record; no operation below rests on them alone.

## The operations it covers (5 lines)
- ops-security-plan.md · decision/live-rig-isolated-child · secret-management · fit: fits type 2 — the VH arm, the named exception
- ops-main-security-plan.md · decision/live-rig-credentials-copy · secret-management · fit: fits type 2 — the VH arm makes no copy, reads no credentials
- ops-architecture.md · decision/claude-first-no-adapter-trait · established-decisions · fit: fits type 2 — the test-only except names the real config's key names under VH
- ops-architecture.md · decision/claude-shapes-in-one-crate · crate-dependency-direction · fit: fits type 2 — the same except
- ops-architecture.md · decision/claude-shapes-isolated · inherited-defaults · fit: fits type 2 — the same except

Rejected before the card, not asked: none fit read `not_master` among the escalated.

## The answer
Chosen option «Name rows outside», its note verbatim:
«Overseer (founder-delegated), 2026-11-19T07:55:31Z: the two security-plan rows (live-rig-isolated-child, live-rig-credentials-copy) are CONFIRMED as shown, PROVISIONAL until the founder's own word at the epoch boundary. The three architecture excepts are inside his answers in SUBSTANCE but worded WIDER than them, so not as shown: the proposed text turns the standing, named readings of the rig's PRIVATE config (its UserPromptSubmit key names, the source word, the one key name cachedChromeExtensionInstalled — his «Разрешить» of 2026-11-18T15:59:31Z named that one key, and the last wrap rejected an «any key name» form for this reason) into «key names» at large. Reword all three the same way and land them: KEEP the standing except's named readings for the private config word for word, and ADD only: under its VH arm, key names and the one needle word of the real Claude config around a live verify, never a value (his «Да, оба чтения», 22:18:16Z, and «Разрешить ветку VH», 22:44:06Z). With that wording the three are confirmed too, PROVISIONAL; if the fit reads the reworded cell otherwise, show me the text.»

## The bound (measured after the answer)
The standing `except` is 200 chars, the `decision` sheet's `except` bound (`master-rows.md` §The sheets: 200). The
directed text — the standing words kept and the VH clause added — reads 311 chars:
«Test-only: the rig seeds its private config and reads, no value, its UserPromptSubmit key names, source word and the cachedChromeExtensionInstalled key name; under its VH arm, key names and one needle word of the real Claude config around a live verify, never a value; the fake agent and rig read trust records.»
`rows-check` refuses a cell past its bound, so the direction cannot land as worded: shown to the overseer (Q2).

## Q2 — the answer
Chosen option «Shorten, keep every name», its note verbatim:
«Overseer's technical answer, 2026-11-19T07:56:19Z: shorten, keep every name — a rule whose except list is silent on a reading the tree now makes is a drift, and the wrap leaves none. Within 200 chars the cell must still NAME: the private config's UserPromptSubmit key names, the source word, the cachedChromeExtensionInstalled key name; under VH the real config's key names and one needle word; never a value; and that the fake agent and rig read trust records. Drop only connective words. If no wording within the bound keeps all of those, do NOT cut a name: stop and show me, and the three stay as they stand with the item written to founder-forks.md. I see the text before it lands.»

## Q3 — the text shown, and the answer
Shown (196 chars): «Test-only, no value: rig private-config UserPromptSubmit key names, source word, cachedChromeExtensionInstalled key name; VH real-config key names, one needle word; fake agent, rig: trust records.»
Chosen option «Land this text (Recommended)», its note verbatim:
«Overseer (founder-delegated), 2026-11-19T07:57:32Z: land it, PROVISIONAL — with ONE change: the standing except also says the rig SEEDS its private config, and your text dropped that word, which would leave the rule silent on a thing the rig does. Use this text, 200 characters by my count, if rows-check takes it: «Test-only, no value: rig seeds private config, reads its UserPromptSubmit key names, source word, cachedChromeExtensionInstalled name; VH real-config key names, needle; fake agent, rig: trust records.» If the tool counts it past the bound, land your 196-character text as shown and name the dropped «seeds» in the report as covered by security-plan decision/live-rig-seed-from-bundle-logic.»
Measured: the overseer's text is 200 chars (python `len`), within the bound. The three architecture rows take it, through
the proposer's re-spawn (`RULED`, `retry-ops-architecture.md`) and a second fit wave.

## Outcome
- security-plan decision/live-rig-isolated-child, decision/live-rig-credentials-copy: confirmed as shown, PROVISIONAL (delegate).
- architecture decision/claude-first-no-adapter-trait, claude-shapes-in-one-crate, claude-shapes-isolated: confirmed with the overseer's 200-char text, PROVISIONAL (delegate).
