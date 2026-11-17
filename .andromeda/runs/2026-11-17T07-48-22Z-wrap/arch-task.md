@task_prose part=session
The session added one standing, committed live rig: a test-only binary viola-live-rig inside the test-only viola-e2e crate, which every later live round reuses.
The rig owns a vt100 screen model of the child's screen, reads the trust dialog and modal dialogs, types only the keys its argv grant allows, isolates each start in a private scratch with its own CLAUDE_CONFIG_DIR, and ends every process it starts (census, sweep, watchdog, stop handlers).
Its rehearsals against the fake agent run in CI. Its live starts against claude 2.1.325 run by hand under a cap of 2 plus 1 spare, and the live verb refuses under CI.
The fake agent under --screens with an untrusted cwd now draws claude 2.1.325's unnumbered trust form over the recorded modal, fires SessionStart only after Enter on the trust row, exits 1 on No, and erases the prompt's last character on DEL. No option, env seam or recorded fixture was added.
Measured live on claude 2.1.325: the trust dialog is unnumbered with the selector first on No, exit; DEL erases the typed character; at submit the CLI keeps a leading run of spaces, removes a trailing run of spaces and TAB, and keeps inner runs and an inner LF; the user's ~/.claude.json reads unchanged under a private CLAUDE_CONFIG_DIR.
The product binary is unchanged; nothing was retired; the release build still carries viola only.
@task_file n=1 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: new; the standing live rig binary of the test-only end-to-end crate, its verbs rehearse, live and census, its argv options and its exit codes
@task_file n=2 path=crates/viola-e2e/src/rig/mod.rs
what: new; the rig's start path: the CI refusal of a live start, the cap admission, the CLI version check, the token-window check, the spawn of the child and its R1, R2 and hold arms
@task_file n=3 path=crates/viola-e2e/src/rig/screen.rs
what: new; the rig's vt100 screen model and its reading of the screen rows: trust dialog, selector, options, input box and modal words
@task_file n=4 path=crates/viola-e2e/src/rig/isolate.rs
what: new; the rig's isolation: the private scratch, the credentials copy and its token-window check, the child's kept environment and the user-config stamp
@task_file n=5 path=crates/viola-e2e/src/rig/procs.rs
what: new; the rig's process hygiene: the census and sweep of its own leftovers by pid and start time, the run marker, the watchdog and the stop handlers
@task_file n=6 path=crates/viola-e2e/src/rig/keys.rs
what: new; the rig's key grant and its typing site: the closed grant parse, the keys allowed per screen reading, the trust-moves reading and the trust pass
@task_file n=7 path=crates/viola-e2e/src/rig/ledger.rs
what: new; the rig's start ledger and kept frames: a start ledgered before it is made, the cap and spare admission, the closed summary words
@task_file n=8 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: new; the rig's rehearsal tests against the fake agent, run in CI, covering the trust pass, the key grant, the exit paths, the credentials copy and the CI refusal
@task_file n=9 path=crates/viola-e2e/Cargo.toml
what: edited; the test-only end-to-end crate's manifest, gaining the viola-live-rig bin target and the vt100 workspace dependency
@task_file n=10 path=crates/viola-e2e/src/lib.rs
what: edited; the test-only end-to-end crate's library root, gaining the rig module
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: edited; the fake agent binary the tests run in place of claude, now drawing the trust form and the input box under its screens mode
@task_file n=12 path=src/bin/viola-fake-agent/input.rs
what: edited; the fake agent's typed-input handling, now erasing on DEL and exposing the prompt text
@task_file n=13 path=Cargo.lock
what: edited; the workspace lockfile, one line for vt100 in the end-to-end crate
@task_item n=1 part=touches
A new test-only binary name viola-live-rig is occupied beside the binary names of the Occupied Resources (viola-fake-agent, viola-harness, viola-hypr-tap, viola-mcp-tap); no name row holds it yet.
@task_item n=2 part=touches
A new harness environment name AGENT_RUN_RIG is occupied under the AGENT_RUN_ prefix rule for test-harness knobs and beside the test harness environment contract items; it is set only on the rig's child as a run marker and is never read by viola.
@task_item n=3 part=touches
The rule that keeps test-only binaries out of release builds and that the release build carries viola only now covers one more test-only binary; the release check still reads viola only.
@task_item n=4 part=touches
The rule that viola-e2e is test-only and that no product crate depends on it: the crate gains a bin target and a rig module, still test-only.
@task_item n=5 part=touches
The rules that a live claude never runs in CI and that real CLI probes run only locally: the rig's live verb refuses under CI while its rehearsals run in CI against the fake agent.
@task_item n=6 part=touches
The rules that tests and CI run against the fake agent: the rig's rehearsals are a new CI test binary against the fake agent.
@task_item n=7 part=touches
The rules that viola types into no trust dialog and that verify never types into a CLI dialog: a test-only rig now types granted keys (at most two down-arrows and Enter on a read trust row, x, DEL, and Ctrl-C only by its stop) into a live CLI's trust dialog; viola itself still types into none.
@task_item n=8 part=touches
The rules that screen row text is read only by verify and that run never reads screen content, and the vt100 screen-model and readiness-gate decisions: the test-only rig reads screen row text through its own vt100 model.
@task_item n=9 part=touches
The decision on compiled screen signatures: the rig reads the input-box prompt through the compiled signatures and the trust and imports dialogs through its own literals.
@task_item n=10 part=touches
The rule that screen fixtures keep only signature rows and that fixtures are recorded only from a clean verify: the fake's trust form keeps the recorded trust row so verify --record over the fake still keeps only signature rows.
@task_item n=11 part=touches
The fake agent's test seam rules (seam variables, the fake named after --): the fake gains a trust form, box rows and DEL erase under its existing screens mode with no new seam, option or fixture.
@task_item n=12 part=touches
The rules that inherited CLAUDE names are stripped from the child and the config.json claude_env_keep contract item: R2's viola home keeps CLAUDE_CONFIG_DIR through claude_env_keep, and the rig's own child keeps only a named host set plus its own names.
@task_item n=13 part=touches
The typed-text normalisation, send line-ending and submitted-form prompt-match decisions: R2 measured claude 2.1.325's submit trim live (leading run kept, trailing spaces and TAB removed, inner run and LF kept); a row whose why cell words the trim set as an assumption not yet read live, with trimming at either end as the candidate, is now stale.
@task_item n=14 part=touches
The rules on test homes cleaned by their owner, test evidence surviving a kill and test-side removals outside the working directory: the rig's census and sweep end only its own marker's or scratch cwd's processes and remove its scratch under a /tmp base on every exit path.
@task_item n=15 part=touches
The pid-plus-start-time liveness decisions: the rig's census identifies its processes by pid and start time through the state crate's start-time reader.
@task_item n=16 part=touches
The print-macro ban and the single human writer in the root bin: a test-only binary in the end-to-end crate prints closed key: word summary lines on stdout.
@task_item n=17 part=touches
The workspace dependency pin rule: vt100 is taken by the end-to-end crate through its one workspace pin.
@task_item n=18 part=touches
The private-mode-before-write rule: the rig places the credentials copy and its scratch config owner-only through the shared private replace helper.
@task_item n=19 part=touches
A test deadline bound: the rehearsal's wait sits at 18 s under the contract lint's 20 s floor and the 30 s end-to-end kill line; a row holding that floor or kill line is touched only to confirm it.
@task_fact n=1 slug=occupied-resources
fact: The binary name viola-live-rig is occupied: the standing live rig, a test-only bin of viola-e2e, never in a release build.
@task_fact n=2 slug=occupied-resources
fact: The environment name AGENT_RUN_RIG, valued rig pid and start time, is occupied: set only on the live rig's child as the run marker its census reads, never read by viola.
@task_fact n=3 slug=ci-cd-approach
fact: One standing committed live rig serves every live round: its rehearsals against the fake agent run in CI, its live starts against the real claude run only by hand under a per-chunk cap, and its live verb refuses under CI.
@task_fact n=4 slug=established-decisions
fact: viola types into no trust dialog, with one exception outside the product: the test-only live rig types only its argv-granted keys (at most two down-arrows and Enter on a read trust row, one x, one DEL) and Ctrl-C only from its stop.
@task_fact n=5 slug=established-decisions
fact: claude 2.1.325 at submit keeps a leading run of spaces, removes a trailing run of spaces and TAB, and keeps an inner run of spaces and an inner LF, measured live.
@task_fact n=6 slug=occupied-resources
fact: The live rig's child runs under its own private CLAUDE_CONFIG_DIR in a scratch it removes on every exit path; the user's own claude config is read only as a hash and mtime, before and after.
@task_item n=1 part=untouched
The wrapper channel protocol, its methods, fault codes, frames and protocol version: no product frame changed.
@task_item n=2 part=untouched
The MCP server, its tools, results and stdin limits, and the plugin's MCP-only install.
@task_item n=3 part=untouched
The viola CLI verbs, their arguments, exit codes, --json documents and human lines: the product binary is unchanged.
@task_item n=4 part=untouched
The normalised event kinds, event data and the events.ndjson log, snapshots and their replay.
@task_item n=5 part=untouched
The hook tiers, hook exits, hook-to-kind map and dialog hold; the fake fires its launch SessionStart through the existing hook path.
@task_item n=6 part=untouched
The wheel, link and pause rules, the send gate, flight lock and confirmation window.
@task_item n=7 part=untouched
The pinned copy, revive, the capability ledger stamps and their single writer, and verify's own probe runs.
@task_item n=8 part=untouched
The Hyprland seam, the fuzz workspace, the mutation runs, coverage floors and the dependency policy beyond the one vt100 use.
