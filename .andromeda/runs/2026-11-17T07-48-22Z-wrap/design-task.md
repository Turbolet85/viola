@task_prose part=session
The session added one standing live rig, a test-only binary named viola-live-rig, that every later live round reuses; no product surface changed and the viola binary is unchanged.
The rig reads the agent CLI's screen through a terminal model, reads and passes the trust dialog with a granted key set (at most two down-arrows and Enter on a read trust row, one x, one DEL, Ctrl-C for the stop alone), and owns scratch isolation, a start cap, a ledger and process census and sweep.
Its rehearsals run in CI against the fake agent; its live arm refuses under CI and ran two live starts by hand on claude 2.1.325 (R1 the trust dialog and box, R2 three viola sends through viola run).
It measured live: the 2.1.325 trust dialog is unnumbered with the selector first on No, exit and the trust row below; the CLI's DEL erases the typed character; at submit the CLI keeps a leading run of spaces, removes a trailing run of spaces and TAB, keeps an inner run and an inner LF.
The fake agent under --screens with an untrusted cwd now draws 2.1.325's trust form over the recorded modal, moves its selector on arrows, fires SessionStart only after Enter on the trust row, draws a box rule and prompt row per typed key and erases on DEL.
Nothing was retired; no count a master states moved.
@task_file n=1 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the standing live rig's entry point, a test-only binary with the verbs rehearse, live and census, its own closed summary words and exit codes 0 to 3
@task_file n=2 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig's start: the plan, the arms R1, R2 and hold, the cap and refusals before a start, the child spawn, and R2's three sends read back as closed words
@task_file n=3 path=crates/viola-e2e/src/rig/screen.rs
what: the live rig's screen model, reading the agent CLI's trust dialog, its selector, its options, the input box and its text, and the imports modal from the screen rows
@task_file n=4 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolation: the private scratch, the private agent config dir, the child's kept environment, the token-window check and the user config read before and after
@task_file n=5 path=crates/viola-e2e/src/rig/procs.rs
what: the live rig's process hygiene: the run marker, the census and sweep of its own leftovers, the watchdog and the stop handlers
@task_file n=6 path=crates/viola-e2e/src/rig/keys.rs
what: the live rig's key grant, the one typing site of a live round: the granted keys, the trust-dialog moves and the refusal of a key outside the grant
@task_file n=7 path=crates/viola-e2e/src/rig/ledger.rs
what: the live rig's start ledger, the cap and spare admission, the kept screen frames and the printed summary lines
@task_file n=8 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal tests against the fake agent, run in CI: trust pass, box after x and DEL, key grant, credentials and processes ended on every exit path, token window, no host path in records, live arm refused under CI
@task_file n=9 path=crates/viola-e2e/Cargo.toml
what: the end-to-end test crate's manifest, now declaring the live rig binary and the terminal emulator dependency
@task_file n=10 path=crates/viola-e2e/src/lib.rs
what: the end-to-end test crate's root, now exporting the rig module
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: the fake agent CLI the tests boot in place of claude: its recorded screens, now its trust dialog in 2.1.325's unnumbered form with a moving selector, its input box rows and its launch SessionStart after trust
@task_file n=12 path=src/bin/viola-fake-agent/input.rs
what: the fake agent's typed-input reader, now holding the prompt text and erasing its last character on DEL
@task_file n=13 path=Cargo.lock
what: the workspace lock file, one line for the terminal emulator dependency now used by the end-to-end test crate
@task_item n=1 part=touches
The prompt-send readback and any row wording the agent CLI's submit trim: R2 measured live on 2.1.325 that a leading run of spaces is kept, a trailing run of spaces and TAB is removed, an inner run and an inner LF are kept; a trailing-only rule stands confirmed, and a row or why cell holding the trim set as unmeasured or a leading trim as open is now stale.
@task_item n=2 part=touches
Indirect, through the rig's R2 arm: the viola send readback and its reply codes (read back, unable, unconfirmable, not-delivered causes, peer reply code words) are now read by the rig as closed code words only; a row naming that vocabulary is what the rig relies on, and no word of it changed.
@task_item n=3 part=touches
Indirect, through the fake agent: viola verify --record over the fake still keeps the recorded trust row, the trust dialog now being drawn over the recorded modal; rows over viola verify's output, its recorded screens and an unclean fixture are reached by the fake's screen change, with no output word changed.
@task_item n=4 part=touches
Indirect, through the fake agent and the live reading: the agent CLI's trust dialog is a dialog shown before the session starts, unnumbered, the selector first on No, exit; rows over dialog reading, dialog-pending and the unknown-dialog refusal reach a dialog form now drawn by the fake and read live.
@task_item n=5 part=touches
Indirect, through the fake agent: the fake now draws a box rule and a prompt row on each typed key and erases on DEL; rows over human typing, input-not-ready and the input box the send path reads are reached by the fake's new box rows.
@task_item n=6 part=touches
Indirect, through R2: viola run started live under a private home with claude 2.1.325 as child and stayed silent while wrapped; rows over viola run's child-started output and the run-silent-while-wrapped rule were exercised live, unchanged.
@task_item n=7 part=touches
Indirect, through the rig's version check: the rig reads the agent CLI's version token and compares it to its --cli word; rows over CLI verification state (verified, unverified-cli) were exercised against 2.1.325, unchanged.
@task_item n=8 part=touches
A name taken outside viola's own verbs: the binary viola-live-rig and the env name AGENT_RUN_RIG are occupied by the test harness; no viola verb or command name was taken or freed.
@task_item n=1 part=untouched
The brand identity, colour palette, typography, spacing, depth, radius and iconography areas: the product binary is unchanged, so no strip, glyph, colour token, SGR style or column rule moved.
@task_item n=2 part=untouched
The viola list board and table, the BAY line, the WHEEL column and the NAME truncation rows: no board output changed.
@task_item n=3 part=untouched
viola's own CLI exit codes, exit 21 causes and their --json causes, revive refusals and codes, run start refusals and strict-modes refusals: none changed; the rig's own exit codes and refusal words are test-only and no viola contract.
@task_item n=4 part=untouched
viola link, unlink, pause, release, answer, wait, last, mcp, hook, plugin install and --help output rows: no verb's output, stream or hint changed.
@task_item n=5 part=untouched
The anti-patterns and self-validation protocol rows: the rig prints closed key: word lines by design as a test tool, which is no human-facing viola output.
@task_item n=6 part=untouched
The web-spa surface and rendered-page absences: still absent.
