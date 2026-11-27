@task_prose part=chunk
A corrective chunk that mends the Epoch 5 audit's ranked defects. It adds no product behaviour, flag, event kind, refusal code or master rule.
It moves the root crate's whole-verify test into the mutants profile's verify-class kill, then makes ONE announced in-diff mutation run of the root crate and records it as an addendum to the code audit's ledger.
It adds killing tests, each with a remove-the-guard control, for viola-pty's nine restart-path survivors and viola-state's three survivors, the ancestor-walk bound among them.
It makes r2-open.sh check the locked desktop and a fresh scratch dir before it starts anything.
It makes three pure moves: one start prologue shared by revive and adopt, one refused() helper shared by open and up, and three splits (upped, the pump's restart code, and run's window and trust steps). It re-measures each moved and remaining file against the audit's ceilings.
@task_file n=1 path=.config/nextest.toml mark=edits
what: the nextest profiles, the test gate's runner configuration and its per-test kill deadlines — edits: the mutants profile's verify_window_ override (20 s x 4), which gains the whole-verify tests by name
@task_file n=2 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY pump behind the viola run TUI passthrough, the portable-pty outer-PTY boundary and the child's restart path — edits: the restart path's generation guard and the time limits in reap_within and drain (new killing tests), and the restart code moved out to a file of its own
@task_file n=3 path=crates/viola-state/src/liveness.rs mark=edits
what: the liveness check over a pid's ancestors — edits: ancestors(pid, max) and its chain-length bound (new killing tests)
@task_file n=4 path=src/cmd/revive.rs mark=edits
what: the viola revive verb, its start and its refusals, the strict-modes refusal that runs before the log opens included — edits: the start prologue, moved into one function that revive and adopt both call
@task_file n=5 path=src/cmd/adopt.rs mark=edits
what: the viola adopt verb — edits: its copy of revive's start prologue, replaced by a call to the shared prologue function
@task_file n=6 path=src/cmd/up.rs mark=edits
what: the viola up verb, its instance line and its --json refusal — edits: the function upped (split for its complexity) and the refused() helper (made one with open's)
@task_file n=7 path=src/cmd/open.rs mark=edits
what: the viola open verb and its --json refusal — edits: the refused() helper, made one with up's
@task_file n=8 path=src/cmd/run.rs mark=edits
what: the viola run start on the boot path, its window step and its folder-trust step — edits: the window and trust steps, moved out to files of their own
@task_file n=9 path=scripts/r2-open.sh mark=edits
what: a live-arm script for the open rehearsal — edits: its preconditions, read before any start: the desktop not locked (hyprctl locked) and a fresh scratch dir
@task_item n=1 part=needs
Restructure invariant. The pure moves keep what each unit does: the revive/adopt start prologue, with the strict-modes refusal before any write and before the log opens; the open/up refused() helper, with its CLI refusal --json shape and exit code; upped, with the viola up output; the pump's restart code, with the TUI passthrough, the pump_child error and the reap wait bound; and run's window and trust steps, with the window step's 5 s wait bound. The existing layers hold these, green with no test edited beyond a re-pointed path or name, and the refusal --json tests stay green unedited.
@task_item n=2 part=needs
A moved unit's tests reach it only by a re-pointed path or name. No test assertion is rewritten, and a moved unit exposes no more than its callers need.
@task_item n=3 part=needs
Each moved file and each remaining file must sit under the code audit's complexity ceiling and its 800-line size ceiling, measured the audit's own way.
@task_item n=4 part=needs
run.rs is on the boot path, so its split is proved by the smoke trio as well as the layers.
@task_item n=5 part=needs
Mutants nextest profile: the whole-verify test path4_a_stamp_failing_the_dialog_rows_leaves_dialogs_to_the_human moves by name into the verify class's kill (20 s x 4), along with any other test P3's census finds that runs a whole verify. The general 15 s limit is not raised, and the unmutated baseline must pass with no TIMEOUT.
@task_item n=6 part=needs
The one announced mutation run: the root crate's 372 in-diff mutants from 15a566e8, run behind an announced stop that the overseer answers. It runs with --copy-target=false, -j 5 and a kill deadline, with TMPDIR set to a short-named scratch dir. It lands as an addendum recording a score with caught, missed, timeout and unviable counts. It is the mutation audit that covers invariants held in crate tests.
@task_item n=7 part=needs
viola-pty restart-path survivors: one killing test per survivor class. The classes are the generation guard (which output copy ended) and the time limits in reap_within and drain, which bear on the switch's 10 s reap wait. Each test is red with its guard removed and green with it.
@task_item n=8 part=needs
viola-state survivors: one killing test per survivor, starting with the ancestor-walk bound (chain.len() < max, the security plan's limit of 8 ancestor pids). Each has its remove-the-guard control.
@task_item n=9 part=needs
r2-open.sh refuses before any start on a locked desktop and on a scratch dir that is not fresh. A check proves this without running the live arm, through a rehearsal or a stub of hyprctl.
@task_item n=10 part=needs
The chunk's own boundary, binding its new tests and its run: no live start, no window opened, nothing dispatched to the compositor, no key sent, no real claude run, no live arm of any r*.sh, and nothing read from the operator's real ~/.claude* or ~/.viola. Live readings never gate.
@task_item n=11 part=needs
Nothing beyond the nine carries. No viola-e2e or whole crate goes under mutation, the window code is not reworked, viola verify is not split, and the r*.sh scripts get no shared library.
@task_standing n=1
convention: each new test runs inside the existing nextest suites, and its failure is a nextest failure; there is no separate driver or suite
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: each new test is free of flakes and gets no retries
home: 10-slo-invariants-a11y-budgets
@task_standing n=3
convention: each new test runs unskipped under the CI test gate
home: 1-a11y-scope-summary
@task_item n=1 part=not_needed
The viola list board rows (caption, order, colour, stale-row dim, bold NAME, NAME truncation, the unwrapped separator): the chunk touches no board output.
@task_item n=2 part=not_needed
The viola --help rows (group labels, usage line, handoff row, served verbs): the chunk adds no verb and no flag.
@task_item n=3 part=not_needed
The rows for viola link and unlink, plugin install, hook calls, MCP frame tools, send refusals and the instance-unreachable details: those verbs and surfaces are outside the nine carries.
@task_item n=4 part=not_needed
The screen-reader rows and the manual keyboard and Orca passes: the chunk runs no manual pass and opens no terminal for a reader.
@task_item n=5 part=not_needed
The N/A web areas (contrast, focus management, WCAG mapping, structured violation JSON) and the absences (internationalization, touch, web or graphical frontend): viola has no such surface, and the chunk adds none.
@task_item n=6 part=not_needed
The cognitive rows on new copy (fixed copy from design, reading-level gate, one hint per code, no hint names release): the chunk writes no new message, hint or code word.
@task_item n=7 part=not_needed
The viola wait, switch result-line and list --json output rows, and the config.json dialog_hold_s and MCP wait timeout_ms values: the chunk changes none of these outputs or values.
