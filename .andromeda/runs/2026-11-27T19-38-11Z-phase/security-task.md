@task_prose part=chunk
A corrective chunk on the Epoch 5 audit's ranked defects: no product behaviour, flag, event kind, refusal code or master rule added.
It moves the root crate's one whole-verify test into the verify class's longer kill under the mutants test profile, then runs ONE announced in-diff mutation run of the root crate and lands its score as an addendum to the code audit's ledger record.
It adds killing tests, each with a remove-the-guard control, for viola-pty's nine restart-path survivors and viola-state's three survivors, the ancestor-walk bound first.
It makes the R2 open script refuse, before any start, on a locked desktop or a scratch dir that is not fresh.
It makes pure moves: one start prologue shared by revive and adopt, one refused() helper shared by open and up, and three splits (up's upped, the pty pump's restart code, run's window and trust steps), each file then under the audit's size and complexity ceilings.
@task_file n=1 path=.config/nextest.toml mark=edits
what: the nextest profiles, the mutants profile's general kill and its per-class overrides — edits: the verify_window_ override's test list (the whole-verify test joins it by name), the general 15 s limit unchanged
@task_file n=2 path=tests/cli_answer.rs mark=re-reads
what: the path4 test where a stamp failing the dialog rows leaves dialogs to the human, which runs a whole viola verify on a custom-stamped home; its unmutated baseline must pass under the mutants profile with no timeout
@task_file n=3 path=crates/viola-pty/src/pump.rs mark=edits
what: the pty output pump, its restart path: the generation guard on which output copy ended and the time limits of reap_within and drain — edits: killing tests for the restart path's survivors; the restart code moved out to a file of its own
@task_file n=4 path=crates/viola-state/src/liveness.rs mark=edits
what: the liveness reader and its ancestor walk, ancestors(pid, max) bounded by the security plan's limit of 8 ancestor pids — edits: killing tests for its three survivors, the chain-length bound first
@task_file n=5 path=src/cmd/revive.rs mark=edits
what: the viola revive verb and its start prologue, the strict-modes check that runs before the log opens included — edits: the prologue, moved into one function revive and adopt both call
@task_file n=6 path=src/cmd/adopt.rs mark=edits
what: the viola adopt verb and its copy of revive's start prologue, the strict-modes check before the log opens included — edits: the prologue copy, replaced by a call to the shared prologue function
@task_file n=7 path=src/cmd/up.rs mark=edits
what: the viola up verb, its upped function and its refused() helper that writes the --json refusal — edits: upped, split for its complexity; refused(), made one with open's
@task_file n=8 path=src/cmd/open.rs mark=edits
what: the viola open verb and its refused() helper that writes the --json refusal — edits: refused(), made one definition both verbs call
@task_file n=9 path=src/cmd/run.rs mark=edits
what: the viola run verb on the boot path, holding the wrapper's window step and the start trust step — edits: the window and trust steps, moved out to files of their own
@task_file n=10 path=scripts/r2-open.sh mark=edits
what: the R2 live-round script for viola open — edits: its preconditions, the desktop not locked (hyprctl locked) and its scratch dir fresh, read before anything starts
@task_item n=1 part=needs
The restructure's invariant: the shared revive/adopt start prologue keeps the strict-modes check of the home tree before the log opens and before any read of the snapshot, in the same order for both verbs, with the same strict-modes refusal for adopt and for run and revive; the shared refused() keeps the --json refusal shape of open and up; the upped split, the pump's restart code (generation guard, reap_within and drain limits) and run's window step (moving only its own terminal's window) and trust step (skipping the home folder and trusted folders, writing the trust record flag only) do what they did; held by the whole layers green with no test edited beyond a re-pointed path or name, and by the smoke trio for run on the boot path.
@task_item n=2 part=needs
The mutants profile's kill for whole-verify tests: the path4 test, and any other test that runs a whole viola verify, joins the verify class's 20 s by 4 override by name; the general 15 s kill is not raised.
@task_item n=3 part=needs
The one mutation run, mutation testing at the epoch audit: the root crate's in-diff mutants only, behind an announced stop the overseer answers, with TMPDIR a scratch dir outside the repo; its mutants.out is not uploaded; its score lands as an addendum to the audit's ledger record with caught, missed, timeout and unviable counts. No other mutation run, no whole crate and no viola-e2e under mutation.
@task_item n=4 part=needs
The ancestor-walk bound in the liveness reader: the walk stops at the limit of 8 ancestor pids and is held in memory; the killing tests prove the chain-length bound, each red with the guard removed and green with it.
@task_item n=5 part=needs
The pty pump's restart path: the generation guard on which output copy ended and the reap_within and drain time limits each get one killing test per survivor class, with its remove-the-guard control; a test that needs a delay reaches the pump through the existing pump delay test seam under the fake-agent feature, never a new release-visible seam.
@task_item n=6 part=needs
The R2 open script's preconditions: it refuses before any start on a locked desktop and on a scratch dir that is not fresh, as its sibling R scripts do; the check that proves it runs no live arm (a rehearsal or a stub of hyprctl), sends nothing to the compositor and opens no window.
@task_item n=7 part=needs
Where moved units may live: the split-out restart code stays inside the viola-pty sync crate with no tokio; the moved window and trust steps and the shared prologue and refused() stay in the root crate's command code, exposing no new verb, flag, environment read, seam or channel method (none of which would come without a log entry).
@task_item n=8 part=needs
The measure the splits are read by: the audit's own complexity ceiling and 800-line size measure, every moved and every remaining file under them; the duplication read no longer listing the refused() pair or the prologue copy.
@task_item n=9 part=needs
The chunk's bounds on the operator's machine: no live start, no real claude run, no key sent, no live arm of any R script, and nothing read of the operator's real Claude home or viola home; the Claude home is written nothing.
@task_item n=10 part=needs
The new killing tests in viola-state and viola-pty stay at the unit layer of their own crates, raise no panic path outside the zero-panics gate's single exemption, and reach a moved unit through its new module path only by a re-pointed path or name.
@task_standing n=1
convention: each new test runs under a home of its own, never the operator's viola home or Claude home, and nothing of a test home enters git
home: secret-management · bootstrap-phases
@task_standing n=2
convention: each new test removes only the directories it created itself, following only a safe linked base for the e2e home
home: security-anti-patterns-never-do-these · security-anti-patterns
@task_standing n=3
convention: each test seam a new test uses sits only under the fake-agent feature and is compiled out of the release build
home: security-anti-patterns · input-validation
@task_standing n=4
convention: no new test runs the real claude CLI, and CI keeps holding no secrets
home: threat-model-summary
@task_item n=1 part=not_needed
The MCP tool, MCP stdin and MCP isError contracts: the chunk changes no MCP tool, input or result.
@task_item n=2 part=not_needed
The wrapper channel methods, channel frames params and channel error codes: the chunk adds no method or frame and changes no reply.
@task_item n=3 part=not_needed
The hook stdin, hook-decision detail and hook capture rows: the chunk touches no hook path.
@task_item n=4 part=not_needed
The paste text and dialog free text character sets, bracketed paste and send refusals: no typed text or send path changes.
@task_item n=5 part=not_needed
The config.json keys and config handling rows: the chunk adds and reads no config key.
@task_item n=6 part=not_needed
The board and viola list rows (board re-parse rules, whole-board refusal, terminal colour facts): viola list is not touched.
@task_item n=7 part=not_needed
The release, dist, attestation and self-update rows: no release stage or release workflow is touched.
@task_item n=8 part=not_needed
The rmcp feature set and banned features: no rmcp use or feature changes.
