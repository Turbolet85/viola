# Control — entry 13 (the gated reader of evidence/r7.md), typed beside the entry

The tool fires no deferred entry, so the control was typed in the tool's shell form, `/usr/bin/bash -o pipefail -c
'{command}'`, its command the entry's `run` with the record's path as the one operand changed (the input operand
alone), each record a scratch file under `$TMPDIR/{scratchpad}/ctl13/`:

- a record holding `r7: green placement=placed active=unchanged` → exit 0, last line `1` (the entry's atoms hold)
- a record holding `r7: red placement=placed active=moved` → exit 1, last line `0` (the entry reads red)
- no record → exit 1 (the entry reads red)
