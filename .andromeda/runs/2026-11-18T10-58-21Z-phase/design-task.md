@task_prose part=chunk
At viola run and viola revive (their one shared start), after the version gate and before the spawn, on a verified CLI, viola writes the CLI's per-project trust record (projects[key].hasTrustDialogAccepted, key the CLI's own) into the one Claude config file the started child reads, when no record already trusts the folder read the CLI's way.
The write takes the CLI's lock when free, replaces the file atomically, reads the record back, at most 3 attempts; any refused file form or a lost record writes nothing and leaves the trust dialog to the human, viola typing nothing.
One closed word per start reaches the run log, nothing of the file anywhere; every refusal and skip gets a case proving nothing was written.
It reuses the trust-record writer built for verify's trust-record row rather than adding a second writer, and reads (not re-derives) the measurement that no CLI flag or setting trusts a folder.
Live proof is S1 (viola run in a fresh untrusted folder under the rig's private config) with its control S2 (a plain claude in the same folder shape), asked as one block at the end of P3.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs
what: the trust record's shape and the key rule (the git root inside a repository, else the cwd) for the CLI's per-project trust record
@task_file n=2 path=src/trust_record.rs
what: the trust-record file write: the CLI's lock, atomic replace keeping the file's mode, the refusals and the closed code words, built for verify's trust-record row
@task_file n=3 path=src/bin/viola-fake-agent.rs
what: the fake agent, whose trust decision is read from the Claude config file so an e2e case of the write can fail when viola's write is gone
@task_item n=1 part=needs
The viola run and viola revive shared start: a new step after the version gate and before the child spawn that writes the trust record; what run and revive print once the child starts stays as it is.
@task_item n=2 part=needs
The CLI verification state, verified versus unverified-cli: the write happens only on a verified CLI; on an unverified CLI (a stamp taken before the 23rd ledger row) run is transport-only and writes no trust record, and the dialog stays the human's.
@task_item n=3 part=needs
viola run stays silent while wrapped: the write prints nothing to the terminal, no line on stdout or stderr, no hint, no progress; a refused or lost write is no start refusal and no exit code.
@task_item n=4 part=needs
One closed word per start in the run log naming what the write did (written, skipped, refused by form, lost race); the word set is closed and names nothing of the file: no path, no key, no folder, no file content.
@task_item n=5 part=needs
No internals in errors or output: no path, pid, config file contents or trust key ever appears in any human line, --json document or log detail the write produces.
@task_item n=6 part=needs
The refusals that write nothing (bytes unchanged, no temp sibling, no lock left): the file absent (never created), unreadable, a link, not the JSON expected (not an object, projects not an object, the entry not an object), oversize past the frame bound, and a record not read back after the 3rd attempt.
@task_item n=7 part=needs
The skips that are no refusal: an unverified CLI, a folder the exact key or a parent already trusts (read the CLI's way: exact key, then the parent walk up to the git root inside a repository or to / outside one), and the home folder.
@task_item n=8 part=needs
The bound on the write: only projects[key].hasTrustDialogAccepted for one folder; no other key changed, no entry removed, nothing for the home folder; the file is CLAUDE_CONFIG_DIR/.claude.json when viola's home keeps that variable for the child (claude_env_keep), else the user's ~/.claude.json, viola reading that one environment value for that purpose only.
@task_item n=9 part=needs
viola types no key into a trust dialog: a lost or refused write leaves the dialog to the human; no emulated key press replaces the write.
@task_item n=10 part=needs
The trust-record row of viola verify output, which names nothing of the file, keeps its wording; the product write reuses the same writer and its closed code words.
@task_item n=11 part=needs
No interactive prompt or pager is added: the write asks the human nothing at start.
@task_item n=12 part=needs
Run log detail codes: the write's closed word joins the run log as a detail code like the existing log detail codes, from a closed list, never free text.
@task_item n=13 part=needs
New tests of the write: each refusal and skip a case proving nothing was written; an e2e case on the fake agent that fails when viola's write is gone; any waiting test bound under the 20 s lint; each control's timeout named and fired; no mutation run.
@task_standing n=1
convention: each new run log closed word is a fixed code word from a closed list, carrying no path, pid or internal detail
home: surface
@task_standing n=2
convention: each new human-facing line or log word is ASCII-only, with control characters escaped and no internals
home: surface-cli · anti-patterns-never-do-these
@task_item n=1 part=not_needed
Brand identity strips, cocked arrival strip, readback box and send readback rows: the write shows no strip, box or send readback.
@task_item n=2 part=not_needed
Colour palette, accent and amber rules, typography, spacing and iconography: the write prints nothing on the terminal.
@task_item n=3 part=not_needed
The viola list board and its table rows (columns, NAME cut, skip counts, row order, BAY line): the write changes nothing a board shows.
@task_item n=4 part=not_needed
viola send readback mirror, viola wait output, viola last output and viola link or unlink output rows: verbs the chunk does not touch.
@task_item n=5 part=not_needed
exit 21 causes, wrapper fault --json detail and peer reply screening rows: no instance-reach or wrapper-socket path is changed.
@task_item n=6 part=not_needed
revive --json refusal codes and run start refusal rows: the write adds no start refusal and no new refusal code.
@task_item n=7 part=not_needed
The web-spa surface and the rendered-page absence: viola has no page.
@task_item n=8 part=not_needed
Self-validation protocol tests (squint, swap, readback box signature): no visible design element is added.
@task_item n=9 part=not_needed
The browser-tools prompt as a modal, the live home's re-stamp and recorded fixtures, readings on 2.1.326 or 2.1.327, and a re-pin of the product the overseer drives with: named out of scope.
