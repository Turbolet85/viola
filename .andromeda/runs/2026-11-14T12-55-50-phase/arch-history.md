@history_item n=1 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Standard Contracts: 6 rows) — added `contract/hyprland-seam-errors/oversize` · `refused` · `malformed` (plus `unsupported`, `unreachable`) and `decision/hyprctl-direct-spawn`; oversize is "the reply passed the frame cap and was refused whole before the exit was read", each a fixed message interpolating nothing. Both new exact-frame tests pin the boundary of `oversize` on `read_within` and `parse_dispatch`, and the dispatch case's at-cap reply must read `refused` (not the success text): the tests must assert these exact closed codes. The fake `hyprctl` stays a direct spawn with a closed argv and no shell.

@history_item n=2 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Infrastructure Patterns: `+ decision/hyprland-seam-crate`) — viola-hyprland is the one compositor seam: sync, tokio-free, depends on viola-core alone among viola crates, guarded by the cargo tree check and the per-sync-crate tokio ban. The chunk's tests live in `crates/viola-hyprland/src/read.rs` / `reply.rs` test modules; any test helper or dev-dependency added must not break that direction or pull tokio.

@history_item n=3 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Conventions: `~ decision/unparseable-external-values-unknown`) — added the exception that a hyprctl reply the seam cannot parse is the seam's `malformed` error, not `"unknown"`. Bounds what an at-cap but unparseable reply must read in the new tests (an error code, never an unknown value).

@history_item n=4 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Occupied Resources: `+ name/crate/viola-hyprland`, `+ name/binary/viola-hypr-tap`) — registered the crate and the test-only `viola-hypr-tap` bin of viola-e2e. The chunk's surfaces sit in this crate; the test-side fake/tap is an already-registered name, so no new name is owed unless the chunk adds one.

@history_item n=5 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (Infrastructure Patterns: `~ decision/root-bin-consumes-normalised-events`) — dropped "depends on all members" because viola-hyprland is only a root dev-dependency while no verb calls the seam. A trap: the seam has no product caller yet, so its bounds are reachable only through the crate's own tests (and root dev-tests), which is why the audit's survivors must be killed in-crate.

@history_item n=6 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (bounded readers, a cross-cutting pattern held by a census) — established `decision/bounded-readers`: every product reader of external bytes is bounded in its own statement or named in `EXCEPTIONS`, held by `tests/contract_bounded_readers.rs` (`BOUNDARIES` rows each naming a witness test, census `product_readers_are_each_bounded_or_named`). `read_within`'s `MAX_FRAME + 1` drain is such a boundary; the new exact-frame test may be the natural witness for its `Boundary {` row, and a test-only chunk must leave the census green.

@history_item n=7 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** ([MCP] the viola mcp stdin line cap) — precedent for exact-frame semantics: "a line of exactly the cap passes whole … a longer line releases no byte", refused whole, never truncated, with the wrapper channel's close-on-oversize rule as its precedent. The same at-cap-passes / cap+1-refused pair is what this chunk pins on both hyprland sites.

@history_item n=8 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** (frame rules) — fixed the original `MAX_FRAME` rule: one line ≤ `MAX_FRAME` (`\n` counted), longer → refused and closed. The origin of the `>` (not `>=`) boundary the audit's two `>` → `>=` survivors mutate; the tests confirm exactly `MAX_FRAME` is inside the cap.

@history_item n=9 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (Standard Contracts: the agents read) — precedent for a capped child read: `read_logged` drains through `take(MAX_FRAME + 1)` by `version_gate::run_capped` with an over-cap read an `oversize` failure. Same drain shape as `read.rs:121`'s `MAX_FRAME + 1` drain feeding the `:77` comparison under test.

@history_item n=10 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — mutation left CI and the pre-push (founder ruling 2026-09-28): CI runs no mutation job; mutation runs only through `run --mutants`, kept for the epoch-boundary code audit (`decision/mutation-at-epoch-boundary`). Grounds the scope's "no mutation run, local or dispatched": the survivors are killed by tests plus one-off remove-the-guard controls, and the audit re-mutates later.

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (mutation runs: no scratch arm, no Windows leg) — the boundary audit runs `run --mutants` on Linux alone, with the Linux host's NOCOW `TMPDIR` as an environment fact; no Windows leg remains. The next audit that must catch both `>=` mutants is this Linux-only run.

@history_item n=12 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (CI on the one `ubuntu-latest` leg; the OS target: Linux only) — CI is one `ubuntu-latest` leg against the fake agent (`decision/ci-against-fake-agent`), and design depends on no tmux or shell (`decision/no-tmux-or-shell-dependency`). The new tests need no OS gate (an OS-gated test needs a `GATES` row in `tests/contract_os_gated_tests.rs`, and a gate naming another OS is a fault).

@history_item n=13 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the six landed ledger rows …) — set that the real `claude` CLI runs only locally and that CI drives only the fake agent (`decision/real-claude-local-only`). Matches the chunk's boundary: no live `claude` start, no real `hyprctl`, only the fake `hyprctl`.

@history_item n=14 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the sanitised error surfaces pattern and its contract binary) — `thiserror_messages_interpolate_nothing` holds every `thiserror` message fixed, and error bodies carry codes and fixed messages only. `HyprlandError::Oversize`'s fixed message ("hyprctl reply over the frame cap") falls under it; any product-line fix found by a test must keep the message fixed.

@history_item n=15 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the one-enum divergences ratified as named exceptions) — ratified `ServeError`, `StampsError`, `PinError`, `StampsReadError`, `Refused` beside `PtyError` and recorded the `thiserror::Error` derive census (9 outside `crates/viola-e2e`). Bears on `decision/typed-errors-per-crate`: viola-hyprland's one typed `HyprlandError` is the rule's form; a test-only chunk adds no enum, and any red that touches errors must keep one enum.

@history_item n=16 marker=2026-10-13-epoch-4-survivors
**2026-10-13-epoch-4-survivors** — precedent for a chunk closing a boundary audit's survivors: the replace loop's const-gated mutants were made reachable by passing the retry decision in, and a test-only shared report file was split per process. It shows the earlier pattern (survivors killed by tests, the mechanism recorded as measured) that this chunk follows test-only on the two `MAX_FRAME` sites.

@history_item n=17 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (four crossings ratified by the founder …) — the user's statusline command through `/bin/sh -c` is the single ratified shell-out, the `except` of `decision/no-shell-out`. Confirms the fake `hyprctl` must be spawned directly, never through a shell.

@history_item n=18 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the recorded live MCP exchange: the tap …) — `viola-mcp-tap`, the test-only tap bin of viola-e2e whose lines go through `take(MAX_FRAME)`, set the precedent `viola-hypr-tap` follows. Useful context for the fake `hyprctl` / recorded replies the read's exact-frame test drives.
