# security extract

## Relevance
relevant — the chunk adds two code-bearing, non-crate binaries to the pinned bin dir and changes which DLL `viola run` loads. That touches the plan's code-bearing-artefact integrity, supply-chain and PATH/search-order resolution controls. The plan names no ConPTY sideload today, so several mandates below need extending to it as amendments.

## Constraints
- **No runtime network.** The `viola` binary makes no outbound network calls (per security-plan §Threat Model Summary, Infrastructure → Networking; §Error Handling, "Error reporting integration"). The nupkg bytes may reach the build or bin dir only through a build/CI-side step. A download at runtime from `viola` would widen the boundary, and only the founder can rule on that.
- **Pinned by SHA-256 and re-hashed before reuse.** §Data Protection (At rest → Code-bearing artefacts) requires the pinned exe to be re-hashed with SHA-256 (`sha2 =0.11.0`, Decisions Log `2026-09-25`) before reuse, with a refusal on mismatch. The chunk must give `conpty.dll` and `OpenConsole.exe` the same cryptographic pin-and-re-hash treatment before any load or launch. The plan names only `viola(.exe)` here, so extending the mandate to the two new files is a plan amendment.
- **How the files are written, and the strict-modes check.** §Authentication & Authorization (`~/.viola/` access control) requires:
  - every file under the home to be written through `viola_state::fs::replace_private`, with its mode set before any byte is written;
  - the Windows owner + DACL strict-modes check to cover the `bin/<version>-<hash>/` folder in use and each trusted file separately, because a file can carry an explicit, non-inherited ACE.

  The plan's trusted-file list names only the pinned `viola.exe`, so adding `conpty.dll` and `OpenConsole.exe` to it is an amendment. The check itself belongs to the Epoch 6 entry "Home and code-bearing file integrity". Whether the code runs any Windows strict-modes check on `bin/` today is research's question.
- **Absolute path only.** Code-bearing artefacts resolve by absolute pinned path, never by a search-order or PATH lookup (per §Input Validation, "Child executable resolution" row; §Security Anti-Patterns → Universal, the exec-form `command` ban). §Threat Model Summary's attack-surface vector "Child process spawning and PATH resolution" does not yet record DLL search-order planting: portable-pty's bare-name `conpty.dll` load is not listed there. The chunk must amend it to list the vector and its control, for both the sideload-present case and the absent/rejected case.
- **No off switch.** No `config.json` key, `VIOLA_*` variable or CLI flag may switch the hash, signature or load-path control off (per §Security Anti-Patterns → Universal). The "without sideload" H2 leg must not add a toggle a release build can reach. Any env seam for it must live only under `cfg(feature = "fake-agent")` and needs its own Decisions Log entry (per §Input Validation, the test-seam rows; §Secret Management, Storage).
- **Dependencies.** Only windows-sys features may grow (e.g. WinTrust, LibraryLoader). No C-building crate, and `cargo deny check` stays green (per §Dependency Security, "`deny.toml` additions" and "Pinning"). The NuGet package is a non-crate source outside `cargo deny`'s graph, so its provenance and MIT licence are recorded as their own gate or Decisions Log entry, never as a silent exemption (per §Security Decisions Log `2026-09-27` browser pipe: "own audit, never an exemption"; `2026-09-25` Conditions: "each further licence exception needs its own entry").
- **Handle inheritance.** Channel handles stay non-inheritable, and no PTY path may spawn with `bInheritHandles = TRUE` in a way that leaks them (per §Data Protection, In transit → Handle inheritance; §Security Anti-Patterns → Data Protection). Whether the sideloaded `conpty.dll` launches `OpenConsole.exe` with only its own pseudo-console handles is research's question.

## Patterns to follow
- **The pinned-exe lifecycle:** write through `replace_private`, key and re-hash with `sha2`, refuse on mismatch (per §Data Protection, Code-bearing artefacts; §Bootstrap phases, `viola-state` SHA-256 re-hash). This is the model for the two sideload files. `viola_state::pin::pin_exe` is the existing writer that research should read.
- **The `scripts/install-node.sh` pin model:**
  - version and sha256 have one home, parsed from file text and never from the environment;
  - the official download is refused on a checksum mismatch before anything is extracted;
  - a `--probe` shows the refusal.

  (per §Dependency Security, Pinning.) This is the model for fetching the nupkg and pinning its per-file hashes.
- **Resolve to an absolute path and refuse the unsafe form.** Resolve code to an absolute path and never hand a bare name to a loader (per §Input Validation, Child executable resolution: the `.cmd`/`.bat` → real `.exe` recipe and `Refusal::BatchScriptChild`).
- **Record each new supply-chain item or boundary widening as a Decisions Log entry,** with Decision / Rationale / Boundary widening / Conditions / Witness / By (per §Security Decisions Log, entries `2026-09-25` and `2026-09-27`).

## Anti-patterns to avoid
- **Never load a code-bearing artefact by bare name or search order.**
  - Never reuse one without a cryptographic re-hash; FNV and `DefaultHasher` are banned (per §Security Anti-Patterns → Data Protection, the `bin/<version>-<hash>/` re-hash ban; → Universal, the PATH-resolution ban).
  - A planted `conpty.dll` in the app dir, CWD or `PATH` must never reach `viola run`.
- **Never let a sideload failure block the human** (per §Security Anti-Patterns → Universal: "NEVER let a security refusal block the human").
  - An absent, mismatched or unsigned sideload degrades to the inbox ConPTY.
  - That differs from the pinned `viola.exe`, whose mismatch refuses start with exit 1 (§Data Protection). P4 must make the difference explicit in the amendment rather than borrow either rule silently.
- **Never add a release-reachable switch that disables the hash, signature or load-path check** (per §Security Anti-Patterns → Universal, the `config.json` / `VIOLA_*` / flag ban).

## Contract bindings
- **security ↔ obs:** the backend-used / fallback-reason field (`pty.spawn` `pty_backend`, obs-plan) must be codes-only.
  - It carries no absolute path, file hash input or loader error text in any external error.
  - Full detail goes only to `instances/<name>/diagnostics/` (per §Bootstrap phases, `logging-redaction-wire`; §Error Handling).
- **security ↔ tests:** the H2 measurement job, and any CI fetch step for the nupkg, must keep:
  - actions SHA-pinned with no new mutable `uses:`;
  - `permissions: {}` / `contents: read`;
  - zizmor green;
  - no `github.event` value inside `run:`;
  - uploads gated on `secret-scan` (per §Dependency Security, CI integration; §Secret Management, "Secret scanning in CI").

  A `security_negatives_*` case for the planted-DLL and tampered-file refusals belongs to the tests plan.
- **security ↔ arch:**
  - New files under `bin/<version>-<hash>/` go into §Occupied Resources.
  - Whether the dir's `<hash>` (the exe bytes only, Decisions Log initial entry, amendment 8) should also cover the sideload is arch's question.
  - `scripts/release-check.sh` judges "viola only" (§Threat Model Summary, Infrastructure → CI/CD). If the sideload ships with or inside the release build, that check's contract changes.
- **security ↔ route CARRY:** if the chunk's plan touches `scripts/wsl-provision.sh` or ci.yml's `test`-job tool line, the operator-only `--install-deps` condition becomes this chunk's acceptance criterion: root runs only `apt-get install` over an allowlisted dry-run list (per §Secret Management, Development; Decisions Log `2026-09-27` browser pipe, Conditions).

## Acceptance criteria contributions
- **Planted DLL.** Put a `conpty.dll` in the CWD and in a `PATH` directory, with the sideload present, absent and tampered in turn. In every case `viola run` never loads the planted file: the loaded `conpty` module is the pinned bin dir's verified copy or the inbox ConPTY, and nothing else (per §Threat Model Summary, attack surface "Child process spawning and PATH resolution"; §Security Anti-Patterns → Universal).
- **Tampered or unsigned sideload.** A sideload file whose SHA-256 differs from its pin, or whose Authenticode signature fails or does not chain to Microsoft, is never loaded or launched. `viola run` still starts on the inbox ConPTY and records the backend as a code, with no path in any external output (per §Data Protection, Code-bearing artefacts; §Security Anti-Patterns → Universal, human always wins; §Error Handling).
- **Supply chain.** After the windows-sys feature growth and any CI fetch step, these all stay green: `cargo deny check`, `bash scripts/deny-probes.sh` and `zizmor .github/workflows/`. No C-building crate is added. The nupkg fetch refuses a sha256 mismatch before extracting, and its `--probe` shows the refusal (per §Dependency Security).
- **No network in `viola`.** A grep over `crates/` and `src/` finds no new network client or download path in the `viola` binary (per §Threat Model Summary, Infrastructure → Networking).
