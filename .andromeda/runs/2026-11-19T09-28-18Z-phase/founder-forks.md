# Founder forks — «The 2.1.328 recorded fixture set» (chunk 2026-11-19-the-2-1-328-recorded-fixture-set)

Written at the END of P3, before P4 (the overseer's directive, `inputs#I1`, item 3). The operator is away: no dialog
was raised. The phase stops here until the answers below arrive. P4 (the plan) has not started.

## The one block

**Window.** No tool measures this builder's context. By its own read, P4 and P5 fit in what is left. If they do not,
a fresh window resumes P4 in this run dir from the rows files, the history files and the landed scope (rev 3) and
research (rev 1).

**One implement window?** Yes. The rig needs NO code change for 2.1.328. `Arm::of_label` maps `R4` / `R4-2` → `VR`
and `R4M` → `MT` for any version, and `live … --cli 2.1.328` resolves the program by its mise path
(`crates/viola-e2e/src/rig/mod.rs:176-177`, `crates/viola-e2e/src/bin/viola-live-rig.rs:233-235`).
- Without C3, the chunk is the two starts, the landing (a plain copy into `fixtures/claude/2.1.328/`), one line
  (`STAMPED` gains `"2.1.328"`, `tests/contract_ledger_probes.rs:44`) and the gates.
- With C3, it adds one small test-only edit to the `CX` arm and one rehearsal update.

**The fake agent (directive item 2).**
- The drift, hygiene and MCP-exchange suites walk EVERY set at run time. The ledger probes run over the literal
  `STAMPED` list.
- The fake is keyed to a version only through the set's own files. Two compiled literals are the exception:
  - `DEFAULT_CLI_VERSION` 2.1.287, not moved here;
  - `TRUST_OPTIONS`, 2.1.325's «No, exit» / «Yes, I trust this folder», drawn over the set's recorded `Screen.modal`.
- A recorded modal keeps signature rows alone, so the new decline text cannot reach the set. It could matter only
  if a compiled literal moved: overseer question T1 below.
- Corroboration (not proof): `L1` stamped 2.1.328 at 24/24 on the live home, every relied row read through the
  compiled literals, and `L2` started on 2.1.328 under the private config with `box: up`, `modal: none`.
- A drift red on the landed set stops implement before the operator pass. It is not stretched there.

### F1 — The cap for this chunk's live starts (the founder's word)

**The starts' shape: the 2.1.325 chunk's two, unchanged but for the version and the ledger path.** All are on claude
2.1.328 by path, under the rig's PRIVATE `CLAUDE_CONFIG_DIR`, in one ledger file
(`.andromeda/cache/chunks/2026-11-19-the-2-1-328-recorded-fixture-set/evidence/fixture-starts.ndjson`). Each start is
ledgered with the clock's time before it is made. `--cap` counts base starts in that file; a `-2` label is the spare.

- **`R4`** (the `VR` arm): one clean `viola verify --record` of claude 2.1.328.
  - **Processes:** up to 9 `claude` processes, one at a time: the rig's `--version` reads, verify's `--version`, the
    print probe, Runs A–F, and Run B's one `claude agents --json`.
  - **Bytes typed:** the rig types NO key. Verify pastes only its compiled literals, into its own runs' input boxes:
    Run B's seven, the probe prompt through `/mcp disable viola-probe-absent`.
  - **STOP:** the rig's own Ctrl-C into its own pty, and only on an overrun of `--deadline`, never while a dialog
    stands. The entry's `stop` is `census --kill`.
  - **Files written:**
    - the scratch under `/tmp/viola-rig-<uid>/`, removed at teardown;
    - the kept record `target/live-rig/R4/record/2.1.328/` and the frames under `target/live-rig/R4/`, both local and
      gitignored;
    - the ledger file.
  - **Real home:** the credentials file is READ once (an owner-only copy placed in the scratch and removed), and
    `~/.claude.json` is stamped read-only by the needle witness. Nothing else under the real home.
- **`R4M`** (the `MT` arm): one `claude -p` with its one MCP server `viola-mcp-tap record … viola --home <vhome> mcp`.
  - **Processes:** 1 `claude`, 1 tap, 1 `viola mcp`.
  - **Bytes typed:** the only input is the compiled stdin literal `reply with the single word ok`, with no key into
    anything.
  - **STOP:** the rig's no-key kill past `--deadline`.
  - **Files written:**
    - the scratch's 0600 `mcp.json` and `capture.ndjson`, removed with the scratch;
    - the tap's fixture `target/live-rig/R4M/record/2.1.328/Mcp.tools-list-1.json`;
    - the ledger file.
  - **Real home:** the same credentials read and read-only stamp as `R4`.
- **Spare `R4-2`:** `R4` again, spent only on a cause `R4` ITSELF showed, after its mend lands through a plan
  revision. `R4M` has no spare.

**Predicates: each start can read RED, and the reading that turns it red.**
- **`R4`** is RED on any of these:
  - `pass-count` other than `24/24`, `failing` other than `none`, `verify-exit` other than `0`, or
    `browser-tools-prompt` other than `absent`;
  - any `screen-b`…`screen-f` other than `box` (`screen-e: box` = Run E started);
  - `record: none`, `exists` or `unread`;
  - a teardown word off: `census` ≠ `0`, `credentials` ≠ `removed`, `scratch` ≠ `absent`, `record-left` ≠ `absent`;
  - `user-config: changed-by-rig`;
  - any `refused:`.

  `user-config: unread` is no red: it is a word the rig prints when it had nothing to read, so it is recorded as
  read and its cause goes to a carry (verification-harness.md 2026-11-19). The 2.1.325 plan's `lacks user-config:
  unread` atom is dropped for that reason.
- **`R4M`** is RED on `mcp-child` ≠ `exit 0`, `capture-lines: 0`, `tap-fixture` ≠ `written`, a teardown word off, or
  any `refused:`.
- **After the landing** (no start spent):
  - the `diff -r` / `cmp` entries print anything;
  - `contract_ledger_probes_pass_over_every_stamped_set` reads anything but `stamped 2.1.328  24 pass  0 fail`;
  - a drift, hygiene or recorded-exchange case reads red.
  Each stops implement before the operator pass.

| option | starts | what it buys | cost |
|---|---|---|---|
| **A (Recommended)** | `R4` + `R4M` as 2 base, + spare `R4-2` | the set, as the 2.1.325 chunk did | up to 11 `claude` processes; C3 then needs F2's other branch |
| B | `R4` + `R4M` + `L2` as 3 base, + spare `R4-2` | the set, and C3's tag half read (F2 option A) | one more wrapped start; one `--model haiku` turn at most |
| C | `R4` + `R4M` as 2 base, no spare | the set | a red `R4` with its own cause ends the round, and the set is owed to a later word |

**Recommendation: B** if F2 takes its option A, else **A**. What the spare `R4-2` is for: a 2.1.328 difference that
only a private-config interactive verify shows. Examples: a startup migration the rig's seed (`migrationVersion` 15,
read from the 2.1.325 bundle) no longer skips, or a first screen verify reads differently. Either is mended through a
plan revision, then `R4-2` is spent once.

### F2 — C3: `/context`'s tag half (the founder's word)

**What P3 measured.**
- This entry's recording `R4` cannot read it: verify's Run B pastes are compiled literals, and `/context` is none of
  them (`crates/viola-agent-claude/src/ledger.rs:180-213`).
- `L2`'s two unread halves were unreadable by construction:
  - `/context` is no listed local command (`crates/viola-agent-claude/src/ledger.rs:201`), so `viola send` waits for
    a `prompt-submitted` the CLI never fires for a local command;
  - the `CX` arm looks the transcript up ONCE, before the send (`crates/viola-e2e/src/rig/mod.rs:1552`), so a
    transcript the CLI writes later stays unread.

**Options, in order.**
- **A (Recommended):** mend the `CX` arm and spend one start in this chunk.
  - **The mend** (test-only, `crates/viola-e2e/src/rig/mod.rs`): when the transcript was absent before the send,
    look it up again after the send and count tags from its start. Its rehearsal is
    `rig_rehearsal_cx_sends_context_once_and_reads_its_post_condition`.
  - **The start:** label `L2` again (no label code), wrapped, `--model haiku`, one `viola send` of `/context` from a
    0600 text file, no key, under the private config. The rig's Ctrl-C into its own pty is the stop.
  - **Files written:** the scratch, removed at teardown; the frames under `target/live-rig/L2/`.
  - **Real home:** the same credentials read and read-only stamp as the other starts.
  - **RED predicate:** `context-transcript: absent`, read AFTER the send by the mended lookup. That is a real
    reading (the file does not stand), and it is what the mend exists to change; it would send C3 to option C.
    `context-tags: unread` follows from it and gets no atom of its own (verification-harness.md 2026-11-19).
  - **Readings, not reds:** a tag count, `0` or more. `0` reads «2.1.328 writes no stdout tag for `/context`», a
    finding recorded as read and never a verification (tests · decision/live-failing-reading-not-verification).
  - **What it does not claim:** `send-1-reply: not-delivered:no-prompt-submitted` is expected and is no red.
    Listing `/context` as a local command is a product change outside this entry.
  - **Cost:** one test-only edit, its rehearsal, one base start.
- **B:** the tag half leaves this entry, unread. It is re-pinned whole on «A live StopFailure payload», the next live
  round under the private config, with this P3 reading attached. Cost: no start here; the carry moves a second time.
- **C:** a primed `CX` start: send `reply with the single word ok` first, wait for `turn-ended`, then send `/context`
  against the transcript the first turn wrote. This is more certain to find a transcript. Cost: one extra model turn
  and a larger edit of the arm (two sends).

**Recommendation: A.** The carry asks for the reading «beside the next live recording under the rig's private
config», which is this round. The mend is the smallest that can read it. C is the fallback if A's start reads
`context-transcript: absent` even after the send.

### Overseer questions (technical — the overseer's relay answers at once)
- **T1.** On the 2.1.328 bundle, did any literal that verify, the recorder or the fake agent reads move?
  - The literals: `Yes, I trust this folder`, `Yes, allow external imports`, `Yes, use my browser`, the footer
    literals, and the refusing row the fake draws as `No, exit`.
  - Is the new decline text a row that is one of those literals?
  - If none moved, the fake needs no 2.1.328 mend before the landing.
- **T2.** Does the 2.1.328 bundle still skip its startup migrations at `migrationVersion` ≥ 15, the rig's seed
  (`crates/viola-e2e/src/rig/isolate.rs:672-686`)?
- **T3.** Does 2.1.328 still open an MCP stdio session with `server/discover`, then `tools/list`, as 2.1.302 did and
  as the 2.1.325 exchange holds? The tap's `fixture` keeps frames up to the first `tools/list`. A changed opening is
  read by `mcp_recorded_exchange` after the landing and stops implement if red.

## Answers
_(appended verbatim when they arrive, then snapped `--message-file`, `--kind relay`, and cited `inputs#I{n}`)_

Arrived as the overseer's relay file `~/dev/projects/additional/viola-overseer/relay-fixture-328-p3.md`, read whole at
2026-11-19T10:11Z and snapped by `--source` (cited `inputs#I2`); its body, verbatim:

> ## The founder's answer — «3 + 1 запасной (Recommended)», given in a dialog and read at 2026-11-19T10:09:58Z
>
> 1. **The live cap for THIS chunk:** THREE base starts — `R4` (one clean `viola verify --record`, up to 9 `claude`
>    processes one at a time), `R4M` (one `claude -p` with `viola-mcp-tap`), `L2` (one `claude` on haiku, one `viola
>    send` of `/context`, after the small test-only mend of the `CX` arm) — plus ONE spare `R4-2`, spent only on a
>    cause `R4` ITSELF showed, after its mend lands through a plan revision. On claude 2.1.328 by path, under the
>    rig's PRIVATE config. From his real home `~/.claude/.credentials.json` is READ once by the rig (a 0600 copy
>    removed at teardown) and `~/.claude.json` is stamped read-only; NOTHING is written under his real `~/.claude*`
>    or `~/.viola`; no byte into any CLI dialog. Each start is ledgered with the clock's time (`date -u`) before it
>    is made; a start refused for the token window is uncounted — stop and say so. A fourth base start is his.
> 2. So your F2 is option A: the `CX` mend and `L2` in this chunk. If `L2` reads `context-transcript: absent` after
>    the send, that reading is RECORDED and the chunk goes on; the primed two-send start is NOT made here — it is a
>    carry proposal for the wrap, and its start a further cap.
>
> ## Technical answers (the overseer's)
>
> 3. **One implement window; no code change to `R4` / `R4M`.** One ledger file; `--cap` is the running total of base
>    starts in it (`R4` 1, `R4M` 2, `L2` 3), the spare by `--spare 1` on `R4`'s entry. A census entry after the LAST
>    live leg is keyed to the operator leg so a round resumed at one entry still fires it.
> 4. **The atom `lacks user-config: unread` STAYS on every one of the three starts.** The cap's promise — nothing
>    written under his real home — rests on that witness being READ; an unread witness leaves the promise unproven,
>    so here it is a red, with that reason written beside the atom. [… items 5–8 as in the snapped copy, inputs#I2]

Items 5–8 (T1–T3 the builder's own static reads at P4; `L2`'s exact readings; the host read before the round; the
seam after P5) stand whole in `inputs#I2`.
