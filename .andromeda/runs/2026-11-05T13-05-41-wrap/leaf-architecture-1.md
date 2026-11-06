@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/gotchas.md lines=147
last: - For architectural decisions and their rationale, see `.and
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `.andromeda/architecture.md` [CLI Version Compatibility] ·
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=3 class=tree
leaf-text: _Extracted from `.andromeda/architecture.md` §Conventions
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: _Extracted from `doc:architecture` §Conventions
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=15 class=retired
leaf-text: `resolve_program`'s two refusals, `BatchScriptChild` and `ProgramNotFound`, each with a fixed message
source: fct-5gkabtaw «`AgentError` holds `resolve_program`'s one refusal, `ProgramNotFound`, with a fixed message»
replace-with: `resolve_program`'s one refusal, `ProgramNotFound`, with a fixed message
@leaf_edit n=3 leaf=.claude/docs/conventions.md line=33 class=retired
leaf-text: `batch-script-child` · `program-not-found` · `pinned-hash-mismatch`
source: fct-tg3zks37 «`program-not-found` · `pinned-hash-mismatch` · `version-unreadable` · `strict-modes-failed` · `unclean-fixture`, beside `interrupted`»
replace-with: `program-not-found` · `pinned-hash-mismatch`
@leaf_edit n=4 leaf=.claude/docs/conventions.md line=33 class=retired
leaf-text: on Unix a connected peer whose euid is not this user's or cannot be read (`peer_admitted`), on Linux and Windows the connected peer's pid or OS start time not the snapshot's
source: fct-7s455w3m «a connected peer whose euid is not this user's or cannot be read, `peer_admitted`; or the connected peer's pid or OS start time not the snapshot's, or that check not made»
replace-with: a connected peer whose euid is not this user's or cannot be read (`peer_admitted`), or the connected peer's pid or OS start time not the snapshot's
@leaf_edit n=5 leaf=.claude/docs/conventions.md line=33 class=retired
leaf-text: (claimed on Linux, Windows and macOS, on macOS an euid mismatch alone; macOS compares no peer pid, `v1-18`'s named carve-out, owned by no route entry)
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: (Linux only; the Windows and macOS legs left with their code, recoverable at the tag `multi-os-final`)
@leaf_edit n=6 leaf=.claude/docs/conventions.md line=33 class=retired
leaf-text: `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch`
source: fct-54jxzo44 «`cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`»
replace-with: `stale-heartbeat` · `pinned-hash-mismatch`
@leaf_edit n=7 leaf=.claude/docs/conventions.md line=45 class=stale
leaf-text: the Unix R8 pass-list of `CLAUDE*` names
source: fct-i4wacyrp «`claude_env_keep` (the R8 pass-list)»
replace-with: the R8 pass-list of `CLAUDE*` names
@leaf_edit n=8 leaf=.claude/docs/conventions.md line=58 class=tree
leaf-text: For complete architecture, see `.andromeda/architecture.md`
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: For complete architecture, see `doc:architecture` (`rec.py read --root . doc:architecture`)
@leaf_edit n=9 leaf=.claude/docs/conventions.md line=59 class=tree
leaf-text: For directory layout, see `.andromeda/architecture.md` Infrastructure Patterns section
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: For directory layout, see `doc:architecture` Infrastructure Patterns (`rec.py contracts architecture`)
@leaf_edit n=10 leaf=.claude/docs/gotchas.md line=3 class=tree
leaf-text: Extracted from `.andromeda/architecture.md` and the specialist plans
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: Extracted from `doc:architecture` and the specialist plans
@leaf_edit n=11 leaf=.claude/docs/gotchas.md line=17 class=retired
leaf-text: waiting for reader EOF to detect child exit hangs forever on Windows — ConPTY does not close the output stream when the child exits.
source: fct-omwrdnlc «Child exit is detected on the process handle, never on EOF; `pty_backend()` is `openpty`»
replace-with: waiting for reader EOF to detect child exit breaks the rule that child exit is detected on the process handle, never on EOF (`pty_backend()` is `openpty`; the ConPTY that never closed its output stream left with the Windows code, recoverable at the tag `multi-os-final`).
@leaf_edit n=12 leaf=.claude/docs/gotchas.md line=23 class=retired
leaf-text: 0.9.0 has the Windows garbage-read bug (wezterm#6783).
source: fct-ad44nn2e «0.9.0's Windows garbage-read bug (wezterm#6783) does not bear on a Linux-only build»
replace-with: 0.9.0's Windows garbage-read bug (wezterm#6783) does not bear on a Linux-only build; the pin stands on 0.8.1 being spike-proven.
@leaf_edit n=13 leaf=.claude/docs/gotchas.md line=24 class=stale
leaf-text: the named swap is portable-pty-psmux 0.9.7 (must keep `bInheritHandles = 0`). The PTY chunk records the pin decision on current evidence (requirements v1-25).
source: fct-ad44nn2e «Swap triggers: portable-pty-psmux 0.9.7 if the O1 stray-spaces artifact reproduces only under `viola run`»
replace-with: moving it off `=0.8.1` (RUSTSEC-2026-0441) is owed to the working-route entry "Version done-check"; the named swap is portable-pty-psmux 0.9.7 if the O1 stray-spaces artifact reproduces only under `viola run`.
@leaf_edit n=14 leaf=.claude/docs/gotchas.md line=28 class=retired
leaf-text: spawning `claude` by name on Windows hits the npm `.cmd` shim; portable-pty builds its own `CreateProcessW` line without std's BatBadBut escaping.
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: spawning `claude` by name leaves the program to a search viola does not own; the Windows npm `.cmd` shim case left with the Windows code, recoverable at the tag `multi-os-final`.
@leaf_edit n=15 leaf=.claude/docs/gotchas.md line=29 class=retired
leaf-text: viola resolves the program itself (PATHEXT only, never the bare name — portable-pty's own search hits the extensionless sh shim first) and resolves the shim to its sibling `…\@anthropic-ai\claude-code\bin\claude.exe` without reading it; `viola run` refuses (exit 1) any other `.cmd` / `.bat` PTY child.
source: fct-5gkabtaw «`resolve_program` (a given path as given, a bare name on `PATH`) returns `Result<PathBuf, AgentError>`»
replace-with: viola resolves the child program to an absolute path itself (`resolve_program`: a given path as given, a bare name on `PATH`) and spawns that; its one refusal is `ProgramNotFound`.
@leaf_edit n=16 leaf=.claude/docs/gotchas.md line=33 class=stale
leaf-text: locking the file you append to fails on Windows (rust-lang/rust#54118).
source: fct-73drmylz «lock files are separate, a layout chosen while Windows (where append + exclusive lock fails) was a target, and kept.»
replace-with: locking the file you append to breaks the kept layout: lock files are separate, a layout chosen while Windows (where append + exclusive lock fails, rust-lang/rust#54118) was a target, and kept.
@leaf_edit n=17 leaf=.claude/docs/gotchas.md line=48 class=retired
leaf-text: long socket paths fail to bind on macOS.
source: fct-wular7qm «Unix domain socket file `<socket dir>/viola-<h12>.sock` in a per-user 0700 directory (`$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`), never the `/tmp` root»
replace-with: the macOS socket-path cap left with the macOS code; on Linux the endpoint is `<socket dir>/viola-<h12>.sock` in a per-user 0700 directory (`$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`), never the `/tmp` root.
@leaf_edit n=18 leaf=.claude/docs/gotchas.md line=94 class=stale
leaf-text: except persistent-environment names (Windows registry `Environment`, Unix `config.json` `claude_env_keep`)
source: fct-jujxlicd «every inherited `CLAUDE*` name, matched exactly, except a name in `config.json` `claude_env_keep`»
replace-with: matched exactly, except a name in `config.json` `claude_env_keep`
@leaf_edit n=19 leaf=.claude/docs/gotchas.md line=108 class=retired
leaf-text: on the Windows runner a key written into ConPTY right after `ResizePseudoConsole` returned was never read by the Rust test child
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: on the retired Windows runner (its code out of the tree, recoverable at the tag `multi-os-final`) a key written into ConPTY right after `ResizePseudoConsole` returned was never read by the Rust test child
@leaf_edit n=20 leaf=.claude/docs/gotchas.md line=109 class=stale
leaf-text: Owner: the working-route entry "Windows-only live measurements", blocked on an interactive Windows host (the harness's `run --local-live` does not claim it).
source: fct-bq65okf4 «OPEN — whether the real `claude` loses a key typed right after a resize · owner: the open residuals of origin `2026-11-04T23-11-43-wrap` (target `another-os`)»
replace-with: Owner: the open residuals of origin `2026-11-04T23-11-43-wrap` (target `another-os`).
@leaf_edit n=21 leaf=.claude/docs/gotchas.md line=112 class=retired
leaf-text: **With the sideloaded ConPTY**
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: **With the sideloaded ConPTY** (removed with the Windows code, recoverable at the tag `multi-os-final`)
@leaf_edit n=22 leaf=.claude/docs/gotchas.md line=116 class=retired
leaf-text: on Windows x64 the sideloaded `OpenConsole.exe` writes
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: on Windows x64 (code out of the tree, recoverable at the tag `multi-os-final`) the sideloaded `OpenConsole.exe` wrote
@leaf_edit n=23 leaf=.claude/docs/gotchas.md line=118 class=retired
leaf-text: owned by the working-route entry "Windows-only live measurements", blocked on an interactive Windows host.
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: a Windows-only finding, left with the Windows code, recoverable at the tag `multi-os-final`.
@leaf_edit n=24 leaf=.claude/docs/gotchas.md line=122 class=retired
leaf-text: a `viola hook` fired within milliseconds of the wrapper's exit can still reach the exiting wrapper's own `\\.\pipe\viola-<h12>`
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: on the retired Windows build (its code out of the tree, recoverable at the tag `multi-os-final`) a `viola hook` fired within milliseconds of the wrapper's exit could still reach the exiting wrapper's own `\\.\pipe\viola-<h12>`
@leaf_edit n=25 leaf=.claude/docs/gotchas.md line=123 class=retired
leaf-text: (a connect refused or the path absent on Unix, the pipe not found on Windows)
source: fct-jv5gqhbl «`Wrapper::kill` waits on `holder_gone` instead (a connect to the path is refused or the path is absent)»
replace-with: (a connect to the path refused or the path absent)
@leaf_edit n=26 leaf=.claude/docs/gotchas.md line=129 class=tree
leaf-text: (`.andromeda/runs/2026-10-01T09-05-15-evolve-diagnose/proposals.md`)
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: (its run folder no longer in the tree)
@leaf_edit n=27 leaf=.claude/docs/gotchas.md line=147 class=tree
leaf-text: see `.andromeda/architecture.md` Established Decisions section
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: see `doc:architecture` Established Decisions section
@leaf_edit n=28 leaf=.claude/docs/services/viola-agent-claude.md line=3 class=tree
leaf-text: Primary source: `.andromeda/architecture.md` [Agent Coverage]
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: Primary source: `doc:architecture` [Agent Coverage]
@leaf_edit n=29 leaf=.claude/docs/services/viola-agent-claude.md line=6 class=retired
leaf-text: It resolves the npm shim to `claude.exe` and holds
source: fct-7tdo3mcl «child program resolution (`resolve_program`: a given path as given, a bare name on `PATH`)»
replace-with: It resolves the child program (`resolve_program`: a given path as given, a bare name on `PATH`) and holds
@leaf_edit n=30 leaf=.claude/docs/services/viola-agent-claude.md line=22 class=retired
leaf-text: (`/bin/sh -c`; both `None` on Windows)
source: fct-p4s6aosk «runs as one argument of `/bin/sh -c` (`statusline::shell_argv`, the fixed three-element argv `[&str; 3]`)»
replace-with: (`/bin/sh -c`, `shell_argv` the fixed three-element argv `[&str; 3]`)
@leaf_edit n=31 leaf=.claude/docs/services/viola-agent-claude.md line=22 class=retired
leaf-text: `USER_SETTINGS`, `WRAPPED_HERE` (the status line is wrapped: Unix), `BIN_SH`
source: fct-sfkaxxdn «`reading`, `user_command`, `override_document`, `shell_argv`, `SETTINGS_FLAG`, `USER_SETTINGS`, `BIN_SH`, `SHELL_FLAG`, `probe_document`»
replace-with: `USER_SETTINGS`, `BIN_SH`
@leaf_edit n=32 leaf=.claude/docs/services/viola-agent-claude.md line=22 class=retired
leaf-text: relied only where the status line is wrapped (`WRAPPED_HERE`, Unix); their Windows legs are owed to "Windows-only live measurements" (architecture, Capability ledger)
source: fct-p4s6aosk «relied on every build: `LedgerRow::relied()` holds all twenty-four rows»
replace-with: relied on every build (`LedgerRow::relied()` holds all twenty-four rows; architecture, Capability ledger)
@leaf_edit n=33 leaf=.claude/docs/services/viola-agent-claude.md line=23 class=retired
leaf-text: relied on Unix only)
source: fct-ehpssyzz «relied on every build (`LedgerRow::relied()`: all twenty-four rows)»
replace-with: relied on every build)
@leaf_edit n=34 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=retired
leaf-text: the per-OS relied set `LedgerRow::relied()` (`ALL` filtered in its order by `relied_on(row, statusline::WRAPPED_HERE)`: all twenty-four on Unix; on Windows twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`, the last three relied on every OS)
source: fct-qwpklckh «The relied set is `LedgerRow::relied()`: every row of `ALL`, in its order, all twenty-four.»
replace-with: the relied set `LedgerRow::relied()` (every row of `ALL`, in its order, all twenty-four)
@leaf_edit n=35 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=retired
leaf-text: every row of `LedgerRow::relied()` `pass`: all twenty-four on Unix; on Windows twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`;
source: fct-62qgonvf «A version is verified only when every row of `LedgerRow::relied()` reads `pass` (all twenty-four, on every build)»
replace-with: every row of `LedgerRow::relied()` `pass`, all twenty-four on every build;
@leaf_edit n=36 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=retired
leaf-text: all `pass`, reads unverified on every OS, and so do one holding the eighteen and `resume-session` and one holding the eighteen, `resume-session` and `agents-join` on Windows; one holding the eighteen, `resume-session`, `agents-join` and `local-command-compact` reads verified on Windows only;
source: fct-qwpklckh «or exactly the twenty-three ids every stamp written before chunk 2026-10-18-compact-post-condition-ledger-row holds, all `pass`, reads unverified»
replace-with: all `pass`, reads unverified;
@leaf_edit n=37 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=retired
leaf-text: also holds `resolve_program`'s two refusals, `BatchScriptChild` and `ProgramNotFound`, each with a fixed message
source: fct-5gkabtaw «`AgentError` holds `resolve_program`'s one refusal, `ProgramNotFound`, with a fixed message»
replace-with: also holds `resolve_program`'s one refusal, `ProgramNotFound`, with a fixed message
@leaf_edit n=38 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=retired
leaf-text: `CASE_INSENSITIVE` and `is_script` are `pub` for `verify`.
source: fct-5gkabtaw «`resolve_program` (a given path as given, a bare name on `PATH`) returns `Result<PathBuf, AgentError>`, in the crate and in the root wrapper `src/run/mod.rs`.»
replace-with: `resolve_program` returns `Result<PathBuf, AgentError>`, in the crate and in the root wrapper `src/run/mod.rs`.
@leaf_edit n=39 leaf=.claude/docs/services/viola-agent-claude.md line=28 class=stale
leaf-text: (ASCII case-insensitive on Windows, exact on Unix) is removed unless it is in the persistent set (Windows registry `Environment` names, Unix `config.json` `claude_env_keep`)
source: fct-jujxlicd «every inherited `CLAUDE*` name, matched exactly, except a name in `config.json` `claude_env_keep`»
replace-with: (matched exactly) is removed unless it is in `config.json` `claude_env_keep`
@leaf_edit n=40 leaf=.claude/docs/services/viola-agent-claude.md line=29 class=retired
leaf-text: `resolve_program`: PATH × PATHEXT on Windows (never the bare name); `claude.cmd`/`.bat` → sibling `node_modules/@anthropic-ai/claude-code/bin/claude.exe` when it is a file, the shim never read; any other `.cmd`/`.bat` → `AgentError::BatchScriptChild`; its other refusal is `AgentError::ProgramNotFound`.
source: fct-5gkabtaw «`resolve_program` (a given path as given, a bare name on `PATH`) returns `Result<PathBuf, AgentError>`»
replace-with: `resolve_program`: a given path as given, a bare name on `PATH`, returning `Result<PathBuf, AgentError>`; its one refusal is `AgentError::ProgramNotFound`, with a fixed message.
@leaf_edit n=41 leaf=.claude/docs/services/viola-agent-claude.md line=47 class=tree
leaf-text: `.andromeda/architecture.md` [CLI Version Compatibility]
source: fct-o2wbgrws «Contracts: ask `rec.py contracts architecture` — one row per key, each with its address; read one key by `rec.py render` of that address; never whole.»
replace-with: `doc:architecture` [CLI Version Compatibility]
