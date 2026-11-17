# Entry 10's controls (revision 1), 2026-11-17T07:18:47Z

These were typed beside the entry, in `bash -o pipefail -c` from the root. Each differs from the entry's `run` by the
version operand alone. Entry 10 is a `round` leg (exempt from a baseline). These controls are its must-pass and
must-fail readings.

- As the entry (2.1.325): stdout `2.1.325 (Claude Code)` · exit 0 → `exit 0` and `contains 2.1.325` hold: green.
- Absent version (9.9.9): no output · exit 1 → `exit 0` fails: red. This is what turns the entry red when 2.1.325
  is uninstalled.
- Another installed version (2.1.326, mise's `latest`): stdout `2.1.326 (Claude Code)` · exit 0 → `contains 2.1.325`
  fails: red.
