# Founder forks — wrap of «Cross-session prefix ledger row» (2026-11-21-cross-session-prefix-ledger-row)

Written at the wrap's P2 halt, 2026-11-21, the operator away (inputs#I12): each item below lies OUTSIDE his recorded
answers, so it took its most conservative option and waits for his own word. Nothing here landed in a master.

## 1. The recorded arrival carries a runtime-dir value (inputs#I12 item 4)
**Measured by this wrap** (the committed file and the code, no start): `fixtures/claude/2.1.328/UserPromptSubmit.cross-session-1.json`'s
`prompt` holds, inside the CLI's own cross-session prefix, the SENDER's socket address — the recording host's runtime-dir
path (holding the uid) and the sender's pid — and a `from-name` holding a seven-digit number; no `/home/` path, no `~/`,
no user word. The scrub does NOT normalise it: `ledger::scrub` (`crates/viola-agent-claude/src/ledger.rs:1940`)
rewrites only the home and the user word; the hygiene walk's `has_absolute_path` (`ledger.rs:2064`) flags only a
drive-letter, `/home/`, `/Users/` or `\Users\` path. Both passed it.
**What he was told** (X1, inputs#I9): the runtime dir's value reaches the sender's argv and the model, «never a log,
stamp or fixture». That holds for what verify writes of its own; it does not hold for what the CLI puts in the arriving
prompt, which `--record` keeps.
**Taken (most conservative):** no master row states the residual as accepted. The proposed provisional row
(security-plan `decision/cross-session-fixture-keeps-runtime-path`) was read `not_master` by fit and not landed. Three
rows that stated the refusal wider than the code were corrected to the code's measured reach (security-plan
`decision/fixture-recording-scrubbed`, `decision/fixture-scrub-and-refusal`; test-plan `decision/fixture-hygiene-walk`:
«a path» / «absolute path» → home, drive-letter or user-folder path) — a correction of what the code already did, no
widening. The fixture stays committed as CI's 29th-row replay.
**His choice:** (a) accept the residual as it stands; (b) a chunk that makes the record scrub normalise the runtime dir
and the pids (and the hygiene walk flag them), then a re-record; (c) another.

## 2. Four exception cells too short for the new reading beside the named ones
Each row's `except` (bound 200 chars) names exceptions he ratified one by one; verify's one `XDG_RUNTIME_DIR` read (X1)
or its uds address check belongs beside them, and none fits without dropping or rewording a named member. The
proposers' sets dropped members' words and were rejected; each row stands as it was, short of the new reading:
- architecture `decision/no-environment-configuration` — 176 chars now; with «and verify's sender address» beside
  «XDG_RUNTIME_DIR for the socket dir»: 227.
- security-plan `decision/no-stored-secrets` — with «and verify's sender address»: 222.
- security-plan `decision/shared-validators-in-core` — with «and verify's uds address check»: 215.
- obs-plan `decision/claude-internals-not-a-contract` — with «verify composes the CLI's inbox socket name»: 239.
The reading itself stands in its own landed rows (security-plan `decision/verify-runtime-dir-held-in-memory`,
`decision/uds-address-closed-shape`; architecture `decision/cross-session-send-by-uds-address`). architecture
`decision/env-var-viola-prefix` names `XDG_RUNTIME_DIR` whole with no purpose: true as it stands, untouched.
**His choice:** (a) raise these rows' except bound; (b) let a named member's words shorten, shown old beside new;
(c) leave them as they stand, the new reading in its own rows.

## 3. The kept-stamp labels XS, XS-2
security-plan `decision/live-rig-kept-stamp-owner-only` names the keeping arms CS, WS, PQ, HF, HG. The overseer's
answer (inputs#I2 item 9) had the row grow by the two label names; fit read that lengthening `not_master` (an
inventory the rig's code holds). Only his own word lands it against fit. The row stands with five names.
**His choice:** (a) add XS, XS-2 (his word over fit); (b) the row names no labels, the arms by rule; (c) as it stands.
