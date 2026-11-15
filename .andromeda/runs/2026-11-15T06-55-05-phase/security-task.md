@task_prose part=chunk
Builds the first live round on the trimmed product: test-only rigs that drive the real unmodified claude CLI headless on a plain pty the rig opens itself, each rehearsed first under the fake agent, then named operator steps fired only on the operator's word, each with its live-start count.
Proves one live wrapped start (matrix v2-03): the child's screen with no viola byte, the human's keys unchanged, the pinned copy run through a freshly written plugin by absolute path, the start order with no budget step, a live or stale name refused, the child's end detected, on a ledger green for the founder's account.
Reads a live dialog hold close to the 3600 s DIALOG_HOLD_MAX ceiling (matrix v2-02) so the founder can settle its PROVISIONAL mark; the ceiling is read, not changed, unless the founder's word moves it.
Re-records the MCP tool-list fixture through the test-only MCP tap so the list tool's description drops the usage budget, and records one box-only Screen fixture by a clean viola verify --record for the fake agent to replay in CI; none is hand-written.
Reads a wrapped session's status line beside an unwrapped one's on a real claude (live half of v2-05, evidence only), reading the user's status-line setting and writing nothing to the Claude settings.
@task_file n=1 path=fixtures/claude/2.1.302/Mcp.tools-list-1.json mark=edits
what: the recorded MCP tool-list fixture of the fixture class for the CLI version, recorded through the test-only MCP tap and scrubbed — edits: the list tool's description, re-recorded live
@task_file n=2 path=scripts/agent-run.sh mark=edits
what: the live-round rig script that drives a claude or fake-agent session on its own pty with a kill deadline
@task_file n=3 path=crates/viola-e2e/src/bin/viola-harness.rs mark=edits
what: the test-only viola-harness rig binary of the viola-e2e crate that runs the wrapped start and its fake rehearsal
@task_file n=4 path=crates/viola-e2e/src/bin/viola-mcp-tap.rs
what: the test-only MCP tap through which MCP fixtures are recorded from a live claude start
@task_item n=1 part=needs
Live real-claude sessions run only locally on an operator step, never in CI: CI holds no secrets and no real claude; the rigs' fake-agent rehearsals are what CI runs.
@task_item n=2 part=needs
Nothing written under the Claude home: no write to the Claude settings file or Claude's config dir, including the status-line reading, which only reads the user's setting.
@task_item n=3 part=needs
Tests use their own home: live homes are test homes on tmpfs under the e2e-home directory, kept out of git, with test-side removal only of dirs the test created and the e2e-home linked-base guard holding; nothing written under the operator's viola home and the pinned product binary and pinned folders untouched.
@task_item n=4 part=needs
Recorded fixture hygiene for the re-recorded MCP tool-list fixture and the new Screen fixture: recordings scrubbed or refused whole by verify's unclean-fixture refusal codes (absolute path, email, home path, username), Screen fixtures keep only signature rows, fixtures reviewed before commit, MCP fixtures recorded only through the test-only tap.
@task_item n=5 part=needs
viola verify --record on a test home writes the capability ledger's stamps for the CLI the round runs; only verify writes the stamps; verify never answers or types into a dialog, needs a pre-trusted folder, reads PTY output only by signatures against compiled literals, and its probe captures and dirs are transient and confined.
@task_item n=6 part=needs
The wrapped start's guarantees as read live: the pinned copy re-hashed and run by absolute path from a freshly rewritten plugin (code-bearing files rewritten each start), the pinned folder first on the child's PATH, inherited CLAUDE credentials and parent identity variables stripped from the child, the start collision check refusing an already-live or stale-heartbeat name.
@task_item n=7 part=needs
Human typing passes through unchanged and takes the wheel; the screen model feed stays isolated from passthrough so no viola byte reaches the child's screen.
@task_item n=8 part=needs
The rig typing into its own pty (arrow and Enter on a trust dialog) is a per-chunk founder word: rig input is test-side driving of the pty, never viola answering a dialog, and each rig states what it does without that word.
@task_item n=9 part=needs
The dialog hold value DIALOG_HOLD_MAX and config.json dialog_hold_s (60 to 3600 s) set only in config; the reading leaves it unchanged unless the founder's word on the PROVISIONAL mark moves it, and no config value disables a control.
@task_item n=10 part=needs
The MCP tool list is an affordance, not enforcement: the list tool's description text changes in the fixture only; the MCP tools contract's list item keeps its behaviour.
@task_item n=11 part=needs
Every live spawn has a kill deadline; child output read by the rigs is bounded and tolerant, and the CLI version is resolved from PATH or the pinned shim as the founder chooses.
@task_item n=12 part=needs
Test-only binaries and seams stay out of the release build: the rigs and taps live in the test-only viola-e2e crate, seams only under the fake-agent feature, compiled out of release.
@task_item n=13 part=needs
Evidence records in the chunk's work dir (one live-round record per operator step) and any CI upload hold no CLAUDE values, credentials or user content; uploads stay gated by the artifact canary scan.
@task_item n=14 part=needs
A guard test the chunk adds carries its remove-the-guard control through gate.py control, its case the test's name alone; no mutation run this chunk.
@task_standing n=1
convention: each new or re-recorded fixture is scrubbed, refused whole if unclean, keeps only signature rows for Screen fixtures, and is reviewed before commit
home: data-protection
@task_standing n=2
convention: each new test or rig uses its own test home, never the operator's viola or Claude home, and removes only what it created
home: secret-management
@task_standing n=3
convention: each new test-only binary or seam stays behind the test-only feature and out of the release build
home: security-anti-patterns
@task_standing n=4
convention: each new CI job or upload holds no secrets, runs no real claude, and is gated by the artifact canary scan
home: bootstrap-phases
@task_item n=1 part=not_needed
Channel protocol rows (wrapper channel methods, channel frames, JSON-RPC error codes, link and unlink) — the chunk changes no frame or method.
@task_item n=2 part=not_needed
Revive arguments and revive --json codes — the round runs no revive.
@task_item n=3 part=not_needed
Send, answer and paste text character-set and refusal rows — the chunk adds no send or answer path.
@task_item n=4 part=not_needed
Hyprland recorder and hyprctl rows — live sessions run headless with no compositor.
@task_item n=5 part=not_needed
Dependency pins, advisory floors, licence list and rmcp feature rows — the chunk adds no dependency.
@task_item n=6 part=not_needed
viola list and the board's home read rule, terminal colour facts — the round reads no board.
@task_item n=7 part=not_needed
Event log rotation and growth rows — no event log format changes.
