@task_prose part=session
The session added one standing live rig: a test-only binary, viola-live-rig, inside the viola-e2e test crate, with verbs rehearse, live and census. No product crate depends on it.
The rig reads the claude screen through a vt100 screen model, reads and passes the trust dialog with granted keys only, isolates each start in a private scratch config, ledgers each start under a cap, and ends every process it starts.
The fake agent under --screens with an untrusted cwd now draws claude 2.1.325's unnumbered trust form over the recorded Screen.modal, holds the launch SessionStart until Enter on the trust row, draws a box rule and prompt row per typed key, and erases on DEL.
Live readings of claude 2.1.325: the trust dialog is unnumbered with the selector first on No, exit; the CLI's DEL erases; at submit the CLI keeps a leading run of spaces, removes a trailing run of spaces and TAB, and keeps inner runs and an inner LF.
The product viola binary is unchanged: no verb, flag, output line, exit code, refusal or hint of the viola CLI moved; nothing retired.
@task_file n=1 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the standing live rig's entry point, a test-only binary outside the viola CLI, its own verbs rehearse, live and census and its own exit codes
@task_file n=2 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig's start sequence, its arms R1 and R2 and its closed key: word summary
@task_file n=3 path=crates/viola-e2e/src/rig/screen.rs
what: the live rig's screen model reading the claude trust dialog, the input box and the imports modal into closed words
@task_file n=4 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolation of each start, its private scratch config, credentials copy and the token-window check
@task_file n=5 path=crates/viola-e2e/src/rig/procs.rs
what: the live rig's process census, sweep, watchdog and stop handlers
@task_file n=6 path=crates/viola-e2e/src/rig/keys.rs
what: the live rig's key grant, the one typing site of the live round
@task_file n=7 path=crates/viola-e2e/src/rig/ledger.rs
what: the live rig's start ledger, the cap and spare admission and the kept frames
@task_file n=8 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal tests run in CI against the fake agent
@task_file n=9 path=crates/viola-e2e/Cargo.toml
what: the end-to-end test crate's manifest, the rig binary and the vt100 dependency
@task_file n=10 path=crates/viola-e2e/src/lib.rs
what: the end-to-end test crate's module list
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: the fake agent, a test-only stand-in for claude that the CLI tests and viola verify --record boot, its trust form, box rows and launch SessionStart
@task_file n=12 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's typed-input handling, its prompt text and DEL erase
@task_file n=13 path=Cargo.lock
what: the dependency lock, one line for vt100 in the test crate
@task_item n=1 part=touches
The viola verify --record output contract: the fake agent's trust form is now drawn over the recorded Screen.modal, and verify --record over the fake still keeps the recorded trust row; a row stating which screen rows --record keeps reads through the fake's screens.
@task_item n=2 part=touches
Any row that words the claude trust dialog's form (numbered options, the selector on the trust row) is contradicted by the live 2.1.325 reading: the form is unnumbered, the selector stands first on No, exit, the trust row one row below.
@task_item n=3 part=touches
Any send-output or readback row that words the CLI's trim of submitted text as unmeasured, or a leading trim as open: R2 measured the leading run kept, the trailing spaces and TAB removed, inner runs and an inner LF kept.
@task_item n=4 part=touches
The viola run passed start output and any start row over the fake agent: under --screens with an untrusted cwd the fake now fires no hook and no launch SessionStart until Enter on the trust row, and Enter on No, exit exits 1.
@task_item n=5 part=touches
The viola send input-not-ready hint and any row over input readiness read off the fake's screen: the fake now draws a box rule and a prompt row on each typed key and erases the prompt's last character on DEL.
@task_item n=6 part=touches
Names taken: the test-only binary viola-live-rig and the env name AGENT_RUN_RIG; a row listing occupied binary or env names, or a verb name the viola CLI reserves, may need them beside it.
@task_item n=1 part=untouched
The viola CLI's verbs, flags, global flags and help output: no verb or flag was added, renamed or removed; the rig's own verbs belong to a separate test-only binary, not to the viola verb list.
@task_item n=2 part=untouched
The CLI exit codes and their --json form: unchanged; the rig's exit codes 0 to 3 are its own, not viola's.
@task_item n=3 part=untouched
The viola list board, its columns, colours, truncation and the BAY line: untouched.
@task_item n=4 part=untouched
The viola mcp tools and tool results, and the MCP hiding decisions: untouched.
@task_item n=5 part=untouched
Refusal lines, hints, the shared refusal word column and the exit-21 details and hints: no refusal or hint of the viola CLI moved.
@task_item n=6 part=untouched
The CLI token rendering and styling decisions (SGR, colour roles, spacing, no spinner, no banner, no pager): untouched.
@task_item n=7 part=untouched
The viola revive, link, unlink, plugin install, wait, last and answer outputs and refusals: untouched.
@task_item n=8 part=untouched
The web-surface absence and the no-deep-links absence: untouched.
