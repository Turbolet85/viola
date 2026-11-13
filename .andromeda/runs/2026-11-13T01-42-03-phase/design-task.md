@task_prose part=chunk
Changes how the typed run of viola verify (Run B: the probe paste, then the long, tag-like, newline, /compact and local-command pastes) decides the input box is up: from the input box itself, never from a footer literal that a transient footer notice (an update notice, a usage warning) displaces after a turn.
Revises the agent's screen signature for the input box and its capability-ledger row, which keeps a viola verify probe; the fake agent's recorded screens and fixtures gain a replayed footer notice.
Re-measures by one viola verify on the newest installed claude CLI, every ledger row passing but the account-dependent rate-limits row, its stamps written by verify itself; 2.1.296 is never re-verified.
Turns CI's supply-chain job green again: the cargo deny advisories against rmcp 3.4.1 and rustix 1.1.5, by a bump or a reasoned per-advisory ignore.
@task_file n=1 path=src/cmd/verify/typed/turns.rs mark=edits
what: viola verify's typed run, its turns and pastes and how each waits for the input box — edits: input_box_up, its callers framing_turns and turn's settle, Run's box_wait
@task_file n=2 path=crates/viola-agent-claude/src/screen.rs mark=edits
what: the Claude agent's screen signatures, the readiness literals read off the screen — edits: SIGNATURES.input_box
@task_file n=3 path=ledger/stamps.json
what: the capability-ledger stamps per CLI version that decide whether a CLI is verified, written only by viola verify
@task_file n=4 path=Cargo.toml mark=edits
what: the workspace dependency list with its pinned third-party ranges — edits: rmcp's entry in workspace.dependencies
@task_file n=5 path=Cargo.lock mark=edits
what: the lockfile of resolved dependency versions — edits: rmcp and rustix
@task_file n=6 path=deny.toml mark=edits
what: the supply-chain check's configuration, run by CI's supply-chain job — edits: the advisories ignore list
@task_item n=1 part=needs
The viola verify verb and its human output: the step line per probe, the fail word, the summary and summary line, a stopped or refused run.
@task_item n=2 part=needs
viola verify's refusals as the proof run may meet them: interrupted run, unclean fixture, version unreadable, CLI not found, home-tree check, and the stamps.json update refused.
@task_item n=3 part=needs
The CLI verification state, verified versus unverified-cli, which the stamps verify writes decide for the newest CLI, and the unverified-cli refusal hint that state drives.
@task_item n=4 part=needs
A readiness wait in the typed run stays a wait without animation: the step counter is the one progress pattern, no spinner or elapsed-time counter while verify waits for the input box.
@task_item n=5 part=needs
A readiness failure reported by verify names no internals: no screen literal, stack trace or path in the error or hint, ASCII-only words, messages on stderr and data on stdout.
@task_item n=6 part=needs
Indirect, through the agent's screen signatures: a readiness literal is a capability-ledger row with a viola verify probe, so a changed input-box signature changes what verify's probe proves and what the stamps attest.
@task_item n=7 part=needs
Indirect, through the agent's screen: reading the screen is the only Claude-facing change; how the agent is sent to or answered stays as it is.
@task_item n=8 part=needs
The not-delivered input-not-ready refusal and its hint, carried by sends that wait for the input box, only where research finds the run pump's readiness gate shares the same literal and failure.
@task_item n=9 part=needs
viola verify's exit codes for a run that passes, fails a probe or is refused, as the proof run reads them.
@task_standing n=1
convention: each new or re-recorded fake-agent screen fixture is a clean fixture verify accepts, recorded under its CLI version
home: surface
@task_standing n=2
convention: each new probe or step a verify run prints keeps the fixed step-line form and its fail word
home: surface-cli · surface
@task_item n=1 part=not_needed
The brand-identity rows on flight-progress strips, the readback box, handoff and pause or release transfer: verify draws no strip or board.
@task_item n=2 part=not_needed
The colour palette, typography, spacing and iconography rows: the chunk adds no colour, weight, column or glyph to any output.
@task_item n=3 part=not_needed
viola list's board and table rows, its columns, skip counts, name cut and reading age: the chunk does not touch the board.
@task_item n=4 part=not_needed
viola revive and viola run start refusals and output, and the revive --json refusal codes: no start path of a wrapped session changes.
@task_item n=5 part=not_needed
viola link, unlink, wait, last and release output rows, and the release --budget lines: none of those verbs changes.
@task_item n=6 part=not_needed
The exit-21 causes, peer reply screening, wrapper fault detail and wrapper socket rows: no instance or socket boundary changes.
@task_item n=7 part=not_needed
The web surface and rendered-page absences: viola has no page and the chunk adds none.
@task_item n=8 part=not_needed
viola --help verb groups and the hidden mcp and hook verbs: no verb is added, renamed or regrouped.
