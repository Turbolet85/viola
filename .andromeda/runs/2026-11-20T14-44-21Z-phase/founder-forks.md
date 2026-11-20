# Founder forks — «A new CLI version stamped automatically» (P3 stop)

Chunk `2026-11-20-a-new-cli-version-stamped-automatically` · viola-0.2.0 · Epoch 4 · run dir
`.andromeda/runs/2026-11-20T14-44-21Z-phase/` · written 2026-11-20T15:18:48Z at the end of P3, before P4 (the phase
directive `inputs#I1`, item 3). Nothing below is planned yet: **the block WAITS for the founder.** Every fact is read
from the code at `816f9a77` (research `doc:2026-11-20-a-new-cli-version-stamped-automatically/research`, rev 2) or
from the store; nothing was started, no `claude` was spawned, and nothing of `~/.claude*` was read.

## F1 — The one block

### 1. How the gate decides today, and what `viola verify` is

**The gate.** `viola run`'s start (`src/cmd/run.rs:261`, and revive's start, which calls the same function at
`src/cmd/revive.rs:154`) asks the resolved `claude` for `--version` once (5 s bound), then reads
`<home>/ledger/stamps.json` behind the strict-modes check (`src/run/version_gate.rs:147-193`). The version is verified
only when its entry holds every one of the 27 relied rows and each one counts (`pass`, or `absent` on the
browser-tools row alone; `crates/viola-agent-claude/src/ledger.rs:1404-1421`). Anything else is
`cli_verified:false`, silently. The bit is taken ONCE: the send slot, the dialog slot, the readiness gate's compiled
signatures and the start's trust step all receive it by value at start (`src/cmd/run.rs:290-336`, `617`). Nothing
re-reads the stamps while a session runs.

**What `unverified-cli` refuses and what it allows.** It refuses every driver `answer` (exit 12) and gives every
dialog `null` (the human answers it). It allows `send`, but each send is confirmed only after the fact, the readiness
gate stays partial, and a listed local command reads `unconfirmable`. It also skips the start's trust-record write
for an untrusted spawn folder.

**`viola verify` as a process** (`src/cmd/verify.rs:155-322`):
- **Its runs.** One `--version` probe. One print-mode probe on Haiku (≤ 120 s). Six interactive PTY runs (A–F) in
  `<cwd>/.viola-verify-<pid>/`, `-dialogs/`, `-plan/` and `-resume/`, plus two OS-temp folders. One
  `claude agents --json` read inside Run B. That is nine `claude` spawns in all.
- **Its wall.** The last measured live verify took **39 s** (2.1.328, 27/27, rig arm `vr`, ledger
  12:56:04Z → 12:56:43Z; `chunks/2026-11-20-permissionrequest-question-answered/evidence/live-round.md:107`). The six
  first screens settled in 0.5–1.1 s. A stuck run can take up to its probe deadlines, several minutes.
- **What it types.** Only compiled pastes into a settled input box: the probe prompt, three framing texts, then
  `/mcp disable …`, `/context`, `/compact` and `/clear` (Run B); four dialog prompts (Run C); one plan prompt (Run D).
  It types no byte into any CLI dialog. A modal start is killed with no key, and Run B ends with Ctrl-C.
- **What it answers.** Runs C and D's tool dialogs are answered only by the hidden `hook --capture --answers` arm.
  That includes Run C's one `allow`, which runs `touch` inside its own folder. It answers nothing else.
- **What it writes.** `ledger/stamps.json` (under its lock), and its probe folders, which are removed on exit,
  Ctrl-C or TERM but left by SIGKILL or a crash. Run F writes a trust record for a temp folder into the Claude config
  and removes it after.
- **What it leaves in the user's Claude config (accepted residuals).** Run B's three transcripts and one each from
  C and D. The print probe's empty project folder. One input-history line from `/mcp disable`, whose effect on the
  real config is unmeasured. The `/context` share. The user's global hooks and status line run inside its children.
- **What it spends.** Derived from the code, not measured per run, because the recorded runs keep no turn count:
  one Haiku turn (the print probe), plus **about ten turns on the user's DEFAULT model** (Runs B–D pass no
  `--model`): Run B's probe turn, three framing turns and one `/compact` summary; Run C's four dialog turns (each
  with tool calls); and Run D's one plan turn. Run E resumes with no key and spends none.
  - The precedent pricing was «a re-stamp … at least 8 live sessions» (arch history, 2026-10-05-permission-end-to-end).

**A red verify still stamps.** It writes the version's entry with its failing rows and exits 1
(`src/cmd/verify.rs:283-317`). A stopped verify writes nothing.

### 2. The candidate forms — recommended first

**(A) RECOMMENDED — `viola run` (and revive's start) starts `viola verify` itself, BEFORE the session, when the version
needs it and consent is set.**

The order is the pinned copy → the `--version` probe → the trigger predicate (below). If the predicate holds, the
start spawns `<pinned viola> --home <home> verify -- <the resolved program>` as a direct child and waits for it. Then
it reads the stamps again and goes on, verified or not.
- **Rests on:** verify stays the stamps' one writer (`update_stamps`' only product caller is `src/cmd/verify.rs:283`,
  code graph), so «only viola verify writes the stamps» stays literally true. Verify's output reaches the terminal
  before the child holds it (decision/run-terminal-child-only kept). Every per-session consumer of `cli_verified` is
  untouched, because the bit is still set once.
- **Who waits:** the human or launcher that typed `viola run`, for verify's wall: about 40 s measured, minutes when a
  run hangs to its deadline. It happens once per new version (or new row set), never again on later starts.
- **What the human sees:** one line before verify (e.g. «verifying claude X.Y.Z before the session»). Then verify's
  own step lines and its summary, `stamped X.Y.Z  N pass  M fail`. Then the session.
- **A second `viola run` started meanwhile:** recommended an attempt lock beside the stamps (`ledger/verify.lock`,
  `try_lock`). The loser does not run a second verify. It starts unverified at once with one line saying the version
  is being verified elsewhere, and a later start finds the stamp.
  - Set aside: the loser waits for the lock (it would block for the same wall).
  - Without any lock: two verifies run, each spending its turns. The stamps lock serialises their writes, and the
    merge keeps only one entry (the last writer's), so nothing corrupts but the spend doubles.
- **Offline or logged out:** verify goes red, and its red entry is stamped. With the red rule below, that version is
  then not retried until one of the named clearers. This is the form's sharpest failure: one offline start poisons the
  version (see §3's inconclusive option).
- **How it fails:**
  - A start that does not need a verify pays only one extra stamps read.
  - Ctrl-C during verify reaches both processes. Verify cleans up and exits `interrupted`; the start must then go on
    unverified or stop, which is a P4 detail.
  - A SIGKILL leaves verify's probe folders and its trust record, as today.

**(B) Set aside — verify run BESIDE the session (start unverified, turn verified when the stamp lands).**
- **Rests on:** a detached child of the wrapper, plus a stamps re-read when it ends.
- **How it fails:** `cli_verified` is taken by value in four places at start. Making it live means a shared mutable
  bit read by the send slot, the dialog slot, the readiness gate's signatures and the snapshot, while sends and
  dialogs are in flight. The trust step (start-only) never runs for that session.
  - Verify's ~10 model turns run concurrently with the human's or driver's turns on the same account.
  - It may print nothing while the child holds the terminal, so a red verify's failed rows reach only the logs.
  - Its PTY children compete with the session for the host.
  - Likely too large for one window, and every new mutable path is new risk.

**(C) Set aside — a separate verb or flag the launcher calls** (e.g. `viola run --verify-new`, or
`viola verify --if-unstamped`).
- **Rests on:** nothing new in `start`.
- **How it fails:** it is not automatic. Every launcher, `vstart.sh` and the overseer's scripts included, must learn
  the flag, and a plain `viola run` stays unverified. It moves the burden the founder wants gone into the launcher,
  which is the alternative he set aside («В продукте Viola» over a launcher script).

**(D) The consent switch — a `config.json` key, not a form of its own: it gates (A).** See §4.

**(E) Any other the code offers.** The pinned-copy path could let verify's code be called in-process from `start`
instead of as a child process.
- **How it fails:** verify's stop handler, its cwd-relative probe folders and its `cli` log role would all run inside
  the wrapper process, before the wrapper's own log and checks. That re-decides decision/verify-probe-dirs-cleanup and
  the log-order rows.
- The child process in (A) keeps every one of verify's own guarantees.

### 3. A RED verify of a new version (the case that matters most)

- **No loop.** The trigger predicate is «the stamps' entry for this version lacks a relied row id» (a new, pure
  `attempted(stamps, version)` beside `verified` in `viola-agent-claude`).
  - A red verify writes every relied id with its word, so the next start reads the version as attempted and does not
    verify again.
  - A stopped or refused verify writes nothing, so a later start tries again. Each try is one human- or
    launcher-started `viola run`, never a loop inside one start.
- **Says which rows failed.** In form (A), verify's own step lines (`[NN/27] row-id … fail`) and its summary are on
  the terminal before the session starts.
  - Recommended: one closing line naming the version and that sends stay unverified until a human runs
    `viola verify`.
  - Later starts of that version print one line saying the version's verify failed on an earlier start, with no row
    text repeated.
- **Leaves the session to the human as today.** The session starts unverified, exactly as now.
- **Where the failed attempt is recorded.** In the stamp entry itself: its rows, `verified_at` and first screens. No
  new file and no new format, so the additive no-`v`-bump rule is not even needed.
- **What clears it:**
  - a human's `viola verify` (it rewrites the entry);
  - a newer viola build whose relied set adds a row (the entry then lacks an id, so the next start verifies again
    once);
  - a newer CLI version (a different entry).
  - A red verify for a non-CLI reason (the account state, as on 2026-11-06, or offline) is cleared only by those three.
- **Option for the founder — inconclusive is not red.** If verify's print probe captured no hook at all (offline or
  logged out), the automatic verify could leave no attempt behind, so the next start retries.
  - Cost: verify itself must then write no entry for that case. That changes verify's stamping, a widening of
    verify's behaviour.
  - Recommended: **no** for this chunk. Keep verify's stamping as is, and let the human's `viola verify` clear an
    offline red.

### 4. The spend and the consent

The automatic verify spends about 10 default-model turns plus one Haiku turn per new version, and writes into the
user's real viola home (the stamp) and Claude config (Run F's removed trust record, plus the residuals above) with no
human in the loop.

**Recommended consent form:** a `config.json` key, `"verify_new_cli": true | false`, **default `false`**. The founder
sets it once in his own home's `config.json`.
- It lands in `CONFIG_KEYS` (`src/obs.rs:33`) and `schemas/config.v1.json` together (decision/config-keys-match-schema).
  A bad value falls back to `false` with one `parse-rejected{parser:"config-json"}` line, as `dialog_hold_s` does.
- It turns a behaviour ON. It switches off no control (decision/controls-not-switchable holds), and it is never an env
  var or a flag (decision/new-env-read-needs-log-entry holds).
- Each time it runs, the start prints its one line first, so the spend is never silent.
- A user who never wants it does nothing, because the default is off.
- **Why `false`:** every test home of the suite is deliberately unstamped where it tests the unverified paths. With
  default `true`, each such `viola run` over the fake agent would run a verify first, changing what those tests
  measure and adding a verify to every unstamped start. A product spending a stranger's model turns on first run is
  also the widening the security history says must be shown whole (security history, 2026-09-27-wrapper-channel).
- **Set aside:** default `true` with the key to turn it off. This matches «automatically» with nothing to set, at the
  cost above.

### 5. Every master row reached (searched across all seven masters by `rec.py find`, then the picked rows)

**Reached** (this form changes or extends their text; each is an Expected amendment at P4):
- architecture:
  - `decision/run-start-order` (a verify between the version probe and the gate's verdict);
  - `decision/verify-child-set`, `decision/stamps-single-writer`, `decision/verify-sole-stamps-writer` and
    `contract/ledger-stamps/writer` (the writer stays verify; who STARTS it widens to the start);
  - `decision/unverified-cli-transport-only` and `decision/partial-gate-on-unverified-cli` (an unverified start may
    now first verify);
  - `decision/version-gate-strict-stamps-read` (read twice, before and after);
  - `contract/instance-snapshot/cli-verified` (from the second read);
  - `decision/config-keys-match-schema` and `contract/config-json/*` (a new key row);
  - `decision/run-terminal-child-only` (verify's lines precede the child);
  - `decision/real-claude-local-only` / `decision/no-live-claude-in-ci` (the automatic verify is off in every CI home
    by the default).
- security-plan:
  - `decision/stamps-written-only-by-verify` (the writer stays verify; the starter is new);
  - `decision/unverified-cli-gate` and `decision/dialog-answers-need-verify-stamp` (an automatic stamp now lifts the
    gate);
  - `decision/version-gate-strict-stamps-read` and `contract/strict-modes-refusal/version-gate`;
  - `contract/cli-exit-codes/12` (fewer exits 12 after a green auto-verify; the code is unchanged);
  - `decision/controls-not-switchable` and `decision/config-tolerant-parse` (the new key is no control switch);
  - `decision/verify-real-cli-transcripts-accepted`, `decision/verify-echo-probe-residuals` and
    `decision/verify-context-probe-residual` (the same residuals, now with no human present: the accepted class
    widens to unattended runs);
  - `decision/driver-permission-policy-outside-viola` (the driver-injection risk now rests on an automatic stamp being
    honest);
  - `decision/start-writes-trust-record-flag-only` (a start verified by its own verify also writes the spawn folder's
    trust flag);
  - `decision/verify-probe-dirs-cleanup` and `decision/probe-artifacts-cleanup` (Ctrl-C at a start now reaches verify);
  - `decision/strict-modes-check` (the start's home-tree check precedes verify's own).
- design-system: `contract/viola-verify-output/*` (verify's lines now also appear inside a `viola run`), plus new
  §Surface: cli rows for the start's automatic-verify lines.
- test-plan:
  - `decision/stamps-written-only-by-verify`, `decision/stamps-only-from-verify`, `decision/single-writers` and
    `decision/single-writer-state-files` (verify stays the writer, now also started by run);
  - `decision/state-files-never-hand-written` (unstamped test homes stay unverified with the key off);
  - `contract/verify-stop/signal` (a stop reaching verify inside a start).
- obs-plan:
  - `contract/viola-verify-refusal/stamp-write-refused` (unchanged words, a new caller);
  - the `process-start` / `process-exit` `subject` enum (`schemas/diag-line.v1.json:116`), which gains a closed
    subject for the start's verify child.

**Read as NOT reached** (they hold unchanged):
- architecture: `decision/screen-rows-read-only-by-verify`, `contract/trust-writer-words/key-present` · `/removed`,
  `contract/refusal-order/answer`, `contract/refusal-reason/unverified-cli`,
  `contract/channel-methods/hook-dialog-unverified-cli`, `decision/claude-first-no-adapter-trait` (the new predicate is
  pure, in `viola-agent-claude`), `decision/revive-program-resolution`, `decision/verify-refuses-untrusted-home-or-stamps`,
  `decision/retired-exit-codes-never-reused` (no new exit code).
- security-plan: `decision/hook-capture-arm`, `decision/new-env-read-needs-log-entry` (no env read),
  `decision/live-round-rig-keys-own-pty` (no key typed into the live session), `decision/revive-recorded-cwd-spawn-dir`
  (verify runs in the start's process cwd, not the recorded one).
- design-system: `contract/viola-verify-refusals/stamps-update`.
- test-plan: `contract/strict-modes-refusal/verify`.
- layout-templates and a11y-plan: no row matched the searches. Their picked rows (83 and 128) are read at P4 by search
  for the new cli lines.

**The widenings, said whole here:**
- (i) `viola run` and revive's start spawn `viola verify`, and through it nine `claude` children, which no start spawns
  today (precedent: «a new process spawning claude» was a founder-ratified widening, 2026-10-17).
- (ii) Verify's accepted residuals (transcripts, project folders, input-history line, `/context` share, global hooks)
  arise with no human present, once per new version or row set.
- (iii) A stamp is written into the user's real viola home with no human running verify.
- (iv) A new `config.json` key.

### 6. How it is proved — fake agent first

**Over the fake agent (CI and local):**
- a home with the key on and no stamp: `viola run` runs verify once, the stamp lands, and the session's snapshot reads
  `cli_verified:true`;
- the same with the key off: no verify, and the session is unverified;
- a red fake verify (a fake script failing one row): the stamp holds the red entry; a second start runs no verify (zero
  verify spawn pairs in its log) and prints the earlier-failure line;
- a stamp short of a relied id: the next start verifies again once;
- two starts at once: exactly one verify spawn pair across both logs, and the loser's line;
- a Ctrl-C during the start's verify: no stamp, and the start ends as P4 decides.

Every wait stays bounded under the 20 s lint; no rehearsal sleeps out a real verify.

### 7. Live starts — the SHAPE before the count

**One live start shape (recommended).** The standing rig in a SCRATCH viola home (never the live home), its private
Claude config and credentials copy as today, `config.json` with `"verify_new_cli": true`. One `viola run` against
PATH's `claude` (2.1.329, unstamped).
- **Processes.** `viola run` → its `--version` probe → `viola verify` (one `--version`, one print probe, six PTY runs,
  one agents read) → the session's `claude` child. That is up to 11 `claude` processes, at most two at a time (verify's
  current child and none other until verify ends).
- **Every byte typed.** Verify's compiled pastes only (§1); none into any CLI dialog. The rig types no key into the
  session; its stop is the rig's TERM to `viola run` once the snapshot is read.
- **Every dialog and how it is answered.** Verify's Runs C and D only, by the capture `--answers` arm, including Run
  C's one `allow` (the `touch` in its own folder). The session raises none, because nothing is sent.
- **The stop.** The rig's existing stop: TERM to `viola run`, which ends its child. Verify's own exits end its children.
- **Every file written, and under which home.**
  - The scratch viola home: `ledger/stamps.json` with the 2.1.329 entry, `instances/<name>/` (snapshot, events, logs),
    `bin/`, `plugin/`, the rig's ledger.
  - The rig's scratch folder: verify's probe folders (removed on exit), Run F's trust record and the start's trust flag
    in the PRIVATE Claude config, and the CLI's transcripts and project folders in that private config.
  - Nothing under the live viola home or the real `~/.claude*`.
- **What each start reads, and from where.** PATH's `claude` (2.1.329); the rig's credentials copy
  (decision/live-rig-credentials-copy); the scratch home's `config.json` and stamps; the private Claude config.
- **The predicate it can FAIL on:** after the run, the scratch stamps hold a `2.1.329` entry carrying all 27 relied
  ids, the session's snapshot `cli_verified` equals that entry's verdict, and the run log holds exactly one verify
  spawn pair.
  - The red reading: 2.1.329 drifts on a row. The entry holds a `fail`, the snapshot reads `cli_verified:false`, and
    the start printed the failed rows. That is the product working, but the version stays unstamped-green, and the
    drift then owns its own entry.
- **Does any typed key or answered dialog move the wheel before a send?** No. Verify's pastes go to its own children,
  never to the session, and the session gets no key and no send.
- **Proposed cap:** 1 live start, 1 spare. The spare is only for a contended host (`hostwatch` CONTENDED with a
  neighbour) or a rig fault before verify's first spawn, never to re-roll a red reading.
  - The «not repeated» check is the fake agent's, not a second live start.
- This is a new rig arm (the existing `vr` arm runs verify alone, not `run`). Its no-start gate entries for the
  scratch home's absence before and the live home's stamp unchanged after are written at P4 (directive item 4). The
  rig's live arm already refuses under CI (`crates/viola-e2e/src/rig/mod.rs:457`).

### 8. The capability claim

No pooled capability of viola-0.2.0 fits this chunk (the 16 `planned, unclaimed` caps are v2-07…15 and v2-24…31:
fleet, console, MCP bounds, ledger rows for prefix and identity, resize, model pair). So the chunk claims none.
- «Verified» for the chunk would rest on the fake-agent tests of §6 plus the one live reading of §7.
- A new capability («a `viola run` meeting an unstamped CLI version stamps it once, red or green, and never loops»)
  would be the route's or the wrap's to mint, and is offered at P5's `## Beyond the plan` only on his word.

### 9. One window, or a split

**One implement window** holds form (A). The parts are:
- the trigger predicate (pure, in `viola-agent-claude`);
- the attempt lock (`viola-state`);
- the start's verify spawn and second stamps read (`src/cmd/run.rs`, `src/run/version_gate.rs`);
- the config key with its schema;
- the closed spawn subject;
- the start's lines (`src/human/`);
- about six fake-agent tests;
- the rig arm;
- the masters' amendments (§5).

The live leg rides the same chunk's operator leg. Form (B) would not fit one window: it would split into «the verified
bit made live» first and «the verify beside the session» after.

### What waits for the founder (each his, none decided here)

1. **The form:** (A) recommended · (B) · (C) · (E).
2. **The default:** key `verify_new_cli` default `false` recommended · default `true`.
3. **The red rule:** the stamp entry records the attempt, cleared by a human verify, a newer row set or a newer CLI,
   recommended · treat an offline / no-capture verify as inconclusive (a verify behaviour change).
4. **The second-start rule:** the loser starts unverified at once recommended · the loser waits.
5. **The spend:** about 10 default-model turns + 1 Haiku turn per new version or row set, with no human present.
6. **The widenings (i)–(iv) of §5.**
7. **The live cap:** 1 start + 1 spare, in a scratch home under the rig, the shape of §7; no key or byte into any CLI
   dialog beyond verify's own compiled pastes and its capture-arm answers.
8. **Any read or write under his real home:** none proposed.

## The answers (appended 2026-11-20T15:25:27Z at the P4 resume; source the overseer's relay `relay-autostamp-p3.md`, snapped as `inputs#I2`)

The founder's own words, given in one overseer dialog and read at 2026-11-20T15:24:50Z (relayed, PROVISIONAL until his
word is shown to this session):
1. **The form:** «A: verify перед сессией (Recommended)» — form (A); his ratification of widenings (i)–(iv) over the
   rows §5 names as reached, and nothing wider. (B), (C) and (E) declined.
2. **The default:** «Выключено, включаете один раз (Recommended)» — key `verify_new_cli`, default `false`; the spend
   told him as about 10 default-model turns plus one Haiku turn per new version, derived from the code, not measured.
3. **A red verify:** «Как любой красный (Recommended)» — verify's stamping unchanged; a red entry (offline or logged
   out among them) records the attempt, cleared only by a human's `viola verify`, a newer relied row set or a newer CLI
   version. The inconclusive option declined.
4. **The cap:** «1 старт + 1 запасной (Recommended)» — one base start (`--cap 1`), one spare, the shape of §7; the spare
   only for a host CONTENDED by a neighbour or a rig fault BEFORE verify's first spawn, never to re-roll a red reading;
   `~/.claude/.credentials.json` READ once per start by the rig (its copy removed on every exit path); nothing written
   under his real `~/.claude*` or `~/.viola`. No capability claimed. A second base start is his.

The overseer's technical answers:
5. The loser of the attempt lock runs no verify and starts unverified at once with its one line.
6. Ctrl-C during the start's verify: the start STOPS (an interrupted start, no session), no stamp written.
7. A verify child that cannot start or exits by a refusal: the start goes on unverified with one line naming verify's
   own refusal word, writes nothing, is not an attempt; each such exit has a fake-agent or unit case.
8. The earlier-failure line names the version and the remedy (`viola verify`), never row text; every new human line
   and log subject a closed word in its contract row; no new exit code.
9. Every §6 fake-agent proof is in the plan; the two-starts case bounded under the 20 s lint with no sleep on a real
   verify; a control removing the predicate's «attempted» arm reads the no-loop case red.
10. The rig arm's no-start entries stand right before the start; an entry a later step falsifies is keyed to the round
    leg; PATH's `claude` is read for its version right before the start, the predicate naming «PATH's version».
11. The reached rows are the wrap's; no master row reworded in implement; a row outside §5's list that P4 finds reached
    is a fork written here.
12. When the plan is approved the phase ENDS.

## P4 forks — rows the change reaches that §5 did not name (written 2026-11-20T15:35:54Z; relay item 11)

P4 read the seven masters' picked rows by search for the start's new lines, its order, its lock and its keys. Three
reached rows stand outside §5's list. The plan is written on each one's recommended option, and none of them is
reworded in implement. Each is the founder's, and each is shown on the P5 card for his word.

**P4-F1 — the start prints lines of its own before the child.**
- Rows: layout-templates `contract/viola-run/start-output` («`viola run` prints nothing of its own, and the start's
  trust step adds no refusal, exit code, hint, `--json` code or terminal byte; the terminal belongs to the wrapped
  claude TUI until it exits.»), and a11y-plan `contract/tui-passthrough/viola-terminal-bytes` («viola writes zero bytes
  of its own to the terminal; only the child's output reaches it.»).
- With the key on, a start that verifies prints its own one line, then verify's step lines and summary, all before the
  child spawns. Form (A) as shown to him said «its lines reach the terminal before the session»; these two rows were
  not named in §5.
- Options:
  1. **(Recommended)** The wrap amends both rows: nothing of the start's own while the child runs, and before the
     child only the start's closed auto-verify lines and verify's own output. Cost: two more rows reworded.
  2. The start prints nothing. Verify's output goes to null, and the start's outcome reaches the log alone. Cost: a red
     verify's failed rows reach no human until he runs `viola verify` himself, against the directive's «say plainly
     which rows failed».

**P4-F2 — the attempt lock guards an action, not a file.**
- Row: architecture `decision/on-disk-file-naming` («one lock file per guarded file named `<name>.lock`»).
- The loser rule he was shown (relay item 5) needs a lock that no verify of one home can hold twice: `ledger/verify.lock`.
  It guards a version's verify, not a file named `verify`.
- Options:
  1. **(Recommended)** The wrap amends the row: one named exception, `ledger/verify.lock`, the automatic verify's
     attempt lock. Cost: one row's exception.
  2. No attempt lock. Two starts at once each run a verify; the stamps lock keeps the file whole and the last writer's
     entry stands. Cost: the spend doubles, and relay item 5's loser line has no basis.

**P4-F3 — keys typed while the start's verify runs.**
- Rows: a11y-plan `decision/human-keystrokes-never-blocked` («A human keystroke in `viola run` … is never blocked,
  refused or delayed past the current atomic paste.») and `contract/tui-passthrough/human-keystrokes`.
- No child exists while the start's verify runs (about 40 s measured, minutes at a probe deadline). A key typed then
  waits in the terminal's own buffer until the child starts. Ctrl-C stops the start (relay item 6).
- Options:
  1. **(Recommended)** The wrap adds one except to both rows, naming the start's verify window, before any child exists.
     Cost: two excepts.
  2. Read both rows as not reached: they speak of the passthrough, which only the child's spawn enters. No amendment.
     Cost: a reading he has not seen.

**Read as NOT reached at P4** (they hold unchanged under the plan):
- obs-plan `decision/run-silent-terminal`, `decision/run-silent-on-terminal`; design-system
  `decision/run-prints-nothing-while-wrapped`; layout-templates `contract/viola-revive/start-output` and
  `contract/cli-verbs/run`. Each speaks of «while the child runs», and the verify ends before the spawn.
- obs-plan `contract/run-start-spans/span-chain`: the plan adds no span; the step's lines ride `run.start`.
- security-plan `decision/start-checks-socket-dir-before-bind`; layout-templates
  `contract/run-revive-start/socket-dir-refused`; a11y-plan `contract/start-refusal/socket-dir` («before any snapshot,
  child or bind»). The plan places the automatic verify AFTER the endpoint bind, so a refused socket directory still
  refuses before any verify child. This is a refinement of §2(A)'s «between the version probe and the gate's verdict».
- a11y-plan `contract/viola-revive-json/start-refusal`: no new `--json` code. Under `revive --json`, verify's stdout and
  stderr go to null and the start prints no line.
- obs-plan `contract/run-start-events/order`: verify writes no product event.
- test-plan `decision/no-verify-start-outside-test-home` and `decision/verify-under-test-owned-user-home`: every
  run-started verify of the suite runs under the run's test-owned user home, with the fake agent told its config file.
- security-plan `decision/live-rig-kept-stamp-owner-only`: the new rig arm keeps no stamp.

## The P4 forks' answers and the P5 word (appended 2026-11-20T15:56:36Z; source `relay-autostamp-p5.md`, snapped as `inputs#I3`)

The founder's own words, given in a dialog and read at 2026-11-20T15:56:34Z (relayed verbatim by the overseer):
- **P4-F1:** «Печатать, строки поправить (Recommended)» — option 1: the wrap amends layout-templates
  `contract/viola-run/start-output` and a11y-plan `contract/tui-passthrough/viola-terminal-bytes`.
- **P4-F2:** «Одно именованное исключение (Recommended)» — option 1: one named exception, `ledger/verify.lock`, in
  architecture `decision/on-disk-file-naming`.
- **P4-F3:** «Исключение в обеих строках (Recommended)» — option 1: one except on a11y-plan
  `decision/human-keystrokes-never-blocked` and `contract/tui-passthrough/human-keystrokes`, naming the start's verify
  window before any child exists.

Each is his ratification for the rows named and nothing wider. The P5 word on `p5-card-1.md`: **yes** (the overseer's
review). The four leans stand as planned. No plan line changes.
