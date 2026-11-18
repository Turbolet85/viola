@task_prose part=chunk
Builds the product write of the CLI's per-project trust record at the one start shared by viola run and viola revive: after the version gate and before the child spawn, on a verified CLI, when no record already trusts the child's folder read the CLI's way.
viola writes only projects[<key>].hasTrustDialogAccepted into the one Claude config file the started child reads, under the CLI's lock when free, by atomic replace, read back, at most 3 attempts; a refusal or a lost record writes nothing more and leaves the trust dialog to the human, viola typing no key.
It reuses the existing trust-record writer built for verify's Run F rather than adding a second writer, and adds a case per refusal form proving nothing was written, plus an e2e case on the fake agent that fails when the write is gone.
One closed word per start reaches the run log. The live proof is S1 (viola run in a fresh untrusted folder under the rig's private config) and its control S2 (a plain claude in the same folder shape) on 2.1.325, under a cap asked at the end of P3; the chunk claims matrix v2-33.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs mark=edits
what: the CLI trust record's shape and the key rule (git root inside a repository, else the cwd) — edits: the trust reading, the exact key then the parent walk
@task_file n=2 path=src/trust_record.rs mark=edits
what: the trust-record file write: the CLI's lock, the atomic replace keeping the file's mode, the refusals and their closed code words — edits: the read-back and retry, the refusals, the write for an absent entry
@task_file n=3 path=src/bin/viola-fake-agent
what: the fake agent standing in for the claude CLI, its trust decision read from the Claude config file (--trust-config) and its trust dialog screen
@task_item n=1 part=needs
The run and revive start sequence: the trust write sits after the version gate and before the child spawn, inside the checks viola run does before the child; revive takes the same write through the shared start, its preflight order otherwise unchanged.
@task_item n=2 part=needs
The write happens only on a verified CLI: the version gate needs every relied ledger row, so a home stamped at 22 rows reads unverified now that trust-record is the 23rd relied row, and such a start writes no trust record and leaves the dialog to the human.
@task_item n=3 part=needs
Trust is read the CLI's way before any write: the exact key, then the parent walk up to the git root inside a repository or to / outside one; a folder the exact key or a parent already trusts gets no record, and nothing is written for the home folder.
@task_item n=4 part=needs
The written record is exactly one key, projects[<key>].hasTrustDialogAccepted, with <key> the key the CLI itself writes on an accepted dialog; no other key changes, no entry is removed, another project's entry stays byte for byte as it was.
@task_item n=5 part=needs
The target file: <CLAUDE_CONFIG_DIR>/.claude.json when viola's home keeps that variable for the child through claude_env_keep, else the user's ~/.claude.json; viola reads that one environment value for that one purpose, through the child environment strip and its kept list.
@task_item n=6 part=needs
The race handling: take the CLI's own lock when it is free, replace the file atomically keeping its mode, read the record back before the spawn, at most 3 attempts; a record still lost after the 3rd attempt writes nothing more and leaves the dialog to the human; the residual lost concurrent write of another session is accepted and named.
@task_item n=7 part=needs
The refusals write nothing and never create the file: absent, unreadable, a link, not the JSON expected (not an object, projects not an object, the entry not an object), oversize past MAX_FRAME; each gets a case proving the file's bytes unchanged, no temp sibling left and no lock left.
@task_item n=8 part=needs
The skips that are no refusal each get a case proving nothing written: an unverified CLI, a folder the exact key or a parent already trusts, the home folder.
@task_item n=9 part=needs
viola never types a key into the trust dialog: on any refusal, skip or lost record the dialog stays the human's; no blanket dialog approval and no emulated keypress stands in for the write.
@task_item n=10 part=needs
One closed word per start in the run log naming the write's outcome, nothing of the config file anywhere (no path content, no bytes, no project keys) in logs, diagnostics or error bodies; the closed code words follow the code-word format and a new word changes a closed enum only by decision.
@task_item n=11 part=needs
The e2e case over the fake agent: the fake reads its trust decision from the Claude config file given by --trust-config, so a viola run in a fresh untrusted folder shows no trust screen with the write and the trust screen without it; a change to the fake's screens or trust reading puts cli_verify in the gate block.
@task_item n=12 part=needs
Tests never touch the host's real ~/.claude.json or claude sessions: every case writes into a test-owned config dir under a test-owned home, a verify-driven or stamped start tied to a test-owned home, the config seeded by the test itself.
@task_item n=13 part=needs
The live starts S1 and S2 on the real claude CLI, local only and never in CI, 2.1.325 by path, under the rig's private config with one other project's entry as sentinel and claudeInChromeDefaultEnabled seeded; the rig's home gets a 23-row stamp; every byte the rig types named, the stop the Ctrl-C into the rig's pty, never while a dialog stands, else the no-key group kill.
@task_item n=14 part=needs
The live predicates, each one its reading can fail: S1 red when the first screen reads trust, when the sentinel entry differs by a byte, or when the written record is not exactly the one key; S2 red when the first screen does not read trust; both red on a missing teardown word; a live reading is recorded as read, never as verification.
@task_item n=15 part=needs
In S1 viola's write goes to the rig's private <scratch>/config/.claude.json; the founder's real ~/.claude.json is only read, by the mark witness; a spare start is spent only on a cause a start itself showed, after its mend lands through a plan revision.
@task_item n=16 part=needs
Each control's --timeout S is named in the plan's prose and fired with that flag, on an entry whose build reaches every binary its case needs; ids within 16 characters; holds means those cases ran and failed with no TIMEOUT line; each control says what the subject does when the guard is gone, measured or marked unmeasured; no mutation run.
@task_item n=17 part=needs
A test that waits takes a bound under the 20 s lint; a test-support helper spawning an instrumented binary under a cleared environment runs it in a scratch directory and re-adds LLVM_PROFILE_FILE.
@task_item n=18 part=needs
The relied ledger row trust-record, read pass live on 2.1.325, is the row the product write lands on; nothing is measured on 2.1.326 or 2.1.327 and nothing is re-pinned.
@task_item n=19 part=needs
The refusal and lost-race faults are real filesystem states made by the test (a link, a chmod, an oversize file, a held lock), never a mocked filesystem; time and retries stay out of sleeps.
@task_standing n=1
convention: every new test function follows the test naming rule and its cases sit in table-driven rstest cases where they share a shape
home: 2-test-strategy
@task_standing n=2
convention: every new test creates its own fresh home and seeds its own data; on-disk product state is produced by viola, never hand-written, and test homes are private directories
home: test-data-fixtures · 7-test-data-fixtures
@task_standing n=3
convention: every new test waits on logged events or file state within a bound, never by sleep or retry
home: 2-test-strategy
@task_standing n=4
convention: every new security control gets one negative test and a bounded remove-the-guard witness
home: test-strategy
@task_standing n=5
convention: every new filesystem fault in a test is a real fault on a real file, and external processes are real stub executables
home: mocking-stubbing-discipline
@task_standing n=6
convention: every new synthetic test input carries a canary and no real data, and every new e2e assertion reads records, never the rendered child screen
home: e2e-test-strategy
@task_standing n=7
convention: every new log line is a codes-only JSON line with its schema fields, detail kept apart
home: log-format
@task_item n=1 part=not_needed
The MCP tool surfaces, their schemas, invalid-params fields and reply screens: the chunk adds no MCP tool and changes no tool result.
@task_item n=2 part=not_needed
The send and answer paths, send confirmation, local commands, dialog decision bodies and hook decision bodies: the trust write is no send and no dialog answer.
@task_item n=3 part=not_needed
The wheel, release, link and unlink methods and their event records: the write does not move the wheel or touch links.
@task_item n=4 part=not_needed
The harness command contracts (boot, status, cleanup, supervise, session record, agent-run documents) and the harness gate's breach and exit codes: the chunk adds no harness command or field.
@task_item n=5 part=not_needed
Hook performance budgets, hyperfine timing and spine deadlines: the write runs at the wrapped start, not on any hook path.
@task_item n=6 part=not_needed
The viola list board, its human columns and JSON rows, heartbeat staleness and instance liveness: nothing the board shows changes.
@task_item n=7 part=not_needed
The browser-tools prompt read as a modal, the 2.1.325 recorded fixture set and the live home's re-stamp, and any reading on 2.1.326 or 2.1.327: the scope places those in other entries.
@task_item n=8 part=not_needed
Fuzz targets, fuzz corpora and property generators of parser surfaces: the chunk adds no new parser surface to fuzz.
