@row_add n=1 row=decision/unverified-cli-transport-only area=established-decisions rec=2,1,3
name: Transport-only degrade on an unverified CLI
rule: On a CLI version that is not verified, viola still types, runs the wheel and emits events, but withholds dialog answers, drivers get `unverified-cli`, and the readiness gate is partial.
why: The docs lag the installed build, and several load-bearing behaviours are measured, not documented.

@row_add n=2 row=decision/linux-only area=established-decisions rec=5,8,4,6,7
name: Linux only
rule: viola targets Linux only: the core on any current Linux distribution, built and CI-tested on `ubuntu-latest`, the window features on Hyprland; no Windows or macOS code, test, fixture or CI job stands in the tree.
why: The Windows and macOS support is cut, recoverable at the tag `multi-os-final`.

@row_add n=3 row=decision/sync-core-tokio-only-in-mcp area=established-decisions rec=9,10,11
name: Hybrid sync core, Tokio only in mcp
rule: The core is synchronous, and Tokio is confined to the `mcp` subcommand; among the product crates only `viola-mcp` enables `tokio`.
why: portable-pty's handles are blocking, so the pump needs threads either way; `hook` must not pay for runtime startup; rmcp and interprocess-async require Tokio.
guard: sole-root `cargo deny` `tokio` ban (`deny-sync.toml`) and a `cargo check` of the sync crates on `ubuntu-latest`
except: `viola-channel`'s optional `tokio` feature, enabled by `viola-mcp`; its graph without the feature is proven tokio-free.

@row_add n=4 row=decision/portable-pty-behind-pty-seam area=established-decisions rec=12,13
name: portable-pty pinned behind viola's pty seam
rule: portable-pty, pinned exactly at `=0.9.0`, is used only behind viola's own `pty` seam (spawn, read, write, resize, wait, kill).
why: Exact, as the workspace pins every third-party version; 0.9.0's graph holds no `pin-utils` (RUSTSEC-2026-0441), and `viola-pty` builds on it unchanged.

@row_add n=5 row=decision/child-exit-on-process-handle area=established-decisions rec=12,13
name: Child exit detected on the process handle
rule: Child exit is detected on the process handle, never on EOF.

@row_add n=6 row=decision/vt100-readiness-gate area=established-decisions rec=14,15
name: vt100 as a pre-send readiness gate only
rule: vt100 0.16.2 serves only as a pre-send readiness gate: after `turn-ended` it waits for a quiet screen, requires no known modal signature and waits, up to its maximum wait, for the version-stamped input-box signature; the passthrough never waits on the model.
why: CLI-native modals bypass every hook, and the screen lags the hooks.

@row_add n=7 row=decision/ndjson-logs-and-atomic-snapshots area=established-decisions rec=16,17
name: ndjson append logs plus atomic JSON snapshots
rule: State is kept in ndjson append logs and atomic JSON snapshots, with std `File::lock` on separate `.lock` files; the log stays the source of truth.
why: The trail stays human-readable and `tail`-able, with no C in the build and no daemon; SQLite and redb were rejected.
