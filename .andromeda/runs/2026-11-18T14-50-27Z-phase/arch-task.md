@task_prose part=chunk
Adds a compiled screen literal for the CLI's browser-tools offer (the 2.1.325 ChromeAutoEnableDialog) to the modal set the one settled-box predicate reads, so verify's pastes and run's pre-send gate read that screen as a standing modal, never as a settled input box.
Adds the 24th relied row of the version-stamped capability ledger for that literal, with a viola verify probe that measures it on an installed CLI and stamps it; the relied row count goes 23 to 24.
run's pre-send gate refuses to type at a standing browser-tools prompt, as at every modal: no byte is typed into the prompt, the stop with the modal standing included.
The fake agent learns the browser-tools screen so CI proves the literal, the row and the gate's refusal against recorded screens; nothing is re-pinned and a home stamped at 23 rows stays as it stands.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the per-behaviour CLI capability ledger in viola-agent-claude, its relied rows and their count — edits: a new relied row (the 24th) for the browser-tools screen literal
@task_file n=2 path=crates/viola-agent-claude/src/screen.rs mark=edits
what: the screen-signature compiled literals and the modal set the settled-box predicate reads before an input-box literal — edits: the browser-tools modal literal, the modal set
@task_file n=3 path=src/run/gate.rs mark=edits
what: run's pre-send readiness gate over the vt100 screen model — edits: the gate's reading of a modal (refuse to type)
@task_file n=4 path=src/cmd/verify.rs mark=edits
what: the viola verify command, its probe runs and its writing of the ledger stamps — edits: the new row's probe, its stamp, the relied row count 23 to 24
@task_file n=5 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent binary CI runs against, its set of recorded screens — edits: a browser-tools prompt screen
@task_item n=1 part=needs
The capability ledger as the single gate for CLI behaviour: the browser-tools literal becomes a relied, version-stamped row, and a CLI version is verified only when every relied row, the new one included, passes.
@task_item n=2 part=needs
Screen signatures as compiled literals: the browser-tools title is a compiled literal in the agent crate, and screen row text is read only by verify, never by run's own content reading.
@task_item n=3 part=needs
The shared settled-box predicate reads modal literals first, so a standing browser-tools prompt is a modal and never a settled input box, for verify's pastes and run's pre-send gate alike.
@task_item n=4 part=needs
viola types into no CLI dialog: verify never answers or types into the browser-tools prompt, and its probe reads it without typing a byte.
@task_item n=5 part=needs
run's pre-send readiness gate on a standing modal refuses to type and waits only within its compiled gate timings and send time bound; the input-not-ready refusal is what a send meets.
@task_item n=6 part=needs
The new row's verify probe: which probe run meets the prompt, the bounded verify child set, verify's own trust record and probe dirs, and the verify box wait for a literal after added turns.
@task_item n=7 part=needs
The ledger stamps envelope: verify as the one stamps writer, its rows and verified version, a merge that replaces only its own version, and a malformed or untrusted stamp read unverified.
@task_item n=8 part=needs
A home stamped at 23 rows read by a binary built from this tree: the version gate reads stamps strictly, and an unverified CLI degrades to the partial, transport-only gate.
@task_item n=9 part=needs
viola verify stdout and exit codes: the step line and summary line for the new row, and exit 1 on a failed relied row.
@task_item n=10 part=needs
viola-agent-claude stays pure: the literal and the modal set do no file I/O, and Claude-specific shapes stay isolated in that one crate.
@task_item n=11 part=needs
Viola writes nothing to turn the prompt off: no seeding of the CLI's browser-tools key, no settings layer, the global CLI config untouched by this chunk.
@task_item n=12 part=needs
Upstream screen text is untrusted content: the prompt's option text stays out of human lines and error surfaces.
@task_item n=13 part=needs
Recorded fixtures only from a clean verify, scrubbed and hygiene-walked, keeping only signature rows, should the recorded set need a browser-tools screen.
@task_item n=14 part=needs
Real CLI probes run only locally, with no live claude in CI: live starts are a founder fork, and CI proves the change against the fake agent.
@task_item n=15 part=needs
Versioned records instead of migrations: the 23 to 24 row step reads an older stamps file tolerantly, with no rewrite of a stamped home.
@task_standing n=1
convention: every new test runs in its own test-owned viola home, removed by its owner, with test sockets outside the product namespace
home: occupied-resources
@task_standing n=2
convention: every new test runs against the fake agent through its test seams, never a live claude, and test seams are no configuration channel
home: occupied-resources
@task_standing n=3
convention: every new test wait stays under the shared root test bound
home: occupied-resources
@task_standing n=4
convention: every new module is reached from the module tree, lints pass with warnings denied and no print macros, and new code keeps the coverage floors
home: build-system · ci-cd-approach
@task_standing n=5
convention: every new log line is a codes-only process log through the one event emitter, content-bearing detail kept to the instance
home: cross-cutting-patterns
@task_item n=1 part=not_needed
The wheel, its claim, take and release rows: the chunk changes no wheel ownership or human-take rule.
@task_item n=2 part=not_needed
The dialog hold, hook tiers and hook-to-kind map: the browser-tools prompt is a screen the CLI shows, answered by no hook and logged as no dialog event.
@task_item n=3 part=not_needed
The MCP server, its stdin reader and tools list: the chunk adds no tool and changes no MCP surface.
@task_item n=4 part=not_needed
Links, link and unlink methods and their events: untouched.
@task_item n=5 part=not_needed
The board, viola list and the agents listing: unchanged.
@task_item n=6 part=not_needed
Revive, its refusals and its JSON error codes: the chunk reaches no revive path.
@task_item n=7 part=not_needed
The Hyprland window seam and the terminal fleet console: not reached.
@task_item n=8 part=not_needed
Release tooling, deployment model and install rows: nothing is released or re-pinned.
@task_item n=9 part=not_needed
The trust writer code words and the wrapped start's session-folder trust write: unchanged by this chunk.
