# Scope — Sideloaded ConPTY

**Marker:** 2026-09-29-sideloaded-conpty · **Version:** viola-0.1.0 · **Epoch:** Epoch 2b — Windows slice I b: events and ledger
**Working entry:** `working-route.md:61` — "Sideloaded ConPTY — Microsoft NuGet conpty.dll and OpenConsole.exe beside the
pinned viola.exe, version and hash pinned, signature checked, loaded only from that path"

## Intent
viola on Windows hosts the child in Microsoft's current ConPTY (`conpty.dll` + `OpenConsole.exe`, from the official
NuGet package) instead of the inbox kernel32/conhost ConPTY. The files sit beside the pinned `viola.exe` and are loaded
only from there, only after a version, hash and signature check. The acceptance is a measurement: the H2 200-iteration
loop on windows-2025, run once with the sideload and once without.

## Founder ruling (CONTEXT, folded verbatim in substance)
- Founder ruling 2026-09-29, live, relayed by the overseer at the 2026-09-29-verify-stamped-test-homes-and-harness wrap:
  acceptance is the H2 200-iteration loop on windows-2025 run with and without the sideloaded ConPTY.
- Overseer relay at this take-up: the founder ruled this entry live on 2026-09-29 at 08:48 after being shown its
  conditions: **the official NuGet package, a pinned version and hash, a checked signature, and loading only from the
  pinned path.** A boundary widening outside those four conditions is a question for the founder (reachable live today),
  never a lean.

## What it builds
- **Provenance:** one pinned release of Microsoft's ConPTY NuGet package, named by package id, exact version and the
  SHA-256 of the `.nupkg`. The two files viola uses (`conpty.dll`, `OpenConsole.exe` for the host architecture) are
  pinned by their own SHA-256. Verified (research facts 1-2): the package id is `Microsoft.Windows.Console.ConPTY`, and
  the newest stable is `1.24.260710001`. That package holds:
  - `runtimes/win-x64/native/conpty.dll`;
  - `build/native/runtimes/x64/OpenConsole.exe`;
  - the arm64 and x86 builds of both files.
- **Delivery:** how the pinned bytes reach the pinned bin dir (`<home>/bin/<version>-<hash>/`) beside `viola.exe`. This
  is an open fork for P4. The bytes either travel inside the build (fetched by a pinned script, then embedded or copied)
  or get committed to the repo. A runtime download from `viola` itself would be a network boundary viola does not have
  today, which makes it a founder question. Research facts: embedding adds 1 176 216 B to a 2 132 480 B release
  `viola.exe`; release-check judges artifact records, so embedded bytes add no artifact; the repo commits no binary
  today.
- **Signature check:** the Authenticode signature of both files is checked and must chain to Microsoft. Verified (research
  fact 3): both x64 files are `Valid`, subject Microsoft Corporation, issuer Microsoft Code Signing PCA 2024, about 16 ms
  warm. Where it is checked stays P4's fork: at vendoring/CI time, or also at run time through WinVerifyTrust, which
  needs `Win32_Security_WinTrust` (absent from the workspace list) and no-network flags.
- **Load-path control:** `conpty.dll` loads only from the pinned bin dir, by absolute path, after the hash check;
  `OpenConsole.exe` is launched only from beside it. Verified at HEAD and by measurement (research fact 6):
  - portable-pty `=0.8.1` `src/win/psuedocon.rs:54` loads a bare `conpty.dll` through `LoadLibraryW`;
  - with the default search order it loads a planted copy from the CWD, or from `PATH` when the CWD is clean — so today
    a planted DLL's `DllMain` runs inside `viola run`;
  - the control must cover both the sideload-present case and the absent/failed case.
  - Mechanism, verified by measurement (research fact 6): a pre-load by absolute path makes the later bare-name load
    return that module, and `SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_SYSTEM32)` makes a bare-name load with plants
    in the CWD and on `PATH` fail (126), so portable-pty falls back to kernel32. System32 holds no `conpty.dll` on this
    host; the runner is unmeasured, and the backend CI records is its witness.
  - [premise-corrected: winconpty.cpp @ main `_ConsoleHostPath` :49-99 — the host is `<dll dir>\OpenConsole.exe`, then
    `<dll dir>\<arch>\OpenConsole.exe`, then silently `System32\conhost.exe`; there is no `PATH` search, but a missing
    `OpenConsole.exe` gives a third state, the sideloaded dll on the inbox conhost, which the check must refuse or
    record.] `OpenConsole.exe` is found by `conpty.dll` from its own module path, never through `PATH`. Its handle
    inheritance is limited to four handles by `PROC_THREAD_ATTRIBUTE_HANDLE_LIST` (research fact 5).
  - [premise-corrected: `src/run/mod.rs:102-109` puts the pinned dir FIRST on the child's `PATH`] Files placed directly
    in `bin/<key>/` would sit on the `PATH` of the whole `claude` process tree. "Beside the pinned viola.exe" is taken as
    the pinned dir's own subtree; a subdirectory keeps the sideload off the child's `PATH`. The exact layout is P4's.
- **Fallback behaviour:** when the sideload is absent, fails its hash check or fails its signature check, `viola run`
  still runs on the inbox ConPTY (the human always wins, and transport never blocks on an optional component) and records
  which backend it used. [premise-corrected: `schemas/diag-line.v1.json:73` types `pty_backend` as a free string, and
  `viola_pty::PTY_BACKEND` is a compile-time const (`lib.rs:21`) used at 3 sites] The value set is closed only by
  obs-plan §6's catalog. A new value or field needs the catalog row and a Decisions Log entry, not a schema-enum edit.
- **Pinning integration:** the pinned bin dir gains the two files, written 0600/0700 per the home rules through
  `viola_state::fs::replace_private_shared` and re-hashed before reuse like the pinned exe. Verified:
  - `pin_exe` (`crates/viola-state/src/pin.rs:64-85`) is the only pin writer;
  - it has two product callers, `run` (`src/cmd/run.rs:231`) and `verify` (`src/cmd/verify.rs:125`);
  - write-if-absent is the only safe form for an in-use DLL/exe (the os-error-5 trap).
- **The H2 measurement:** the 200-iteration H2 loop on the `windows-2025` runner, run twice — once with the sideload,
  once without — and both results recorded as this chunk's evidence.
  - [premise-corrected: `git show dce98ad` — the loop's target test
    `spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` now waits for `size 120x40` before its
    key, so re-running the `d8b5051` loop at HEAD would not exercise the H2 race at all] The measurement needs the race
    shape: the key written right after the resize while the child reads. It must exist only for the measurement pushes,
    and the with-sideload leg must pre-load the pinned `conpty.dll` in the viola-pty test process. Like the loop step
    (`d8b5051` added, `6d05367` removed, ci.yml byte-identical), it is removed before the wrap.
  - The acceptance is the measurement taken, not a particular result. Whether the sideload fixes H2 is the question the
    loop answers; a sideload that does not change the loss rate still ships if the conditions hold, and the result is
    recorded either way. Verified against the entry's CONTEXT: it names the with/without loop as acceptance and states no
    target rate. A target would be a P4 question, not a lean.

## Boundaries
- Windows only. macOS/Linux keep openpty; nothing there changes.
- Transport only: no change to the wheel, hooks, readiness, or anything Claude-specific. `viola-agent-claude` is not
  touched.
- portable-pty stays `=0.8.1` (architecture [PTY]); no fork, no swap to psmux, no own ConPTY on windows-sys.
- No new network access from the `viola` binary without the founder's word.
- No new C-building crate; windows-sys features may grow (a Cargo.toml change `cargo deny` sees). [premise-corrected:
  `Cargo.toml:201` — the features are declared once at the workspace level, where `Win32_System_LibraryLoader` already
  is.]
- The binary files' licence is MIT (research fact 1, the nuspec). It sits outside `cargo deny`'s crate graph, so it is
  recorded as its own Decisions Log entry (security history: own entry, never a silent exemption).
- The real `claude` key-after-resize impact is not claimed: "First live test and self-drive" owns it. This chunk
  measures the test child.

## Surfaces and contracts touched
- `crates/viola-pty` (Windows) — the load-path control: a search restriction and a pre-load of a caller-supplied,
  already-verified absolute path. The crate knows no pinned path. Verified: all four `viola_pty::spawn` callers (`viola
  run`, harness `supervise`, root `OuterPty`, viola-pty's own tests) reach the same bare-name load in their own process.
- `crates/viola-state` `pin` — the pinned bin dir's contents.
- `src/main.rs` / `src/cmd/run.rs` / `src/run/` — the search restriction at process start; start order pin → verify →
  pre-load → spawn; the backend recorded.
- Architecture [PTY] and the §Occupied Resources registry (new files under `<home>/bin/<version>-<hash>/`); security-plan
  (DLL load path, signature trust, supply chain of a non-crate binary); obs-plan (`pty.spawn` backend field); test-plan
  (the H2 loop, CI Windows legs).
- CI: `ci.yml` Windows legs gain what the delivery fork needs; the H2 loop runs measurement-only.

## CARRY folded (chunk 2026-09-27-browser-verdict-reachability, overseer live ratification, operator-only)
Before the WSL distro is next re-provisioned, `scripts/wsl-provision.sh --install-deps` must stop running user-writable
code as root. Root runs only `apt-get install` over the package list an unprivileged dry run produced (Playwright
1.63.0's `install-deps --dry-run` lists it and exits 1 while packages are missing, measured at that chunk's
evidence/wsl-chromium-deps.md), checked against a committed allowlist, so apt installs only signed distro packages. The
chunk that first re-provisions takes it; until then it moves on with the first markerless entry.
- Disposition for this chunk: **conditional**. Verified against research's touch list: neither `scripts/wsl-provision.sh`
  nor ci.yml's `test`-job `tool:` line is in it. If the plan comes to touch either, the CARRY becomes this chunk's work
  with its own acceptance criterion; otherwise it moves on unchanged.

## CI verdict read at Setup (5a), re-read at P3
- `fb78ddc` (the last wrap's flip, = HEAD): at Setup, **verdict not yet available** — run ci#36541599814 in progress,
  checks 15/15 registered, 4 running (oldest `test (ubuntu-latest)` 175 s). Re-read at P3 through `ci.py conclusion`:
  **green**, checks 15/15, wall-clock 305 s, ci#36541599814 completed/success. Nothing to fold.
