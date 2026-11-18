# Founder forks — «The 2.1.325 recorded fixture set» (chunk 2026-11-18-the-2-1-325-recorded-fixture-set, phase stopped at P3's end)

The operator is away, so no dialog was raised (the overseer's directive, `inputs#I1`, items 2–5). This file is the
one block. The phase stops here before P4; nothing is planned past P3. The scope is
`doc:2026-11-18-the-2-1-325-recorded-fixture-set/scope` (rev 3) and the research is `doc:…/research` (rev 2).
Paths are written relative to the repository, or `~/…` under the home.

## 1 · Size

**This window at this point:** P1 to P3 of this run are done in it. That means the take-up, 3 fan-out batches
(170 agents), the research reads and the scope closure. I estimate roughly two thirds of its context is used. No
tool measures this, so it is the orchestrator's own reading.

**What the entry asks, as P3 read it** (11 carries, C1–C11; the directive counts ten because C5 and C1's CARRY 2 are
the same start):

| work | kind | carries |
|---|---|---|
| A recording label on the live rig. Today its `V` arm's `--record` is removed with the scratch, and no `R4` label exists (`crates/viola-e2e/src/rig/isolate.rs:571-573`, `crates/viola-e2e/src/rig/mod.rs:1566`). Also a rig witness that the record home names no `home` component. | code, test-only crate | C1·2, C5, C6 |
| The live R4 start, the record reviewed, `fixtures/claude/2.1.325/` landed whole, `STAMPED` gaining `2.1.325`, and the drift suites, probes and recorded-exchange suites run over the new set | live + gates | C1·2, C5, C10 |
| The `viola-mcp-tap` re-record of `Mcp.tools-list-1.json` (one `claude -p`) | live | C1·1 |
| The `bt-absent` control, fired on a filtered entry | gate | C11 |
| The placeholder hint's cell attribute | measure | C7 |
| A live `StopFailure` payload, and settling its exception | live + decision | C2 |
| The live home's 24-row re-stamp, and the product re-pin after it | live under the REAL home | C3, C8, C9 |
| `/context`'s post-condition, and whether `/mcp disable` writes to the REAL config | live under the REAL config | C4 |

**Does it fit one implement window with its live work?** No, not as it stands. Three parts need things one window
cannot hold:
- C3/C8/C9 need the founder's host prepared first (see §2 (b)).
- C2 needs a means not yet measured and then his decision (see §4).
- C4 needs his real config.

The fixture-set core (the rig label, two starts under the rig's private config, the landing, `STAMPED`, the drift
suites and `bt-absent`) should fit one window, on one condition: the 2.1.325 recording replays under the fake agent
with no fake-agent change. That condition is the main size risk, and it is not measured. If a drift suite reads red
on the new set, fake-agent work of unknown size follows.

**The split, proposed (its placement is the founder's: proposed here, not placed):**
- **Stays with the fixture set:**
  - C1 (both CARRYs)
  - C5 (R4, the same start as C1·2)
  - C6 (the rig's record-home half)
  - C10 (read from R4's record: whether Run E starts)
  - C11 (`bt-absent`, filtered)
  - C7, only if R4's kept frames can show the cell attribute; otherwise it leaves with the next entry, its reason «no
    frame read it».
- **Leaves as its own entry, «The live home's re-stamp and the product re-pin»:** C3, C8, C9. Its precondition is
  the browser-tools prompt answered once by a person on 2.1.325. Its next reader is that entry's phase.
- **Leaves as its own entry, «Local commands under the real config»:** C4 (`/context`'s post-condition and the
  `/mcp disable viola-probe-absent` write). It could instead ride the re-stamp entry above, because that verify's
  Run B already pastes `/mcp disable`. That choice is also his.
- **Leaves as its own entry, «A live StopFailure payload»:** C2, with its means and the exception's fate as §4 asks.

> **Recommended:** the split above. The fixture set then needs no start under his REAL home or config. Every start
> it makes is under the rig's private scratch, and the three entries that leave each carry their own precondition
> and reader. The alternative, all of it here, holds the whole chunk until his host is prepared, and puts a
> real-home verify and a means nobody has measured into one implement window.

## 2 · The live starts, each one's shape before its count

### (a) Under the rig's private config: nothing written under the real `~/.claude*` or `~/.viola`

**L1 · R4: the clean `viola verify --record` on 2.1.325** (C1·2, C5)
- **Command:** `viola-live-rig live R4 --cli 2.1.325 --cap 2 --spare 1 --ledger <this chunk's ledger file>`. The
  label is built by this chunk; until then the label reads `refused: usage`.
- **Home and config:**
  - Scratch root: `/tmp/viola-rig-<uid>/<run>/`, holding `cwd/` (made a git root), `config/` (the private
    `CLAUDE_CONFIG_DIR`, seeded with the used-config markers and the browser-tools key OFF), `vhome/` (the viola
    home) and `record/`.
  - No path component is named `home`: `vhome` is not one, and the recorder refuses a payload holding `/home/`.
  - The real `HOME` is kept, so the recorder's scrub rewrites it to `~`.
- **Processes:**
  - The rig.
  - One `viola` built from the tree, the rig's sibling, its hash read right before the start (verification-harness
    2026-10-20).
  - Up to 9 `claude` processes, one at a time, all `~/.local/share/mise/installs/claude/2.1.325/claude` by path.
    A bare `claude` reads 2.1.327 on this host today. The 9 are: the rig's two `--version` reads, verify's
    `--version`, its print-mode probe, Runs A–F, and Run B's one `claude agents --json`.
- **Every byte typed:**
  - The rig types no key into the `V` arm while it runs.
  - Its stop presses Ctrl-C (`03`) every 500 ms only if verify has not exited by itself, then kills the group
    after 10 s.
  - verify types only its compiled literals: Run B's seven pastes (the probe prompt, the long text, the tag-like
    text, the newline text, `/compact`, `/clear`, `/mcp disable viola-probe-absent`), each into a settled box, and
    Runs C/D's probe prompts.
  - Runs C/D's dialogs are answered by the hidden `hook --capture --answers` arm, never by a key.
  - Run E types nothing. No byte goes into any CLI dialog.
- **What it writes, and where:**
  - The scratch: a 0600 copy of the credentials, removed on every exit path; the private config; `vhome/` with its
    `ledger/stamps.json`; `record/2.1.325/*.json`; the CLI's own transcripts under the private config.
  - `target/live-rig/<run>/`: the frames, the failed first screens if any, and the kept record (the new label's
    copy, taken before the teardown).
  - The ledger file: one line before the start, one summary after.
  - Then `fixtures/claude/2.1.325/`, as a plain copy of the recorder's own output, never edited.
  - **Under the real home:** `~/.claude/.credentials.json` is READ once, and `~/.claude.json` is stamped before and
    after, read only. Nothing is written there.
- **Predicates its reading can FAIL:**
  - Rig exit 0, and the summary's `failing` reads `none`.
  - verify exits 0 and its stamp line reads `stamped 2.1.325  24 pass  0 fail`, with `browser-tools-prompt`
    `absent`.
  - `record-left` reads `absent`.
  - `user-config` reads anything but `changed-by-rig`.
  - The kept record holds every file class the 2.1.302 set holds, `SessionStart.resume-1.json` included (C10: its
    absence reads that Run E did not start).
  - `contract_fixture_hygiene` passes over the set.
  - `contract_ledger_probes` passes `2.1.325` at 24 rows.
  - `contract_fake_agent_drift`, `mcp_recorded_exchange` and `cli_verify`'s recorded-screen cases pass over it.

**L2 · the MCP re-record** (C1·1)
- **Command:** one `claude -p` on 2.1.325 by path, its `--mcp-config` naming a server whose command is
  `viola-mcp-tap record <capture> <viola> --home <scratch vhome> mcp`. The prompt goes on stdin, with nothing
  positional after the variadic option (verification-harness 2026-10-20).
- **Config:** a private `CLAUDE_CONFIG_DIR` holding a credentials copy, as the rig places one. How the 2.1.302
  recording was run, and under which config, was NOT read in this window: P4 reads it from that chunk's record. A
  by-hand run under the real config would belong to group (b).
- **Processes:** 1 `claude`, 1 `viola mcp` under the tap.
- **Every byte typed:** the stdin prompt only, a compiled literal the plan names. No key into any dialog.
- **What it writes:** the capture (0600, in a private dir, never committed) and `fixtures/claude/2.1.325/Mcp.tools-list-1.json` by `viola-mcp-tap fixture`.
- **Predicates its reading can FAIL:**
  - `fixture` exits 0 and writes the file; refused codes are `home-path` · `absolute-path` · `username` · `email`.
  - `mcp_recorded_exchange` passes over the file.
  - `list`'s recorded description lacks «and the usage budget».

**No start:** `bt-absent` (C11) and C7's attribute reading, if R4's frames hold it.

**Cap for group (a):** recommended `--cap 2 --spare 1`. The spare is R4's alone (`R4-2`). It is spent only on a cause
R4 itself showed, after its mend lands through a plan revision (directive item 8). One ledger file holds this
chunk's starts, and a later entry reusing it passes the running total as its `--cap`.

### (b) Touching the founder's REAL home or config (leaves under the split; shown so it can be weighed)

**L3 · the live home's 24-row verify** (C3, C8)
- **Command:** `viola --home ~/.viola verify -- <2.1.325 path>`, under the real `HOME` and the real `~/.claude.json`.
- **Processes:** up to 9 `claude` processes, as in L1.
- **Every byte typed:** the same compiled literals as L1, Run B's `/mcp disable viola-probe-absent` among them.
- **What it writes:**
  - `~/.viola/ledger/stamps.json`: the 2.1.325 entry, the only file it writes under `~/.viola`. Its `cli` log opens
    only under a parsed `VIOLA_NAME`, which the start removes.
  - `~/.claude.json`: verify's own trust record for its fresh temp folder, written and then removed.
  - `~/.claude.json` again: whatever `/mcp disable` writes, unmeasured. That is C4's reading.
  - `~/.claude/projects/<key>/`: Runs B/C/D's transcripts, and the print probe's empty project dir (accepted
    residuals).
  - One input-history line from the `/mcp` paste.
  - `<cwd>/.viola-verify-<pid>*`: removed when verify ends.
- **Host precondition:** the CLI's browser-tools prompt answered ONCE by a PERSON in a real 2.1.325 session on
  this host. viola never answers or seeds it, and whether it ever should is asked of him separately (C9). If the
  prompt was never answered, the start reads BP's shape: 8/24, verify exits 1, the stamp is unverified, and nothing
  is gained.
- **Predicate:** `stamped 2.1.325  24 pass  0 fail`.

**L4 · the product re-pin** (C3, C8): see §3.

**L5 · `/context` under a real session** (C4)
- **Shape:** one wrapped `viola run` of 2.1.325, one send of `/context`, its post-condition read.
- **Processes:** 1 `claude`.
- **Bytes typed:** the text `/context` and Enter, through `viola send`, then the Ctrl-C stop.
- **Note:** `/context` does not need the real config. Only the `/mcp disable` write question does, and L3's Run B
  answers that by key-name read of `~/.claude.json` before and after.

**Cap for group (b):** recommended 0 in this chunk under the split. If he keeps L3 here: `--cap 1 --spare 0`, only
after his browser-tools answer.

## 3 · The pinned product (directive item 4)

This chunk does not rebuild or re-pin the product unless his answer says so.

**What a re-pin needs:**
1. A 24-row stamp for 2.1.325 in the live home `~/.viola`: L3, one verify start (up to 9 `claude` processes),
   gated on the browser-tools precondition.
2. The rebuild and its install: no `claude` start.
3. The overseer's switch to the new binary: his.

**What stays transport-only until then:** a binary built from the tree reads every 22- or 23-row stamp unverified
(`verified` needs all 24 relied rows to count; a missing row does not count, `crates/viola-agent-claude/src/ledger.rs:1310-1312`).
On it, `answer` exits 12 and the `/clear`, `/compact` and `/mcp` sends are `unconfirmable`. The pinned product keeps
its own reading until it is rebuilt.

## 4 · The `StopFailure` exception (C2, PROVISIONAL since 2026-11-15; his to settle, not settled here)

**Is a live payload reachable without breaking a connection he uses?** Plausibly yes, by two contained means. Both
touch only the child, and both are UNMEASURED:
1. **A network namespace with no route.** `unshare -rn` works on this host (measured: exit 0;
   `unprivileged_userns_clone` 1). The child's API call fails at connect.
   - Risk: the CLI may fail before it draws its box, or retry for long.
   - Its error word for a lost connection may be `unknown`, or a word outside the 13. That word would read
     `api-error`, with one `parse-rejected` `unknown-value` line.
2. **`ANTHROPIC_BASE_URL` in the child's environment, pointed at a closed local port.** The CLI reads this
   documented variable, and the connection is refused at once.
   - The rig's environment builder would take a test-only option for it. No `VIOLA_*` variable or viola-read
     variable is added.

Each means is one live start: 1 `claude`, one prompt typed and Enter, the Ctrl-C stop.

**The row and probe, if a payload is read:** a recorded `StopFailure.<variant>.json` in the set, its `error` word
read against the nine-word map. A ledger row with a verify probe would need verify itself to induce the failure (a
new run), which is a large change. Recording the payload while keeping the exception for the probe is the smaller
form.

**Options, in order, each with its cost:**
1. **(Recommended)** Its own entry, «A live StopFailure payload»: a contained live payload by means 1 or 2, recorded
   as a fixture, and the row-or-exception decision taken on that reading.
   - Cost: one entry and one or two live starts, none under his real config.
   - Why: the 13 words have never been seen live, a contained means exists, and this chunk stays sized.
2. The exception kept on his word: no row, no probe; the synthetic-payload tests stand.
   - Cost: nothing now.
   - The 13 words stay unmeasured live.
3. A ledger row from the static bundle read, no live payload.
   - Cost: a row whose only evidence is the bundles. It reads as measured when it is not.

## The questions for him, in one block

1. **The split:** take the split in §1 (recommended), or keep all eleven carries here? If split, where the three
   leaving entries go is his to place.
2. **Group (a) starts:** `--cap 2 --spare 1` (R4 + the MCP re-record, the spare R4's alone; recommended), or
   another count?
3. **Group (b):** no start under his REAL home or config in this chunk (recommended under the split), or L3 here,
   after he answers the browser-tools prompt once himself?
4. **The pinned product:** not rebuilt or re-pinned by this chunk (recommended), or re-pinned after L3?
5. **`StopFailure`:** option 1 (recommended), 2 or 3 of §4?
