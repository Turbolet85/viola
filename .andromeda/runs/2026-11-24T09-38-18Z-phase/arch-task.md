@task_prose part=chunk
Adds a CLI verb `viola adopt <session-id> --name <n> [--parent <p>]` that starts a viola wrapper whose claude child resumes a plain claude session, so it stands on the board as a named instance with its lineage and is driven like any wrapped session.
It is built on revive's start path: the `--resume <id>` child argv, the `claude agents --json` live-elsewhere preflight and its closed session-live refusal, and the start shared with run.
Under the conservative arm it never ends or signals a process it did not start and never takes over a running process's terminal: it resumes only a session live nowhere and refuses a live one with a hint to end it by hand.
It also takes the folded CI item: it measures what the two env-capture security-negative test cases wait on and cost locally and under coverage, and gives them a margin by their own test class or reduced work, never by raising the kill line for every test.
@task_file n=1 path=src/cmd/revive.rs
what: the revive command — the resume start path adopt is built on: the resume argv through the agent crate, the claude agents live-session preflight and its session-live refusal, the start shared with run
@task_file n=2 path=.config/nextest.toml mark=edits
what: the test runner configuration — per-test kill line (slow-timeout) and test classes for cases that wait by design — edits: the class or timeout override covering the two env-capture security-negative cases
@task_item n=1 part=needs
The CLI verb surface: a new `adopt` verb in the verb table and help rows, its clap arguments (a positional session id, `--name`, optional `--parent`), the flag and subcommand style, and a new entry in the command-name register.
@task_item n=2 part=needs
The claude session id shape: the id is checked against the closed session-id shape before any home read beyond the strict-modes ones, and a malformed id is refused with a closed code word.
@task_item n=3 part=needs
The resume crossing: the child is started with `--resume <id>` built by the agent crate's resume-args function, the claude flag literals staying in the agent crate, the logged session id carried on the child's command line, and the child argv's plugin-dir and launch-pair flags as for revive.
@task_item n=4 part=needs
The live-elsewhere refusal: a session that the claude agents listing reports live is refused with the closed session-live refusal before any child is spawned, as revive refuses a session a live claude holds; what adopt does when the agents read degrades or fails is defined, not assumed.
@task_item n=5 part=needs
No process taken over: adopt never ends, signals or attaches to a process it did not start, never takes over another process's terminal, and starts only its own wrapper and claude child.
@task_item n=6 part=needs
The shared start path with run: strict-modes check before any log opens, the socket-directory check, the pinned exe copy and the bytes it checked, inherited CLAUDE names stripped from the child's environment, the child cwd set explicitly, the child program resolved by viola, and the child spawned last in the start order.
@task_item n=7 part=needs
Held names and repeats: a start refuses a held name and never starts over a live or stale instance; adopt onto a name already held and a second adopt of the same session id each end in a defined closed outcome.
@task_item n=8 part=needs
The instance snapshot written at adopt: name as the instance key, agent_session_id, cwd, launch pair, lineage, endpoint, pid and started_at, written through the one atomic snapshot writer in the versioned envelope.
@task_item n=9 part=needs
Lineage: the operator-given parent lands as the instance's lineage label and the lineage event the board reads (list item parent); lineage stays a label only, with the parent-identity floor where it applies.
@task_item n=10 part=needs
The source of cwd, model and effort where no snapshot ever recorded them: the candidates are the config defaults (default_model, default_effort), the compiled default launch pair, and the spawn directory rule; a reading from another process is a crossing to be named.
@task_item n=11 part=needs
Exit codes and documents: the CLI exit-code table, refusal words from the closed refusal set, the `--json` error and refusal documents in revive's shape, the human stderr refusal line and its hint, and no JSON document once the child has held the terminal; the command stays silent at the internal-error catch site as run and revive do.
@task_item n=12 part=needs
Events and logs: the launch and lineage events the wrapper appends, the process-start line of the claude child (env kept and stripped), and codes-only home-level process logs and role diagnostics files for the new command.
@task_item n=13 part=needs
The adopted instance is then driven like any wrapped session: it is listed on the board as wrapped, with liveness by heartbeat and pid plus start time, and reachable through its recorded endpoint by the existing frame verbs.
@task_item n=14 part=needs
The terminal question: whether adopt runs in the terminal it is typed in (as revive) or opens a window (as open and up) — the window rows and the Hyprland seam bind only on the arm P3 takes; run writes only the child's output to the terminal.
@task_item n=15 part=needs
The MCP surface: whether adopt is an MCP tool is a P3 fork (conservative arm CLI-only); the MCP tools list stays held to its recorded exchange unless that fork widens it.
@task_item n=16 part=needs
The capability ledger and version gate on start: adopt's start reads the stamps strictly and degrades as run does on an unverified CLI.
@task_item n=17 part=needs
Live readings gated: no live claude in CI, real-CLI probes local only, and every live reading of adopt (it ends an operator session and spends real claude starts) planned GATED on the operator's grant; acceptance under the fake agent through its test seams.
@task_item n=18 part=needs
The folded CI item: the two env-capture security-negative cases measured per case against the nextest kill line, locally and under coverage, and given a margin by their own test class or reduced work, with root test waits sharing one bound; the global kill line is never raised.
@task_item n=19 part=needs
Every refusal and outcome word adopt adds is a new member of a closed set in kebab-case on the wire, its document fields snake_case with absent optional fields omitted.
@task_item n=20 part=needs
Security of the resume crossing: the session id handed to the child is checked before it reaches the argv, the home is read under the strict read rule, and no content-bearing detail reaches a process log.
@task_standing n=1
convention: every new test runs against the fake agent in its own isolated viola home, its homes cleaned by owner record, with no live claude
home: established-decisions
@task_standing n=2
convention: every new test-harness knob carries the AGENT_RUN_ prefix and every fake-agent seam is a test seam variable, never a configuration channel
home: conventions
@task_standing n=3
convention: every new log line is a codes-only process-log line emitted through the one event emitter, with no print or tracing level macros
home: occupied-resources · build-system
@task_standing n=4
convention: every new error is a thiserror variant of its crate's one enum with a message that interpolates nothing, anyhow only at the binary edge, error chains only in the detail file
home: cross-cutting-patterns
@task_standing n=5
convention: every new module is wired into the tree (no orphan modules) and passes formatting and clippy with warnings denied
home: build-system
@task_standing n=6
convention: every new end-to-end test leaves zero panic lines, and an OS-gated test names its reason
home: project-directory-structure · cross-cutting-patterns
@task_item n=1 part=not_needed
The send pipeline: bracketed-paste send, send confirmation windows, paste bound and cut mark, send refusals and typed-text normalisation — adopt types nothing into the session.
@task_item n=2 part=not_needed
Dialog handling: the dialog hold, answer responses, hook tiers and the hook-to-kind map — adopt adds no hook and answers no dialog.
@task_item n=3 part=not_needed
The wheel: byte-source claim, release, human interrupt and wheel events — the adopted instance's wheel starts as any wrapper's and adopt changes none of it.
@task_item n=4 part=not_needed
Driver-to-driven links, pause, link and unlink, and cross-session sends by inbox socket — adopt records lineage, not a link.
@task_item n=5 part=not_needed
The wrapper channel protocol: JSON-RPC framing, channel methods, fault codes and protocol-version bumps — adopt adds no channel method.
@task_item n=6 part=not_needed
viola verify: its probes, trust record writer and its code words, ledger stamps writing and recorded fixtures — adopt only reads the stamps.
@task_item n=7 part=not_needed
Plugin install and the marketplace tree, and the plugin's MCP manifest — adopt installs nothing.
@task_item n=8 part=not_needed
The fleet console, model and effort switching, and up's reboot reading — out of this chunk's scope.
@task_item n=9 part=not_needed
Fuzz workspace, release tooling, mutation runs, weekly advisories and fuzzing, and the deploy, hosting and container absences — the chunk adds no fuzz target, release step or deployment.
@task_item n=10 part=not_needed
The screen model's prompt classification, harness-prompt prefixes and local-command confirmation rows — adopt reads no screen content and sends no command.
