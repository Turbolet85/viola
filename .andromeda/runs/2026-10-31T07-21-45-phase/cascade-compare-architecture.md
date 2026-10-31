# Cascade compare — architecture · phase · 2026-10-31-sanitised-error-surfaces
E items 180 · held 53 · lost at classify 4 · lost at strong 0 · beside a placed fact - · cascade only 21 · listed 3 · body 86 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### occupied-resources#1.1 · lost at classify · .andromeda/architecture-detail/occupied-resources.md:8
- (.andromeda/architecture-detail/occupied-resources.md:8) The recorded `endpoint` is an absolute path, so a connect failure on any surface must report a code and never echo it, and the witness must hold it in play.
  > `run` resolves the path once and records it as `endpoint` in the instance snapshot.

### stack-and-technologies#1.3 · lost at classify · .andromeda/architecture-detail/stack-and-technologies.md:12
- (.andromeda/architecture-detail/stack-and-technologies.md:12) The CLI enumeration must cover this verb list, the clap feature set must hold unchanged, and clap's `error-context` usage render must be named in the enumeration as a body or as an explicit carve-out, never left unread.
  > clap 4.6.7 (derive; `default-features = false` with `std`, `derive`, `help`, `usage`, `error-context`, `suggestions` — its default set minus `color`, so help and usage output is plain and no colour/terminal crate is in the graph) | Subcommands `run · revive · send · wait · last · list · answer · hook · mcp · verify · pause · release · link · unlink · plugin install`

### stack-and-technologies#1.23 · lost at classify · .andromeda/architecture-detail/stack-and-technologies.md:32
- (.andromeda/architecture-detail/stack-and-technologies.md:32) The chunk's code must pass clippy under these bans and the obs ripgrep gates, with no error text routed around the print and tracing-macro bans.
  > rustfmt, clippy (`-D warnings`, with the workspace `print_stdout` / `print_stderr` / `dbg_macro` bans and `clippy.toml` `disallowed-macros` on the tracing level macros), `cargo check`; ripgrep 15.2.0 (PCRE2; the obs G1/G3 gate tool, installed by `scripts/install-ripgrep.sh`);

### stack-and-technologies#1.23 · lost at classify · .andromeda/architecture-detail/stack-and-technologies.md:32
- (.andromeda/architecture-detail/stack-and-technologies.md:32) A source census behind the enumeration may take syn and proc-macro2 only inside `viola-e2e`, and must add neither to a product crate or to the root package.
  > syn `=2.0.119` (`default-features = false`; features `full`, `parsing`, `printing`, `visit`) and proc-macro2 `=1.0.107` (feature `span-locations`), direct dependencies of the test-only `viola-e2e` alone (the `#[cfg]` reader behind `run --mutants`' host exclusion, `harness::run::mutants::host`; no product crate depends on either);

### standard-contracts#11.1 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:87
- (.andromeda/architecture-detail/standard-contracts.md:87) An append failure on `hook.event` is a `-32603` source, so an I/O error's path or text must not reach its body.
  > An invalid event is not appended, and an append failure is `-32603`.

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### established-decisions#31.1 · listed (lost at classify) · .andromeda/architecture-detail/established-decisions.md:273
- (.andromeda/architecture-detail/established-decisions.md:273) New code and witnesses must sit in these workspace members, with the `Refusal` decision settled inside `viola-agent-claude` and no new product crate.
  > Members: `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`, plus the test-only `viola-e2e` (the `viola-harness` agent-run harness; no product crate depends on it).

### stack-and-technologies#1.2 · listed (lost at classify) · .andromeda/architecture-detail/stack-and-technologies.md:11
- (.andromeda/architecture-detail/stack-and-technologies.md:11) Error-surface code on the CLI and wrapper paths must stay on std threads, with any Tokio use confined to `mcp`, so the sanitising work brings no runtime to those paths.
  > Hybrid: std threads for `run`, `hook`, `send`, `wait` and every other CLI verb; Tokio 1.53.3 built only inside `mcp` | No async runtime on the hot `hook` path. The async ecosystem is used only where rmcp requires it

### stack-and-technologies#1.22 · listed (lost at classify) · .andromeda/architecture-detail/stack-and-technologies.md:31
- (.andromeda/architecture-detail/stack-and-technologies.md:31) Each surface's witness must run on all three OSes against the fake agent, must plant its own `CLAUDE*` value because the local gate's children start from `env -i`, and the verdict must be read from CI, the local gate being a filter only.
  > Build, lint and test on all three OSes against the fake agent, on native runners. Before the operator push, the local `viola-harness pre-push` gate runs the ubuntu test job's suites natively on the Linux dev host, in the working tree itself (no clone), every child through `/usr/bin/env -i HOME=<passwd home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin`; it refuses `pre-push-linux-only` off Linux, and its `tools` stage checks `cc`, the `rust-toolchain.toml` channel and ci.yml's `test`-job pins (it installs nothing) — a pre-push filter, never the verdict of record

## Held

- conventions#1.1
- conventions#2.1
- conventions#3.1
- conventions#5.1
- conventions#6.1
- conventions#7.1
- conventions#8.1
- conventions#9.1
- cross-cutting-patterns#1.1
- cross-cutting-patterns#2.1
- cross-cutting-patterns#3.1
- established-decisions#14.1
- established-decisions#16.1
- established-decisions#17.1
- established-decisions#18.1
- established-decisions#21.1
- established-decisions#25.1
- established-decisions#25.2
- established-decisions#27.1
- established-decisions#30.1
- established-decisions#35.1
- established-decisions#9.1
- occupied-resources#13.1
- occupied-resources#15.1
- occupied-resources#16.1
- occupied-resources#18.1
- occupied-resources#2.1
- occupied-resources#20.1
- occupied-resources#21.1
- occupied-resources#22.1
- occupied-resources#24.1
- occupied-resources#25.1
- occupied-resources#3.1
- occupied-resources#34.1
- occupied-resources#35.1
- occupied-resources#4.1
- occupied-resources#5.1
- occupied-resources#6.1
- stack-and-technologies#1.11
- stack-and-technologies#1.12
- stack-and-technologies#1.15
- stack-and-technologies#1.16
- stack-and-technologies#1.18
- stack-and-technologies#1.8
- standard-contracts#12.1
- standard-contracts#14.1
- standard-contracts#15.1
- standard-contracts#16.1
- standard-contracts#18.1
- standard-contracts#4.1
- standard-contracts#5.5
- standard-contracts#8.1
- standard-contracts#9.1

## Cascade only

- established-decisions#12.1
- established-decisions#13.1
- established-decisions#15.1
- established-decisions#2.1
- established-decisions#24.1
- established-decisions#25.3
- established-decisions#26.1
- established-decisions#29.1
- established-decisions#32.1
- established-decisions#4.1
- established-decisions#5.1
- occupied-resources#11.1
- occupied-resources#12.1
- occupied-resources#14.1
- occupied-resources#29.1
- occupied-resources#32.1
- occupied-resources#39.1
- occupied-resources#39.2
- stack-and-technologies#1.13
- standard-contracts#3.0
- standard-contracts#7.1
