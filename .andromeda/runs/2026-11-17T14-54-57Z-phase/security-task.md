@task_prose part=chunk
Takes the first live readings on the committed standing live rig, each written down as closed words in the chunk's evidence.
R3: whether a fired scheduled wake's prompt-submit payload carries the optional source field, a live turn-ended scheduled_wakes count of 1, and the hook-latency window before a send's paste.
R4: a live viola verify --record whose record home holds no path component named home; plus readings of the external-imports modal and of R2's unexplained holds-text input box.
One CI-configuration fix: the nextest ci 20 s kill override gains the viola-e2e test binaries that drive a stamping verify through boot, with the slow lifecycle case bounded by a shorter readiness deadline or a bound of its own, and its wall measured again.
@task_file n=1 path=.config/nextest.toml mark=edits
what: the nextest test-runner configuration whose ci profile holds the 20 s kill line for verify-driven test binaries — edits: the ci profile's second override list (the binary set under the 20 s kill), and a bound of its own for the harness_lifecycle binary if that fork is taken
@task_file n=2 path=crates/viola-e2e/src/bin/viola-live-rig.rs mark=edits
what: the standing live rig's entry point, the test-side tool that starts the real claude CLI in an isolated scratch root and keys only into its own pty — edits: its arms, extended only as far as a reading needs (a wake arm, a latency arm, an imports-modal arm, a verify-record arm)
@task_file n=3 path=crates/viola-e2e/src/rig/ mark=edits
what: the standing live rig's modules: the scratch root, the private config dir, the transient credentials copy, the screen kept locally and the closed-word record — edits: the arms a reading needs, nothing of the isolation as built
@task_item n=1 part=needs
The live rig's isolation stands as built for every reading: a scratch root outside every repository, a private config dir for the child, the credentials copy guarded by the token-window read and removed on every exit path and never recorded, the live session screen kept locally, and only closed words in a record.
@task_item n=2 part=needs
The rig keys only into its own pty; no key is typed into a CLI-native dialog or modal (the external-imports modal included) without the founder's grant, and none is granted; a reading the rig cannot take as built names what the rig lacks.
@task_item n=3 part=needs
R3a captures a fired scheduled wake's raw prompt-submit hook payload shape: the hook payload is content, parsed tolerantly, and the reading is written down as closed words only — field names and closed values, never the prompt text or transcript content.
@task_item n=4 part=needs
R3b reads turn-ended's scheduled_wakes count live as 1 (one start, two sends, the wrapper stopped before the wake is due): the count is carried as a count only.
@task_item n=5 part=needs
R3c measures the hook-latency window between a CLI-started turn and its UserPromptSubmit reaching the wrapper before a send's paste: the send stays one bracketed paste, and the human-typing and flight-lock refusals stand as they are.
@task_item n=6 part=needs
R4 runs a live viola verify --record against the real claude CLI, locally only: only verify writes the stamps file, stamps are checked under the strict-modes check and under lock, and verify never answers or types into a dialog, so it needs a prepared trusted folder.
@task_item n=7 part=needs
R4's record home comes from a helper like the tests' record home helper and holds no path component named home; nothing is written under the Claude home or the user's own Claude config, and the test home stays out of git.
@task_item n=8 part=needs
If R4's own record home is the reading and fixtures are recorded, they are scrubbed or refused whole (absolute path, home path, username, email) and reviewed before commit; screen fixtures keep only signature rows.
@task_item n=9 part=needs
Which claude a start runs: resolved from PATH (2.1.326, unstamped) or the pinned 2.1.325 install; each reading names its binary, and a reading on an unstamped version and R4's stamping of it are counted apart under the unverified-CLI version gate.
@task_item n=10 part=needs
The external-imports modal reading: a cwd whose CLAUDE.md imports a file outside the project, with no approval in the rig's private config; its shape, what closes it and what the start reads if it does not show are read as closed words; verify's pre-trusted-folder narrowing is what it closes.
@task_item n=11 part=needs
R2's holds-text input box: input-box readiness is read against compiled literals only; the reading names what the screen showed without keeping the screen.
@task_item n=12 part=needs
The live child's environment: inherited CLAUDE credentials and parent identity variables are stripped from the child, by name only, and the rig's child runs isolated from the user's config.
@task_item n=13 part=needs
Live prompts are synthetic, and live session transcripts and probe directories the readings leave are owner-only and removed; nothing of them reaches a record beyond closed words.
@task_item n=14 part=needs
The nextest kill-line fix: cli and harness_tied join the ci 20 s kill override as they are; harness_lifecycle either has its case's boot readiness deadline shortened or takes a bound of its own, with the measured wall after the change, never by letting the kill line end that case.
@task_item n=15 part=needs
A shortened readiness deadline for the lifecycle case is a test-side parameter only: no product behaviour changes, and any seam it uses stays under the fake-agent feature and compiled out of release.
@task_item n=16 part=needs
CI stays without secrets and without the real claude CLI: the live readings run locally only, never as a CI job.
@task_item n=17 part=needs
This session sets no scheduled wake or timer of its own; R3's wake is set only inside the rig's own started session.
@task_standing n=1
convention: every new test or rig arm uses its own home and removes only the directories it created, following only a safe linked base for the e2e home
home: secret-management · security-anti-patterns-never-do-these
@task_standing n=2
convention: every new test seam lives only under the fake-agent feature and is compiled out of the release build
home: security-anti-patterns · input-validation
@task_standing n=3
convention: every new environment read or seam is given a log entry
home: security-anti-patterns-never-do-these
@task_standing n=4
convention: every recorded artefact a reading commits is held to fixture hygiene, scrubbed or refused whole and reviewed before commit
home: data-protection
@task_item n=1 part=not_needed
The MCP server's tools, stdin line cap and isError results: no reading drives viola mcp.
@task_item n=2 part=not_needed
The wrapper channel's frames, methods, error codes and the clients' server and peer-euid verification: no channel method or refusal is added or changed.
@task_item n=3 part=not_needed
revive and its --json codes, link and unlink, list and the board's read rule: none of these verbs is touched.
@task_item n=4 part=not_needed
Ledger rows, a verify probe or the classification of a wake prompt: that is a later entry's, not built here.
@task_item n=5 part=not_needed
Mutation testing at the epoch audit: no mutation run in this chunk.
@task_item n=6 part=not_needed
Fuzz scope, the fuzz workspace and scheduled fuzzing: no parser surface is added.
@task_item n=7 part=not_needed
The release, signing and deploy stage, dist attestations and release-workflow caching: nothing is released.
@task_item n=8 part=not_needed
The Hyprland recorder and terminal colour facts: no reading reaches them.
@task_item n=9 part=not_needed
config.json keys and bad-value handling: no configuration key is added or read differently.
@task_item n=10 part=not_needed
Plugin install and the marketplace tree, the pinned binary placement and its re-hash: no reading installs a plugin or places a pinned copy.
