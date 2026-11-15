# Implement outcome — 2026-11-15-live-round-on-the-trimmed-product

**Outcome: green, stopped before entry 9 as directed.** Gate entries 1–8 read `green 8 · red 0`, fired twice (the
trail `gate-2026-11-15-live-round-on-the-trimmed-product.json`, firings 1 and 2). No live `claude` start was made, no key
reached a real CLI, nothing was written under `~/.viola/` or to `~/.claude/settings.json`, and the pinned product binary
was not touched. `live-sessions.ndjson` holds no `start` row (`live-read.py ledger` → `ledger: starts 0 · reds 0 · unread 0`).
All five lines of `founder-words.md` (four lines carrying the five forks) stand `unanswered`.

Setup's approval line, quoted: `2026-11-15T06-55-05-phase/matrix-2026-11-15-live-round-on-the-trimmed-product.json · open · 4666 B`
(`rec.py runs --kind '*-phase' --file '*-2026-11-15-live-round-on-the-trimmed-product.json'`: 1 run, the newest, holds
the matrix file).

## Files changed
Product tree: none (`git status` over `crates src fixtures plugin tests` empty; `gate.py scope`: `scope: clean — changed 0`).
In the work dir `.andromeda/cache/chunks/2026-11-15-live-round-on-the-trimmed-product/evidence/` (research's New files):
- `live-pty.py`, `live-ledger.py` — copied byte for byte (entry 1 `cmp` exit 0).
- `live-tap.py` — copied; step 1's one change (`CLI_VERSION` from the `cli:` line), then step 4's `rehearse` and `live`.
- `live-read.py` (new) — `words` · `start` · `statusline` · `box` · `census` · `ledger` · `place`.
- `live-hold.py` (new) — `rehearse-|live-` `verify` · `wrapped` · `unwrapped`.
- `founder-words.md` — the four lines, all `unanswered`.
- `rigs.md` — the step-1 diff and the pty-host deviation; `rehearsal.md` — the readings and the closing lines;
  `live-sessions.ndjson` — two rehearsal `reading` rows (the first a pre-gate probe's red, kept as written), no `start`.
Not yet made (they belong to the live entries): `round-state.json`, `hold-journal.md`, `live-round.md`.

## Gate entries 1–8 (firing 2's lines)
```
  1 probe       green · exit 0 · 0.36s · 340 B → 1.2.log
  2 probe       green · exit 0 · 0.03s · 29 B → 2.3.log
  3 build       green · exit 0 · 0.22s · 144 B → 3.2.log
  4 e2e         green · exit 0 · 4.8s · 253 B → 4.2.log
  5 e2e         green · exit 0 · 76.73s · 1297 B → 5.2.log
  6 e2e         green · exit 0 · 7.71s · 484 B → 6.2.log
  7 e2e         green · exit 0 · 0.04s · 176 B → 7.2.log
  8 build       green · exit 0 · 0.2s · 144 B → 8.2.log
not run — excluded · 12: 9-20
entries 20 · green 8 · red 0 · recorded 0 · timeout 0 · not-run 12
```
After entry 8, `target/debug/viola` is the plain build: sha256 prefix `a0e247fd3a77f744`, the
`target/debug/deps/viola-b542566ddb0401f6` inode. The rehearsals ran on the fake-agent build `f5606b4881aa2da2`.
The plan's must-pass and must-fail readings, and every closing line, are in `evidence/rehearsal.md`.

## Matrix
No `ref` written. v2-02 and v2-03 are this chunk's gated caps (`planned, gated`), and the live entries that prove them
(9–12) are unfired: a rehearsal never verifies a gated cap.

## Deviations, with justifications
1. **The pty host** (inputs#I3, the overseer's technical answer). `live-hold.py` hosts its sessions on a pty it opens
   itself, in `live-pty.py`'s form, instead of on `live-pty.py`, which writes nothing to the master and keeps no byte.
   It adds only the key writes, each behind its founder line (refused on `unanswered`/`no`; trust keys only while the
   trust literal stands, ≤ 2 down-arrows and 1 Enter, the Enter only on a read `selected`; a modal that isn't passed is
   SIGKILLed with no key), and the in-memory scan (counts of `unable: `/`hint: `, a stripped tail). `live-pty.py` stays
   byte-identical. Both arms of both grants are rehearsed under the fake agent.
2. **Rehearsal-only key order, and the live erase check** (inputs#I4, the overseer's technical answer). The fake agent
   keeps a typed DEL in its prompt (`src/bin/viola-fake-agent/input.rs` `plain()`). With the keys before the send, the
   rehearsal read `send-issued/prompt-submitted/wheel/send-refused` (`not-delivered`/`no-prompt-submitted`) and no hold.
   So `rehearse-wrapped` reads (d) after the hold. `live-wrapped` keeps the plan's order (d) → (e) and, after `x` and
   DEL, reads the drained bytes for the erased box before `release` and the send. If the `x` still stands it types
   nothing more, sends nothing, records `ended=erase-stands`, and goes to the killed-wrapper teardown (no key) and the
   census. The rehearsal takes that stop arm (`keys=echo=True,human=True,erase=stands · keys_stop_arm=stopped`), then
   reads the release alone (`release_probe=release=0,driver=True`) and the erase predicate over fixed texts.
   **«The real CLI erases on DEL» is NOT MEASURED.** No ledger row, recorded fixture or live reading in the tree
   measures it. The masters name DEL only as a refused paste character (architecture
   `decision/control-characters-refused`, security-plan `decision/paste-text-character-set`), and `src/run/wheel.rs:452`
   reads 0x7f only inside an escape string. Entry 10's erase check is the first reading of it.
3. **Two fake versions.** `rehearse-verify` stamps and records under `--cli-version 2.1.302`. The session rehearsals use
   2.1.287, because `viola run`'s version gate asks `--version` with no child argument and the fake answers its default.
4. **Rehearsal rows in the ledger.** Rehearsals append `reading` rows with `mode: rehearsal` and never a `start` row.
   `live-read.py ledger` and the cap count read only live rows.
5. **Four lines, five forks.** The directive says "five lines". The plan's step 2 writes four, with fork 5
   (`statusline-read`) as the third part of the `keys:` line, and entry 2 asserts `words: 4 forks`. The plan's form
   was followed.
6. **The compiled literals mirrored.** `live-read.py` holds `screen::SIGNATURES`' footer, modal and box literals
   (`crates/viola-agent-claude/src/screen.rs`) as constants, because a reader outside the product can't link them.
7. **The live echo reading has no terminal model.** Live, the `x` echo and the erase are read from the stripped stream's
   last prompt-glyph row (no `pyte` on the host, research). An unread redraw reads `stands`, so it stops rather than
   sends.
8. **A bytecode cache moved, not removed.** `py_compile` wrote `evidence/__pycache__/`. The host denies a recursive `rm`,
   so it was moved to the session scratchpad (`$TMPDIR/claude-1000/…/scratchpad/pycache-from-evidence`) and the rigs set
   `sys.dont_write_bytecode`. It is for the operator to delete.
9. **Entry 7 is fast (0.04 s).** The tap rehearsal reads `replies=2` and an equal shape, so its speed is the real
   `viola mcp` answering two frames, not a skipped step.

Scope record: no line (no out-of-list file was written; `scope: clean`).

## What the rehearsals do not reach
The trust `selected` → Enter path (the fake never draws a selected row); the erased-box branch (the fake can't erase);
the status-line hits (rehearsals read `statusline-read` as `no`, so `~/.claude/settings.json` is never opened); a hold
near 3600 s (rehearsed at 60 s: `hold_s=60 · deadline_hit=True · duration_ms=60019`).

## Carry-forward (for the wrap)
- **The fake agent's DEL fidelity**: `src/bin/viola-fake-agent/input.rs` `plain()` pushes 0x7f into the prompt instead
  of erasing. Left unchanged by the overseer's word (inputs#I4). Proposed owner: «Key after a resize»
  (`chk-hzxz2gl4`, the route entry that measures what real `claude` does with keys typed into it). The owner is the
  wrap's pin.

## The founder's words, and what `cli: 2.1.325` changes in the plan (added after the run's first report)
The founder's own words on forks 1, 2, 3 and 5 (overseer1's dialog at 2026-11-15T07:52:18Z), relayed as
`~/dev/projects/additional/viola-overseer/relay-live-round-founder-words.md`, read whole and snapped as **inputs#I5**.
They are written into `evidence/founder-words.md` as his:
`cap: 9 starts` · `keys: trust yes · input-box yes · statusline-read yes` · `cli: 2.1.325` · `hold: unanswered`
(`live-read.py words` → `words: 4 forks · 3 answered`). mise's `latest`, read only, resolves to 2.1.325
(`mise ls claude`; `installs/claude/latest` → `./2.1.325`).

**Fork 3 is not the plan's lean, so no live step was fired.** These parts of the plan and its gates change under
`cli: 2.1.325`, for the revision through the phase door:

- **Goal.** «the MCP `list` fixture re-recorded … a box-only screen fixture from a clean `viola verify --record`» now
  means read and kept as evidence, not landed.
- **Step 3, rig environment / Step 4 `live`.** The founder's binding: run on mise's `latest` as it reads at entry 9,
  record the version it resolves to, and STOP if `latest` has moved past 2.1.325. The rigs today put
  `installs/claude/<cli:>` first on `PATH` and read `claude --version` by path. They do not read `latest`, so the
  stop on a moved `latest` is not built. It needs a step line, and then a rig change (`live-hold.py` live
  subcommands, `live-tap.py live`) with its must-fail reading.
- **Step 8 (`live-read.py place`, entry 13).** It already places nothing off 2.1.302 («each file is recorded and
  carried»). Under 2.1.325 the step's placement branch never fires. The revision should say the files stay in
  `evidence/` (or `priv/`) and that a CARRY mints the new-set entry at the wrap's route-resolve.
- **Codebase touchpoints → Files to modify.** `fixtures/claude/2.1.302/Mcp.tools-list-1.json` and
  `fixtures/claude/2.1.302/Screen.ready.json` / `Screen.turn.json` («only on `cli: 2.1.302`») are no longer touched.
  The list then holds only `crates/viola-core/src/lib.rs` (step 9, on `hold: stands`).
- **Test Commands.**
  - Entry 13 (`place`) records `cli_places: false`: no file placed.
  - Entry 14's note («the placed fixtures under the hygiene walk»), entry 15's («the re-recorded exchange replayed»),
    entry 16's («over the placed fixtures and the comment edit») and entry 20's («the placed fixtures replayed in
    CI») all name a placement that does not happen. The entries run over the fixtures as committed (their
    baselines), so they prove nothing about the recordings.
  - Entry 9's assertion `stamped <ver>  … 0 fail` now stands on an unstamped CLI: 2.1.325 is stamped nowhere and its
    relied-row count is unmeasured. The «21 pass» in the v2-03 criterion is 2.1.302's and the fake's count, not a
    2.1.325 measurement.
- **Acceptance criteria.**
  - **CARRY 1:** «on `cli: 2.1.302`, entry 15 green over the re-recorded file, whose `list` description no longer
    holds «and the usage budget»» cannot be reached. The committed 2.1.302 file keeps the stale description;
    entry 12's `live: green` reads the 2.1.325 recording only. CARRY 1 stays open and goes to the new-set entry.
  - **CARRY 2:** the «placed, and entries 14 and 16 green» half cannot be reached. Only «recorded as read and carried»
    remains, whatever `box` reads.
  - **v2-03:** «the home entry 9 stamped green for the round's CLI (`stamped <ver>  21 pass  0 fail`, the relied set)».
    Its count is a 2.1.302/fake figure. On 2.1.325 the criterion should name the relied set, not a number. If 2.1.325's
    verify reads red, v2-03 and v2-02 go unread, as in 0.1.0.
  - **The last criterion** (entry 16 «after the placement», CI green): there is no placement, so the criterion
    reduces to the suite and CI on the unchanged fixtures.
- **Implementation notes.** «`cli: 2.1.302` is the only arm that places files» stands, but it is now the untaken arm.
  The expected amendment «test-plan §7 … on placement, the 2.1.302 set holds a box-only screen» falls away.
  New amendment candidate: a new-set route entry for the 2.1.325 recordings (the CARRY).
- **Research → Open questions.** Fork 3's question is answered: the files land in no fixture dir, and
  `tests/contract_ledger_probes.rs`'s `STAMPED` is untouched.

**Unaffected:** the rigs and the rehearsals of entries 1–8 (`live-tap.py` already reads `CLI_VERSION` from the
`cli:` line, now `2.1.325`); entries 10 and 11 (they need 9's green stamp for the same `cli:`); the matrix (v2-02,
v2-03 gated as before); `cap: 9` (verify 6 · wrapped 1 · unwrapped 1 · tap 1, the same on any CLI).

## Next
**First, the plan revision through the phase door** (`/andromeda-phase`, whose pending guard offers the revision on
the operator's word), carrying the changes above and the `latest`-check rig step. Only then the live round,
operator entries only, each on its own word and after the founder's lines reach
`founder-words.md` by relay: 9 (`live-verify`, 6 starts) → 10 (`live-wrapped`, 1 start, the ≈ 3600 s hold) →
11 (`live-unwrapped`, 1) → 12 (`live-tap.py live`, 1). Each is fired as
`gate.py run --chunk 2026-11-15-live-round-on-the-trimmed-product --operator {n}`, and each refuses (exit 3) while a line
it needs reads `unanswered`. Then 13–17 (the placement and the suites), then the operator pass 18–20, then
`/andromeda-wrap-session`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| rehearsal `viola run round` wrappers, `viola-fake-agent` children, `viola verify` and `viola-mcp-tap`/`viola mcp` (entries 4–7, both firings, and the pre-gate probes) | this run | terminated — `census_held=0` in each closing line; the `gate.py census` reading below lists none of them |
| the 15 processes `gate.py census` lists (`comm` viola/claude/…) | NOT DERIVED — not this run's: the overseer sessions, the prototype tree's `viola run` builders and `~/.viola/bin/0.1.0-…/viola mcp` | left running — not this run's; their owners stop them |

Census call: `gate.py census --comm viola --comm viola-fake-agent --comm viola-mcp-tap --comm claude --args
target/e2e-home/viola-live --args projects/viola/target/debug/viola` → `census: 15 running`. None runs from
`projects/viola/target/debug/viola` or under `target/e2e-home/viola-live`. `target/e2e-home/` lists as before the run,
and no `target/mcp-live-tap-*`, `target/live-round-cwd-*` or `.viola-verify-*` stands.

## Decisions & corrections
- The overseer's technical answer, the pty host: an own pty loop in `live-pty.py`'s form, key writes bound to founder
  lines, both arms rehearsed (relay-1.md, inputs#I3).
- The overseer's technical answer, the rehearsal key order: keys after the hold in the rehearsal only; live checks the
  erase and stops on a standing `x`; say whether the DEL erase is measured; the fake's DEL fidelity is a Carry-forward
  (relay-2.md, inputs#I4).
- A guard refusal: the bash guard refused a recursive `rm` of the work dir's `__pycache__` (a host deny rule). It was
  moved to the scratchpad instead (deviation 8).
- A guard refusal: the bash guard refused a `cd` into a skill references dir, and a pipeline tool's output piped into
  a filter. Both were re-issued bare.
- A standing learning met: `target/debug/viola` is a hard link cargo re-points per feature set (verification-harness.md
  2026-10-20). Entry 8 read its effect: the plain build `a0e247fd3a77f744` stands after it.
