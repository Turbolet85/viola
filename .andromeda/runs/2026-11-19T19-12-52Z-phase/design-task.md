@task_prose part=chunk
Adds `/context` to the compiled list of local commands a send confirms by its post-condition: one more local-command stdout tag in the session transcript after the send's own pre-paste end, confirmed the way `/mcp` is today.
Adds a capability-ledger row for that post-condition on `/context`, with its `viola verify` probe (a compiled-literal paste in an interactive verify run), the row stamped live on the installed agent CLI 2.1.328; whether the row is relied is left to P3.
A send of `/context` past the change answers confirmed by the post-condition, or `ok` with `confirmed: false` and detail `unconfirmable` where the post-condition is not measured on the version.
The fake agent rehearses the new paste and post-condition so CI proves the path without a live start; no live start, no re-pin, no documentation figure in this phase.
Carries two open readings to the entry's rig start (the agent prompt's placeholder-hint cell attributes, attributes only; the intermittent rig rehearsal red, acted on only at its next red).
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the capability ledger of the agent driver: the compiled local-command list with each command's delivery post-condition, and the capability rows a verify run stamps per CLI version — edits: the local-command list (a `/context` entry with the command-stdout post-condition), a new capability-ledger row for that post-condition on `/context`
@task_item n=1 part=needs
The send readback for a local-command send: `/context` gets a readback of the same kind `/mcp` has, read back only once its post-condition (one more stdout tag in the transcript) is seen.
@task_item n=2 part=needs
The unconfirmable send answer: where the `/context` post-condition is not measured on the running version, the send answers ok with confirmed false and detail unconfirmable, shown as the unconfirmable readback and its readback-mirror form, never as read back.
@task_item n=3 part=needs
No optimistic confirmation: nothing in the send's output announces `/context` delivered before the post-condition is measured.
@task_item n=4 part=needs
The send verb's human readback mirror and its --json result: a `/context` send prints through the same open, read-back and unconfirmable mirror forms and the one stdout document the send verb already has.
@task_item n=5 part=needs
The verify verb's output for the new probe: the `/context` probe appears as one more step line of an interactive verify run, counted in its summary, with its fail word, stamps-update refusal and stopped-run forms as any other step.
@task_item n=6 part=needs
The CLI verification state: the new row's live stamp on 2.1.328 feeds the verified and unverified-cli states a send reads; whether the row is relied (so later binaries read every stamped version unverified until re-stamped) is left to P3, and the unverified-cli refusal hint is the surface that answer reaches.
@task_item n=7 part=needs
The verify verb's step-counter progress: the new step stays inside the one progress pattern, with no spinner, no elapsed counter and no animation.
@task_item n=8 part=needs
ASCII-only human output and control-character escaping apply to every new line the send and verify verbs print for `/context`.
@task_item n=9 part=needs
Data on stdout, messages on stderr: any new readback, step or refusal line for `/context` keeps to the CLI streams split and the plain output modes.
@task_item n=10 part=needs
No internals in errors or hints: a `/context` refusal or unconfirmable answer names no transcript path, tag text or ledger internals.
@task_item n=11 part=needs
The send and verify command names stay as they are: `/context` is a payload of the send verb and a probe of the verify verb, never a new verb.
@task_standing n=1
convention: every send readback a new local command adds confirms only on its measured post-condition and otherwise reads unconfirmable, never success first
home: brand-identity
@task_standing n=2
convention: every new verify probe prints as a step line in the verify output's fixed step, summary and fail-word forms
home: surface-cli · surface
@task_item n=1 part=not_needed
The fleet board and flight-progress strip rows (STATUS, WHEEL, DIALOG, stale and cocked strips, list columns and board widths): the chunk adds no board column or strip state.
@task_item n=2 part=not_needed
The colour palette and colour depth rows: no new colour, accent or depth decision is made.
@task_item n=3 part=not_needed
Typography, spacing, border radius, depth, iconography glyph rows and the web surface: viola renders no page and the chunk adds no glyph or column.
@task_item n=4 part=not_needed
The run and revive start refusals, revive --json codes, exit-21 causes and strict-modes refusals: the chunk starts no instance and adds no start or home-tree refusal.
@task_item n=5 part=not_needed
The link, unlink, wait, last, pause, release, answer and plugin-install verb outputs: none of them carries a local-command send.
@task_item n=6 part=not_needed
The hidden mcp verb and its stdout: that verb is viola's own server, not the agent's `/mcp` local command the new entry is modelled on.
@task_item n=7 part=not_needed
The CLI refusal hints that never quote text: the placeholder-hint reading the chunk carries is of the agent's own prompt hint cells, not a viola hint line.
@task_item n=8 part=not_needed
The handoff display, transfer marker and pause or release as control transfer: a `/context` send transfers no control.
