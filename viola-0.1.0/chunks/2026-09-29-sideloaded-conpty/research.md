# Codebase Research — 2026-09-29-sideloaded-conpty

## Scope
- **Depth:** deep · **Reads:** 14 · **Globs/Greps:** 12 · **Graph queries:** 2 (rust plane)
- **Harness rules consulted:** none. No live leg in this chunk: the H2 measurement is a CI job on the windows-2025
  runner, and the E2E evidence is the existing suites.
- **Platform issues consulted:** no issue-tracker search, because no runner-only failure is folded (CI on `fb78ddc` is
  green). One platform source was read directly: microsoft/terminal `src/winconpty/winconpty.cpp` @ `main`, fetched raw
  (below). It is the `main` branch, not the `v1.24.260710001` tag. The two behaviours this plan relies on (the host path
  and the handle list) are re-measured in /implement against the pinned binaries, never assumed from `main`.

## Measured facts (this host: Windows 11 Pro 10.0.26200, 2026-09-29)
1. **The package.** `Microsoft.Windows.Console.ConPTY` has these versions on nuget.org:
   - stable: `1.24.260303001` · `1.24.260512001` · `1.24.260710001` (published 2026-07-13, the newest stable);
   - `1.25.*-preview` ×4.
   - Every version is licence `MIT`, authors `Microsoft`, project `github.com/microsoft/terminal`.
   - Source of the list: `https://api.nuget.org/v3-flatcontainer/microsoft.windows.console.conpty/index.json` plus the
     registration index, both fetched.
2. **The pinned candidate `1.24.260710001`.**
   - The `.nupkg` is 1 732 296 B, sha256 `175640566a3b59c4b132070ee96c2c77e5ab7edd2e92732a5eb3610bbf63d90e`, and carries a
     `.signature.p7s`.
   - x64 files, as `unzip` listed them:
     - `runtimes/win-x64/native/conpty.dll`: 109 920 B, sha256
       `39fba2713e2495117b1591ae8c32a3b904bea7aa66069cf7815e2844c76d75d8`
     - `build/native/runtimes/x64/OpenConsole.exe`: 1 066 296 B, sha256
       `b7fd936c2668b87b9ecf7b3366dc6568afc1c6f981874cba3e955a1c35cf8160`
   - arm64 and x86 builds of both files are also in the package.
   - The package's own `build/native/*.targets` copies `conpty.dll` beside the app and `OpenConsole.exe` into an
     `<arch>\` subdirectory (`DestinationSubDirectory x64\`).
3. **Signatures.** Measured with `Get-AuthenticodeSignature`:
   - both x64 files read `Valid`, `Authenticode`, subject `CN=Microsoft Corporation, O=Microsoft Corporation,
     L=Redmond, S=Washington, C=US`, issuer `CN=Microsoft Code Signing PCA 2024`;
   - a warm check of both files takes 15.7–16.0 ms (five runs, the first 24.0 ms), timed in PowerShell, cmdlet overhead
     included — a proxy for a WinVerifyTrust call.
4. **How `conpty.dll` finds its host** (`winconpty.cpp` @ `main`, `_ConsoleHostPath` :49-99). It tries, in order:
   - `<dll dir>\OpenConsole.exe`;
   - then `<dll dir>\<x64|arm64|x86>\OpenConsole.exe` (chosen by `IsWow64Process2`'s native machine);
   - then **silently** `\\?\<System32>\conhost.exe`.

   The dll dir comes from `GetModuleFileNameW` of the conpty module itself: there is no `PATH` or CWD search. So a loaded
   sideloaded `conpty.dll` with a missing `OpenConsole.exe` runs on the **inbox** conhost. That is a third state beside
   "sideload" and "inbox".
5. **What the host process inherits** (`winconpty.cpp` :213-270). The host is started through `CreateProcessAsUserW`
   with `bInheritHandles = TRUE`, restricted by `PROC_THREAD_ATTRIBUTE_HANDLE_LIST` to four handles: server, input,
   output, signal. So no viola channel handle can reach `OpenConsole.exe` through inheritance.
6. **The bare-name load at HEAD is plantable** (probe `scratchpad/conpty/loader-probe.ps1`: one fresh `powershell.exe`
   per case, planted files = copies of `System32\version.dll` renamed `conpty.dll`, real sideload = the pinned x64
   `conpty.dll`):
   - default search, plant in CWD → `LoadLibraryW("conpty.dll")` loads **the CWD plant**;
   - default search, plant only on `PATH` → loads **the PATH plant**;
   - `SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_SYSTEM32)` with both plants → `NULL`, error 126;
   - `LoadLibraryExW(<abs path>, 0, 0)` first, then the bare name → **the pre-loaded module** (both plants present);
   - restriction plus pre-load → the pre-loaded module.

   portable-pty 0.8.1 makes exactly this bare-name call: `src/win/psuedocon.rs:54`
   `ConPtyFuncs::open(Path::new("conpty.dll"))` → `shared_library` `dynamic_library.rs:340` `LoadLibraryW`, reached once
   per process through the `lazy_static` at :61-63. A planted DLL's `DllMain` therefore runs inside `viola run` today,
   whatever happens to its exports.
7. **`System32\conpty.dll` does not exist on this host**, nor `SysWOW64\conpty.dll` (measured with `ls`). On the
   windows-2025 runner this is unmeasured, so a restricted bare-name load there is expected to fail and fall back to
   kernel32 only if the runner matches. It is witnessed by the backend the CI run records.

## Files inspected
- `D:/dev/rust/cargo/registry/.../portable-pty-0.8.1/src/win/psuedocon.rs` (full): `load_conpty` :44-59 opens `kernel32.dll`
  first (panics if absent), then prefers a bare `conpty.dll`. `CreatePseudoConsole` flags `RESIZE_QUIRK | WIN32_INPUT_MODE`
  at :86. The child is spawned with `bInheritHandles = 0` (:141).
- `shared_library-0.1.9/src/dynamic_library.rs` (:323-340): `open` → `LoadLibraryW(filename)`, with no flags.
- `crates/viola-pty/src/lib.rs`:
  - :1-160: `PTY_BACKEND` is a compile-time const (:21), recorded on the `pty.spawn` span (:119-123).
  - `spawn` :124-146 is the one seam into `native_pty_system()`.
  - :400-800: the H2 test rig: the `pty_child_entry` child, the `viola-pty-watch` reports, the `count_dsr` DSR counter,
    and the `resize`/`key` step helpers :705-715.
  - `spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` :718-750 now waits for `size 120x40`
    before its key (:735).
- `crates/viola-pty/Cargo.toml` (full): deps portable-pty, tracing, windows-sys (Windows), libc (Unix). The features come
  from the workspace declaration.
- `Cargo.toml`:
  - :201: `windows-sys =0.61.2` features are declared ONCE at the workspace level, with `Win32_System_LibraryLoader`
    already in the list (so every crate taking `windows-sys.workspace = true` has it, viola-pty included).
  - `Win32_Security_WinTrust` and `Win32_Security_Cryptography` are absent.
- `crates/viola-state/src/pin.rs` (full): `pin_exe` :64-85 is the only pin writer.
  - It writes `bin/<VERSION>-<16 hex of the exe's SHA-256>/viola(.exe)` via `replace_private_shared(.., DIR_MODE)` when
    absent.
  - When present, it re-hashes in 64 KiB chunks and returns `HashMismatch` on any difference, leaving the file as found.
  - `PinError` is thiserror with fixed messages.
- `src/cmd/run.rs`:
  - :112-199: the start order as coded is resolve → collision → `pin_and_plugin` (:229-247, span `run.pin_copy`, outcome
    `ok` | `pinned-hash-mismatch`) → strip plan → version gate → bind → `start_state` → `child_launch` → `spawn_child`
    (:305-333, `HostTerminal::enter()` then `viola_pty::spawn`, then `run::log_child_start`).
  - `refused(detail)` :201-204 logs `process-exit{exit_code:1}`.
- `src/main.rs` :45-70: the first statement is `std::panic::set_hook`, then `role_of`, then the `catch_unwind` around
  clap + dispatch. No DLL-search call exists anywhere (grep `SetDefaultDllDirectories|LoadLibrary` over
  `src crates tests`: 0 hits).
- `src/run/mod.rs`:
  - :86-122: `child_launch` puts the pinned dir FIRST on the child's `PATH` (:102-109). A file placed directly in
    `bin/<key>/` therefore sits on the `PATH` of the whole `claude` process tree.
  - :46: `process-start{subject:"claude-child"}` carries `pty_backend = viola_pty::PTY_BACKEND`.
- `schemas/diag-line.v1.json` :73: `pty_backend` is `{ "type": "string" }` — a free string in the schema, not an enum.
  The closed set lives in obs-plan's §6 catalog, which G4's schema does not enforce by value.
- `src/panic_frames.rs` :50-55: the only existing `LibraryLoader` user (`GetModuleHandleExW` / `GetModuleFileNameW`); it
  loads nothing.
- `scripts/release-check.sh` :3-37: it judges cargo's `compiler-artifact` records, never a directory listing. Bytes
  embedded in `viola` add no artifact.
- `.github/workflows/ci.yml` (job map, :18-325):
  - `test` and `perf` each run on `[windows-2025, macos-latest, ubuntu-latest]`;
  - the Windows test leg runs the pwsh shim `run --coverage`, then the harness lifecycle, the browser suite, G2, G4, the
    capture, the secret scan, the uploads and the gate verdict.
- `git show d8b5051 -- .github/workflows/ci.yml` (the H2 loop, removed at `6d05367`):
  - a bash step `if: matrix.os == 'windows-2025'` after the pwsh coverage step;
  - 200 × `cargo llvm-cov nextest --no-report --profile ci -p viola-pty -E 'test(/spawn_runs_a_raw_child_that_sees_its_size_a_resize/)'`;
  - `TMP`/`TEMP` pointed at `$RUNNER_TEMP/h2-tmp`;
  - a lost iteration = a failed run with a kept `viola-pty-watch` report, whose contents it prints;
  - the JUnit is saved and restored around the loop;
  - the verdict line is `h2-loop: iterations N · losses M`.
- `git show dce98ad -- crates/viola-pty/src/lib.rs`: split `resize_then_key` into `resize` + `key` and inserted
  `wait_line(.., "size 120x40")` before the key.
- Environment: the release `target/release/viola.exe` is 2 132 480 B (2026-09-28 build). There are no committed binaries
  (`git ls-files | grep -E '\.(dll|exe|nupkg|bin)$'`: 0 hits). `.gitattributes` is `* text=auto eol=lf` only.

## Graph impact (rust plane; trace `tree-query-2026-09-29-sideloaded-conpty.json`)
Line numbers below are 0-indexed graph lines + 1.
- **`viola_pty::spawn`** — 4 callers:
  - `cmd/run/spawn_child` @ `src/cmd/run.rs:323`;
  - `harness/supervise/spawn_in_pty` @ `crates/viola-e2e/src/harness/supervise.rs:40`;
  - `OuterPty::spawn_sized` @ `tests/support/outer_pty.rs:49`;
  - `tests/spawn_child_entry` @ `crates/viola-pty/src/lib.rs:626`.

  All four reach portable-pty's bare-name `conpty.dll` load in their own process: `viola run`, the harness binary and
  the root test binaries (outer PTY drivers), and the viola-pty test binary. The signature of `spawn` need not change.
- **`pin_exe`** — 2 product callers, `cmd/run/pin_and_plugin` @ `src/cmd/run.rs:231` and `cmd/verify/measure` @
  `src/cmd/verify.rs:125`, plus its 5 own tests in `crates/viola-state/src/pin.rs`.
  - `verify` pins too, so a sideload written by the pin path lands in verify-stamped test homes as well.
  - `tests/cli_verify.rs` pins verify's step lines by literal, so the pin path must add no output line.
- **`PTY_BACKEND`** — 3 uses besides its definition (grep `PTY_BACKEND|pty_backend` over `src crates` `*.rs`):
  `crates/viola-pty/src/lib.rs:122` (span field), `src/run/mod.rs:46` (`process-start`), and `src/cmd/run.rs:615` (a test
  asserting the span equals the const).

## Patterns detected
- **Write-if-absent, re-hash-before-reuse** (`crates/viola-state/src/pin.rs:71-78`). The plugin is the opposite pattern
  (rewritten every start, `src/cmd/run.rs:241-245`). For a loaded DLL or a running exe, only write-if-absent is safe: an
  in-use file refuses a replace with OS error 5 (the arch history's `REPLACE_ATTEMPTS` trap).
- **Compiled-in content, key follows the bytes** (`include_str!` plugin; `content_key` over the exe bytes,
  `pin.rs:44-46`). Bytes embedded in `viola` change the `bin/<key>/` key by construction, so a pinned dir never mixes an
  old exe with a new sideload.
- **Fixed-message error enums** (`PinError` thiserror, `pin.rs:15-21`; `PtyError` hand-written `Display`,
  `lib.rs:66-89`). A sideload error joins the owning crate's existing enum, never a new one (one-enum rule).
- **A measurement-only CI step, then byte-identical removal** (`d8b5051` / `6d05367`), with the verdict line printed by
  the step itself.
- **Pinned official download with a `--probe`** (`scripts/install-node.sh`, `scripts/install-ripgrep.sh`): sha256 checked
  before extraction, the pin in one textual home.

## Conventions to follow
- **Fail-open toward the human, codes-only record**: `refused(detail)` exists for exit-1 refusals only
  (`src/cmd/run.rs:201`). The sideload degrade must not route through it, and must write no stderr line (`src/human.rs`
  is the only human writer).
- **Span per start step**: every start step is an `#[instrument(skip_all, name = "run.<step>", fields(outcome = Empty))]`
  with the outcome recorded as a code (`run.pin_copy`, `run.collision_check`, `run.version_gate`: `src/cmd/run.rs:207,
  228`; the span-order test at :562-615).
- **`cfg(windows)` blocks with a `// SAFETY:` line on each `unsafe`** (`crates/viola-pty/src/lib.rs:486-495`,
  `src/panic_frames.rs:50-55`).
- **Test children under a PTY are the test binary itself**, dispatched by an env var (`pty_child_entry`,
  `lib.rs:424-469`). Their reports go under `<temp>/viola-pty-watch/`, kept on failure.

## New files to create
- `crates/viola-pty/src/sideload.rs` — Windows-only: the process-wide DLL search restriction and the pre-load of a
  verified `conpty.dll` by absolute path, returning which ConPTY the process will use (a code). No pinned-path knowledge.
- `src/conpty.rs` — Windows-only: the pinned package constants (version, per-file SHA-256) and the embedded bytes of the
  two x64 files, handed to the pin path.
- `scripts/conpty-vendor.sh` — fetches the pinned `.nupkg` from nuget.org, refuses a sha256 mismatch before extracting,
  extracts the two x64 files, checks their per-file sha256 and their Authenticode signer; `--probe` proves each refusal.
- `vendor/conpty/1.24.260710001/x64/conpty.dll` — the pinned x64 `conpty.dll`, byte-identical to the package's (branch
  "committed" of the delivery fork).
- `vendor/conpty/1.24.260710001/x64/OpenConsole.exe` — the pinned x64 host, byte-identical to the package's (same
  branch).
- `tests/conpty_sideload.rs` — `cfg(windows)` integration cases in real stamped homes: sideload present, absent,
  hash-tampered, OpenConsole missing, planted `conpty.dll` in CWD and on `PATH`; each asserts the recorded backend code,
  empty `run` streams, and no path in any home-level line.

## Files to modify
- `src/main.rs` — the DLL search restriction as the second statement, after the panic hook, on Windows.
- `src/cmd/run.rs` — the sideload step between the pinned copy and the spawn: pin the files, verify, pre-load, and record
  the backend code on its span; its test module's span-order test (:562-615) gains the new start span and the backend
  assertion.
- `src/run/mod.rs` — `process-start{subject:"claude-child"}` carries the backend actually used instead of the const.
- `crates/viola-pty/src/lib.rs` — `pub mod sideload`; the backend value on `pty.spawn` becomes the process's actual
  backend; the measurement-only H2 race test (added for the measurement pushes, removed before the wrap).
- `crates/viola-pty/Cargo.toml` — windows-sys features only if a crate-local feature is needed (the workspace list is
  the home today).
- `crates/viola-state/src/pin.rs` — write-if-absent and SHA-256 re-hash of the two sideload files in a subdirectory of
  the pinned dir; `PinError` gains no new enum (a variant at most).
- `Cargo.toml` — windows-sys `Win32_Security_WinTrust` (+ `Win32_Security_Cryptography`) only if the signature is checked
  at run time.
- `.gitattributes` — `*.dll` / `*.exe` marked `binary` (only on the committed branch of the delivery fork).
- `.github/workflows/ci.yml` — the measurement-only H2 loop for the with/without pushes, then byte-identical to its
  pre-chunk form; the vendor verification step if the delivery fork takes the fetch-in-CI branch.

## Open questions
- **Delivery of the pinned bytes into the build.** Commit the two x64 files under `vendor/conpty/<version>/x64/`, or fetch
  the `.nupkg` in every build context (CI legs, the pre-push host, `cargo install --path .`) through a pinned script? Both
  end in `include_bytes!` inside `viola`. → blocks: plan-decision
- **Where the signature is checked.** At vendoring/CI time only (the script plus a CI Windows step; run time checks only
  the SHA-256 of bytes whose signature was checked when pinned), or also at every `viola run` through WinVerifyTrust?
  The run-time check needs `Win32_Security_WinTrust`, about 16 ms warm (fact 3), and
  `WTD_REVOKE_NONE | WTD_CACHE_ONLY_URL_RETRIEVAL` to guarantee no network. → blocks: plan-decision
- **Loading a DLL from `bin/` before Epoch 6's strict-modes check.** Is it a new interim gap that needs the founder's live
  ratification? The pinned `viola.exe` is already re-hashed without strict-modes today. The sideload load can hold a
  no-write-share handle across its hash check and `LoadLibraryExW` to close a swap window. → blocks: plan-decision
