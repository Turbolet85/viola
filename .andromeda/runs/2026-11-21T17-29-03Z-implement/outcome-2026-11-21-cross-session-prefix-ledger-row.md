# Outcome — 2026-11-21-cross-session-prefix-ledger-row · implement run 2026-11-21T17-29-03Z-implement

**Outcome: green** — Revision 1 (plan rev 7), re-entered at step 8. `XS-2` read live 29/29 on claude 2.1.328 by path
(`cross-session-prefix: pass`, `cross-session: harness`), base line 2 of `--cap 2 --spare 1`, the spare unspent. The
record landed over `fixtures/claude/2.1.328/` (32 → 33). The whole unit, integration and E2E layers are green over the
landed set; the smoke is green. Next → the OPERATOR PASS in this window (entries pre-push, hygiene, push, CI read).

## Gates (this run; entries 1–8 not re-fired, per the revised plan — green in run 2026-11-21T13-20-00Z-implement)
- 9 `cargo fmt --all --check` green · 10 clippy green · 11 agent crate green · 12 rig lib cases green · 13 root bin
  green · 14 the row's `cli_verify` cases green (5 run: the pass case, the no-runtime-dir case, the keeps-its-red
  rstest ×2, the kill case) · 15 the canary binary green · 16–19 the filtered control entries green (19: the eight
  `uds_address` cases).
- Controls (`evidence/controls-rev1.txt`): `prefix-match` holds · `no-payload` holds · `unread-shape` holds (also
  failed case_6, case_7) · `address-shape` holds (also failed case_6_a_control_byte).
- Round dry-run: `round: DRY-RUN · legs 1`, no pre-flight refusal.
- Host read: `hostwatch.py read --from 2026-11-21T17:38:01Z --to 2026-11-21T17:38:18Z` → `QUIET - 0 s stalled`.
- Round (one call, date -u before it 2026-11-21T17:38:27Z): `round: COMPLETE · legs fired 1/1 · entries 48 · green 14 ·
  red 0 · recorded 1` — `evidence/round-173827Z.txt`; XS-2's summary whole in `evidence/live-round.md`.
- After the span: 35 `--operator` `census: 0` (history: unmoved) · 36 `29 ok` · 37 recorded: outcome harness · prefix
  cross-session · latency_ms 4413 · sender_tools ['search', 'send'] · send_result delivered.
- Landing (`evidence/landing.md`): plain copy; 38 `diff -r` green, no output.
- 39 `run --unit` green · 40 `run --integration` red (1 of 600: `fake_agent_receipt_env_holds_names_never_values`) →
  fixed → green 600/600 · 41 `run --e2e` red (1 of 336: `fake_agent_dialog_replay_matches_every_recorded_dialog_set`,
  2.1.328 dialog drift) → fixed → green 336/336 (`contract_ledger_probes_pass_over_every_stamped_set` PASS).
- Operator pass so far: 45 `pre-push` red (2 of 3951: the VH and AV rig rehearsals, `cross-session: not-run` under `env -i`) → `with_option` fixed, the two reproduced green under the same `env -i` form → 45 green (3951 passed, gate `breaches: []`).
- Smoke: 42 boot green (both instances `cli_verified: true` at 2.1.328) · 43 status `ready` · 44 cleanup
  `processes_gone`, `endpoint_gone`.

## Files changed
The revised steps' files under research's lists, plus 8 scope-record lines (all companion, `gate.py scope` clean):
- `crates/viola-agent-claude/src/ledger/test_cases_framing.rs` · companion · serves step 15 · self (the stopped run's)
- `tests/support/home.rs` · companion · serves step 12 · self
- `crates/viola-e2e/src/harness/boot.rs` · companion · serves step 12 · self
- `src/bin/viola-fake-agent/test_cases_opts.rs` · companion · serves fixtures/claude/2.1.328/ · self
- `tests/support/start.rs` · companion · serves fixtures/claude/2.1.328/ · self
- `tests/chaos_revive.rs` · companion · serves fixtures/claude/2.1.328/ · self
- `tests/cli_send.rs` · companion · serves fixtures/claude/2.1.328/ · self
- `tests/cli_fake_agent.rs` · companion · serves src/bin/viola-fake-agent.rs · self

## Deviations, with justification
1. Every test and harness `viola verify` gets a short owner-only test-owned runtime dir (the support's verify helpers,
   `start_as_user`, the harness boot stamp, and the rig rehearsals when the host sets none). The plan gave one only
   to the pass case, but step 12's one `XDG_RUNTIME_DIR` read makes every 29-row stamp depend on one, and `pre-push`
   runs under `env -i` (verification-harness.md 2026-11-19), where it is unset.
2. The fake agent's start receipt also records `pid` and the print `prompt` (step 14 names the prompt "the receipt
   line records" and Run B's pid "its receipt line records"); `tests/cli_fake_agent.rs`'s key pin moved to nine keys
   (the stopped run's `settings`, `print` had left it red since its phase B, unseen by the filtered entries).
3. `contract_fake_agent_drift`'s dialog-tier reader leaves out a hook whose payload names the send tool: after the
   landing the fake sender runs its two synthetic hooks during Run B, ahead of the dialog runs.
5. `rig_rehearsal.rs`'s `with_option` now carries the rebuilt command's env over: it dropped the case's runtime dir, so under `pre-push`'s `env -i` the VH and AV rehearsals read `cross-session: not-run` (the first `pre-push` red).
4. The landed set moved its recorded session ids; four literals moved with it (testing.md 2026-11-21):
   be8c67ee… → 4e678ce3… (default), 95caed5c… → 9fb0be0b… (clear).

## Process hygiene
| process | started by | final state |
|---|---|---|
| entries 9–19 and 38–41 (cargo, nextest, the harness's test children) | this run (gate.py) | terminated — each entry exited |
| the four control clones' firings | this run (gate.py control) | terminated — each `clone removed` |
| the round's entries 20–34 (XS-2's rig, its verify, its claude children, the sender) | a leg | terminated — rig `census: 0`, entry 35 `census: 0` |
| the smoke session `cross-smoke` (supervisor, two wrappers, two fake agents) | the operator entry 42, fired by this run | terminated — entry 44 `processes_gone: true` |
| the 6 processes `gate.py census` lists (the lab prototype's wrappers and wait, the pinned product's `viola mcp` and wait) | NOT DERIVED — none of this run's | left running: not this run's — their owners |

## Decisions & corrections
- The overseer's word (inputs#I11): host read from 2026-11-21T17:38:01Z, the round as ONE call; read QUIET, fired.
- Guard refusals, each re-issued once: a pipe of `gate.py run --dry-run` into `cut` (run bare); two `cd` into a
  directory outside the root (absolute paths instead).
- Standing learning met: a whole re-record moves the recorded session ids (testing.md 2026-11-21) — the old set's ids
  read before the copy, searched right after, four literals moved before the after-landing suites ran.
- Carry cry-5swgr2h3: not met — `rig_rehearsal_reads_the_box_empty_after_x_and_del` PASS in both integration firings.
