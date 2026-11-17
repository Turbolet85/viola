# Outcome — 2026-11-17-standing-live-rig · run 2026-11-17T06-49-10Z-implement

**Outcome: stuck** — the non-live block is green (entries 1–9) and all five remove-the-guard controls hold, but the live
round stopped at its round probe (entry 10), a plan entry that reads red on this host as written. Moving it is the
plan's revision, and the operator's rule for this run («A red that moves a plan entry stops you BEFORE the operator
pass») stops the run here: no live start was made, no operator entry was fired, no matrix ref was written.

## What stuck
- **Entry 10** `test -x "$HOME/.local/share/mise/installs/claude/2.1.325/bin/claude" && … --version` → `red · exit 0 ✗
  (exit 1)`; `round: STOPPED at 10 · legs fired 0/2` (`.andromeda/cache/chunks/2026-11-17-standing-live-rig/evidence/round-070702Z.txt`).
  Measured: the install holds `~/.local/share/mise/installs/claude/2.1.325/claude` with no `bin/` dir (2.1.302's install,
  also read, has the same layout). The rig itself resolves `<version dir>/claude` and puts `<version dir>` first on `PATH`.
- **Also for the revision, read now, not yet red:**
  - `mise ls claude` reads 2.1.326 as `latest` (installed 2026-11-17); the plan names 2.1.325, the stamped version.
    Per verification-harness.md 2026-11-13, a newer install moves the proof only on the operator's word; the rig and
    entries 11–12 stay on 2.1.325 as written.
  - Entries 11–12 run `cargo run -q -p viola-e2e --bin viola-live-rig`, which builds no root `viola`. R2 takes
    `viola` from the rig's own bin dir (`target/debug/viola`), so R2 drives whatever build that hard link points at
    (possibly a fake-agent feature build; verification-harness.md 2026-10-20). The rig prints its hash
    (`viola-build:`). A step that builds `viola` (`cargo build --bin viola`) before R2 would pin it.
  - The fake agent's trust rows follow plan step 9 (selector on the trust row, numbered options). The 2026-11-15 live
    round's evidence (`chunks/2026-11-15-live-round-on-the-trimmed-product/evidence/live-round.md`, entry 11) read
    2.1.325's options unnumbered, with the selector first on a «No, …» row above the trust row. Step 12 would align
    the fake after R1. The rig's key logic handles both forms (≤2 down, Enter only on `trust-yes`).
- Live starts: **0 of 2 + spare unspent**. `evidence/live-starts.ndjson` was never written.

## Gates (this run's trail `.andromeda/runs/2026-11-17T06-49-10Z-implement/gate-2026-11-17-standing-live-rig.json`)
- Firing 1: 8 green, entry 2 (clippy) red `let_and_return` at `crates/viola-e2e/src/rig/mod.rs:205`; fixed.
- Firing 2 (whole block): `entries 16 · green 9 · red 0 · recorded 0 · timeout 0 · not-run 7` (1 fmt · 2 clippy · 3 unit 23 rig cases · 4
  rehearsal 9 of 9 · 5 cli_fake_agent · 6 contract_fake_agent_drift · 7 `release-check: viola only` · 8 cargo deny · 9
  canary `last line 0`).
- Controls (`evidence/control-1.txt`): `control: 5 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`.
- Before the block, by hand: the rehearsal binary read 8 of 9 once (A3: the fake receipted the stop's Ctrl-C as a key
  inside its trust dialog); fixed in the fake, then 9 of 9.

## Files changed
- New: `crates/viola-e2e/src/bin/viola-live-rig.rs`; `crates/viola-e2e/src/rig/{mod,screen,isolate,procs,keys,ledger}.rs`;
  `crates/viola-e2e/tests/rig_rehearsal.rs`.
- Modified: `crates/viola-e2e/Cargo.toml` (the `[[bin]]`, `vt100.workspace`), `crates/viola-e2e/src/lib.rs`
  (`pub mod rig;`), `src/bin/viola-fake-agent/input.rs` (DEL erases; `prompt()`), `src/bin/viola-fake-agent.rs` (the
  trust selector and its keys, the box rows), `Cargo.lock` (scope record: `` `Cargo.lock` · mechanical · serves
  `crates/viola-e2e/Cargo.toml` · self ``).
- `gate.py scope`: `scope: clean — changed 13 · listed 12 · recorded 1 (companion 0 · mechanical 1 · in-intent 0 · widening 0)`.

## Deviations from the plan, with justifications
1. **The fake draws its box rows.** Under `--screens` the fake now draws a box rule and a prompt row (`❯ ` plus the
   prompt's printable characters) on each typed key, in `src/bin/viola-fake-agent.rs` (a listed file). Step 2 reads
   `box-text` from the box's prompt row, and A2/A10 compare it on the fake, but the fake's recorded ready screen has no
   prompt row and step 9 adds none. A fourth `box-text` word, `absent`, reads a frame with no prompt row; step 2's
   literal rule would have read that `holds-text`.
2. **R2's viola home keeps `CLAUDE_CONFIG_DIR`.** `vhome/config.json` holds `{"v":1,"claude_env_keep":["CLAUDE_CONFIG_DIR"]}`.
   `viola run` strips every `CLAUDE*` name from its child, which would drop the private config dir. This is the survey
   rig's own measured form.
3. **R2's argv** is the CLI's real form, `viola --home <vhome> run rig-r2 -- claude --model haiku`. The plan wrote
   `viola run --home … --name rig-r2 -- claude`. `--model haiku` is added to keep three turns cheap; it does not
   touch the trim reading.
4. **Census scope.** The run marker is `AGENT_RUN_RIG=<rig pid>-<start time>`. `census [--kill]` and the start's
   `leftover` refusal select only processes whose marker names a rig no longer alive, so parallel rehearsals and a
   running rig are never swept. Within a start, the teardown census is the run's own marker or a cwd under its scratch root.
5. **Watchdog.** A thread raises the same stop flag that SIGINT/SIGTERM raise, and every wait reads it.
6. **Exit-path instruments (A4/A8).** «A refusal after the copy» is a spawn of a missing program (`refused: spawn`);
   «the child's own exit 1» is `/usr/bin/false`; «a hook that outlives it» is the `--exit-no-eof` holder on the
   watchdog-deadline path, which reads `census: 1 killed`. Every other path reads `census: 0`.
7. **Controls** were fired under the tool's default bound, not `--timeout 60`, because the clone builds the workspace
   cold. Every firing still read under 60 s (in place 47.56 s; removed 4.4–12.5 s), so none was a `timeout` reading.
8. **Frames** go under `target/live-rig/<run>/` by default; rehearsals pass `--out` to keep theirs in the case's own
   `/tmp` root.

## Surfaced
- No spec↔reality gap.
- The plan defect above: entry 10's path. The 2.1.326 and R2 build notes go with it for the plan's revision.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `viola-live-rig` rehearsals (by hand: r1, r2; and the rehearsal binary's cases) | this run | terminated — each rig exited after its own teardown read `census: 0`; `gate.py census` reads no `viola-live-rig` |
| `viola-fake-agent` children, `viola run` (R2 rehearsal) | this run (through the rig) | terminated — the census lists none |
| clone build and controls | a leg of `gate.py control` | terminated — `clone removed` |
| the live round | a leg | entry 10 only, 0.0 s; no leg fired |
| 8 `claude` sessions in `gate.py census` (overseers and builders) | NOT DERIVED — the host's own sessions, not this run's | left running: not this run's |
- Scratch: my two hand rehearsals' empty scratch bases `$TMPDIR/vr1-4070306` and `$TMPDIR/vr2-4070078` were removed (`rmdir`, read empty first).

## Decisions & corrections
- The operator's rule for this run: «A red that moves a plan entry stops you BEFORE the operator pass» (`/andromeda-implement` args). Applied: the run stopped after the round's red at entry 10.
- The operator's cap: «2 (R1, R2) + 1 spare … each ledgered with the clock time before it is made». Applied: none spent.
- Guard and hook refusals: none.

## Next
- Re-plan through `/andromeda-phase`'s revision offer: entry 10's path (`…/2.1.325/claude`). Optionally a `viola`
  build step before R2, and the operator's word on 2.1.325 against 2.1.326. Then re-run `/andromeda-implement`. The
  tree is preserved and the gates are green to that point.
