# Scope — 2026-10-10-statusline-pass-through · Statusline pass-through

**Working entry** (`viola-0.1.0/working-route.md:117`, the head of the markerless tail, in
`### Epoch 4 — Session state & governance`): Statusline pass-through — named read-only source, per-home redirect
for tests, settings.json rewritten each start with absolute pinned path, user output unchanged, readings to
budget.json — plus four CARRY blocks (folded below; completeness check `route.py pins` → the run dir's trail:
four `:117` rows, 231, 200, 212 and 288 chars, no abstention). The entry carries no `BLOCKED-ON`, no `PREREQ` and
no `WATCH`: the chunk has no acceptance gate.

**The operator's direction at take-up** (`inputs#I1`, the invocation, verbatim in the snapshot):
- no mutation run, no witness run and no workflow dispatch is planned;
- a ledger row, a new `viola verify` child, a re-stamp and any live `claude` start are the founder's, and no cap
  is given. The fake agent rehearses. The review card states, each option priced, which rows and probes the entry
  needs now and how many live starts, as steps that wait for his word;
- if the four CARRYs do not fit one builder window, the review card proposes the cut, each left-out part to its
  real first consumer;
- `settings.json` rewritten on every start, and a statusline command run through a shell, are boundary matters:
  each is shown on the review card as a widening, unsettled;
- the operator CI read at the end is the `ci.py conclusion` tool call.

**How a founder matter is planned** (`inputs#I2`): a founder ruling stands until his own word. Work that departs
from one, or that only he can open, is planned up to a stop rule that names what counts as his word; it is never
a decisive lean and never a confirm item.

**CI at take-up** (Setup 5a; the last wrap's flip through HEAD is one sha): `4d77eac` reads green, 15/15 checks,
`ci#38065411713`, wall 451 s. Nothing red or not-green is carried.

**As landed at HEAD `4d77eac`** (read at P1, re-read at P3): no statusline code exists. A search of `src`,
`crates`, `plugin`, `fixtures` and `scripts` for `statusline` printed two test-data lines only
(`crates/viola-agent-claude/src/hook.rs:323`, `ledger.rs:1421`). There is no `hook statusline` arm, no
`instances/<name>/settings.json` writer, no `budget.json` writer, no `statusline_command` snapshot field, no
statusline fixture under `fixtures/claude/`, and no statusline row among the seventeen of `LedgerRow`
(`crates/viola-agent-claude/src/ledger.rs:20-37`). The diagnostics schema already admits every statusline line
and field obs-plan names (research.md finding 2).

**What research changed (P3).** Four things the entry does not say:
- the per-home redirect has one form that keeps every test home off the developer's own settings (item 2);
- the documented statusline payload gives `resets_at` as a number, and omits `rate_limits` in most invocations
  (item 5);
- the Windows half cannot be built honestly now, and is proposed as a cut (item 9);
- the harness flag `boot --statusline-echo` has no user before the governor, and is proposed as a cut (item 10).

## What this chunk builds

### 1. The source: the user's statusline command, named and read-only
- Source: the entry's title ("named read-only source"); architecture §Hook contract (`architecture.md:324`): `run`
  resolves the command from the user's effective Claude Code settings at start, in `viola-agent-claude`, and
  records it in the instance snapshot as `statusline_command` (`architecture.md:255`, an optional field).
- The entry states: the source is named and read-only. viola never writes the user's settings file.
- [premise-corrected: research.md, Platform issues consulted and finding 3 — "effective settings" has several
  scopes, and one named location is what the entry asks for] The named source is one file: the user-scope
  settings file `<user home>/.claude/settings.json`, key `statusLine.command` where `statusLine.type` is
  `"command"`. A status line set only in project, local or managed settings is not seen; the review card says
  so. Which home reads it is item 2.
- Verified (`crates/viola-state/src/snapshot.rs:58-61`, the `cwd` field): `statusline_command` is an optional,
  additive snapshot field, omitted on write when no command was found; the snapshot's `v` stays 1.
- Verified (the security and obs extracts; `security-plan.md:431`): the command string is user content. It
  stands in `snapshot.json` and reaches no process-log line, error body or event.
- Verified (research.md, Patterns detected): the parse of the settings bytes is a pure function in
  `viola-agent-claude`; the read of the file is the root bin's, through `Read::take(MAX_FRAME)`. No
  `viola-agent-claude` → `viola-state` edge is added.
- A settings file that is absent, unreadable, over the cap or not an object, or that holds no command, yields
  no command. The start is not refused and prints nothing (the design and layouts extracts: no statusline
  cause stands in the start-refusal list).

### 2. The per-home redirect for tests
- Source: the entry's title ("per-home redirect for tests"); test-plan §6 Path 6 step 1 (`test-plan.md:838`): the
  settings source the harness writes the user's statusline command to is a §12 open question, and until it is
  resolved the pass-through bullets of that path are blocked.
- Verified (architecture §Cross-cutting Patterns, Config management; the security extract): the redirect is no
  environment variable, no flag and no third test seam.
- [premise-corrected: research.md findings 3 and 13 — a redirect a test must plant first cannot protect a home
  that does not exist before viola creates it] The form: a file in the viola home, `<viola home>/
  statusline-source.json`, in the Claude settings shape, is the source whenever it exists. The user's own
  settings file (item 1) is read only by the default home, `<user home>/.viola`. A home named by `--home` that
  holds no such file has no statusline command. So no test home and no `pre-push` child reads the developer's
  settings, with no step in any test.
- This is a new input that decides a command viola runs through a shell, and a new file in the home that
  nothing lists as trusted (the security extract's last binding). The playbook's "Boundary widening" pattern
  names it. The P4 fork took this form as a planning form only (`inputs#I3`): the widening is being shown to the
  founder, and the plan holds it behind its stop rule until his answer is relayed.

### 3. The per-session override: `instances/<name>/settings.json`
- Source: the entry's title ("settings.json rewritten each start with absolute pinned path") and the third
  CARRY (chunk 2026-09-27-instance-state-and-start-order wrote no such file; this entry rewrites it on every
  start through `viola_state::fs::replace_private`, security-plan §Data Protection).
- Re-verified at P1: `replace_private` exists (`crates/viola-state/src/fs.rs:263`); `security-plan.md:275` (under
  `## Data Protection`) says the file is rewritten by `viola run` at every start, 0600, never reused from an
  existing copy, its command the absolute pinned `bin/<version>-<hash>/viola(.exe)` path; no writer exists.
- The file wraps the statusline with `viola hook statusline` and is passed to the child through the
  settings-override mechanism (`architecture.md:367`), which is an unlanded ledger row (item 8).
- Verified (`src/cmd/revive.rs:103`): `viola revive` starts through `run`'s own start, so it writes the file too.
- Verified (research.md finding 8): `viola verify` already starts the real CLI with `--settings` (Runs C and
  D). The flag is the mechanism; that a `statusLine` key in it replaces the user's status line is not measured.
- [premise-corrected: research.md finding 6 — the documented `command` is a shell string and no `args` key is
  documented] The override's command is the pinned forward-slash path followed by `hook statusline`, written
  with no quoting, and only when the path holds nothing a shell would treat specially (ASCII letters, digits,
  `_`, `-`, `.`, `/` and `:`). For any other pinned path `run` writes no override and passes none: the session
  keeps the user's own status line and viola records no reading. The start is not refused.
- The override is written and passed whenever the start reaches the spawn, whether or not the user has a
  statusline command, so readings are recorded for a user with none.
- **Boundary matter, unsettled** (`inputs#I1`): the review card shows this as a widening. The plan does not
  treat the masters' sentences as his ratification.

### 4. The wrapper: `viola hook statusline`
- Source: the entry's title ("user output unchanged, readings to budget.json"); architecture §Hook contract
  (`architecture.md:324`), §Occupied Resources (`:359`, `:402`), `:173` (it emits no event).
- The entry and the masters state: it reads the statusline JSON on stdin, records the `rate_limits` reading to
  `budget.json`, runs the user's own command with the same stdin, prints that command's stdout unchanged (empty
  when the command fails) and exits 0. It never writes stderr.
- Verified (`security-plan.md:230`, `:556`; `test-plan.md:853`): stdin goes through `Read::take(MAX_FRAME + 1)`,
  and the command never runs from a home or instance directory that fails the strict-modes check.
- Verified (`src/cmd/hook.rs:72-87`; `crates/viola-state/src/snapshot.rs:104-117`): the command is read from the
  snapshot through `VIOLA_DIR`; with no snapshot, an unreadable one, a newer one or one without the field, the
  wrapper prints nothing and exits 0.
- [premise-corrected: research.md finding 10 — `HookEvent` is the closed set of nine hook events] `statusline`
  is not a hook event. The arm is dispatched beside the nine; `HookEvent` and the perf rows are unchanged.
- [premise-corrected: the security extract, first two constraints — the hook's dated gap is worded for the
  `endpoint` read and the `hook.event` frame] The arm borrows no dated gap. Before it reads the snapshot it
  canonicalises `VIOLA_DIR`, requires the result to end `<home>/instances/<name>`, and runs
  `viola_state::strict::check_instance` on that home and instance directory (security-plan §Input Validation,
  CLI arguments row). A failure runs nothing, prints nothing and exits 0 with the landed detail
  `strict-modes-failed`.
- [premise-corrected: research.md finding 11 — no master states a bound] The wait for the user's command has a
  bound, a named constant marked PROVISIONAL. At the bound the arm ends the command it started, prints nothing
  and exits 0 with the landed detail `deadline`. The number is on the review card.
- The user's command gets the stdin bytes the arm read, and the arm's own environment and working directory
  unchanged (what the CLI would have given the user's command itself; the documentation says the CLI sets
  `COLUMNS` and `LINES` there). Its stderr is discarded. No master states these four; the card lists them.
- The arm logs `hook-invoked`, the `statusline-shell` `process-start` / `process-exit` pair and `hook-decision`
  with `budget_written`, as obs-plan §4 Scenario: Budget governor names them; all are in the schema already.
- **Boundary matter, unsettled** (`inputs#I1`): the command is a user-written shell string and the masters run
  it through the shell Claude Code would use (`architecture.md:324`, `:469`; `security-plan.md:34`, `:596`), the
  only shell-out in viola. The review card shows this as a widening. On Unix the shell is `/bin/sh -c`, by its
  absolute path, a hypothesis until item 8's row is measured. The Windows half is item 9.

### 5. The reading: `budget.json`
- Source: the entry's title ("readings to budget.json"); architecture §Occupied Resources (`architecture.md:402`)
  and §Standard Contracts (`:216`, `:183`).
- The masters state: `budget.json` with `budget.json.lock` holds the newest `rate_limits` reading across wrapped
  sessions, timestamped, last-writer-wins, written only by `hook statusline`: per window `{used_percentage,
  resets_at}`, either field `"unknown"` when unparseable, the whole window `"unknown"` when the statusline omits
  it, plus `read_at`.
- Verified (`architecture.md:96`): the statusline JSON is parsed tolerantly; an unparseable value is never an
  error.
- [premise-corrected: research.md finding 4 — the documentation gives `resets_at` as Unix epoch seconds] The
  reader takes `resets_at` as a number of epoch seconds or as an RFC 3339 string, and writes it as RFC 3339
  UTC; anything else is `"unknown"`. `used_percentage` is a number from 0 to 100; anything else is `"unknown"`.
- [premise-corrected: research.md finding 5 — most invocations carry no `rate_limits`] The file is written only
  when the payload holds a `rate_limits` object. Without one nothing is written, the last reading stands, and
  the decision line says `budget_written: false`.
- The file carries `v` (every viola format does) and is replaced through the one helper under its lock.
- This chunk writes the reading. It builds no reader: the gate, the `budget-gate` evaluation, `budget-paused`
  and `release --budget` are the next route entry's ("Budget governor").

### 6. The fake agent
- Source: the second CARRY (chunk 2026-09-24-fake-agent-and-test-data-fixtures deferred the fake agent's
  `statusline-echo` mode and its `settings.json` read to this entry; test-plan §7; "no shape before a recorded
  fixture").
- Re-verified at P1: no `statusline-echo` mode exists; `test-plan.md:838` describes it: the user statusline
  command becomes `viola-fake-agent statusline-echo`, which prints a fixed marker and appends to a marker file.
- [premise-corrected: the tests extract and its history — the one payload the fake agent sets without a
  recording stands on an explicit word, which does not cover statusline stdin] The fake agent invents no
  statusline payload. It runs the override's command only when a test hands it the stdin bytes in a file named
  by an argv option, and feeds those bytes unchanged. Synthetic statusline stdin is test input
  (`test-plan.md:97`, `:1060`), never a file under `fixtures/claude/<version>/`.
- Verified as the design (`src/bin/viola-fake-agent.rs:421-428`): the fake agent reads the override file named
  by the `--settings` argument `run` passes, takes the statusline command there, and runs it directly, never
  through a shell, and only when its first word is an absolute path. It receipts the run.
- `statusline-echo` takes the marker file's path as an argument (the fake agent reads no environment).

### 7. The property test
- Source: the fourth CARRY (chunk 2026-09-27-hooks-to-normalised-events landed the hook stdin and
  `prompt-submitted` round-trip properties but not the statusline one).
- Re-verified at P1: `test-plan.md:1034` (§6 Property suite) says the hook stdin parser landed at `cases: 512`
  with `crates/viola-agent-claude/proptest-regressions/hook.txt` committed, and "statusline JSON with arbitrary
  `resets_at` joins with 'Statusline pass-through'".
- This chunk adds that property: `cases: 512`, committed seeds. The `hook_stdin` fuzz target also feeds the
  statusline reader, with two synthetic seeds (security-plan's parser-surface list names `hook statusline`
  inside the hook stdin parser).

## What waits for the founder's word (planned as steps behind a stop rule)

### 8. The statusline ledger rows and their `viola verify` probes
- Source: the first CARRY (chunk 2026-09-28-capability-ledger-and-viola-verify): the statusline ledger rows
  (`rate_limits.*`, the settings-override mechanism, the shell a statusline command runs through per OS) and
  their `viola verify` probes land here.
- Re-verified at P1: `architecture.md:79` lists the three as rows of the capability ledger; none is among the
  seventeen landed rows.
- Bound by `inputs#I1`: a ledger row, a new `viola verify` child, a re-stamp and a live start are the founder's.
  None is built or run on this plan's own authority. The review card states, each option priced, which of the
  three rows items 3 and 4 rely on now, what probe each needs, and how many live starts.
- [premise-corrected: the arch extract's third constraint and its history — the masters do not say what `run`
  does with the override on a CLI that holds no row for it, and every precedent took the founder's word] Three
  precedents stand: a behaviour with no row was withheld (chunk 2026-10-05-permission-end-to-end); a behaviour
  flowed on the existing stamp under a dated gap he ratified (chunk 2026-10-04-dialog-answers-by-dialog-id);
  `--resume` landed with no row on his live word (chunk 2026-10-10-viola-revive). Which of them the statusline
  shapes follow is his. The plan builds and rehearses under the fake agent and stops before the operator pass
  until his word is recorded.
- Which row each part needs now (research.md): items 3 and 4 rely on the settings-override mechanism and on
  the shell; `rate_limits.*` is first relied on by a `send` refusal, which is the governor's.

## What this chunk does not build

### 9. The Windows half (the proposed cut)
- [premise-corrected: research.md finding 7 — the documented Windows shell is Git Bash when installed, else
  PowerShell; how the CLI finds Git Bash is neither documented there nor measured, and no interactive Windows
  host exists] On Windows `run` writes no override and passes none, and `hook statusline` runs no command: a
  Windows session keeps the user's own status line, the CLI never calls the arm there, and so viola records no
  reading in a Windows session. The arm itself, run directly with statusline stdin, still writes the reading on
  every OS (the seed path test-plan §7 names for `budget.json`). The Windows legs of the two rows go to the
  route entry "Windows-only live measurements". The P4 fork's second answer cut it provisionally (`inputs#I3`):
  the cut is asked of the founder, and a different word from him comes as a revision.

### 10. The harness flag (the proposed cut)
- [premise-corrected: research.md finding 9 — no `--statusline-echo` and no `statusline-source-unresolved`
  refusal exist in the harness, and the flag's first user is Path 6's test] `boot --statusline-echo` is not
  built here. It lands with its first user, on the route entry "Budget governor". This chunk's cases plant the
  source through the root test helpers.

### What else is owed elsewhere
- The budget gate, the `budget-gate` event's evaluation from a reading, the `budget-paused` refusal, the
  `release --budget` override and thresholds in `config.json`: the route entry "Budget governor".
- The `budget` field of `list` and of `/api/sessions`: "The board: viola list" and the Epoch 8 entries.
- The home strict-modes check at every entry point: the Epoch 6 entry "Home and code-bearing file integrity".
  This chunk's arm runs the check it needs before the shell-out (item 4) and borrows none of the seven dated
  exceptions. The Windows case of `check_instance` stays owed to that entry.
- Test-plan §6 Path 6 as a whole: its budget, refusal and override bullets need the governor.
- A perf row for `hook statusline`: the five rows are hook events and the arm is not one.
- The `spend_limit` window the documentation names: architecture knows two windows.

## The capability
- `v1-24` (requirements.md:36) names this entry's subject: the source is named, read-only to viola and
  redirectable per test home, and the wrapped statusline prints the user's output unchanged. P4 reads its
  acceptance and whether a fake-agent run proves it.
- `v1-35` (the governor) and `v1-45` (home integrity) are advanced here and proven by later entries.

## Boundaries
- No mutation run, no witness run, no workflow dispatch; no ledger row, verify child, re-stamp or live `claude`
  start without the founder's word (`inputs#I1`).
- No new env var, config key, flag, listener or channel method. `hook statusline` emits no event and writes no
  snapshot.
- viola writes nothing under the user's Claude Code directory and never edits the user's settings.
- `viola release` stays a human verb; nothing here hints it to a driver.
- Universal invariants that bound every item: the human always wins; `viola hook` always exits 0 and never
  writes stderr; upstream text is content, never a command (the statusline command is the one stated
  exception, and it is on the card); 0700 dirs and 0600 files set explicitly; every replaced file through
  `replace_private`; every external reader through `Read::take(MAX_FRAME)`; only the wrapper writes
  `snapshot.json`; external errors carry codes and fixed messages; stdout is reserved; Claude-specific shapes
  only in `viola-agent-claude`; each relied-on CLI behaviour is a ledger row with a probe.
