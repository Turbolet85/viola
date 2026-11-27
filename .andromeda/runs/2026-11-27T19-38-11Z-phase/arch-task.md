@task_prose part=chunk
A corrective chunk with no product behaviour added: no flag, no event kind, no refusal code, no master rule.
It makes the root crate's mutation baseline green by moving the test that runs a whole verify into the verify class's longer kill in the mutants test profile, then runs the one announced in-diff mutation run and records its score as an addendum to the audit ledger.
It adds killing tests for the nine viola-pty restart-path survivors and the three viola-state survivors, and adds the locked-desktop and fresh-scratch preconditions to the r2 open script.
It makes three pure moves: one shared start prologue for revive and adopt, one shared refusal helper for open and up, and three splits (the up command's upped function, the pty pump's restart code, the run command's window and trust steps). All layers stay green and each file must be under the audit's size and complexity ceilings.
@task_file n=1 path=.config/nextest.toml mark=edits
what: the nextest configuration: the mutants profile and its per-test kill overrides — edits: the verify_window_ override's test list (gains the whole-verify test by name; the general 15 s limit unchanged)
@task_file n=2 path=tests/cli_answer.rs mark=re-reads
what: the root crate's mutation baseline check: the dialog-rows stamp test that runs its own whole viola verify, passing under the mutants profile with no TIMEOUT
@task_file n=3 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY output pump of viola-pty: its restart path, the generation guard on which output copy ended, and the time limits of reap_within and drain — edits: the restart code split out into its own module, killing tests for the nine survivors
@task_file n=4 path=crates/viola-state/src/liveness.rs mark=edits
what: session liveness in viola-state: the ancestor walk ancestors(pid, max) and its chain-length bound of 8 ancestor pids — edits: killing tests for its three survivors
@task_file n=5 path=src/cmd/revive.rs mark=edits
what: the viola revive verb: its start prologue, the strict-modes check before any log opens included — edits: the prologue, moved into one shared function
@task_file n=6 path=src/cmd/adopt.rs mark=edits
what: the viola adopt verb: its copy of revive's start prologue — edits: the prologue copy, replaced by a call to the shared function
@task_file n=7 path=src/cmd/up.rs mark=edits
what: the viola up verb: the upped function and its refused() refusal helper — edits: upped, split under the complexity ceiling; refused(), replaced by the one shared helper
@task_file n=8 path=src/cmd/open.rs mark=edits
what: the viola open verb: its refused() refusal helper, a byte-identical copy of up's — edits: refused(), made the one shared helper both verbs call
@task_file n=9 path=src/cmd/run.rs mark=edits
what: the viola run verb's start path on the boot path: the window step and the trust step — edits: the window and trust steps, split out of the file
@task_file n=10 path=scripts/r2-open.sh mark=edits
what: the r2 open live-rehearsal script and the preconditions it checks before starting anything — edits: its preconditions (desktop not locked, scratch dir fresh)
@task_item n=1 part=needs
The mutants nextest profile and its kill limits: the test that runs a whole viola verify joins the verify class's 20 s × 4 override by name, plus any other test that runs a whole verify (found by what it runs, not its name). The general 15 s × 3 limit stays unchanged.
@task_item n=2 part=needs
The one mutation run at the epoch boundary: the root crate's in-diff mutants from the recorded base, using the recorded flags (copy-target off, five jobs, a kill deadline). It uses its own relative target dir, and its TMPDIR is a short-named dir under the NOCOW mutation scratch directory. It runs once, behind an announced stop the overseer answers. The result is an addendum to the audit ledger record giving the caught, missed, timeout and unviable counts.
@task_item n=3 part=needs
The viola-pty restart path's guards: the generation guard on which output copy ended, and the time bounds in reap_within and drain. Each survivor class gets a killing test, with a control that goes red when the guard is removed and green when it is restored.
@task_item n=4 part=needs
The viola-state ancestor walk's bound, a chain capped at the security plan's 8 ancestor pids, and the other two liveness survivors. Each gets a killing test with its remove-the-guard control.
@task_item n=5 part=needs
The restructure invariant: the start prologue revive and adopt share (with the strict-modes check still running before any log opens), the open and up refusal helper (with its --json refusal shape and human refusal line), up's upped function, the pty pump's restart code, and run's window and trust steps all move without any change in what they do. The existing layers hold this, green, with no test edited beyond a re-pointed path or name.
@task_item n=6 part=needs
The strict-modes check runs before any log opens in the shared start prologue, in that one place: revive and adopt both reach it through the shared function, never a copy.
@task_item n=7 part=needs
The measure the splits are read by: the audit's complexity ceiling and its 800-line size ceiling, applied by the audit's own measure to each moved file and each remaining file.
@task_item n=8 part=needs
Where moved units may live: each new module is declared in its parent (no orphan modules). The pty restart module stays inside viola-pty, which depends on no viola crate and knows no agent. The run command's window and trust steps stay in the root binary and reach the compositor only through the viola-hyprland seam.
@task_item n=9 part=needs
The one shared refusal helper writes through the root binary's single human writer and keeps the CLI --json refusal document and the human stderr refusal line as they are for both open and up.
@task_item n=10 part=needs
The run command is on the boot path: the plan keeps the smoke trio over the run start order after the window and trust steps move.
@task_item n=11 part=needs
The r2 open script's new preconditions: before it starts anything, it refuses on a locked desktop (read through hyprctl locked) and on a scratch dir that is not fresh, matching its sibling scripts. The check that proves this runs no live arm, only a rehearsal or a stubbed hyprctl.
@task_item n=12 part=needs
How the chunk is proved: no mutation run beyond the one addendum, no witness run, no live start, no window opened, nothing sent to the compositor, no key sent, no real claude run, no live arm of any r-script, and nothing of the operator's real claude or viola home read.
@task_item n=13 part=needs
Killing tests reach moved units only through re-pointed paths or names. New tests for the pump restart path and the liveness walk exercise the guard directly, and their remove-the-guard control is a one-off demonstration, never a committed mutant.
@task_item n=14 part=needs
The discretionary items: the UNRESOLVED markers in the masters' histories are resolved or re-dated through the letters' own door and never deleted by hand, and the project's boot paths are declared once in the test plan. Both are expected wrap amendments to the masters or their sidecars, never a plan step's write.
@task_standing n=1
convention: each new test runs in its own isolated viola home against the fake agent, never a live claude
home: established-decisions
@task_standing n=2
convention: each new test home and test channel socket is owned and cleaned by its owner process under the short /tmp root, its evidence surviving a killed test
home: occupied-resources
@task_standing n=3
convention: each new root-crate test's waits share the one root wait bound, and any frozen wrapper is frozen through the one SIGSTOP guard
home: occupied-resources
@task_standing n=4
convention: each new OS-gated test names its reason, and no test gate names another OS
home: cross-cutting-patterns
@task_standing n=5
convention: each new file passes formatting and clippy with warnings denied, with no print or dbg macros
home: build-system
@task_standing n=6
convention: each new snapshot assertion runs in check mode only, and end-to-end runs log zero panic lines
home: project-directory-structure
@task_standing n=7
convention: each new test runs in CI on ubuntu against the fake agent, and real claude probes stay local
home: ci-cd-approach
@task_item n=1 part=not_needed
MCP tool rows, MCP tool results and MCP exit codes: the chunk touches no MCP surface.
@task_item n=2 part=not_needed
The wrapper channel's methods, frames, params, fault codes and response details: no frame or method changes.
@task_item n=3 part=not_needed
Hook tiers, the hook-to-kind map, the hook contract and hook diagnostics: no hook behaviour changes.
@task_item n=4 part=not_needed
Normalised event kinds, per-kind event data and wrapper event appends: the chunk adds no event kind and changes no event line.
@task_item n=5 part=not_needed
Send semantics (bracketed paste, the confirm window, the paste bound, the readiness gate, send refusals and results): no send code changes.
@task_item n=6 part=not_needed
Wheel rows (byte-source claim, release, human interrupt): the wheel is untouched.
@task_item n=7 part=not_needed
Dialog hold, dialog answers and answer responses: no dialog behaviour changes.
@task_item n=8 part=not_needed
The board and list payload, the agents listing and list exit codes: no listing changes.
@task_item n=9 part=not_needed
config.json keys, configuration precedence and the colour environment: no configuration changes.
@task_item n=10 part=not_needed
Release tooling, dist, crates.io publishing, licences and the dependency-ban policy: no release or dependency changes.
@task_item n=11 part=not_needed
The fuzz workspace, its nightly toolchain and its corpus replay: no fuzz target changes.
@task_item n=12 part=not_needed
The planned terminal fleet console and the graphical, mobile and web absences: this chunk builds none of the console.
@task_item n=13 part=not_needed
Plugin install, the embedded plugin, the marketplace tree and the plugin's .mcp.json: the plugin is untouched.
