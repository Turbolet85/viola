# Founder forks — phase of «Model and effort at launch» (2026-11-23T06-49-18Z-phase)

## P3 block — 2026-11-23T07:24:59Z

**This block holds 9 forks of the operator's (F1–F9): the phase stops here, before P4, for his word.**

**Window.** Phase run `.andromeda/runs/2026-11-23T06-49-18Z-phase/`, chunk `2026-11-23-model-and-effort-at-launch`
(pending, tx 4176), taken up 2026-11-23T06:49Z at `d31c2707` (CI green 7/7). P1 and P2 done (1332 of 4834 rows picked,
7 history files, one re-spawn); P3 done (`doc:2026-11-23-model-and-effort-at-launch/research` rev 1, scope rev 3).
No live start, no rig start, no mutation or witness run, nothing of the real `~/.claude*` read. The directive is
`inputs#I1`.

### Measurements (each from `--help`, the recorded fixtures or the code; none from a start)
- **M1 · start.** `claude --help` at 2.1.330: `--model <model>` (an alias such as fable, opus or sonnet, or a full
  name; no closed list) and `--effort <level>` over a closed list of **5** words (low, medium, high, xhigh, max). The
  help names **0** environment names and **0** settings keys for either: both **unmeasured**.
- **M2 · resume.** The help's `--resume` line names no model or effort: what a resume with no flag runs on is
  **unmeasured**. viola's revive adds **0** model or effort words today (`--plugin-dir`, `--resume <id>`,
  `[--fork-session]`, the words after `--`). verify's six typed runs pass `--model haiku`, Run E's `--resume` start
  among them, and 2.1.330 stamps 29 pass, so the CLI **accepts** `--model` beside `--resume`. That the resumed session
  runs on it is **unmeasured**. **0** in-repo spawns pass `--effort`.
- **M3 · payloads.** Fixtures 2.1.328 (33 files): `effort.level` in **16** (PermissionRequest 3, PreToolUse 6,
  PostToolUse 6, Stop 1), value `medium` in all 16, with no `--effort` given. `model` in **1**: the compact
  SessionStart, `claude-haiku-5-5`, the full name of verify's alias. **0** transcripts are recorded, so what a
  transcript carries is **unmeasured**. viola's hook parse reads neither key.
- **M4 · what viola passes and strips.** `viola run` hands `--plugin-dir <dir>` then the words after `--` verbatim. A
  `--model` / `--effort` typed there reaches the child today, unchecked and unrecorded. viola sets **4** environment
  names. It strips every `CLAUDE*` name (identity floor **13** always, **`CLAUDE_EFFORT` among them**). An
  `ANTHROPIC_*` name passes through, unread. The fake agent ignores `--model` / `--effort`.
- **M5 · where a record lives.** The lineage chain: one wrapper line after `wheel{start}`, a snapshot field, a replay
  rule (the last line wins), and revive keeping it from its one snapshot read. A snapshot-only field is lost on a
  replay (`cwd`).
- **Fake vs real.** The fake agent can witness only the argv viola hands it, once taught to receipt it. For the real
  CLI, several things are unmeasured:
  - that a session runs on the pair: model measured once, at start, from a fixture; effort never; resume never;
  - what an unknown value does;
  - any environment or settings channel.

### What «verified» can rest on HERE, per clause of v2-30 (directive item 1)
1. «started with no model or effort … runs on viola's configured default pair» — the argv the child receives (a fake
   receipt) plus the start's record. The real CLI running on it rests on the documented flags (M1) and one fixture (M3,
   model at start only).
2. «one started with a pair runs on it» — the same.
3. «after a resume … runs on the pair its instance recorded» — the revived child's argv (a fake receipt) carrying the
   recorded pair. The real CLI on a resume is unmeasured: the replay hands the pair as flags itself and does not lean
   on the CLI's memory.
4. «an open», 5. «an up», 6. «an adopt» — `viola open` / `up` / `adopt` do not exist (v2-07, v2-12, v2-13 unclaimed).
   These clauses cannot be verified in this chunk.

### The design each fork leans to, and the forks (each option priced, the recommendation marked)

**F1 — how v2-30 is claimed.**
- (a) **Recommended:** claim nothing here. v2-30 stays pooled with a dated ledger note naming what this chunk proves
  (clauses 1–3 at the argv and record level). The claim waits for the last of «`viola open`» / «`viola up`» /
  «`viola adopt`» to land. Cost: v2-30 stays unclaimed through three more entries; nothing is weakened.
- (b) Refine v2-30 into two caps on his word: start + resume here, inheritance by `open` / `up` / `adopt` there. Cost:
  a matrix refine that changes the cap's acceptance text (the never-weaken checkpoint).
- (c) Claim it whole now with the three verbs' clauses gated. Cost: a gate is for an outside block, but these are this
  project's own later entries, so the claim stays open across three chunks and the wrap holds it.

**F2 — viola's default pair: where it is configured, and its value.**
- (a) **Recommended:** both. `viola run --model <M> --effort <E> -- …` gives a pair, and two `config.json` keys hold the
  default.
  - The keys are `default_model` and `default_effort`, read once at the start like `dialog_hold_s`.
  - A bad value takes the compiled default, with one `parse-rejected{config-json, malformed}` line. Absent keys take
    the compiled default.
  - Cost: a new input (a boundary widening, as `dialog_hold_s` was his own answer); obs `contract/config-json/known-keys`
    goes from five to seven; `CONFIG_KEYS`, `schemas/config.v1.json` and the controls table gain rows.
- (b) Flags plus a compiled default only, with no key. Cost: changing the default is a rebuild; no new input.
- (c) Flags plus one key `launch: {model, effort}`. Cost: a nested shape new to `config.json`.
- **The VALUE of the compiled default pair is his alone** (no recommendation): a model word (an alias such as `opus`,
  `sonnet` or `fable`, or a full name) and one of the five effort words.

**F3 — a start with only one of the two named; and a pair also typed after `--`.**
- One half named:
  - (a) **Recommended:** the other half comes from the default pair, and the record always holds both. Cost: none.
  - (b) Usage exit 2 unless both are given. Cost: every start types both.
- The caller's words after `--` also hold `--model` / `--effort` (exactly, or as `--model=` / `--effort=`):
  - (a) **Recommended:** usage exit 2 with one fixed stderr line and no document, as design `contract/cli-exit-codes/2`
    has it. Cost: a new fixed line, and viola reads its caller's words for these two flags alone.
  - (b) viola's pair goes before the caller's words and the CLI decides. Which one wins is unmeasured, so the record may
    name a pair the child never ran on.
  - (c) Take the trailing words as the pair given. Cost: viola parses the caller's argv as its input.

**F4 — validated against a closed list, or passed through; what an unknown value does.**
- Effort:
  - (a) **Recommended:** a compiled closed list of the 5 help words. Any other word is clap's usage exit 2, so no
    `--json` document and no new revive code. Cost: a later CLI word is refused until viola is rebuilt.
  - (b) Pass it through as a code word, and the CLI judges it (its outcome is unmeasured).
- Model:
  - (a) **Recommended:** a closed shape: a code word of `[a-z0-9.-]`, 1–64 bytes, starting with a letter. The CLI judges
    an unknown model (outcome unmeasured), and viola records what it handed. Cost: none.
  - (b) A closed alias list (fable, opus, sonnet, haiku). Cost: it refuses full names and rots with each CLI release.
  - (c) Any string. Cost: it fails the requirement's «closed set of values» and the code-word rule for a recorded value.
- None of the leans adds a refusal with a code, a hint or an exit other than 2.

**F5 — the record's shape, and what `list` and `--json` show.**
- Shape:
  - (a) **Recommended:** a new normalised kind `launch {model, effort}`, `source: wrapper`, written after `lineage` at
    every `run` and `revive` start.
    - It adds a snapshot field `launch?` and a replay rule where the last line wins. It is log-only and untraced, like
      `lineage`, with no `v` move.
    - Cost: `EventKind::ALL` goes from 16 to 17. The start writes four records instead of three: test-plan
      `contract/boot-readiness/events` «lines 1, 2 and 3» becomes 1–4, and `contract/run-start-events/order` moves.
  - (b) Two fields on the `lineage` line, `{parent, role, model, effort}`. Cost: the kind count stays put, but
    architecture `decision/lineage-label-only` and obs `decision/lineage-names-confined` widen to a pair that is no
    lineage.
  - (c) A snapshot field only. Cost: it is lost on a replay, so a revive over a corrupt snapshot hands no pair.
- Shown:
  - (i) **Recommended:** `list --json` and MCP `list` items gain `model?` and `effort?` (wrapped rows only, each
    re-parsed and dropped otherwise). The board is unchanged. Cost: architecture `contract/list-payload/items` and
    test-plan `contract/viola-list-json/items` move.
  - (ii) Not shown anywhere. Cost: none, but a driver cannot read a session's pair.
  - (iii) On the board too. Cost: a seventh column against design `decision/list-six-fixed-columns`, his ruled six.

**F6 — the recorded pair onto the resumed child's argv; and a recorded pair the CLI no longer has.**
- The head question is his: a logged value crossing onto a child's command line is the class he ruled himself for the
  session id (security `decision/session-id-argv-closed-shape`, «a ratified crossing»). **Does the recorded pair cross
  onto the revived child's argv?**
  - (a) **Recommended:** yes. Two words of their closed shape, each one argv element of a direct spawn, built only by
    the agent crate's pair builder, beside `resume_args`, which stays unchanged so verify's Run E argv does not move.
  - (b) No: a revive hands the default pair. Cost: v2-30's resume clause is never met.
- Recorded pair no longer valid:
  - (a) **Recommended:** hand the recorded pair as recorded, and let the CLI judge an unknown model (outcome
    unmeasured).
    - A recorded value that no longer parses takes that half of the current default: an effort word gone from the
      compiled list, or a malformed line.
    - The new start's own record line names what was handed.
    - Cost: none.
  - (b) Refuse with a new closed code. Cost: an eleventh revive `--json` code, a hint and a `tests/cross_exit_causes.rs`
    row.
- Does `revive` take `--model` / `--effort` as an override?
  - (a) **Recommended:** yes, with the same flags, the override recorded. Cost: layout `contract/viola-revive/usage`
    moves.
  - (b) No. Cost: a session's pair can change only by v2-31's later verb.

**F7 — a capability-ledger row.**
- (a) **Recommended:** no row.
  - `--model` / `--effort` are documented in `--help` (M1).
  - The replay hands them explicitly, so viola relies on no undocumented behaviour: architecture
    `decision/capability-ledger-gate` covers undocumented behaviour only.
  - Cost: whether the real CLI runs on the pair is not stamped per version.
- (b) A relied row `launch-pair`, probed in verify's Run B (`--effort` added, the payloads' `effort.level` and the
  SessionStart `model` read).
  - Cost: a widening of verify's child set (his class), relied rows from 29 to 30, and fixtures re-recorded.
  - Every stamped home reads `cli_verified:false` until it is verified again, which turns on transport-only mode and
    turns off the trust step.

**F8 — a live cap.**
- (a) **Recommended:** **0** live starts in this chunk. The real-CLI reading (a pair honoured at a start and on a
  resume) is owed to «The fleet on the founder's host» (v2-28) as a carry. Cost: until then clauses 1–3 rest on the
  documented flags and one fixture.
- (b) **2** capped starts at the operator pass:
  - one `viola run` with `--model haiku --effort low`, then one `viola revive` of it, each reading the payloads' model
    and effort;
  - the fake agent cannot give this reading: it ignores both flags and replays recorded bytes;
  - each start needs no instance of its name present and a stamped home present, read by a no-start entry the plan
    names.
  - Cost: two live starts on his account under the rig's private CLI config.

**F9 — a split.**
- (a) **Recommended:** no split; **one implement window holds it**. About 10 product files and 6 test files, the size
  of «Lineage registry».
- (b) Split it: the record and replay first, the configured default second. Cost: two chunks.

### Settled already — cited, not re-asked
- The pair goes to the child by argv flags alone: no `--settings`, no settings key, no environment name, nothing of the
  user's settings read or written (architecture `absence/status-line-interception`; test-plan
  `contract/viola-run-start/settings`), so v2-31's byte-identical promise stays reachable.
- No terminal byte at the start (layout `contract/viola-run/start-output`).
- A bad flag value is clap's exit 2 with no document (design `contract/cli-exit-codes/2`).
- No env read (architecture `decision/no-environment-configuration`).

### Every master row the change reaches, swept over all seven masters (research §Sweeps)
**Reached (on the leans).** Rows marked `+` are new:
- architecture:
  - `contract/instance-snapshot/data` (`{… lineage?}` + the pair)
  - `decision/snapshot-replay-log-fields-only` and `contract/snapshot-envelope/unreadable-snapshot` (the recovered
    field set «links, agent_session_id, wheel and lineage» + the pair)
  - `decision/wrapper-events-log-only` (its kind list + `launch`)
  - `+ contract/normalised-event-kinds/launch` and `+ contract/event-data/launch`
  - `decision/revive-in-place` («keeps the parent and role» + the pair)
  - `contract/child-argv/caller-arguments`, `contract/claude-child-argv/plugin-dir` and `contract/claude-child-argv/resume`
    (the pair's place in the argv order)
  - `decision/claude-flag-literals-in-agent-crate` (+ `--model`, `--effort`)
  - `+ contract/viola-cli-arguments/run-model` and `run-effort` (beside `run-parent` / `run-role`)
  - `+ contract/config-json/default-model` and `default-effort`
  - `contract/list-payload/items` and `+ contract/list-item/model` / `effort`
- security-plan:
  - `+` siblings of `decision/run-lineage-flags-checked`, `decision/lineage-kept-to-snapshot-and-event-log`,
    `decision/lineage-parent-authenticates-nothing` and `decision/board-lineage-and-links-name-checked`, for the pair
  - `decision/session-id-argv-closed-shape`'s class (F6)
  - `+` a `contract/config-json/*` row for each key
- test-plan:
  - `contract/run-start-events/order` (3 records → 4)
  - `contract/boot-readiness/events` («lines 1, 2 and 3» → 1–4)
  - `decision/snapshot-replay-fields` (the field set)
  - `contract/viola-list-json/items` (+ `model?`, `effort?`)
  - `decision/controls-not-disableable`'s guard `every_flag_variable_and_config_key_is_a_row_of_the_table` (two flags,
    two keys)
  - `contract/fake-agent-receipt/start` (+ `model`, `effort`)
- obs-plan:
  - `contract/run-start-events/order` and `decision/product-order-from-events-log` (+ the `launch` line)
  - `+` a sibling of `decision/lineage-line-untraced`
  - `contract/config-json/known-keys` («five known keys» → seven)
- layout-templates:
  - `contract/viola-revive/usage` (`[--model <M>] [--effort <E>]`, on F6's override)

**Read as not reached.** Each holds as written:
- architecture:
  - `decision/run-start-order` («start events» covers the line)
  - `contract/channel-wait/wake-kinds` (the line wakes no wait)
  - `decision/lineage-label-only`
  - `decision/capability-ledger-gate` (on F7 (a))
  - `decision/no-environment-configuration`, `decision/r8-identity-floor`, `contract/config-json/unknown-keys`
- security-plan:
  - `decision/config-cannot-disable-controls` (no key lifts a gate)
  - `decision/r8-claude-env-strip`, `decision/non-identity-hook-names`
  - `decision/revive-trailing-words-as-argv`
- test-plan:
  - `contract/viola-run-start/settings` (stays true)
  - `decision/fake-agent-replays-recorded-bytes` (no payload set)
  - `decision/cli-mcp-identical-payloads`, `decision/identity-floor-tested-against-literal-copy`
- obs-plan:
  - `contract/run-start-spans/span-chain` (the pair step opens no span)
  - `contract/claude-child-process-start/env-*` (no env)
  - `contract/replayed-snapshot/live-fields`
- design-system:
  - `decision/list-six-fixed-columns` and `contract/viola-list/columns` (on F5 (i))
  - `decision/clap-without-colour-feature`, `contract/cli-exit-codes/2`
- layout-templates:
  - `contract/cli-verbs/run`, `contract/viola-run/start-output`, `contract/viola-verb-help/usage`
- a11y-plan:
  - `contract/config-json/dialog-hold-s`, `decision/zero-viola-bytes-on-the-terminal` (no byte)
  - no time-bound row: the leans add no new time bound.

**Counted literals the rows and tests pin:**
- `EventKind::ALL` 16 → 17 (F5 (a))
- start records 3 → 4
- `config.json` known keys 5 → 7 (`CONFIG_KEYS: [&str; 5]` → 7) (F2 (a))
- the identity floor stays 13
- relied ledger rows stay 29 (F7 (a))
- revive `--json` codes stay ten (F4 / F6 (a))

### Disk, SIGKILL and identity (directive item 6)
- **What each new record writes to disk:**
  - one `launch` line in `instances/<name>/events.ndjson`: owner-only append, 0600, never following a link;
  - the snapshot's `launch` field, through `replace_private`, 0600.
  - Both are written before the spawn, beside `lineage`. `config.json` is read-only to this chunk.
- **What outlives a SIGKILL:** both records, once the start passed its record step. A kill before it leaves neither,
  and the start never spawned.
- **Parent identity in a readable file:** none. The record holds two words, a model word and an effort word: no path,
  no session id, no socket, no pid. It stands under the 0700 home, readable by the same OS user only. Nothing lands in
  a public tree. `list --json` / MCP `list` (F5 (i)) show the two words to the same-user caller.

### Test children and bounds (directive item 7)
- No new test child binary.
- The fake agent's `start` receipt gains `model` and `effort`: the word after each flag in its own argv, `null` when
  absent. Cases:
  - `tests/path1_run_start.rs`: a start with no pair receipts the default pair; a start with a pair receipts that pair;
  - `tests/cli_revive.rs`: a revived child receipts the recorded pair;
  - `tests/chaos_snapshot_replay.rs`: after a corrupt snapshot the revive still hands the pair.
- Every wait goes through `tests/support/fake.rs`'s `wait_for`, bounded by `tests/support/watch.rs`'s `WITHIN`, under
  the 20 s lint. Every test start runs under a test-owned user home.

### One implement window?
Yes, on the leans. F9 (b) is the split, if he wants one.

## Answers — 2026-11-23T08:37:39Z (`inputs#I2`, the relay read by the overseer 2026-11-23T08:37:23Z)

**The operator's own words** (his question dialog in overseer1's session, answered 2026-11-23T08:36:42Z, relayed
verbatim):
- **F2, where:** «Флаги + два ключа конфига (Recommended)» — arm (a): `viola run --model M --effort E` plus
  `default_model` and `default_effort` in the home's `config.json`; a bad or absent key takes the compiled default.
- **F2, the compiled default's value:** «opus + medium» — the model word `opus`, the effort word `medium` (the pair
  the builders run on today).
- **F1:** «Не заявлять в этом чанке (Recommended)» — arm (a): v2-30 stays unclaimed with a dated ledger note of what
  this chunk proves (clauses 1–3 at the argv and record level); the claim waits for the last of `open` / `up` /
  `adopt`; nothing of v2-30's acceptance is reworded.
- **F6, head:** «Да, переходит (Recommended)» — arm (a): the recorded pair crosses onto the revived child's argv, two
  words of a closed shape, each one argv element of a direct spawn, no shell — a second ratified crossing beside the
  session id's.

**The overseer's technical answers — a delegate's**, each the recommended arm:
- **F3:** (a) and (a) — one half from the default, the record always both; a pair also after `--` (exact or `=`) →
  usage exit 2, one fixed stderr line, no document.
- **F4:** (a) and (a) — effort against the compiled five; model by its closed shape, the CLI judging an unknown one.
- **F5:** (a) and (i) — kind `launch {model, effort}` after `lineage`, snapshot field, last-line replay, log-only, no
  `v` move; `list --json` / MCP `list` items gain `model?` / `effort?`, the board unchanged.
- **F6, the rest:** (a) and (a) — a recorded half that no longer parses takes that half of the current default, no
  refusal, the new line naming what was handed; `revive` takes `--model` / `--effort` as a recorded override.
- **F7:** (a) — no ledger row; relied rows stay 29.
- **F8:** (a) — zero live starts; the real-CLI reading is a CARRY on «The fleet on the founder's host», its text
  saying what is unmeasured.
- **F9:** (a) — no split.

**For P4 and the card (the relay's directions):** readers of every counted literal named with their search (start
records 3 → 4, kinds 16 → 17, config keys 5 → 7, the list item's keys); the MCP `list` crossing under W1's standing
mark said on the card; the compiled default ONE named constant pair, no figure where its name can stand; no nextest
filter naming a binary not yet built; no `rec.py check` entry between evidence and its land; whether a config key read
at start can leak a host value into a test.
