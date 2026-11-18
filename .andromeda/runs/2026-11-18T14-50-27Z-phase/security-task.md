@task_prose part=chunk
Adds a compiled screen literal for the claude CLI's browser-tools offer at an interactive start (the 2.1.325 Chrome auto-enable dialog), so the one shared settled-box predicate reads that screen as a modal standing, never as a settled input box.
Adds its row to the capability ledger (the 24th relied row) and a viola verify probe that measures it on an installed CLI, version-stamped like every other screen signature; the verify row count goes from 23 to 24.
viola run's pre-send gate refuses to type while the browser-tools prompt stands, as at every modal: no byte is typed into the prompt, the stop with the modal standing included.
The fake agent learns the browser-tools screen, so CI proves the literal, the row and the gate's refusal against recorded screens.
Viola answers nothing in the prompt and writes nothing to turn it off; nothing is re-pinned and a home stamped at 23 rows stays as it stands.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the Claude adapter's capability ledger of relied rows, each measured by viola verify and version-stamped — edits: a new relied row for the browser-tools prompt literal (the 24th row)
@task_file n=2 path=crates/viola-agent-claude/src/screen.rs mark=edits
what: the Claude adapter's compiled screen-signature literals and the input box readiness read against them — edits: the browser-tools screen literal, the modal set the settled-box predicate reads
@task_file n=3 path=src/run/send/slot.rs mark=edits
what: viola run's pre-send gate, which types text into the child only when the input box reads settled — edits: its reading of a standing modal (refuse to type)
@task_file n=4 path=src/cmd/verify.rs mark=edits
what: viola verify, which runs probe children against the installed claude CLI, reads their screens only against compiled literals and writes the stamps file — edits: the new row's probe and its stamp, the row count 23 to 24
@task_file n=5 path=src/cmd/verify/typed/runs.rs mark=edits
what: viola verify's interactive probe runs on a PTY, each run's first screen read in closed words — edits: the run in which the browser-tools prompt is read without typing
@task_file n=6 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent test binary that stands in for the claude CLI in CI, with its recorded screen set and test-only seams — edits: a browser-tools prompt screen
@task_item n=1 part=needs
The settled-box predicate shared by verify's pastes and run's pre-send gate reads the screen only against compiled literals, modal literals first; the new browser-tools literal joins that compiled modal set and never makes the screen read as a settled input box.
@task_item n=2 part=needs
No byte is ever typed into the browser-tools prompt: not by run's pre-send gate, not by verify's probe runs, not at the stop with the modal standing; verify never answers a dialog and never types into one.
@task_item n=3 part=needs
Upstream screen text is content, never a command: the prompt's title and options are matched against a compiled literal and never acted on as an instruction or carried outward as text.
@task_item n=4 part=needs
The new ledger row is a measured, version-stamped dependence: verify's new probe reads it by signature only, the stamp's first screen in closed words, and only viola verify writes the stamps file, through the strict-modes check.
@task_item n=5 part=needs
A home stamped at 23 rows is read by the version gate through the strict-modes stamps read; what run does there under a binary with 24 rows follows the existing unverified-CLI gate and dialog-answers-need-a-verify-stamp rules, with no control switched off.
@task_item n=6 part=needs
Verify's probe children are spawned directly, with the credential strip and the hermetic child environment, their probe directories confined to their own directory, owner-only and removed by drop guards; the probe needs a pre-trusted folder and answers no import or trust dialog.
@task_item n=7 part=needs
Viola writes nothing under the Claude home and nothing in Claude's config to turn the prompt off: no browser-tools default key is seeded, and the live rig's child config stays seeded from bundle logic only.
@task_item n=8 part=needs
The fake agent's browser-tools screen sits behind the test-only fake-agent feature; its seams compile out of the release build.
@task_item n=9 part=needs
Any recorded screen fixture added for the prompt keeps only signature rows, is scrubbed or refused whole for paths, user names and emails, and is reviewed before commit.
@task_item n=10 part=needs
Live starts on the standing rig (planned only, not run, at this phase) key only into the rig's own PTY, keep the prompt capture to closed words, use a transient unrecorded credentials copy and an isolated child config; live session screens stay local and the founder's real config is read mark-witness only.
@task_item n=11 part=needs
The PTY screen feed for the new literal stays bounded, isolated from passthrough and under the panic catch; verify's PTY output stays bounded and caught.
@task_item n=12 part=needs
Any verify record word added for the new row is a closed code word and carries no path, pid, id or upstream text.
@task_standing n=1
convention: every new test uses its own test-owned home, kept out of git, and removes only the directories it created
home: secret-management
@task_standing n=2
convention: every new test seam or test-only screen lives only under the fake-agent feature and compiles out of release
home: security-anti-patterns
@task_standing n=3
convention: every new recorded fixture keeps only signature rows and passes the fixture hygiene scrub before commit
home: data-protection
@task_standing n=4
convention: every new CI step uploads nothing unscanned; uploads stay gated by the artifact canary scan
home: bootstrap-phases · bootstrap-phases-derive-for-route-setup-project
@task_item n=1 part=not_needed
The channel, its frames, methods, peer euid checks and socket-directory rules: the chunk adds no channel method, frame or listener.
@task_item n=2 part=not_needed
MCP server tools, their inputs, structuredContent and stdin line caps: no MCP tool changes.
@task_item n=3 part=not_needed
Hook payload parsing, hook decisions and hook refusals: the prompt is a screen read, not a hook event.
@task_item n=4 part=not_needed
Paste and dialog free-text character sets, send refusal words and typed-text normalisation: no new text is typed.
@task_item n=5 part=not_needed
Link and unlink, revive, viola list and the board's read rule: none of those verbs is touched.
@task_item n=6 part=not_needed
Dependency pins, cargo-deny, licence and advisory lists, toolchain pins and workflow pinning: no dependency or workflow is added.
@task_item n=7 part=not_needed
Pinned binary and plugin placement, re-hash and marketplace rules: nothing is re-pinned and no plugin file changes.
@task_item n=8 part=not_needed
The start trust record writer and folder-trust skip rules from the previous chunk: this chunk writes no trust record.
@task_item n=9 part=not_needed
Hyprland recorder and hyprctl reads, terminal colour facts and human output escaping: untouched.
@task_item n=10 part=not_needed
Event log growth, rotation and link lines, and stop background-task and cron counts: no event line or stop field changes.
