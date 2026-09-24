# Measured: clippy 1.98.1 `disallowed-macros` versus an allow inside an exported macro

For wrap to fold into the chunk's research and into the obs-plan §3 amendment. /implement does not edit research.md.

**Setup.** A throwaway two-crate workspace, built with `mk.py` (kept in the session scratchpad `dmprobe/`):
- crate `m` exports `ev!`, which expands `::tracing::event!`;
- crate `c` calls `m::ev!()`;
- `clippy.toml` disallows `tracing::event` and `tracing::info`;
- tracing is `=0.1.44`, the build is `--offline` against viola's `Cargo.lock`, and the command is `cargo clippy -p c -- -D warnings`.

**Readings.** The count is per `ev!` call:

| placement of `allow(clippy::disallowed_macros)` | result |
|---|---|
| on an inner block inside the macro (obs-plan §3's form, HEAD's `obs_event!`) | 2 errors |
| on the `event!` statement inside the macro | 2 errors |
| on a closure wrapping `event!` inside the macro | 2 errors |
| `#[expect(...)]` on the inner block | 2 errors |
| on the macro's outermost expanded node | 2 errors |
| on the call-site statement in `c` | 2 errors |
| on the calling fn in `c` | 2 errors |
| crate-level `#![allow(...)]` in `c` | clean |

In viola itself, `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` with `tracing::event` in `clippy.toml` gave 16 `use of a disallowed macro tracing::event` errors, all at `obs_event!` call sites (`src/obs.rs`, `src/run/mod.rs`; log `p1-clippy-2.log`).

**Consequence.** obs-plan §3 `logger-stack-install` ("obs_event! expands to a block whose inner ::tracing::event! statement carries #[allow(clippy::disallowed_macros)], so caller crates pass -D warnings") cannot hold on this toolchain.

**Operator decision** (overseer, founder-delegated, at /implement P1):
- `clippy.toml` bans `tracing::{info,warn,error,debug,trace}` by definition path.
- A raw `event!` is caught by a fail-closed grep (`\bevent!\s*[({\[]` in any `.rs` outside `crates/viola-core/src/obs.rs`), which has its own positive and negative probe in `scripts/lint-probes.sh` and runs as a step in the CI lint job.
- The now-inert inner `#[allow]` in `obs_event!` is left in place: `crates/viola-core/src/obs.rs` is outside this chunk's modify-set.
