@row_add n=1 row=decision/exact-toolchain-pin area=project-directory-structure rec=1
name: Exact Rust toolchain pin
rule: The project builds on one exactly pinned stable Rust toolchain with rustfmt and clippy, never a floating channel.
except: The separate fuzz workspace uses its own pinned nightly toolchain.
@row_add n=2 row=decision/dual-licence area=project-directory-structure rec=1
name: Dual MIT or Apache-2.0 licence
rule: The project is licensed MIT OR Apache-2.0, with both licence texts at the root and the README naming both.
@row_add n=3 row=decision/no-tracing-level-macros area=project-directory-structure rec=1
name: Tracing level macros banned
rule: The tracing info, warn, error, debug and trace macros are disallowed in the code.
guard: clippy disallowed-macros, checked by the lint probes
@row_add n=4 row=decision/anyhow-at-bin-edge-only area=project-directory-structure rec=1
name: anyhow at the binary edge only
rule: anyhow error handling is confined to the edge of the viola binary.
@row_add n=5 row=decision/cargo-deny-bans area=project-directory-structure rec=1
name: cargo-deny supply-chain policy
rule: cargo-deny checks advisories, licences and sources, and bans C dependencies, telemetry and listed features.
guard: deny-probes negative probe per ban with a clean control
@row_add n=6 row=decision/no-tokio-in-sync-crates area=project-directory-structure rec=1
name: No tokio in sync crates
rule: Every crate on the single sync-crate list must have no tokio in its graph, each checked as the sole root.
guard: cargo-deny tokio ban over the sync-crate list
@row_add n=7 row=decision/channel-tokio-behind-feature area=project-directory-structure rec=1
name: Channel Tokio client behind a feature
rule: viola-channel's normal dependency graph carries no tokio; the Tokio client exists only behind its tokio feature.
guard: channel-tokio-free check in CI supply-chain
@row_add n=8 row=decision/rmcp-exact-features area=project-directory-structure rec=1
name: rmcp features exactly server and transport-io
rule: viola's release graph requests rmcp with exactly the features server and transport-io.
guard: rmcp-features check in the CI lint job
@row_add n=9 row=decision/release-carries-viola-only area=project-directory-structure rec=1
name: Release build carries viola only
rule: The release build carries the viola binary only and no test-only feature.
guard: release-check in the CI release job
@row_add n=10 row=decision/fuzz-separate-workspace area=project-directory-structure rec=1
name: Fuzzing in a separate workspace
rule: Fuzz targets live in a separate cargo-fuzz workspace with its own lockfile, excluded from the root workspace, with committed synthetic seeds.
@row_add n=11 row=decision/plugin-embedded-in-binary area=project-directory-structure rec=1
name: Claude plugin embedded in the binary
rule: The Claude plugin files are embedded in the viola binary and written out by viola run.
@row_add n=12 row=decision/board-read-model-shape area=project-directory-structure rec=1
name: Board read model is sync and Claude-free
rule: The board read model is sync and std only, carries no Claude shape, and none of its types holds endpoint, pinned_bin, cwd, statusline_command, pid, started_at or links.
@row_add n=13 row=decision/zero-panics-gate area=project-directory-structure rec=1
name: Zero panic lines in end-to-end runs
rule: No panic role line may appear under the end-to-end home, checked fail-closed.
guard: obs G2 zero-panics gate
except: A panic whose location is the hook seam module.
@row_add n=14 row=decision/snapshots-check-mode-only area=project-directory-structure rec=1
name: Insta snapshots in check mode only
rule: Root integration test snapshots are only checked by tests, never written by them.
@row_add n=15 row=decision/channel-json-rpc-local-sockets area=project-directory-structure rec=1
name: JSON-RPC over local sockets for the channel
rule: The channel speaks JSON-RPC 2.0 as newline-delimited JSON over interprocess local sockets.
@row_add n=16 row=decision/pty-seam-portable-pty area=project-directory-structure rec=1
name: portable-pty is the PTY seam
rule: The PTY seam is built over portable-pty, with host raw mode through libc termios.
@row_add n=17 row=decision/mcp-server-rmcp area=project-directory-structure rec=1
name: rmcp is the MCP server
rule: The MCP server is an rmcp stdio server, a thin adapter over the channel.
@row_add n=18 row=name/crate/viola-core area=project-directory-structure rec=1
class: crate
name: viola-core
@row_add n=19 row=name/crate/viola-pty area=project-directory-structure rec=1
class: crate
name: viola-pty
@row_add n=20 row=name/crate/viola-channel area=project-directory-structure rec=1
class: crate
name: viola-channel
@row_add n=21 row=name/crate/viola-state area=project-directory-structure rec=1
class: crate
name: viola-state
@row_add n=22 row=name/crate/viola-agent-claude area=project-directory-structure rec=1
class: crate
name: viola-agent-claude
@row_add n=23 row=name/crate/viola-mcp area=project-directory-structure rec=1
class: crate
name: viola-mcp
@row_add n=24 row=name/crate/viola-e2e area=project-directory-structure rec=1
class: crate
name: viola-e2e
@row_add n=25 row=name/binary/viola area=project-directory-structure rec=1
class: binary
name: viola
@row_add n=26 row=name/binary/viola-fake-agent area=project-directory-structure rec=1
class: binary
name: viola-fake-agent
@row_add n=27 row=name/binary/viola-harness area=project-directory-structure rec=1
class: binary
name: viola-harness
@row_add n=28 row=name/binary/viola-mcp-tap area=project-directory-structure rec=1
class: binary
name: viola-mcp-tap
