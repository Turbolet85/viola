# The Epoch 2 audit's 11 mutation "survivors" against the project's own union (plan step 15, scope item 3)

The Epoch 2 code audit ran cargo-mutants per crate on this Windows host only (`.andromeda/runs/2026-09-27T13-39-34-code-audit/`
`c-mutation-viola-{channel,state,pty}.json`), with no union. The project's verdict is the union of the ubuntu and windows legs, each
mutant judged only by the legs whose `#[cfg]`s compile its line (`harness::cfg_legs`). Nine of the 11 sit in `#[cfg(unix)]` code the
Windows host compiles out. The two viola-state ones were caught on the ubuntu leg, which is enough for the union (it judges a mutant
caught when a compiling leg catches it). Readings are from CI `mutants-verdict-<leg>.json` artifacts (`gh run download`), as
research.md fact 5 recorded them at P3.

| crate | audit survivor (host, per-crate run) | code | CI run (sha) · leg | outcome on the compiling leg |
|---|---|---|---|---|
| viola-channel | `client.rs:193` `open` | `#[cfg(unix)]` | 36318398739 (`3efed41`) · ubuntu-latest | `client.rs:195` unviable |
| viola-channel | `endpoint.rs:75` `host_socket_dir` | `#[cfg(unix)]` | 36318398739 (`3efed41`) · ubuntu-latest | `endpoint.rs:76` caught |
| viola-channel | `server.rs:66–79` Unix `Guard` drop | `#[cfg(unix)]` | 36318398739 (`3efed41`) · ubuntu-latest | `server.rs:75` caught |
| viola-channel | `server.rs:106` Unix `listen` | `#[cfg(unix)]` | 36318398739 (`3efed41`) · ubuntu-latest | `server.rs:108` unviable |
| viola-state | `fs.rs:16` `restrict` | its Windows body is a no-op | 36298052174 (`ed359cd`) · ubuntu-latest | caught (windows leg: missed) |
| viola-state | `pin.rs:74` the `NotFound` guard | — | 36298052174 (`ed359cd`) · ubuntu-latest | caught (windows leg: missed) |
| viola-pty | `HostTerminal::enter` `420:9` → `None` | `#[cfg(unix)]` | 36165685381 (`17ea8c7`, lines then 10 lower) · ubuntu-latest | caught |
| viola-pty | `HostTerminal::enter` `420:9` → `Some(Default)` | `#[cfg(unix)]` | 36165685381 · ubuntu-latest | unviable |
| viola-pty | `422:64` | `#[cfg(unix)]` | 36165685381 · ubuntu-latest | caught |
| viola-pty | `427:71` | `#[cfg(unix)]` | 36165685381 · ubuntu-latest | caught |
| viola-pty | `host_size` Unix block `472:76` | `#[cfg(unix)]` | 36165685381 · ubuntu-latest | caught |

- Per crate: viola-channel 2 caught + 2 unviable, viola-state 2 caught, viola-pty 4 caught + 1 unviable. So all 11 were caught or unviable
  on their compiling leg, and the union never counted them as survivors. No killing test is owed (scope item 3, premise-corrected).
- This chunk's own union (the green pre-push, then CI) judges every mutant its diff regenerates, the moved pty and channel code
  included. A mutant that union reports missed is this chunk's to kill.
- Line numbers are those of each run's own sha. The pty rows are from `17ea8c7`, where the lines sat 10 lower than at `a0e6506`.
