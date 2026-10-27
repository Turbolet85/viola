# security-plan — amendments

## 2026-09-24-three-os-ci-headless-harness-skeleton — pinned actions and toolchain source
**Section:** §Dependency Security (CI integration; Pinning) · §Threat Model Summary (Supply chain entry point)
**Change:**
- The SHA-pinned action set is now exactly what `ci.yml` uses: `actions/checkout` v7.0.1 (with `persist-credentials: false`), `Swatinem/rust-cache` v2.9.2, `taiki-e/install-action` v2.87.19, `actions/upload-artifact` v7.0.1.
- `dtolnay/rust-toolchain` is replaced by a `rustup toolchain install` step reading `rust-toolchain.toml`.
- The toolchain is the exact 1.98.1 pin; the workspace `rust-version` floor is 1.96.
- Event-payload values reach steps only through `env:`.
**Why:** the chunk shipped `ci.yml` and `rust-toolchain.toml`. The mutable-ref ban needs no change.
**Ref:** .andromeda/runs/2026-09-24T07-05-59-wrap/

## 2026-09-24-three-os-ci-headless-harness-skeleton — rejected: interim `--home` and R8-strip Decisions-Log entries
**Section:** none (proposals rejected)
**Change:** none.
**Why:**
- The walking-skeleton `viola run` has two known gaps: `--home` is not canonicalised or strict-modes-checked and the Windows protected DACL is not set; and it spawns the child with the full inherited environment, so the R8 `CLAUDE*` strip is not applied yet.
- Both are sequencing deferrals (playbook rule 1), owned by markerless route entries rather than body prose: "Home and code-bearing file integrity" and "CLI machine contract — global --home" own the first (each gets a `CARRY:` pin); "PTY wrapper on Windows" owns the second (it already names the CLAUDE* strip).
**Kept:** the §Input Validation CLI row, §Secret Management and the §Anti-Patterns Data Protection ban stay as the target.
**Ref:** .andromeda/runs/2026-09-24T07-05-59-wrap/

## 2026-09-24-supply-chain-and-workflow-gates — trust boundary, CI jobs, deny.toml additions, nightly.yml
**Section:** §Threat Model Summary → Supply chain (Trust boundary) · §Architecture Overview → CI/CD · §Dependency Security (`deny.toml` additions, CI integration)
**Change:**
- Trust boundary: names `deny.toml` with its four families plus the sole-root `deny-sync.toml` tokio ban.
- CI/CD: adds `nightly.yml` and the ubuntu supply-chain job.
- `deny.toml` additions: the heading no longer says arch lists only licences and bans (was that; now retired). The arch-bans bullet places the tokio ban in `deny-sync.toml` per sync crate as sole root, and records that `scripts/deny-probes.sh` proves every ban live.
- CI integration: the weekly advisory run is a separate workflow, `nightly.yml` (weekly `schedule` + `workflow_dispatch`, no cache), not a trigger on `ci.yml`.
**Why:** these sites restated claims the chunk's pass retired, so they were amended as routine cascade dependents.
**Kept:** the pinned-action set, because no new action was added.
**Ref:** .andromeda/runs/2026-09-24T09-41-13-wrap/

## 2026-09-24-diagnostics-plane — config.json diagnostics_level validation row, MAX_FRAME consumer
**Section:** §Input Validation (Configuration values row; Constants)
**Change:**
- The `config.json` row names the closed `diagnostics_level` (info | debug; any other value falls back to info and is reported as `parse-rejected`), the `MAX_FRAME` cap, and the read's home in the root-bin `viola::obs`.
- The `MAX_FRAME` consumer list adds the `config.json` read.
**Why:** the chunk shipped the `config.json` read; the row and consumer list omitted it. Routine accurate-additions.
**Ref:** .andromeda/runs/2026-09-24T10-40-06-wrap/

## 2026-09-24-observability-gates — CI runs the obs artifact canary scan before uploads
**Section:** §Secret Management ("Secret scanning in CI") · §Bootstrap phases (secret-scanning-ci-gate)
**Change:**
- Was "Not in v1" / "Not wired in v1"; now "no repo secret scanner in v1" (still true: no scanner was researched).
- Both sites record that CI runs obs-plan §9's artifact canary scan (`viola-harness secret-scan`, `id: secret-scan`) before every test-home upload, against this plan's NEVER-log floor, and names its classes. It never prints or writes matched bytes.
- The `mutants.out/` upload is noted as unscanned: a CARRY on "Quality gates".
**Why:** the previous text read as "CI does no secret scanning", which the new CI contradicts.
**Kept:** the Decisions Log entry and the Threat Model's verbatim CI list are history and unchanged; the Decisions Log line "Secret scanning in CI (no scanner researched)" stays, still true of a repo scanner.
**Ref:** .andromeda/runs/2026-09-24T13-07-17-wrap/

## 2026-09-24-quality-gates — fuzz lockfile exemption, fifth pinned action, fuzz and MSRV toolchains, unscanned uploads, declined concurrency
**Section:** Threat Model Summary (Supply chain entry point and trust boundary; Infrastructure CI/CD) · Dependency Security (`deny.toml` additions; Pinning ×2; CI integration: cargo-deny scope, nightly fuzz job, declined `concurrency:`, pinned actions, toolchains) · Bootstrap phases `secret-scanning-ci-gate` · Security Decisions Log (new 2026-09-24 entry)
**Change:**
- `fuzz/` is a separate workspace whose `fuzz/Cargo.lock` (`libfuzzer-sys =0.4.13`, which builds C++, and `arbitrary =1.4.2`) sits outside `cargo deny`: a test-only exemption, conditional on never linking and never joining the root `[workspace]`, with its advisory/source audit owed to "Workspace tree and code-graph planes".
- Both lockfiles are committed.
- `actions/download-artifact@3e5f45b… # v8.0.1` joins the pinned set (5).
- Toolchains: `rust-toolchain.toml` 1.98.1, `fuzz/rust-toolchain.toml` `nightly-2026-09-20`, and MSRV `1.96` via `RUSTUP_TOOLCHAIN`, all through rustup (was "no toolchain action"; now names the three rustup toolchains).
- `nightly.yml` gains the fuzz job.
- zizmor pedantic `concurrency-limits` is declined (with its reason).
- The `mutants.out/` CARRY is closed by removal. Two unscanned uploads are admissible by content: the verdict JSON (repo-relative source locations and outcomes only, never absolute paths), and the nightly `fuzz/artifacts/` from the synthetic corpus.
**Why:** the fuzz exemption with a CARRY'd audit and both unscanned uploads (admissible by content, repo-relative source locations only) were ratified by the overseer (founder-delegated) at wrap P2. The Threat Model is amended in place per the prior-wrap precedent.
**Ref:** .andromeda/runs/2026-09-24T14-48-15-wrap/

## 2026-09-24-workspace-tree-and-code-graph-planes — fuzz lockfile audit wired, release-check and orphans in the CI job list
**Section:** §Threat Model Summary (Supply chain trust boundary; Infrastructure CI/CD workflows and jobs) · §Dependency Security (the `fuzz/` bullet; CI integration Job 4 and the nightly workflow) · §Security Decisions Log 2026-09-24 (Conditions)
**Change:**
- The `fuzz/Cargo.lock` audit was "owed"; now wired. The ci.yml `supply-chain` step `Fuzz lockfile audit (advisories, sources)` runs `cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources` into `target/supply-chain/deny-fuzz.json`, and weekly `nightly.yml` `advisories` runs `cargo deny --manifest-path fuzz/Cargo.toml check advisories`.
- Both run from the repo root, where cargo-deny resolves the root `deny.toml`. The root run's families (the C-build ban included) still do not reach the fuzz graph; the root graph covers the root lock only.
- Job 4 states the root-lock scope and the same job's separate fuzz step.
- The Threat Model CI job list adds the cargo-modules orphans gate, the fuzz lockfile audit, and the per-OS release build through `scripts/release-check.sh` (refuses any test-only binary), replacing a bare `cargo build --release`.
- The Decisions Log Conditions record the CARRY as delivered.
**Why:** the chunk's report substantiates each change, so they were raised as routine; the operator's P4 decision added the nightly advisories.
**Kept:** the Decisions Log "outside cargo deny" history line and the nightly advisories statements without fuzz (the bootstrap phase that wired the weekly run; manual review driven by the weekly run) stay, still true.
**Ref:** .andromeda/runs/2026-09-24T16-23-20-wrap/

## 2026-09-25-security-prerequisites — SQOS spike passed, SHA-256 crate picked, 0BSD per-crate exceptions
**Section:** §Authentication & Authorization (IPC client-side server verification row); §Data Protection (code-bearing artefacts); §Dependency Security (`deny.toml` additions, new `[licenses]` bullet); §Bootstrap phases (`viola-channel` client and `viola-state` re-hash bullets); §Security Decisions Log (the two Open questions removed; new `2026-09-25` entry).
**Change:**
- The client-verification row: was an open spike; now the adoption spike passed (a same-user server reads `SecurityIdentification`), `FILE_FLAG_OVERLAPPED` is required, and the default connect reads `SecurityImpersonation` and stays banned.
- §Data Protection: the SHA-256 implementation was an open question; now `sha2 =0.11.0` (`default-features = false`).
- §Dependency Security: `allow` unchanged plus two per-crate `0BSD` exceptions (`doctest-file`, `recvmsg` via interprocess 2.4.4), never through `allow`; each further exception needs its own entry.
- Both bootstrap bullets cite the resolved decisions; the two Open questions are removed.
- The Decisions Log `2026-09-25` entry carries the measurements, conditions and ratification.
**Why:** the windows CI `test` leg passed the SQOS witness. The 0BSD exceptions widen a hardened boundary (playbook "Boundary widening", escalate); resolved by the overseer's (founder-delegated) ratification of per-crate exceptions at phase P4 and again at wrap P2.
**Kept:** the `windows-sys` restatements across the masters are consistent; the other client-open and DACL lines need no change.
**Ref:** .andromeda/runs/2026-09-25T13-11-43-wrap/

## 2026-09-25-pty-wrapper-on-windows — R8 persistent-environment exemption, registry name reader, child resolution as built
**Section:** Input Validation (Configuration values row; new row "Persistent-environment names (Windows registry)"; Child executable resolution row) · Secret Management → Storage (inherited credentials) · Error Handling (fixed `Display`) · Bootstrap phases (`error-sanitization-wire`) · Security Decisions Log (new `2026-09-25` entry)
**Change:**
- `config.json` `claude_env_keep`: ≤ 32 names matching `^CLAUDE[A-Z0-9_]*$`, rejected whole when bad, a floor name ineffective.
- New Input Validation row for the registry reader: value names only, bounded buffer, missing key → empty set, the identity floor overrides it.
- Child executable resolution as built: viola resolves the program to an absolute path (PATHEXT only on Windows, never the bare name), `claude.cmd`/`.bat` → fixed sibling `claude.exe` with the shim never read, every other `.cmd`/`.bat` → closed `Refusal::BatchScriptChild` (exit 1, no spawn, two fixed stderr lines), explicit cwd.
- Secret Management: R8 described as the prefix rule + persistent exemption + 11-name identity floor (both messaging variables on the floor), names only.
- `PtyError`'s `Display` is hand-written (the other enums thiserror); fixed messages unchanged.
- Decisions Log: new `2026-09-25` entry recording the ratification.
**Why:** a boundary widening (the child may receive persistent `CLAUDE*` names; `config.json` admits a key; registry names are read), ratified by operator ruling 1, confirmed at wrap P2.
**Kept:** the two Threat Model Summary edits were rejected — that section is the verbatim copy of threat-assessment.md; the winning sections carry the truth. The summary/rules leaves' "resolve to the real .exe" stays true.
**Ref:** .andromeda/runs/2026-09-25T17-43-18-wrap/

## 2026-09-26-ci-chunk-base-and-union-verdict — chunk.diff out of the canary scan and the harness upload; ci.yml reads no event value
**Section:** Dependency Security → CI integration (event-payload `env:` rule; declined `concurrency-limits` reason) · Bootstrap phases → `secret-scanning-ci-gate` · Secret Management → Secret scanning in CI
**Change:**
- The canary scan covers the harness capture except the mutation leg's `target/agent-run/chunk.diff` (repository source text by construction); the `harness-<os>` upload drops the same file, so it is never scanned and never uploaded; a `chunk.diff` elsewhere is still scanned. The two admissible-by-content unscanned uploads are unchanged.
- The event-payload rule stays; its example is retired: `ci.yml` reads no `github.event` value (1 → 0), the mutation base is derived by the harness, `AGENT_RUN_CHUNK_BASE` is only a local override.
- The declined-concurrency reason keeps the `always()` gate/upload chain and retires the "drop a push's `--in-diff` mutation diff" half.
**Why:** the chunk changed the harness scan scope and the mutation-base derivation; the concurrency reason was a cross-master restatement of the architecture reason.
**Kept:** no drift in the syn/proc-macro2 exact pins (MIT OR Apache-2.0, deny four-green) or in input validation.
**Ref:** .andromeda/runs/2026-09-26T20-59-23-wrap/

## 2026-09-26-local-linux-pre-push-gate — `FAKE_AGENT_PUMP_DELAY_MS` carve-out; WSL pre-push distro: CI's pins, `env -i`
**Section:** §Input Validation (boundary table) · §Dependency Security → Pinning · §Secret Management → Storage (Production, Development) · §Security Anti-Patterns → Universal · §Security Decisions Log `2026-09-27`
**Change:**
- Input Validation: a row for the test seam — `fake-agent`-only, u64 ms capped at 5 000, compiled out of release builds, not `VIOLA_*`, configures nothing, disables no control, widens no redaction.
- Anti-Patterns Universal: the env rule gains the one carve-out and a NEVER for another seam (or this one in a release build) without a Decisions Log entry; Storage (Production) points at it.
- Dependency Security: the WSL2 `Ubuntu` distro `pre-push` drives is a third install site of CI's own pins (`scripts/wsl-provision.sh`: sha256-pinned rustup-init 1.29.1, the `rust-toolchain.toml` channel, `cargo install --locked` of ci.yml's `test`-job line; `tool-pin-mismatch`; no sudo); the toolchain sentence names host, CI and the WSL2 distro.
- Storage (Development): every WSL call runs under `env -i` (HOME + PATH only), so no `CLAUDE*` value crosses — the second layer behind `WSLENV` forwarding only `WT_*`.
- Decisions Log `2026-09-27`: the seam and the gate's env/pin posture (ruling of record).
**Why:** the seam is a boundary widening (playbook "Boundary widening", never routine) — a test build of `viola` reads a new input; ratified by the overseer's (founder-delegated) wrap direction as a carve-out behind `cfg(feature="fake-agent")`, capped at 5 s, absent from release builds.
**Kept:** "every toolchain installed by `rustup`, no toolchain action" stays true.
**Ref:** .andromeda/runs/2026-09-27T00-51-08-wrap/

## 2026-09-27-instance-state-and-start-order — the tempfile `persist` helper; `TMPDIR` carve-out under `env -i`
**Section:** §Authentication & Authorization (Token / session storage; `~/.viola/` access control) · §Data Protection (plugin folder, `settings.json`) · §Bootstrap phases (auth-scaffolding-baseline) · §Secret Management (pre-push `env -i`) · §Dependency Security (the sha2 KAT line) · the pre-push Decisions/critical line (`env -i`)
**Change:**
- Was "atomic-write-file" at every site; now the shared tempfile `persist` helper `viola_state::fs::replace_private` (mode set on the temp file before any byte, then `sync_all` + `persist`); the plugin rewrite uses `replace_private_shared` (a failed replace counts as done only over byte-identical content).
- `env -i`: the Linux mutation leg's `TMPDIR=<distro home>/viola-pre-push-scratch` is recorded as the one named, constant, distro-derived assignment beside HOME and PATH — no host value; any assignment taking a host value stays a boundary widening needing its own Decisions Log entry.
- sha2: now a `viola-state` product dependency; the root dev-dependency stays for the KAT.
**Why:** the `TMPDIR` crossing is the boundary-widening class (never routine), ratified by the overseer (founder-delegated) before the fan-out as exactly this carve-out: a named, constant, distro-derived assignment, no host value.
**Kept:** the snapshot `endpoint` as a verification reference once bound stays true.
**Ref:** .andromeda/runs/2026-09-27T06-12-23-wrap/

## 2026-09-27-wrapper-channel — listener hardening as landed: SDDL through windows-sys, chmod 0600 after the bind
**Section:** §Authentication & Authorization (Library: IPC; IPC access control (Windows); IPC access control (Unix)) · §Bootstrap phases (auth-scaffolding-baseline: amendment 2, `viola-channel` listener hardening) · §Security Anti-Patterns → Authentication
**Change:**
- Windows: the protected DACL `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)` is converted by windows-sys (`Win32_Security_Authorization`) into `ListenerOptionsExt::security_descriptor`; was `SecurityDescriptor::deserialize` (not used: it needs `widestring`). Windows reads it back canonical as `D:P(A;;FA;;;<sid>)(A;;FA;;;SY)` with the SID possibly an alias (`LA`); the control is the protected flag plus exactly the two allow ACEs.
- Unix secondary control: was `ListenerOptionsExt::mode(0o600)` "where supported, `Unsupported` on macOS ignored"; now chmod 0600 right after the bind on every Unix OS (`mode` fails the bind on macOS, and ignoring it left the socket at the umask mode). The anti-pattern bans `ListenerOptionsExt::mode` and keeps "never the only control".
- Bootstrap: arch amendment 2 is folded (arch IPC endpoints); the SDDL, the per-user dir and the 0600 socket landed with "Wrapper channel", ahead of Epoch 6's admission entry; the `peer_creds` decision and the directory verification stay with their route entries.
**Why:** the report disproved the `mode(0o600)` premise and the GA read-back literal (CI run 36313377307). Weighed against "Boundary widening": not that class — nothing new crosses, and the socket now reaches 0600 on every Unix OS.
**Kept:** the Decisions Log IPC entry and the Threat Model Summary (a verbatim copy) keep their wording as history.
**Ref:** .andromeda/runs/2026-09-27T12-33-51-wrap/

## 2026-09-27-wrapper-channel — founder ratification of three standing boundary widenings
**Section:** §Input Validation (the `FAKE_AGENT_PUMP_DELAY_MS` test-seam row) · §Secret Management (pre-push `env -i`: the Linux mutation leg's `TMPDIR`) · the CI `supply-chain` artifact uploaded unscanned (obs-plan §8 item 6, admitted at chunk 2026-09-24-workspace-tree-and-code-graph-planes)
**Change:** no body text changes. The three carve-outs stand exactly as written: the `fake-agent`-only `FAKE_AGENT_PUMP_DELAY_MS` seam (capped at 5 s, absent from release builds); `TMPDIR=<distro home>/viola-pre-push-scratch` as the one named, constant, distro-derived `env -i` assignment; the `supply-chain` artifact (`deny.json`, `deny-fuzz.json`, `zizmor.json`) admissible by content, its guard clause binding.
**Why:** founder ratified 2026-09-27, relayed by the overseer (driving-guide provenance rule: a founder ruling is recorded as the founder's). Each widening had rested on an earlier overseer direction (founder-delegated); under the founder's ruling of the same day a boundary widening needs a live answer and an earlier direction never ratifies one, so these three were put to the founder live and ratified. The standing rule for later chunks: a widening halts the wrap for a live answer.
**Ref:** .andromeda/runs/2026-09-27T12-33-51-wrap/

## 2026-09-27-epoch-2-cleanup — wsl-exec.sh ratified (overseer, live), test-only feature proven absent, host scratch guard
**Section:** §Secret Management (the WSL crossing bullet) · Decisions Log (new `2026-09-27` entry)
**Change:**
- §Secret Management: the pre-push gate stays the only harness launcher into WSL2; `scripts/wsl-exec.sh` added as the one other launcher (operator aid; the same `env -i` form with the distro's HOME and PATH; only operator-typed argv and `--cd` cross). A checkable invariant: no gate, harness command or plan entry runs a distro command through it, its own `--probe` exempt.
- Decisions Log: viola-channel's `test-support` feature and release-check's refusal of `test-support` / `fake-agent` artifacts (probe `5/5`), the host mutation scratch guard (exact name, never the repository or an ancestor, no fallback), and the `wsl-exec.sh` widening with its ratification and Conditions.
**Why:** the `wsl-exec.sh` crossing is a boundary widening (playbook, never routine), ratified LIVE by the overseer at this wrap under the founder's 2026-09-27 ruling — recorded as the overseer's word, not the founder's own. The overseer then narrowed the invariant to running a command through it, so its `--probe` self-test stays a gate entry.
**Kept:** the 2026-09-26 entry's Conditions line stands as history; the new entry records it enforced.
**Ref:** .andromeda/runs/2026-09-27T17-20-44-wrap/

## 2026-09-27-browser-verdict-reachability — npm lockfile audit, pinned Node, operator-only root install in WSL
**Section:** §Dependency Security (Audit tool; Pinning, incl. the WSL2 bullet; CI integration: Job 4, nightly, Toolchains) · §Bootstrap phases (`dep-audit-tooling-install`, `dep-security-ci-gate`) · §Secret Management (Storage → Development) · §Security Decisions Log (new `2026-09-27` entry)
**Change:**
- The test-side npm graph `e2e-web/package-lock.json` (`@playwright/test` =1.63.0, 3 packages) gets its own gate: `scripts/npm-audit.sh` (advisories at every level + registry.npmjs.org-only sources; `--probe`; `--advisories-only`, the weekly `npm-advisories` twin), JSON in `target/npm-audit/`, never the uploaded `target/supply-chain/`.
- Node is pinned exactly: the `NODE_PIN_*` lines in ci.yml's workflow `env:` (their only home), parsed from the file text by `scripts/install-node.sh` (sha256 checked before extraction). Toolchains: Node from `Node (pinned)`, Chromium from Playwright's own install; no toolchain action, no new `uses:`.
- WSL2: the distro also installs the pinned Node and Chromium; `pre-push` refuses `node` off-pin; the `env -i` PATH gains only `<home>/.local/viola-node/bin`. "The script never runs sudo" became "the user run never runs sudo".
- Development: one root launch, `wsl.exe -d Ubuntu -u root … wsl-provision.sh --install-deps <user home>`, operator-only (never the gate tool, a harness command or a pre-push stage); it ran once on 2026-09-27 (28 packages). `wsl-exec.sh` is now "the one other user-level launcher" (was "one other launcher").
**Why:** founder ruling W125's pipe; P4 operator forks 1 and 3. The root launch is a boundary widening, ratified live by the overseer under the founder's 2026-09-27 ruling, operator-only, because as shipped root runs user-writable code; before any re-provision `--install-deps` must run only `apt-get install` over an allowlisted dry-run list (a route CARRY).
**Kept:** the Threat Model Summary is a verbatim upstream copy and was not amended; the facts live in §Dependency Security.
**Ref:** .andromeda/runs/2026-09-27T19-50-23-wrap/

## 2026-09-27-hooks-to-normalised-events — `hook.event` served; the hook's interim gap, ratified by the founder; hook stdin row
**Section:** §Security Decisions Log (new `2026-09-28` entry) · §Authentication & Authorization (IPC client-side server verification row, Timing; `~/.viola/` access-control row, strict-modes entry points) · §Input Validation (Channel frames; Hook stdin; CLI arguments / stdin, Home path; Own state files, Integrity) · §Error Handling (Internal logging) · §Security Anti-Patterns → Authentication
**Change:**
- The wrapper serves `hook.event` (re-validated: the five hook kinds, object `data`, `prompt-submitted` `text` string + `origin` ∈ harness|human; invalid → not appended), mapped to the unchanged IPC controls.
- Interim gap until the Epoch 6 entries "Server verification before any frame" and "Home and code-bearing file integrity": `viola hook` sends `hook.event` to the snapshot's `endpoint` with no server verification and no strict-modes, and shape-checks `VIOLA_DIR` only (absolute, ending `<home>/instances/<name>`, `VIOLA_NAME` through `ViolaName::try_new`). Each site that required both before any `hook.event` frame or snapshot read now names the dated exception; the Anti-Patterns ban says no other frame or process may borrow it.
- Hook stdin: `take(MAX_FRAME + 1)` (was `take(MAX_FRAME)`) so an over-limit payload is detected; `<event>` maps into the closed `HookEvent`; fail-open details `oversize-stdin` · `malformed-json` · `channel-unreachable`.
**Why:** a boundary widening (prompt text, `last_assistant_message` and tool names reach an endpoint read from an unchecked snapshot). Ratified by the founder, live, on 2026-09-28, after it was shown to him, relay: the Viola overseer; the P4 overseer ruling had not ratified it. The Epoch 6 entries own the verifier, and the hook must not grow a partial one; the first-instance pipe and the bind arbiter already stop a squatter taking a live name.
**Ref:** .andromeda/runs/2026-09-27T23-42-19-wrap/

## 2026-09-28-hook-perf-gate — test seam `FAKE_AGENT_HOOK_PANIC` and G2's exact-path exemption, ratified by the founder
**Section:** §Input Validation (test-seam rows); §Secret Management → Storage; §Security Anti-Patterns → Universal; §Security Decisions Log (`2026-09-28`, test seam)
**Change:**
- A second test-seam row: `FAKE_AGENT_HOOK_PANIC`, a closed value (panics only on exactly `1`), fixed synthetic payload, fired after `viola_obs_init` so `hook` still fails open; named only in `src/cmd/hook/seam.rs`; compiled out of release builds.
- Storage and Universal: "the one variable / the one carve-out" (`FAKE_AGENT_PUMP_DELAY_MS`) → two test seams; the ban now names another seam or another G2 exemption as needing its own entry.
- New Decisions Log entry: the seam, and G2 (`scripts/g2-zero-panics.sh`) not counting a panic line at exactly `src/cmd/hook/seam.rs:<digits>` (whole-string compare, never prefix, suffix, regex or home path); Conditions: `release-check` `viola only`, G2's `--probe` look-alike and empty-scope controls before every check.
**Why:** a boundary widening (a test build reads a new input; a zero-panics gate admits one location). Ratified by the founder, live: the seam at 06:21 on 2026-09-28, the seam with the G2 exemption (shown to him as new) at 09:52:07; relay the Viola overseer.
**Ref:** .andromeda/runs/2026-09-28T07-37-52-wrap/

## 2026-09-28-hook-perf-gate — CI integration: the per-OS `perf` job and its scan-gated uploads
**Section:** §Dependency Security → CI integration
**Change:** the `perf` job (`contents: read`) installs hyperfine with `cargo install --locked hyperfine@1.20.0` as its own step (no new action, no `github.event` value), runs `agent-run run --perf`, then G2, G4 and its own `secret-scan`; `perf-<os>` and `diag-perf-<os>` upload only on a successful scan, `secret-scan-perf-<os>` only on a failed one; every input is synthetic.
**Why:** an expected amendment no detector raised (Validate check 5); the uploads are the existing scan-gated, synthetic-input class (the operator's ruling, recorded in the obs sidecar).
**Ref:** .andromeda/runs/2026-09-28T07-37-52-wrap/

## 2026-09-28-capability-ledger-and-viola-verify — `run`'s and `verify`'s stamps reads before strict-modes, a ratified interim gap until Epoch 6
**Section:** Authentication & Authorization `~/.viola/` access control (Strict-modes check); Input Validation rows CLI arguments / stdin (Home path) and Own state files on read (Integrity); Security Decisions Log (new `2026-09-28` stamps-read + capture-arm entry)
**Change:**
- A second interim gap beside `hook`'s: `viola run`'s version gate reads `ledger/stamps.json` through `read_stamps` (no lock, `take(MAX_FRAME + 1)`, over the cap an error; unreadable or malformed → one `parse-rejected{parser:"ledger-stamps"}` WARN and `cli_verified:false`), and `viola verify`'s `update_stamps` reads the current bytes for its locked read-modify-write, both without the home strict-modes check, until "Home and code-bearing file integrity" (Epoch 6) adds both to the entry-point set.
- The Decisions Log entry records both widenings of this chunk (this gap and the capture arm), their conditions and witness (ci#36460408121 on `6486276`).
**Why:** no process runs the strict-modes check yet (Epoch 6 owns it), and a bad stamps file only degrades `run` to unverified. The founder ratified `run`'s read live at 2026-09-28 12:39:40 and its extension to `verify`'s read at 20:24:32, each after it was shown (relay: the Viola overseer). A non-`null` dialog decision still needs strict-modes on `run`'s stamps read first (PREREQ on "Dialog answers by dialog_id").
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/

## 2026-09-28-capability-ledger-and-viola-verify — the hidden `hook --capture` arm, verify's probe captures and the fixture scrub
**Section:** Input Validation rows Hook stdin (capture arm) and CLI arguments / stdin (hook sentence); Error Handling (hook bullet); Data Protection At rest (Probe captures) and Repository fixtures
**Change:**
- `hook <event> --capture <DIR>` (only `verify`'s probe plugin calls it): an absolute, existing `<DIR>`; stdin through `take(MAX_FRAME + 1)`, written raw and unparsed via `replace_private` (0600) to the first free `<DIR>/<PascalEvent>.<k>.json`, `k` over `1..=n+1`; no `VIOLA_*` read, no obs init, no channel; every failure writes nothing; exit 0 with empty stdout and stderr. The hook's `VIOLA_NAME`/`VIOLA_DIR` shape checks and one-object parse hold for its event path only.
- Probe captures are transient content-bearing 0600 files under the 0700 `ledger/probes/<pid>/`, removed whole by a drop guard on every exit path of `verify`.
- `verify --record` scrubs every string and key (home → `~`, both separator spellings, case-folded on Windows; the user word → `<user>`) and refuses the whole recording, nothing written, when a drive path, `/home/`, `/Users/`, `\Users\` or the user word survives.
**Why:** a boundary widening (a new `hook` crossing: a raw payload written to an argv-named directory), ratified live by the founder at 2026-09-28 20:24:32 after it was shown (relay: the Viola overseer). It avoids a new channel method or an env-var switch, runs as the same user through the pinned exe, and no other caller may register it.
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/

## 2026-09-28-capability-ledger-and-viola-verify — `MAX_FRAME` consumers extended, verify's CI split
**Section:** Input Validation Constants; Bootstrap phases (CI/CD, "run only locally")
**Change:**
- `MAX_FRAME` also bounds the `--version` reads of `run`'s gate and `viola verify` (stdout and stderr drained through the cap, killed at 5 s), the `hook --capture` stdin, and the `ledger/stamps.json` read (`take(MAX_FRAME + 1)`: over the cap reads as absent in `update_stamps`, an error in `read_stamps`).
- "The real `claude` CLI and `viola verify` run only locally" is now: the real CLI runs only locally; `verify` runs in CI only against the fake agent, its real-CLI probe and `--record` only locally.
**Why:** the chunk's new readers are capped as §Input Validation requires; only the list was stale. The CI line cited architecture's CI/CD note, amended in the same pass.
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/

## 2026-09-28-mutation-testing-to-the-epoch-boundary — the distro TMPDIR carve-out, the leg-verdict upload and the download-artifact pin retire
**Section:** Dependency Security (CI integration: the concurrency note, the SHA-pinned list, the event-payload line) · Bootstrap phases (`secret-scanning-ci-gate`) · Secret Management (Development: the pre-push WSL crossing; Secret scanning in CI) · Security Decisions Log (new `2026-09-28` entry)
**Change:**
- The pre-push WSL crossing carries exactly `HOME` and `PATH` (was: plus the Linux mutation leg's constant `TMPDIR=<distro home>/viola-pre-push-scratch`); a host-valued assignment stays a boundary widening.
- One CI upload stays unscanned, the nightly `fuzz/artifacts/` (was two: the mutants legs' `mutants-verdict-<os>.json` left with the CI mutation jobs); the scan's exact-path skip is `run --mutants`' `chunk.diff` (was "the mutation leg's"), the skip and the upload exclusion unchanged.
- SHA-pinned actions: checkout, rust-cache, install-action, upload-artifact (`actions/download-artifact` v8.0.1 removed, 5 → 4). The concurrency note reads that CI runs no mutation job; the event-payload line re-scopes the derived mutation base to `run --mutants`.
- A narrowing Decisions Log entry records both retirements; the 2026-09-24 and 2026-09-27 entries stay as history.
**Why:** the founder's 2026-09-28 17:59 ruling removed the CI and pre-push mutation legs, so the carve-out, the upload and the pin have no user. A narrowing, never the widening class.
**Kept:** the Threat Model Summary (its `download-artifact` and "mutation legs plus a union verdict" lines) — a verbatim copy of threat-assessment.md; the facts live in §Dependency Security.
**Ref:** .andromeda/runs/2026-09-28T21-04-49-wrap/

## 2026-09-29-sideloaded-conpty — vendored Microsoft ConPTY binaries and the third interim gap (founder live)
**Section:** Security Decisions Log (`2026-09-29`); Authentication & Authorization → `~/.viola/` access control
**Change:**
- New Log entry: `conpty.dll` + `OpenConsole.exe` from `Microsoft.Windows.Console.ConPTY` 1.24.260710001 (MIT) committed under `vendor/conpty/…`, embedded, written to `bin/<version>-<hash>/conpty/` and sideloaded; their own audit (`conpty-vendor.sh`), never a `cargo deny` exemption; no network call at build or run time; the System32 DLL-search restriction as the control for the planting vector.
- A third dated interim gap, beside the two of `2026-09-28`: until "Home and code-bearing file integrity" (Epoch 6) adds the `conpty/` folder and both files to the Windows strict-modes set, `run` loads them under the `FILE_SHARE_READ`-only held handle and the full SHA-256 re-hash alone, without the owner/DACL check.
**Why:** both are boundary widenings (a third-party prebuilt binary outside every lockfile audit hosts the child; code-bearing files loaded before the owner/DACL check), ratified by the founder live on 2026-09-29 at 10:41:12, relayed by the Viola overseer, after both forks were shown at the chunk's phase P4. The gap closes with the Epoch 6 entry (route CARRY).
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/

## 2026-09-29-sideloaded-conpty — the DLL search order and the companions' integrity controls
**Section:** Input Validation (new row: DLL search order + sideloaded ConPTY companions); Data Protection → Code-bearing artefacts; Security Anti-Patterns → Data Protection, Universal
**Change:**
- Input Validation row: every `viola` process restricts its DLL search to System32 as the second statement of `main`, so a bare-name load (portable-pty's included) never resolves from the CWD or `PATH`; `run` writes the companions write-if-absent, re-hashes each in full through a `FILE_SHARE_READ`-only handle and only then pre-loads `conpty.dll` by absolute path, the handles held until `spawn_child` returns; a failure degrades to the inbox ConPTY, recorded only as codes (`run.conpty_sideload`, `sideload_fallback`).
- Code-bearing artefacts: the list was the pinned exe, the plugin files, `settings.json` and `stamps.json`; it now also holds `bin/<version>-<hash>/conpty/{OpenConsole.exe,conpty.dll}` (write-if-absent, held-handle re-hash, a mismatch left as found).
- Anti-patterns: never load or launch a companion without that re-hash, never overwrite or delete a failing one; never let a Windows `viola` process reach a DLL load before the restriction, never pre-load by a relative path, and a sideload failure is never a refusal, exit change or terminal byte.
**Why:** the controls this chunk shipped, measured by its planted, tamper and two-sided restriction tests; the Threat Model Summary stays a verbatim copy (playbook "Verbatim upstream copy").
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/

## 2026-09-29-sideloaded-conpty — the vendored binaries' own audit, pins and CI step
**Section:** Dependency Security → Audit tool, Pinning, CI integration; Bootstrap phases → dep-audit-tooling-install, dep-security-ci-gate
**Change:**
- Audit tool: the vendored ConPTY binaries sit outside every cargo and npm graph; their gate is `scripts/conpty-vendor.sh` — `--verify` (fetch outside the tree, nupkg SHA-256 before extraction, byte compare, Authenticode signer; `conpty-vendor: verified <version>`) and `--probe` (four refusals and a control; `conpty-vendor probe: 4/4 refused, control clean`). The signer is checked there and in CI; at run time the SHA-256 pin carries it.
- Pinning: four exact pins (package version, nupkg SHA-256, two file SHA-256s) with one textual home, `src/conpty.rs`; committed binary; a version move is a re-vendor through the script.
- CI integration and both bootstrap phases: the `test` job's `windows-2025` step `ConPTY vendor verification` and the script join the lists.
**Why:** a new non-crate dependency needs its own audit like the npm graph and the fuzz lockfile before it (an own audit, never an exemption).
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/

## 2026-09-29-fake-agent-drift-contract — The cross-session tag filed as harness origin
**Section:** Security Anti-Patterns → Code Patterns (the compiled-prefix ban); Security Decisions Log (`2026-09-29`, the cross-session-message tag)
**Change:**
- Code Patterns: the ban on building harness prefixes from runtime or upstream text stands; it now names the set — four compiled literals matched on the raw start, no trim (`<agent-message from=`, `<task-notification>`, `<\cross-session-message`, `<cross-session-message`) — and the accepted side effect: the escaped cross-session form is the one escaped tag that classifies `harness`, so a human who types it at a prompt's start is filed `harness`.
- Decisions Log: a `2026-09-29` entry records the decision, the boundary widening and its side effect, the relayed status of the escaped injection form, the condition (the first live test measures it and owns the ledger row) and the witnesses.
**Why:** a boundary widening of the classifier's `harness` class, ratified live by the founder on 2026-09-29 (relay: the Viola overseer) after the widening and its side effect were shown.
**Ref:** .andromeda/runs/2026-09-29T14-38-17-wrap/

## 2026-09-29-t15-07-57-wrap — registry migration (U35): the security-plan Decisions Log leaves the body
**Section:** §Security Decisions Log · §Authentication & Authorization · §Data Protection · §Dependency Security (CI integration; after the omitted Supply chain integrity note) · §Security Anti-Patterns → Code Patterns, Universal
**Change:**
- The log moved verbatim to security-plan-amendments-archive.md: 14 entries, from the `2026-09-23` initial entry through the `2026-09-29` cross-session-message tag.
- Authentication & Authorization: the accepted risk that a driver LLM can be prompt-injected into `answer` `allow` (viola's part is only the closed `behavior` enum and the `unverified-cli` gate).
- Data Protection: the accepted local-availability risk — another local user can hold GUI connections or SSE streams open (no bound in v1), and unbounded `events.ndjson` growth can exhaust local disk (availability only).
- Dependency Security → CI integration: `scripts/release-check.sh` judges the build's own artifact records and refuses any executable other than `viola` and any `test-support` / `fake-agent` artifact (`--probe` 5/5). `test-support` is enabled only by the root package's `[dev-dependencies]`.
- Dependency Security → CI integration: G2's one exact-path exemption (`src/cmd/hook/seam.rs:<digits>`, whole-string compare), its look-alike `--probe` run before every check, and the rule that the perf arm's own check exempts nothing.
- Dependency Security: the v1.x release prerequisites (dist attestations and signing, cargo-auditable + `cargo audit bin`, self_update `signatures`, zizmor cache-poisoning, no `rust-cache`).
- Anti-Patterns → Code Patterns: `run --mutants` wipes only the guarded host mutation scratch (`scratch-refused` / `scratch-wipe-failed`, no fallback, the path never printed).
- Anti-Patterns → Universal: no non-`null` dialog decision while `run`'s stamps read skips strict-modes (the PREREQ on "Dialog answers by dialog_id").
**Why:** a Decisions Log is keyed by time, so it is history, not current truth. Its in-force items now stand in the body.
**Ref:** .andromeda/runs/2026-09-29T15-07-57-wrap/

## 2026-10-01-t12-19-55-wrap — arch amendments 1, 5 and amendment 3's URNs folded
**Section:** Bootstrap phases → auth-scaffolding-baseline
**Change:** The sequencing list now records that amendments 1 (v1 GUI cookie) and 5 (`ui/<port>.url`) are folded into architecture [GUI Control Scope] and §Occupied Resources (Filesystem), and amendment 3's two Problem Details URNs into architecture §Conventions (GUI HTTP errors), ahead of the `viola-ui` chunk; amendment 3's `control-character` detail lands with the route's "Confirmed send" entry.
**Why:** arch still said the GUI had "no token or CSRF" against this plan's ratified cookie; the overseer (founder-delegated) directed the fold at this wrap.
**Kept:** the Threat Model Summary's "There is no token and no CSRF protection" lines, a verbatim copy of threat-assessment.md (§Authentication & Authorization wins).
**Ref:** .andromeda/runs/2026-10-01T12-19-55-wrap/

## 2026-10-03-mutation-scoring-completion — the native pre-push crossing; the WSL launchers retired
**Section:** §Secret Management → Storage → Development (the pre-push bullet) · §Dependency Security → Pinning (the toolchain bullet; the gate-host bullet) · §Security Anti-Patterns → Code Patterns (the mutation-scratch bullet)
**Change:**
- Development: was every WSL call `wsl.exe -d Ubuntu [--cd D] --exec /usr/bin/env -i HOME=… PATH=…`, the operator aid `scripts/wsl-exec.sh`, and the operator-only uid-0 `wsl-provision.sh --install-deps`. Now `pre-push` runs only on a Linux host and launches every child natively as `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:<home>/.local/viola-node/bin:/usr/local/bin:/usr/bin:/bin`, with no other assignment. `<home>` is the passwd field 6, read by two probes carrying only `PATH=/usr/bin:/bin`; the harness's `$HOME` is never read for it. The canary pair reads 0 `CLAUDE*` names through the launcher and 1 through the control without `-i`. No other launcher and no uid-0 launch exists.
- Pinning: the WSL2 distro is no longer an install site. `pre-push`'s `tools` stage checks the native host against CI's own pins (`cc`, the channel, ci.yml's `test`-job `tool:` line, `NODE_PIN_VERSION`) and installs nothing; no provisioning script exists.
- Anti-Patterns: the scratch guard and wipe run only on a Windows host (`HOST_SCRATCH = cfg!(windows)`); the Linux host's same-named NOCOW `TMPDIR` is the operator's, never wiped by the harness.
**Why:** the WSL gate retired with the Windows dev host. No widening: what crosses `env -i` is still exactly HOME and PATH, and no host value crosses. HOME moved from the distro's `printenv HOME` to the host's passwd entry, never the harness's environment (the founder was shown it at plan review, no widening chosen). CARRY 4 (the root `--install-deps` must stop running user-writable code) is retired with `wsl-provision.sh`, not dropped.
**Ref:** .andromeda/runs/2026-10-04T01-02-04-wrap/

## 2026-10-04-windows-boundary-mutation-workflow — the third workflow under CI integration
**Section:** §Dependency Security → CI integration (the workflow list, a new `windows-mutants.yml` bullet, the concurrency note, the event-payload bullet)
**Change:**
- The CI-integration list gains `.github/workflows/windows-mutants.yml`.
- New bullet: `workflow_dispatch` only, no `inputs:` (no event-payload input class), at the epoch-boundary audit, never a gate or a `ci.yml` dependency. Workflow `permissions: {}`, job `contents: read`. Checkout and install-action are SHA-pinned with `persist-credentials: false` and ci.yml's tool pins; `matrix.*` reaches the step only via `env:`. No secret, cache, `needs:` or upload: the job log carries repo-relative documents only, and the unscanned-upload list is unchanged.
- Concurrency note: `concurrency-limits` was "(2 low findings)"; now 3 low, one per workflow (zizmor 1.30.1, dev host, 2026-10-04). "CI runs no mutation job" is now: no push or pull-request run carries one, and the workflow uploads nothing.
- Event-payload bullet: no workflow reads a `github.event` value; `ci.yml` runs no mutation job; the workflow's `--package` arm reads no base.
**Why:** founder ruling C2 (2026-10-04). Not a boundary widening: no permission, secret, input class, upload or action pin is added.
**Ref:** .andromeda/runs/2026-10-04T04-08-06-wrap/

## 2026-10-04-readiness-gate-and-timing-constants — the vt100 degrade as landed on the PTY output row
**Section:** §Input Validation (PTY output bytes (vt100) row)
**Change:** the row keeps its rule (feed vt100 through `catch_unwind`; passthrough continues) and now states the landed degrade: a tee on `run`'s pump output writes the human's bytes first and hands a copy to a feed thread, which runs every feed and resize under the catch; a caught panic poisons the screen model until the host size changes, with one `parse-rejected{parser:"vt100-feed", detail:"panicked"}` line per poisoning and no screen text; a poisoned model reads `input-not-ready`, which confirmed `send` reports as `not-delivered`/`input-not-ready`. vt100 0.16.2 panics are reachable at real small sizes (a 24×1 screen + a wide character; 1×1 `?u`; 1×2 `abc`); `viola_pty::host_size` never yields a zero size. Open: the tee → feed queue is an unbounded `std::sync::mpsc`, owed a bound ("bound every input") by confirmed `send`. Where-column: `src/run/gate.rs` tee + feed thread; the `screen` model.
**Why:** the readiness-gate chunk landed the feed; the panic sizes are measured; the overseer routed the queue's bound to confirmed `send`. Not a boundary widening: the feed reads the same PTY output bytes the row already governs.
**Ref:** .andromeda/runs/2026-10-04T05-25-03-wrap/

## 2026-10-04-confirmed-send-with-cl-1-records — send's input validation as landed; the feed bound closed
**Section:** §Input Validation (rows: Paste text · Channel frames · CLI arguments / stdin · PTY output bytes (vt100); Constants)
**Change:**
- Paste text: `validate_paste_text(&str)` was `-> Result<(), CoreError>`; now `-> Result<(), NotDelivered>`. It runs in `viola send` (exit 13 before any frame, a client `send-refused` line) and again first in the wrapper's `send`; the paste is one bracketed `write_all` through `viola-pty`'s `PasteHandle` (was "the `run` pump's paste writer").
- Channel frames: `send` params — `text` a string, `from` absent, `null` or a `ViolaName` — else `-32602` `"invalid params"`, `data: null`, before the paste check.
- CLI arguments / stdin and Constants: `viola send`'s stdin / `--file` text joins the `MAX_FRAME` consumers (`take(MAX_FRAME + 1)`; over the cap or non-UTF-8 → exit 2; a second positional is a usage error).
- PTY output bytes: was "Open: the tee → feed queue is an unbounded `std::sync::mpsc`, owed a bound"; now `sync_channel(FEED_CAPACITY)`, 256 messages (≤ 2 MiB), `try_send` after the human's write, a dropped copy poisoning the model (one `parse-rejected{vt100-feed, oversize}` per episode) until a size change; the model behind a mutex shared with the gate, the catch inside the lock.
**Why:** confirmed `send` is the bound's owner (the readiness-gate chunk's routing) and the paste surface's first consumer. Not a widening: every check the rows mandate is present.
**Ref:** .andromeda/runs/2026-10-04T06-44-39-wrap/

## 2026-10-04-confirmed-send-with-cl-1-records — the fourth dated gap: viola send before server verification (F3)
**Section:** §Authentication & Authorization (IPC client-side server verification; `~/.viola/` access control) · §Input Validation (CLI arguments / stdin; Own state files on read) · §Security Anti-Patterns → Authentication
**Change:**
- A fourth dated interim gap, until the Epoch 6 entries "Server verification before any frame" and "Home and code-bearing file integrity" (`:109` / `:111` remove it): CLI `viola send` writes its `send` frame after a liveness-only pre-check — the snapshot's pid + start time alive, the heartbeat live, an `endpoint` present, else exit 21 — and checks neither the serving process's identity nor the strict-modes of the snapshot it reads. The residual is a same-user process squatting a stale endpoint name; the pipe DACL and the 0700 socket directory still apply. It does not borrow the `hook.event` exception.
- The server-verification ban had one dated exception (`hook.event`); now two, and no other frame or process may borrow either.
- `send` is served by the wrapper (it answered `-32601` before this chunk), under the channel row's controls and this gap.
**Why:** a boundary widening, shown at P4 and held; ratified by the founder live on 2026-10-04, relayed by the overseer.
**Ref:** .andromeda/runs/2026-10-04T06-44-39-wrap/

## 2026-10-04-wait-and-last — the fifth dated gap: CLI wait / last after the liveness-only pre-check
**Section:** Authentication & Authorization → IPC client-side server verification · `~/.viola/` access control · Input Validation → Channel frames · CLI arguments / stdin · Own state files on read · Security Anti-Patterns → Authentication
**Change:**
- CLI `viola wait` / `viola last` write their `wait` / `last` frame after `send`'s liveness-only pre-check (snapshot pid + start time alive, heartbeat live, `endpoint` present, else exit 21 `during:"connect"`), with no server identity and no strict-modes, until the Epoch 6 entries `:109` / `:111`; they borrow neither the `hook.event` nor `send`'s exception; their snapshot reads join the home and own-state-file interim lists; the NEVER-write rule's dated exceptions now name three frames; served-as: `wait` / `last` from this chunk.
- Channel frames: the `wait` / `last` params clause (`after`/`timeout_ms` absent or `u64`, `from` the `send` rule, unknown fields ignored, else `-32602` `data: null`); the envelope types no method param.
- CLI arguments: `<name>` through `ViolaName::try_new`, `--after` / `--timeout-ms` clap `u64`.
- Own state files: the events reader `read_from` takes each line through `take(MAX_FRAME + 1)`, skips and counts over-long and non-object lines, never returns or heals an unterminated last line.
**Why:** the founder's ruling, live, 2026-10-04 (F1 at this chunk's P4, given after the widening was shown), relayed by the Viola overseer — the boundary-widening escalation resolves on it. The params and reader clauses record validation the chunk shipped; a mistyped `from` was `-32600` from the envelope until this chunk.
**Ref:** .andromeda/runs/2026-10-04T10-29-04-wrap/

## 2026-10-04-dialog-answers-by-dialog-id — the sixth dated gap: answer and hook.dialog frames, and their validation
**Section:** Authentication & Authorization → IPC client-side server verification · `~/.viola/` access control (strict-modes interim list) · Input Validation → Dialog free text · Channel frames · Hook stdin · CLI arguments / stdin · Own state files on read · Security Anti-Patterns → Authentication
**Change:**
- A sixth dated gap, until the Epoch 6 entries `:109` / `:111` land: CLI `viola answer` writes its `answer` frame after the liveness-only pre-check (`live_endpoint`, else exit 21), and `viola hook pre-tool-use|permission-request` writes its `hook.dialog` frame after only its payload shape check (`classify`); neither checks server identity or the snapshot's strict-modes. Residual: a process squatting the endpoint could answer a dialog, a PermissionRequest `allow` included. They borrow no earlier exception; the Anti-Patterns frame ban lists both, the strict-modes interim list, the CLI arguments row and the Own state files exceptions name `viola answer`'s snapshot read.
- Channel frames: was `send` / `wait` / `last` / `hook.event` params only; now also `hook.dialog` (closed `kind`, object `data`, closed `hook_event`, closed-or-absent `tool` / `input` / `continuation`; an invalid frame is not registered) and `answer` (`dialog_id` `u64`, `from` per the `send` rule, closed per-kind `response`, then `validate_paste_text`).
- Hook stdin: `HookEvent` was 7 events; now 9 with `PreToolUse` / `PermissionRequest`; on the dialog path `classify` maps into `DialogKind` / `DialogTool`, a malformed payload fails open (`DialogMalformed`), the reply read is bounded at `DIALOG_DEADLINE + 5 s`, one stdout write.
- Dialog free text: `validate_paste_text` runs first on both sides, `viola answer` before any frame (exit 13) and the wrapper's handler. `answer` arguments: `<name>` through `ViolaName::try_new`, `<dialog_id>` a clap `u64`.
**Why:** the chunk serves the two dialog methods, a boundary widening: the operator ratified at this wrap the founder's live ruling F1 (answered at P4, 2026-10-04, through the overseer, the residual shown), recorded as the founder's.
**Ref:** .andromeda/runs/2026-10-04T16-53-44-wrap/

## 2026-10-04-dialog-answers-by-dialog-id — run's strict stamps read; decisions on the six-row stamp until :82
**Section:** Authentication & Authorization → `~/.viola/` access control (the second interim gap) · Input Validation → CLI arguments / stdin · Own state files on read · Security Anti-Patterns → Universal
**Change:**
- The second interim gap was "`viola run`'s version gate and `viola verify`'s `update_stamps` read `ledger/stamps.json` without strict-modes"; now `verify`'s `update_stamps` alone. `run`'s gate reads through `viola_state::stamps::read_stamps_strict` (`strict::check_stamps` first): a strict-modes refusal, an unreadable or a malformed file is one `parse-rejected{parser:"ledger-stamps"}` WARN (detail `strict-modes-failed` on a refusal) and `cli_verified:false`.
- Universal: the interim ban "NEVER answer non-`null` while `run`'s gate still reads stamps without strict-modes … closes before Dialog answers by dialog_id answers any dialog" is now standing: NEVER answer non-`null` unless `run`'s gate read the stamps through the strict check.
- Universal stamp gate: a dated gap until `working-route.md:82` — a stamp holds only the six spine rows (`LedgerRow::ALL`, counter `/06`), no S3 / S7 / S8 / dialog-concurrency row, so a non-`null` decision flows on that six-row stamp whenever `cli_verified` is true; residual: an S3 / S7 / S8 body-shape change in a new CLI is not caught until `:82` lands the rows and their re-probe.
**Why:** the strict read is this chunk's PREREQ before any decision flows (a tightening). The six-row flow is a boundary widening: the operator ratified at this wrap the founder's live ruling R2 (2026-10-04, relayed by the overseer, the residual shown), `:82` the closer.
**Kept:** the R3 creation half (a home outside `%USERPROFILE%` gets the protected user + SYSTEM DACL at creation) landed exactly as `~/.viola/` access control already words it, so that row's body is unchanged.
**Ref:** .andromeda/runs/2026-10-04T16-53-44-wrap/

## 2026-10-04-the-wheel — the seventh dated gap: pause and release frames, and their validation
**Section:** Authentication & Authorization → IPC client-side server verification · Driver-originated `release` · `~/.viola/` access control (strict-modes interim list) · Input Validation → Channel frames · CLI arguments / stdin · Own state files on read · Security Anti-Patterns → Authentication · Bootstrap phases → auth-scaffolding-baseline
**Change:**
- A seventh dated gap, until the Epoch 6 entries `:109` / `:111` land: CLI `viola pause` and `viola release` write their `pause` / `release` frame after the liveness-only pre-check (`live_endpoint`, else exit 21); neither checks server identity or the snapshot's strict-modes. Residual: a process squatting the endpoint could swallow a `pause` or answer a `release`. They borrow none of the six earlier gaps; the frame ban, the strict-modes interim list, the CLI arguments row and the Own state files exceptions name them.
- Driver-originated `release`: was "`release` params are `{budget?}` only" and any `from` refused; now `{budget?, from?}`, a string `from` refused `-32602` "invalid params" `data: {"reason":"release-from-driver"}` (`ProtocolError::ReleaseFromDriver`) with one `release-from-driver` line (`from_trust:"self-reported"`), another type `-32602` `data: null`; CLI `viola release` forwards `VIOLA_NAME`, so a driver session's call exits 20.
- Channel frames: a `pause` / `release` params clause (`pause`'s `from` per the `send` rule; `release`'s `budget` absent or a bool). CLI arguments: `<name>` through `ViolaName::try_new`, `from` from `VIOLA_NAME` only.
- Bootstrap amendment 4 (`release-from-driver`): was "before the bin channel dispatch"; now landed.
**Why:** the chunk serves the two wheel methods, a boundary widening: the founder's live ruling F-W1 (answered at `:80`'s P4, 2026-10-04, through the overseer's AskUserQuestion, the pause-swallow residual shown), relayed by the overseer; the operator recorded it at this wrap as that existing ruling, no new decision.
**Ref:** .andromeda/runs/2026-10-04T20-44-01-wrap/

## 2026-10-05-real-cli-verify-probes — verify's typed-input PTY probe: the widening, its dirs, residual and refusal
**Section:** Threat Model Summary → Child process spawning · Input Validation → Child process output · Constants · Data Protection (Interactive probe dirs · Repository fixtures · two new notes) · Security Anti-Patterns → Universal
**Change:**
- Child process spawning: verify spawns two interactive `claude` PTY children after its print probe, both direct spawns under the R8 strip at 80×24 with output capped at `MAX_FRAME` into a `Screen` under `catch_unwind`. Run A is untrusted in a fresh 0700 OS-temp dir with empty input and is killed. Run B is trusted in `<cwd>/.viola-verify-<pid>/`, gets one compiled `PROBE_PROMPT` paste and ends by Ctrl-C ×2, then kill. Neither run sends a byte into a CLI-native dialog; a modal start is killed with no key. viola writes nothing under `~/.claude`.
- Child output row and Constants: `MAX_FRAME` also caps verify's PTY output, read only against the compiled literals.
- Interactive probe dirs: both outside the viola home, 0700, removed on every exit path.
- Repository fixtures: screens keep only signature rows. The refusal also covers an email-shaped token and a seam-split username. It was the fixed `a recorded payload still holds a path or a username`; now it is `a recorded fixture is not clean: <file> <code>` (`<file> row <n>[ seam] <code>` for a screen), with the closed codes `home-path` · `absolute-path` · `username` · `email`, never the content.
- New accepted risk: Run B leaves the CLI's synthetic-prompt transcript under `~/.claude/projects/` and runs the user's global hooks and status line (dev host: 5 transcripts, 13 → 18 dirs).
- New dev-host prerequisite: run from a trusted folder whose external import is answered (keyed on the git root). The overseer set the repo-root flags to "No" in one atomic `~/.claude.json` edit with a backup.
- Universal: the R2 dated gap was the six-row stamp (`/06`) until `:84`; now the ten-row stamp (`/10`) until "Dialog rows and re-probe".
**Why:** boundary widenings, each the founder's:
- the probe, no-key and dirs: his live rulings R-S2 (~00:00Z), "two runs, never accept" (~07:00Z, re-affirmed ~08:50Z) and the two dirs (08:25Z), all 2026-10-05, each answered after the widening was shown, relayed by the overseer and ratified by the operator at this wrap;
- the named refusal: his live ruling at this wrap, through the overseer's AskUserQuestion, with the closed code set and the no-content rule shown;
- the residual and the flag edit: accepted at this wrap on the overseer's word, relayed by the operator — the founder saw the class at his trust ruling, and the edit was his amended live ruling (~09:05Z).
**Ref:** .andromeda/runs/2026-10-05T10-37-44-wrap/

## 2026-10-05-dialog-rows-and-re-probe — the capture arm answers, Runs C and D, R2 closed
**Section:** Threat Model Summary → child spawning (`viola verify`) · §Input Validation (Hook stdin capture arm · CLI arguments · Constants) · §Data Protection (probe captures · interactive probe dirs · Accepted risk) · §Error Handling → Internal logging (`hook`) · §Security Anti-Patterns → Universal
**Change:**
- Capture arm: was "raw, unparsed, to the first free `k` … always exits 0 with empty stdout and stderr"; now it claims `<PascalEvent>.<k>.json` exclusively (`viola_state::fs::create_private_new`, the next `k` on `AlreadyExists`, then `replace_private` over its claim), and the hidden `--answers <DIR>` (inert without `--capture`) reads `<DIR>/<Event>.<j>` through `take(64)` into a closed `ProbeAnswer` id; `ledger::probe_body` shape-checks the payload and builds the body only through `dialog::decision_body`; one stdout write; nothing on any failure; stderr empty, exit 0. CLI arguments and Error Handling restate it.
- Threat Model and Constants: verify spawns four interactive children (was two); Run C (`-dialogs/`, an `ask` rule; its one `allow` runs `touch viola-probe-permission` in its own dir) and Run D (`-plan/`, `--permission-mode plan`, `plansDirectory` a 0700 `plans/` inside it); `MAX_FRAME` caps all four outputs. No byte typed into any dialog.
- Data Protection: four 0700 probe dirs; the `questions/` / `plan/` roots with `answers/`; no plan file under `~/.claude/plans` (measured, 17 → 17); the accepted transcripts residual covers Runs B, C, D (+2 per verify).
- Anti-Patterns: R2's dated gap retired — a stamp holds fourteen rows (`/14`, was ten / `/10`), so `10 pass  4 fail` records `cli_verified:false` and `answer` exits 12.
**Why:** boundary widenings — the founder's live rulings of 2026-10-05, each answered after the widening was shown: M7 = A (the hook answers, the one `touch`, the +2 transcripts) and STOP 7 (the Run D re-run, `plansDirectory`), relayed by the overseer; the operator ratified them as the founder's at this wrap, making no new decision. The free-`k` claim was measured racy.
**Ref:** .andromeda/runs/2026-10-05T14-27-34-wrap/
## 2026-10-06-local-command-and-paste-framing-rows — Run B's four compiled pastes and its wait; the seventeen-row stamp
**Section:** §Threat Model Summary → Child process spawning and PATH resolution (the `viola verify` bullet) · §Data Protection → Accepted risk (verify's trusted runs) · §Security Anti-Patterns → Universal (the stamp ban)
**Change:**
- Run B: was one bracketed paste of the compiled `PROBE_PROMPT`; now four bracketed pastes, all compiled literals (`PROBE_PROMPT`, `PROBE_LONG_PASTE`, `PROBE_TAG_PASTE`, `PROBE_LOCAL_COMMAND` = `/clear`, which opens a second CLI session inside Run B's process). Each added paste goes only into rows that hold the input-box literal and no modal literal; after the long and the tag-like turn Run B waits for those rows up to `PROBE_DEADLINE` (120 s) from that turn's Stop; a modal, a poisoned screen and the deadline each end the added pastes with no key.
- Accepted risk: Run B leaves two transcripts under `~/.claude/projects/` per real-CLI verify (was one), the same accepted class; measured 2 / 1 / 1 for Runs B / C / D.
- The stamp holds seventeen rows, counter `/17` (was fourteen, `/14`): the three framing rows join; a home stamped `13 pass  4 fail`, or a stamp of exactly the fourteen older ids all `pass`, records `cli_verified:false`.
**Why:** a boundary widening, ratified by the founder's own live answers, each given after the change was shown and relayed by the overseer: 2026-10-06T19:38Z for the three added pastes and the one more transcript (the card was labelled a boundary widening and named both), and 2026-10-06T20:49Z for the wait up to the 120 s probe deadline instead of 5 s with the guard kept (the red-round card; it was not labelled a widening). The operator confirmed this record at the wrap, 2026-10-06. No key is typed into any dialog and a modal start is still killed with no key.
**Kept:** "four interactive `claude` children" and the probe dirs stand unchanged; viola still writes nothing under `~/.claude`.
**Ref:** .andromeda/runs/2026-10-06T21-43-53-wrap/
## 2026-10-07-test-homes-off-the-contended-volume — test-side removals reach the dev host's tmpfs backing through the home link
**Section:** §Security Anti-Patterns → Code Patterns (a new bullet after the `run --mutants` scratch bullet)
**Change:**
- Added: NEVER let a test-side removal under `target/e2e-home` take anything but a dir the test side itself created: the root chain's owner sweep and a test home's drop (by `owner.json` record, by its own `TempDir`) and harness `cleanup` (by the `viola-session-` name).
- Added: on the Linux dev host that path is a link to `/tmp/viola-e2e-home-<uid>`, so these creations and removals reach outside the working directory. Was: the mutation scratch the one test-side site outside the workspace this section named; now the home link is the second.
- Added: the two keepers (`prepare_home_base`, `Workspace::ensure_e2e_home`) NEVER follow a linked base unless its target is absolute and one `lstat` reads a real directory with no group or other bit; a target that is itself a link is refused; a refusal is a fixed message naming no path. They create the target 0700 through the directory builder (never a chmod after, never a parent) and remove nothing. Ownership is not read separately: the `tempdir_in` that follows succeeds in an owner-only directory only for its owner.
- Added: `viola` never reads or writes the backing by that name; no CI runner has the link.
**Why:** a boundary widening (the harness creates and deletes outside the working directory through the link), ratified by the founder, 2026-10-07T07:25Z, live, after it was shown to him in those terms, relayed by the overseer; the overseer named that answer as the ratification of this amendment at the wrap. The backing takes a start's pinned copy off the volume other builders pace. Trap for later chunks: with the relative-target check neutralised a keeper resolves the target against the test's working directory and makes a directory there, so a mutation run of the harness keeper leaves `backing/` in its copied tree.
**Kept:** the `run --mutants` scratch bullet stands unchanged; no owner-equals-euid read was added, since the mode check and the following `tempdir_in` carry it.
**Ref:** .andromeda/runs/2026-10-07T08-21-13-wrap/
## 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host — two scratch measurements across the credential boundary, ratified by the founder
**Section:** Threat Model Summary (Data classification: the inherited-credential bullet) · §Secret Management → Storage (the R8 / identity-floor sentence)
**Change:**
- Threat Model Summary records two dev-host measurements that crossed this boundary once each: one fixed synthetic message sent once, through the CLI's own peer messaging, from the builder session into a scratch `claude` 2.1.287 probe session that was the one new idle peer; and a scratch hook's names-only list of its `CLAUDE*` environment, read against a literal copy of the floor. No value was read into any file; no product code, `viola verify` run or capture arm took part; neither answer covers a second message, a verify child or any other probe.
- §Secret Management: a Linux dev-host reading stands beside the floor's Windows origin and differs. The environment 2.1.287 hands a hook holds 12 names, both messaging variables among them, four outside the eleven (`CLAUDE_ENV_FILE`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PROJECT_DIR`) and three of the eleven absent (`CLAUDE_CODE_BRIDGE_SESSION_ID`, `CLAUDE_CODE_EXECPATH`, `CLAUDE_EFFORT`). The floor stays the eleven, the four not being identity names; the prefix rule already removes them whenever they are inherited outside the persistent set.
**Why:** both measurements are boundary widenings, ratified by the founder's own live answers of 2026-10-07T09:43Z, each given after its widening was shown to him and relayed by the overseer; the overseer confirmed the ratification with this wrap's direction. The floor's ruling is the founder's own (live, 2026-10-07T11:37Z, relayed by the overseer), given on a route card of this wrap. Standing rule: the answers are spent; another peer message or another names probe needs its own showing.
**Kept:** `IDENTITY_FLOOR`, the R8 rule and the capture arm's ruled shape (it reads no environment).
**Ref:** .andromeda/runs/2026-10-07T10-53-41-wrap/
## 2026-10-07-a-send-ending-in-a-newline-is-confirmed — trailing LF is removed after validation, the one removal beside "reject, don't strip", the founder's ruling
**Section:** §Input Validation (the Paste text row) · §Security Anti-Patterns → Input · Threat Model Summary (the CLI input vector: Entry point, Trust boundary)
**Change:**
- Paste text row: "Reject, don't strip" stands for refused characters: a text that holds one is refused whole, and none is ever removed with the rest typed. Its reason "stripping would break the delivery-confirmation exact match" is retired. Added: after `validate_paste_text` has passed on the text as received, the wrapper's `send` takes the typed text once (`viola_agent_claude::hook::typed_text`: every trailing LF and nothing else, not a CR, a TAB, a space or an inner newline), and it is the only text `send` uses: the local-command classification, the text the exact match compares, `text_bytes` and the paste. A text ending in a newline that holds a refused character is still refused `control-character` with nothing typed.
- Anti-Patterns → Input: the three bans stand. Added: the trailing-LF removal is not the banned stripping (LF is allowed, validation runs first, no refused character is removed); answer free text is typed as validated.
- Threat Model Summary: the bridge send writes the typed text; delivery confirmation compares `prompt-submitted` text with the typed text (was "the sent text").
**Why:** the founder's ruling, live, 2026-10-07T15:21Z, relayed by the overseer, given after every option and the `/clear`-plus-newline consequence were shown to him. The overseer's word at the wrap: this amendment is not held for him. Judged at the wrap as not a boundary widening: validation is unchanged and first, no new input class is admitted, and fewer bytes cross into the PTY. Standing rule: any other removal from a sent text, and any removal from answer free text, needs its own ruling.
**Kept:** `validate_paste_text`, its set and its place first in the wrapper's order; the client-side check; the one-write bracketed paste.
**Ref:** .andromeda/runs/2026-10-07T19-12-23-wrap/
## 2026-10-08-first-live-test-and-self-drive — a new attack-surface vector: the human's terminal on `viola run`'s stdin
**Section:** Threat Model Summary → Attack surface (a new vector after "CLI input")
**Change:** the vector names every byte the terminal writes to `viola run`'s stdin; its trust boundary is the stdin classifier's closed non-editing list (F-W2), widened once by seven terminal reply shapes after `CSI`, each by exact grammar (`0 n`, `? 997;1 n` and `? 997;2 n`, `4;n;n t`, `6;n;n t`, `8;n;n t`, `48;n;n;n;n t`, `> 4;n m`), with the nearest human keys and one-field or one-prefix neighbours still typing and no reply added unmeasured; the classifier reads bytes, not time. It also records that a key reaching this stdin is the human's whoever typed it: an agent typed five keys with `wtype` on a compositor the chunk started itself, nested on the locked desktop, one start, each key after a guard read, the desktop lock untouched and read locked after the start, before each live step and twice after the end. Was: no text on this surface.
**Why:** two boundary widenings, both ratified by the founder himself and relayed verbatim by the overseer: the seven shapes on 2026-10-08 after the exact grammars and a narrower option were shown; the agent-typed takeover key by his answers of 2026-10-08, the method first, then the compositor's form and price after they were shown. The summary keeps its verbatim-copy label and is kept current.
**Ref:** .andromeda/runs/2026-10-08T09-10-03-wrap/
## 2026-10-09-epoch-3-cleanup — trailing CR and LF removed; an empty typed text refused
**Section:** §Input Validation › Paste text row, CLI arguments / stdin row; §Threat Model Summary › CLI input vector (Entry point, Trust boundary); §Security Anti-Patterns › Input; §Error Handling › Channel
**Change:** the one removal of allowed characters is every trailing CR and LF, in any order, and nothing else (was: every trailing LF, "not a CR"). A listed local command followed by LF, CR or CRLF is that command. A text that ends in CR, LF or CRLF and holds a refused character is still `control-character` with nothing typed. A text whose typed text is empty, the empty text included, is refused `not-delivered` / `empty-text` (exit 13): by the client's `refusal_of` before any frame, after the refused-character read, and by the wrapper's `send` directly after the typed text is taken and before the first wheel read. The wire outcome and the channel's `result.refusal` name both details. "NEVER refuse LF, CR or TAB" stands, with the sentence that `empty-text` refuses a text with nothing to type and never a text with content.
**Why:** the founder's rulings of 2026-10-09 (the strip; the detail's name, condition and hint), each given live with the options shown and relayed; the rung is the overseer's. The control is present on both sides, so the row's description changed and no boundary widened.
**Kept:** `validate_paste_text` still returns only `control-character`; an inner CR or LF is typed as received.
**Ref:** .andromeda/runs/2026-10-09T17-10-00-wrap/
## 2026-10-09-epoch-3-cleanup — route entries cited by title
**Section:** §Authentication & Authorization › IPC client-side server verification row; §Input Validation › the `events.ndjson` reader row
**Change:** the second to fifth interim-gap sentences name the route entries "Server verification before any frame" and "Home and code-bearing file integrity" (was: "`:125` / `:127` remove it", four times; the entries stand elsewhere on the route). The reader row names the entry "Self-healing state" (was: `working-route.md:109`).
**Why:** the operator's direction that every stale bare route number found is cited by title; the citation sweep reads no bare number.
**Ref:** .andromeda/runs/2026-10-09T17-10-00-wrap/

## 2026-10-09-inner-cr-and-crlf-in-a-sent-text — an inner CR or CR LF is typed as one LF after validation, the founder's ruling
**Section:** §Input Validation (Paste text row) · §Threat Model Summary (Attack surface: CLI input, Entry point and Trust boundary) · §Security Anti-Patterns § Input
**Change:** after `validate_paste_text` has passed on the text as received, the typed text is the text with every CR LF pair as one LF, every other CR as one LF, and every trailing CR and LF removed; it holds no CR (was: "Trailing CR and LF, the one removal", every trailing CR and LF "and nothing else"). The row argues the class anew: only CR and LF, both allowed characters, are rewritten or removed; the set of accepted texts is unchanged; the typed text of a valid text is valid and holds no character that is neither in the received text nor LF, pinned by a 512-case property; no refused character is typed or removed; a raw channel client gets the same rule from the wrapper; the delivery-confirmation match stays an exact equality with no tolerance on the submitted side. The Threat Model's Entry point and Trust boundary carry the same definition. The anti-pattern's carve-out names the inner rewrite beside the trailing removal as "not this stripping".
**Why:** the founder ruled, live in the overseer's dialog on 2026-10-09T16:51Z with the options shown, relayed by the operator, that `send` types a CR or a CR LF inside a text as the LF the CLI submits. The detector's `escalate` severity was resolved on that ruling with no halt: no boundary widens, since no new input class is admitted and validation is unchanged and first. A later change that types a character the received text did not hold, other than this LF, would be a new class and needs its own ruling.
**Kept:** reject, don't strip; answer free text is typed as validated (`answer` does not call `typed_text`); the `empty-text` refusal and its two sides; the two earlier rulings on the ending.
**Ref:** .andromeda/runs/2026-10-09T20-50-14-wrap/

## 2026-10-10-windows-mutation-grade — the Windows mutation workflow's nine-item matrix, the jobs sentence without a mutation job, one dispatch outside the audit
**Section:** §Dependency Security → CI integration (the `windows-mutants.yml` bullet and its Permissions sub-bullet) · §Threat Model Summary → Infrastructure → CI/CD (the Jobs sentence)
**Change:**
- CI integration, Permissions: the workflow's one job definition runs on `windows-2025` over a nine-item matrix, counted as the `- package:` items under `matrix.include`; the root `viola` package is four of them, split by file; the job name is `mutants (<label>)` (was "its one job (`windows-2025`, six-package matrix)"). `permissions: {}` and `contents: read` are unchanged.
- CI integration: after the sentence that states founder ruling C2, one sentence names the one dispatch made outside the epoch-boundary audit: 2026-10-10, for this chunk alone, on the founder's own word, run 38036448183.
- Threat Model Summary, Jobs: "mutation (ubuntu and windows legs plus a union verdict)" is retired. The sentence names the per-OS `perf` job (`agent-run run --perf`) in its place and says that no job of `ci.yml` runs mutation since 2026-09-28, when mutation testing moved to the epoch-boundary code audit.
**Why:** the chunk split the `viola` matrix item per file, so the six-package count was false. The Jobs sentence had listed a mutation job that left `ci.yml` on 2026-09-28 and omitted `perf`; the route entry carried it to this chunk, and the plan named the change. The dispatch was allowed by the founder for this chunk only, relayed verbatim by the operator on 2026-10-10.
**Kept:** the ruling's own sentence (dispatch-only, no `inputs:`, at the epoch-boundary audit, never a gate, never a dependency of `ci.yml`) stands word for word; the exception is a sentence of its own. The Threat Model Summary keeps its verbatim-copy label. The two new dependencies of the test-only `viola-e2e` (syn, proc-macro2) pass `cargo deny check` with no new ignore and need no edit of the ban list.
**Ref:** .andromeda/runs/2026-10-10T08-56-51-wrap/

## 2026-10-10-self-healing-state — own state files on read: three counts, the appender's heal, the replay as library code
**Section:** §Input Validation (row "Own state files on read") · Threat Model Summary (Filesystem state, Trust boundary)
**Change:**
- "Own state files on read", as landed: the reader counts per read `Skipped{unknown_kinds, unknown_fields, torn_lines}`, not logged: an over-long line, a non-object line and an unterminated last line are `torn_lines`; an unknown kind is `unknown_kinds` and is not returned; a known-kind line with a key outside its contract is returned and counted once. Was `Skipped{oversize, malformed}` with the last line neither returned nor healed and healing owed to "Self-healing state".
- The same row: a reader never writes. The next append heals a torn last line under the lock every append holds, after a one-byte read of the log's end, by one LF ahead of its line in the same single write, and logs one `state-recovered` line holding a detail code, a basename and an offset.
- The same row: `snapshot::read_snapshot_classified` and `viola_state::replay` read through the same bounds and write no file; library code with no caller, so no product reader's handling of an absent, unreadable or newer snapshot changed.
- Threat Model Summary: readers skip and count torn lines and the next append heals; the replay for a snapshot that fails to parse or carries a newer `v` stands as library code that no product reader takes yet (was "Readers tolerate torn lines and replay the log when a snapshot fails to parse").
**Why:** the chunk landed the healing and its count. No boundary widened: the line cap, the `MAX_FRAME` reads and the Integrity part are unchanged, and nothing under `src/` moved.
**Kept:** the row's Line cap sentence and its Integrity part, with every dated exception, stand as written.
**Ref:** .andromeda/runs/2026-10-10T11-03-09-wrap/

## 2026-10-10-viola-revive — the revived child: the recorded cwd ratified, the session id on its argv recorded and not ratified
**Section:** §Threat Model Summary (Data classification, operational metadata; Attack surface, Child process spawning entry points) · §Input Validation (Child executable resolution row; a new row, Session id as a child argv value)
**Change:**
- Child executable resolution row, now titled for `run`, `revive` and `claude agents`: `run` sets the child's cwd to the spawner's current directory and writes it into the first snapshot's `cwd` when valid UTF-8. `revive` launches the fixed program `claude` by name on its own PATH and sets the child's cwd to the snapshot's recorded `cwd`: read only after the strict-modes check, used as the spawn directory alone, an existing directory or `cwd-missing` (was "the spawner's current directory", one source). Program resolution and the version gate never read it.
- New row: the id after `--resume` is the newest logged `agent_session_id` of the closed session-id shape (36 characters, hyphens at offsets 8, 13, 18, 23, ASCII hex elsewhere) or an `--id` the log holds; otherwise `no-session` before any spawn; a malformed `--id` is exit 2; one argv element of a direct spawn, built only by `viola_agent_claude::resume_args`, after the wrapper's `--plugin-dir` and before the words after `--`; `--list` prints only ids of that shape. The row says **Not ratified**.
- Threat Model: `revive` is a second spawner of the wrapped child; snapshots hold the optional `cwd`, a host path that can carry the OS username, in the 0600 snapshot only.
**Why:** the recorded directory as a spawn directory is a boundary widening, ratified by the founder live on 2026-10-10 after it was shown to him as one, the overseer and the operator as relay. The logged id on the child's command line is a second crossing that was not shown to him as one: on the operator's answer at this wrap's Phase 2 halt it lands as fact with his word owed, and no ratification is written. The next wrap records his word as his.
**Ref:** .andromeda/runs/2026-10-10T15-07-22-wrap/

## 2026-10-10-viola-revive — revive reads its own state: the instance check as an entry point, the replay's one caller, its arguments
**Section:** §Threat Model Summary (Attack surface, Filesystem state trust boundary) · §Authentication & Authorization (`~/.viola/` access control row, Strict-modes check) · §Input Validation (CLI arguments / stdin row; Own state files on read row)
**Change:**
- Strict-modes check: `revive`, both arms, joins the entry points through `viola_state::strict::check_instance` (the home, `instances/`, the instance directory, its `snapshot.json` and `events.ndjson`, each that exists, in that order), before any value of either file is used; a refusal is `strict-modes-failed`: exit 1, one `unable:` / `hint:` pair, no child, and on the start arm one `process-exit` line. It is none of the dated interim gaps. Proven by `cfg(unix)` cases; no Windows case widens an instance tree's DACL, owed to the route entry "Home and code-bearing file integrity".
- Own state files on read: `events::read_from` also serves `replay::session_chain`. The classified read and the replay have one product caller, revive's preflight, taken only after `check_instance` passed (was "library code with no caller yet", per "2026-10-10-self-healing-state — own state files on read: three counts, the appender's heal, the replay as library code"); the snapshot is written by the revived wrapper alone; the four other snapshot readers still call `read_snapshot`, so their handling of an absent, unreadable or newer snapshot is unchanged. The Threat Model's filesystem bullet says the same.
- CLI arguments row: revive's arguments, `<name>` by the shared name parser, `--id` a clap parser of the closed session-id shape (else exit 2), `--list` conflicting with `--id`, `--fork` and the trailing words, the trailing words as argv elements of a direct spawn, no `--json`. Revive is not among the row's interim gaps.
**Why:** the chunk landed a reader of instance state that checks it first. The four older readers' behaviour stays owed to the entries the previous wrap's CARRYs name.
**Ref:** .andromeda/runs/2026-10-10T15-07-22-wrap/

## 2026-10-10-statusline-pass-through — the statusline source and the shell-out: two Input Validation rows, the trusted-file list
**Section:** §Input Validation (new rows: Statusline source; Statusline shell-out; Own state files on read; Constants) · §Authentication & Authorization (`~/.viola/` access control: the trusted-file list) · §Threat Model Summary (Data classification, config)
**Change:**
- New row, Statusline source: at every start `run` reads one source through `take(MAX_FRAME)`: `<home>/statusline-source.json` whenever it exists, else for the default home the user's `~/.claude/settings.json`; a `--home` home reads no user settings. `statusline::user_command` takes a command only from a `statusLine` of type `command`, non-empty, with no NUL. No log line and no span. The command stands in `snapshot.json` only. viola writes neither source.
- New row, Statusline shell-out: only `hook statusline` runs the command, on Unix, as one argument of `/bin/sh -c`; the instance check first; environment and directory unchanged; stderr discarded; stdout through `take(MAX_FRAME)`, printed and never parsed; bounded by `STATUSLINE_DEADLINE` (5 s, PROVISIONAL; detail `deadline`). Not controlled: what the command does. It remains the only shell-out.
- The two sources are named as not on the trusted-file list: one sits in the checked home with no owner or mode check of its own, the other lies outside the viola home under no viola check. The Own state files row says write access to the source in use also decides the command.
- `MAX_FRAME`'s reader list gains the source read, the arm's stdin and the command's stdout. The Threat Model's config list names the two sources and says the override never holds the user's command.
**Why:** both are boundary widenings the founder ratified live on 2026-10-10 after each was shown to him with its options priced, choosing the file in the viola home for the source (relayed verbatim by the operator); the operator confirmed the one card at this wrap, 2026-10-10. The shell is relied on with no ledger row on the founder's word the same day; the row is owed to the working-route entry "Paste newline ledger row", the Windows shell-out to "Windows-only live measurements".
**Ref:** .andromeda/runs/2026-10-10T19-55-42-wrap/

## 2026-10-10-statusline-pass-through — the settings override as landed, the hook rows, the arm's check before its log
**Section:** §Data Protection (At rest, Code-bearing artefacts) · §Input Validation (Hook stdin row; CLI arguments / stdin row) · §Authentication & Authorization (`~/.viola/` access control: the entry-point list) · §Bootstrap phases (`auth-scaffolding-baseline`; `logging-redaction-wire`)
**Change:**
- Code-bearing artefacts: `instances/<name>/settings.json` is `{"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}`, rewritten whole at every start through `replace_private` at 0600, after the first snapshot and before the spawn. Was "rewritten at every start" with no condition; now Unix only, and only for a pinned path of ASCII letters, digits and `_ - . / :`, since a shell runs the string unquoted. The child gets `--settings` after `--plugin-dir`, only when an override was written. The flag is `statusline::SETTINGS_FLAG`; `src/cmd/verify/typed.rs` holds the literal at two sites from before it, owed to the working-route entry "Interrupted verify cleanup". The `auth-scaffolding-baseline` bullet carries the same two conditions.
- Hook stdin row: the word `statusline` is taken before the `HookEvent` mapping. The statusline payload: the same cap and one-object rule; no `rate_limits` object is no write; an absent window is `"unknown"`; `used_percentage` kept from 0 to 100; `resets_at` is epoch seconds or RFC 3339 (was "parsed with chrono in UTC only"), written RFC 3339 UTC with milliseconds; `budget.json` written only by this arm.
- Entry-point list: `hook statusline` runs `check_instance` on its canonicalised instance directory before its log opens; a refusal is `strict-modes-failed` with no command run, nothing printed, `budget.json` unchanged, exit 0. It is none of the dated interim gaps, and the CLI arguments row scopes the hook gap to the nine `HookEvent` arms.
- `logging-redaction-wire`: the user's command stands in `snapshot.json` only (was "and `instances/<name>/settings.json`").
**Why:** the override is a boundary widening the founder ratified live on 2026-10-10 (relayed verbatim by the operator; the operator confirmed the card at this wrap, 2026-10-10), with the Windows half cut on his word and owed to "Windows-only live measurements". The check runs first because obs init sets an existing home to 0700, and the chunk measured that a check made after it passes a home at 0770.
**Ref:** .andromeda/runs/2026-10-10T19-55-42-wrap/

## 2026-10-10-statusline-pass-through — revive: the session id ratified, its arguments, its log before its check, the fifth snapshot reader
**Section:** §Input Validation (Session id as a child argv value; Own state files on read) · §Threat Model Summary (Attack surface: Child process spawning and PATH resolution) · §Authentication & Authorization (`~/.viola/` access control: the entry-point list)
**Change:**
- Session id row: "**Not ratified** … his word is owed, and no ratification is recorded here" is now "**Ratified**": the logged session id as the value of `--resume` on a revived child's command line is a crossing the founder ratified (was recorded and not ratified, per "2026-10-10-viola-revive — the revived child: the recorded cwd ratified, the session id on its argv recorded and not ratified"; that entry's cwd half stands). The Threat Model's revive bullet says the same (was "the founder's word on it as a crossing is owed").
- The revive bullet's child arguments: `--plugin-dir <dir>`, then `--settings <instance dir>/settings.json` when the start wrote an override, then `--resume <id>`, `--fork-session`, the words after `--` (was no `--settings`).
- Entry-point list, `revive`: its start arm opens the wrapper log before the instance check, and opening a log sets an existing home to 0700, so there the check cannot refuse a home for a group- or other-writable mode. Written as read in source, not measured on `revive`.
- Own state files row: five other snapshot readers still call `read_snapshot` (was four): `src/cmd/hook/statusline.rs` joins. The rule: `read_snapshot(` under `src/`, outside test modules.
**Why:** the founder ratified the session id live on 2026-10-10, shown to him as a crossing with two options (ratify it, or allow only an id the human types), relayed verbatim by the overseer; the operator confirmed the card at this wrap, 2026-10-10. `revive` starts through `run`'s `start`, which writes the override, so its child takes the flag too. The log-before-check order is owed to the working-route entry "Budget governor" (the operator's direction at this wrap).
**Ref:** .andromeda/runs/2026-10-10T19-55-42-wrap/

## 2026-10-10-budget-governor — budget.json read by the gate, not ratified
**Section:** §Authentication & Authorization (`~/.viola/` access control, beside the trusted-file list; Driver-originated `release`) · §Input Validation (Configuration values; Own state files on read)
**Change:** `budget.json` is not on the trusted-file list, and the wrapper's budget gate reads it and refuses a driver's `send` as `budget-paused` on what it holds. The read (`viola_state::budget::read_budget`, the file's one reader) runs `check_path` on the home and on the file, takes a shared lock on `budget.json.lock`, reads through `take(MAX_FRAME + 1)`, parses tolerantly and requires `v` = 1; every failure is no reading with no line, on which the gate does not pause; it never writes, and an absent file creates nothing. This is written as a boundary widening that is NOT ratified. Configuration values: the thresholds are `five_hour` and `seven_day` (defaults 90 and 85), each a JSON whole number that fits `Percent`, else that key's default and one `parse-rejected{parser:"config-json", detail:"malformed"}` per bad key; a document that is no object or whose `v` is not 1 gives both defaults; no value switches the gate off; the read lives in the root bin (`src/run/budget.rs`); was "Thresholds go into `Percent` (0–100)" under `viola-state`. Driver-originated `release`: the string-`from` refusal comes before `budget` is acted on, so it lifts no pause.
**Why:** a file a hook arm writes now decides a wrapper's refusal, which widens what crosses the home's trust boundary. It landed as built on the operator's word, which is no ratification under the playbook's "what ratifies it" rule. The operator directed at the wrap that it be written as not ratified with the founder's word owed, and that no card be confirmed.
**Kept:** the trusted-file list itself is unchanged. Adding `budget.json` to it, or dropping the refusal, is the founder's ruling to make.
**Ref:** .andromeda/runs/2026-10-10T22-36-31-wrap/

## 2026-10-10-budget-governor — the two instance checks ahead of any log
**Section:** §Authentication & Authorization (`~/.viola/` access control, the strict-modes entry points: `revive`, `hook statusline`)
**Change:** `revive`'s check runs before the wrapper log opens. A refusal is exit 1 with the state-files pair and no child, and on either arm no log is opened and no line written; was "on the start arm one `process-exit` line in `run-<name>.ndjson`". The sentence that its start arm "opens the wrapper log before the check (read in `src/cmd/revive.rs` …, not measured on `revive`)", owed to the entry "Budget governor", is retired: the order was measured and fixed. `hook statusline` runs its check before anything under the home is read or opened; a refused instance returns exit 0 at once and leaves no trace, however often the arm is invoked; was "a refusal is `strict-modes-failed`: no command run, nothing printed, `budget.json` unchanged, exit 0". Its earlier one-invocation refusal is recorded as measured.
**Why:** a log opened before or after a refused check narrowed the home to 0700 and so erased the state the check refuses. That a refused check writes no line is the operator's answer at planning; the founder has not ruled on it.
**Ref:** .andromeda/runs/2026-10-10T22-36-31-wrap/

## 2026-10-10-session-links — the link-set restore reads the log
**Section:** §Input Validation (row "Own state files on read")
**Change:** The list of passes of the one `events.ndjson` reader (`viola_state::events::read_from`) gains the wrapper's link-set restore at its start: `LinkSet::restore` in `src/run/links.rs`, one pass from offset 0, beside the wait feed's rebuild and before the endpoint serves, so a start reads the log twice (`WaitFeed::rebuild`, then `LinkSet::restore`). The restore appends nothing and logs nothing. `link` and `unlink` are kinds this build knows: the reader returns such a line and no longer counts it under `unknown_kinds`. The link rule `viola_state::replay::apply_link_line` uses a line's `data.driver` and `data.driven` only when both are strings and does not parse them as names again (the wrapper wrote them: the driver from a `from` that passed `ViolaName::try_new`, the driven its own name); a line of another kind or with a name that is no string changes nothing. The one-product-caller sentence is of the replay, whose result now holds the logged link set (`Replayed.links`), so a revive over a replayed snapshot reads the logged links; the rule alone has a second product caller, the wrapper's `LinkSet`.
**Why:** The chunk added a second pass of the existing reader over the wrapper's own log, at the point of the start where the first already ran, and a rule over its lines. No refusal and no permission rests on `from` or on the link set. The row claims no order between the restore and the strict-modes check, because the chunk's report states none.
**Kept:** The sentences on the `link` / `unlink` methods, the dated gaps and the anti-patterns are unchanged. The explicit verbs are not built, and the boundary widening they need waits for the founder's own word.
**Ref:** .andromeda/runs/2026-10-11T00-33-08-wrap/

## 2026-10-11-the-board-viola-list — viola list joins the strict-modes entry points; the replay and budget.json gain a reader
**Section:** §Authentication & Authorization (`~/.viola/` access control); §Input Validation (CLI arguments / stdin, Configuration values, Own state files on read, Child executable resolution); Threat Model Summary (filesystem and child-spawn vectors); §Bootstrap phases → logging-redaction-wire
**Change:**
- `list` is a strict-modes entry point: `board::scan` runs `check_path` on the home and `instances/` (absent passes, unstattable refused, a link never followed) and `check_instance` per instance in name order, before any file under the home is opened and before its log; the first failure refuses the whole board with exit 21 (`--json` `{"v":1,"error":"instance-unreachable","detail":null}`, stderr empty; human: the home-state or instance-state pair, stdout empty), no log, no line, the home's mode as found. None of the dated interim gaps; `cfg(unix)` cases only.
- `list` takes `[--json]` only; it makes the home absolute with `std::path::absolute`, not canonicalised, as every `cli` verb resolves `--home` today; the canonicalisation stays owed to "CLI machine contract".
- The classified read and the replay have two product callers (was one, `viola revive`): revive's preflight and `board::read`, each after `check_instance`. `events::read_from` gains `board::log_pass`; `list`'s `skipped` is the first product reading of the counts.
- `budget.json` has one checked read, `read_budget_stamped`, with two product paths (was one reader, the gate): the gate's `read_budget` and `list`, which shows the reading and decides nothing on it. The thresholds gain `list` as a reader through `read_thresholds`.
- CLI `list` as landed resolves no `claude` and reads no `claude agents --json` (was: `list`, `mcp` and `ui` spawn it); it joins that rule with "Unwrapped sessions on the board".
- The logging floor: `wait` / `last` keep `\n` and `\t`; `list` rows escape every control character, `\n`, `\t`, DEL and C1 included, as `\xHH` (`escape_row`); as landed every row field comes from viola's own state.
**Why:** the board as built. The whole-board refusal and its no-line shape are the operator's answer at that chunk's planning, not ruled by the founder. The board's read of `budget.json` is no boundary widening: the same checked read, nothing decided on it; the gate's refusal stays the unratified widening.
**Ref:** .andromeda/runs/2026-10-11T08-23-35-wrap/

## 2026-10-13-t06-38-35-wrap — budget.json ratified; the board's refusal and the statusline arm's no-trace shape his
**Section:** §Authentication & Authorization (`~/.viola/` access control: the entry-point list, the `budget.json` sentence) · §Input Validation (Own state files on read)
**Change:**
- `budget.json`: a file a hook arm writes, outside the trusted-file list, deciding a wrapper's refusal is a boundary widening the founder ratified as built: read behind the strict-modes check on the home and the file, failing open. Was "NOT ratified … the founder's word is owed" (access control) and "the gate refusing a driver on it is not ratified" (Own state files on read), per "2026-10-10-budget-governor — budget.json read by the gate, not ratified", whose read, threshold and `release` claims stand.
- Entry-point list, `list`: the whole-board refusal and its no-line shape are his; was the operator's answer, not ruled.
- Entry-point list, `hook statusline`: a refused check leaving no trace is his; was the operator's answer, not ruled.
- Entry-point list, `revive`: unchanged — a refused check writing no line stays the operator's answer at the budget-governor planning, not ruled.
The superseded marks stand in the body as the operator's word, superseded 2026-10-13.
**Why:** the founder's own answers of 2026-10-13 («Ратифицировать» for the widening; «Подтвердить всё» for the board's shapes and the budget governor's item 3 as shown to him), relayed verbatim by the Viola overseer through the operator's directive. The `revive` arm was not in the question shown to him (the operator's word at this wrap); it goes to his next batch. The trusted-file list itself is unchanged: `budget.json` is still not on it.
**Ref:** .andromeda/runs/2026-10-13T06-38-35-wrap/

## 2026-10-13-t06-38-35-wrap — the statusline pass-through card: the operator's confirm recorded as provisional, his batch word supersedes it
**Section:** none in the body — the record of three entries' `**Why:**` lines: "2026-10-10-statusline-pass-through — the statusline source and the shell-out: two Input Validation rows, the trusted-file list" · "2026-10-10-statusline-pass-through — the settings override as landed, the hook rows, the arm's check before its log" · "2026-10-10-statusline-pass-through — revive: the session id ratified, its arguments, its log before its check, the fifth snapshot reader"
**Change:** the body is unchanged: it already records W1 (the settings override), W2 (the `/bin/sh` shell-out), W3 (the statusline source) and the revive session id on `--resume` as widenings the founder ratified live on 2026-10-10. Corrected is what the three entries say of the card: each wrote "the operator confirmed the (one) card at this wrap, 2026-10-10" beside his ratification, as though the confirm settled it. That confirm was not his own word: it was PROVISIONAL (`delegate viola-overseer`), and it is superseded by his own confirm of the same card, W1, W2, W3 and the session id, in the epoch-boundary batch of 2026-10-13 («Подтвердить всё»). The three entries stand otherwise; append-only, they are not edited.
**Why:** the letter's answer card: a confirm that is not the founder's own word is provisional, written `delegate {who}` or `relayed by {who}`, and superseded by his own word in the batch at the epoch boundary. His batch answer was relayed verbatim by the Viola overseer through the operator's directive (§2, §4.1 (d)). The directive named two of the three sites; the third (the source and shell-out entry) records the same card's confirm and is corrected with them.
**Ref:** .andromeda/runs/2026-10-13T06-38-35-wrap/

## 2026-10-13-epoch-4-cleanup — the wiped mutation scratch re-created owner-only
**Section:** §Security Anti-Patterns › Code Patterns (the `run --mutants` row)
**Change:** the wiped scratch is re-created owner-only through `viola_state::fs::create_private_dir` (the protected user + SYSTEM DACL on Windows), so an instance tree a test lays inside it passes the strict-modes check as one under `%TEMP%` does; NEVER re-create it with a plain `create_dir_all`, and NEVER loosen the check for it. The guard, no-fallback and no-path rules are unchanged.
**Why:** run 38215104120's root-package baseline failed the instance check inside a plainly re-created scratch; the fix is in the harness's lay-down, a narrowing, and `check_instance` is unchanged.
**Ref:** .andromeda/runs/2026-10-13T10-47-05-wrap/

## 2026-10-13-interrupted-verify-cleanup — verify's dirs removed on a Ctrl-C / TERM stop, the `--settings` move done, the matrix count
**Section:** §Data Protection (Interactive probe dirs · Probe captures · Code-bearing artefacts) · §Threat Model Summary, Child process spawning (Run B's ending) · §Dependency Security, CI integration (`windows-mutants.yml`)
**Change:**
- The probe dir and the interactive probe dirs were "removed on every exit path". Now they are removed when `verify` passes, fails, or is stopped by Ctrl-C or TERM: Unix `SIGINT` / `SIGTERM`, Windows `CTRL_C_EVENT` / `CTRL_BREAK_EVENT`, through `cmd::verify::stop`, installed by `viola verify` alone; its handler only sets one flag every wait reads. A stopped run kills and reaps its child with no key, and its guards remove only what that run created, never by a pattern or a listing. No owner record and no remover of an earlier run's or another process's dirs. SIGKILL, a crash, a failed handler install (ignored, fail-open) and a Windows close, logoff or shutdown leave the dirs by hand.
- A stopped Run B is ended by a kill with no key, pastes nothing more, and starts no later run.
- Was: `typed.rs` held the `--settings` literal, owed to "Interrupted verify cleanup". Now Runs C and D read `statusline::SETTINGS_FLAG`.
- `windows-mutants.yml` was a nine-item matrix with the root package four; now ten items with the root package five (`viola-cmd-verify-stop` over `src/cmd/verify/stop.rs`).
**Why:** The scope is the founder's narrow entry (relayed by the overseer, 2026-10-10); the event set is the operator's P4 answer. The matrix line is forced by the repository's contract test that every `cfg(windows)` product file is scored on Windows, and it was taken on the operator's word at implement; nothing was dispatched. The stop adds no IPC, file or env crossing: it carries no data, and its decision is closed (two events, two signals).
**Kept:** the same-pid pre-clean in `ProbeDir::create` / `TrustedDir::create` stands, predating the narrow ruling.
**Ref:** .andromeda/runs/2026-10-13T20-00-28-wrap/

## 2026-10-13-paste-newline-ledger-row — Run B's fifth compiled paste, eighteen stamp rows, the shell's row owed on "Owed ledger rows"
**Section:** §Threat Model Summary (Child process spawning, `viola verify`'s interactive children) · §Security Anti-Patterns → Universal (the stamp) · §Input Validation (Statusline shell-out row)
**Change:**
- Run B: was four bracketed pastes; now five compiled literals. `PROBE_NEWLINE_PASTE` (113 bytes, ASCII, one inner LF, no CR) goes before `/clear`. Was three added pastes; now four, each only into rows holding the input-box literal and no modal literal, none after the stop. After the long, tag-like and newline turns Run B waits for those rows. No start, spawn pair or transcript is added.
- The stamp: was seventeen rows, `/17`; now eighteen (`LedgerRow::ALL`, counter `/18`), the newline row `paste-newline` after the three framing rows. The dialog-less home reads `14 pass  4 fail`. A stamp holding exactly the fourteen, or exactly the seventeen ids of an older stamp, all `pass`, records `cli_verified:false`.
- Statusline shell-out: the shell's row was owed to "Paste newline ledger row"; now to "Owed ledger rows".
**Why:** The fifth paste widens the ratified verify child set. Ratified by the founder's own answer to P4 fork 1, relayed verbatim by the operator (the chunk's `inputs#I2`). The wrap's card confirm is PROVISIONAL (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary. The earlier three added pastes stay his 2026-10-06 answer. The owed row's move is his own answer to fork 3 (`inputs#I2`).
**Kept:** §Data Protection's Run B transcript count (two per real-CLI verify) stands: the newline paste opens no session.
**Ref:** .andromeda/runs/2026-10-13T21-52-39-wrap/

## 2026-10-13-owed-ledger-rows — Run B's probe --settings, the statusline capture arm, the relied set and the kept workspace.repo
**Section:** §Threat Model Summary (verify child set); §Input Validation (Hook stdin row, CLI arguments row, statusline shell-out row); §Data Protection (probe captures, Repository fixtures, accepted risk of verify's trusted runs); §Error Handling; §Security Anti-Patterns → Universal, → Data Protection
**Change:**
- Run B, on Unix and only for shell-plain paths, carries exactly one `--settings <probe dir>/typed/statusline.json` (0600, `replace_private`). Its command is `<pinned viola> hook statusline --capture <dir> --shell "$0"`.
- Windows passes none. Runs A, C and D keep their argv, and no run or start is added. On Unix, Run B's status line is the probe capture, not the user's.
- New hidden arm `hook statusline --capture <DIR> [--shell <WORD>]`, taken before the pass-through:
  - `<DIR>` must be absolute and exist;
  - stdin goes through `take(MAX_FRAME + 1)` and is refused past `MAX_FRAME`;
  - the payload is written raw at the first free `StatusLine.<k>.json` (`create_private_new` + `replace_private` 0600);
  - `StatusLine.<k>.shell` holds exactly `bin-sh` or `other` (the closed `ShellVerdict`), and the word is written nowhere;
  - it prints nothing, exits 0 on every path, and reads no `VIOLA_*`, obs init, channel, `budget.json` or snapshot.
- verify reads the files back through `take(MAX_FRAME + 1)`, the shell file capped at 16 bytes.
- The probe dir holds `typed/statusline/` (0700) and `typed/statusline.json`.
- The shell-out row: the shell, the override and `rate_limits` were "relied on with no ledger row, owed to Owed ledger rows". They are now the rows `statusline-shell`, `statusline-override` and `statusline-rate-limits`, stamped on 2.1.287 with two fixtures. The Windows shell-out stays owed to "Windows-only live measurements".
- Universal: the ledger has twenty-one rows, and a stamp is read over the relied set (`/21` on Unix, `/18` on Windows). An all-`pass` 18-id stamp reads `cli_verified:false` on Unix.
- Repository fixtures: a `workspace.repo` naming this repository's own remote is kept, neither rewritten nor refused; nothing else in the rule widens.
**Why:**
- The probe `--settings` and the capture arm are a boundary widening of the ratified verify child set. Ratified by the founder's own answer to P4 fork 3, relayed verbatim by the operator (`inputs#I1`). The wrap's card confirm is PROVISIONAL (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary.
- The kept `workspace.repo` is the founder's own word (`inputs#I3`, relayed verbatim by the operator, given after the gap was shown to him). Its wording is the operator's (`inputs#I4`), and its landing here is the overseer's application of it.
**Ref:** .andromeda/runs/2026-10-14T06-37-54-wrap/

## 2026-10-14-resume-and-compact-ledger-rows — Run E in the verify child set; a third session-id source
**Section:** §Threat Model Summary (the `revive` and `viola verify` entry points); §Input Validation (Session id as a child argv value; Constants `MAX_FRAME`); §Data Protection (Probe captures; Interactive probe dirs; Accepted risk (verify's trusted runs, B, C, D and E)); §Security Anti-Patterns (Universal, the stamp-gated dialog decision)
**Change:**
- verify spawns five interactive children (was four). Run E runs last:
  - in `<cwd>/.viola-verify-<pid>-resume/` (0700, its guard);
  - with the run args, `--plugin-dir` and the `resume/` capture plugin, then `--resume <id>` from `resume_args(id, false)`;
  - with no `--settings` and empty input, so no key is typed;
  - it waits for one SessionStart up to `PROBE_DEADLINE`, settles, and is killed.
- An uninterrupted verify writes seven spawn pairs.
- The Session id row is retitled (`viola revive`, `viola verify`'s Run E) and gains a third id source:
  - the `session_id` of Run B's first SessionStart capture, through `ledger::resume_target`, only when `is_session_id` accepts it;
  - with none, Run E spawns nothing and `resume-session` fails;
  - `resume_args` gains its second caller.
- `MAX_FRAME`'s child output covers runs A to E.
- Probe captures gain the `resume/` root, and the interactive probe dirs gain `-resume/`.
- The accepted risk names Run E. It adds no transcript file, as measured on 2.1.287: `~/.claude/projects/` 643 → 647 → 651 over two rounds, four per round.
- The ledger has twenty-two rows (with `resume-session`). The counter is `/22` on Unix and `/19` on Windows.
- An all-`pass` 18- or 21-id older stamp reads unverified on every OS. The failing example now reads `18 pass  4 fail`.
**Why:**
- Run E and its id source widen the ratified verify child set and the session-id crossing.
- They are ratified by the founder's own answer to P4 fork 2, relayed verbatim by the operator (`inputs#I1`).
- The wrap's card confirm of the proposals inside it is PROVISIONAL (delegate viola-overseer, `inputs#I6`), superseded by his own word at the Epoch 5 boundary.
**Ref:** .andromeda/runs/2026-10-16T14-05-30-wrap/

## 2026-10-16T14-58-10-wrap — the loopback GUI leaves 0.1.0: no HTTP surface, no GUI secret
**Section:** Threat Model Summary (scope, the Local HTTP vector, data classification, the elevations); Authentication & Authorization (auth approach, the token, cookie and GUI principal rows, recovery flows, strict-modes `ui/` reads); Input Validation (`Last-Event-ID`, Host and brake rows); Data Protection; API Security; Dependency Security; Bootstrap phases; Secret Management; Error Handling; Security Anti-Patterns
**Change:**
- Was a loopback HTTP GUI: per-launch 256-bit token, `ui/<port>.url`, `viola_<port>` cookie, Host allowlist, CSP, `/api/*`, SSE, RFC 9457 Problem Details, a threat vector and its elevations. Now viola has no HTTP surface. API Security holds the stub line ("The frontend is deferred to 0.2.0; its technology is not chosen.") and keeps only the MCP transport row.
- The only secrets are the inherited `CLAUDE_CODE_MESSAGING_TOKEN` / `_SOCKET` and the R8-stripped `CLAUDE*` values. The NEVER-log floor and the external-error rules name no GUI token, cookie, launch URL, `/api/info` or Problem Details.
- The strict-modes and DACL reads no longer name `ui/`. The elevations are two, not three.
**Why:** The founder's ruling of 2026-10-16 (his own words, relayed verbatim by the Viola overseer) makes 0.1.0 a pure CLI and moves the frontend to 0.2.0, its technology not chosen.
**Kept:** By the operator's answer at this wrap (overseer, technical, founder-delegated): the masters stay true to the tree.
- The npm / Node / npm-audit part of Dependency Security stands, marked as owned by the route entry "Remove the browser leg".
- The secret-scan classes `?t=`, `cookie:` / `viola_<port>=` and the GUI token from `ui/*.url` stand, with their `?t=` note, because `secret_scan.rs` still tests them.
- The `axum` `http2` ban stands, because `deny.toml` still holds it.
- The listener ban stands (only «/GUI» is cut).
**Ref:** .andromeda/runs/2026-10-16T14-58-10-wrap/

## 2026-10-16-remove-the-browser-leg — the npm graph, the Node pin, the axum ban and the GUI scan forms leave
**Section:** §Dependency Security (Audit tool, `deny.toml` additions, Pinning, CI integration); §Bootstrap phases (dep-audit-tooling-install, secret-scanning-ci-gate, dep-security-ci-gate); §Secret Management (Storage → Development, Secret scanning in CI)
**Change:**
- Was a test-side npm graph (`e2e-web/package-lock.json`, `@playwright/test` 1.63.0) audited by `scripts/npm-audit.sh` in `supply-chain` and nightly `npm-advisories`, a Node pin (ci.yml's `NODE_PIN_*`, `scripts/install-node.sh`), Chromium from Playwright, and a stand-until sentence. Now none exists: the Audit tool list is cargo-deny, zizmor, the deferred cargo-audit and the ConPTY vendor audit (outside every cargo graph); job 4 audits the root and `fuzz/` lockfiles; nightly runs `advisories` and `fuzz`; toolchains come from `rustup toolchain install` alone.
- `[[bans.features]]` holds the `rmcp` ban alone (`transport-streamable-http-server`, `auth`); the `axum` `http2` ban left with its `deny-probes.sh` row. Every remaining ban is still proven live.
- The pre-push `tools` stage checks `cc`, the `rust-toolchain.toml` channel and the `test`-job `tool:` pins (cargo-nextest, cargo-mutants, cargo-llvm-cov); no Node check and no `node` detail. Every child runs as `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin <argv>` (four PATH elements); the HOME + PATH-only rule, the passwd-home probes and the canary pair are unchanged.
- The artifact canary scan's classes: the stripped `CLAUDE*` canaries (the twelve `claude-stripped` needles), the case-insensitive `cookie:` header, the content canary, a non-`0600` diagnostics file (Unix). Was also `?t=`, `viola_<port>=` and the GUI token read from each home's `ui/*.url`. The `chunk.diff` skip's reason: repository source text that trips the scan's own needles whenever it touches the scan's source.
**Why:** This chunk removed the browser test leg (founder ruling 2026-10-16, item 3). The scan forms and the axum ban went on the operator's P4 answers as the overseer (technical, founder-delegated; `inputs#I1` of this chunk): each removed form matches only the GUI's own token or cookie, which no 0.1.0 path writes, and the ban's reason left with the GUI while axum is in no lockfile. These narrow a control whose subject is gone; no boundary is widened.
**Ref:** .andromeda/runs/2026-10-16T17-15-05-wrap/

## 2026-10-16-unwrapped-sessions-on-the-board — the claude agents --json child of list, revive and verify Run B
**Section:** §Threat Model Summary (child-spawning entry point; the `viola verify` entry point); §Input Validation (child process output row; child executable resolution row; Constants); §Bootstrap phases → logging-redaction-wire; §Security Anti-Patterns → Universal (the stamp gate)
**Change:**
- The `claude agents --json` child of CLI `list` (one per run, after `board::read`), of `viola revive`'s preflight (before its recorded-directory reading, skipped under `--fork`; a live id elsewhere is the closed refusal `session-live`, no child; a failed read passes) and of `viola verify`'s Run B is a direct spawn of the resolved program under the R8 strip, its stdout through `take(MAX_FRAME + 1)`, killed at 5 s, parsed tolerantly by `viola_agent_claude::agents::parse` into rows or the closed `AgentsFailure` (`oversize` · `malformed`, one `parse-rejected{parser:"claude-agents-json"}`); a failed, oversize or malformed read leaves `list`'s board as `board::read` gave it and passes `revive`. `list` and `revive` resolve `claude` by `run::resolve_program` from their current directory; `mcp` spawns none (its `list` is not built). Each `agents-probe` pair holds no row text, id, `cwd` or path. Was: none does, `list` resolves no `claude`, a `take(MAX_FRAME)` read whose failure becomes `unknown`.
- Run B makes ONE bounded agents read once its first turn has settled, before the framing pastes, the capture in memory; `--record` keeps only Run B's own row in `Agents.recorded-1.json`; verify writes eight spawn pairs (was seven).
- An unwrapped NAME (the row's `sessionId`) is never a `ViolaName` or `target` and is printed through `escape_row`, its width in characters (was: escaped "when those rows land").
- The ledger has twenty-three rows (`agents-join` last); the stamp gate reads `/23` on Unix and `/20` on Windows (the first eighteen, `resume-session`, `agents-join`); a dialog-less home reads `19 pass  4 fail`; a stamp of exactly the twenty-two pre-chunk ids reads unverified.
**Why:** Run B's read widens verify's child set and `revive`'s read and refusal widen its preflight: each the founder's own answer to a P4 fork (fork 1; forks 3 and 4), `inputs#I1`, relayed verbatim by the operator; the wrap's card confirm provisional (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary. `list`'s spawn was stated before this chunk.
**Ref:** .andromeda/runs/2026-10-16T22-30-02-wrap/

## 2026-10-17-cli-machine-contract — security: revive's --json documents; the CLI home resolution as landed (not ratified)
**Section:** §Input Validation CLI arguments / stdin row (Home path, `revive` arguments, `list` arguments → Home resolution); §Input Validation Statusline source row; §Authentication & Authorization `~/.viola/` access control row (revive's strict-modes refusal)
**Change:**
- `revive`: `--json` accepted on both arms (was: "the verb takes no `--json`"); a refused strict-modes check or a preflight refusal prints exactly `{"v":1,"error":"<code>","detail":null}` (stderr empty, exit 1; closed codes `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`), `--list --json` the escaped `ts`, the closed cause word and the session-shaped id only — never a path or pid; a post-preflight `run` start refusal prints no document (owed to "Exit-cause code catalogue"). A refused check, in either form, opens no log and writes no line; `--list` logs in the caller's `cli-<VIOLA_NAME>.ndjson` only after a passed check.
- Home resolution (was: every `cli` verb makes `--home` absolute without canonicalising it, owed to "CLI machine contract"): every CLI verb, `run` and `revive` included, resolves `--home` as given, else the `instance_of` home (`VIOLA_NAME` through `ViolaName::try_new`, `VIOLA_DIR` an absolute `<home>/instances/<name>`; a failing pair falls through), else `<user home>/.viola`; `canonical_home` canonicalises an existing home (Windows: only the verbatim-drive prefix stripped) and creates nothing. The Home path sentence reads "once the home exists".
- Statusline source: the default-home test is `canonical_home(default_home(…))`, the read `resolved_home`'s; a `VIOLA_DIR` home reads no user settings.
**Why:** revive's machine form is closed-code output (the overseer: routine, no input class added). The CLI verbs' `VIOLA_DIR` step and the canonicalisation are a boundary widening NOT ratified: the Viola overseer's answers (P4 `inputs#I2`, this wrap's Q1 `inputs#I3`) are a delegate's; the founder's own word is owed at his next batch.
**Ref:** .andromeda/runs/2026-10-17T03-50-43-wrap/

## 2026-10-17-cli-output-discipline — the terminal colour facts NO_COLOR / TERM / COLORTERM join the admitted env set
**Section:** §Input Validation (a new row); §Secret Management → Storage; §Security Anti-Patterns → Universal
**Change:**
- Input Validation: a row for `NO_COLOR`, `TERM`, `COLORTERM`, read in every build by `viola list`'s human mode only: through `std::env::var_os` as text with stdout's `IsTerminal`, compared only, never echoed, decided into the closed `Depth` (`Truecolor` · `Ansi256` · `Ansi16`) or none — none under `--json`, a passthrough role, `NO_COLOR` present (empty included), `TERM` exactly `dumb`, a non-terminal stdout, or a failed Windows VT enable (`SetConsoleMode(mode | 0x0004)`, restored on drop, never `0x000c`); they decide only SGR bytes around rows already escaped, reach no log line, error or refusal, change no escaping or check, disable no control, widen no redaction; `COLUMNS`, `CLICOLOR`, `CLICOLOR_FORCE`, `CI` are not read; the width is `viola_pty::host_size()`.
- Storage: was "the two variables outside `VIOLA_*` plumbing, the test-only seams"; now the seams plus the three colour facts, read in every build for colour only.
- Universal: was "the two carve-outs are the test seams"; now the three colour facts stand beside them, switching off no control; NEVER another seam or environment read, or a colour fact changing a control or reaching a log, error or refusal, without its own Decisions Log entry.
**Why:** a boundary widening ratified by the founder's own answer «Читать их» to P4 fork Q1 (relayed verbatim by the operator); the answer card confirmed by a delegate, the Viola overseer — PROVISIONAL, superseded by his own word at the Epoch 5 boundary batch.
**Ref:** .andromeda/runs/2026-10-17T08-28-14-wrap/

## 2026-10-17-mcp-server-for-drivers — the eighth dated gap: MCP frame tools after the liveness-only pre-check
**Section:** Authentication & Authorization (IPC client-side server verification row; the `~/.viola/` strict-modes entry points); Security Anti-Patterns → Authentication; Input Validation (CLI arguments, the interim gaps reading `snapshot.json`)
**Change:**
- Server verification row: was seven dated gaps; now a sixth interim gap, the eighth dated gap overall — until "Server verification before any frame" and "Home and code-bearing file integrity" land, `viola mcp`'s `send` / `wait` / `last` / `answer` tools write their frames through the Tokio client after the liveness-only pre-check (`viola_state::liveness::live_endpoint`; else `{"error":"instance-unreachable","detail":null}`), checking neither the server's identity nor the snapshot's strict-modes, borrowing none of the seven earlier gaps.
- Anti-Patterns → Authentication: the dated exceptions add the MCP tool frames (the eighth).
- Strict-modes entry points: `mcp` checks at start only its home and `diagnostics/` (each that exists) through `check_path` before anything under the home opens (exit 1, nothing printed, no role file); its frame tools read `snapshot.json`'s `endpoint` under the eighth gap, without the check; the gap list adds the eighth.
- CLI arguments gap list: an eighth — the MCP frame tools read `snapshot.json` the same way.
**Why:** a boundary widening ratified by the founder's own answer to P4 fork Q1, «Восьмой gap + agents», live in the overseer's dialog on 2026-10-17, relayed verbatim by the overseer. The wrap's answer-card confirm is a delegate's (the Viola overseer) — PROVISIONAL, superseded by his own word at the Epoch 5 boundary batch. The gap closes with the other seven at the two Epoch 6 entries.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/

## 2026-10-17-mcp-server-for-drivers — the mcp process spawns claude agents --json
**Section:** Threat Model Summary (the `claude agents --json` spawn bullet); Input Validation (Child executable resolution row)
**Change:** was "`mcp` spawns none (its `list` is not built)" and "`mcp`'s `list` is not built"; now the `mcp` process's `list` tool makes one `claude agents --json` read per call through CLI `list`'s board path (`list::board_document`, run on a blocking thread), its `agents-probe` pair logged in `diagnostics/mcp.ndjson` under the same no-row-text, no-id, no-`cwd`, no-path rule; its output stays untrusted and `MAX_FRAME`-bounded, a row's `name` never a `ViolaName` or `target` in the MCP `list` tool.
**Why:** a boundary widening (a new process spawning `claude`) ratified by the founder's own answer to P4 fork Q1, «Восьмой gap + agents» (MCP `list` reads agents as CLI `list` does), relayed verbatim by the overseer. The wrap's answer-card confirm is a delegate's (the Viola overseer) — PROVISIONAL, superseded by his own word at the Epoch 5 boundary batch.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/

## 2026-10-17-mcp-server-for-drivers — MCP tool input validation and error bodies as landed
**Section:** Input Validation (MCP tool inputs row; Paste text row, Where it runs); Error Handling (External responses, the MCP `isError` bullet)
**Change:**
- MCP tool inputs: each handler takes the raw argument object (never rmcp's typed deserialisation): `target` through `ViolaName::try_new`, `text` a string, `after` / `timeout_ms` absent, `null` or a `u64` (`wait`'s clamped to `WAIT_CAP` 50 000 ms, also its default), `dialog_id` a `u64`, `response` through `Response::parse`, then `validate_paste_text` on `send`'s text and every free-text answer value; a failure is `isError: true`, `{"error":"invalid-params","detail":"<field>"}` (closed field words `target` · `text` · `after` · `timeout_ms` · `dialog_id` · `response`), text `refused: invalid-params`, no frame; an unknown tool `-32602`.
- Paste text: was "MCP when it lands"; now the MCP `send` tool runs `validate_paste_text` then the empty-typed-text check client-side, each refused `not-delivered` / `control-character` | `empty-text` with one client-side `send-refused` line, before any frame.
- `isError` bullet: was the closed list `instance-unreachable | wrapper-fault`; now it adds `invalid-params`, `instance-unreachable` carrying `detail: null`, `wrapper-fault` the fault's `{code,message,data}`, every text `refused: <code>`; any other handler fault, a reply of no known shape or a panicked call is JSON-RPC `-32603` `internal error`, `data: null`.
**Why:** the chunk landed the validation the plan required; the bodies stay codes only.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/

## 2026-10-17-mcp-server-for-drivers — the tokio ban's tree form and the rmcp release-graph gate
**Section:** Dependency Security (the arch bans bullet); Threat Model Summary (Trust boundary; CI jobs); API Security (MCP transport row)
**Change:**
- Dependency Security: the sole-root `deny-sync.toml` tokio ban runs over the four crates of `scripts/sync-crates.txt`; `viola-channel` (its Tokio client, tokio `=1.53.1` `io-util`, behind its `tokio` feature) left the list and is covered by `scripts/channel-tokio-free.sh` (no `tokio v` line without the feature, one with its control), proven by `deny-probes.sh`'s `channel-tokio-edge` probe and control (13/13).
- Trust boundary: names both tokio-ban forms.
- CI jobs: lint adds the per-OS `rmcp features` step (`scripts/rmcp-features.sh`); supply-chain runs the tree form after the sole-root loop.
- MCP transport: was an unscoped "rmcp features exactly `server` and `transport-io`"; now scoped to `viola`'s release graph and checked by `scripts/rmcp-features.sh` (rmcp present, exactly those two); the test-only `viola-e2e` takes rmcp `client` only, never `transport-child-process`.
**Why:** the ban's invariant holds in a new form: no listed sync crate depends on `viola-channel`, so the sole-root loop could not cover it (premise corrected by measurement). New dependencies tokio `=1.53.1` and schemars `=1.2.2` are exact pins; rmcp's range and features match the bans.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/

## 2026-10-17-mcp-server-for-drivers — viola mcp's home through resolved_home, not ratified
**Section:** Input Validation (CLI arguments row, **Home resolution**)
**Change:** the hidden `viola mcp` resolves its home through the same `resolved_home` as every CLI verb (`--home`; else the `instance_of` home from `VIOLA_NAME` / `VIOLA_DIR`; else the default; then `canonical_home`). Its use of the `VIOLA_DIR` step and the canonicalisation is written as built and is NOT ratified.
**Why:** a boundary widening of the class already carried for the CLI verbs; written on the Viola overseer's answer at this wrap (a delegate's), it joins that carried item, the founder's own word owed.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/

## 2026-10-17-optional-plugin-install — plugin install's tree, its check and the marketplace .mcp.json
**Section:** §Authentication & Authorization (the `~/.viola/` access-control row); §Data Protection (At rest → Code-bearing artefacts); Threat Model Summary (Data classification → config; Attack surface → Filesystem state under `~/.viola/`)
**Change:**
- Access control: `viola plugin install` checks only its home and each existing `marketplace/` tree dir through `check_path` before anything under the home is created, written or opened; a refusal exits 21 (`instance-unreachable` under `--json`), nothing created, no log opened. `marketplace/` is no trusted file (viola never reads it back); the installed `command` gets no re-hash.
- Code-bearing artefacts: `marketplace/` is written only by the verb — two manifests and `viola/.mcp.json`, whose `mcpServers.viola` `command` is the canonical exe that ran the install and whose `args` are `["--home", <the canonical home>, "mcp"]`, built by `serde_json`, no hooks; each file rewritten whole at 0600, each dir 0700, any other entry left as found. viola spawns no `claude` and writes nothing under `~/.claude`.
- Threat Model: `marketplace/viola/.mcp.json` joins the config files holding an exec path and the code-bearing entry points.
**Why:** the chunk built the verb; write + print with no `claude` child is the founder's own answer to P4 fork Q1, relayed verbatim by the Viola overseer. The unpinned `command` is the standing design ([Deployment / Distribution]). The tree's integrity against a same-user writer is owned by the route entry "Home and code-bearing file integrity", on the operator's word at P5.
**Ref:** .andromeda/runs/2026-10-18T02-36-56-wrap/

## 2026-10-17-optional-plugin-install — the live round's residual under ~/.claude
**Section:** §Data Protection (an accepted residual)
**Change:** a live round that installs the plugin runs the CLI's own four commands (`claude plugin marketplace add` / `install` / `uninstall` / `marketplace remove`) at `--scope local` from a rig dir under `target/` that is its own git repo, since the CLI takes the local scope's project from the git work tree root. The CLI's writes under `~/.claude` are its own: after the removal its orphaned plugin cache `~/.claude/plugins/cache/viola/viola/<version>/` stays, `installed_plugins.json` and `known_marketplaces.json` back as before (as measured on claude 2.1.287 at this chunk). viola itself writes nothing under `~/.claude`.
**Why:** a boundary widening (the round writes the CLI's own configuration), ratified by the founder's own answer to P4 fork Q3 «Local scope в стенде», relayed verbatim by the Viola overseer, and his later answer to the same fork «proj/ — свой git-репо», relayed verbatim by the operator. The wrap's answer-card confirm that this entry lies inside them is a delegate's (the Viola overseer) — PROVISIONAL, superseded by his own word at the Epoch 5 boundary batch.
**Ref:** .andromeda/runs/2026-10-18T02-36-56-wrap/

## 2026-10-18-compact-post-condition-ledger-row — Run B's sixth paste, its residual, and the twenty-four-row stamp
**Section:** Threat Model Summary (Child process spawning, the `viola verify` entry point bullet); Data Protection (Accepted risk, verify's trusted runs); Security Anti-Patterns → Universal (the `hook.dialog` / `ledger/stamps.json` bullet)
**Change:** Run B gets six compiled pastes, `PROBE_COMPACT_COMMAND` (`/compact`, `LOCAL_COMMANDS[2].0`, which compacts its session in place and keeps its id) between the newline paste and `/clear`; each of the five added pastes only into the settled input box, none after the stop, `/compact`'s answer waited up to `PROBE_DEADLINE` before the `/clear` rows; no run, start or spawn pair added. Run B's accepted residual is three files (its session, the compaction's `<id>-backup-<n>.jsonl`, the `/clear` session), measured 60 → 65 per round. The stamp gating non-`null` decisions reads twenty-four rows, `local-command-compact` relied on every OS (`/24` Unix, `/21` Windows); a `20 pass  4 fail` Unix home (`17 pass  4 fail` Windows) and a twenty-three-id older stamp read `cli_verified:false`. Was: five pastes, four added; two Run B transcripts; twenty-three rows, `/23` / `/20`, `19 pass  4 fail`.
**Why:** a boundary widening of the ratified verify child set: the sixth Run B paste is the founder's own answer to P4 fork 1 («Шестая вставка в Run B», `inputs#I3`, relayed verbatim by the operator); this wrap's answer-card confirm of the proposals inside it is a delegate's (delegate the Viola overseer), PROVISIONAL until his own word in the Epoch 5 boundary batch; its option text named the transcript effect as unmeasured, so the measured backup lies inside it.
**Ref:** .andromeda/runs/2026-10-18T21-59-05-wrap/

## 2026-10-18-plugin-precedence-live-readings — the live round's rig outside the work tree, its trust keys and residual
**Section:** §Data Protection (Accepted residual, a live round of `viola plugin install`)
**Change:** was "from a rig dir under `target/` that is its own git repo" and "viola itself writes nothing under `~/.claude`", per 2026-10-17-optional-plugin-install — the live round's residual under ~/.claude; now the four CLI commands run at `--scope local`, a fresh count of four per rig, from a `proj/` that is its own git repo outside this work tree (the e2e-home tmpfs, no ancestor `CLAUDE.md`), since an in-tree `proj/` meets the external-imports dialog after the trust dialog; the rig passes the folder-trust dialog by keys into its own pty only, never a desktop key — at most 2 arrows toward the trust row and 1 Enter only on a read `selected`, any other screen no key — and on a wrapped start `viola release <name>` precedes the driver's send; the residual is the CLI's own writes: its `~/.claude.json` trust entry per accepted dialog, the transcripts under `~/.claude/projects/`, the orphaned plugin cache re-written in place, `installed_plugins.json` and `known_marketplaces.json` back as before; viola writes nothing under `~/.claude` or into `~/.claude.json`.
**Why:** boundary widenings ratified by the founder's own words, relayed verbatim by the operator: his answer to P4 fork 1 «Стрелка + Enter» and to fork 2 «3 старта» (the keys, the release, four commands per rig) and his word «Стенд вне репо» after the in-tree start met the external-imports modal (the rig place, given after that widening was shown to him). This wrap's answer-card confirm of the F1 / F2 part is a delegate's (delegate the Viola overseer), PROVISIONAL until his own word in the Epoch 5 boundary batch; the rig place was confirmed in the same halt.
**Ref:** .andromeda/runs/2026-10-19T07-00-05-wrap/

## 2026-10-18-plugin-precedence-live-readings — the VIOLA_DIR home step ratified
**Section:** §Input Validation (CLI arguments / stdin — home resolution)
**Change:** was "widen what chooses a CLI verb's home and are NOT ratified: the founder's own word is owed …; `mcp`'s use of them is written as built and is NOT ratified either"; now the CLI verbs' `VIOLA_DIR` step and the canonicalisation, `mcp`'s use of them included, widen what chooses a CLI verb's home: a widening the founder ratified.
**Why:** the founder's own word «Ратифицировать», relayed verbatim by the operator, answered a question describing the whole step, canonicalisation and `mcp` included (the operator's correction at the P5 review); the lift was confirmed in this wrap's halt. It supersedes the delegate answers the marks were written on.
**Ref:** .andromeda/runs/2026-10-19T07-00-05-wrap/

## 2026-10-19-dialog-hold-for-the-driver — `dialog_hold_s` validated into `DialogHold`; the dialog reply bound at the ceiling
**Section:** §Input Validation (Configuration values (`config.json`) row; Hook stdin (all hook events + `hook statusline`) row).
**Change:**
- Configuration values: `dialog_hold_s` is read once at the wrapper's start by `run::dialog::read_hold`, right after the thresholds, through its own `take(MAX_FRAME)` read with `v` = 1. A JSON whole number of seconds that fits `DialogHold` (60–3600, `DialogHold::try_new`; default 60, `DIALOG_HOLD_DEFAULT`) is the hold. Anything else gives 60 and one `parse-rejected{parser:"config-json", detail:"malformed"}` line. A document that is no object, or whose `v` is not 1, gives 60 with no rejection. No value makes the hold zero or unbounded, and no env var or flag sets it. The How column names the hold read in `src/run/dialog.rs`, and `DialogHold` as `viola-core`'s beside `Percent`.
- Hook stdin: the dialog reply read is bounded at `DIALOG_HOLD_MAX + 5 s` (3605 s; the wrapper replies `null` at its own configured hold's end). Was `DIALOG_DEADLINE + 5 s` (65 s), the constant retired.
**Why:** The key is a validated surface admitting a new input, a boundary widening the founder answered himself to P4 fork Q1, relayed verbatim by the operator (`inputs#I2`). The wrap's answer-card confirm was the Viola overseer's (delegate viola-overseer), PROVISIONAL, superseded by his own word at the Epoch 5 boundary. The hook's bound follows the ceiling so `plugin/<key>/` stays byte-identical across starts.
**Ref:** .andromeda/runs/2026-10-19T09-28-14-wrap/

## 2026-10-19-overseer-switch-to-viola — question option text crosses into the event and `wait` payloads; `background_tasks` a count
**Section:** §Input Validation (Hook stdin row, **Question and Stop data**); Threat Model Summary (Data classification, the dialog content's Where); Bootstrap phases → `logging-redaction-wire` (the floor's Scope); Security Anti-Patterns → Code Patterns (R1).
**Change:**
- Hook stdin: each question's `option_details` takes an option's `description` and `preview` only as strings, else `null`, never a refusal (`options` keeps its labels); the Stop's `background_tasks` is read as `serde::de::IgnoredAny` elements and carried only as the array's length, `null` when absent, `null` or not an array, a non-array also writing the drift entry `{"path":"background_tasks","expected":"array"}`. The option text crossing into `events.ndjson`, `hook.dialog`, CLI `wait --json` and the MCP `wait` is recorded as the founder's own widening.
- Data classification: `question` carries questions, options and each option's description and preview text, which also go out over CLI `wait --json` and MCP `wait`. Was "questions and options".
- The floor's Scope: user content is "prompts, `last_assistant_message`, plan, question and option text". Was "plan and question text".
- R1: upstream text lists "plan text, question and option text, tool `input`". Was "plan text, tool `input`".
**Why:** A validated surface admits more upstream content: a boundary widening the founder answered himself at P4 fork Q4 («Строить оба», `inputs#I14`, relayed by the overseer), `background_tasks` count only by the same answer. The wrap's answer-card confirm of these four sections is his own word, relayed by the overseer (`inputs#I21`).
**Ref:** .andromeda/runs/2026-10-19T19-16-36-wrap/

## 2026-10-19-overseer-switch-to-viola — verify's print-mode probe residual accepted
**Section:** §Data Protection (Accepted risk: verify's trusted runs).
**Change:** After "Run A leaves nothing": the print-mode probe leaves the CLI's empty project dir for its cwd `ledger/probes/<pid>/` under `~/.claude/projects/` (a `memory/` dir, no transcript), accepted into the same class, as measured on 2.1.296 and 2.1.302 on the Linux dev host. Was unstated.
**Why:** Measured at the switch's verify residual count. The founder's own word, live, relayed by the overseer (`inputs#I21`), accepts it into the class of the Run B–D transcripts. viola itself still writes nothing under `~/.claude`: this dir is the CLI's write.
**Ref:** .andromeda/runs/2026-10-19T19-16-36-wrap/

## 2026-10-19-newer-claude-cli-rows — the scratch footer reading of a newer CLI accepted
**Section:** §Data Protection (Accepted residual: a scratch footer reading of a newer CLI).
**Change:** New. One `viola run` start of the CLI under a tmpfs `--home`, in a detached 80×24 pane, its cwd a fresh `.viola-footer-<pid>/` under the repository root, with no key typed. One `viola list` read. At two instants, the pane's bottom three rows, scrubbed of the home and user word, written to the chunk's `evidence/`, never a row above the footer. The start is then killed and its cwd removed. It left no transcript (no prompt typed), as measured on 2.1.302. It is no product path and no standing permission: another reading needs its own answer.
**Why:** A new crossing (a subprocess start of the user's CLI, its screen rows into the repository): a boundary widening the founder answered himself to Fork 2 of the chunk's P4 («Один scratch-старт»), relayed by the Viola overseer with his option labels verbatim. The wrap's answer-card confirm is his own word too («Да, внутри»), relayed verbatim by the operator. Earlier scratch answers (2026-10-07) stay spent.
**Ref:** .andromeda/runs/2026-10-20T05-48-20-wrap/

## 2026-10-19-newer-claude-cli-rows — Run B's pastes go into rows holding an input-box literal
**Section:** Threat Model Summary (verify's Run B, the six compiled pastes).
**Change:** Each of the five added pastes goes only into rows that hold *an* input-box literal and no modal literal. It was "the input-box literal". The compiled set now holds two input-box literals (architecture [Screen Model]).
**Why:** A wording follower of the second compiled literal. The pastes, their order and the start count are unchanged.
**Ref:** .andromeda/runs/2026-10-20T05-48-20-wrap/

## 2026-10-20-t06-58-31-wrap — the founder's Epoch 5 batch: the card confirms his, revive's no-line refusal his
**Section:** §Threat Model Summary (the `claude agents --json` children; `viola verify`'s PTY children) · §Authentication & Authorization (`~/.viola/` access control) · §Input Validation (hook stdin; the session id as a child argv value) · §Data Protection (accepted risk: verify's trusted runs) · §Error Handling (`hook` never writes to stderr)
**Change:**
- The founder's own answers on `revive`'s agents read, Run B's probe `--settings`, its agents read, its newline and `/compact` pastes and its compaction residual, and Run E, stand as his; was "the wrap's card confirm of it provisional (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary" on ten clauses and "the wrap's card confirm provisional (delegate the Viola overseer)" on one; now the clauses are gone.
- `viola revive`'s refused strict-modes check opens no log and writes no line as built; was "that a refused check writes no line is the operator's answer at that chunk's planning, and the founder has not ruled on it"; now the clause is gone.
**Why:** the founder's own answers to his Epoch 5 batch, 2026-10-20 (ratify every delegate mark of the epoch; accept the refused checks' no-line shapes as built), relayed by the Viola overseer through the operator's directive. Each provisional record stands in its own entry, superseded 2026-10-20 (the next entry). The bodies take writing rule 7's form, by the overseer's technical answer at this wrap.
**Ref:** .andromeda/runs/2026-10-20T06-58-31-wrap/

## 2026-10-20-t06-58-31-wrap — the Epoch 5 entries' provisional records, superseded 2026-10-20
**Section:** none in the body — this file's history: the entries named below
**Change:** each named entry's provisional mark (a delegate's card confirm or technical answer) is superseded 2026-10-20 by the founder's own word; every other claim of those entries stands, and none is pruned:
- 2026-10-13-paste-newline-ledger-row — Run B's fifth compiled paste, eighteen stamp rows, the shell's row owed on "Owed ledger rows"
- 2026-10-13-owed-ledger-rows — Run B's probe --settings, the statusline capture arm, the relied set and the kept workspace.repo
- 2026-10-14-resume-and-compact-ledger-rows — Run E in the verify child set; a third session-id source
- 2026-10-16T14-58-10-wrap — the loopback GUI leaves 0.1.0: no HTTP surface, no GUI secret (the masters kept true to the tree)
- 2026-10-16-unwrapped-sessions-on-the-board — the claude agents --json child of list, revive and verify Run B
- 2026-10-17-cli-output-discipline — the terminal colour facts NO_COLOR / TERM / COLORTERM join the admitted env set
- 2026-10-17-mcp-server-for-drivers — the eighth dated gap: MCP frame tools after the liveness-only pre-check
- 2026-10-17-mcp-server-for-drivers — the mcp process spawns claude agents --json
- 2026-10-17-optional-plugin-install — plugin install's tree, its check and the marketplace .mcp.json (the exe's canonical path with `--home`, and `marketplace/` off the integrity set)
- 2026-10-17-optional-plugin-install — the live round's residual under ~/.claude
- 2026-10-18-compact-post-condition-ledger-row — Run B's sixth paste, its residual, and the twenty-four-row stamp
- 2026-10-18-plugin-precedence-live-readings — the live round's rig outside the work tree, its trust keys and residual (the F1 / F2 card)
- 2026-10-19-dialog-hold-for-the-driver — `dialog_hold_s` validated into `DialogHold`; the dialog reply bound at the ceiling
**Why:** the founder's own word on his Epoch 5 batch, 2026-10-20, relayed by the Viola overseer: every Epoch 5 delegate mark is his. The axum ban and the GUI scan forms of 2026-10-16-remove-the-browser-leg stay a separate founder item.
**Ref:** .andromeda/runs/2026-10-20T06-58-31-wrap/

## 2026-10-20-epoch-5-cleanup — the e2e-home keepers refuse a backing below the headroom floor
**Section:** §Security Anti-Patterns (the test-side removal guard; the two keepers)
**Change:** The two keepers also refuse a linked base whose filesystem reads less than the 9 GiB headroom floor free (`statvfs`). Their refusal stays a fixed message that names no path; the headroom one holds the two counts (free and floor, in MiB). Was: absolute target, a real owner-only directory, no target that is itself a link; now that, and the floor.
**Why:** a refusal added on the backing the keepers already reach. It reads filesystem metadata only, creates and removes nothing, and widens no boundary. The floor is a compiled constant; no env var, flag or config key sets it.
**Ref:** .andromeda/runs/2026-10-20T09-40-55-wrap/

## 2026-10-20-epoch-5-cleanup-ii — the `Mcp.` exchange recorded by the test-only tap
**Section:** §Data Protection (Repository fixtures)
**Change:** `fixtures/claude/<cli-version>/` is recorded by `viola verify --record`, its `Mcp.` exchanges by the test-only `viola-mcp-tap`. The tap's `record` sits between one live CLI start and a direct spawn of `viola mcp` (no shell, no PATH lookup), reads each line through `take(MAX_FRAME)` and appends it to a 0600 capture in a private directory, never committed; its `fixture` step keeps the client frames up to and including the first `tools/list` request and each request's result keyed by method, applies the same scrub, and refuses through `ledger::screen_fault` with the file name and the same closed codes `home-path` · `absolute-path` · `username` · `email`, never the content. The class is validated by `schemas/claude-mcp-fixture.v1.json` and hygiene-walked. Was: the directory recorded by `verify --record` alone.
**Why:** a boundary widening — the live CLI's MCP client crosses into a test-only tap and its scrubbed frames are committed — ratified by the founder's own answers to its P4 fork (a cap of one start, then a second start with the prompt on stdin), each relayed verbatim; at this wrap's answer card he confirmed, relayed verbatim, that this section lies wholly inside them.
**Ref:** .andromeda/runs/2026-10-20T19-32-00-wrap/

## 2026-10-20-epoch-5-cleanup-ii — Run E's `resume_run` re-cited at its module
**Section:** §Input Validation (the session id as a child argv value)
**Change:** the row cites `src/cmd/verify/typed/runs.rs` `resume_run` (Run E). Was: `src/cmd/verify/typed.rs`.
**Why:** `typed.rs` is now a module root; Runs A–E live in `typed/runs.rs`.
**Ref:** .andromeda/runs/2026-10-20T19-32-00-wrap/

## 2026-10-25-u55-door — security-plan: split into its body and 9 category files
**Section:** (the whole master — the door)
**Change:** split into its body and 9 category files; the map .andromeda/security-plan.map.tsv
**Why:** the operator's word: the founder's standing word of 2026-10-25 for the door on viola's masters (F2), given after three review rounds, the scoped third pass ACCEPT with 0 DISPUTE, at 1.37 M (1.47× its cost guide); relayed by overseer1.
**Ref:** $TMPDIR/claude-1000/-home-turbolet-dev-projects-additional-andromeda-worker/a2a855dc-9f94-4016-87d5-2f52c0246ab5/scratchpad/security-plan.card.md

## 2026-10-25-windows-endpoint-admission — the Windows pipe's admission controls gain their CI witnesses
**Section:** Authentication & Authorization (authentication-authorization.md: a new `values · auth` group below the IPC access control (Windows) record)
**Change:**
- IPC access control (Windows), the remote-client rejection's witness: asserted on CI's `test (windows-2025)` by `security_negative_viola_listener_rejects_remote_clients`, which opens a client end of a bound `viola_channel::Server::bind` listener and reads its `NamedPipeType` through `NtQueryInformationFile` / `FilePipeLocalInformation`, the `FILE_PIPE_REJECT_REMOTE_CLIENTS` bit (2) set; two test-owned control pipes tell the bit apart, `security_negative_control_accept_remote_pipe_reads_the_bit_clear` (accept-remote, clear) and `security_negative_control_reject_remote_pipe_reads_the_bit_set` (reject-remote, set). Was unasserted; now asserted.
- IPC access control (Windows), the DACL's effect's witness: beyond its text read-back, the protected user + SYSTEM DACL is asserted on `test (windows-2025)` by `security_negative_viola_listener_denies_an_anonymous_open`: with the listener bound first, a `GENERIC_READ` open under `ImpersonateAnonymousToken` is denied `ERROR_ACCESS_DENIED` (5); its control `security_negative_control_default_pipe_admits_an_anonymous_read` shows the same open admitted on a pipe with the default descriptor (null security attributes). Was unasserted; now asserted.
- Both rows' Stack reference: `tests/security_negatives_channel.rs` (Windows only); windows-sys 0.61.2.
**Why:** the chunk's Windows cases passed on CI's `test (windows-2025)`, moving the remote-client rejection and the DACL's effect from unasserted to asserted; the pipe-type read-back and the anonymous-token open are the founder's own choices on the chunk's P4 forks, given live by dialog in the overseer session. The record's own `accept_remote(false)` clause measured true and now has its witness.
**Kept:** the IPC access control (Windows) record's text stands as written; the witnesses sit in their own group below it, with no chunk name.
**Ref:** .andromeda/runs/2026-10-26T00-17-51-wrap/

## 2026-10-25-windows-endpoint-admission — the supply-chain red on RUSTSEC-2026-0386 and its owner
**Section:** Dependency Security (dependency-security.md: a new `values · deps-supply-chain` group after the last one)
**Change:**
- One item: the open advisory RUSTSEC-2026-0386 (`unsound`) against the locked tokio `=1.53.1` fails CI's `supply-chain` job. Its owner is the working-route entry "tokio RUSTSEC-2026-0386 lock bump": a lock bump of `tokio` to `>=1.53.3`, with no ignore-list widening, so RUSTSEC-2017-0008 stays the only advisory ignore.
**Why:** the report's CI read: the full row red with `supply-chain` its one red, on RUSTSEC-2026-0386 against the unchanged tokio 1.53.1, not this chunk's change. The owning route entry is the overseer's founder-delegated answer, relayed by the operator, confirmed by the founder's own yes on the P5 review card, live by dialog in the overseer session. The lock bump's own wrap retires this item.
**Kept:** the `values · deps` item's tokio `=1.53.1` pin stands: no dependency was bumped and `Cargo.lock` did not move.
**Ref:** .andromeda/runs/2026-10-26T00-17-51-wrap/

## 2026-10-26-tokio-rustsec-2026-0386-lock-bump — viola-channel's Tokio client pin at tokio 1.53.3
**Section:** Dependency Security (dependency-security.md: the `values · deps` group's arch-bans item)
**Change:**
- `viola-channel`'s Tokio client, behind its `tokio` feature, reads tokio `=1.53.3`, `io-util` (was `=1.53.1`, per 2026-10-17-mcp-server-for-drivers — the tokio ban's tree form and the rmcp release-graph gate; now `=1.53.3`). The rest of the item stands as written: the C-building ban in `deny.toml`, the `deny-sync.toml` tokio ban for three target triples run per sync crate as the sole root over the four crates of `scripts/sync-crates.txt`, the `cargo tree` form `scripts/channel-tokio-free.sh`, and `scripts/deny-probes.sh`'s `channel-tokio-edge` probe and control at `13/13 banned, control clean`.
**Why:** the chunk bumped the `[workspace.dependencies]` exact pin of `tokio` from `=1.53.1` to `=1.53.3` (`default-features = false` kept) and the lock's tokio entry with it, every other crate unmoved; tokio's containment did not move: the four sole-root bans pass, `channel-tokio-free: ok`, `deny-probes: 13/13 banned, control clean`.
**Ref:** .andromeda/runs/2026-10-26T06-27-48-wrap/

## 2026-10-26-tokio-rustsec-2026-0386-lock-bump — the RUSTSEC-2026-0386 supply-chain red closed
**Section:** Dependency Security (dependency-security.md: the file's last group, a `values · deps-supply-chain` group with one item, removed whole; the file now ends with the `values · deps-supply-chain` group of pinned actions)
**Change:**
- Retired: the item stating that the open advisory RUSTSEC-2026-0386 (`unsound`) against the locked tokio `=1.53.1` fails CI's `supply-chain` job, owned by the working-route entry "tokio RUSTSEC-2026-0386 lock bump" (a lock bump of `tokio` to `>=1.53.3`, no ignore-list widening). Was an open advisory and its owner; now no open-advisory item stands, and the group that held it is gone.
**Why:** the chunk named in that owner line closed the advisory. With tokio locked at `=1.53.3`, a version the advisory lists as patched, `cargo deny check` (all four families, cargo-deny 0.20.2, the CI pin) exits 0 and its output does not contain RUSTSEC-2026-0386. `deny.toml` is untouched, so no ignore was widened, and CI's `supply-chain` job reads green on the pushed HEAD.
**Kept:** RUSTSEC-2017-0008 stays the only advisory ignore, stated by the body's `[advisories]` line. The `deps-supply-chain` subject stays, because three other groups carry it.
**Supersedes:** 2026-10-25-windows-endpoint-admission — the supply-chain red on RUSTSEC-2026-0386 and its owner
**Ref:** .andromeda/runs/2026-10-26T06-27-48-wrap/

## 2026-10-26-server-verification-before-any-frame — the client's server-pid read and the Unix client euid check as built
**Section:** §Authentication & Authorization (authentication-authorization.md: the IPC client-side server verification row's Windows and Unix identity checks; security-plan.md: the Library row) · §Security Anti-Patterns (security-anti-patterns.md: the frame-before-verification ban's pid clause)
**Change:**
- Windows identity check: was `server_process_id()` must equal the snapshot `pid`; now the connected stream's peer pid (the observed server pid) must, beside the sysinfo start time against `started_at`.
- Library row, IPC: was interprocess `PipeStream::server_process_id()`; now `Stream::peer_creds()`, its `pid()` also the client's read of the serving process: Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-opened client end, none on macOS.
- Unix identity check: the client-side `peer_creds().euid()` = `geteuid()` check is marked not built, owed to the route entry "Unix endpoint and home hardening"; the Linux/FreeBSD pid and start-time match is marked built on Linux (the `SO_PEERCRED` peer pid).
- The ban's pid clause: the Windows pid was `server_process_id()`; now the connected stream's peer pid, interprocess `peer_creds().pid()`, which calls `GetNamedPipeServerProcessId` on the SQOS-adopted client end; the Linux/FreeBSD client euid check is owed to "Unix endpoint and home hardening".
**Why:** the build reads the server pid through interprocess 2.4.4's `peer_creds().pid()`, read in that crate's source and witnessed on `test (windows-2025)` by the two client server-pid tests. The chunk built no client euid check, and the working route's line for "Unix endpoint and home hardening" holds the peer euid check on both sides.
**Kept:** the SQOS identification-level open and its never-fall-back rule, and the macOS clause (euid plus the verified 0700 socket directory, the euid check never skipped, a recorded known gap).
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the dated gaps keep only their strict-modes half
**Section:** §Authentication & Authorization (authentication-authorization.md: the IPC client-side server verification row's interim gaps, the first and the fourth to eighth dated gaps) · §Input Validation (input-validation.md: the CLI arguments / stdin row's fourth to eighth gaps)
**Change:**
- Each of the six gaps now holds only its strict-modes half, owed to "Home and code-bearing file integrity"; was no server-identity check until that entry and "Server verification before any frame" landed, per 2026-09-27-hooks-to-normalised-events (the hook's interim gap), the fourth to seventh dated-gap entries of 2026-10-04 and 2026-10-17-mcp-server-for-drivers — the eighth dated gap; their strict-modes halves stand.
- On Linux and Windows each frame now verifies the serving process first: `viola hook`'s `hook.event` and `hook.dialog` (after its shape check) through `src/cmd/hook.rs`'s shared `connect` (`read_snapshot`, `connect_by`, `verify_server`); CLI `send`, `wait` / `last`, `answer`, `pause` / `release` after the liveness pre-check through `client::connect` (the peer pid and its start time against `pid` and `started_at` from the same `live_endpoint` read); `viola mcp`'s four tools after `TokioClient::connect` (`server_verified`, the start-time read on `tokio::task::spawn_blocking`). None reads the snapshot under strict-modes.
- Each residual (a squatted endpoint taking a frame, answering a dialog, swallowing a `pause` or answering a `release`) is closed on Linux and Windows and open on macOS (was open).
- The CLI arguments row: the five gaps are owed to "Home and code-bearing file integrity" alone; `viola send` resolves the endpoint, `pid` and `started_at` through `live_endpoint` and verifies the serving process against them before the frame on Linux and Windows.
**Why:** the report's Retired line: the server-identity half of the first and the fourth to eighth dated gaps is closed on Linux and Windows. Each gap's ratification stands as recorded. macOS reports no peer pid and stays on the recorded gap.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — a failed server check: no frame, and what each client reports
**Section:** §Authentication & Authorization (authentication-authorization.md: the server verification row's On mismatch clause; the new `values · auth` group's CLI refusal item) · §Error Handling (error-handling.md: the `hook` item; two new items)
**Change:**
- On mismatch, or a check that cannot be made: no frame is written; the CLI frame verbs exit 21 `instance-unreachable` with one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` WARN line; MCP returns `instance-unreachable` with `detail: null` and writes no line; `hook` fails open with exit 0, no body and its `hook-decision` line carrying `server-verify-failed` (was CLI/MCP `instance-unreachable` and a hook note in `diagnostics/`). No line or output holds a pid, start time, path or endpoint.
- CLI refusal (new): `send`, `wait`, `last`, `answer`, `pause`, `release` print under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty, in human mode the `unable  <name>  instance-unreachable` line with the hint `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted` last on stderr; the client's exit-13 refusals come first.
- The `hook` item: as landed it runs the server verification on Linux and Windows (macOS the recorded gap), and its fail-open details add `server-verify-failed`; strict-modes is owed to "Home and code-bearing file integrity" (was server verification and strict-modes joining at Epoch 6, per 2026-09-27-hooks-to-normalised-events, the hook's interim gap).
- New: a hook event or dialog whose check fails writes no frame and exits 0 with empty streams (the dialog path after `hook-invoked{corr:null}`, `decision_emitted:false`); a SessionEnd whose check fails goes through `append_session_end`.
- New: an MCP frame tool whose check fails sends no frame and returns `isError` `instance-unreachable`, `detail: null`, text `refused: instance-unreachable`, with no error word added and no log line.
**Why:** the report's built outcome: `server-verify-failed` was already in the `process-exit` and `hook-decision` detail enums, and this build is the first that writes it.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — run's collision check: the server check retired as moot
**Section:** §Authentication & Authorization (authentication-authorization.md: the server verification row's Collision check clause)
**Change:**
- Was "`run` uses the same verification"; now `run`'s collision check reads the snapshot through `read_snapshot` and judges the recorded wrapper by its `pid` and OS start time and the heartbeat; it opens no connection and writes no frame, so there is no serving process to verify and the peer-pid server check does not apply to it (retired as moot).
**Why:** the clause stood before this chunk and was never built: the collision check verifies no server. Rewriting it as built rather than giving it an owner is the overseer's founder-delegated answer at this wrap's halt, 2026-10-26, consistent with the founder's own ruling on fork 1, which keeps the collision check unchanged.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the observed server pid and the verdict recorded as built
**Section:** §Authentication & Authorization (authentication-authorization.md: the server verification row's Stack reference; the new `values · auth` group's server-pid, verdict, exiting-wrapper and macOS items) · §Bootstrap Phases (bootstrap-phases.md: the `viola-channel` client bullet)
**Change:**
- Server pid (new): `Client::server_pid` / `TokioClient::server_pid` (`server_pid_of`, interprocess 2.4.4 `StreamCommon` `peer_creds().pid()`); `SERVER_PID_REPORTED`, a `const bool` re-exported by `viola-channel`, false on macOS; the read writes and logs nothing.
- Verdict (new): the closed `viola_state::liveness::ServerVerdict { Matches, Mismatch, Unverifiable }` from the pure `server_verdict` and the joined `verify_server` (the start time through `process_start_time`), comparing the peer pid and its start time with `pid` and `started_at` taken with the `endpoint` from one `read_snapshot` call — the CLI verbs and `viola mcp` through `live_endpoint` (now `LiveEndpoint { endpoint, pid, started_at }`, its liveness pre-check first), `viola hook` through its shared `connect`. Only `Matches` writes a frame; no peer pid reported is `Matches` with no comparison.
- Exiting wrapper's Windows pipe (new): passes by design; whether sysinfo still lists a terminated process whose pipe handle is open, and SessionEnd's outcome there, is a CARRY owed to "Windows-only live measurements".
- macOS cases (new): the socket directory loosened to 0755 giving exit 21 and the client euid check are a CARRY owed to "Unix endpoint and home hardening"; the server-identity negatives compile out on macOS.
- Stack reference: adds the peer-pid API and `viola-state` liveness (`live_endpoint`, `server_verdict`, `verify_server`), called by the root bin's `cmd::client::connect` and `cmd::hook` and by `viola-mcp`'s `frame`.
- Bootstrap: was the client's server verification against a snapshot read after strict-modes; now `viola-channel` gives the SQOS open and the peer pid, the verdict lives in `viola-state` liveness, built on Linux and Windows, and the strict-modes read is owed to "Home and code-bearing file integrity".
**Why:** the report's Symbols as built; the two CARRYs are re-pinned by the P5 review card's word (delegate the Viola overseer, founder-delegated).
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the snapshot readers are three, and stay on read_snapshot
**Section:** §Input Validation (input-validation.md: the Own state files on read row)
**Change:**
- The other snapshot readers are three: `src/cmd/hook.rs` (one read shared by the event and dialog paths), `src/cmd/hook/statusline.rs` and `run`'s collision check in `src/cmd/run.rs` (was five, `src/cmd/client.rs` and `src/cmd/hook.rs` twice among them, per 2026-10-10-statusline-pass-through — revive: the session id ratified, its arguments, its log before its check, the fifth snapshot reader; its other claims stand). The rule: `read_snapshot(` under `src/`, outside test modules. `viola_state::liveness::live_endpoint`, the CLI frame verbs' and `viola mcp`'s one read, also calls `read_snapshot`.
- Every snapshot reader stays on `read_snapshot` (option 1); their move onto `read_snapshot_or_replay` (`v1-41`, test-plan §6 E5 steps 1–3) is a CARRY owed to the route entry "Home and code-bearing file integrity".
**Why:** measured at the chunk: `src/cmd/client.rs` holds no read since `live_endpoint` took it, and the hook's two reads joined into one. Option 1 is the founder's own ruling on fork 1, given by dialog at his wake on 2026-10-26 and relayed by the overseer; it replaces the delegate's provisional answer.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the frame ban's exceptions and the threat narrowed to the strict-modes half
**Section:** §Security Anti-Patterns (security-anti-patterns.md: the frame-before-verification ban's dated exceptions; security-plan.md: the Authentication ban) · §Threat Model Summary (threat-model-summary.md: the IPC endpoint boundary) · §Authentication & Authorization (security-plan.md: the server verification summary)
**Change:**
- The detail ban: the dated exceptions hold only the strict-modes half (reference values from a `snapshot.json` read without the check), owed to "Home and code-bearing file integrity"; on Linux and Windows each excepted frame verifies the serving process first (macOS the recorded gap), the CLI and MCP frames after the liveness pre-check and the server check (was every exception after the liveness-only pre-check, until both Epoch 6 entries landed).
- The master ban: the excepted frames verify the server against a snapshot read without strict-modes, owed to "Home and code-bearing file integrity"; every client's euid check and macOS's pid and start-time check are owed to "Unix endpoint and home hardening" (was the exceptions standing until both Epoch 6 entries landed).
- The summary: the client verifies against its one snapshot read, on Linux and Windows by the peer pid (Windows over the SQOS open) and its start time, a mismatch or an unmakeable check sending no frame; the strict-modes gap and the euid / macOS owners named as in the ban (was Windows the serving pid, Unix euid and pid, macOS euid and directory, the gaps until their two entries).
- Threat model, new: as built, on Linux and Windows every frame-writing client compares the peer's pid and start time with the snapshot's before its first frame and sends none on a mismatch or an unmakeable check; the snapshot read without strict-modes and macOS stay open.
**Why:** the report's Retired line; the no-borrowing rule stands in both bans.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-explicit-session-links — the served `link` / `unlink` methods behind the IPC controls
**Section:** Authentication & Authorization (authentication-authorization.md: RBAC / permissions); Input Validation (input-validation.md: Channel frames); Threat Model Summary (threat-model-summary.md: the Local IPC vector)
**Change:**
- RBAC / permissions: the full surface a caller passing the OS check gets is named — the nine served methods (`send`, `wait`, `last`, `hook.dialog`, `answer`, `pause`, `release`, `link`, `unlink`) and the `hook.event` notification, each behind the IPC rows and its own params check (was "the full surface", unnamed).
- Channel frames: the wrapper's `link` / `unlink` params rule — `driver` and `driven` are strings that pass `ViolaName::try_new` (one shared check); a missing, non-string or invalid name, a `driven` other than the serving wrapper's own name and a `driver` equal to `driven` are each `-32602` `"invalid params"` with `data: null`, no line written; neither method reads `from`, so a `from` of any shape changes nothing.
- Local IPC vector: whoever can connect can also add or remove a driver link, whose effect is a `link` / `unlink` line in `events.ndjson` and the snapshot's links only; neither method reads the self-reported `from`.
**Why:** the chunk's wrapper serves `link` / `unlink` (both answered `-32601` before), routed to its `LinkSet`. Serving them is a boundary widening, a new crossing of the hardened IPC boundary, that the founder answered himself at this chunk's P4 fork (inputs#I3, live by dialog in the overseer session, 2026-10-26T14:41Z, relayed verbatim by the operator); the P2 halt's confirm (inputs#I6) is the overseer's founder-delegated word, PROVISIONAL until the founder's own word at the epoch boundary, and the ratification stays his. This entry is the Security Decisions Log record the Universal ban asks for a new channel method: `link` / `unlink` sit under the listener, peer and frame-cap controls, their names under `ViolaName::try_new` with the own-name and self-pair `-32602`; their verbs' instance check, liveness and server check are the entries below.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the collision check's code citation re-pointed
**Section:** Authentication & Authorization (authentication-authorization.md: IPC client-side server verification (anti-impersonation), the collision check)
**Change:**
- `run`'s collision check cites `src/cmd/run.rs:383` (was `src/cmd/run.rs:379`).
**Why:** the chunk's additions to `src/cmd/run.rs` (the `Methods` link set and its `link` / `unlink` routing) moved the check; the wrap's citation sweep re-pointed the one citation, digits only.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — `viola link` / `viola unlink`'s instance check before any log
**Section:** Authentication & Authorization (authentication-authorization.md: `~/.viola/` access control, the strict-modes entry points); Input Validation (input-validation.md: CLI arguments / stdin)
**Change:**
- The strict-modes entry-point list gains `link` / `unlink`: `viola_state::strict::check_instance` on the absolute home and `instances/<driven>` before `client::start`, so before any role file or log opens and before any snapshot value is read. A refusal exits 21 — the human form the fixed instance-state `unable:` / `hint:` pair on stderr through `human::refuse`, `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty — opens no log, writes no line and leaves the directory's mode as found; it is none of the dated interim gaps; proven by `cfg(unix)` cases only.
- CLI arguments: `<driver>` and `<driven>` through `ViolaName::try_new` (the verbs' shared name parser) and `--json`; the verbs' instance is the driven one, `instances/<driven>` joined only from that parsed name and checked by `check_instance` before any snapshot read, so neither verb is among the row's interim gaps. A `driver` equal to `driven` is not refused client-side: the wrapper refuses it (`-32602`).
**Why:** the form is the founder's own answer at this chunk's P4 fork (inputs#I3, live by dialog, relayed verbatim by the operator; the P2 halt's confirm inputs#I6 the overseer's founder-delegated word, PROVISIONAL): the instance check before any log or snapshot read, no dated gap. Exit 21, as `list` / `plugin install` and not `revive`'s exit 1, is the founder's option text (card lean 5). The self-pair refused by the wrapper only is the operator's answer of chunk 2026-10-10-session-links (`I2-relay-2`). A trap for later verbs: opening a role file narrows an existing home to 0700, so a check placed after `client::start` passes a 0770 home; `link_verbs_refuse_a_widened_instance_before_any_log` reads the mode after the refusal.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — `viola link` / `viola unlink` take no dated server-verification gap
**Section:** Authentication & Authorization (security-plan.md: IPC client-side server verification; authentication-authorization.md: the new row IPC client-side server verification, `viola link` / `viola unlink`, and the CLI refusal row); Bootstrap phases (bootstrap-phases.md: auth-scaffolding-baseline); Security Anti-Patterns › Authentication (security-anti-patterns.md: the frame ban); Threat Model Summary (threat-model-summary.md: the Local IPC vector)
**Change:**
- The snapshot read without the strict-modes check, the dated gap owed to "Home and code-bearing file integrity", now excepts CLI `link` / `unlink`, which read it only after the strict-modes check and take no gap (the master, threat-model-summary; bootstrap-phases: owed "for every client but CLI `link` / `unlink`").
- New row: both verbs run `check_instance`, then the liveness-only pre-check (`live_endpoint`; none is exit 21 with the not-running hint), then on Linux and Windows the server check through `cmd::client::connect` before their one frame (macOS the recorded gap); their reference values come from a snapshot whose instance passed the check in the same process, so they borrow none of the eight dated gaps. Stack reference `src/cmd/link.rs` (`link_call`, shared by `src/cmd/unlink.rs`).
- The CLI frame verbs are eight, `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink` (was six), in the CLI refusal row, its stack reference `src/cmd/{send,wait,last,answer,pause,release,link,unlink}.rs` and threat-model-summary's frame-writing clients; `link` / `unlink`'s strict-modes refusal (exit 21) comes before their liveness and server checks.
- Frame ban: `link`/`unlink` join its frame list and meet it in full; only every client's euid check and macOS's pid and start-time check stay owed to "Unix endpoint and home hardening"; they borrow no dated exception.
**Why:** `link_call` runs `check_instance` before `client::start`, then `live_endpoint`, then one request through `cmd::client::connect` (its callers 6 → 8). The no-gap form is the founder's own answer at this chunk's P4 fork (inputs#I3, live by dialog, relayed verbatim by the operator), under the P2 halt's founder-delegated PROVISIONAL confirm (inputs#I6).
**Kept:** the eight dated gaps stand for every other frame-writing client, each still owed to "Home and code-bearing file integrity".
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the tests holding the link verbs' controls
**Section:** Authentication & Authorization (authentication-authorization.md: the new row `viola link` / `viola unlink`, the tests holding their controls)
**Change:**
- New row: `channel_link_names_are_validated_by_the_wrapper` sends raw bad-name, foreign-`driven` and self-pair `link` / `unlink` frames past every CLI check and reads `-32602` with no line written; `link_verbs_refuse_a_widened_instance_before_any_log` (Unix only) reads a group-writable driven instance refused before any log, its mode as found; the server-identity table's `link` / `unlink` rows read no frame and exit 21 in both modes against an impostor server on Linux and Windows, compiled out on macOS with the file. Stack reference `tests/channel_links.rs`, `tests/cli_links.rs`, `tests/security_negatives_server_identity.rs`.
**Why:** the report's security outcomes, each met: the wrapper's name negatives (14 passed), the impostor-server tables (25 passed, CI ubuntu and windows witnesses), the widened instance on ubuntu and macOS, its remove-the-guard control 1 of 1 red when the check is moved after `client::start`.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link lines the served methods write, and the names they keep out
**Section:** Data Protection (data-protection.md: two new Logs groups); Input Validation (input-validation.md: Own state files on read)
**Change:**
- New group, `events.ndjson`'s link lines: the served `link` appends one `link` line through `LinkSet::record`, as an implicit link does, and the served `unlink` one `unlink` line through `LinkSet::remove`, each with `data: {driver, driven}` — the two instance names only, no user content; a held pair's second `link` and an absent pair's `unlink` append nothing; the replies carry only `{since, already_linked}` (`link`) and `{}` (`unlink`).
- New group, the wrapper's process log and the verbs' output: the methods open no span and write no process-log line of their own; the `channel-request` / `channel-response` pair carries `method:"link"|"unlink"` and no name; `viola link` / `viola unlink` print only their own parsed `ViolaName` arguments, never a name read back from the reply or the log.
- Own state files: a link line's driver comes from a `from` or from a served method's `params.driver`, each having passed `ViolaName::try_new` (was from a `from` only); the `LinkSet` writes at its restore, its record of an implicit link or of a served `link`, and its removal for a served `unlink` (was its restore and its record of an implicit link).
**Why:** `LinkSet::remove` has its first product caller, the `unlink` method, so a product path now appends an `unlink` event line, the first; `link_verbs_log_the_channel_pair_and_no_names` asserts no driver name in the wrapper's process log.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link methods' and verbs' error answers
**Section:** Error Handling (error-handling.md)
**Change:**
- New item: the wrapper's `link` / `unlink` answer a failed `record` / `remove` as `-32603` with the fixed message `internal error` and `data: null`. At `viola link` / `viola unlink` a wrapper error reply (a `-32602` included) is a wrapper fault, exit 20, and a `Refused` reply or an `ok` of a shape other than the verb's own (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object) is an internal error, exit 1.
**Why:** a `record` / `remove` error is `ProtocolError::Internal`; the verb reads its reply through `client::answer_of`, a wrapper error through `client::fault`.
**Kept:** the verbs' exit-21 outputs (the strict-modes refusal and the server mismatch) stay recorded with their checks in Authentication & Authorization, not repeated here.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the dated strict-modes gaps retired: every client checks the instance before the snapshot
**Section:** §Authentication & Authorization (authentication-authorization.md: the IPC client-side server verification row and its stack reference; security-plan.md: the server verification summary) · §Bootstrap phases (bootstrap-phases.md: the `viola-channel` client) · §Threat Model Summary (threat-model-summary.md: the Local IPC vector)
**Change:**
- The server verification row: was six interim gaps here (the first and the fourth to eighth dated gaps: `hook.event`, CLI `send`, `wait` / `last`, `answer` with `hook.dialog`, `pause` / `release`, `viola mcp`'s four tools), each reading the snapshot without the strict-modes check until "Home and code-bearing file integrity", per 2026-10-26-server-verification-before-any-frame — the dated gaps keep only their strict-modes half; now **Strict-modes check first, no dated gap**: every client runs the instance's strict-modes check before it reads the snapshot, then (CLI and MCP) the liveness-only pre-check, then on Linux and Windows the server check (macOS the recorded gap). The residuals stay closed on Linux and Windows and open on macOS.
- New: on a strict-modes refusal, made before the liveness and server checks, no frame and no line is written, no `strict-modes-failed` line included; the CLI frame verbs exit 21 with the instance-state `unable:` / `hint:` pair (under `--json` the `instance-unreachable` document, `detail: null`); `viola hook` exits 0 with no body; MCP returns `instance-unreachable` with `detail: null`.
- The stack reference adds `viola-state` `strict::check_instance`, called by `cmd::client::refuse_untrusted`, `cmd::hook`'s `trusted` and `viola-mcp`'s `frame`.
- The `viola-channel` client, the threat model and the master summary: was the snapshot read after the check owed to that entry for every client but CLI `link` / `unlink`; now every client reads the snapshot only after `check_instance`, a refusal sending no frame and writing no line; the client euid check and macOS stay owed to "Unix endpoint and home hardening".
**Why:** the report's Retired line: the eight founder-ratified dated gaps, each "until" this entry, now each run the strict-modes check before the first read or open. A refused check writing no line, as `link` / `unlink`'s, is the founder's own answer to P4 fork Q2 (`inputs#I3`, live by dialog, relayed by the overseer). The companion rewrites of the retired lines were confirmed at the P2 halt by the overseer, founder-delegated (`inputs#I6`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the six CLI frame verbs share `link` / `unlink`'s instance check
**Section:** §Authentication & Authorization (authentication-authorization.md: the strict-modes entry points' `link` / `unlink` clause and gap list; the `viola link` / `viola unlink` row; the CLI refusal row) · §Input Validation (input-validation.md: CLI arguments / stdin) · §Error Handling (error-handling.md: a new item)
**Change:**
- `client::refuse_untrusted` runs `check_instance` on the home and `instances/<name>` first in `send` (after its text read), `wait`, `last`, `answer`, `pause` and `release`, and in `link` / `unlink` (was `link_call`'s own call), each before `client::start`, so before any role file, log or snapshot value.
- A refusal exits 21: the instance pair (`human::instance_state_pair`) through `human::refuse`, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`; nothing opened under the home, no line. Proven for `link` / `unlink` by `cfg(unix)` cases only, for the six frame verbs on all three OSes.
- The fourth to seventh dated gaps (`send`, `wait` / `last`, `answer`, `pause` / `release` reading the snapshot without the check) leave the strict-modes entry points and the CLI arguments row, per 2026-10-04-confirmed-send-with-cl-1-records — the fourth dated gap: viola send before server verification (F3), 2026-10-04-wait-and-last — the fifth dated gap: CLI wait / last after the liveness-only pre-check, 2026-10-04-dialog-answers-by-dialog-id — the sixth dated gap: answer and hook.dialog frames, and their validation, and 2026-10-04-the-wheel — the seventh dated gap: pause and release frames, and their validation (their other claims stand); the row now holds a **Frame verbs** clause: the check, then `live_endpoint`, then on Linux and Windows the server check.
- The CLI refusal row: was only `link` / `unlink`'s strict-modes refusal coming before the liveness and server checks; now every frame verb's; its stack reference adds `refuse_untrusted`.
- The link row: was "so they borrow none of the eight dated gaps"; now "the order every CLI frame verb takes"; its stack reference adds `src/cmd/client.rs`.
**Why:** the report's Retired line (gaps 4 to 7). The six verbs printing `link`'s instance pair is the founder's own answer to P4 fork Q3, and no line on a refused check his answer to Q2 (`inputs#I3`, relayed by the overseer).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — `viola mcp`'s frame tools check the instance before `live_endpoint`
**Section:** §Authentication & Authorization (authentication-authorization.md: the server verification row's MCP clause; the strict-modes entry points' `mcp` clause and gap list) · §Input Validation (input-validation.md: CLI arguments / stdin) · §Error Handling (error-handling.md: a new item) · §Threat Model Summary (threat-model-summary.md: the MCP tools vector)
**Change:**
- Was the eighth dated gap: the `send` / `wait` / `last` / `answer` tools read `snapshot.json`'s `endpoint` through `live_endpoint` without the check, per 2026-10-17-mcp-server-for-drivers — the eighth dated gap: MCP frame tools after the liveness-only pre-check (its start check through `check_path` stands); now each runs `check_instance` on the home and the instance directory before `live_endpoint`.
- A refused check, like a replayed or absent snapshot, sends no frame and returns `isError` `instance-unreachable` with `detail: null`, text `refused: instance-unreachable`, and no log line.
- The MCP trust boundary gains the check, ahead of the liveness pre-check and the server check.
**Why:** the report's Retired line (gap 8) and its Symbols (`tools.rs` `frame`); no line on a refused check is the founder's own answer to P4 fork Q2 (`inputs#I3`, relayed by the overseer).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the server check's reference values and the collision check read through the replay
**Section:** §Authentication & Authorization (authentication-authorization.md: the IPC client-side server verification row's Collision check and stack reference; the verdict row)
**Change:**
- The verdict: was the reference values taken with the `endpoint` from one `read_snapshot` call; now from one `read_snapshot_or_replay` call made after the instance's strict-modes check, only a parsed snapshot giving them: a replayed or absent one is no `LiveEndpoint` for the CLI frame verbs and `viola mcp`, and `NotSent::Unreachable`, failing open, for `viola hook`'s `connect`.
- The collision check: was reading the snapshot through `read_snapshot`, per 2026-10-26-server-verification-before-any-frame — run's collision check: the server check retired as moot (its moot server check stands); now it takes the caller's one read through `read_snapshot_or_replay` (`run` after its home-tree check; `revive`'s preflight reads once and hands that read to both its collision check and its recorded-directory reading) and judges a parsed snapshot by `pid` + OS start time and the heartbeat, a replayed one by the beat alone (a `beat_age` within `STALE_AFTER` is `already-live`, else free), an error or an absent snapshot free. Its citation: was `src/cmd/run.rs:383`; now `src/cmd/run.rs:418`.
- The stack reference adds `replay::read_snapshot_or_replay`, read by `live_endpoint` and `cmd::hook`'s `connect`.
**Why:** the founder's own answer to P4 fork Q1 (`inputs#I3`, live by dialog, relayed by the overseer), his boundary-widening ratification, under the overseer's founder-delegated PROVISIONAL card confirm at the P2 halt (`inputs#I6`). One read per revive keeps one replay and one `state-recovered` line (the report's Deviation 4); the old citation no longer named the check.
**Supersedes:** 2026-10-26-explicit-session-links — the collision check's code citation re-pointed
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — `check_instance`'s set and the read rule as built
**Section:** §Authentication & Authorization (authentication-authorization.md: `~/.viola/` access control, the strict-modes check's `revive` clause, the Unix and Windows read rules; security-plan.md: `~/.viola/` access control) · §Bootstrap phases (bootstrap-phases.md: `viola-state`)
**Change:**
- `check_instance`: was reading the home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson` in that order; now it holds the home, the instance directory and its `diagnostics/` to the read rule, `instances/` and `snapshot.json` to the write rule, and `events.ndjson` to the write rule on Unix and the read rule on Windows, each that exists, a folder before what it holds.
- The Unix read rule: was the home and `instances/<name>/`; now also `diagnostics/` (`strict::unix_read_verdict`). The Windows read rule names `READ_RIGHTS` = `0x9000_0001` and `strict::windows_read_verdict`.
- A refusal names its property: `Refused` holds `Readable` ("a trusted state path is readable by another user") and `Symlink` ("a trusted state path is a symbolic link"), each fixed, neither borrowing `Writable`.
- `viola-state` bootstrap: was the Unix strict-modes check, its read rule on the home and `instances/<name>/`; now `viola_state::strict`, the read rule on both OSes over the home, the instance and its `diagnostics/`, with `check_home_tree`, `check_diagnostics`, `check_present` and the `Readable` / `Symlink` refusals.
- The master's access control: was refusing a trusted path another user can write, or a home another user can read, with its dated interim gaps; now also a home, instance directory or `diagnostics/` another user can read (Unix any group or other bit; Windows a read-right allow ACE to another SID, inherit-only included) or a symlinked `diagnostics/` folder or file, before the first read or open under the home.
**Why:** the report's Symbols (`strict.rs`) and its Count 3, the read rule built; the two variants give fixed messages that name the refused property (Deviation 2).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the Windows legs measured: the protected DACL, a plain home refused, an unstattable home
**Section:** §Authentication & Authorization (authentication-authorization.md: `~/.viola/` access control, the `revive` clause's proof and the outcomes paragraph) · §Data Protection (data-protection.md: Files (`~/.viola/`))
**Change:**
- `revive`'s check: was proven by `cfg(unix)` cases only, a Windows case widening an instance tree's DACL owed to this entry; now also by `cfg(windows)` cases (an instance tree granted Users write refused beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read refused, a plain home made before viola refused `Writable`), green on `windows-2025`.
- The outcomes: was "The outcomes match Unix"; now they match but one: an unstattable home (a path under a regular file) is a strict-modes refusal on Unix (`verify` exit 1 with the home pair, the frame verbs 21 with the instance pair), while on Windows such a path stats `NotFound`, passes as absent, and the catch-site line stands.
- The witnesses named: `tests/security_negatives_strict_modes.rs` (the six frame verbs, a start under umask 000, the Windows protected DACL read back, a read-granted pre-existing home refusing `run`, single writers with the wrapper stopped) and `tests/chaos_snapshot_replay.rs` (E5 steps 1–3), green on all three OSes.
- Data protection, new: the protected owner-only DACL of a home viola makes outside `%USERPROFILE%` reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`, canonical SDDL spelling the runner's user (the local Administrator, RID 500) `LA`; a plain home made before viola (`create_dir_all`) is refused `Writable`, the refused entry an inherited Users (`S-1-5-32-545`) write ACE.
**Why:** measured on `windows-2025` at the chunk's CI (the report's Cross-project claims and Spec claims disproved). The plain home's SDDL itself was not recorded; the refusal and the refused ACE are asserted instead (Deviation 6). A path under a regular file needs a per-OS oracle (Deviations 8 and 10).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — `viola list`'s home-check inconsistency, a CARRY on "Unix endpoint and home hardening"
**Section:** §Authentication & Authorization (authentication-authorization.md: `~/.viola/` access control, the strict-modes entry points' `list` clause)
**Change:**
- New: `check_path` holds the home to the write rule while `check_instance` holds it to the read rule, so a 0750 home passes a board with no instance and is refused, naming the instance (`ScanRefused::Instance`), once it holds one; the inconsistency is a CARRY owed to the route entry "Unix endpoint and home hardening".
**Why:** the report's Count 14 (`board.rs` unchanged; the wrap directive, `inputs#I5`, asks an owner). The owner is the overseer's answer to the P2 halt's Q3, founder-delegated (`inputs#I6`): the home's checks belong to that entry. P5 pins the CARRY.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — `viola hook`'s event and dialog paths check the instance before obs init
**Section:** §Authentication & Authorization (authentication-authorization.md: the strict-modes entry points' `hook <event>` clause and gap list) · §Input Validation (input-validation.md: Hook stdin; CLI arguments / stdin, Home path) · §Error Handling (error-handling.md: the hook fail-open item) · §Threat Model Summary (threat-model-summary.md: the Hook stdin vector)
**Change:**
- Was the first dated gap: `viola hook <event>` read `snapshot.json`'s `endpoint` without the check and its nine `HookEvent` arms neither canonicalised `VIOLA_DIR` nor ran it (Decisions Log `2026-09-28`), per 2026-09-27-hooks-to-normalised-events — `hook.event` served; the hook's interim gap, ratified by the founder; hook stdin row (its other claims stand); now the event and dialog paths canonicalise the instance directory and run `check_instance` (`canonical`, then `trusted`, shared in `src/cmd/hook.rs` with `hook statusline`) before the diagnostics level and obs init.
- A refused or absent instance exits 0 at once: no `config.json` read, no role file, no stdin read, no line, no frame, no SessionEnd append. The two `--capture` arms stay ahead of the check.
- On every hook path a `VIOLA_DIR` naming an instance directory that does not exist canonicalises to no instance, so the hook writes nothing.
- The error-handling item: was "strict-modes is owed to" this entry; now the check as above. The threat model: every hook path runs it before anything under the home is read or opened.
**Why:** the report's Retired line (gap 1), its Symbols (`hook.rs`) and its Count 23; no line on a refused check is the founder's own answer to P4 fork Q2 (`inputs#I3`, relayed by the overseer).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — `ledger/stamps.json` checked before `ledger/` is made, and `verify`'s home tree before its log
**Section:** §Authentication & Authorization (authentication-authorization.md: the strict-modes entry points' `update_stamps` clause and gap list; the failure outcomes) · §Input Validation (input-validation.md: CLI arguments / stdin, Home path; Own state files on read, Integrity; security-plan.md: the CLI arguments line) · §Error Handling (error-handling.md: two new items) · §Security Anti-Patterns (security-anti-patterns.md: a new ban)
**Change:**
- Was the second dated gap: `update_stamps` read `ledger/stamps.json` without the check (Decisions Log `2026-09-28`, stamps read), per 2026-09-28-capability-ledger-and-viola-verify — `run`'s and `verify`'s stamps reads before strict-modes, a ratified interim gap until Epoch 6; now it runs `strict::check_stamps` before `create_private_dir(ledger/)` and again under its lock before the capped read; a refusal, `StampsError::StrictModes` (its other error `StampsError::State`), reads and writes nothing.
- `viola verify` runs `check_home_tree` on the home, `ledger/`, `bin/` and `plugin/` before its `cli` log; that refusal, or `StampsError::StrictModes`, exits 1 with the home pair, no step line, no summary and no line. On failure `run`, `verify` and `mcp` exit 1 (was `run` and `mcp`).
- `StampsError`'s fixed message: "the capability stamps failed the strict-modes check"; a write failure its own fixed message.
- New ban: never let `viola verify` read or write `ledger/stamps.json` before the check passes, nor run the check after a create, a mode set or a log open that narrows the path it checks.
- The master's CLI arguments line: was "except the ratified dated interim gaps until the Epoch 6 entries land"; now `ledger/stamps.json` is checked before `ledger/` is created and again under its lock, a refusal reading and writing nothing.
**Why:** the report's Retired line (gap 2) and its Deviation 1: checked under the lock alone, the check comes after the create, which narrows a widened `ledger/` to 0700 first, so it is made before the create too, by the plan's order rule.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the ConPTY companions' owner/DACL check before the held open
**Section:** §Authentication & Authorization (authentication-authorization.md: the strict-modes entry points' ConPTY clause and gap list) · §Data Protection (data-protection.md: Code-bearing artefacts) · §Input Validation (input-validation.md: DLL search order + sideloaded ConPTY companions; security-plan.md: its line) · §Error Handling (error-handling.md: a new item) · §Threat Model Summary (threat-model-summary.md: Filesystem state)
**Change:**
- Was the third dated gap: on Windows x64 `run` loaded `bin/<version>-<hash>/conpty/{OpenConsole.exe,conpty.dll}` under the held handle and the re-hash alone, without the owner/DACL check (Decisions Log `2026-09-29`), per 2026-09-29-sideloaded-conpty — vendored Microsoft ConPTY binaries and the third interim gap (founder live) (its other claims stand); now `pin_companions` runs `strict::check_present` on `bin/<key>/conpty/` and each companion that exists before the `FILE_SHARE_READ`-only held open. A refusal is `PinError::StrictModes` (fixed message "the pinned copy failed the strict-modes check"): nothing written, loaded or launched from that folder, the child on the inbox ConPTY, its exit unchanged.
- `process-start.sideload_fallback`: was four words; now five, `hash-mismatch` · `strict-modes-failed` · `unreadable` · `load-failed` · `not-built`; `run` maps `PinError::StrictModes` to `strict-modes-failed`.
- New: `run`'s start pins the bytes it has already checked (`pin::pin_bytes`, under the key `pin::pin_key`, `<VERSION>-<16 hex>`); `pin_exe`, which `verify` calls, delegates to it; the `pin_exe` first-start race and reproducible builds are owed to "Pinned copy race and reproducible builds".
**Why:** the report's Retired line (gap 3) and its Counts 7 and 18; one `strict-modes-failed` in the Windows leg's kept homes, the companion test's own. The pin race's new owner is the founder's own answer to P4 fork Q4 (`inputs#I3`) and his yes on the P5 card's Beyond-the-plan row 1 (`inputs#I4`), each relayed by the overseer. `pin_exe` keeps its signature for `verify` (Deviation 3).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — `marketplace/` off the integrity set, no owner
**Section:** §Authentication & Authorization (authentication-authorization.md: the strict-modes entry points' `viola plugin install` clause) · §Threat Model Summary (threat-model-summary.md: Filesystem state)
**Change:**
- The installed `command` gets no re-hash: was its integrity owed to the route entry "Home and code-bearing file integrity"; now `marketplace/` is off the integrity set, with no owner. The threat model's integrity line says so too.
**Why:** CARRY 17 closed by ruling: the founder's own yes on the P5 card's Beyond-the-plan row 2 (`inputs#I4`, relayed by the overseer), his Epoch 5 batch (2026-10-20) having made `marketplace/` off the integrity set his word.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the home tree checked at `run` and `revive`'s start, before the wrapper log
**Section:** §Authentication & Authorization (authentication-authorization.md: a new row, `~/.viola/` access control, the home tree at start) · §Input Validation (input-validation.md: CLI arguments / stdin, the `revive` arguments) · §Error Handling (error-handling.md: a new item) · §Threat Model Summary (threat-model-summary.md: CLI arguments; Filesystem state) · security-plan.md (the Filesystem state vector)
**Change:**
- New row: `viola_state::strict::check_home_tree(home, name, key)` checks the home (read rule), with a name `check_instance`'s set, then `ledger/`, `bin/` and `plugin/`, then with a key `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/`, its `hooks/` and its `.claude-plugin/` (write rule), each that exists, and returns `TreeRefused { HomeLevel, InstanceLevel }`.
- `run` and `revive`'s start run it through `checked_exe` (the exe's bytes read once, its `pin_key` taken) before the wrapper log opens; a refusal exits 1 with the home or the instance pair (`refuse_tree`; `revive --json` the `strict-modes-failed` document), no log, no line and no child.
- The threat model: was "Nothing checks integrity beyond the content hash in the folder name"; now beyond it the start holds the home tree to the strict-modes check (Unix owner and mode, Windows owner and DACL) before the wrapper log, and `pin_companions` checks the `conpty/` folder; the CLI boundary: `run`, `revive`'s start and `verify` hold the home tree, the frame verbs the instance, so a tree chosen by `--home` or `VIOLA_DIR` that another user can write or read is refused.
- The master's Filesystem state vector: was "nothing checks integrity beyond the content hash"; now the start holds the home tree, the pinned binary and the plugin folders to the strict-modes check before any log opens.
**Why:** the report's Symbols (`run.rs` `checked_exe` / `refuse_tree`, `revive.rs`) and its acceptance criterion: `run` refuses a 0770 home, a widened `instances/<name>`, `bin/` or pinned copy, exit 1 with its pair before any log.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the diagnostics opens refuse a symlink or a widened path
**Section:** §Bootstrap phases (bootstrap-phases.md: `viola-state`) · §Data Protection (data-protection.md: a new Logs item) · §Error Handling (error-handling.md: a new item)
**Change:**
- New: `open_role_file` and `write_detail` run `viola_state::strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file before `ensure_private_dir` narrows anything: a symlink is refused; a folder takes the read rule; a file the read rule on Unix and the write rule on Windows; an absent path passes.
- A refusal is a failed open: the role writer ends at `std::io::sink` and the detail line is dropped.
**Why:** the report's Symbols (`src/obs.rs`'s `check_diagnostics`, `strict.rs`'s `check_diagnostics`): the check comes before anything is narrowed.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the snapshot readers move onto `read_snapshot_or_replay`
**Section:** §Error Handling (error-handling.md: two new items) · §Input Validation (input-validation.md: Own state files on read; Statusline shell-out; security-plan.md: the Own state files line) · §Threat Model Summary (threat-model-summary.md: Filesystem state) · §Authentication & Authorization (security-plan.md: the server verification summary)
**Change:**
- The classified read and the replay: was two product callers (`viola revive`'s preflight, `viola list`'s `board::read`); now six, each after its entry point's strict-modes check: also `viola_state::liveness::live_endpoint` (the CLI frame verbs and `viola mcp`'s tools), hook `connect` (one read for the event and dialog paths), statusline `serve` and `run`'s collision check.
- No product caller under `src/` calls `read_snapshot` (the rule: `read_snapshot(` under `src/`, outside test modules); was the three other readers and `live_endpoint` staying on it, their move a CARRY owed to this entry. The integrity clause: read only after the check, at every entry point (was its dated exceptions).
- Over a replayed snapshot, which carries no `endpoint`, `pid`, `started_at` or `statusline_command`: `live_endpoint` gives no endpoint, so no frame; hook `connect` writes no frame and fails open as `channel-unreachable`, a SessionEnd appended through `append_session_end`; `hook statusline` runs and prints nothing, exit 0, empty stderr; the collision check judges by the heartbeat alone.
- The master's Own state files line and server verification summary: a replayed snapshot yields no frame and runs no command.
**Why:** the founder's own answer to P4 fork Q1 (`inputs#I3`, live by dialog 2026-10-26T18:11Z, relayed by the overseer), his ratification of the boundary widening: hook, MCP and CLI processes read `events.ndjson` on a new basis. The answer card's confirm at the P2 halt is the overseer's, founder-delegated, PROVISIONAL (`inputs#I6`); the ratification stays his `inputs#I3`. The report: `read_snapshot(` product callers 3 → 0, replay readers 2 → 6.
**Supersedes:** 2026-10-26-server-verification-before-any-frame — the snapshot readers are three, and stay on read_snapshot
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the frame ban met in full by every frame
**Section:** §Security Anti-Patterns (security-anti-patterns.md: the frame-before-verification ban; security-plan.md: the Authentication ban)
**Change:**
- The detail ban: was dated exceptions holding the strict-modes half for `hook.event`, `hook.dialog`, CLI `send`, `wait` / `last`, `answer`, `pause` / `release` and `viola mcp`'s tool frames, with `link` / `unlink` alone meeting it in full, per 2026-10-26-server-verification-before-any-frame — the frame ban's exceptions and the threat narrowed to the strict-modes half (its euid and macOS owners stand); now every frame meets the ban in full: the CLI frame verbs and `link` / `unlink` after `cmd::client::refuse_untrusted`'s `check_instance`, `viola hook`'s `hook.event` and `hook.dialog` after its instance check (`trusted`), `viola mcp`'s tool frames after `check_instance`, each verifying the server on Linux and Windows.
- The master ban: was the strict-modes exceptions owed to "Home and code-bearing file integrity" beside the euid and macOS gaps; now the dated gaps are only every client's euid check and macOS's pid and start-time check, owed to "Unix endpoint and home hardening"; no other frame or process may borrow them.
**Why:** the report's Retired line: the eight dated strict-modes gaps closed, so no frame takes its reference values from an unchecked snapshot.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-27-pinned-copy-race-and-reproducible-builds — the pinned `viola` placed by `place_private_new`, never replaced
**Section:** §Authentication & Authorization (authentication-authorization.md: `~/.viola/` access control, its Implementation and Stack reference) · §Data Protection (data-protection.md: Code-bearing artefacts, two new groups: the pinned copy's placement, a present pinned-copy name)
**Change:**
- `replace_private`'s mode clause: was "0600, or 0700 for the pinned `viola`"; now 0600 only. The pinned `viola` is never replaced: its absent copy goes through `viola_state::fs::place_private_new`, which sets the temp file's mode (0700) before any byte is written, `sync_all`s it, closes its write handle and places it with `persist_noclobber`, never over a present name.
- Stack reference: adds `into_temp_path` + `persist_noclobber` via `viola_state::fs::place_private_new` (the pinned `viola`) beside tempfile 3.27.0 `persist` via `replace_private`.
- New group, the placement: `pin_bytes` writes an absent `bin/<version>-<hash>/viola(.exe)` by a no-clobber placement, never a replace: a temp file beside it (`NamedTempFile::new_in`), its mode set before any byte, the bytes written, `sync_all`, the write handle closed (`into_temp_path`), then `persist_noclobber`, so the name never appears while a handle holds it open for write. `replace_private_shared` keeps only the plugin files and the ConPTY companions.
- New group, a present name: `place_private_new` never replaces one: `AlreadyExists` is `Placed::Present`, the target untouched and the temp file removed; any other failure is an error that leaves the target as it was, with no retry.
**Why:** `pin_bytes`' absent-copy arm moved from `replace_private_shared` to `place_private_new`, because tempfile's `persist` renames while the write handle is still open and `into_temp_path` closes it first. The chunk measured the mode set before any byte (0700 / 0600 on Unix) and a present name left byte-identical with no temp left. No dependency was added: tempfile 3.27.0 already holds both methods.
**Ref:** .andromeda/runs/2026-10-27T02-51-54-wrap/

## 2026-10-27-pinned-copy-race-and-reproducible-builds — the Windows plain home refused by its inherited write right, not a Users ACE
**Section:** §Authentication & Authorization (authentication-authorization.md: `~/.viola/` access control, the `revive` clause's `cfg(windows)` cases) · §Data Protection (data-protection.md: Files (`~/.viola/`), the strict-modes line)
**Change:**
- The refused entry: was "an inherited Users (`S-1-5-32-545`) write ACE", per 2026-10-26-home-and-code-bearing-file-integrity — the Windows legs measured: the protected DACL, a plain home refused, an unstattable home (its other claims stand); now "an inherited allow ACE that grants a write right to a SID the check does not trust (read by the right, whichever SID holds it)", measured at this chunk. The verdict `Writable` stands.
- The `revive` clause's case "a plain home made before viola refused `Writable`" now adds "(read by the inherited write right whichever SID holds it, with a viola-made-home control)".
**Why:** the Users-SID claim was disproved by measurement: on one `windows-2025` runner Users held read only and Authenticated Users held the inherited write. The product's `Writable` verdict was right, and only the test's oracle naming one SID failed, because the runners' inherited temp-dir ACLs differ by host. The case now reads the grant by its right, with a control that cannot pass vacuously. Folding this red into the chunk is delegate overseer1's ruling (founder-delegated). It stays PROVISIONAL and is owed the founder's own word.
**Kept:** the write rule's text, which gives `Users`, `Authenticated Users` and `Everyone` as examples of an untrusted SID.
**Ref:** .andromeda/runs/2026-10-27T02-51-54-wrap/

## 2026-10-27-pinned-copy-race-and-reproducible-builds — the first-start race on the pinned copy closed, a lost placement re-hashed
**Section:** §Data Protection (data-protection.md: Code-bearing artefacts, the `pin_bytes` line and a new group: a lost placement) · §Threat Model Summary (threat-model-summary.md: a new Filesystem-state vector)
**Change:**
- The `pin_bytes` line: was "the `pin_exe` first-start race and reproducible builds are owed to the route entry "Pinned copy race and reproducible builds"", per 2026-10-26-home-and-code-bearing-file-integrity — the ConPTY companions' owner/DACL check before the held open (its other claims stand); now "an absent copy is placed, never replaced, a copy another start placed first is re-hashed before reuse, and one tree's release builds can key two folders (each in the groups below)".
- New group, a lost placement: on `Placed::Present` (a concurrent first start placed the copy first) `pin_bytes` re-hashes the copy that stands. Equal bytes are reused. Different bytes are `PinError::HashMismatch`, and the file is left as found, never overwritten or deleted. A reused lost placement reads `ok` on `run.pin_copy` (`ok|pinned-hash-mismatch`).
- New vector: the pinned binary's first placement by concurrent first starts. It is placed once and never replaced. An exec of a copy that another start still holds open for write fails with `ETXTBSY` (raw 26 on Linux), so the placement closes its write handle before the name appears. 0 of 24 two-instance boots failed.
**Why:** the chunk read the race's errno on the product run: 4 of 24 two-instance boots failed at the base, each with raw 26, and 0 of 24 after. The single-instance control failed 0 of 24 on both sides. A closed-handle placement is what defeats `ETXTBSY`. The case where another start placed the copy first is green: equal bytes are reused, and different bytes give `HashMismatch` with the file byte-identical.
**Ref:** .andromeda/runs/2026-10-27T02-51-54-wrap/

## 2026-10-27-pinned-copy-race-and-reproducible-builds — one tree's release builds can key two pinned folders
**Section:** §Data Protection (data-protection.md: Code-bearing artefacts, a new group: one tree's release builds)
**Change:**
- New group: clean release builds of one tree can give two files whose loaded sections are equal, so one tree can key two `bin/<key>/` and `plugin/<key>/` folders. The reading stands at architecture §Established Decisions [Deployment / Distribution]. No cause is claimed, and acting on it is the founder's.
**Why:** measurement B ran six cold release builds of one tree and got two different files. Only three sections were unequal, none of them loaded (`.note.gnu.build-id`, `.symtab`, `.strtab`). Its cause is not explained. The operator's wrap directive (the overseer, founder-delegated, the founder asleep) records it as measured, with no cause claimed, as a founder item: on the handoff's FOR DISCUSSION list and as a CARRY on "Version done-check". It sits in this master as one sentence, and the section-level reading stays at architecture.
**Ref:** .andromeda/runs/2026-10-27T02-51-54-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — serde_json's `unbounded_depth` banned in `deny.toml`
**Section:** §Dependency Security (dependency-security.md: a new `values · deps` group; body: `deny.toml` additions, `[[bans.features]]`) · §Security Anti-Patterns (Input: the `unbounded_depth` ban)
**Change:**
- `[[bans.features]]` gains a row beside rmcp's: `serde_json` denies `unbounded_depth`, its reason that every parse of external bytes keeps serde_json's default recursion limit. The `scripts/deny-probes.sh` row `serde_json-unbounded-depth` proves the ban fires, and the test `serde_json_unbounded_depth_is_banned_in_deny_toml` reads it in `deny.toml`.
- The Input ban: was "NEVER enable serde_json's `unbounded_depth` feature."; now it adds "`deny.toml` bans it (§Dependency Security)".
**Why:** the chunk added the `deny.toml` table (`crate = "serde_json"`, `deny = ["unbounded_depth"]`) and its probe row, so a ban the plan only wrote is now enforced by the gate. No dependency was added or bumped, and `cargo deny check` stays clean.
**Kept:** the Channel frames line's "serde_json's default depth limit, never `unbounded_depth`" at §Input Validation, a true claim, stands as written.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the deny-probes verdict at 14/14, its counting rule named
**Section:** §Dependency Security (dependency-security.md: the arch-bans item, now in the `values · deps` group after the serde_json one)
**Change:**
- The proof clause: was "`13/13 banned, control clean`", per 2026-10-17-mcp-server-for-drivers — the tokio ban's tree form and the rmcp release-graph gate and per 2026-10-26-tokio-rustsec-2026-0386-lock-bump — viola-channel's Tokio client pin at tokio 1.53.3 (their other claims stand); now "`14/14 banned, control clean`, counted as the `probes=(` rows of `scripts/deny-probes.sh` plus the planted channel probe", as measured at the chunk's report. The rest of the item stands as written.
**Why:** the new `serde_json-unbounded-depth` probe row brings the `probes=(` rows to 13, and the planted channel probe `channel-tokio-edge` makes 14; the chunk's gate and pre-push runs read `deny-probes: 14/14 banned, control clean`. A count in a master carries the rule that produced it, so the rule now stands beside the number.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — `viola mcp`'s stdin capped at `MAX_FRAME` per line, an over-long line ending the session
**Section:** §Input Validation (input-validation.md: a new Boundary entity, MCP stdin; the `values · input` Constants list; body: a new MCP stdin line after the MCP tool inputs line, the Constants line) · §Security Anti-Patterns (Input: the bounded-reader ban; API: the request size limit)
**Change:**
- New boundary, MCP stdin (`viola mcp`'s transport, the driver's JSON-RPC lines): each line passes `viola_mcp::stdin::CappedLines` at `MAX_FRAME`, its `\n` counted, before rmcp reads it; a line of exactly the cap passes whole; a longer line releases no byte, is refused whole, never truncated or skipped, and the reader then reads as end of stream, so the session ends as a stdin close does; at the inner end a held unterminated line within the cap is released.
- On the cut: after the session ends `serve` writes one WARN `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` through `obs_event!`, holding no byte of the line, and returns exit 0 for an initialized session, `ServeError::Initialize` (exit 11) when the cut lands before `initialize` completes.
- How: `CappedLines` (crate-private), handed to rmcp 3.4.1 as `(CappedLines::new(stdin, MAX_FRAME), stdout)` through the `(reader, writer)` transport `transport-io` already enables, no rmcp feature added.
- Both Constants lists name `viola mcp`'s stdin lines among `MAX_FRAME`'s capped reads.
- Input ban adds: NEVER hand rmcp a `viola mcp` stdin reader that does not cap each line at `MAX_FRAME`, nor truncate or skip an over-long line instead of ending the session. API: was "`MAX_FRAME` on the channel"; now "on the channel and on each `viola mcp` stdin line".
**Why:** rmcp 3.4.1's stdio transport reads each line with an unbounded `read_until`, and `JsonRpcMessageCodec::new_with_max_length` bounds only the writer, so the bound sits on the reader viola hands rmcp, never on the codec. A truncated line could dispatch a padded valid request, and a skipped one would leave the peer's request unanswered with no signal; the channel's close-on-oversize rule is the precedent. The unbounded read was named this chunk's subject by overseer1's ruling under the founder's night word.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the vt100 passthrough after a caught feed panic, witnessed on the outer terminal
**Section:** §Input Validation (input-validation.md: the PTY output bytes (vt100) entity, a new Witness clause)
**Change:**
- New clause: after a caught feed panic `chaos_feed_panic_refuses_send_and_keeps_passthrough` reads the outer terminal: on Unix the child's bytes (`e4 b8 ad`) reach it, and on every OS none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` appears on it; the Windows byte reading is owed to "Windows-only live measurements".
**Why:** the chunk's test now asserts the passthrough on the outer terminal, green on ubuntu, macOS and windows-2025. ConPTY re-renders the child's output, and a wide character on a 1-column screen has no guaranteed rendering, so Windows reads the literal absence only. The byte reading is the P5 card's CARRY on "Windows-only live measurements", approved by delegate overseer1 (founder-delegated), PROVISIONAL until the founder's own word.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the bounded-reader census, the compare read bounded by its own length, the own-executable reads named
**Section:** §Input Validation (input-validation.md: the `values · input` Constants entity, three new clauses; body: the Constants line)
**Change:**
- Bounded by its own length, not `MAX_FRAME`: `replace_private_shared`'s fallback compare reads its target through `take(bytes.len() + 1)` (`holds_exactly`), never the whole file, its outcome a whole-file compare's for an equal, a longer, a shorter, a different and an absent target. The body's Constants line says so.
- Enumerated and censused: `tests/contract_bounded_readers.rs` (an E2E `contract_` binary) names each external boundary's bounded read and its witness in `BOUNDARIES`; its census `product_readers_are_each_bounded_or_named` requires every product reader call to be bounded in its statement or named in `EXCEPTIONS`, and a self-check reads a planted unbounded read as a hit.
- Not external input: viola's reads of its own executable for the pinned copy. The census's `EXCEPTIONS` rows name them, and the body's Constants list names them no external input.
**Why:** the compare read now stops one byte past the bytes it compares, with the same outcome as a whole-file compare. The census turns "every external reader capped" into a test over the product sources rather than a list kept by hand: with the compare-read change undone it reads red at that read. Hashing viola's own binary reads no external bytes, so those reads are named exceptions, not caps.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the MCP-over-stdio threat vector bounded at its stdin lines
**Section:** §Threat Model Summary (threat-model-summary.md: a new `values · boundary-mcp` group; body: the MCP over stdio vector line)
**Change:**
- New group, vector MCP over stdio, the driver's stdin lines: each stdin line reaches rmcp only through viola's `CappedLines` reader at `MAX_FRAME`, its `\n` counted: an over-long line from the driver is refused whole and ends the MCP session after the earlier lines are answered, never truncated, skipped or dispatched in part, with one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` WARN holding no byte of it.
- The body vector line adds, after the input check: each stdin line capped at `MAX_FRAME` before rmcp reads it, an over-long line refused whole and ending the session, never dispatched in part.
**Why:** the driver, an LLM agent, writes `viola mcp`'s stdin, and rmcp reads those lines unbounded. The chunk measured a line past `MAX_FRAME` ending the session after the earlier lines are answered: exit 0, stderr empty, stdout only the earlier frames, one `parse-rejected` line, and no byte of the line in any home-level file, green on all three CI OSes.
**Kept:** the rest of the vector stands: the driver answers the driven session's PermissionRequest outside viola, and leaving `release`, `pause`, `link` and `unlink` off the tool list is an affordance, not an enforcement.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/
