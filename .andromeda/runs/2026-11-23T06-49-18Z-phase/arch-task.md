@task_prose part=chunk
Every session viola starts names its model and its effort: a start (`viola run`, and the start arm of `viola revive`) hands its claude child a model and an effort on every spawn, the pair the operator gave, else viola's configured default pair.
The pair is recorded per instance as a closed set of values, beside the lineage record (the events log, the instance snapshot, the snapshot replay).
`viola revive` replays the recorded pair on its resume, so the resumed session runs on the pair its instance recorded.
It measures first, at P3, the claude CLI channel for model and effort (flag, environment name or settings key), what a resume does with model and effort today, and where and in what shape the pair is recorded; `open`, `up` and `adopt` do not exist and are not built here.
@task_item n=1 part=needs
The `viola run` and `viola revive` argument surface (clap): new arguments naming a model and an effort, held to the CLI flag and subcommand style and to the clap-parses-the-CLI rule, with a bad value refused as a usage error under the CLI exit codes.
@task_item n=2 part=needs
The claude child's argv and environment at spawn: the model and effort handed on every start and every revive start, beside the existing flags passed to the child (the caller's arguments, `--resume`, `--fork-session`, `--plugin-dir`).
@task_item n=3 part=needs
Claude flag literals live in the agent crate only (viola-agent-claude), which does no file I/O and keeps Claude-specific shapes isolated; the root bin consumes only normalised values from it.
@task_item n=4 part=needs
viola's configured default pair: if it lands as a `config.json` key, the key joins the config schema, a bad or unusable value falls back to the default, configuration precedence holds (operator-given pair over the configured default), and environment variables are no configuration channel.
@task_item n=5 part=needs
The pair recorded per instance as a closed set of values beside the lineage record: an event in the events log and a field of the instance snapshot, recovered by the snapshot replay from log-derived fields only; event kinds and their data added only in viola-core.
@task_item n=6 part=needs
The recorded shape of the pair follows the own-format rules: versioned-tolerant records instead of migrations, snake_case field names, kebab-case enum values on the wire, absent optional fields omitted on write, missing and null read the same, so an instance recorded before this chunk replays without a pair.
@task_item n=7 part=needs
The closed set of model and effort values as validated domain newtypes (nutype), their validation errors kept out of the crate error enums; an unparseable external value becomes unknown rather than failing.
@task_item n=8 part=needs
Revive resumes in place through the start path, reads the snapshot once and replays the recorded pair onto the resumed child, under the existing revive refusals; revive over a replayed snapshot keeps its existing rules.
@task_item n=9 part=needs
Every undocumented claude behaviour the chunk relies on (the model and effort channel, what a resume does with them) is a row of the per-behaviour CLI capability ledger with a verify probe; real CLI probes run only locally, never in CI.
@task_item n=10 part=needs
The chunk writes nothing to the user's own claude settings and passes no settings layer at start (the status-line interception absence: no --settings at start), keeping the later switch of a running session's pair able to leave the user's settings byte-identical.
@task_item n=11 part=needs
If the channel is an environment name, the child's environment strip binds it: inherited CLAUDE names are stripped by name only, the kept names follow `claude_env_keep`, and the process-start line of the claude child records what was kept and stripped.
@task_item n=12 part=needs
The child is spawned last in the start order and the start refuses a held name or a live instance as today; the pair is recorded before the child spawns so a start that fails leaves no pair for a session that never ran.
@task_item n=13 part=needs
`viola list`, `viola list --json` and the MCP `list` tool: whether the pair is shown as a list payload item field is the operator's fork; if shown, it is read caller-side from state without a wrapper.
@task_item n=14 part=needs
The fake agent stands in for claude in every test: it accepts the model and effort on its argv or environment after `--`, and its test seams carry the readings tests assert on.
@task_item n=15 part=needs
Mixed-version tolerance: a newer viola reads an instance recorded by an older one, and the snapshot envelope version and the protocol version bump only where their rules say.
@task_standing n=1
convention: each new test runs in its own viola home, removed by its owner process
home: established-decisions
@task_standing n=2
convention: each new test runs against the fake agent, never a live claude in CI
home: ci-cd-approach
@task_standing n=3
convention: each new log line is a codes-only process-log line emitted through obs_event!, with no print or tracing level macros
home: build-system
@task_standing n=4
convention: each new recorded fixture is scrubbed and hygiene-walked before it is kept
home: occupied-resources
@task_standing n=5
convention: each new OS-gated test names its reason and no test gate names another OS
home: cross-cutting-patterns
@task_standing n=6
convention: each new test counts toward the coverage floors the harness gate enforces
home: ci-cd-approach
@task_item n=1 part=not_needed
The wheel rows (byte-source claim, release, human interrupt, wheel in wrapper memory): the pair is chosen at spawn and the wheel is not touched.
@task_item n=2 part=not_needed
The send, paste and confirmation rows (bracketed paste, paste bound, send refusals, confirmation windows, local command echo): nothing is typed into the session to set the pair.
@task_item n=3 part=not_needed
The dialog hold, dialog hooks and answer rows (permission, plan, question answers, hook tiers): the chunk answers no dialog and adds no hook.
@task_item n=4 part=not_needed
The link and unlink rows (link recording, link set, link channel methods): the pair sits beside the lineage record but no link changes.
@task_item n=5 part=not_needed
The Hyprland window seam rows and the terminal fleet console: no window is spawned or placed.
@task_item n=6 part=not_needed
The plugin install and marketplace rows: the embedded plugin and its install are unchanged.
@task_item n=7 part=not_needed
The pinned binary copy rows (placement, re-hash, never replaced): the start pins its copy as it does today.
@task_item n=8 part=not_needed
The release and deployment rows (dist, release build contents, crates.io): nothing is released.
@task_item n=9 part=not_needed
The MCP frame tools send, wait, last and answer and the wrapper channel frame methods: no frame carries the pair.
