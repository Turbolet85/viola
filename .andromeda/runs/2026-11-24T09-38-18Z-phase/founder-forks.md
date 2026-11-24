# Founder forks — «`viola adopt`» (2026-11-24-viola-adopt)

## Block 1 — written at P3's end, 2026-11-24T10:14:15Z, before P4

**Window:** the builder's phase window of 2026-11-24 (run `.andromeda/runs/2026-11-24T09-38-18Z-phase/`), under the
overseer's phase directive `inputs#I1`. Nothing live ran: no `claude` start, no window, no key, no signal, no real
session read, and nothing of the real `~/.claude*` read. The one local test run was the folded CI binary under a
test-owned home (`target/run-archive/641`).

### Measurements (research §Measurements, §Folded CI item)
- **What the tree already gives.** `viola run <name> -- claude --resume <id>` already starts a wrapped resume of any id
  today, unchecked. `revive` adds the closed id shape (`is_session_id`, a usage exit 2), the live-elsewhere reading
  (`claude agents --json` from the typed-in folder, a listed id refused `session-live`, a failed read passing) and the
  id as one argv element (`resume_args`). `adopt` = that start with those two guards and an operator-given name and
  parent. There is no logged-session reading and no recorded directory: a plain session never had either.
- **«Closes a session».** viola has no act today that ends a process it did not start. An agents row (pid, sessionId,
  startedAt, cwd, status) exists only while the session runs.
- **A second resume beside a running one.** No capability-ledger row holds it: UNMEASURED by the ledger. The
  architecture row `decision/revive-refuses-live-session` states as its reason: «both sessions run under one id, two
  writers on one transcript» (a master's statement, not a ledger row).
- **cwd, model, effort.** cwd = the folder `adopt` is typed in, as on `run`: the relied row `resume-session` resumes a
  session from a folder other than its own (Run E's own dir; recorded on 2.1.328; 2.1.330's stamp not read here).
  Model and effort = `--model` / `--effort`, else the configured default. Nothing is read from another process.
- **Name and parent.** They land through `start`: first snapshot `lineage`, then the `lineage` line. A second `adopt`
  of the same id: `session-live` while the first adopted instance runs, passing once it is dead. An `adopt` onto a
  live or stale name: `already-live` / `stale-heartbeat`; onto a dead name: reused, as `run` does.
- **What stays the fake's, unmeasured live:** a real plain `claude` session, started outside viola in its own folder,
  ended and then resumed under viola from another folder on the host's CLI; the real agents read listing a real live
  session.
- **Folded CI item.** `env_name_with_the_user_word_reads_fail_and_records_nothing`: 20.013 s killed (CI attempt 1),
  14.081 s (attempt 2), 9.505 s locally. `kept_name_reaches_no_stamp_or_record`: 16.907 s, 9.130 s, 7.519 s. Each runs
  one whole `viola verify --record`. Neither is in the `verify_window_` class that `tests/support/verify.rs:564` puts
  every verify-driven test under, so each takes its binary's 20 s kill. Attempt 1 had 27 of 440 cases at ≥ 10 s in
  the 20 s-kill binaries, against 9 on attempt 2: the runner's slow tail sat on top of the cases' own cost.

### Rows reached (to be amended at the wrap, none edited here)
- **design-system:** `contract/help-output/verb-groups`, `contract/viola-help/verb-groups` (the `setup:` row gains
  `adopt`).
- **layout-templates:** `contract/cli-verbs/help`, `contract/viola-help/verb-groups` (the same).
- **architecture:** `contract/cli-json-output/revive-refusal-codes` and `contract/cli-json-error-document/shape` (the
  code list for `adopt`'s documents); `decision/revive-refuses-live-session` (`adopt` takes the same reading: a row
  beside it); `contract/child-argv/resume` and `contract/claude-child-argv/resume` («a revived start adds `--resume`»,
  and now an adopted one too); `decision/silent-roles-at-internal-error-catch` (adopt's start joins the named list);
  `decision/cli-flag-and-subcommand-style` (holds: one lower-case word).
- **security-plan:** `decision/session-id-argv-no-shell` (holds, and binds the new id source: W2);
  `decision/refusals-hold-no-paths-pids-ids` (holds); `decision/state-tree-strict-modes-before-use`,
  `decision/home-tree-strict-modes-before-log`, `decision/exec-commands-absolute-pinned-path` and
  `decision/no-writes-under-claude-home` (each names `run` and `revive`'s start: `adopt`'s start joins each list).
- **obs-plan:** `contract/strict-modes-refusal-exit/1`, `decision/home-tree-check-before-log-init` (the same lists
  grow); revive's start-arm role rows (`adopt` logs as process `run` into `run-<name>.ndjson`).
- **test-plan:** `contract/cli-human/internal-error` (its silent list grows).
- **a11y-plan:** `contract/start-refusal/home-strict-modes`, `contract/start-refusal/socket-dir`,
  `decision/strict-modes-refusal-before-any-write`, `contract/json-fault/internal-error` (their named lists grow).
- **Read, not reached:** the MCP tool set (`adopt` is CLI-only under W3's arm); the two time-bound list rows and the
  operator's 2026-11-24 bound answer (`adopt` adds no bound — the agents read keeps its 5 s); the Hyprland seam's
  closed request set and «product caller» wording (`adopt` makes no compositor request); the window-step rows (no
  window step under T2's arm).
- **Playbook escalate rows read before this block:** `rul-r74hshif` and `rul-p2zl2kgl` (boundary widening, which only
  a human's own word ratifies), `rul-o2tr55aj` (held items), `rul-bznresq5` (his own answer to a P4 fork). W1–W3 below
  are of the first class.

### His forks (each a boundary widening; the conservative arm is taken PROVISIONAL meanwhile)
**W1 — does `adopt` ever end or signal a process it did not start?**
1. **(Recommended, taken PROVISIONAL)** Never. `adopt` resumes only a session that is live nowhere, and refuses a
   listed one (`session-live`, exit 1) with a hint to end it by hand there first. Cost: you end the plain session
   yourself (`/exit` in its terminal), and copy its id before you do, since the board lists it only while it runs.
2. Signal the agents row's pid (SIGTERM), after a pid + `startedAt` check, then resume. Cost: viola's first path that
   kills a process it did not start; a pid-reuse window; a session ended mid-turn loses that turn; new security and
   obs rows, and a live reading of the real CLI's exit on SIGTERM (unmeasured).
3. Resume beside the running one. Cost: two writers on one transcript, by the architecture row's own reason — the
   case `revive` refuses.

**W2 — an operator-typed session id crosses onto the claude child's argv** (`revive` takes only an id its log holds).
1. **(Recommended, taken PROVISIONAL)** Only in the closed session-id shape (`is_session_id`, a usage exit 2
   otherwise), as one argv element built by `resume_args`, in a direct spawn, never through a shell. Typed by a human
   at the CLI only (W3). Cost: none beyond today's `run -- claude --resume <id>`, which takes any words unchecked.
2. As 1, plus a driver may give the id through MCP (W3 option 2).

**W3 — is `adopt` an MCP tool?**
1. **(Recommended, taken PROVISIONAL)** CLI-only, like `release`, `pause`, `link`, `focus` and `up`. Cost: a driver
   cannot adopt; you type it.
2. An MCP tool `adopt`. Cost: the tool set grows (the `Tools exactly …` rows); a driver could resume any closed session
   of the same user under a name; the MCP call deadline would bound a start that waits on the automatic verify.

**W4 — the wording of `adopt`'s own `session-live` pair** (layout-templates' history: a new pair's wording is a
planning form until your word; `revive`'s pair is your wording of 2026-10-16 and names `viola revive <name> --fork`,
which does not fit `adopt`).
1. **(Recommended, taken PROVISIONAL)** `unable: that session is open in another claude` / `hint: close it there
   first, then adopt it again` — no name, id or path, no hint naming `release`, and shared with no other code
   (a11y-plan · decision/one-hint-per-code). Cost: one more pair, pinned by literal.
2. Your own wording. Cost: none to the build — the literal moves in one place and its tests.

**Also reached, from the history read:** architecture · contract/instance-snapshot/cwd, whose two readers stand
ratified on your word (2026-11-23). `adopt` writes `cwd` through the same start as `run`, from the typed-in folder,
adding no new reader. It is named here so the wrap does not meet it unannounced.

**R5 — the live reading (gates v2-13; fired only on your word, its moment yours to name).**
- **R5a, adopt a real plain session.** In a scratch folder you start a plain `claude` yourself, copy its session id,
  and end it yourself (`/exit`). Then, from another scratch folder: `viola adopt <id> --name adopt-probe --parent
  <a name of yours>`, run by `scripts/r5-adopt.sh` (no inherited `VIOLA_*`, a test-owned viola home). The script reads
  the board's named row and its parent (with no `claude` reachable), sends one prompt and waits for its turn end, then
  ends the wrapper. **Starts:** your one plain start, plus one resumed start under viola, plus the automatic verify's
  own starts if 2.1.330 is unstamped in that home. **Leaves:** the instance dir under the test home (removed by hand,
  named on the card), the session's transcript growing by the probe's turn (the CLI's), and the trust record for the
  second scratch folder in the Claude config (it stays, as on every `run`). **Undo:** remove the two scratch folders
  and the test home by hand.
- **R5a's grant parameters, beside the home (`inputs#I2`, added at P5 review 1):**
  - the one prompt: `Reply with the single word: adopted`;
  - the pair: `haiku` / `low`, the cheapest the CLI takes, handed on the resumed child's argv and read back only as
    recorded;
  - the spend: one model turn on that pair, whose input is the adopted session's whole transcript plus the prompt —
    a few thousand input tokens for a one-exchange probe session — and about one word out; nothing more unless the
    home is unstamped.

  He may name another prompt-free pair in his grant; the prompt is compiled.
- **How the script hosts `adopt` (both arms, and the rehearsal by the same function):** a util-linux `script -qfec`
  pseudo-terminal the script starts, its input a FIFO the script holds open, closed on every exit path by one trap
  that ends only the two pids it recorded (the wrapper, verified by `/proc/<pid>/exe`, then `script`), with a 10 s
  bound before SIGKILL to the same process, then the FIFO removed by name. Each line reads `ended=yes` only when both
  are gone.
- **R5b, the live-elsewhere refusal on a real session.** You start a plain `claude` and leave it running. The script
  runs `viola adopt <its id> --name adopt-live` and reads `session-live`, exit 1, and no child. **Starts:** your one
  plain start; no viola start. **Leaves:** nothing beyond the test home's `run-adopt-live.ndjson`. **Undo:** end your
  session as you like.

### Technical forks (to the overseer)
- **T1, the argument surface:** `viola adopt <session-id> --name <n> [--parent <p>] [--model <m>] [--effort <e>]
  [--json]`. `--parent` defaults to the inherited `VIOLA_NAME` (as on `run`), and a `--parent` equal to `--name` is a
  usage exit 2. No `--role`, and no words after `--`. Lean: this.
- **T2, the terminal:** `adopt` runs in the terminal it is typed in (as `revive`), opening no terminal and running no
  window step. Lean: this; `viola open`'s path stays `open`'s.
- **T3, the cwd:** the typed-in folder, with no `--cwd`. Lean: this (above).
- **T4, a dead name:** reused as `run` reuses it. Lean: this, adding no word.
- **T5, words and exits:** exit 1 under the closed codes `strict-modes-failed` · `already-live` · `stale-heartbeat` ·
  `session-live` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error` (each already
  one of revive's), each as the stderr pair or under `--json` the one document; exit 2 for usage. The `session-live`
  pair for `adopt` names no id: `unable: that session is open in another claude` / `hint: close it there first, then
  adopt it again`. No new record, field, log line, event kind or time bound.
- **T6, the folded CI item:** rename both cases into the `verify_window_` class (the tree's own form for a test that
  drives a whole `viola verify`). The witness is each case's measured time against its 80 s class kill, locally and
  under coverage. The kill line is not moved for any test.
- **Window:** one implement window holds the entry and the folded CI item: one new verb file of revive's shape, one
  booted test file, the help and table rows, one live script with its rehearsal, and two renames. No split is asked.

### What an `adopt` writes, leaves and lets happen (directive item 6)
- **To disk:** nothing new in kind. These are `run`'s start writes under the adopted name: the instance dir (0700),
  `snapshot.json` (its `lineage` and `launch`), `events.ndjson` (`wheel`, `lineage`, `launch`, then the hook's
  `session-start` with the resumed id), `heartbeat`, `diagnostics/run-<name>.ndjson` (the start pair, the
  `agents-probe` pair, a refusal's `process-exit`), the pinned copy and plugin folder, and the Claude trust record for
  the typed-in folder on a verified CLI (it stays).
- **After a SIGKILL:** a stale instance (snapshot, log, heartbeat), as any killed wrapper leaves it, which `revive`
  reopens. The session's own transcript is the CLI's.
- **What one session can do to another through `adopt`:** end nothing; start one claude child resuming a closed
  session of the same user under a name; learn only whether an id is listed live, which `viola list` already shows;
  change that session's transcript by the turns typed into it afterwards. CLI-only under W3, so a driver reaches it
  only through a shell it already has.

### Can the entry be built whole on these arms?
Yes. On W1 1, W2 1, W3 1 and T1–T6, every clause of v2-13 is built and proved over the fake agent: the id shape, the
`session-live` refusal over planted agents rows, the resume argv, the name and parent recorded, and a send and a wait
on the adopted instance. The clause that the fakes cannot carry — a real plain session ended and resumed under viola
on the host's CLI — is the claim's gate, R5, fired only on your word. The phase goes on to P4 without waiting.
