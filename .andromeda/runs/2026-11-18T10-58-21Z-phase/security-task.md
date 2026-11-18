@task_prose part=chunk
Adds the product write of the Claude CLI's per-project trust record at a wrapped start: at viola run and viola revive (their one shared start), after the version gate and before the spawn, on a verified CLI, when no record already trusts the child's folder read the CLI's way, viola writes projects[<key>].hasTrustDialogAccepted into the one Claude config file the started child will read.
The write takes the CLI's lock when free, replaces the file atomically keeping its mode, reads the record back, at most 3 attempts; a refused file form or a lost record writes nothing more and leaves the trust dialog to the human, viola typing nothing.
One closed code word per start reaches the run log, nothing of the file anywhere; every refusal form and skip gets a case proving nothing was written, and an e2e case on the fake agent that fails when the write is gone.
Reuses the trust-record writer built for verify's Run F rather than adding a second writer; reads back, never re-derives, the sealed measurement that no flag or setting trusts a folder and the live pass of the trust-record ledger row on 2.1.325.
The live proof is S1 (viola run in a fresh untrusted folder under the rig's private config) with its control S2 (a plain claude in the same folder shape), shaped and capped at the end of P3; claims matrix v2-33.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs
what: the Claude adapter's trust record shape and the key rule (the git root inside a repository, else the cwd), the trust reading the product write reuses
@task_file n=2 path=crates/viola-agent-claude/src/trust_record.rs
what: the trust record writer — the file write under the CLI's lock, atomic replace keeping the file's mode, its refusals and its closed code words — that the product write reuses
@task_file n=3 path=src/bin/viola-fake-agent
what: the fake agent, whose trust decision is read from the Claude config file (its trust-config reading and its trust dialog screen)
@task_item n=1 part=needs
The boundary widening itself: viola now writes into the Claude CLI's own config file, against the rows that say viola writes nothing under the Claude home or into Claude's config; those rows are amended by this entry's wrap as Expected amendments, and the widening takes its Decisions Log entry, bound as ratified (R2).
@task_item n=2 part=needs
What the write may touch: ONLY projects[<key>].hasTrustDialogAccepted for one folder, no other key changed, no entry removed, nothing written for the home folder, and a file never created when absent.
@task_item n=3 part=needs
Where the write sits in the start: only at run and revive, after the version gate and before the spawn, only on a verified CLI as the unverified-CLI gate and the version gate's strict stamps read decide; a home whose stamp predates the 23rd ledger row reads unverified and writes no trust record.
@task_item n=4 part=needs
The key and the trust reading: the key is the CLI's own (git root inside a repository, else the cwd), and trust is read as the CLI reads it — the exact key, then the parent walk to the git root or to /, so no record is written for a folder a parent already trusts.
@task_item n=5 part=needs
Which file: the child's config file is <CLAUDE_CONFIG_DIR>/.claude.json when viola's home keeps that variable for the child through config.json claude_env_keep, else the user's ~/.claude.json; viola reads that ONE environment value for that one purpose, a new environment read that owes its log entry and whose value is never logged.
@task_item n=6 part=needs
The refusals, each writing nothing: the file absent, unreadable, a link, not the JSON expected (not an object, projects not an object, the entry not an object), or oversize past MAX_FRAME; the read is bounded at MAX_FRAME with the default JSON depth limit and joins the bounded product reader census.
@task_item n=7 part=needs
The race (R3): take the CLI's own lock when free, replace the file atomically through the one replace helper keeping its mode, read the record back before the spawn, at most 3 attempts; the residual of a concurrent write lost in the window is accepted and named in the row.
@task_item n=8 part=needs
On any refusal, skip or lost record the start goes on and the trust dialog stays the human's: viola types no byte into the dialog and the refusal never blocks the human.
@task_item n=9 part=needs
The run log carries one closed code word per start from the trust record's closed code words; no path, key, file content or CLAUDE value reaches any log, error message, stderr or output.
@task_item n=10 part=needs
The trust record writer guarded as it stands is reused, not duplicated: one writer serves verify's Run F and the product start, and its guards hold for both callers.
@task_item n=11 part=needs
Each refusal and skip case proves nothing was written: the file's bytes unchanged, no temp sibling left, no lock left — covering absent, unreadable, link, the three wrong JSON shapes, oversize, a lost race after the 3rd attempt, an unverified CLI, a folder the exact key or a parent already trusts, and the home folder.
@task_item n=12 part=needs
The e2e case reaches the write through the fake agent, which reads trust from the Claude config file, so the case FAILS when viola's write is gone; a change to the fake's screens or its trust reading puts cli_verify in the gate block, and any fake seam stays under the fake-agent feature, compiled out of release.
@task_item n=13 part=needs
The live arms S1 and S2 run under the rig's private config, the child isolated from the user's config and seeded from bundle logic only, with one other project's entry as sentinel and the browser-tools key claudeInChromeDefaultEnabled seeded; the rig keys only into its own pty, the stop the standing Ctrl-C never while a dialog stands, else the no-key group kill.
@task_item n=14 part=needs
The live predicates each a reading can FAIL: S1 RED when the first screen reads trust, the sentinel differs by a byte, or the written record is not exactly the one key; S2 RED when the first screen does not read trust; both RED on a missing teardown word; S1's write goes only to the rig's private config file, the founder's real ~/.claude.json read only by the mark witness.
@task_item n=15 part=needs
The live screens and evidence of S1 and S2 stay local, any rig credentials copy transient and unrecorded, and a spare start is spent only on a cause a start itself showed, after its mend lands through a plan revision.
@task_item n=16 part=needs
Each control's timeout named in the plan's prose and fired with that flag, ids within 16 characters, holds meaning the cases ran and FAILED with no TIMEOUT line, each control saying what the subject does when the guard is gone; no mutation run.
@task_item n=17 part=needs
A test that waits stays under the 20 s lint bound; a test-support helper spawning an instrumented binary under a cleared environment runs it in a scratch directory; a gate entry naming an installed tool's path is checked against the host.
@task_item n=18 part=needs
The write acts within the same OS user trust boundary: tampering with the Claude config file by the same user stays outside the boundary, and the strict-modes checks on viola's own home still run before the start reads its state.
@task_standing n=1
convention: each new test runs under its own viola home and its own scratch Claude config, never the user's
home: secret-management
@task_standing n=2
convention: each test removes only the directories it created and follows only a safe linked base for the e2e home
home: security-anti-patterns-never-do-these · security-anti-patterns
@task_standing n=3
convention: each new test seam lives only under the fake-agent feature and is compiled out of release
home: security-anti-patterns · input-validation
@task_standing n=4
convention: each new log line carries closed code words only, nothing of user content, paths or CLAUDE values
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=5
convention: each new error message is fixed and interpolates nothing, its writer named
home: error-handling
@task_item n=1 part=not_needed
MCP tools, the stdio MCP server and its structuredContent results: the chunk adds no tool and changes no MCP answer.
@task_item n=2 part=not_needed
The wrapper's channel methods, channel frames, JSON-RPC error codes and the client seam check order: the write sits before the spawn and sends no frame.
@task_item n=3 part=not_needed
Socket directory, socket mode, peer euid checks and the endpoint bind: the chunk changes no listener or connect.
@task_item n=4 part=not_needed
Paste and send text character sets, typed text normalisation and the stdin non-editing list: viola types nothing in this chunk.
@task_item n=5 part=not_needed
The hook contract, hook payload parsing and hook decision details: the write runs at the start, not in the hook.
@task_item n=6 part=not_needed
Recorded fixtures, their scrub and the unclean-fixture refusal codes: the 2.1.325 recorded fixture set and the live home's re-stamp are another entry's.
@task_item n=7 part=not_needed
Pinned binary and plugin placement, re-hash and marketplace rewrite: the chunk re-pins nothing and leaves the product the overseer drives with untouched.
@task_item n=8 part=not_needed
viola list, the board scan, link and unlink and their event lines: none of them is reached.
@task_item n=9 part=not_needed
Readings on 2.1.326 or 2.1.327 and the browser-tools prompt read as a modal: both stand with other entries.
@task_item n=10 part=not_needed
Release, signing, deploy, hosting and the release workflow's attestations: the chunk adds no release stage.
@task_item n=11 part=not_needed
API-error classes, Hyprland recorder strings and the transcript read: the write touches none of those child outputs.
