@task_prose part=chunk
A corrective chunk on viola-0.2.0: it mends the Epoch 5 audit's ranked defects and makes the root crate's mutation baseline green, adding no product behaviour, flag, event kind, refusal code or master rule.
It moves the whole-verify test (and any other test that runs a whole viola verify) into the verify class's kill under the mutants profile, then runs the one announced root-crate in-diff mutation run as an addendum to the code audit's ledger.
It adds killing tests for viola-pty's nine restart-path survivors and viola-state's three survivors, and makes the r2-open rehearsal script refuse on a locked desktop or a stale scratch dir.
It makes three pure moves: one shared start prologue for revive and adopt, one refused() helper for open and up, and three splits (upped in up, the pump's restart code, run's window and trust steps), each file re-measured under the audit's ceilings.
@task_file n=1 path=.config/nextest.toml mark=edits
what: the nextest profiles, the mutants profile's kill limits and overrides — edits: the verify_window_ override's test set
@task_file n=2 path=tests/cli_answer.rs mark=re-reads
what: the answer verb's integration tests, among them the test that runs its own viola verify on a custom-stamped home, re-run under the mutants profile with no TIMEOUT
@task_file n=3 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY output pump and its restart path — edits: the generation guard, the time limits in reap_within and drain, the restart code moved out
@task_file n=4 path=crates/viola-state/src/liveness.rs mark=edits
what: the liveness reads, the ancestor walk ancestors(pid, max) and its bound — edits: the ancestor-walk bound's tests, the other two survivors' tests
@task_file n=5 path=src/cmd/revive.rs mark=edits
what: the viola revive verb — edits: its start prologue, the strict-modes check that runs before the log opens included
@task_file n=6 path=src/cmd/adopt.rs mark=edits
what: the viola adopt verb — edits: its copy of revive's start prologue, replaced by the shared call
@task_file n=7 path=src/cmd/up.rs mark=edits
what: the viola up verb — edits: upped (split for its complexity), refused()
@task_file n=8 path=src/cmd/open.rs mark=edits
what: the viola open verb — edits: refused()
@task_file n=9 path=src/cmd/run.rs mark=edits
what: the viola run verb on the boot path — edits: the window step and the trust step, moved out
@task_file n=10 path=scripts/r2-open.sh mark=edits
what: the r2 open rehearsal script with a live arm — edits: its preconditions read before any start
@task_item n=1 part=needs
The restructure's invariant, one keeping: the revive/adopt start prologue (the strict-modes check before the log opens, the start refusals in their order), the refused() helper of open and up (the human refusal line and the --json refusal document), upped of up (its outcome lines), the pump's restart code, and run's window and trust steps (its placement word, its automatic-verify line, its trust refusal) each do what they did; held by the existing tests green with no edit beyond a re-pointed path or name, and for run the smoke trio.
@task_item n=2 part=needs
The measure a split is read by: each moved and each remaining file under the code audit's complexity ceiling and its 800-line size row, by the audit's own measure, so its over_ceiling and over_800 rows no longer list those three.
@task_item n=3 part=needs
Where a moved unit may live and what it exposes: inside the same crate and command tree, adding no new verb, flag, refusal code, event kind, output line or hint; the shared prologue and the one refused() keep both verbs' --json refusal shape byte for byte.
@task_item n=4 part=needs
How a test reaches a moved unit: by its new path or name only; no assertion of a moved unit's output is rewritten.
@task_item n=5 part=needs
The mutants nextest profile: the whole-verify test moves by name into the verify class's kill (20 s x 4), any other test that runs a whole verify found by P3's census moving with it, the general 15 s kill unchanged; the unmutated root-crate baseline passes with no TIMEOUT.
@task_item n=6 part=needs
The one announced mutation run: behind a stop the overseer answers go, the root crate's in-diff mutants from the recorded base in the recorded form, copy-target off, five jobs, a kill deadline, TMPDIR a short-named scratch dir; landed as an addendum recording a score with caught, missed, timeout and unviable counts.
@task_item n=7 part=needs
viola-pty's nine restart-path survivors (which output copy ended under the generation guard, the reap_within and drain time limits): one killing test per survivor class, each shown red with its guard removed and green with it, no mutation run beyond the one announced.
@task_item n=8 part=needs
viola-state's three survivors, the ancestor-walk bound first (the security plan's limit of 8 ancestor pids, the chain's length below max): one killing test per survivor with its remove-the-guard control.
@task_item n=9 part=needs
The r2 open rehearsal's new refusal: before any start it refuses on a locked desktop (the compositor's locked read) and on a scratch dir that is not fresh, as its sibling scripts do; proved by a check that runs no live arm (a rehearsal or a stub of the lock read).
@task_item n=10 part=needs
What the chunk's checks may touch: no live start, no window opened, nothing sent to the compositor, no key, no real claude run, no live arm of any rehearsal script, and nothing of the operator's real claude or viola home read; every new test hermetic on its own fixture home.
@task_item n=11 part=needs
The work order and its sizing: planned in list order 1, 2, 4, 5, then 3, 6, 7; where it does not fit one implement window, the split is written to the founder forks, not taken by the phase.
@task_item n=1 part=not_needed
Brand identity: the flight-progress strip, the readback box, dialog-pending and stale strips, handoff markers; no strip, board or readback display is built or changed.
@task_item n=2 part=not_needed
Colour palette, typography, spacing and depth: no colour token, SGR span, bold or dim rule, column gap or terminal-background rule is added or changed.
@task_item n=3 part=not_needed
The viola list board and table: columns, row order, NAME width and cut, skip counts, row escaping; the list verb is not touched.
@task_item n=4 part=not_needed
Send readback, the send mirror and viola wait output: no send, readback or wait line is added or changed.
@task_item n=5 part=not_needed
viola link and unlink output and their --json documents; neither verb is touched.
@task_item n=6 part=not_needed
Wrapper fault --json detail, peer reply screening and the wrapper socket's refused peer; no wrapper-socket path is touched.
@task_item n=7 part=not_needed
The hidden mcp and hook verbs' stream and exit contracts; neither verb is touched.
@task_item n=8 part=not_needed
viola --help verb groups and the flat verb register: no verb or command name is added, removed or regrouped.
@task_item n=9 part=not_needed
viola focus exit-1 codes and focus refusals; focus is not touched.
@task_item n=10 part=not_needed
The self-validation protocol's squint, swap and readback-box signature checks and the domain-design rule: they judge a new display, and the chunk adds none.
@task_item n=11 part=not_needed
The web surface and rendered-page absences: there is no web or page surface to reach.
