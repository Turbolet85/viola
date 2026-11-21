@task_prose part=chunk
Turns two claude CLI behaviours the product already relies on without a capability-ledger row into relied ledger rows in viola-agent-claude, each with a viola verify probe.
Row one is the harness prompt prefixes: the compiled raw-prefix set that files a prompt as a harness turn, which never moves the wheel. Its probe has a second session send into a verify child, and the prompt's raw start is read to find the harness-origin prefix.
Row two is the R8 identity floor: the eleven measured CLAUDE* parent-identity environment names always stripped from the child. Its probe captures env names, never values.
The new rows are counted in a viola verify stamp from a live run in the live rig's scratch home only. The fake agent replays the new probes in CI, and the facts the directive measured (host CLI version, 27 relied rows, the standing stamps and fixture set) are re-read from the tree.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the capability ledger of the Claude adapter, with its relied rows, the verify runs and probes behind them, and the stamp's relied-id set — edits: the relied-row list, the stamp's relied-id set, the verify runs and child set for the two new probes
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases.rs mark=edits
what: the capability ledger's tests over its rows, runs, checks and stamps
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs
what: the hook side of the Claude adapter holding the harness-prefix literals (the compiled raw-start prefixes that file a prompt as a harness turn) and the hidden hook capture arm with its closed probe ids
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs
what: the Claude adapter's child-environment boundary holding IDENTITY_FLOOR, the eleven CLAUDE* parent-identity names stripped from the claude child
@task_item n=1 part=needs
Two new relied capability-ledger rows, each with a viola verify probe, under the universal invariant that every dependence on an undocumented claude behaviour is a ledger row with a verify probe. The stamp's relied-id set and the relied-row count grow with them.
@task_item n=2 part=needs
The harness prompt prefixes: the four raw-start literals compiled into the product, harness prompts filed as not human, and a harness turn that never moves the wheel. The probe reads a prompt's raw start against these literals only.
@task_item n=3 part=needs
The R8 identity floor: the eleven-name floor stays eleven names. Parent identity variables and inherited CLAUDE credentials are stripped from the child, and the strip reads names only.
@task_item n=4 part=needs
Env names never values: the identity-floor probe may capture variable names only. No CLAUDE value is ever serialised, logged, stamped, written to a fixture or shown on a screen, and verification output carries no identity value. A failing test proves that a captured value is refused.
@task_item n=5 part=needs
verify's runs and child set: the new probe child is spawned directly, verify never answers or types into a dialog, and it reads the PTY screen only by signatures against compiled literals, with bounded output caught.
@task_item n=6 part=needs
The second session sending into a verify child: whatever process sends (a viola endpoint, the channel or a paste, which is P3's call) sends only synthetic probe prompts. Probe effects stay in the probe's own directory, and probe directories and captures are transient and removed.
@task_item n=7 part=needs
The hidden hook capture arm: it answers only from closed probe ids, so any new probe id joins that closed set. The capture arms stay isolated from the product paths.
@task_item n=8 part=needs
verify --record and its scrub: recorded fixtures are scrubbed or refused whole under the unclean-fixture refusal codes (absolute-path, home-path, username, email), and the refusal line keeps its form. A captured env name that itself carries private text must be handled by the scrub.
@task_item n=9 part=needs
The recorded fixture set under the per-version claude fixtures folder (2.1.328, 2.1.329 or both, left to P3 and the founder): only signature rows are kept, and fixtures are reviewed before commit.
@task_item n=10 part=needs
Stamped live: only viola verify writes the stamps file, stamps are checked under lock after the strict-modes check, and the first screen of a stamp is kept in closed words.
@task_item n=11 part=needs
The live rig's verify arm: the child is isolated from the user's config and seeded from bundle logic only, the credentials copy is transient and unrecorded, and the kept stamp is owner-only and seeded to the rig's own home.
@task_item n=12 part=needs
The live home and the operator's real Claude config stay untouched: viola writes nothing under the Claude home, this phase reads none of it, and the live home's stamps, config.json and the pinned product stay with the re-pin step.
@task_item n=13 part=needs
Fake-agent replay of the new probes in CI: CI holds no secrets and runs no real claude CLI. Any new test seam lives only under the fake-agent feature and is compiled out of the release build.
@task_item n=14 part=needs
Env reads and the env keep-list: config's claude_env_keep cannot disable the identity-floor strip, no config, env var or flag switches off a control, and environment variables are not a configuration channel.
@task_item n=15 part=needs
The process-start claude-child log line's env_kept and env_stripped_known fields carry names and counts only. Any new env read or seam the probe adds needs its log entry.
@task_item n=16 part=needs
The gate that reads the stamp's relied-id set: the unverified-CLI version gate and dialog answers that need a verify stamp read the grown relied set through the strict-modes stamps read.
@task_item n=17 part=needs
verify's residual rows for the new probes: real-CLI transcripts, trusted runs and the pre-trusted folder that verify needs. Any residual a new probe leaves is accepted as measured or named.
@task_item n=18 part=needs
The CI artifact canary scan's claude-stripped class: no stripped CLAUDE value may reach an uploaded artifact, and the scan never echoes a match.
@task_item n=19 part=needs
Error and refusal output from the new probes interpolates nothing: no paths, pids, ids, env values or upstream text, and full detail goes to diagnostics only.
@task_standing n=1
convention: each new recorded fixture is scrubbed or refused whole, keeps only signature rows and is reviewed before commit
home: data-protection
@task_standing n=2
convention: each new test uses its own home, never the operator's, and holds no secret
home: secret-management
@task_standing n=3
convention: each new test removes only the directories it created, and each new test seam stays under the fake-agent feature
home: security-anti-patterns
@task_standing n=4
convention: each new log line carries no CLAUDE value, path or upstream text, and goes to owner-only diagnostics
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=5
convention: each new environment read or seam gets its log entry
home: security-anti-patterns-never-do-these
@task_item n=1 part=not_needed
The wrapper channel's methods and frames (link, unlink, pause, release, wait, last, answer), their params and JSON-RPC error codes: the chunk adds no channel method or frame shape.
@task_item n=2 part=not_needed
The MCP server's tools, structuredContent and isError documents: the chunk adds or changes no MCP tool.
@task_item n=3 part=not_needed
viola revive's arguments, --json codes and refusal words, and the recorded cwd it spawns in: revive is not reached.
@task_item n=4 part=not_needed
The Hyprland recorder, hyprctl reads and their reply handling: not reached.
@task_item n=5 part=not_needed
Event log rotation, growth and link lines in events.ndjson: the chunk writes no new event line.
@task_item n=6 part=not_needed
viola list's board scan and its terminal colour facts: not reached.
@task_item n=7 part=not_needed
Plugin install, the marketplace tree and its live install rounds: not reached.
@task_item n=8 part=not_needed
Release, signing, self-update and deploy rows: the chunk ships no release.
@task_item n=9 part=not_needed
Absences of hosting, a public listener, a database, webhooks, file upload, user accounts and remote access: the chunk adds none of these.
