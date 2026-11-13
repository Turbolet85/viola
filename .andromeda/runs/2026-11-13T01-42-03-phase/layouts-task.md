@task_prose part=chunk
viola verify's typed run (the probe paste, then the long, tag-like, newline, /compact and local-command pastes) decides the input box is up from the input box itself, no longer from a footer hint literal that a transient footer notice (Update available!, a usage warning) displaces after a turn.
The readiness literal is a screen signature with its capability-ledger row and a viola verify probe; the fake agent's screens and fixtures gain a replayed footer notice.
It re-measures: one viola verify run on the newest installed claude CLI reading every ledger row pass but the account-dependent statusline-rate-limits, its stamps written by verify itself (a named operator step awaiting the founder's live-start cap).
It also turns CI's supply-chain job (cargo deny advisories against rmcp 3.4.1 and rustix 1.1.5) green again, by a dependency bump or a reasoned per-advisory ignore.
@task_file n=1 path=src/cmd/verify/typed/turns.rs mark=edits
what: viola verify's typed run, its turns and pastes and the input-box readiness check before each paste — edits: input_box_up and its callers framing_turns, turn's settle, Run's box_wait
@task_file n=2 path=crates/viola-agent-claude/src/screen.rs mark=edits
what: the claude agent's screen signatures, the readiness literals backed by capability-ledger rows — edits: SIGNATURES.input_box
@task_file n=3 path=Cargo.toml mark=edits
what: the workspace manifest, its workspace dependencies and rmcp's ratified version range — edits: the rmcp entry of workspace.dependencies
@task_file n=4 path=Cargo.lock mark=edits
what: the lockfile pinning resolved dependency versions — edits: the rmcp and rustix entries
@task_file n=5 path=deny.toml mark=edits
what: cargo deny's supply-chain policy that CI's supply-chain job checks — edits: the advisories ignore list
@task_item n=1 part=needs
viola verify's human output during the typed run: its step lines, failing-row lines, summary line and exit 0 or exit 1 outcome, which the proof run must read as every row pass but statusline-rate-limits.
@task_item n=2 part=needs
viola verify's stop and refusal outcomes (CLI not found, version unreadable, recorded fixture not clean, verify stopped) that the typed run can still reach when the input box never comes up.
@task_item n=3 part=needs
viola verify --record and its recorded fixtures per CLI version, which the replayed footer-notice screens join.
@task_item n=4 part=needs
viola verify's output stays plain ASCII with no spinner, no progress glyphs and no persistent footer, whatever wait the new readiness check adds before a paste.
@task_item n=5 part=needs
The input-box readiness state of the claude screen: up, covered by a modal, or displaced by a transient footer notice after a turn.
@task_item n=6 part=needs
The capability ledger's stamps, written only by viola verify, for the newest installed CLI version.
@task_item n=7 part=needs
Indirect: a readiness literal is a capability-ledger row probed by viola verify, so a changed input-box signature changes what verify's output reports as a row.
@task_item n=8 part=needs
Indirect: the /compact and local-command pastes of the typed run, whose readbacks for /compact and /clear are the verify probes of the send readback words.
@task_item n=9 part=needs
CI's supply-chain job and its cargo deny advisories check, green as the acceptance of the supply-chain item.
@task_item n=10 part=needs
The rmcp dependency that viola mcp's server rests on: a bump past the ratified range must leave viola mcp's tools and tool results unchanged.
@task_standing n=1
convention: each new or changed viola verify step, failing-row or stop line the typed run prints keeps viola verify's ratified output form: plain ASCII, results on stdout and the rest on stderr, words carrying the state, no colour-only signal
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page, and this chunk adds none.
@task_item n=2 part=not_needed
The viola list board and its BAY line, columns, colour depth, stale rows and --json document: the chunk changes no board output.
@task_item n=3 part=not_needed
viola revive and viola run start output and refusals, and their --json error codes: no start path changes.
@task_item n=4 part=not_needed
viola link and unlink output, wheel verb output (pause, release, release --budget) and viola answer output: none is touched.
@task_item n=5 part=not_needed
viola plugin install output and refusals: the plugin install path is not reached.
@task_item n=6 part=not_needed
The frame-verb exit-21 hints and the instance-unreachable --json detail kinds: no wrapper reachability path changes.
@task_item n=7 part=not_needed
CLI token rendering rows (colour, spacing, radius, motion and type roles): verify's output is plain and gains no styled element.
@task_item n=8 part=not_needed
viola wait and viola last output: the chunk changes no turn-ended, dialog or message reporting.
