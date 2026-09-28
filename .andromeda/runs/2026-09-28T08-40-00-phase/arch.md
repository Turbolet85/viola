# arch extract

## Relevance
relevant: the arch plan governs where the layer lives (the root `viola` bin), which dependencies and features it may use, which stream each role may write to, and which env vars a `viola` build may read. That last point is the chunk's main architectural tension.

## Constraints
- **Placement: root bin, no new crate.** arch §Established Decisions [Module Boundaries] and §Infrastructure Patterns → Crate dependency direction put subcommand dispatch and human-facing edge code in the root `viola` bin. The locked member list (§Occupied Resources → Workspace crates; §Inherited Defaults) admits no new crate. §Infrastructure Patterns → Project directory structure lists `src/main.rs`, `src/cmd/` (one module per subcommand) and `src/run/`, and has no shared human-output module. A new shared module under `src/` is therefore a tree addition the wrap must record in that section.
- **Dependencies: nothing new, windows-sys pinned.** arch §Stack and Technologies pins windows-sys 0.61.2 and names `Win32_System_Console` as a feature of the PTY layer, behind `HostTerminal` in `viola-pty`. §Infrastructure Patterns → Crate dependency direction lists the root bin's own windows-sys use as the registry reader only. The chunk may add a Cargo feature to the existing pin but no new crate, and no C-building or colour crate (§Infrastructure Patterns → Build system: `[workspace.dependencies]` pins, cargo-deny bans). Whether the root bin already enables `Win32_System_Console` (directly, or through feature unification from `viola-pty`) is research's question.
- **Env-var admission.** arch §Cross-cutting Patterns → Config management says env vars are not a configuration channel, and that the only non-`VIOLA_*` variables any `viola` build reads are the two `fake-agent` test seams. §Conventions → Naming patterns → Environment variables requires the `VIOLA_` prefix for every variable the product reads, with only those two ratified exceptions. §Occupied Resources → Environment variables lists no `NO_COLOR`, `TERM` or `COLORTERM`. Reading them needs an amendment to all three sections, a named exception that only ever turns colour off or down. Whether the plan's current wording already admits them is P3's question and a likely P4 fork.
- **Stream reservation per role.** arch §Cross-cutting Patterns → Diagnostic output channels requires the following:
  - `hook` writes only its decision body to stdout and nothing to stderr.
  - `run` writes nothing to the terminal while the child runs.
  - `mcp` writes only MCP frames to stdout.
  - Only `ui` and short-lived CLI verbs may use stderr for human output.

  arch §Established Decisions [Hook Contract] and §Conventions → `viola hook` exit codes forbid any new output path for `hook`. The layer's decision order has to preserve all of these.
- **`--json` shape and exit codes.** Under `--json`, arch §Standard Contracts → CLI `--json` output requires one `{"v":1,…}` document on stdout: `ok`, `refusal` + `detail`, or `error` (`instance-unreachable`, `wrapper-fault`). arch §Stack and Technologies → Serialization keeps declared key order through serde_json `preserve_order`. §Conventions → CLI exit codes fixes the typed exit code per outcome. §Established Decisions [CLI Conventions] fixes human text by default and `--json` for agents. The layer renders these outcomes and does not redefine them.
- **Catch-site errors.** arch §Established Decisions [Error Handling] keeps anyhow in the bin only. The catch-site reporter is `viola::obs::report_internal_error`, and the context chain goes only to `instances/<name>/diagnostics/detail-<role>.ndjson`, never to a role line, stdout or stderr. The folded CARRY's `error: internal error` line must not carry any part of that chain. §Occupied Resources → Filesystem already reserves the home-level `cli-<name>.ndjson` role file, but whether any producer or a `cli` role exists yet is research's question.
- **Cross-platform, sync, untrusted text.**
  - The Windows VT-enable branch is `cfg`-gated and compiles and is tested on its own CI runner, with the Unix side a plain no-op (arch §Cross-cutting Patterns → Cross-platform discipline).
  - The layer is sync std code with no Tokio (§Cross-cutting Patterns → Tokio containment).
  - Upstream-origin fields are content, forwarded as escaped strings and never interpreted (§Cross-cutting Patterns → Untrusted upstream text).

## Patterns to follow
- **Human stderr writes.** The root bin's only human stderr site today is the `run` start refusals: two fixed `unable:`/`hint:` lines through one `refuse` helper, using `writeln!` on the locked stderr, a form the workspace `print_stderr` ban does not flag (arch §Infrastructure Patterns → Build system). The shared output layer should follow the same form (locked handle plus `writeln!`) and absorb or subsume that helper, not sit beside it. That section's "only human stderr site" sentence then goes stale and becomes a wrap amendment.
- **Console modes.** arch §Established Decisions [PTY] (the as-built paragraph) and §Stack and Technologies → PTY layer place console-mode handling behind `HostTerminal`: modes are saved and restored on Drop, and it is a no-op off a terminal. P4 has to choose between two options, and should note that `viola-pty` "knows no agent":
  - reuse the seam: a VT-output enable exposed by `viola-pty`;
  - a separate direct windows-sys call in the bin.
- **Exit codes and fixed stderr lines.** arch §Conventions → CLI exit codes (item `1`) shows that each human refusal is a fixed `unable:` line plus a fixed `hint:` line, with no path, pid or pipe name, and a typed exit code. Later verbs rendering through this layer keep that shape.
- **Test placement.** arch §Infrastructure Patterns → Project directory structure puts human-mode expected output in `tests/cmd/*.toml` (trycmd with snapbox redactions) and `tests/snapshots/` (insta, check mode). The workspace lint inheritance is asserted by `tests/contract_lints.rs` (§Infrastructure Patterns → Build system).

## Anti-patterns to avoid
- **No `print!`, `println!`, `eprint!`, `eprintln!` or `dbg!` in the layer, and no crate-level print allow added to the root bin.** The workspace `[workspace.lints.clippy]` bans are inherited by every product member (arch §Infrastructure Patterns → Build system).
- **No configuration channel for colour.** Add no `VIOLA_*` colour variable and no `config.json` colour key: env vars are not a configuration channel, and `config.json` holds only the settings listed (arch §Cross-cutting Patterns → Config management).
- **No new crate or colour dependency in the lockfile.** Examples are a terminal-colour crate or a new workspace member (arch §Established Decisions [Module Boundaries]; §Infrastructure Patterns → Build system dependency policy).

## Contract bindings
- **arch ↔ security (env-var admission).** The "only env vars" sentence is in both arch §Cross-cutting Patterns → Config management and the security-plan (mirrored in `.claude/rules/security.md`). Any `NO_COLOR`/`TERM`/`COLORTERM` exception must amend both, plus arch §Occupied Resources → Environment variables and §Conventions → Environment variables, in one wrap.
- **arch ↔ design-system (tokens and streams).** design-system §Surface: cli owns the tokens, the depth table and the decision order. arch owns the root-bin placement, the windows-sys pin and feature, and the per-role stream reservation (§Cross-cutting Patterns → Diagnostic output channels). The design-system §Streams split must not contradict arch's role rules for `hook`, `mcp` and `run`.
- **arch ↔ obs (catch site).** The CARRY's `error: internal error` stderr line (obs-plan §7) binds to arch §Established Decisions [Error Handling]: `report_internal_error`, with the chain only in the detail file. It also binds to the `cli-<name>.ndjson` role file in §Occupied Resources → Filesystem. The closed `ObsEvent` set means the colour decision logs nothing.
- **arch ↔ tests (human output).** Human-mode output is pinned by trycmd cases and insta snapshots (arch §Infrastructure Patterns → Project directory structure). The Windows-only VT branch has to be tested on the `windows-2025` runner (§Cross-cutting Patterns → Cross-platform discipline).

## Acceptance criteria contributions
- The layer lives in the root `viola` bin. `Cargo.lock` gains no new package, no workspace member is added, and windows-sys stays at 0.61.2, with at most a feature added (per arch §Established Decisions [Module Boundaries]; §Stack and Technologies).
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` passes with the workspace `print_stdout`/`print_stderr` bans in force and no print allow added to the root bin (per arch §Infrastructure Patterns → Build system).
- Under `--json`, stdout holds exactly one `{"v":1,…}` document with no SGR byte and no `hint:` line. On the `hook`, `mcp` and `run` (child-running) paths, the layer emits no byte to stdout or stderr (per arch §Standard Contracts → CLI `--json` output; §Cross-cutting Patterns → Diagnostic output channels).
- At wrap, every non-`VIOLA_*` env var the layer reads is named in arch §Occupied Resources → Environment variables and admitted by §Cross-cutting Patterns → Config management. A variable the plan does not name fails (per arch §Cross-cutting Patterns → Config management; §Conventions → Environment variables).
