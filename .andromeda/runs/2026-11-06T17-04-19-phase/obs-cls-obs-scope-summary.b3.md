@answer n=1 entity=obs-scope-summary#16.3 v=no
why: load-time config parse logged through viola-state; the chunk edits build manifests (Cargo.toml, deny.toml), not a parsed runtime config
@answer n=2 entity=obs-scope-summary#17.1 v=no
why: source citation line for the CI/CD platform row; it states no rule the chunk must honour
@answer n=3 entity=obs-scope-summary#17.2 v=no
why: CI/CD platform classed not-instrumentable; the chunk's CI gate re-runs are its own work, and this classification constrains no code or test it adds
@answer n=4 entity=obs-scope-summary#17.3 v=no
why: fixtures are scrub-required data excluded from obs sections; the chunk adds no fixture, and synthetic inputs are already covered by S5
@answer n=5 entity=obs-scope-summary#18.1 v=yes
why: each process's panic hook writes one line, and the task's zero-panic-line error budget (G2) counts those lines, so the hook must survive the chunk
@answer n=6 entity=obs-scope-summary#18.2 v=yes
why: plain CLI output is static fixed-column text with no prompt, TUI or pager; plain_drive and tests/cli_output_plain.rs are the chunk's edited plain-output units
@answer n=7 entity=obs-scope-summary#18.3 v=yes
why: CLI verb sink rules (stdout carries results only, stderr only fixed-message lines, verbs log as cli to their instance file) rule over plain_drive in src/cmd/mod.rs, which the chunk edits
@answer n=8 entity=obs-scope-summary#18.4 v=no
why: service.name, CARGO_PKG_VERSION-based version and the process/instance identity are unchanged; the chunk bumps a dependency, not the crate version or ViolaName
@answer n=9 entity=obs-scope-summary#18.5 v=yes
why: stderr fault and refusal lines are fixed strings with no paths or upstream text, which plain_drive's plain-mode output must keep; the internal_error_document unit is only a re-check
@answer n=10 entity=obs-scope-summary#19.1 v=yes
why: run's tracing spans cover the viola-pty seam operations and must be sync with no Tokio, so the portable-pty bump must keep the seam instrumented and the graph Tokio-free (task item 5)
@answer n=11 entity=obs-scope-summary#19.2 v=no
why: frontend N/A with the terminal carrying the child's screen; a description, not a rule the chunk's code or tests must act on
@answer n=12 entity=obs-scope-summary#19.3 v=yes
why: task item 10 names run writing nothing to the terminal and logging to diagnostics/run-<name>.ndjson, the live-session evidence file; the scope names this sink
@answer n=13 entity=obs-scope-summary#19.4 v=no
why: run's service identity (process="run", instance) is unchanged, and no part of the chunk or its scope names it
@answer n=14 entity=obs-scope-summary#19.5 v=yes
why: the default panic hook must be replaced before the child spawns, and child process-exit comes from the handle wait, not reader EOF; the portable-pty bump and task item 6 rest on that exit path
@answer n=15 entity=obs-scope-summary#20.1 v=yes
why: the dialog hook emits hook-invoked and hook-decision lines and one ERROR panic line; task item 12's evidence is the hook-invoked line written after the hook.dialog response
@answer n=16 entity=obs-scope-summary#20.2 v=no
why: frontend N/A for the hook path; no frontend hook is added or read
@answer n=17 entity=obs-scope-summary#20.3 v=yes
why: the hook sink is diagnostics/hook-<name>.ndjson, file only, with stderr forbidden and stdout limited to the decision body; task item 12's dialog evidence is written to this sink
@answer n=18 entity=obs-scope-summary#20.4 v=no
why: hook service identity and the silent no-op when no instance resolves are unchanged; the live dialog session only uses the hook, and the scope does not name these clauses
@answer n=19 entity=obs-scope-summary#20.5 v=no
why: the per-hook latency budget, append atomicity and run --perf hook rows concern the hook logger and instance check, which the chunk does not touch; perf.rs is edited only by EXE_SUFFIX joins
@answer n=20 entity=obs-scope-summary#21.1 v=no
why: channel corr and trace-context propagation; the chunk adds no channel method, verb or cross-surface call (task not-needed item 3)
@answer n=21 entity=obs-scope-summary#21.2 v=no
why: frontend N/A for the channel; no frontend hook is added
@answer n=22 entity=obs-scope-summary#21.3 v=no
why: per-side log file of the channel; the chunk's live evidence files are the run and hook sinks (19.3, 20.3), and the channel sink is untouched
@answer n=23 entity=obs-scope-summary#21.4 v=no
why: each side's process identity and the peer's params.sender are unchanged; the chunk changes no channel params
@answer n=24 entity=obs-scope-summary#21.5 v=yes
why: peer_admitted's refused-stream drop with no line is the mutation audit's one survivor by construction, which task item 15 requires to keep its dated note and untestable zone in host.rs
@answer n=25 entity=obs-scope-summary#22.1 v=no
why: MCP tool-call spans around the viola-channel client; the chunk edits no MCP code, and the Tokio-free rule binds only the sync crates
