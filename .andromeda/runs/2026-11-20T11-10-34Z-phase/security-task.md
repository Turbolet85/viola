@task_prose part=chunk
A question dialog (the CLI's question tool) first raised through the PermissionRequest hook is answered on the driver's answer with an allow decision body whose updatedInput carries the driver's answers, where today that question stays null to the human.
It adds one relied capability-ledger row in the Claude adapter for that body, with its viola verify probe; dialog answers stay gated on a verify stamp on which every relied row counts.
It re-stamps the CLI versions against the grown relied set in the rig's scratch home (which versions is the founder's word at P3); the live home's re-stamp stays with the product re-pin step.
It claims capability v2-20; the present path and its null are read at P3, with no live start and no read of the operator's real Claude home in this phase.
@task_file n=1 path=crates/viola-agent-claude/src/dialog.rs mark=edits
what: the Claude adapter's dialog answer mapping — edits: the question answer mapping into the allow body's updatedInput
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the capability ledger of the Claude adapter — edits: the relied rows list and their count, the new row for the allow body
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the Claude adapter's hook payload parsing and hook decision body — edits: the PermissionRequest decision body for a question
@task_file n=4 path=src/cmd/verify.rs mark=edits
what: viola verify, its probes against the real CLI and the verify stamp it writes — edits: the new probe, the stamp's row set
@task_file n=5 path=src/cmd/hook.rs mark=edits
what: viola hook, the hook entry that relays a dialog to the wrapper over hook.dialog and prints the decision — edits: the PermissionRequest path for a question
@task_file n=6 path=src/run/dialog.rs mark=edits
what: the run wheel's dialog relay, which holds a dialog for the driver's answer and falls back when the hold expires — edits: the PermissionRequest question relay
@task_file n=7 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent test binary — edits: its PermissionRequest question script
@task_file n=8 path=fixtures/claude/2.1.328 mark=edits
what: recorded Claude CLI fixtures for one CLI version, under the fixture hygiene walk
@task_item n=1 part=needs
The PermissionRequest hook decision for a question: an allow decision body whose updatedInput carries the driver's answers, printed by viola hook on the dialog answer relayed over the hook.dialog channel method.
@task_item n=2 part=needs
The driver's answer path: the answer channel method, the MCP answer tool and the CLI answer verb, with their answer params, refusal words and control-character refusal; their shape changes only if the answers must change shape.
@task_item n=3 part=needs
viola stays mechanism: it carries the driver's answers into the body and never chooses them; question option text and answer text are content crossing into events and the decision body, never a command.
@task_item n=4 part=needs
Dialog free text in the answers keeps the dialog free-text character set and control-character refusal, refused whole and never stripped, checked on the client and re-checked by the wrapper before it reaches updatedInput.
@task_item n=5 part=needs
Dialog answers need a verify stamp: the question answer stays gated on a stamp on which every relied ledger row counts, the new row included, read through the strict-modes check on the stamps file.
@task_item n=6 part=needs
The new verify probe: verify never answers a dialog nor types a dialog byte of its own beyond its compiled probe, reads the screen only against compiled literals and signatures, uses synthetic prompts, a pre-trusted folder, transient captures and probe directories it removes.
@task_item n=7 part=needs
Only viola verify writes the stamps file; the re-stamp of the CLI versions is written in the rig's scratch home, its kept stamp owner-only and seeded to its own home.
@task_item n=8 part=needs
No read of the operator's real Claude home and nothing written under the Claude home or Claude's config; the live rig's child stays isolated from the user's config, its credentials copy transient and unrecorded; the real CLI runs only locally, never in CI.
@task_item n=9 part=needs
The human always wins: a refusal reaches automation only, the human at the keyboard can answer the dialog himself at any moment, security refusals never block the human, and an expired dialog hold keeps today's conservative fall-back (hook fails open to the human; the hold is set only in config).
@task_item n=10 part=needs
viola hook rules the new path acts inside: instance and strict-modes check before any read, server verified before any frame, hook stdin over-limit detected and dialog reply read bounded, tolerant payload parsing with security fields in closed enums, silent with exit zero, a refused hook writing no frame.
@task_item n=11 part=needs
The hook payload and the wrapper's dialog reply are content: hook.dialog params and the answer frame stay closed types re-validated by the wrapper, and invalid hook frames are dropped.
@task_item n=12 part=needs
Error output on the new path holds no upstream text, no paths, pids or ids; full detail only in diagnostics, user content only in diagnostics, and the hook-decision detail words stay closed.
@task_item n=13 part=needs
Recorded fixtures for the question body are scrubbed or refused whole, keep only signature rows of screens, and are reviewed before commit under the fixture hygiene walk.
@task_item n=14 part=needs
The fake agent's PermissionRequest question script is a test seam only under the fake-agent feature, compiled out of the release build.
@task_item n=15 part=needs
The capability ledger's relied row list and count wording, and verify's probe list, as the plan's rows word them, change with the new row.
@task_standing n=1
convention: each new test runs in its own home, removes only directories it created and keeps test homes out of git
home: secret-management
@task_standing n=2
convention: each new test seam or environment read added for the fake agent or verify needs its log entry and stays under the fake-agent feature
home: security-anti-patterns-never-do-these
@task_standing n=3
convention: each new recorded fixture is scrubbed, keeps signature rows only and passes the fixture hygiene walk before commit
home: data-protection
@task_item n=1 part=not_needed
viola revive, its arguments, its --json codes and the recorded cwd spawn: the chunk does not touch session revival.
@task_item n=2 part=not_needed
viola link and unlink, their link lines and exit codes: the chunk adds no link behaviour.
@task_item n=3 part=not_needed
viola list's board scan, its home read rule and terminal colour facts: the board is not reached.
@task_item n=4 part=not_needed
pause and release with their from field and the driver-release refusal: the wheel's ownership verbs are unchanged.
@task_item n=5 part=not_needed
The socket bind, socket chmod and per-user socket directory, and the start collision check: the endpoint and its creation are unchanged.
@task_item n=6 part=not_needed
Pinned binary placement, its SHA-256 re-hash, plugin install and the marketplace tree: no pin or plugin step is part of this chunk.
@task_item n=7 part=not_needed
The Hyprland recorder and its reply parsing: no window manager reading is reached.
@task_item n=8 part=not_needed
Absences of hosting, public listeners, databases, webhooks, file upload, user accounts and remote access: the chunk adds none of these.
@task_item n=9 part=not_needed
Release workflow artefacts, attestations, signed self-update and CI workflow pinning or permissions: no release or workflow is changed.
