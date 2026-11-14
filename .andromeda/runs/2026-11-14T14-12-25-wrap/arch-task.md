@task_prose part=session
The session added two unit tests to viola-hyprland, test-only, and changed no product line, no caller and no public API.
One test pins the hyprctl read's frame bound at the exact frame: a reply of exactly MAX_FRAME bytes reads as a parsed clients reply, one byte more reads as the oversize seam error with an oversize process-log detail.
The other pins the dispatch reply's frame bound at the exact frame: exactly MAX_FRAME bytes passes the size check and reads as refused, one byte more reads as oversize.
The read test drives the crate's own planted hyprctl stub, not the root fake agent, so the crate's own tests can kill a mutant of the bound; remove-the-guard controls on both bounds held.
No dependency, schema, config, crate, name, gate or CI surface moved; no capability was claimed or retired; no MAX_FRAME value changed.
@task_file n=1 path=crates/viola-hyprland/src/read.rs
what: viola-hyprland's read of hyprctl, the one compositor seam's bounded reader that spawns hyprctl directly and drains its reply under the MAX_FRAME cap, refusing an oversize reply as the oversize seam error
@task_file n=2 path=crates/viola-hyprland/src/reply.rs
what: viola-hyprland's reply parsing, the Hyprland seam's parse of the clients reply and of the dispatch reply, each refused as oversize past MAX_FRAME and read into the closed seam errors (malformed, refused, oversize)
@task_item n=1 part=touches
decision/bounded-readers: the Hyprland seam's two readers (the hyprctl read and the dispatch parse) now have their MAX_FRAME bound pinned at the exact frame by tests; the rule stands unchanged, only its witness grew.
@task_item n=2 part=touches
contract/hyprland-seam-errors/oversize: the oversize error is now proven to fire at MAX_FRAME plus one byte and not at MAX_FRAME, on both the read and the dispatch parse; the contract item's words stand.
@task_item n=3 part=touches
contract/hyprland-seam-errors/refused: a dispatch reply of exactly MAX_FRAME bytes that is not the success text reads as refused, pinned by a test; the item stands.
@task_item n=4 part=touches
decision/hyprland-seam-crate: the one compositor seam crate gained tests only; its boundary and dependency direction are unchanged.
@task_item n=5 part=touches
decision/hyprctl-direct-spawn: indirect, the read test plants a hyprctl stub on the crate's own spawn path; the direct spawn with a closed argv is unchanged.
@task_item n=6 part=touches
contract/wrapper-channel-frames/max-frame: indirect, the MAX_FRAME name is shared; the Hyprland seam's cap value did not move and the wrapper channel's frame cap is not reached.
@task_item n=7 part=touches
decision/mutation-at-epoch-boundary: indirect, the two bounds were boundary-audit survivors; the new tests are in-crate so the mutated package's own tests now kill a removed bound.
@task_item n=8 part=touches
decision/no-shell-out: indirect, the read test's stub is a test-only shell script standing in for hyprctl; no product shell-out was added.
@task_item n=1 part=untouched
The wrapper channel, its JSON-RPC frames, methods, fault codes and the viola mcp stdin line cap: not reached; the session follows the mcp stdin exact-frame test as precedent only.
@task_item n=2 part=untouched
CLI verbs, exit codes, --json documents and human output lines: no CLI surface changed.
@task_item n=3 part=untouched
Hooks, statusline, dialogs, the wheel, budget and send confirmation: not reached.
@task_item n=4 part=untouched
State on disk (events log, snapshots, stamps, config, diagnostics files) and the strict-modes rules: not reached.
@task_item n=5 part=untouched
Stack, dependency policy, CI workflow, coverage floors, pre-push gate and the occupied names: no dependency, gate, job, binary, crate or name was added, renamed or freed.
