@task_prose part=chunk
A question dialog (the CLI's question tool) first raised through the PermissionRequest hook is answered, on the driver's answer carrying the answers, with an allow decision body whose updatedInput carries those answers, where today it stays null to the human.
A new relied capability-ledger row in viola-agent-claude for that body, with its own viola verify probe; the dialog decision stays gated on a verify stamp counting every relied row.
The CLI versions are re-stamped against the grown relied set (which versions and which home are settled at P3).
Capability v2-20: a question raised by a permission request is answered by the driver, never left to the human; the new row has its probe and the re-stamped versions pass it.
@task_file n=1 path=crates/viola-agent-claude/src/dialog.rs mark=edits
what: the dialog answer mapping and the hook decision body for dialogs (question, permission, plan) — edits: the question answer mapping on the PermissionRequest path, the allow decision body carrying updatedInput
@task_file n=2 path=crates/viola-agent-claude mark=edits
what: the agent adapter crate holding the capability ledger, its relied rows and their count — edits: the relied row list (one new row), the relied row count
@task_file n=3 path=src/cmd/verify mark=edits
what: viola verify and its ledger-row probes, writing the verify stamp over the relied row set — edits: a new probe for the new row, the stamp's row set
@task_file n=4 path=src/cmd/hook mark=edits
what: the viola hook exec-form entry points and the run wrapper's dialog relay — edits: the PermissionRequest path for a question dialog
@task_file n=5 path=fixtures/claude mark=edits
what: recorded Claude hook payload fixtures per CLI version, replayed by the fake agent — edits: the PermissionRequest question payload fixtures
@task_item n=1 part=needs
The question hook decision body: a question raised via PermissionRequest is answered with an allow decision whose updatedInput carries the driver's answers, replacing today's null to the human; pinned as a decision body snapshot.
@task_item n=2 part=needs
The dialog answer mapping: the driver's answer for a question maps to that body; viola carries the answers verbatim and never chooses them; option and answer text is content, never a command.
@task_item n=3 part=needs
The capability ledger grows by one relied row; the relied row count and every place that counts it change with it.
@task_item n=4 part=needs
Every ledger row has a verify probe: the new row's probe in viola verify, its step line and summary line, and its contract-suite coverage against the fake agent.
@task_item n=5 part=needs
The version gate needs every relied row: a stamp lacking the new row (a 26-row stamp of an older relied set) leaves the CLI unverified, so the question decision body and answer stay gated; stamps refuse a widened ledger.
@task_item n=6 part=needs
Decision bodies only for a driver on a stamped CLI: an unverified CLI degrades to send only, answer refused as unverified-cli on both CLI and MCP.
@task_item n=7 part=needs
The human always wins: the human at the keyboard can answer the dialog at any moment; a refusal reaches automation only; no blanket dialog approval.
@task_item n=8 part=needs
Dialog hold expiry: an expired hold keeps today's conservative fall-back; dialog timeouts outlast the longest hold; hooks fail open.
@task_item n=9 part=needs
Hook decision body and hook stdout contracts: one decision body per dialog, the second concurrent dialog, the hook-decision log record and its corr.
@task_item n=10 part=needs
The dialog relay events: the question dialog event logged once, the pending dialog on the snapshot before its line, the question event data and option details, dialog-raised and dialog-answered log lines.
@task_item n=11 part=needs
The answer surface (viola answer and the MCP answer tool) carries the answers; its shape changes only if the scope's P3 reading requires it, and CLI and MCP stay at payload parity.
@task_item n=12 part=needs
Fake agent and verify fixtures are the contract: the fake agent's PermissionRequest question script and recorded fixtures per CLI version, replayed byte for byte, schema-checked, scrubbed and recorded only by verify with zero failing rows.
@task_item n=13 part=needs
Stamps come only from verify (against the fake agent in tests); check-guarded cases start stamped, so test stamps must cover the grown relied set.
@task_item n=14 part=needs
Compatibility across CLI versions: every CLI-version fixture set replayed; the re-stamp of 2.1.328 (and possibly 2.1.329) against the grown set; real claude only locally, never in CI; a live reading recorded as a reading, never as verification.
@task_item n=15 part=needs
Unknown hook payload fields tolerated; recorded hook payloads walked at run time; hook unknown events counted, not failed.
@task_item n=16 part=needs
No live start and no read of the operator's real claude home in this phase; tests never read the host's claude sessions and run under a test-owned user home.
@task_item n=17 part=needs
Carry: the intermittent rig rehearsal red (box empty after x and del) stays open; at its next red the rig summary and kept after-del frame are read and the cause named before any wait is mended, with no retry rule and no loosened entry.
@task_standing n=1
convention: each new decision body is pinned by an insta snapshot in check mode only, asserted as a literal
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=2
convention: each new test follows the test function naming, uses rstest case tables, lives inline for unit tests and under the surface-prefixed binary for integration and E2E
home: 2-test-strategy
@task_standing n=3
convention: each new test creates its own private home, waits on logged events, never sleeps or retries, never asserts on the child's screen
home: 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: each new fixture is synthetic, scrubbed of real data, carries a canary, and passes the fixture hygiene walk
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=5
convention: each new security-relevant path has one negative test, and secrets never reach logs or diagnostics
home: e2e-test-strategy
@task_standing n=6
convention: each new log line is structured JSON with its event and corr fields per the log format
home: log-format
@task_item n=1 part=not_needed
viola revive: its preflight order, refusal codes, exit codes and documents; the chunk adds no revive behaviour.
@task_item n=2 part=not_needed
The viola list board and list --json rows, liveness and skipped counts; nothing on the board changes.
@task_item n=3 part=not_needed
Link and unlink methods, link records and links surviving restart.
@task_item n=4 part=not_needed
Send confirmation: prompt-submitted, /clear and /compact post-conditions, typed framing, send refusal order and send human-mode lines.
@task_item n=5 part=not_needed
Heartbeat staleness classification, start collisions and stale-heartbeat start refusals.
@task_item n=6 part=not_needed
Harness session, boot, cleanup and status documents, supervisor exits and the pre-push stages; the chunk adds no harness command.
@task_item n=7 part=not_needed
CI artifact names, workflow permissions, SHA pinning, zizmor and advisory scans; the chunk adds no workflow.
@task_item n=8 part=not_needed
Fuzz targets and corpora for the vt100 and agents parsers; no new parser surface.
@task_item n=9 part=not_needed
Wait and last: wake kinds, timeouts and scheduled wakes in turn-ended data.
@task_item n=10 part=not_needed
Windows and macOS support, a database, outbound HTTP, a browser suite and a deploy stage.
