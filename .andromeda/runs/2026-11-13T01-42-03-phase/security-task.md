@task_prose part=chunk
Changes how `viola verify`'s typed run (Run B: the probe paste, then the long, tag-like, newline, `/compact` and local-command pastes) decides the input box is up: from the input box itself, not from a footer literal that a transient footer notice («Update available!», a usage warning) displaces after a turn.
Changes the claude adapter's input-box screen signature and its capability-ledger row(s), and the fake agent's screens and recorded fixtures that replay a footer notice.
Re-measures with one live `viola verify` on the newest installed claude CLI, every ledger row reading pass but the account-dependent statusline rate-limits row, stamps written for that CLI version by verify itself; each live start is a named operator step waiting on the founder's per-chunk cap.
Turns CI's supply-chain job (cargo deny advisories) green: three RustSec advisories against rmcp 3.4.1 and one against rustix 1.1.5, by a bump past rmcp's ratified range or a reasoned deny ignore per advisory, rustix by a lockfile bump.
@task_file n=1 path=src/cmd/verify/typed/turns.rs mark=edits
what: viola verify's typed run, its turns and pastes and how it reads the screen for readiness — edits: the function `input_box_up`, its callers `framing_turns`, `turn`'s settle and `Run`'s `box_wait`
@task_file n=2 path=crates/viola-agent-claude/src/screen.rs mark=edits
what: the claude adapter's compiled screen signatures, the literals verify and the screen model read the PTY output against — edits: the table entry `SIGNATURES.input_box`
@task_file n=3 path=Cargo.toml mark=edits
what: the workspace manifest, its third-party dependency pins and rmcp's ratified version range and feature set — edits: the `[workspace.dependencies]` rmcp entry
@task_file n=4 path=Cargo.lock mark=edits
what: the root dependency graph's lockfile that cargo deny audits — edits: the rmcp and rustix lock entries
@task_file n=5 path=deny.toml mark=edits
what: the cargo-deny gate's configuration over the root graph — edits: the advisories `ignore` list
@task_file n=6 path=ledger/stamps.json
what: the verify stamps file, per claude CLI version, written only by viola verify
@task_item n=1 part=needs
The readiness check verify uses before each typed paste: screen reads only against compiled signature literals, never free text from the screen.
@task_item n=2 part=needs
The claude adapter's capability ledger: a readiness literal is a ledger row with a viola verify probe, so a changed input-box signature changes its row(s) and its probe.
@task_item n=3 part=needs
Verify never types into a dialog and never answers one: the new readiness check must still see a modal on screen as not-ready, so no paste byte lands in a dialog.
@task_item n=4 part=needs
Verify's pastes are each one bracketed paste, typed only once the input box is up.
@task_item n=5 part=needs
The screen model the readiness check reads: the vt100 feed kept bounded, isolated from passthrough and under a panic catch.
@task_item n=6 part=needs
Verify's PTY output read bounded and caught, and verify's child claude spawned directly, in a pre-trusted probe folder whose directories and transient captures are removed.
@task_item n=7 part=needs
The stamps file: written only by viola verify, checked before and under lock, touched only after the strict-modes check; the proof's stamps come from verify itself, never by hand.
@task_item n=8 part=needs
The real claude CLI runs only locally, never in CI; CI holds no secrets and replays the fake agent's recorded screens instead; verify's real-CLI transcripts are an accepted risk.
@task_item n=9 part=needs
Live claude starts (the proof run, any probe of the footer notice) are named operator steps waiting on the founder's per-chunk cap; no mutation run.
@task_item n=10 part=needs
The fake agent's screens and recorded fixtures under the per-CLI-version fixture folder that replay a footer notice: scrubbed or refused whole, keeping only signature rows, reviewed before commit; verify's unclean-fixture refusal codes apply.
@task_item n=11 part=needs
The fake agent's test seams: compiled out of release, present only under the fake-agent feature.
@task_item n=12 part=needs
The cargo-deny gate on the root graph and CI's supply-chain job: advisories must clear, the plan choosing between no advisory ignores and a reasoned ignore per advisory.
@task_item n=13 part=needs
Exact third-party version pins and rmcp's ratified range: a bump past it means the wrap reconciles that pin decision.
@task_item n=14 part=needs
rmcp's pinned feature set and its banned HTTP server and auth features: the three advisories hit the OAuth client and Streamable HTTP, features the workspace does not enable; the critical-CVE response for a disabled versus an enabled feature applies.
@task_item n=15 part=needs
Any bumped or newly locked crate (rmcp, rustix, through tempfile) still passes the licence allow list, the crates.io-only source rule, the C-building-crate ban and the no-tokio-in-sync-crates rule.
@task_item n=16 part=needs
Manual dependency update review, and the weekly scheduled advisory run that reports the same advisories.
@task_item n=17 part=needs
viola run's pump readiness gate for driver sends stays out unless research shows it shares the same literal and failure; then it is named, not silently widened.
@task_standing n=1
convention: each new or re-recorded screen fixture is scrubbed of paths, user names and emails, keeps only signature rows, and is reviewed before commit
home: data-protection
@task_standing n=2
convention: each new test runs under its own viola home, never the user's, and leaves no local home in git
home: secret-management
@task_standing n=3
convention: each new test seam sits only under the fake-agent feature and is compiled out of release
home: security-anti-patterns
@task_standing n=4
convention: each new seam or environment read needs a log entry
home: security-anti-patterns-never-do-these
@task_item n=1 part=not_needed
The wrapper's channel methods, channel frames, channel JSON-RPC errors and the IPC client refusals: the chunk adds or changes no channel method or frame.
@task_item n=2 part=not_needed
The socket directory, socket file modes and peer-euid checks of the authentication area: no listener or client connection changes.
@task_item n=3 part=not_needed
The hook contract — hook stdin, hook decisions, hook capture arm and its silent exit: the chunk does not touch viola hook.
@task_item n=4 part=not_needed
viola revive's arguments, its JSON codes and the start collision check: not reached.
@task_item n=5 part=not_needed
budget.json, the statusline command and its shell-out, and the config.json keys: not reached.
@task_item n=6 part=not_needed
link and unlink, their event lines and exit codes: not reached.
@task_item n=7 part=not_needed
The viola send refusals and the human-typing wheel: the chunk's pastes are verify's own, not viola send.
@task_item n=8 part=not_needed
Plugin install, the marketplace tree and pinned binary placement and re-hash: not reached.
@task_item n=9 part=not_needed
Release attestations, signed self-update, the release workflow's cache rules: there is no release stage and the chunk ships none.
@task_item n=10 part=not_needed
Workflow hardening — actions pinned by SHA, least workflow permissions, zizmor: the chunk edits no workflow.
@task_item n=11 part=not_needed
The secret-management credential strip and the hermetic pre-push child environment: the chunk changes no environment handling.
