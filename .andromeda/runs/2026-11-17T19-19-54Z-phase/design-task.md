@task_prose part=chunk
Adds the 23rd relied capability-ledger row on the Claude side: the claude CLI honours a per-project trust record (hasTrustDialogAccepted under the CLI's own project key) that another process wrote into the config file its child reads.
Gives that row its viola verify probe: a sixth interactive verify run that writes the trust record for its own OS-temp dir, expects the child's first settled frame to be no trust dialog, and removes only its own record on every exit path; a control with the record withheld must read the dialog.
Builds the trust-record writer once (lock when free, atomic replace, read-back, at most 3 attempts), where a later product write can reuse it; the fake agent learns to read the record, not argv alone.
Re-runs the step-count sweep the sixth run moves over every spelling of the count, and stamps the row with one live viola verify (S0, plus one spare) on the 2.1.325 install under the rig's private config.
Changes nothing of viola run or revive; the product's own trust write and its claim belong to a later entry.
@task_item n=1 part=needs
The viola verify verb (name/command/verify) gains a sixth interactive run, so its step count grows by one; every step line, the summary and the summary line must carry the new count in their fixed forms.
@task_item n=2 part=needs
The new probe's row in viola verify output: pass only when the child's first settled frame is no trust dialog with the record written; fail when the CLI does not honour the record, printed with the verify fail word.
@task_item n=3 part=needs
When the record cannot be written (config file absent, unreadable, a link, not the JSON expected, or oversize), nothing is written and the probe's row reads a closed word naming what it could not do, never pass; no internals, paths or stack traces in that line.
@task_item n=4 part=needs
The verify summary prints what proves verify's own trust record is gone after the run, as a plain ASCII line within the summary's form.
@task_item n=5 part=needs
Interrupted or stopped verify runs (viola verify refusals: interrupted run; verify output: stopped or refused run): the stop still removes verify's own record, and what a SIGKILLed verify leaves is stated.
@task_item n=6 part=needs
CLI verification state: once the 23rd row is relied, every 22-row stamp reads unverified-cli, including the 2.1.325 one; the unverified-cli state and its refusal hint are what a not re-stamped home now shows.
@task_item n=7 part=needs
stamps.json update after the live S0 verify: the stamp now covers 23 rows; the stamps-update refusal and stamps-refused output forms still apply if the write is refused.
@task_item n=8 part=needs
Indirect, through the verify refusals the run already makes: the home-tree check, unclean fixture, CLI not found and version unreadable refusals keep their forms with the sixth run added; the new run's temp dir must leave the fixture clean.
@task_item n=9 part=needs
Indirect, through the step counter: verify keeps the step counter as its one progress pattern, with no spinner or elapsed-time counter for the new run.
@task_item n=10 part=needs
Indirect, through verify's output streams: step lines and the summary stay on their existing streams, ASCII-only, with control characters escaped, no colour beyond the existing tokens and no interactive prompt.
@task_item n=11 part=needs
The trust dialog stays the human's: verify presses no key into any dialog, and the probe only observes whether the dialog appears; viola writes into the user's claude config only verify's own bounded record, never another.
@task_item n=12 part=needs
The verify --help text names the run set, so the sixth run and its probe appear there if the help lists runs.
@task_standing n=1
convention: each new verify step line, fail line or summary line follows the fixed viola verify output forms, ASCII-only, on its existing stream
home: surface-cli · surface
@task_standing n=2
convention: each new human-output line carries no internals, paths, pids or stack traces, and uses no check-mark or emoji prefix
home: anti-patterns-never-do-these
@task_item n=1 part=not_needed
The flight-progress strip, the readback box, handoff display and the fleet board (viola list table and board, strip density, stable slots, stale dimming, cocked strip): the chunk changes no board or strip.
@task_item n=2 part=not_needed
Send readback and the viola send mirror, viola wait output, viola last output and peer reply screening: no send, wait or last path is touched.
@task_item n=3 part=not_needed
viola run and viola revive output, their start refusals, --json refusal codes and exit-21 causes: run and revive do not change in this chunk.
@task_item n=4 part=not_needed
viola link, unlink, pause, release, answer and plugin install outputs, and the wrapper fault detail and wrapper socket rows: none of those verbs is reached.
@task_item n=5 part=not_needed
The hook and mcp verbs' streams and exit codes: the chunk adds nothing to either hidden or hook verb.
@task_item n=6 part=not_needed
Web surface, rendered page, typography, spacing, radius and depth rows: viola renders no page and the chunk adds none.
