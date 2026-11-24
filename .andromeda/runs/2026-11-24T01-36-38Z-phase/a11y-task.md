@task_prose part=chunk
Adds a new `viola focus <name>` CLI verb: resolve the home, validate the name, read the instance's state under the strict-modes checks, find the session's window and bring it forward on its workspace through the closed Hyprland request set only.
Window identity comes from the window recorded for a session `viola open` started, or for a hand-started `viola run` from the wrapper's process ancestry matched against the compositor's client list.
Every no-window outcome ends in a named refusal, and outside Hyprland the verb refuses as unsupported; refusal words, exits and hints are added beside the existing refusal, exit-code and help tables.
Tests run on the fake compositor's recorded replies and the fake agent; no live start, window, dispatch or key in this phase.
@task_file n=1 path=src/cmd mark=edits
what: the root CLI's verb set, the served verbs the help lists and the exit-cause table each verb's refusals print from — edits: a new focus verb, its help row, its refusal codes and exits
@task_item n=1 part=needs
The new verb appears in `viola --help` as a served verb with its own row under the existing group labels and usage line.
@task_item n=2 part=needs
Every focus refusal (no session, no window found, a recorded window that is gone, unsupported outside Hyprland) prints in the CLI refusal form: an unable line with its kebab-case reason code beside a plain word, then exactly one hint line, on stderr.
@task_item n=3 part=needs
Each new refusal code carries its one hint, no hint names a release, and an unlisted code word stays visible; the `unknown` refusal keeps no hint.
@task_item n=4 part=needs
Each focus refusal maps to an exit code from the existing CLI exit-code table (usage error exit 2 for a bad name, the instance-unreachable and refusal exits as they stand), or a new code entered beside them.
@task_item n=5 part=needs
Under --json a focus failure is one clean failure document with no hint and no SGR, its code and exit matching the human form.
@task_item n=6 part=needs
An unstattable or wrongly-moded home, or socket directory, is a strict-modes refusal before any read of state or any Hyprland request, with its instance-unreachable detail word.
@task_item n=7 part=needs
The name printed in any focus line or refusal is the parsed, validated argument, never a value read back from the compositor or the snapshot.
@task_item n=8 part=needs
Any text from the compositor's replies or the snapshot (window address, workspace name, client title) that reaches output is screened: C0 and C1 control characters printed escaped, compositor replies screened to code words.
@task_item n=9 part=needs
Focus output is static and linear: a single result or refusal in one write, no spinner, no progress line, no interactive prompt.
@task_item n=10 part=needs
Failure and refusal words are never coloured; no SGR off a colour-capable terminal or under --json, and any SGR only beside its word.
@task_item n=11 part=needs
No time bound falls on the human: any wait on the compositor's reply is a bound on the driver only, and no new time bound is added while that question stands open.
@task_item n=12 part=needs
Error messages carry a word, detail and hint shape; thiserror messages interpolate nothing and no stack trace or bare status is the error text.
@task_item n=13 part=needs
Each fixed focus string (refusal words, hints, help row) comes from the design copy and is pinned by its literal in a test.
@task_item n=14 part=needs
Focus moves the operator's window focus only through the compositor; it writes no byte to any wrapped session's terminal and sends it no key.
@task_item n=15 part=needs
Clauses the fakes cannot carry (a live window raised on the host) are a live reading that never gates and is planned gated on the operator's grant.
@task_standing n=1
convention: each new a11y assertion lives inside the existing test suites, not a separate a11y driver or suite
home: 1-a11y-scope-summary · 9-ci-integration
@task_standing n=2
convention: each new test runs under nextest with no skip, no retry and zero flakiness; its failure fails the build
home: 10-slo-invariants-a11y-budgets
@task_standing n=3
convention: each new fixed CLI string is pinned by its literal in a test and its refusal layout held by literal pins
home: 8-cognitive-accessibility · cognitive-accessibility
@task_standing n=4
convention: each new test is sequenced on records from the fakes and never inspects claude's content
home: keyboard-test-harness
@task_item n=1 part=not_needed
The TUI passthrough contracts of `viola run` (entry, human keystrokes, focus, mouse and resize sequences, vt100 feed panic, run diagnostics): focus never enters the wrapped session's terminal stream.
@task_item n=2 part=not_needed
The `viola list` board rows (caption order, colour, stale-row dim, NAME truncation, unwrapped separator, empty board): focus prints no board.
@task_item n=3 part=not_needed
The `viola revive` refusals, --list output and start-against-a-holder refusals: focus starts and revives nothing.
@task_item n=4 part=not_needed
The `viola link` and `viola unlink`, `viola plugin install`, `viola send`, `viola wait` and `viola verify` output rows: other verbs whose output focus does not change.
@task_item n=5 part=not_needed
The hook event and dialog call rows and the dialog hold in config: focus raises no dialog and answers none.
@task_item n=6 part=not_needed
The manual screen-reader and keyboard passes and their Orca setups: supplemental, gating nothing this chunk builds.
@task_item n=7 part=not_needed
The absences of a web or graphical frontend, a touch surface, internationalization and a rendered page: none reached by a CLI verb.
@task_item n=8 part=not_needed
The structured violation JSON schema rows: the CI report shape is unchanged by a new verb.
