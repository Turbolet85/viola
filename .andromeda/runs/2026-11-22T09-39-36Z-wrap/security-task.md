@task_prose part=session
The session measured whether real claude on Linux loses a key typed right after a resize of its own pty, with no product change: the live-round rig gained a KR arm (labels KR, KR-2, KR8, KR8-2) that resizes only its own pty and types up to 13 x and 13 DEL under the grant box,resize.
On real claude 2.1.330 (KR) and 2.1.328 (the spare KR8-2) every one of 12 keys typed 0, 20 or 100 ms after a resize was kept; KR8 was cut at round 8 as unverified by the rig's own settle, which this session mended for KR alone (a byte drained before the key never settles it); no start read lost, so the driver-paste fix was not needed.
The KR arm reads its box row's cell attributes only, never text, and its summary and ledger hold closed words, counts, delays and settle times; kept frames stay 0600 under the gitignored rig output dir; the arm is refused under CI.
The rig's rehearse verb gained two rehearse-only options for the kr arm, --quiet-ms (100 to 400) and --kr-del-seam withheld or late; any other arm or form, and any live start carrying one, is refused usage with exit 2 before any ledger line.
The operator's wrap answers: the live rig's kept-stamp row states its rule and names no labels; the cross-session fixture's runtime-dir path is an accepted residual with its own security-plan row; the four short except cells stand; a route entry for a bounded paste write follows this chunk.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the live-round rig itself, test-only: its arms (HB, KR and the earlier ones), its grants, its settle, its summary and ledger of closed words and counts
@task_file n=2 path=crates/viola-e2e/src/rig/keys.rs
what: the live-round rig's key grants, which say what keys the rig may type into its own pty and now whether it may resize that pty
@task_file n=3 path=crates/viola-e2e/src/rig/screen.rs
what: the live-round rig's screen model, which reads its own pty's box row by cell attributes and now follows a resize
@task_file n=4 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live-round rig's command line, its live and rehearse verbs, their grants, caps and refused usage
@task_file n=5 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsals over the fake agent, which hold each arm's reading and its teardown before any live start
@task_item n=1 part=touches
decision/live-round-rig-keys-own-pty: the KR arm resizes its own pty up to 12 times and types up to 13 x and 13 DEL per start into it, only under the grant box,resize, else refused grant.
@task_item n=2 part=touches
decision/live-rig-hb-arm-keeps-attributes-only: the KR arm, like HB, keeps cell attributes and counts only, plus delays and settle times; its own row stands beside this one.
@task_item n=3 part=touches
decision/live-rig-kept-stamp-owner-only: on the operator's word the row states the owner-only seeded-stamp rule and names no arm labels; the stamp read unmoved before and after KR8-2.
@task_item n=4 part=touches
decision/verify-cross-session-probe-residuals and decision/verify-runtime-dir-held-in-memory: the operator accepted the cross-session fixture's runtime-dir path as a residual, its own row proposed beside these.
@task_item n=5 part=touches
decision/live-session-screens-kept-local: KR and KR8-2 frames kept 0600 under the gitignored rig output dir, never read into evidence, a message or the tree.
@task_item n=6 part=touches
decision/live-rig-isolated-child, decision/live-rig-credentials-copy and decision/live-rig-seed-from-bundle-logic: the KR starts ran the same isolated child with a seeded config and a transient credentials copy; hygiene read clean with 0 host paths kept.
@task_item n=7 part=touches
decision/real-cli-local-only and decision/ci-no-secrets-no-real-claude: the KR live arm is refused under CI; its rehearsals run over the fake agent only.
@task_item n=8 part=touches
decision/test-seams-fake-agent-only and decision/new-env-read-needs-log-entry: the rig's rehearse verb gained the kr seam option and a quiet option, rehearse-only, which a live start refuses as usage, exit 2, before any ledger line.
@task_item n=9 part=touches
decision/live-rig-prompt-capture-closed: the KR summary keys (control, rounds, kept, lost, unread, kept by delay, lost-at, kr-reading, sizes, dirty-box, late-redraw, key-refusal) are closed words and counts only.
@task_item n=10 part=touches
decision/verify-screen-read-against-literals and decision/input-box-readiness-from-literals: the KR arm refuses shape on a trust dialog, a modal or a box not up on its first screen, read from the rig's attribute words.
@task_item n=11 part=touches
decision/plugin-install-live-round-rig: the live rounds' isolation and ledger admit under a cap and a spare held for KR8-2 (cap 2, spare 1), one new out dir, none gone.
@task_fact n=1 slug=authentication-authorization
fact: The live rig's KR arm keeps only its box row's cell attributes, counts, delays and settle times, resizing and typing only into its own pty under the grant box,resize; it reads no screen text.
@task_fact n=2 slug=input-validation
fact: The live rig's quiet and kr seam options are accepted by the rehearse verb with the kr arm alone; any other arm, any value outside the closed form, or any live start carrying one is refused usage, exit 2, before any ledger line.
@task_fact n=3 slug=data-protection
fact: The cross-session fixture's runtime-dir path is an accepted residual, on the operator's word.
@task_item n=1 part=untouched
The product's IPC channel, socket directory, peer euid and server-verify rows: no product crate changed.
@task_item n=2 part=untouched
The strict-modes, home-tree and stamp-writer rows of the product: viola's own state handling did not move.
@task_item n=3 part=untouched
The bracketed paste, paste character set and send refusal rows: the driver-paste fix was not needed and the product's send path is unchanged; the bounded paste write is a later route entry.
@task_item n=4 part=untouched
The MCP tools, channel frames, exit codes and refusal contracts: no contract item moved.
@task_item n=5 part=untouched
Dependency security, toolchain pins, CI workflows and the artifact canary scan: no dependency, no CI step, no workflow changed.
@task_item n=6 part=untouched
Configuration, environment and secret rows: no config key, env var or product flag added.
@task_item n=7 part=untouched
The absences: no capability added or removed in viola itself.
