# P5 review card — `2026-11-21-cross-session-prefix-ledger-row`, Revision 1

Run `.andromeda/runs/2026-11-21T14-00-53Z-phase`. A plan REVISION after the stopped implement run
`2026-11-21T13-20-00Z-implement` (red `XS`). Inputs: `inputs#I7` (the invocation) and `inputs#I8` (the operator's
answer, the overseer's items 3–10). No start was made, nothing of the real `~/.claude*` or `~/.viola` was read, and no
compiled screen literal is printed here.

**The plan:** `doc:2026-11-21-cross-session-prefix-ledger-row/plan`, rev 7. Its render is at
`.andromeda/cache/docs/2026-11-21-cross-session-prefix-ledger-row/plan.md`, to open in an editor. No plan stands in a
chunk's folder. Writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-21-cross-session-prefix-ledger-row`
is a diff of the chunk's records.

## Goal
The 29th relied row `cross-session-prefix` (v2-25), stamped live on 2.1.328. `XS` read `search send` · `refused` ·
`no-prompt`, and the bundle shows why: a bare session id never resolves a local session on 2.1.328 (C4 by
construction). The mend changes the ADDRESS only. Verify composes `uds:<runtime dir>/cc-socks/<Run B's child
pid>.sock` from its own live child, with no directory read, no listing and no name, and hands it to the sender as
`to`. The bare id leaves the prompt. The mended start is `XS-2` under `--cap 2 --spare 1`, in `XS`'s shape.

## Fork X1 — answer it with the word (`.andromeda/runs/2026-11-21T14-00-53Z-phase/founder-forks.md`)
Verify reads no `XDG_RUNTIME_DIR` today: Run B INHERITS it, because the R8 strip removes only `CLAUDE*` names. So
«the env name verify already hands its children» (`inputs#I8` item 3) is an inheritance, not a read. Composing from
the runtime dir's name adds one environment read to a product build. That reaches four rows §4 never named:
architecture `decision/env-var-viola-prefix` and `decision/no-environment-configuration`, and security-plan
`decision/no-stored-secrets` and `decision/new-env-read-needs-log-entry` (the last needs a Decisions Log entry).
- **(a) Recommended:** verify reads `XDG_RUNTIME_DIR` once, in memory only. The value reaches only the sender's argv,
  and through it the sender's prompt to the model; it configures nothing. With it absent or refused, verify spawns no
  sender and the row reads `fail`. This is the CLI's own source for the name (2.1.328 names its socket
  `(XDG_RUNTIME_DIR, else the temp dir)/cc-socks/<pid>.sock`, past 103 bytes `/tmp/cc-socks-<uid>/`). The four rows
  each add this reading beside the channel's, and the Decisions Log entry is his.
- **(b):** compose `/run/user/<euid>/cc-socks/<pid>.sock`, with no env read and no row moved. Where `XDG_RUNTIME_DIR`
  is elsewhere the start reads red, never a false pass. This re-plans steps 12 and 14 before `XS-2`.
The plan is written to (a) and HELD behind this fork at step 12.

## What changed (steps and entries), and what implement does
- **Done, with evidence:**
  - steps 1–17 are written;
  - phase A (entries 1–8) and phase B (9–15) read green;
  - three controls read «holds»;
  - `XS` fired red, as base line 1.
  The tree stands uncommitted.
- **Steps changed:**
  - **8**: `uds_address` with its four compiled literals, and `sender_prompt` over an address;
  - **9**: `not-run` also covers «no address»;
  - **11**: the gate is `child_pid` plus the composed address, no longer `resume_target`; no read of the roster, the
    peer key or `cc-socks/`;
  - **12**: the one `XDG_RUNTIME_DIR` read (X1);
  - **13**: the fake sender resolves by the uds address, a listening sibling by pid; the fake reads no env;
  - **14**: the pass case asserts the composed address, and the new `cross_session_prefix_reads_not_run_without_a_runtime_dir`;
  - **15**: `uds_address_composes_the_cli_socket_name` (eight literal cases, the 103-byte bound among them) and the
    sender-prompt case over an address;
  - **18**: the controls re-fired, plus `address-shape`;
  - **19–23**: `XS-2`; **24**: `XS-2`'s record.
- **Entries changed:**
  - **19** is new, the `address-shape` control entry;
  - **20–34**, the round re-keyed to `XS-2`. Its ABSENT check is on `XS-2/record` and `cross-stamp-2.json`. The
    kept stamps' sha (`HF`, `HG`, and `XS`'s `cross-stamp.json` PRESENT and unchanged), the live-home sha and the
    stale-socket count are RE-TAKEN into files keyed `-xs2`. The after-reads are `kept-stamps: unchanged`,
    `live-stamp: unmoved`, the stale-socket reading and the two name diffs;
  - **35**, the operator census, now after the span;
  - **36–38** read `XS-2`'s products: `29 ok` on `cross-stamp-2.json`, the block, and `diff -r` on `XS-2/record`.
  The stale notes «controls fire on this entry» on 11 and 13 are corrected, and so is «the first seven entries».
- **Implement re-enters at step 8.** It fires entries 9–19 and the four controls, then the host read, then the round
  as ONE call. Entries 1–8 are not re-fired. On a red `XS-2` it fires 30–32 by hand, verbatim (the tool fires a round
  step after the last leg only in the whole round), then 35–37, and stops. A third base start is the operator's.
- **The round fires as one command:** `gate.py run --chunk … --run-dir … --live-legs --dry-run` printed `round:
  DRY-RUN · legs 1` with NO pre-flight refusal. The span 20–34 holds round and live entries only.
- **The stopped run's four deviations, folded as plan text:**
  1. the fake sender needs `<pid>.listening` (step 13);
  2. the `no-payload` control's edit is inside `capture_names_only` (control table);
  3. phase A is EIGHT entries (step 5);
  4. the round is re-ordered (step 20).
  Its notes 5–7 are written in at steps 9, 15 and 16.
- **Implementation notes:**
  - **the cause table**, extended for the uds form: `send` · `none` is the gate (C2) or the path refused at input
    (AMBIGUOUS); `send` · `refused` is a thrown send (path form, socket gone, 5 s timeout, paced; AMBIGUOUS; C2 and C4
    excluded); `send` · `delivered` is the receive side (peer key, startup hold, kill switch; AMBIGUOUS);
  - **the peer key:** verify reads nothing of it. On Linux the 2.1.328 sender sends a frame with no token when the
    key is missing, and whether Run B drops it is unmeasured: words for the residual row;
  - **W3:** closed by address, save a pid reused between compose and send. Verify composes only while Run B is its
    live child, and ends it only after the sender. A Run B that exits on its own mid-send is reaped by its pump, so
    the open window is another claude session taking that pid AND binding its socket in the same runtime dir within
    ≤ 120 s;
  - **expected amendments:** every reached row named again (four more under X1 (a)). The address reaching the model
    provider in the sender's prompt is in the residual row's words.

## Top acceptance criteria
- `XS-2`: `cross-session-prefix: pass`, `cross-session: harness`, `pass-count: 29/29`, and `cross-stamp-2.json` at
  `29 ok`. After the landing, `contract_ledger_probes` reads 2.1.328 at `29 pass 0 fail`.
- The address: the `uds_address` cases, the prompt holding the address and no session id, the no-runtime-dir case,
  and the `address-shape` control «holds».
- The keeps-its-red case; the names-only, reading and rows criteria as before; every relied count at 29.
- After-reads: `kept-stamps: unchanged`, `live-stamp: unmoved`, the name diff at 2; the stale-socket counts a READING.

## Touchpoints and test commands
30 modify paths, 1 new (`fixtures/claude/2.1.328/UserPromptSubmit.cross-session-1.json`), as at the take-up. 48
`[[gate]]` entries: phase A 1–8 · phase B 9–15 · control entries 16–19 · round 20–34 (one live leg, 29) · operator
census 35 · leg-less reads 36–38 · after the landing 39–41 · operator pass 42–48.

Research: lists synced to the revision's delta (+0 −0), rev 3; not re-derived since the take-up's rev 2.

## Checks, every one re-run on rev 7
- **The fence:** `gate.py run --dry-run` parses 48 entries.
- **Baselines:** `gate.py baseline` reads 16 seen green, 0 not seen, 9 new, 23 leg. The baselines of the changed
  non-leg entries were run and written: 19 red (exit 4, 0 run, 785 skipped), 36 red (`cross-stamp-2.json` absent),
  37 recorded (exit 1), 38 red (exit 2, the record dir absent). Entries 12, 14, 16, 17 and 18 carry their runs
  unchanged and keep the take-up's baselines.
- **planlint:** 0 hits. Its two «unchanged» claims (lines 133, 327) read true.
- **Research's lists:** `scope lists: parsed 31 items`.
- **The sync:** `scope sync: PASS · delta +0 −0`.
- **The input citations:** `inputs: 8 entries`, unparsed 0; I4–I6 uncited, I5 and I7 n/a (messages).
- **The round:** `--live-legs --dry-run` reads `round: DRY-RUN · legs 1` with no refusal.
- **Validation-1:** aligned. The scope's intent (the row, the sender, the accept literal, stamped live) stands; the
  scope at rev 4 carries the premise correction (C4 by construction) and the mend closed against `inputs#I8`.

## The claim (P5 step 4, after the word)
v2-25 stays this chunk's and is still proven. Its concretization changes, and it is re-claimed with a dated note:
- **now:** «… `XS` (rig label `XS`, kept stamp `cross-stamp.json`) prints `cross-session-prefix: pass`, … a
  print-mode `claude -p` sender's one `SendMessage` to Run B's session id arrives … A red `XS` verifies nothing.»
- **proposed:** «… `XS-2` (rig label `XS-2`, kept stamp `cross-stamp-2.json`) prints `cross-session-prefix: pass`,
  `cross-session: harness` and `pass-count: 29/29`, and its kept stamp reads `29 ok`. Its probe: Run B starts with the
  compiled `crossSessionInbound` accept literal; a print-mode `claude -p` sender's one `SendMessage` to Run B's socket
  address, which verify composes as `uds:<runtime dir>/cc-socks/<Run B's child pid>.sock` with no read, arrives as a
  UserPromptSubmit whose raw start is one of the four compiled harness literals (outcome `harness`). The recorded
  `UserPromptSubmit.cross-session-1.json` … `29 pass 0 fail`; under the accept literal the row still reads `fail` when
  no prompt arrives or it arrives unprefixed (…). A red start (`XS`, or a red `XS-2`) verifies nothing.»
The outcome is not weakened: the same three live atoms, the same `29 ok`, the same CI replay and the same keeps-its-red
case.

## Beyond the plan
none

Apply? (yes / review / cancel) — with the word, X1's answer: (a) or (b).
