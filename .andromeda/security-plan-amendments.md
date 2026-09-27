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
