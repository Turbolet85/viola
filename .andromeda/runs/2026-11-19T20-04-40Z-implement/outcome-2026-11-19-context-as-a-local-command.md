# Outcome — 2026-11-19-context-as-a-local-command

**Outcome:** green — the plan's non-operator entries green, the live round COMPLETE (CS and CC green, the spare
unspent), the three controls hold. Next → the OPERATOR PASS (entries `bash scripts/agent-run.sh pre-push`, the
`gate.py hygiene` entry, the clean-tree push, the `ci.py conclusion` read), then the wrap.

## Files changed
- crates/viola-agent-claude/src/ledger.rs — `/context` seventh in `LOCAL_COMMANDS` (`CommandStdout`),
  `PROBE_CONTEXT_COMMAND`, `LedgerRow::LocalCommandContext` (25th, last in `ALL`), its `check`,
  `TypedRun::context_latency_ms`, `merge_stamp`'s `measured.typed_probe.context_latency_ms`
- crates/viola-agent-claude/src/ledger/test_cases.rs · test_cases_checks.rs · test_cases_framing.rs ·
  test_cases_runs.rs · test_cases_stamps.rs — the list, row, check, stamp and order cases at 25
- src/cmd/verify/typed/turns.rs — Run B's eighth paste `/context` after the echo paste; src/cmd/verify/typed/runs.rs
  — the latency slot; src/cmd/verify.rs — `step_line` pins at /25
- src/bin/viola-fake-agent.rs — `--framing` echoes the exact `PROBE_CONTEXT_COMMAND` too
- src/run/send/classify.rs — three `/context` unit cases and the `echoed_in` helper (a transcript declared, not yet on disk)
- src/run/version_gate.rs — the stamps-verdict ladder at 25
- tests/cli_send.rs — the two `/context` integration cases
- tests/cli_verify.rs · tests/cli_verify/runs.rs · tests/cli_verify/stamps.rs · tests/contract_ledger_probes.rs —
  count pins at 25
- crates/viola-e2e/src/harness/run.rs — `LEDGER_ROWS` and the live-proof stdout at 25
- crates/viola-e2e/src/rig/mod.rs — labels `CS`/`CS-2`/`CC`/`CC-2`, `ContextShape`, the kept stamp, `config-top`, the
  stamp seed, `arm_cc`, the shared context reading, the hint-cell reading, `send_word`'s two `ok` shapes, the
  `local-command-context` stamp line, `FLOOR_ROWS`'s doc; crates/viola-e2e/src/rig/screen.rs — `Model::hint_cells`
- crates/viola-e2e/src/bin/viola-live-rig.rs — the `cs`/`cc` rehearsal words and the help text
- crates/viola-e2e/tests/rig_rehearsal.rs — the `cs`/`cc`/refusal rehearsals, the `cx` move, the 25-row pins

## Deviations, with justifications
- Plan step 9 places the fake's declared transcript under `<receipt dir>/transcripts/`; the fake keeps it beside its
  own exe (`src/bin/viola-fake-agent.rs` `transcript_of_session`), i.e. the shared `target/debug/`. The first-act case
  therefore runs a private copy of the fake (`fake::claude_bin` in a test-owned tempdir) and removes its own file, so no
  other `--transcript` test races the removal. No helper or fake option added.
- Plan step 19 reads «`refused: stamp`, no `start` line, `scratch: absent`»: the refusal is made before the ledger
  line and before any scratch (uncounted, exit 3, the summary the one `refused: stamp` line), so the rehearsal asserts
  the scratch base empty rather than a printed `scratch: absent` word.
- The `/context` reading's waits (`context_reading`) are bounded by `min(SUBMIT_BOUND, settle)`: a listed local
  command submits no prompt, so the 20 s wait ran the `cc` rehearsal past its 15 s deadline. Live settle is 60 s, so
  L2's and CC's live bound stays 20 s; L2's words are unchanged.
- `FLOOR_ROWS`'s doc comment now names `browser-tools-prompt` among the floor's exceptions: it was already outside the
  21 rows before this chunk (the new inline case read it), so the doc named three of the four exceptions.
- Control ids: the tool's id form is 1–16 chars, so the plan's `first-act-unconfirmable` fired as `first-act` and
  `kept-stamp-noclobber` as `kept-noclobber`, each guard line naming the plan's id.
- Scope record: `tests/cli_verify/stamps.rs` · companion · serves step 10 · self (a `7 pass  17 fail` summary pin).
  `gate.py scope`: clean — changed 22 · listed 21 · recorded 1 (companion 1).

## Surfaced / stuck
none.

## Gates (this run's last firing of each)
- 1 `cargo fmt --all --check` green · 2 clippy green · 3 `cargo nextest run -p viola-agent-claude` green ·
  4 `run --unit` green · 5 `run --integration` cli_send|cli_verify green · 6 `run --e2e` ledger probes|hygiene|drift green ·
  7 `run --integration` rig_rehearsal green · 8 `cargo nextest run -p viola-e2e --lib` green
- reds before green, each with an edit between (recorded): first firing — 3 red (three stamp tests pinning bare
  25/26/23 counts), 4 red (the same three and the floor-rows oracle), 7 red (`cc` past its 15 s deadline); 27 red once
  (`contract_verify_home` read a rehearsal's writer-field literal as a verify start).
- live round (`--live-legs --entry 16`, then `--entry 20`): round COMPLETE · legs 1/1 each; 9–15 green/recorded, CS
  green, 17–19 green, CC green; 21–23 not fired (spare unspent); 24, 25 (operator) green: census 0, live-stamps unmoved.
- 26 `run --integration` green · 27 `run --e2e` green.
- controls: context-listed holds · first-act holds · kept-noclobber holds (evidence/control-1.txt).
- C4 (`rig_rehearsal_reads_the_box_empty_after_x_and_del`): PASS in all 7 firings that ran it (7.log … 7.5.log, the
  control's 7.without-kept-noclobber.log, 26.log); no red, not acted on.

## Process hygiene
| process | started by | final state |
|---|---|---|
| CS's `viola verify` and its 9 `claude` children | a leg (entry 16) | terminated — the rig's `census: 0` |
| CC's `viola run`, its `claude` child, two `viola send` | a leg (entry 20) | terminated — the rig's `census: 0` |
| harness sessions, fake agents, test wrappers | the gate entries (4–8, 26, 27) | terminated — `gate.py census` reads none under `projects/viola/target` |
| the control clone's builds and tests | `gate.py control` | terminated — clone removed |
`gate.py census --comm claude --comm viola --comm viola-live-rig --comm viola-fake-agent --comm bwrap --args
projects/viola/target`: 16 running, none this run's (the overseer sessions, their wrapped builders, the product and
prototype `viola` bridges).

## Decisions & corrections
- The operator's word (inputs#I4): go on with the live round, hostwatch before each start, fire on no neighbour named.
  The first read (20:30:49Z) was CONTENDED naming escher: no start; re-read every 2 min until 20:45:17Z's QUIET read
  naming no build tool; CS fired then, CC after a QUIET read at 20:46:23Z.
- The relay says the overseer read the credentials token window by hand before the round; the rig made its own check
  too (no refusal). verification-harness.md 2026-11-18 names the rig's check as the token window's only read.
- Sweep hazard: the retired-shape probe (`/24\]`, `; 24\]`, `24 pass  0 fail`) misses a bare count pinned on a
  derived set (`rows.len(), 25`, `results.len(), 23`): three stamp tests went red at the first firing.
- Sweep hazard: `tests/contract_verify_home.rs` reads any `"verify"` literal in a test source as a verify start; an
  assertion on a stamp's `writer` field tripped it.
- The fake agent writes `--transcript` files beside its own exe (`target/debug/transcripts/` for the shared build), a
  path every concurrently running test shares.
- Guard refusals: a `cat` heredoc to a scratch file (re-issued through the Edit tool); a `gate.py` dry-run piped into
  `grep` (re-run bare); a `cd` into the skill's references dir (absolute paths used).
- The `code` evolve playbook was read at Setup, batched with the system file, before its checkpoint.
