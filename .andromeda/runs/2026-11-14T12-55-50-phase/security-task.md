@task_prose part=chunk
Pins by tests the exact-frame case on both MAX_FRAME bounds of the viola-hyprland seam: a reply of exactly MAX_FRAME bytes is accepted, one byte more is oversize.
The bounded hyprctl read (read_within) is driven through the fake hyprctl with MAX_FRAME and MAX_FRAME + 1 bytes; parse_dispatch is given MAX_FRAME bytes (refused, not oversize) and MAX_FRAME + 1 (oversize).
Each new guard test carries a one-off remove-the-guard control on its site (the > to >= mutation), run through the gate's control step under a kill deadline; no mutation run, local or dispatched.
Test-only: no product line changes unless a test proves one wrong; parse_clients' bound is not re-pinned, its existing exact-frame test is re-read as holding it.
@task_file n=1 path=crates/viola-hyprland/src/read.rs mark=edits
what: the Hyprland seam's bounded hyprctl read — the child's stdout drained through MAX_FRAME + 1 and refused as oversize past MAX_FRAME — edits: its test module (a new exact-frame test of read_within through the fake hyprctl)
@task_file n=2 path=crates/viola-hyprland/src/reply.rs mark=edits
what: the Hyprland seam's reply parsers, parse_dispatch and parse_clients, each refusing a reply past MAX_FRAME as oversize — edits: its test module (a new exact-frame test of parse_dispatch)
@task_item n=1 part=needs
The one shared MAX_FRAME external read cap on the bounded hyprctl read: a reply of exactly MAX_FRAME bytes is read whole and accepted, MAX_FRAME + 1 bytes is refused as oversize; the test names the shared constant, never a literal, and leaves the cap's value as it is.
@task_item n=2 part=needs
The MAX_FRAME cap on the dispatch reply parse: exactly MAX_FRAME bytes is not oversize (refused as not the success text), MAX_FRAME + 1 bytes is oversize.
@task_item n=3 part=needs
Every product reader stays bounded: the hyprctl read keeps its cap before and while the child's output is read, and the bounded readers census still names it as bounded; a product line changed only if a test proves it wrong, then folded as its own item with its own acceptance.
@task_item n=4 part=needs
The hyprctl child bounded by a deadline: the read's new test runs the fake hyprctl under the read's existing deadline and must not hang it.
@task_item n=5 part=needs
Child output bounded and parsed tolerantly: an oversize or unrecognised hyprctl reply ends in a closed error outcome (oversize, refused), never a panic and never a partial accept.
@task_item n=6 part=needs
Hyprland reply text dropped at the parse: the refused and oversize outcomes carry no reply text, and the new tests assert on the closed outcome, not on upstream bytes.
@task_item n=7 part=needs
Only the fake hyprctl: no real hyprctl, no live desktop step and no live claude start; CI holds no secrets and the real claude CLI stays local only.
@task_item n=8 part=needs
Test seams stay compiled out of release: the new tests and anything they use live in the test modules or behind the existing test-only feature, adding no seam to the product build and no test-only feature to the release graph.
@task_item n=9 part=needs
Mutation testing belongs to the epoch audit: the chunk runs no mutation tool and uploads no mutants output; each guard's remove-the-guard control is a one-off check of the site, its case the test's name alone, holding only where that case ran and failed.
@task_item n=10 part=needs
The sync crates stay free of tokio: the new tests in the Hyprland seam crate add no async runtime.
@task_item n=11 part=needs
No new dependency, dev-dependency or feature: the root graph still passes the cargo-deny gate, exact pins and the crates.io-only source rule unchanged.
@task_item n=12 part=needs
parse_clients' MAX_FRAME bound is re-read as already pinned by its existing exact-frame test, which must still pass unchanged.
@task_standing n=1
convention: each new test that needs a home or scratch directory uses its own, never the user's real home
home: secret-management
@task_standing n=2
convention: a test removes only the directories it created itself
home: security-anti-patterns
@task_standing n=3
convention: any recorded or committed fixture is scrubbed and reviewed; test reply data is synthetic, holding no path, user name or address
home: security-anti-patterns-never-do-these
@task_standing n=4
convention: test homes and scratch output stay out of git
home: bootstrap-phases
@task_item n=1 part=not_needed
Peer euid checks, socket directory and socket file modes, server verification and strict-modes refusal rows: the chunk opens no channel, socket or viola home.
@task_item n=2 part=not_needed
Wrapper channel methods, channel frame contracts and channel error codes: the chunk sends and reads no channel frame.
@task_item n=3 part=not_needed
MCP tools, MCP stdin line cap and rmcp feature rows: the chunk touches no MCP surface.
@task_item n=4 part=not_needed
Hook payload, hook stdin and hook-decision rows: the chunk touches no hook.
@task_item n=5 part=not_needed
CLI exit codes, CLI --json error documents and revive codes: the chunk adds or changes no CLI verb or output.
@task_item n=6 part=not_needed
Paste, send and answer text character sets and their refusal words: the chunk types no text into a terminal.
@task_item n=7 part=not_needed
Statusline command and budget.json rows: the chunk reads and writes neither.
@task_item n=8 part=not_needed
Pinned binary, plugin file and Claude-home rows: the chunk places, re-hashes or writes none of them.
@task_item n=9 part=not_needed
viola verify probe, stamps and screen-signature rows: the chunk runs no verify probe and reads no PTY screen.
@task_item n=10 part=not_needed
Event log, diagnostics log and log rotation rows: the chunk adds no log line.
@task_item n=11 part=not_needed
GitHub Actions SHA pins, workflow permissions, release signing and attestation rows: the chunk changes no workflow and no release step.
@task_item n=12 part=not_needed
Child environment credential strip rows: the chunk starts no claude child.
