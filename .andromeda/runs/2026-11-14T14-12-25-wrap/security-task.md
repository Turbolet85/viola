@task_prose part=session
The session is test-only: two new unit tests in the viola-hyprland crate pin its two MAX_FRAME bounds at the exact frame.
The hyprctl read test plants a hyprctl stub whose reply is exactly MAX_FRAME bytes (read whole, no parse rejection) and one byte more (refused as oversize, logged with detail oversize).
The dispatch parse test feeds exactly MAX_FRAME bytes (past the size check, refused as not the success text) and one byte more (refused as oversize).
No product line changed: both bound lines stand as they were, no caller changed, no dependency, schema, config or public API moved, nothing retired.
Remove-the-guard controls proved each test kills its bound: with either size check removed its test goes red; the crate's test count moved 82 to 84, which no row states.
@task_file n=1 path=crates/viola-hyprland/src/read.rs
what: the Hyprland seam's hyprctl read: the child bounded by a deadline, its stdout capped at MAX_FRAME and refused whole as oversize past it; the session added only a unit test of that cap
@task_file n=2 path=crates/viola-hyprland/src/reply.rs
what: the Hyprland reply parse: the MAX_FRAME size check before any parse of the clients reply and the dispatch reply, reply text dropped at the parse; the session added only a unit test of the dispatch cap
@task_item n=1 part=touches
The one external read cap and the shared MAX_FRAME cap (decision max-frame-cap, max-frame-shared-cap, reads-capped-at-max-frame, request-size-limits): the hyprctl reply read and the dispatch reply parse are now proven at exactly MAX_FRAME accepted and MAX_FRAME plus one refused; the cap value itself did not move.
@task_item n=2 part=touches
Bounded readers (decision bounded-readers-census, bounded-external-readers): the hyprctl reply reader stays a bounded reader; its bound is now witnessed by a test that drives the read's own drain past its cap, not only by a parse test.
@task_item n=3 part=touches
Child output bounded and tolerant (decision child-output-bounded, child-output-tolerant, child-output-tolerant-unknown): the hyprctl child's stdout bound is the one pinned; over-limit output is refused whole as oversize, never truncated.
@task_item n=4 part=touches
The hyprctl child deadline (decision hyprctl-read-deadline): the read's cap sits in the same bounded read as the deadline, whose own test is unchanged; no deadline value moved.
@task_item n=5 part=touches
Hyprland reply text dropped at the parse (decision hyprland-reply-text-dropped): an exactly-MAX_FRAME dispatch reply that is not the success text reads as a closed refusal, carrying no reply text.
@task_item n=6 part=touches
Mutation testing at the epoch audit (decision mutation-testing-at-audit): the two Hyprland MAX_FRAME size-check mutants are now killed by the crate's own unit tests, since the boundary audit runs only the mutated package's tests.
@task_item n=7 part=touches
MAX_FRAME checked against data and its revisit signal (decision max-frame-checked-against-ledger, max-frame-revisit-signal): an exactly-MAX_FRAME Hyprland reply is accepted, so the cap's edge is measured for this seam; no ledger value moved.
@task_item n=8 part=touches
The process log line for a refused hyprctl read (the oversize detail after process-start and process-exit) is the value the test asserts; no row's code word changed.
@task_item n=9 part=untouched
The MCP stdin line cap and its oversize-line contract, the channel frame cap, and the hook stdin oversize contract: the session follows the MCP exact-frame test as precedent but touches none of those readers.
@task_item n=10 part=untouched
Authentication and authorization: socket directory, peer euid, strict-modes checks and their refusal contracts are not reached.
@task_item n=11 part=untouched
Secret management, environment stripping and the artifact canary scan: no environment, credential or upload changed.
@task_item n=12 part=untouched
Dependency security: no crate, pin, version floor, licence, deny gate, workflow or toolchain changed.
@task_item n=13 part=untouched
Data protection: no fixture, recorder allowlist, log mode, pinned copy or state file changed.
@task_item n=14 part=untouched
Error handling and the CLI, MCP, channel and revive contracts: no error enum, code word, exit code or refusal message changed.
