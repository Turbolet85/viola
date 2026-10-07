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
