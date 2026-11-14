@task_prose part=chunk
Pins by tests the exact-frame case on both MAX_FRAME bounds of viola-hyprland, the one compositor seam: a reply of exactly MAX_FRAME bytes is accepted, one byte more is oversize.
Site one is the bounded read of a hyprctl reply (read_within, driven through the fake hyprctl); site two is the dispatch reply parse (parse_dispatch).
Each new guard test carries a one-off remove-the-guard control that mutates its site (> to >=) and must see that test fail, so the next code audit catches both survivors.
Test-only: no product line changes unless a test proves one wrong; parse_clients' bound stays as already pinned and is not re-pinned.
@task_file n=1 path=crates/viola-hyprland/src/read.rs mark=edits
what: the Hyprland seam's bounded read of a hyprctl reply under MAX_FRAME, with its test module — edits: the test module (a new exact-frame test of read_within through the fake hyprctl)
@task_file n=2 path=crates/viola-hyprland/src/reply.rs mark=edits
what: the Hyprland seam's reply parsers (parse_clients, parse_dispatch) and their MAX_FRAME bound, with its test module — edits: the test module (a new exact-frame test of parse_dispatch)
@task_item n=1 part=needs
The bounded read's exact frame: a hyprctl reply of exactly MAX_FRAME bytes is not oversize, one of MAX_FRAME + 1 bytes is the seam's oversize error; the child's stdout is drained through MAX_FRAME + 1.
@task_item n=2 part=needs
The dispatch parse's exact frame: exactly MAX_FRAME bytes are not oversize and read as refused (they are not the success text); MAX_FRAME + 1 bytes are the seam's oversize error.
@task_item n=3 part=needs
The Hyprland seam errors as the tests assert them: oversize for one byte past the frame, refused for a full frame that is not the dispatch success text.
@task_item n=4 part=needs
The bounded-reader rule over the compositor boundary: every reply read from hyprctl is held to MAX_FRAME, and the new tests pin that bound at its exact edge on both sites.
@task_item n=5 part=needs
The read's test reaches hyprctl only through the fake hyprctl the read's tests already use: hyprctl spawned directly with a closed argv, no real hyprctl, no live desktop step, no live claude start.
@task_item n=6 part=needs
viola-hyprland stays the one compositor seam; the new tests live in the two existing test modules of the seam and add no crate, dependency or product surface.
@task_item n=7 part=needs
Each new guard test owes a one-off remove-the-guard control: the site's > mutated to >=, the case named by the test's name alone, reached past fail-fast under a kill deadline; it holds only when that case ran and failed.
@task_item n=8 part=needs
No mutation run inside the chunk, local or dispatched: mutation testing stays at the epoch boundary, and the declined viola-e2e and viola mutation units stay out.
@task_item n=9 part=needs
Test-only change: no product line is edited unless a new test proves it wrong; a defect found then folds in as its own inferred item with its own acceptance.
@task_item n=10 part=needs
parse_clients' MAX_FRAME bound is already pinned by its exact-frame test and is not re-pinned; any other MAX_FRAME comparison in the seam lacking an exact-frame case is read at P3.
@task_item n=11 part=needs
No requirement added and no capability claimed: the window features that target Hyprland keep their present claims.
@task_item n=12 part=needs
Every statement about how hyprctl, a crate or the compositor behaves is measured at P3 and cited, or marked unmeasurable.
@task_standing n=1
convention: tests run against the fake agent and fakes only; no real claude or live compositor in CI
home: ci-cd-approach
@task_standing n=2
convention: new test code passes formatting and clippy with warnings denied, honouring the print, dbg and tracing-level macro bans
home: build-system
@task_standing n=3
convention: a test gated to an OS names its reason, and no test gate names another OS
home: cross-cutting-patterns
@task_standing n=4
convention: new tests count toward the coverage floors the harness gate enforces
home: ci-cd-approach
@task_item n=1 part=not_needed
The wrapper channel's frame bound and its JSON-RPC framing: a different MAX_FRAME over the wrapper channel, not the compositor reply; the chunk touches no channel frame.
@task_item n=2 part=not_needed
The budget gate, budget readings and the statusline wrapper: the Epoch 2 theme leaving later, not this test-only chunk.
@task_item n=3 part=not_needed
CLI verbs, exit codes, --json documents and human output lines: the chunk adds no verb, document or line.
@task_item n=4 part=not_needed
MCP tools, the MCP server and its stdin line cap: the chunk touches no MCP surface.
@task_item n=5 part=not_needed
Hooks, dialogs, the wheel and link recording: no hook tier, dialog or wheel rule is reached.
@task_item n=6 part=not_needed
The events log, instance snapshots, ledger stamps and their replay: the chunk writes and reads no state file.
@task_item n=7 part=not_needed
The PTY seam, the vt100 readiness gate and send confirmation: the compositor seam's tests reach none of them.
@task_item n=8 part=not_needed
Release, install, deployment and the pinned binary copy: no build or install output changes.
