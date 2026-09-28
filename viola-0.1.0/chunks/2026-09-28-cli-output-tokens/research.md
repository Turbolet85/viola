# Codebase Research — 2026-09-28-cli-output-tokens

## Scope
- **Depth:** moderate · **Reads:** 11 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (read in full, 5 Session Additions; applied: `run --mutants` mutates only diff lines, a `#[cfg(unix)]`/`#[cfg(windows)]` body needs its compiling leg, nextest filter needs `tests::name`) · `.claude/rules/testing.md` (read in full, 17 Session Additions; applied: 2026-09-24 every function needs an observable effect, keep an OS-gated body to a minimal reader with its logic in a plain function, `|`→`^` equivalent mutants on combined flags, one inline `#[cfg(test)]` module)
- **Platform issues consulted:** none. There is no runner-only bullet: the Setup read found ci#36398308023 green 18/18 on `c04e332`.

## Files inspected
- `src/main.rs` (1–120) — The catch site. `Role` is `Hook | Other` (main.rs:69-72). `role_of` maps only `hook` specially (main.rs:76-90). `exit_code` gives `Other` exit 1 on `Failed | Panicked | Unparsed` (main.rs:104-110). A clap usage error for `Other` goes through `usage.exit()` (main.rs:50), which is clap's own printer.
- `src/cmd/mod.rs` (full, 64 lines) — The `Command` enum is `Run(RunArgs)` and a hidden `Hook(HookArgs)` (mod.rs:28-34). There is no other verb. `hook` returns before the home resolves (mod.rs:45).
- `src/obs.rs` (200–320) — `internal_error_exit_line` writes a role line only for `ObsProcess::Run` (obs.rs:220-231). `report_internal_error` writes the role line plus the chain to the detail file and "nothing reaches stdout or stderr" (obs.rs:240-247).
- `src/cmd/run.rs` (355–400) — `refuse(unable, hint)` is the root bin's only human stderr writer: a locked `io::stderr()` and two `writeln!` with the results discarded (run.rs:360-364). It has five callers, all `run` start refusals.
- `crates/viola-pty/src/lib.rs` (228–347) — `HostTerminal::enter` sets stdin raw and stdout `vt_output_mode`, saves both, and restores them on Drop (lib.rs:266-329). `VT_OUTPUT = 0x000c` is ENABLE_VIRTUAL_TERMINAL_PROCESSING **plus DISABLE_NEWLINE_AUTO_RETURN** (lib.rs:238-239). `host_size` reads `GetConsoleScreenBufferInfo` (lib.rs:332-347).
- `crates/viola-core/src/lib.rs` (grep, 187 lines) — `pub mod obs`, `SERVICE_NAME`, `VERSION`, `MAX_FRAME`, `ViolaName`, `EventKind`. There is **no** `validate_paste_text` and no control-character predicate yet; re-derived with `grep -nE 'pub (fn|const|struct|enum|mod)|control|paste' crates/viola-core/src/lib.rs`, which returned 7 lines, none of them a validator.
- `Cargo.toml` (1–30, 96–200) — clap is `{ version = "=4.6.7", features = ["derive"] }`, so its default features are on (Cargo.toml:143). The workspace windows-sys already lists `Win32_System_Console` (Cargo.toml:177), and the root bin takes `windows-sys.workspace = true` under `cfg(windows)` (Cargo.toml:122-123). The workspace clippy lints deny `print_stdout` / `print_stderr` / `dbg_macro` (Cargo.toml `[workspace.lints.clippy]`). There are 15 `[[test]]` entries for 18 `tests/*.rs` files, so a test without `required-features` is auto-discovered.
- `clippy.toml` (full) — `disallowed-macros` bans only the tracing level macros, so no print ban hides there.
- `scripts/wsl-provision.sh` (grep) — its pins come only from `rust-toolchain.toml` (line 41), ci.yml's test-job `name@version` tool line (lines 44, 125-127), `NODE_PIN_VERSION` (line 52) and the e2e-web lockfile's Playwright Chromium (line 134). The workspace `Cargo.lock` is not a provisioning input.
- `tests/run_cli.rs` (505–518), `tests/hook_fail_open.rs` (316–330) — `viola_without_a_verb_is_usage` asserts only exit 2, and `hook_verb_is_hidden_from_the_help` asserts only words in `--help`. Neither pins colour.
- `tests/cli_controls_not_disableable.rs` (1–40) — the interim table covers hook-path controls only. Its module doc names the rows still to join; a colour variable is not a control.

## Graph impact (from the code-graph query; rust plane, db `fresh`, 16 rows, trace `tree-query-2026-09-28-cli-output-tokens.json`)
- **refuse** — 5 callers, all in `src/cmd/run.rs`: `refuse_batch_script` @ run.rs:367, `refuse_live` @ run.rs:376, `refuse_stale` @ run.rs:381, `refuse_tampered_pin` @ run.rs:388, `refuse_squatted` @ run.rs:397 (editor lines, 0-indexed + 1). Routing `refuse` through the new stream writer keeps these five call sites unchanged if its signature stays `(&str, &str)`.
- **report_internal_error** — 1 production caller, `main` @ main.rs:55, plus 2 tests in `src/obs.rs` (obs.rs:627, obs.rs:665). No chunk change is planned (see the CARRY closure).
- **role_of** / **exit_code** — `main` @ main.rs:45 plus unit tests in main.rs. Unchanged.
- No symbol for the new layer exists yet: the collision check for the module name is P4's, and the query above found no `human`/`style`/`escape` symbol in the root bin.

## Patterns detected
- **Locked-stderr `writeln!`, result discarded** (run.rs:360-364): the as-built form the workspace print ban admits without a local `#[allow]`. A failed write (a closed pipe) is swallowed, never a panic. That is the obs-plan §10 zero-unlogged-panics requirement the obs extract raised.
- **Console-mode save/restore guard** (viola-pty lib.rs:266-329): a guard saves modes on `enter` and restores them on `Drop`, and is a no-op off a terminal. The layer's VT enable should follow the same shape, but **not** reuse `vt_output_mode`: DISABLE_NEWLINE_AUTO_RETURN is right for the child's passthrough screen and wrong for viola's own `\n`-terminated lines (hypothesis per the Win32 console-mode documentation, not measured here: under that flag a bare LF moves down without returning to column 0).
- **Pure classifier over injected facts** (`viola_state::liveness::classify`, cited by the tests history): the colour decision and depth take `(mode, NO_COLOR present, TERM value, COLORTERM value, stdout is a TTY, VT result)` as arguments, and only a thin reader touches `std::env` and the console.
- **OS-gated minimal reader** (testing.md 2026-09-24): the `#[cfg(windows)]` body is only the `GetStdHandle`/`GetConsoleMode`/`SetConsoleMode` calls, the Unix side is a constant, and the mode arithmetic is a plain `const fn` tested on every OS.

## Conventions to follow
- **Human-output print ban**: write through a locked `io::stdout()` / `io::stderr()` with `writeln!` (run.rs:361-363), never `print!`/`eprintln!` and never a crate-level allow (Cargo.toml `[workspace.lints.clippy]`; `tests/contract_lints.rs`).
- **Combined flag constants as one literal** (viola-pty lib.rs:235-239; testing.md 2026-09-27): the VT-processing value is a single literal pinned by a test.
- **Inline `#[cfg(test)] mod tests`** with rstest `#[case::label]` tables and literal oracles (testing.md; test-plan §4).
- **Root integration test naming**: a new `tests/cli_<topic>.rs` joins the integration layer (test-plan §2). One that needs no fake agent needs no `[[test]]` entry (auto-discovered, like 3 of the 18 today).

## Key findings
- **F1 — clap already colours viola's output, and reads terminal env vars to decide.** clap's default `color` feature pulls in `anstream` → `anstyle-query` (Cargo.lock:44-88). `anstyle-query-1.1.5/src/lib.rs` reads `CLICOLOR` (line 24), `CLICOLOR_FORCE` (line 35), `NO_COLOR` (line 50), `TERM` (lines 58, 75, 96), `COLORTERM` (line 116) and `CI` (line 134). Measured on `target/debug/viola.exe` (built 2026-09-28 00:35, whose clap line was last changed in b0236ca, 2026-09-24): `CLICOLOR_FORCE=1 viola --help` with stdout piped emits **25 ESC bytes** (`\x1b[1m\x1b[4mUsage:\x1b[0m …`), and the same call without it emits 0. So at HEAD an env var can force SGR into piped output, and clap's bold/underline headers colour output other than `DIALOG`/`stale`. Both break design-system §Surface: cli, which bans colour on anything but `DIALOG`/`stale` and any SGR under non-TTY. The architecture/security-plan sentence that "the only other variables any `viola` build reads are the two test seams" (architecture.md:586; security-plan.md:427) is therefore already false through a dependency, a narrow-basis claim in the spec. Turning clap's `color` feature off removes both problems and removes `anstream`, `anstyle-query`, `anstyle-wincon` and `colorchoice` from the lockfile. That is a lockfile removal, not an addition.
- **F2 — the layer has no production consumer yet.** Only `run`'s `refuse` writes human text today. No verb prints a table, a readback line, `wait`/`last` text or a `DIALOG` word until Epochs 3–5. The token table, the decision and the escaper would be dead code in the `viola` bin. clippy's `dead_code` is a warning, and `-D warnings` makes it red. See the open question.
- **F3 — the escape set.** Rust's `char::is_control` is Unicode `Cc`: U+0000–U+001F, U+007F (DEL) and U+0080–U+009F. That is exactly "C0, DEL and C1", classified over decoded `char`s, so UTF-8 continuation bytes never count as C1. Message mode keeps U+000A and U+0009. CR (U+000D) is escaped in both modes, because the design/security/layouts extracts keep only `\n`/`\t`. The tests extract's "except LF/CR/TAB" is the paste-input set, not the output set.

## New files to create
- `src/human.rs` — the shared human-output layer: the per-depth token table, the pure colour decision and depth rule, the thin env/TTY/VT reader, the stdout/stderr stream writer, and the inline unit tests (split into `src/human/` submodules if it passes the 700-line cap)
- `tests/cli_output_plain.rs` — process-level plain-output witnesses: `viola --help` and a usage error, under `CLICOLOR_FORCE=1` / `NO_COLOR` / `TERM=dumb` with stdout piped, carry no ESC byte
- `crates/viola-core/src/text.rs` — only if P4 fork 2 places the escaper in `viola-core`: the C0/DEL/C1 `\xHH` escaper with its two modes and its unit table

## Files to modify
- `src/main.rs` — declare the new module and hold the VT guard for a CLI verb's lifetime, if P4 keeps a guard
- `src/cmd/run.rs` — `refuse` writes its two lines through the layer's stderr refusal writer, with the same five call sites
- `Cargo.toml` — the workspace clap dependency turns `default-features` off and keeps `std`, `derive`, `help`, `usage`, `error-context`, `suggestions`
- `Cargo.lock` — drops `anstream`, `anstyle-query`, `anstyle-wincon`, `colorchoice` and any crate only they pulled in
- `crates/viola-core/src/lib.rs` — only if P4 fork 2 places the escaper in `viola-core`: declare `pub mod text`

## Open questions
- Where does a consumer-less layer land under `-D warnings` (F2)? Options: a `#[cfg_attr(not(test), expect(dead_code, reason = …))]` on the not-yet-consumed items (self-removing, since `expect` fails once a consumer lands); moving the pure pieces into `viola-core` as `pub`; or narrowing the chunk to what has a consumer now. → blocks: plan-decision (P4 fork 2)
- Admitting `NO_COLOR` / `TERM` / `COLORTERM` breaks the exhaustive sentence in architecture.md:586 and security-plan.md:427, and turning off clap's `color` removes the reads that already break it silently. Per the security history (`2026-09-27-wrapper-channel`), a boundary widening halts the wrap for a live founder answer. Does the plan admit them as a named "only ever turns colour off or down" exception ratified at wrap, or read none of them in this chunk? → blocks: plan-decision (P4 fork 1)
