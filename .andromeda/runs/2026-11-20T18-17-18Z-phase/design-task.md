@task_prose part=chunk
Two behaviours of the claude CLI that the product already relies on become relied rows of the capability ledger, each with its own viola verify probe.
The first is the harness prompt prefixes: the set of raw prompt starts that files a prompt as a harness turn, one that never moves the wheel. Its probe has a second session send into a verify child and reads where the prompt's raw text starts.
The second is the identity floor: the eleven CLAUDE parent-identity environment names that are always stripped from the child's environment. Its probe captures environment variable names only, never their values.
The new rows are counted in a viola verify stamp from a live run, made only in the rig's scratch home. The live home's stamps and the pinned product stay untouched.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the capability ledger of the claude agent — edits: the relied-row list, the stamp's relied-id set, the verify probes of the new rows
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases.rs mark=edits
what: the capability ledger's own test cases, which hold the ledger's rows, runs, checks and stamps
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the hook handling of the claude agent, which holds the harness prompt prefix literals and the hidden hook capture arm with its closed ids — edits: the harness-prefix literals, the capture arm
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs mark=edits
what: the claude agent's crate root, which holds the identity floor of eleven parent-identity environment names stripped from the child's environment — edits: IDENTITY_FLOOR
@task_item n=1 part=needs
The viola verify run gains two probe steps. Each new step prints in the same step-line form as the existing steps, and the summary line's relied-row count grows from 27 to 29.
@task_item n=2 part=needs
The viola verify stamp counts the two new relied rows. A stamp that lacks them reads as a stamp that does not cover every relied row, so the CLI verification state the stamp feeds into, verified or unverified-cli, follows the larger set.
@task_item n=3 part=needs
The automatic-verify line that viola run and revive print (verifying, no stamp, earlier red, did not verify) works over the larger relied-row set without any change to its form.
@task_item n=4 part=needs
The hidden hook verb's capture arm may gain closed ids for the new probes. The hook verb's exit code and its stderr contract still hold, and the hook keeps its place in the colour decision order: it never colours.
@task_item n=5 part=needs
The identity-floor probe captures environment variable names only. No value may reach a stamp, a fixture, a log line or any screen output. If a variable's name itself carries private text, the record scrub must handle it, and no internals may appear in errors or hints.
@task_item n=6 part=needs
viola verify --record writes a recorded fixture set that includes the new probes. The unclean-fixture refusal and the stamps.json update refusal still apply to that set.
@task_item n=7 part=needs
Any refusal or failure on a new probe uses the existing viola verify forms: the fail word, the stopped or refused run, the interrupted run, CLI not found, version unreadable. It adds no new message shape.
@task_item n=8 part=needs
viola verify output stays ASCII-only and plain: data goes to stdout and messages to stderr, the step counter is the only progress pattern, there is no spinner, and a new step line shows no check-mark or emoji prefix.
@task_item n=9 part=needs
The harness-prefix probe must not print a compiled screen literal in any human-facing line. Hints never quote text, and screen content is never echoed.
@task_item n=10 part=needs
Behaviour of the harness prefixes: a prompt that starts with a harness prefix is a harness turn and never moves the wheel. The probe proves this behaviour is still there; it does not change what the wheel shows.
@task_item n=11 part=needs
Behaviour of the identity floor: the child's environment is always stripped of the eleven names. The probe proves the stripping still holds and does not widen or narrow the eleven names.
@task_item n=12 part=needs
The trust-record row of the viola verify output still names nothing private, even after the new rows are added.
@task_standing n=1
convention: every new viola verify step and summary line follows the verify output's step-line and summary-line forms, in ASCII only, with the step counter as its only progress pattern
home: surface-cli
@task_standing n=2
convention: every new refusal or failure line names no internals, path or pid, and goes to stderr
home: anti-patterns-never-do-these
@task_item n=1 part=not_needed
The flight-progress strip, the readback box, the send readback and the handoff display. The chunk adds ledger rows and verify probes and does not change how a session is shown or how a send is read back.
@task_item n=2 part=not_needed
The viola list board and table: its columns, NAME width and cut, skip counts, row escaping and the strict-modes scan. Nothing this chunk adds is shown on the board.
@task_item n=3 part=not_needed
The colour palette, colour depth and colour tokens (amber, dim, callsign, handoff blue, stale). Verify's lines gain no colour.
@task_item n=4 part=not_needed
The viola revive --json refusal codes, the exit-21 causes and the wrapper fault detail codes. The chunk adds no refusal code and no exit code.
@task_item n=5 part=not_needed
The viola link, unlink, wait, last and send mirror outputs, and the peer reply screening. None of them reads or prints a ledger row.
@task_item n=6 part=not_needed
The run and revive start refusals (already-live, squatted-name, stale-heartbeat, pinned-hash-mismatch, socket directory). A verify probe child is not a run or revive start.
@task_item n=7 part=not_needed
Typography, spacing, iconography and the rendered-page and web-surface absences. The chunk draws no new glyph, column or page.
@task_item n=8 part=not_needed
The brand-identity rows on strips: the cocked strip, dimmed stale strips, stable slots and strip density. They govern the board, not verify.
