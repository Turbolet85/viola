# Founder forks — 2026-11-21-mcp-calls-bounded («MCP calls bounded»), P3's end

**This block holds forks of the operator's: a live start (its shape and cap), a widening of the live rig, and what
«verified» rests on for matrix#v2-24. The phase waits here, before P4.**

Written 2026-11-21T19:51:28Z by the phase run `.andromeda/runs/2026-11-21T19-17-43Z-phase/`, under the directive
`inputs#I1` (`chunks/2026-11-21-mcp-calls-bounded/inputs/I1-phase-mcp-directive.md.txt`). No live start was made, no
`claude` was run, nothing under the operator's real `~/.claude*` was read. The research document is
`doc:2026-11-21-mcp-calls-bounded/research` rev 1, the corrected scope `doc:2026-11-21-mcp-calls-bounded/scope` rev 3.

## 1. The window
The take-up (P1), the row read (P2: 7 tasks, 156 picker draws, 7 history agents, every one passing its first try,
1641 of 4782 rows picked) and P3's research are done in this one window; P4 (the plan) and P5 (the review) are not
begun. Remaining in this window after the block: the research checkpoint only.

## 2. What the installed bundles read with NO start (a READING, not a measurement)
Read by `strings -n 8` over `~/.local/share/mise/installs/claude/2.1.328/claude` and `…/2.1.330/claude`. The shapes
are the same in both; the minified names differ.
- **Default, hard limit:** a tool call's wall-clock bound is the server config's `timeout` (when a number ≥ 1000),
  else the env `MCP_TOOL_TIMEOUT`, else **1e8 ms (≈ 27.8 h)**, clamped to ≥ 1000 ms and (by a constant read by name
  only) ≤ 2 147 483 647 ms. The CLI's own schema text: «Hard wall-clock limit per call; progress notifications do not
  extend it.»
- **Idle limit (stdio differs):** for a `stdio` server `CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT` else **1 800 000 ms (30 min)**
  (5 min for other remote types; none for `sse-ide` / `ws-ide`); ≤ 0 disables it; clamped up to the per-server
  `timeout` and down to the hard limit. A 30 s watchdog tick aborts a call with no response or progress for that
  long. **A progress notification resets the idle clock, never the hard limit.**
- **Auto-background:** `CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS`, else **120 000 ms** behind a gate that defaults on, in an
  interactive session (a non-interactive one only under `CLAUDE_AUTO_BACKGROUND_TASKS`); stdio servers are not
  excluded. What it does to the call past 120 s is read from the gate's shape only.
- **At the bound:** the one call is rejected — error class «MCP tool timeout» (`MCP server "<s>" tool "<t>" timed out
  after <n>s`, plus a fixed sentence that the outcome is unknown) or «MCP tool idle timeout»; both marked
  `downstream_unreachable`. **A server teardown at the bound is UNREAD** (not read absent).
- **The names that move it (names only):** the server config key `timeout`; env `MCP_TOOL_TIMEOUT`,
  `CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`, `CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS`, `CLAUDE_AUTO_BACKGROUND_TASKS`,
  `CLAUDE_CODE_DISABLE_MCP_TASK_BACKGROUND`; the gate `tengu_mcp_auto_background`.
- **The only 60 s figure** in either bundle floors the HTTP transports' per-request timeout — not the stdio call's.
  `WAIT_CAP`'s comment («below the client's smallest tool-call default read in the 2.1.287 bundle (60 s)») and
  architecture · decision/mcp-wait-cap's why rest on a 2.1.287 reading that these two versions no longer show;
  `.claude/docs/services/viola-mcp.md:24` records 2.1.287's default as «exceeds 50 s (as measured live)».

**What only a start tells:** that a real client applies this to a real call of viola's stdio server — whether a call
of viola's own longest length returns whole, the error class and the elapsed seconds at a bound, and whether the
server outlives a timed-out call. A strings reading cannot tell which branch a gate takes on this account.

## 3. Each tool's longest block as the code bounds it at `53669b68`
| tool | longest block in code | can it pass the client's bound? |
|---|---|---|
| `send` | gate ≤ 8.5 s (`GATE_MAX_WAIT`) + the paste (**no time bound**: one blocking PTY `write_all`, `viola-pty/src/pump.rs:77`) + window 10 s, or 40 s for `/compact` → compiled 48.5 s + paste | not at the read defaults (30 min idle, 120 s auto-background); yes if a user sets `MCP_TOOL_TIMEOUT` < ~49 s, or the paste or the wrapper stalls |
| `wait` | min(given, 50 000 ms), the wrapper's own clock | not at the read defaults; yes under a user `MCP_TOOL_TIMEOUT` < 50 s |
| `last` | no wait | no |
| `answer` | no wait (the dialog hold is the hook's call) | no |
| `list` | disk scan + one agents read killed at 5 s | no |
**Common to the four frame tools:** the MCP side awaits the wrapper's reply with **no deadline of its own**
(`viola-mcp/src/tools.rs:359` → `viola-channel/src/tokio_client.rs:54`, bounded in bytes only), so a stalled wrapper
holds the call until the client's bound (30 min idle at the default). viola's `plugin/.mcp.json` sets no `timeout`.

## 4. The answer path — recommended, and the ones set aside
**Q1. How does the bound stand in viola?** (the directive's fork of the block)
- **(A) Recommended — a compiled bound with tests, and the reading recorded as a reading.** A compiled MCP-side
  deadline on every frame tool, above every wrapper-side compiled bound (wait 50 s; send 48.5 s + paste) and below
  the tightest client bound read (120 s auto-background); recommended **55 s**, so it fires only on a stalled wrapper
  or paste. Its expiry is a closed result word (P4 names it; a new `UnreachableCause` member or a `wrapper-fault`
  code), never upstream text. A unit test holds each tool's bound (the compiled sum < the MCP deadline < 120 s); a
  test-clock test of the expiry. `WAIT_CAP`'s comment and the mcp-wait-cap why re-based on this reading, per
  version. *Cost:* no start, no ledger row, no stamp moved; one implement window. *How it fails:* nothing in it tells
  that a real client applies the bound as read — on its own it cannot honestly claim «measured», so matrix#v2-24
  is **not claimed** without Q2(S1) (never-weaken: its acceptance says «measured»); a user who sets
  `MCP_TOOL_TIMEOUT` below 55 s still cuts calls short (named in the docs, not prevented).
- **(B) Set aside — a 30th relied ledger row with a `viola verify` probe.** *How it fails / costs:* verify's child
  set widens again (a print-mode MCP client child, its `viola mcp` grandchild, a wrapped fake instance); the probe
  holds a call ≥ 50 s inside every verify; every stamped version (the live home's 2.1.328 and 2.1.330, already short
  five rows) goes unverified until re-stamped, and the automatic verify re-runs on the next start; the fixture set
  (2.1.328, 33 files) re-records; two or more windows. architecture · decision/capability-ledger-gate reads it as the
  default home of an undocumented behaviour — the bound's names are in the CLI's own shipped schema text, its
  default is not: the operator's word decides whether that makes it a ledger row.
- **(A′) Set aside — A plus viola pinning its own server's `timeout` in `plugin/.mcp.json`.** It would make the client's
  bound for viola's server viola's own number whatever the user's env, but whether the CLI honours a plugin-declared
  `timeout` is itself an unmeasured behaviour (a ledger row by the rule above), and `plugin/.mcp.json` is a surface the
  pinned scope does not name (the row read re-runs whole).
- **The paste's missing bound** lives in `viola-pty`, a file the pinned scope names as read only: (A)'s MCP deadline
  covers it from the driver's side; the paste bound itself is research's finding, carried forward by the wrap, not
  folded (folding it is the operator's word and re-runs the row read).

**Q2. Does «measured» in v2-24 rest on a live start?**
- **(S1) Recommended — one live measurement per version by the live rig, by hand:** cap **2** (2.1.330, 2.1.328 by
  path), **1 spare** (for a start refused or lost to a cause outside the bound: a host contention, a rig refusal, an
  account limit — never a retry of a red).
- **(S0) Set aside — no start:** v2-24 stays pooled (`notes` line naming the failed concretization); the chunk ships
  (A) alone.

## 5. Every master row the change reaches (all seven searched)
Searched with `rec.py find` over each of the seven masters (tool-call / timeout / WAIT_CAP / 50 000 / stdio default;
MCP-near time / bound / deadline / wait; closed cause sets; verify's child set; relied-row counts) and the union's
rows files.
**Reached under (A):**
- architecture · decision/mcp-wait-cap (dec-xgdfrbld) — its why («below the CLI's stdio tool-call default») re-based
- architecture · decision/send-time-bound (dec-xr2axwfk) — the total gains the paste's unbounded write and the MCP deadline
- architecture · decision/mcp-unreachable-result-for-every-cause (dec-tywniq7h), contract/mcp-frame-tools/* (server-verify,
  socket-dir-refused, strict-modes-failed) — a new expiry word beside them, if (A) takes an `UnreachableCause` member
- security-plan · contract/instance-unreachable-cause/* (endpoint-lost, instance-dead, socket-dir-refused,
  unwrapped-name), contract/mcp-iserror/instance-unreachable — the closed cause set, if the expiry is a cause word
- test-plan · contract/instance-unreachable-cause/* (six rows), contract/mcp-frame-tools/refusal-result,
  contract/mcp-unreachable-detail/endpoint-lost — the same
- test-plan · decision/time-faked-through-seams (dec-d6bwecvy) — «the MCP adapter runs no tokio timer, so no paused
  test clock»: (A) adds a Tokio timer in `viola-mcp`, so this row's words change (a widening of the test seam, said here)
- obs-plan · contract/mcp-iserror/instance-unreachable, decision/mcp-unreachable-carries-one-cause-word — the same set
- a11y-plan · decision/time-bounds-fall-on-the-driver (dec-2s6hklwf), decision/time-bounds-never-on-human (dec-3cppjeep)
  — each names its bounds by list («MCP wait cap, dialog hold, readiness gate, confirm windows, agents read, the
  start's Claude config lock wait»): the MCP deadline is a new named member beside them, never folded into a class
**Reached under (B) as well:** architecture · decision/capability-ledger, decision/cli-capability-ledger,
decision/capability-ledger-gate, decision/verify-child-set (dec-uczlwirj), decision/version-verified-all-relied-rows
(dec-vlpcishg), contract/viola-verify-output/*, contract/ledger-stamps/*; security-plan · decision/unverified-cli-gate,
decision/dialog-answers-need-verify-stamp, contract/cli-exit-codes/12; obs-plan · contract/verify-exit-codes/0.
**Reached under (S1):** security-plan's live-rig sentence (the rig's MCP recording arm keeps no CLI output, its MCP
config private 0600 inside the scratch) gains a new arm beside the named ones; test-plan's rig rows as P4 finds them.
**Read as NOT reached:** security-plan · contract/mcp-tool-inputs/timeout-ms, test-plan · contract/mcp-wait/timeout-ms,
contract/mcp-wait-tool/timeout-bound, a11y-plan · contract/mcp-wait-tool/timeout-ms (the 50 000 ms cap is unchanged);
architecture · decision/mcp-stdin-bound-in-own-reader, decision/mcp-stdin-line-cap (bytes, not time); contract/mcp-tools/*
and tools/list (the tool set is unchanged); design-system and layout-templates (no surface); the residual rows (no new
residual under (A); (S1)'s are said in §7).
**Except cells under their bound:** none found; the a11y rows' named lists take the MCP deadline as one added member —
if either row's cell cannot hold it under its bound, that is shown here as the operator's, never met by dropping a name.

## 6. The live start (S1) — its SHAPE, before the count
- **Who starts it:** the operator by hand, through `viola-live-rig`'s new arm (implement builds it, with its
  rehearsal over the fake agent in CI; the live arm refuses under CI).
- **Under which home:** the rig's own scratch home under `target/e2e-home/` and a private `CLAUDE_CONFIG_DIR` the rig
  seeds there — never the operator's real `~/.claude*`; the rig's token-window check runs first, its own, never a
  hand read.
- **Processes, per start:** one `viola run` wrapper over the **fake agent** (no real claude child) in the scratch
  home; one `claude -p` print-mode client (the version under test, by absolute path), a direct spawn, no shell, with
  `--mcp-config` naming a private 0600 file inside the scratch that runs `viola mcp` (the scratch home's pinned
  binary) and `--allowedTools` naming `mcp__viola__wait` alone; its `viola mcp` child.
- **Every byte typed:** none. The prompt is one compiled literal passed as argv, asking for one `wait` call on the
  fake instance with no `timeout_ms` (so viola's own longest, 50 000 ms). No key reaches any PTY.
- **Dialogs:** none raised — print mode with the one tool allowed; none is answered. **The wheel:** the fake
  instance's wheel is not moved; `wait` reads, it types nothing.
- **What it reads, and from where:** the client's stdout (`--output-format stream-json`) through
  `take(MAX_FRAME + 1)`, read in memory: the tool result's `is_error` boolean, whether its structured content holds
  `timed_out: true` (a boolean), the seconds between the tool-use and tool-result lines (a number), and one closed
  word for an error (`tool-timeout` · `idle-timeout` · `other`), from a compiled prefix match — never a text kept.
- **The predicate it can FAIL:** `is_error` false AND `timed_out` true AND elapsed ≥ 50 s. **Red readings:**
  `is_error` true with `tool-timeout` or `idle-timeout` (the client's bound fell under viola's longest call);
  `is_error` false with `timed_out` absent (the call never reached viola's cap: the start measured nothing).
- **The stop:** the client exits after its one turn; the rig kills it at a 120 s bound if not; the wrapper and the
  `viola mcp` child are ended by the rig; every process it started ends.
- **Files written, and where:** the private MCP config (0600, inside the scratch, removed with it); the CLI's own
  session transcript under the scratch `CLAUDE_CONFIG_DIR` (a residual inside the scratch, removed with it); one
  evidence line per start in `.andromeda/cache/chunks/2026-11-21-mcp-calls-bounded/evidence/` holding the version,
  the two booleans, the number and the closed word. **No fixture, no stamp, no log line** gets any of the CLI's
  output.
- **What the CLI itself puts in its output (named before any promise):** the model's own text, the tool-use input
  (the instance name and no `timeout_ms`), viola's tool result (`{"timed_out":true}`), the session id, the cwd (the
  scratch path), the model name, cost and token counts. The rig keeps none of it; the prompt and the tool result go
  to the model provider (as every print-mode probe's do).
- **Versions measured:** 2.1.330 (mise `latest`) and 2.1.328 (by path). 2.1.329 is not.
- **Needed ABSENT or PRESENT on disk outside the tree** (each a no-start gate entry right before its start, so a
  round resumed at one entry still fires it): the arm's scratch dir under `target/e2e-home/` ABSENT; `XDG_RUNTIME_DIR`
  PRESENT; the kept products `HF`, `HG`, `XS`, `XS-2` PRESENT and unchanged — read before AND after each start.
  **Their paths are not stated by the directive: asked of the overseer here.**
- **Before the round:** `hostwatch.py read --last 15`; a CONTENDED reading with a neighbour named stops and is re-read.

## 7. Widenings the operator has NOT seen (said whole)
- (S1) The live rig gains an arm that spawns a print-mode `claude` as an MCP client of `viola mcp`, a wrapped
  fake-agent instance and that `viola mcp` child, all in its scratch home; it reads the client's stream-json stdout
  in memory and keeps two booleans, one number and one closed word. **Proof no text lands:** the reader holds the
  bytes in memory only; the evidence line is written from typed fields (bools, a `u64`, a closed enum), so no
  string from the CLI can reach it; the hygiene walk and the record scrub do not see it because nothing is recorded.
  The residuals: the CLI's transcript and project dir inside the scratch; the prompt and result sent to the model
  provider.
- (A) A Tokio timer in `viola-mcp` (test-plan · decision/time-faked-through-seams changes); a new closed result word.
- (B) only if chosen: verify's child set widened, every stamp re-made, the fixture re-recorded.

## 8. The capability's concretized claim, and what «verified» rests on
- matrix#v2-24 now: «The MCP client's tool-call timeout is measured on the current CLI, and every MCP tool call, the
  longest send included, returns below it; a test holds each call's bound.»
- **Concretized under (A)+(S1):** on 2.1.330 and 2.1.328 a real print-mode client's call of viola's longest tool
  call (an MCP `wait` with no `timeout_ms`, 50 s) returns whole, `is_error` false (the rig's evidence lines, by
  hand); the client's bound as READ from both bundles is recorded per version; every frame tool returns by a
  compiled MCP deadline of 55 s, held by a unit test of the compiled sum (send 48.5 s, wait 50 s < 55 s < 120 s) and
  an expiry test. **«Verified» rests on** the two live evidence lines (the operator's leg) plus the unit tests.
- **Under (A) alone:** v2-24 is not claimed; one `notes` line names the failed concretization.

## 9. The cap, the spare, the window
- **Cap 2** base starts (one per version), **1 spare**; the ledger's `--cap` is the running total.
- **One implement window** holds (A) and the rig arm with its rehearsal; the two live starts are the operator's leg.
  (B) would not fit one window.

## 10. Questions for the overseer (facts the directive does not state)
- The paths of the kept products `HF`, `HG`, `XS`, `XS-2`.
- Whether «the client's tool-call timeout measured» on the route entry is met by (S1)'s reading of viola's own 50 s
  call returning whole — it measures that the bound sits above viola's longest call, not the bound's value.

## Answers
_(the operator's word, appended here and snapped; the phase resumes at P4 on it)_

Relayed by the overseer in `~/dev/projects/additional/viola-overseer/relay-mcp-p3.md` (snapped at P4; cited in the
plan as its `inputs#I{n}`), appended verbatim below.

### The operator's own words — given in one dialog and read at 2026-11-21T21:34:21Z

1. **Q1, the bound's form: «Вшитый дедлайн 55 с (Recommended)».** Your (A): a compiled MCP-side deadline of 55 s
   on every frame tool, above viola's own longest calls and below the tightest client bound read (120 s), its
   expiry a new closed result word; no ledger row, no stamp moved. He was shown that a user who sets
   `MCP_TOOL_TIMEOUT` under 55 s still cuts calls short (named in the docs, not prevented) and that the paste's
   missing bound stays a finding of its own for the route. (B) the 30th relied row and (A′) the plugin's
   `timeout` are declined.
2. **Q2, the measurement and the cap: «3 базовых + 1 запасной (Recommended)».** THREE base starts and ONE spare on
   one ledger, through a new arm of the live rig, in the shape of your §6 and §7 as shown to him: a print-mode
   `claude -p` as the MCP client of `viola mcp` over the FAKE agent, in the rig's scratch home and private
   config; one tool allowed (`wait`); no key, no dialog; the client's output read in memory, only two booleans,
   one number and one closed word kept; nothing into a fixture, stamp or log; his credentials file read once
   per start by the rig; nothing written under his real `~/.claude*` or `~/.viola`; the prompt and the tool
   result go to the model provider. This is his ratification of that rig widening and nothing wider.
   - **Two «long call» starts** — 2.1.330 and 2.1.328 by path: your S1 as written (a `wait` with no
     `timeout_ms`, returning whole at viola's own 50 s cap).
   - **One «cut» start, on 2.1.330:** the same call with the rig's PRIVATE MCP config giving viola's server a
     small `timeout`, so the client must cut the call at that bound — it measures the MECHANISM as read: that
     the per-server bound is applied, the error's class, the elapsed seconds, and (as a closed word) whether the
     server outlives the cut call. He was told this is the overseer's addition to your block.
   - The spare: a start refused or lost to a cause outside the reading (a host contention, a rig refusal, an
     account limit) — never a retry of a red. A fourth base start is his.
3. **«Verified» for v2-24 rests on** the three live evidence lines plus the unit tests of the compiled bound and
   its expiry, as your §8 says, with the cut start's line added.

### Technical answers (the overseer's)

4. **The starts are fired by YOU in implement's round, under his cap, as every live leg before** — not by the
   operator's hand: `leg = 'live'` entries on the rig, the ledger's `--cap` the running total (1, 2, 3).
5. **The cut start's shape, for the plan:** the server `timeout` a compiled test value well under the call (say
   whether the client's floor of 1000 ms and its clamp make 5000 ms safe; pick from the code's reading); its
   predicate that can FAIL: `is_error` true AND the closed word `tool-timeout` AND elapsed within a stated
   band of the bound; red readings: the call returned whole (the bound not applied), or `other`. Whether the
   server outlived it: one closed word from a reading you name (a second cheap tool call in the same turn, or the
   `viola mcp` child's state at the client's exit) — if it cannot be read without keeping text, record `unread`
   and say so; it is a READING, never an atom.
6. **The print-mode client runs on Haiku** (name the flag), with no session persistence, in the rig's scratch;
   say what it leaves and that the scratch removal takes it.
7. **Your §10:** the kept products stand in `target/live-rig/`: `floor-stamp.json` (`HF`), `floor-stamp-2.json`
   (`HG`), `cross-stamp.json` (`XS`), `cross-stamp-2.json` (`XS-2`), and the record dirs `HG/` and `XS-2/`
   (derive the exact set from `ls target/live-rig/` at P4 and name each). «Measured» on the route entry is met
   as item 2 words it: the bound sits above viola's longest call on both versions (the long starts), and the
   bound's mechanism is applied as read (the cut start); the default's VALUE stays a reading per version and the
   claim says so.
8. **The expiry word:** your P4 choice between an `UnreachableCause` member and a wrapper-fault code — pick the
   one that moves fewer closed sets and say which rows; either way it is a named member added, never a class.
   The a11y rows' named lists take the MCP deadline as one added member; a cell that cannot hold it under its
   bound is a fork in `founder-forks.md`, shown before the wrap.
9. **The paste's missing bound** is not folded: the plan names it as a finding for the wrap's route-resolve.
10. **The card, as last time:** control ids 1–16 characters, each control on an entry filtered to its guard's own
    cases; every after-read of state OTHER actors move keyed to the round leg, a count other sessions move a
    READING; the round's span holds round and live entries only and the whole-round dry-run shown; the live
    home's stamp sha taken right before the round. A row NOT in your §5 that P4 finds reached is a fork.
11. **The seam after P5.** When the plan is approved the phase ENDS and nothing else starts. On an account-limit
    refusal, stop where you are.

## P4 — master rows found reached that §5 did not name (relay item 10: each a fork; shown on the P5 card)

Found by P4's search of all seven masters (`rec.py find`, the cause-set, tokio, live-rig and timer patterns), after
the expiry word was chosen (P4 choice, relay item 8: `UnreachableCause::CallDeadline`, word `call-deadline` — one
named member of one closed set; a wrapper-fault code was set aside because a code outside the five is read by the
class rule «a code outside the five reads wrapper fault», never as a named member, and because every wrapper-fault
row speaks of a fault the wrapper replied).

- **P4-F1. architecture · contract/unreachable-cause/cause-set (`ctr-x7v6ouwh`)** — «Exit 21 has a closed cause set
  (StrictModes, InstanceDead, UnwrappedName, EndpointLost, ServerVerify, SocketDir as socket-dir-refused) written by
  both clients». The plan adds `CallDeadline` (`call-deadline`), written by the MCP client alone (the CLI never
  writes it: no CLI exit 21 carries it). Options, in order: **(1) Recommended** — the row gains CallDeadline as one
  named member beside the six, with «written by the MCP client alone» on it, at the wrap; cost: one row amended, no
  other set moved (`schemas/diag-line.v1.json` is untouched: MCP writes no line for an unreachable instance).
  (2) a closed word outside UnreachableCause (an MCP-only error word) — moves the MCP error-word set instead and
  three masters' MCP result rows; set aside.
- **P4-F2. security-plan · decision/live-rig-mcp-arm-keeps-no-output (`dec-w5vqy6om`)** — «The live rig's MCP
  recording arm keeps no CLI output, stdout drained and dropped …». The new `MB` arm reads the client's stdout in
  memory and keeps two booleans, one number and closed words (the operator's ratified shape, relay item 2), so the
  recording arm's row stays true of `MT` and a row of its own is owed to `MB`. Options: **(1) Recommended** — a new
  row beside it for the `MB` arm, carrying the ratified shape word for word, the founder's own answer named
  (playbook `rul-bznresq5`: the wrap halts on it once as the answer card); (2) widen the existing row to cover both
  arms — set aside (it would fold a new member into the recording arm's words).
- **Read as not reached:** architecture · decision/tokio-only-in-mcp-crate (`dec-wtettosv`) — its words hold (only
  viola-mcp enables tokio); only its verified evidence quote (`features = ["rt"]`) changes, which the next rows
  verification reads. test-plan · decision/tokio-only-in-e2e-crate — the new handlers case lives in viola-mcp's
  existing Tokio-using suite.
- **Answered** (`~/dev/projects/additional/viola-overseer/relay-mcp-p5.md`, snapped at P5; a delegate's confirm by the
  overseer at 2026-11-21T21:49:41Z, read as lying INSIDE the operator's own answers of 2026-11-21, items 1 and 2,
  PROVISIONAL for his epoch batch; neither moves a plan line; carried to the wrap's answer card), verbatim:
  - **P4-F1: option (1).** `CallDeadline` (`call-deadline`) joins the unreachable-cause set as ONE named member,
    «written by the MCP client alone» — inside his answer «Вшитый дедлайн 55 с», whose option named the expiry as a
    new closed result word.
  - **P4-F2: option (1).** A row of its own for the `MB` arm beside the recording arm's row, carrying the shape he
    ratified word for word — inside his answer «3 базовых + 1 запасной», whose question described that arm.
  - The route entry the card proposes for the paste's missing bound is NOT answered here: a new entry is the
    operator's own; the wrap carries the finding and its proposal to him.
- **Harness files reached (not masters; owed to the wrap's curation, never edited by implement):** `.claude/rules/testing.md`
  §Determinism («`viola-mcp` runs no tokio timer, so no paused test clock»), `.claude/rules/api.md` §MCP (the
  `WAIT_CAP` sentence «below claude 2.1.287's stdio tool-call default»), `.claude/docs/services/viola-mcp.md:24`.
