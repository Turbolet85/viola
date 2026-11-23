@task_prose part=chunk
Adds `monitors`, a new query, to the Hyprland seam's closed request set: its closed argv builder and a parse that drops reply text except what a closed per-request keep-list names. The test-only tap records it read-only with no window, and the fake agent standing as hyprctl replays it.
Adds ONE closed move-to-workspace dispatch: a validated window address and an integer workspace are all that enter its argv, and it is no exec. Its exact argv, recorded fixture and replay case come from the one live reading R1.
Settles the hyprctl read deadline at 5 s, with a test that times a real read against it, and gives the hyprctl reply bounded-reader row a witness that drives the read's own stdout drain past MAX_FRAME.
Implement builds and greens everything that needs no live window first, then stops ONCE before R1. At R1 the tap opens one foot window with no claude in it, moves it once, kills the foot pid it started and leaves only fixture files. The chunk claims no capability and adds no verb.
@task_file n=1 path=crates/viola-hyprland mark=edits
what: viola-hyprland, the one compositor seam crate: its closed Request set, the WORDS list, the closed argv builders, the reply parses, and the hyprctl read with its deadline — edits: the Request set (a `monitors` query and one move-to-workspace dispatch), WORDS, the argv builders, the parses and their keep-lists, the read deadline HYPRCTL_DEADLINE
@task_file n=2 path=tests/contract_bounded_readers.rs mark=edits
what: the bounded-readers contract check, one Boundary row per bounded reader naming its witness test — edits: the `hyprctl reply` Boundary row's witness
@task_file n=3 path=fixtures/hyprland/0.56.2 mark=edits
what: the recorded Hyprland reply fixtures for one compositor version, which the fake agent replays by exact argv — edits: a `monitors` recording and the move-to-workspace dispatch recording
@task_item n=1 part=needs
The Hyprland seam's closed request set: a new `monitors` query with its own closed argv builder and parse, hyprctl spawned directly with a closed argv and never through a shell.
@task_item n=2 part=needs
One closed move-to-workspace dispatch: only a validated window address and an integer workspace enter its argv, it is no exec, and nothing else can be dispatched through the seam.
@task_item n=3 part=needs
Reply parses of the new request: compositor reply text is untrusted upstream text, parsed tolerantly, and dropped at the parse except for the fields a closed per-request keep-list names.
@task_item n=4 part=needs
The Hyprland seam's closed error words (unreachable, refused, unsupported, malformed, oversize): the new request and dispatch report only through these. A new closed word would be a fork for the operator, not something this chunk adds.
@task_item n=5 part=needs
The hyprctl read deadline: settled at 5 s (no longer provisional), with a test that times a real read against it rather than only driving a shorter deadline.
@task_item n=6 part=needs
Bounded readers at every boundary: the hyprctl reply read drains stdout under the frame cap MAX_FRAME, and the new witness drives that drain past the cap and gets oversize.
@task_item n=7 part=needs
viola-hypr-tap, the test-only recorder binary: it records the `monitors` reply read-only with no window. Its R1 window verb runs only at R1, opening one foot window with no claude in it, moving it once, killing exactly the pid it started and writing only fixture files.
@task_item n=8 part=needs
The fake agent standing as hyprctl: it replays recorded replies by exact argv in its replay modes (recorded, oversize, malformed, absent), with replay cases for the `monitors` query and the dispatch.
@task_item n=9 part=needs
Recorded Hyprland fixtures: scrubbed and hygiene-walked before they are kept, holding only what the parse keeps, with no username, home path, absolute path or other personal detail.
@task_item n=10 part=needs
Crate dependency direction: viola-hyprland stays the one compositor seam, and in this chunk it gets no product caller. It stays a root dev-dependency only.
@task_item n=11 part=needs
Window features target Hyprland: the move-to-workspace dispatch is the compositor's own dispatch on the measured version, and its argv is measured live at R1, never guessed.
@task_item n=12 part=needs
Test-only binaries and test-support code (the tap, the fake agent's hyprctl replay) stay out of release and product builds.
@task_item n=13 part=needs
No live compositor or window in CI: the seam's tests run against the fake agent's replay. A live reading (R1 and R3) runs only on the operator's host, and in this phase only a read-only compositor query with nothing dispatched is allowed.
@task_item n=14 part=needs
The seam's typed errors: one thiserror enum in the crate, with messages that interpolate nothing, so no compositor reply text or window address reaches an error surface.
@task_item n=15 part=needs
Timed tests: the deadline test that times a read stays inside the shared root test wait bound, and it uses no timer tool.
@task_standing n=1
convention: each new test that cannot run off Linux or off a live compositor is gated with its reason named, and no test gate names another OS
home: cross-cutting-patterns
@task_standing n=2
convention: each new recorded fixture is scrubbed and hygiene-walked, and is recorded only from a clean run of its recorder
home: occupied-resources
@task_standing n=3
convention: each new test runs in CI on ubuntu against the fake agent, never against a live claude or compositor, and stays inside the coverage floors and the format and clippy gate
home: ci-cd-approach
@task_standing n=4
convention: each new test-only binary verb or test seam is kept out of release and product builds and is no configuration channel
home: occupied-resources
@task_item n=1 part=not_needed
CLI verbs, their arguments, exit codes and --json documents: the chunk adds no verb. `viola open` and its flags belong to a later chunk.
@task_item n=2 part=not_needed
The MCP server's tools, results and errors: nothing in the chunk reaches the MCP surface.
@task_item n=3 part=not_needed
The wrapper channel's methods, frames, params and fault codes: the seam talks to hyprctl, not to a wrapper.
@task_item n=4 part=not_needed
Normalised event kinds, event data and wrapper event appends: the chunk records no window and appends no event.
@task_item n=5 part=not_needed
Instance snapshot fields and the list payload or board: a window kind and snapshot field belong to `viola open`.
@task_item n=6 part=not_needed
Wheel, send, paste, dialog hold and answer rules: the chunk sends no key and drives no claude session.
@task_item n=7 part=not_needed
The capability ledger, ledger stamps and viola verify's refusals and exit details: the chunk claims no capability.
@task_item n=8 part=not_needed
Revive, the launch pair, lineage, links and the claude child's argv and environment: no start or revive path is touched.
@task_item n=9 part=not_needed
config.json keys, the plugin and its install, and the PTY seam: none of them carries the compositor seam.
@task_item n=10 part=not_needed
Trust writer code words and the hook contract and its exits: the chunk touches no hook and no claude config.
