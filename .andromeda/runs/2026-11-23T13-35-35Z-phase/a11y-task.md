@task_prose part=chunk
Adds `monitors`, a new query, to the Hyprland seam's closed request set, with its closed argv builder and a parse that drops reply text except what a closed per-request keep-list names; the test-only tap records it read-only with no window, and the fake agent standing as hyprctl replays it.
Adds one closed move-to-workspace dispatch that moves a window address to an integer workspace. Only a validated address and an integer enter its argv, and it is not an exec. Its exact argv, recorded fixture and replay case come from R1, the one live reading.
Settles the hyprctl read deadline at 5 s, with a test that times a real read against it, and gives the hyprctl reply boundary row a witness that drives the read's own stdout drain past MAX_FRAME.
Implement builds and greens everything that needs no live window first, then stops once, before R1. The chunk claims no capability and adds no verb, no product caller and no window record.
@task_file n=1 path=crates/viola-hyprland/src/lib.rs mark=edits
what: the Hyprland seam's closed set: its Request enum, the WORDS list, the closed argv builders and the closed HyprlandError words — edits: the Request set (monitors, move-to-workspace dispatch), WORDS, the argv builders
@task_file n=2 path=crates/viola-hyprland/src/reply.rs mark=edits
what: the seam's reply parses, which drop reply text at the parse save a closed keep-list — edits: a monitors parse and its per-request keep-list, the dispatch parse for the move
@task_file n=3 path=crates/viola-hyprland/src/read.rs mark=edits
what: the seam's bounded hyprctl read, its deadline HYPRCTL_DEADLINE and its stdout drain — edits: the deadline, settled at 5 s, and the timed deadline test
@task_file n=4 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs mark=edits
what: the test-only viola-hypr-tap recorder of hyprctl replies into fixtures — edits: a monitors record, the R1 window verb (open one foot window, move it once, end its own pid)
@task_file n=5 path=src/bin/viola-fake-agent/hyprctl.rs mark=edits
what: the fake agent's hyprctl replay by exact argv in its recorded, oversize, malformed and absent modes — edits: replay cases for monitors and the move dispatch
@task_file n=6 path=fixtures/hyprland/0.56.2/Monitors.recorded-1.json
what: the recorded fixture of the monitors query on Hyprland 0.56.2, written by the tap
@task_file n=7 path=tests/contract_bounded_readers.rs mark=edits
what: the bounded-readers contract test and its Boundary rows — edits: the hyprctl reply Boundary row's witness, now a test that drains stdout past MAX_FRAME
@task_item n=1 part=needs
The seam's closed request set grows by exactly two: the monitors query and one move-to-workspace dispatch. Each has a closed argv builder and a closed parse. Any request beyond these two is an operator fork, written and stopped on.
@task_item n=2 part=needs
Reply text is dropped at the parse except what a closed per-request keep-list names, so no hyprctl reply text travels past the seam. Peer replies are screened down to closed code words.
@task_item n=3 part=needs
The closed error words stay as they are. A new closed word is an operator fork, not something the chunk adds. Unlisted code words are not coined to cover the new requests.
@task_item n=4 part=needs
The move dispatch's argv takes only a validated window address and an integer workspace, and is never an exec. Its exact form on Hyprland 0.56.2 is what R1 measures.
@task_item n=5 part=needs
The hyprctl read deadline is settled at 5 s by the operator's answer. It is a time bound on the driver (the read of a compositor reply), never on a human, and the human sees no timeout from it because the seam has no product caller.
@task_item n=6 part=needs
A test times a real read against the 5 s deadline, not the short read_within bound the current deadline test uses. A time bound beyond the settled 5 s is an operator fork.
@task_item n=7 part=needs
The drain cap gets a witness: a test drives the read's own stdout drain past MAX_FRAME. It replaces the parse test the hyprctl reply Boundary row names today.
@task_item n=8 part=needs
The tap records monitors read-only with no window (R3, allowed in implement). The R1 window verb opens one foot window with no claude in it, moves it once, ends exactly the foot pid it started and leaves only fixture files. It runs only at R1, at a moment the operator names.
@task_item n=9 part=needs
The fake agent standing as hyprctl replays the new requests by exact argv in each of its replay modes (recorded, oversize, malformed, absent), so tests reach the seam through recorded fixtures and never through a live compositor.
@task_item n=10 part=needs
Live readings never gate: R1 and R3 are measurements that write fixtures. CI and the tests run against recorded fixtures only, and the chunk's wrap waits for R1 without any test depending on a live window.
@task_item n=11 part=needs
The seam stays an internal crate with no product caller, a root dev-dependency only. The carve-out that internal crates are not assertable for accessibility applies, so no output-discipline assertions are owed for the new requests.
@task_item n=12 part=needs
Invariants of the new requests, the deadline and the drain are held in crate tests that the mutation audit would cover. This phase itself runs no mutation tier.
@task_item n=13 part=needs
This phase makes no live start, opens, moves, focuses or closes no window, sends no key, runs no mutation tier and calls no timer tool. The only allowed live call is a read-only compositor query that dispatches nothing.
@task_standing n=1
convention: each new test runs inside the existing nextest suites, and its failure is a nextest failure, with no separate driver or suite of its own
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: each new test, the timed deadline test included, is deterministic: zero flakiness, no retries
home: 10-slo-invariants-a11y-budgets
@task_standing n=3
convention: no new test is skipped or ignored in the CI test gate
home: 1-a11y-scope-summary
@task_standing n=4
convention: each new test counts toward the CI gate that requires coverage and doctest
home: ci-integration
@task_item n=1 part=not_needed
Colour, SGR, bold, dim and NAME-truncation rules of the list board: the chunk adds no output that a human sees.
@task_item n=2 part=not_needed
The TUI passthrough contracts and the rules on the wheel, human keystrokes and terminal sequences: the chunk wraps no session, drives no outer PTY and sends no key.
@task_item n=3 part=not_needed
CLI refusal, --json failure document and exit-code contracts: the chunk adds no verb and no CLI caller of the seam.
@task_item n=4 part=not_needed
The revive, link, unlink, plugin install and start-against-a-holder refusal contracts: those verbs are untouched.
@task_item n=5 part=not_needed
Screen-reader targets and the manual Orca and keyboard passes: R1's foot window is a fixture measurement with no claude in it, not an accessibility pass.
@task_item n=6 part=not_needed
Help text, board captions and fixed CLI copy that comes from design: no fixed CLI string is added or changed.
@task_item n=7 part=not_needed
Hook-call, MCP frame-tool and frame-verb refusal contracts, along with the socket-directory and strict-modes refusals: the channel and the socket are not touched.
@task_item n=8 part=not_needed
The absences of a web or graphical frontend, a touch surface and internationalization: nothing here adds any of them.
