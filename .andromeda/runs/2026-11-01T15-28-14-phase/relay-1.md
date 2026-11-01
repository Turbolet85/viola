# The P4 dialog, chunk 2026-11-01-unix-endpoint-and-home-hardening

Three forks, asked in one dialog at P4. Each option as shown; the answer and its authority as the dialog returned them.

## Q1. Socket word
Founder fork, a boundary widening (playbook: Boundary widening). A refused socket directory has no code today: at a start it
is an anyhow error at the catch site. Which word does it carry on exit 21, MCP and the log? A new closed cause tells a
same-user driver one more check that refused it.
- New word (Recommended): `socket-dir-refused`: the sixth exit-21 cause on CLI and MCP (`detail`), the eleventh code of
  `revive --json` at exit 1, and a word on `process-exit` and `hook-decision`, no path. Fits Founder Direction 4 (a distinct code
  per cause) and the catalogue's one hint per code. It crosses to the driver the way your Q1 of 2026-10-31 let the other five.
- Reuse strict-modes-failed: No new word crosses. But that word says the viola home or state files are at fault, which this
  directory is not (it lies outside the home), and on exit 21's `process-exit` no arm writes it today. Needs two hints on one
  code or a hint that misdirects.
- New word, CLI only: MCP reads `instance-dead` for it; no new word reaches a driver. MCP's body then stops equalling the CLI
  document less `v`, the parity rule of test-plan §6, for this one cause.

Answer: New word. Authority: "the founder's own word, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given
after this widening was shown to him".

## Q2. Euid word
Founder fork, a boundary widening. A client meeting a server of another euid: which word? `server-verify-failed` is recorded
«off macOS» and your 2026-10-31 carve-out leaves macOS's server check here. The pid and start-time comparison stays absent on
macOS in every option.
- Reuse server-verify (Recommended): The euid mismatch reads `server-verify-failed`, on Linux and macOS: its hint (`the process
  on <name>'s endpoint is not its recorded wrapper; it was not contacted`) stays true, and no new word crosses to a driver. The
  word's «off macOS» is restated: on macOS it means an euid mismatch alone, the comparison staying the named gap.
- New word: `peer-euid-mismatch`: its own hint and a seventh cause. Tells the two checks apart, and keeps `server-verify-failed`
  off macOS as written, at the price of one more word crossing to a driver for a state one account cannot reach.

Answer: Reuse server-verify. Authority: "the founder's own word, live, 2026-11-01 before 16:27 Z, through the overseer's dialog,
given after this widening was shown to him".

## Q3. Board rule
The board's one home rule. `board::scan` is shared code, and the same two-rule split stands on Windows: a home with a read grant
to another SID passes an empty board and is refused once it holds an instance. Does the read rule move on every OS?
- Every OS (Recommended): The home takes the read rule on all three: one rule in fact, the same home read the same way by the
  board and by a start, whose home-tree check already holds it. The one Windows reading that moves: a read-granted home holding
  no instance is now refused, witnessed on windows-2025 with `grant_read`. `instances/` stays the write rule; pairs, exit 21 and
  no-line stand.
- Unix only: The read rule under `cfg(unix)`; Windows keeps its split, so the CARRY stays half open and needs a new owner on the
  route. No Windows reading moves.

Answer: Every OS. Authority: "the overseer's technical answer, founder-delegated".
