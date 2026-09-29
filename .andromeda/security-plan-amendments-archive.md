# security-plan — archived amendment originals

Writer = wrap P7 only · read by NO loop skill · cold history, never cited for current truth; each run's originals under its own heading.

# Consolidated at the 2026-09-27-wrapper-channel wrap — 12 re-worded · 0 pruned

## 2026-09-24-three-os-ci-headless-harness-skeleton — pinned actions and toolchain source
**Section:** §Dependency Security (CI integration; Pinning) · §Threat Model Summary (Supply chain entry point)
**Change:**
- The SHA-pinned action set is now exactly what `ci.yml` uses:
  - `actions/checkout` v7.0.1 (with `persist-credentials: false`)
  - `Swatinem/rust-cache` v2.9.2
  - `taiki-e/install-action` v2.87.19
  - `actions/upload-artifact` v7.0.1
- `dtolnay/rust-toolchain` is replaced by a `rustup toolchain install` step reading `rust-toolchain.toml`.
- The toolchain is the exact 1.98.1 pin, and the workspace `rust-version` floor is 1.96.
- Event-payload values reach steps only through `env:`.

**Why:** the chunk shipped `ci.yml` and `rust-toolchain.toml` (report: Harness / gate surface, Schema / config; expected amendment 2). Sweep `dtolnay|@stable|Swatinem` over security-plan: lines 131, 320 and 330–331 were amended; line 548 (the mutable-ref ban) needs no change.

## 2026-09-24-three-os-ci-headless-harness-skeleton — rejected: interim `--home` and R8-strip Decisions-Log entries
**Section:** none (proposals rejected)
**Change:** none.
**Why:**
- The walking-skeleton `viola run` has two known gaps:
  - `--home` is not canonicalised or strict-modes-checked, and the Windows protected DACL is not set;
  - it spawns the child with the full inherited environment, so the R8 `CLAUDE*` strip is not applied yet.
- Both are sequencing deferrals (playbook rule 1), owned by markerless route entries rather than by body prose:
  - "Home and code-bearing file integrity" and "CLI machine contract — global --home" own the first. Both receive a `CARRY:` pin at this wrap's route-resolve.
  - "PTY wrapper on Windows" owns the second; that entry already names the CLAUDE* strip.
- The §Input Validation CLI row, §Secret Management and the §Anti-Patterns Data Protection ban stay as the target.

## 2026-09-24-supply-chain-and-workflow-gates — trust boundary, CI jobs, deny.toml additions, nightly.yml
**Section:** §Threat Model Summary → Supply chain (Trust boundary) · §Architecture Overview → CI/CD · §Dependency Security (`deny.toml` additions, CI integration)
**Change:**
- Trust boundary: now names `deny.toml` with its four families plus the sole-root `deny-sync.toml` tokio ban.
- CI/CD: adds `nightly.yml` and the ubuntu supply-chain job.
- `deny.toml` additions: the heading no longer says arch lists only licences and bans. The arch-bans bullet places the tokio ban in `deny-sync.toml` per sync crate as sole root, and records that `scripts/deny-probes.sh` proves every ban live.
- CI integration: the weekly advisory run is a separate workflow, `nightly.yml` (weekly `schedule` + `workflow_dispatch`, no cache), not a trigger on `ci.yml`.

**Why:** chunk 2026-09-24-supply-chain-and-workflow-gates. The fan-out had 0 proposals from this doc's detectors, which all held. Its return flagged these sites as restatements of the claims the pass retires, so the orchestrator raised them as routine cascade dependents. Sweep: see architecture-amendments.md, same entry heading. For this master, 5 sites were amended (:134, :161, :308, :314, :327). The pinned-action set was left unchanged, because no new action was added.

## 2026-09-24-diagnostics-plane — config.json diagnostics_level validation row, MAX_FRAME consumer
**Section:** §Input Validation (Configuration values row; Constants)
**Change:**
- The `config.json` row names the closed `diagnostics_level` (info | debug; any other value falls back to info and is reported as `parse-rejected`), the `MAX_FRAME` cap, and the read's home in the root-bin `viola::obs`.
- The `MAX_FRAME` consumer list adds the `config.json` read.
**Why:** chunk 2026-09-24-diagnostics-plane shipped the read (report Schema/config). The security detector returned no violation and noted both omissions. The orchestrator raised them as routine accurate-additions.

Sweep: `budget thresholds, GUI port\)` and the config-row wording over all seven masters. The only amend-site is security :230 (arch :365/:483 were amended in their own entry). 0 remaining.

## 2026-09-24-observability-gates — CI runs the obs artifact canary scan before uploads
**Section:** §Secret Management ("Secret scanning in CI") · §Bootstrap phases (secret-scanning-ci-gate)
**Change:**
- "Not in v1" / "Not wired in v1" now reads "no repo secret scanner in v1". That stays true: no scanner was researched.
- Both sites record that CI runs obs-plan §9's artifact canary scan (`viola-harness secret-scan`, `id: secret-scan`) before every test-home upload, against this plan's NEVER-log floor, and names its classes. It never prints or writes matched bytes.
- The `mutants.out/` upload is noted as unscanned: a CARRY on "Quality gates".
**Why:** chunk 2026-09-24-observability-gates (report Changes: Symbols / APIs `secret-scan`, Harness / gate surface). The previous text read as "CI does no secret scanning", which the new CI contradicts. The Decisions Log entry and the Threat Model's verbatim CI list are history and are unchanged.
**Sweep (cascade step 2):**
- Masters, 7 of 7:
  - `Not wired in v1`: 0 hits after the apply;
  - `Secret scanning in CI:** Not`: 0 hits.
- `security-plan.md:601` (the Decisions Log line "Secret scanning in CI (no scanner researched)") needs no change; it is still true of a repo scanner.
- Leaves: `security-summary.md` and `.claude/rules/security.md` state no scanner claim; 0 hits.

## 2026-09-24-quality-gates — fuzz lockfile exemption, fifth pinned action, fuzz and MSRV toolchains, unscanned uploads, declined concurrency
**Section:** Threat Model Summary (Supply chain entry point and trust boundary; Infrastructure CI/CD) · Dependency Security (`deny.toml` additions; Pinning ×2; CI integration: cargo-deny scope, nightly fuzz job, declined `concurrency:`, pinned actions, toolchains) · Bootstrap phases `secret-scanning-ci-gate` · Security Decisions Log (new 2026-09-24 entry)
**Change:**
- `fuzz/` is a separate workspace whose `fuzz/Cargo.lock` (`libfuzzer-sys =0.4.13`, which builds C++, and `arbitrary =1.4.2`) sits outside `cargo deny`. This is a test-only exemption, conditional on never linking and never joining the root `[workspace]`, with its advisory/source audit owed to "Workspace tree and code-graph planes".
- Both lockfiles are committed.
- `actions/download-artifact@3e5f45b… # v8.0.1` joins the pinned set (5).
- Toolchains: `rust-toolchain.toml` 1.98.1, `fuzz/rust-toolchain.toml` `nightly-2026-09-20`, and MSRV `1.96` via `RUSTUP_TOOLCHAIN`, all through rustup.
- `nightly.yml` gains the fuzz job.
- zizmor pedantic `concurrency-limits` is declined (why).
- The `mutants.out/` CARRY is closed by removal. Two unscanned uploads are admissible by content: the verdict JSON (repo-relative source locations and outcomes only, never absolute paths), and the nightly `fuzz/artifacts/` from the synthetic corpus.
**Why:** chunk 2026-09-24-quality-gates. Operator rulings at wrap P2 (the overseer, founder-delegated): "Ratify + CARRY audit"; "Ratify both by content", "repo-relative source locations only, never absolute paths" (verified: 118/118 names in the local leg verdict start `crates/`).
**Sweep:** `mutants\.out` 3 hits, the amended `:384`, the new Decisions entry, and none stale. `no toolchain action` 2 hits, both amended to name the three rustup toolchains. `upload-artifact` in pinned-set phrasing: every site lists download-artifact. Threat Model amended in place per the prior-wrap precedent (sidecar entries at lines 4 and 30). Leaves re-derived: `.claude/rules/security.md` (fuzz exemption bullet), `.claude/docs/commands.md` (deny scope). `.claude/docs/security-summary.md` recomputed with no change (it states no action set, CI jobs or upload list).

## 2026-09-24-workspace-tree-and-code-graph-planes — fuzz lockfile audit wired, release-check and orphans in the CI job list
**Section:** §Threat Model Summary (Supply chain trust boundary; Infrastructure CI/CD workflows and jobs) · §Dependency Security (the `fuzz/` bullet; CI integration Job 4 and the nightly workflow) · §Security Decisions Log 2026-09-24 (Conditions)
**Change:**
- The `fuzz/Cargo.lock` audit is no longer "owed". The ci.yml `supply-chain` step `Fuzz lockfile audit (advisories, sources)` runs `cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources` into `target/supply-chain/deny-fuzz.json`, and weekly `nightly.yml` `advisories` runs `cargo deny --manifest-path fuzz/Cargo.toml check advisories`.
- Both run from the repo root, where cargo-deny resolves the root `deny.toml`. The root run's families (the C-build ban included) still do not reach the fuzz graph.
- Job 4 states the root-lock scope and the same job's separate fuzz step.
- The Threat Model CI job list adds the cargo-modules orphans gate, the fuzz lockfile audit, and the per-OS release build through `scripts/release-check.sh` (refuses any test-only binary) in place of a bare `cargo build --release`.
- The Decisions Log Conditions record the CARRY as delivered.
**Why:** chunk 2026-09-24-workspace-tree-and-code-graph-planes (report Changes: Harness/gate surface, Schema/config; Expected amendments). No detector proposed these: the orchestrator raised them under Validate check 5, routine because the report substantiates each. Operator P4 decision 3 added the nightly advisories.
**Sweep** (same pass `sweep.py`): security-plan hits after the apply:
- `root Cargo.lock only` 2 hits, amended (:134 now "root graph covers the root lock only … gets its own audit"; :327 plus the fuzz step);
- `outside cargo deny` 2 hits, :315 amended; :646 no change (Decisions Log history, true);
- bare `cargo build --release` :161 amended;
- nightly advisories without fuzz, 2 hits, no change: :399 (the bootstrap phase that wired the weekly run, true) and :617 (manual review driven by the weekly run, true);
- `owed` (word-bounded) 0 after the apply.

Leaves re-derived: `.claude/rules/security.md` (weekly advisories over both lockfiles; the fuzz lockfile's own audit; a new release-build-carries-`viola`-only line). `.claude/docs/security-summary.md` was checked: 0 hits (unchanged).

## 2026-09-25-security-prerequisites — SQOS spike passed, SHA-256 crate picked, 0BSD per-crate exceptions
**Section:** §Authentication & Authorization (IPC client-side server verification row); §Data Protection (code-bearing artefacts); §Dependency Security (`deny.toml` additions, new `[licenses]` bullet); §Bootstrap phases (`viola-channel` client and `viola-state` re-hash bullets); §Security Decisions Log (the two Open questions removed; new `2026-09-25` entry).
**Change:**
- The client-verification row now states that the adoption spike passed (a same-user server reads `SecurityIdentification`), that `FILE_FLAG_OVERLAPPED` is required, and that the default connect reads `SecurityImpersonation` and stays banned.
- §Data Protection names `sha2 =0.11.0` (`default-features = false`) as the SHA-256 implementation.
- §Dependency Security records `allow` unchanged plus two per-crate `0BSD` exceptions (`doctest-file`, `recvmsg` via interprocess 2.4.4), never through `allow`; each further exception needs its own entry.
- Both bootstrap bullets cite the resolved decisions.
- The Decisions Log `2026-09-25` entry carries the measurements, conditions and ratification.
**Why:**
- The chunk's report: Dependencies; Cross-project claims (scratch probe, levels 1/2, the hang, interprocess `ReOpenFile`); Spec claims disproved #3; Expected amendments.
- CI run 36138441784 on `8e25ca7`: windows `test` leg PASSed the witness.
- The 0BSD exceptions widen a hardened boundary (playbook "Boundary widening", escalate). The escalation is resolved by the operator's ratification, recorded here: overseer, founder-delegated, at the phase P4 fork ("per-crate exceptions") and again in the wrap P2 directive item (2).
**Sweep** (cascade step 2, over the seven masters + CLAUDE.md + `.claude/rules/*` + `.claude/docs/session-learnings.md` + playbook + drift-base):
- Patterns: `open spike|spike \(Decisions Log|Spike: does|replacement SQOS open|must pass, or its replacement`, `open question.{0,40}(SHA|hash)|(SHA|hash).{0,60}open question|must first be picked|SHA-256 implementation is an open`, `no hash crate|pick.{0,20}hash`. All 0 hits after the apply.
- Known-positive control: the same patterns over `git show HEAD:.andromeda/security-plan.md` = 4 hits (:205, :265, :373, :375 and :609/:611 open questions), all amended.
- The detector's sweep found :265, which the report's site list missed.
- `windows-sys` restatements across the masters were read (29 lines): all consistent. security-plan :189/:202/:207/:374/:589 describe the same client open or unrelated DACL work, so no change.
- Leaves re-derived:
  - `.claude/docs/security-summary.md` (Open questions → a new "Resolved prerequisites" section);
  - `.claude/rules/security.md:27` (the licence allowlist + 0BSD exceptions);
  - `.claude/docs/stack.md` (Security-plan additions);
  - `.claude/docs/services/viola-channel.md:29`;
  - `.claude/docs/services/viola-state.md:30`.
- 0 hits in CLAUDE.md `GENERATED` blocks, the curation homes, playbook and drift-base.

## 2026-09-25-pty-wrapper-on-windows — R8 persistent-environment exemption, registry name reader, child resolution as built
**Section:** Input Validation (Configuration values row; new row "Persistent-environment names (Windows registry)"; Child executable resolution row) · Secret Management → Storage (inherited credentials) · Error Handling (fixed `Display`) · Bootstrap phases (`error-sanitization-wire`) · Security Decisions Log (new `2026-09-25` entry)
**Change:**
- `config.json` `claude_env_keep`: ≤ 32 names matching `^CLAUDE[A-Z0-9_]*$`, rejected whole when bad, a floor name ineffective.
- New Input Validation row for the registry reader: value names only, bounded buffer, missing key → empty set, the identity floor overrides it.
- Child executable resolution as built: viola resolves the program to an absolute path (PATHEXT only on Windows, never the bare name), `claude.cmd`/`.bat` → fixed sibling `claude.exe` with the shim never read, every other `.cmd`/`.bat` → closed `Refusal::BatchScriptChild` (exit 1, no spawn, two fixed stderr lines), explicit cwd.
- Secret Management: R8 described as the prefix rule + persistent exemption + 11-name identity floor (both messaging variables on the floor), names only.
- `PtyError`'s `Display` is hand-written (the other enums thiserror); fixed messages unchanged.
**Why:** chunk 2026-09-25-pty-wrapper-on-windows report Changes (Schema / config, Symbols, Coverage) and expected amendment 4. Boundary widening (the child may receive persistent `CLAUDE*` names; `config.json` admits a key; registry names are read) ratified by operator ruling 1, confirmed at wrap P2 (E1) and recorded in the Decisions Log. Rejected (E4): the two Threat Model Summary edits (:42, :109) — that section is the verbatim copy of threat-assessment.md (:18); the winning sections carry the truth.
**Sweep:** patterns as the architecture entry of this chunk over the 7 masters: security :42 and :109 (verbatim Threat Model) no change per E4; :675 the new Decisions Log entry itself; :389 and :465 (thiserror `Display` on `PtyError`) amended. Leaves: `.claude/docs/security-summary.md` and `.claude/rules/security.md` recomputed — no line states the strip list, the config keys or the resolution mechanism beyond "resolve to the real .exe", which stays true: no change.

## 2026-09-26-ci-chunk-base-and-union-verdict — chunk.diff out of the canary scan and the harness upload; ci.yml reads no event value
**Section:** Dependency Security → CI integration (event-payload `env:` rule; declined `concurrency-limits` reason) · Bootstrap phases → `secret-scanning-ci-gate` · Secret Management → Secret scanning in CI
**Change:**
- The canary scan covers the harness capture except the mutation leg's `target/agent-run/chunk.diff` (repository source text by construction); the `harness-<os>` upload drops the same file, so it is never scanned and never uploaded; a `chunk.diff` elsewhere is still scanned. The two admissible-by-content unscanned uploads are unchanged.
- The event-payload rule stays; its example is retired: `ci.yml` reads no `github.event` value, the mutation base is derived by the harness, `AGENT_RUN_CHUNK_BASE` is only a local override.
- The declined-concurrency reason keeps the `always()` gate/upload chain and retires the "drop a push's `--in-diff` mutation diff" half.
**Why:** chunk 2026-09-26-ci-chunk-base-and-union-verdict report Symbols/APIs, Harness / gate surface, Counts (`ci.yml` `github.event` 1 → 0), Expected amendments (security-plan). The :340 example was flagged by the doc-agent outside its detectors and raised by the orchestrator (check 5); :331 was found by the cascade sweep (cross-master restatement of the architecture reason).
**Sweep:** the test-plan entry's 12 patterns; security-plan rows :331, :340, :386, :430–:438 amended; no other hit. Leaves: `docs/security-summary.md` and `rules/security.md` state neither the scan scope nor the base (0 hits) — no change. Fanned 2 proposals (D-security-auth, 1 `dependent-of`) + 2 orchestrator-raised, all applied with text re-derived from the report; D-security-deps (syn/proc-macro2 exact pins, MIT OR Apache-2.0, deny four-green) and D-security-input found no drift.

## 2026-09-26-local-linux-pre-push-gate — `FAKE_AGENT_PUMP_DELAY_MS` carve-out; WSL pre-push distro: CI's pins, `env -i`
**Section:** §Input Validation (boundary table) · §Dependency Security → Pinning · §Secret Management → Storage (Production, Development) · §Security Anti-Patterns → Universal · §Security Decisions Log `2026-09-27`
**Change:**
- Input Validation: a row for the test seam — `fake-agent`-only, u64 ms capped at 5 000, compiled out of release builds, not `VIOLA_*`, configures nothing, disables no control, widens no redaction.
- Anti-Patterns Universal: the env rule gains the one carve-out and a NEVER for another seam (or this one in a release build) without a Decisions Log entry; Storage (Production) points at it.
- Dependency Security: the WSL2 `Ubuntu` distro `pre-push` drives is a third install site of CI's own pins (`scripts/wsl-provision.sh`: sha256-pinned rustup-init 1.29.1, the `rust-toolchain.toml` channel, `cargo install --locked` of ci.yml's `test`-job line; `tool-pin-mismatch`; no sudo); the toolchain sentence names host, CI and the WSL2 distro.
- Storage (Development): every WSL call runs under `env -i` (HOME + PATH only), so no `CLAUDE*` value crosses — the second layer behind `WSLENV` forwarding only `WT_*` (research M5; the gate's canary 0, its `WSLENV` control 1).
- Decisions Log `2026-09-27`: the seam and the gate's env/pin posture (ruling of record).
**Why:** chunk 2026-09-26-local-linux-pre-push-gate report Symbols/APIs (the seam), Harness / gate surface, Dev-tool versions, Expected amendments 10, 11, 17. The seam is a boundary widening (playbook "Boundary widening", never routine) — a test build of `viola` reads a new input; ratified by the operator's wrap directive (overseer, founder-delegated): "record the test-only `FAKE_AGENT_PUMP_DELAY_MS` seam as the architecture + security-plan amendments — a carve-out behind cfg(feature=\"fake-agent\"), capped at 5 s, absent from release builds".
**Sweep:** the test-plan entry's 23 patterns (+4 hand-controlled). Security-plan lines :232, :324, :326, :422, :426, :580 amended and :683-691 added (Decisions Log); :131 (every toolchain installed by `rustup`, no toolchain action), :332, :341, :342 true — no change. Leaves re-derived: `docs/security-summary.md` (:39 carve-out, a `pre-push` critical decision), `rules/security.md` (Dependencies and CI: two bullets); CLAUDE.md warnings env line recomputed — still true, no change. Full row list: `.andromeda/runs/2026-09-27T00-51-08-wrap/sweep-dispositions.md`.

## 2026-09-27-instance-state-and-start-order — the tempfile `persist` helper; `TMPDIR` carve-out under `env -i`
**Section:** §Authentication & Authorization (Token / session storage; `~/.viola/` access control) · §Data Protection (plugin folder, `settings.json`) · §Bootstrap phases (auth-scaffolding-baseline) · §Secret Management (pre-push `env -i`) · §Dependency Security (the sha2 KAT line) · the pre-push Decisions/critical line (`env -i`)
**Change:**
- Every "atomic-write-file" site → the shared tempfile `persist` helper `viola_state::fs::replace_private` (mode set on the temp file before any byte, then `sync_all` + `persist`); the plugin rewrite uses `replace_private_shared` (a failed replace counts as done only over byte-identical content).
- `env -i`: the Linux mutation leg's `TMPDIR=<distro home>/viola-pre-push-scratch` is recorded as the one named, constant, distro-derived assignment beside HOME and PATH — no host value; any assignment taking a host value stays a boundary widening needing its own Decisions Log entry.
- sha2: now a `viola-state` product dependency; the root dev-dependency stays for the KAT.
**Why:** chunk report Spec claims 1, Dependencies, Harness (`pre-push` TMPDIR); Expected amendment 6. The `TMPDIR` crossing is the boundary-widening class (never routine): RATIFIED by the overseer (founder-delegated) before the fan-out — "a named, constant, distro-derived assignment; no host value" — recorded here as that exact carve-out.
**Sweep:** rows :197, :207, :265, :266, :380, :426, :686 are this pass's text; :671 amended (fold); :207@c1782, :500, :579 (the snapshot `endpoint` as a verification reference once bound) true — no change. Leaves re-derived: `rules/security.md` (:15, :32), `docs/security-summary.md` (:66); `docs/security-summary.md:28` true. Full rows: `runs/2026-09-27T06-12-23-wrap/cascade-sweep.md`.

## Registry migration (U35) — 2026-09-29

<!-- U35 · security-plan.md · ## Security Decisions Log · sha256 b5017655aae4d4bff03c0c4b142cca865d3ba3c5c93bdd121cb306a206763b00 -->

## Security Decisions Log

_Records key decisions during plan generation + manual additions between phase loops._

**Initial entry:**

`2026-09-23` — Initial security plan generated by `/andromeda-security`
- **Tier:** Minimal (0) with targeted local-boundary elevations. Justified by: local-only, single-user, no accounts, no database, no stored credentials, no regulated data. The elevations cover the IPC endpoint as a code-execution interface, the loopback GUI streaming persistent conversation content, and the code-bearing state files (Threat Model Sec 6).
- **Key decision:** The same-OS-user boundary is enforced per OS through interprocess 2.4.4 primitives plus additions:
  - Windows: an explicit SDDL DACL, a client SQOS Identification open via windows-sys, and server pid + start-time verification. For `~/.viola/`, an owner + DACL startup check (the Windows strict-modes equivalent) that tolerates only the user, SYSTEM and Administrators (`CREATOR OWNER` treated as the user) as writers, counting inheritable ACEs because viola's files inherit them, and as readers of the home, `instances/<name>/`, `instances/<name>/diagnostics/`, `ui/` and each existing `events.ndjson` before it is opened (files inherit the directory DACLs, but `events.ndjson` is appended to rather than replaced, so its own ACE is checked too; this protects `events.ndjson`, `diagnostics/` and the `.url` token). It refuses an unreadable descriptor, a NULL DACL or a volume without persistent ACLs, and it tests the generic bits (`GENERIC_READ`, `GENERIC_WRITE`, `GENERIC_ALL`) that inherit-only ACEs keep unmapped. The check also covers the `bin/`/`plugin/` version folders in use and the trusted files (`snapshot.json`, `ledger/stamps.json`, the pinned exe), because a file can carry an explicit, non-inherited ACE that the directory checks do not see. On both OSes the check runs in every process that reads `snapshot.json`, `ledger/stamps.json` or `statusline_command`, including every `hook` event and the CLI client verbs, because the snapshot's `pid`/`started_at` are the Windows server-verification reference.
  - Unix: a 0700 per-user socket directory as the primary control, `mode(0o600)` as secondary, and a `peer_creds` euid check on both sides. For `~/.viola/`, the strict-modes check covers the listed directories, the `bin/`/`plugin/` version folders in use and the trusted files (`snapshot.json`, `ledger/stamps.json`, the pinned exe, which is the only file created 0700 rather than 0600). It also refuses group/other permissions on the home, `instances/<name>/` and `ui/`, matching the Windows read rule, so pre-existing homes are covered on both OSes. On both OSes the check also covers the plugin folder's `hooks/` and `.claude-plugin/` subfolders. `viola run` rewrites `hooks.json` and `plugin.json` into them atomically, and the replacement file keeps its own mode or ACL, so a writable subfolder would let another user replace them after the rewrite (code execution on every hook event).
  - The GUI gets a per-launch getrandom token exchanged for a `SameSite=Strict` HttpOnly cookie.
- **Arch amendments this plan requires** (for arch/route to fold in):
  1. **GUI Control Scope**: "no token" becomes a per-launch cookie on `/api/*` and SSE in v1, not only for the v1.x brake. Loopback TCP lets other OS users read prompts and tool `input` (Threat Model Sec 2), which breaks the Local endpoint trust boundary. This follows the axum research pattern.
  2. **Occupied Resources (IPC endpoints)**: the Unix endpoint moves from `$TMPDIR/viola-<h12>.sock` to `<per-user 0700 dir>/viola-<h12>.sock` (`$XDG_RUNTIME_DIR/viola/` on Linux, `$TMPDIR/viola/` on macOS, `/tmp/viola-<uid>/` as a fallback). The snapshot-recorded `endpoint` mechanism is unchanged.
  3. **Conventions (refusal details)**: add `not-delivered` detail `control-character`, checked before the documented refusal order. Add the Problem Details URNs `urn:viola:problem:unauthorized` (401) and `urn:viola:problem:cross-origin-forbidden` (403).
  4. **Open item resolved**: a `release` whose params carry `from` gets `-32602` `release-from-driver`. This is an affordance guard, not enforcement, because same-user processes are trusted in v1. The research's optional per-instance 0600 proof file was not adopted: a same-user driver's Bash tool could read it, so it adds nothing over ambient trust.
  5. **New file**: `<viola home>/ui/<port>.url` (0600).
  6. **New constant**: `MAX_FRAME` (16 MiB) in `viola-core`.
  7. **Capability ledger (CLI Version Compatibility)**: new row "largest hook payload seen" (bytes per hook event kind), measured by `viola verify`, which checks `MAX_FRAME` against data.
  8. **Deployment / Distribution**: `<hash>` in `bin/<version>-<hash>/` and `plugin/<version>-<hash>/` is a truncated SHA-256 of the exe bytes (16 hex characters).
- **Accepted risks:**
  - A same-user process, including a driver agent's Bash tool, can reach every channel method (v1 ambient trust, Cross-cutting Patterns).
  - A driver LLM can be prompt-injected by driven-session output into `answer` `allow`. Policy is outside viola (R1/R4), and viola's only role is the closed `behavior` enum and the `unverified-cli` gate.
  - A squatted endpoint name blocks `run` (exit 1), a denial of service rather than a takeover.
  - `~/.viola/` content is plaintext by design.
  - `claude` for `claude agents --json` is resolved from the calling process's PATH (same-user trust). A planted `claude` can at most inject display rows into `list`, the MCP `list` tool and `/api/sessions`. Those rows are never a `ViolaName` or `target`, are rendered as text only in the GUI, and are printed by human-mode `list` only with C0/C1 controls escaped.
  - Cross-port cookie exposure: `viola_<port>` is sent to every `127.0.0.1` port. A loopback server run by another local user can capture it if the user navigates there directly during the browser session, and could then read the feed until `viola ui` restarts. The viola page never requests other ports (CSP).
  - Local availability: another local user can hold GUI connections or SSE streams open (no connection or stream bound in v1), and unbounded `events.ndjson` growth can exhaust local disk until arch resolves retention. The impact is availability of the human's view and disk only, with no confidentiality or integrity impact.
- **Open questions:**
  - Retention and bounding of `events.ndjson` is an arch open item. Any scheme must keep 0600 and valid offsets.
  - (The SQOS spike and the SHA-256 crate were resolved on `2026-09-25`; see that entry.)
- **Known gaps (for the public version):**
  - macOS client verification has no pid check: `peer_creds()` gives no pid there, so it rests on euid + the verified 0700 socket directory. Accepted for v1 because macOS is CI-only (brief O5 unmeasured). Revisit before public distribution.
- **Needs without a researched tool (flagged, not invented):**
  - GUI/IPC rate limiting: not needed at this tier.
  - Secret scanning in CI (no scanner researched): revisit with the v1.x signing credentials.
  - An automated dependency-update bot (none researched): manual review, driven by the weekly `cargo deny check advisories`.
  - An HTML sanitizer for assistant Markdown (none researched): the frontend must render text only.
  - Encryption at rest (none researched, and plaintext is an arch decision).
- **v1.x release prerequisites** (Supply chain integrity, deferred with release):
  - dist `>=0.33.0` with GitHub Artifact Attestations and Azure Artifact Signing (x86_64).
  - cargo-auditable `>=0.7.6` + `cargo audit bin` (cargo-audit `>=0.22.2`) on release binaries.
  - self_update 1.3.0 built with `signatures` (zipsign).
  - zizmor cache-poisoning findings addressed.
  - No `rust-cache` in the release workflow.

`2026-09-23` — Phase 3.5 user review (round 1)
- **Decision:** Items 1–5 accepted as drafted (v1 GUI cookie, Unix per-user 0700 socket directory, `control-character` detail, `release`-with-`from` refused, `MAX_FRAME`), and the Auth & Authz section kept. Plus:
  - `validate_paste_text` exact set: LF, CR and TAB allowed; every other C0, DEL and C1 refused (ESC above all), checked over decoded `char`s.
  - Windows pipe SDDL is user SID + SYSTEM only; the logon SID is rejected.
  - macOS: euid + 0700 directory with no pid check accepted, recorded as a known gap.
  - `<hash>` is a truncated SHA-256 of the exe bytes.
  - `MAX_FRAME` stays 16 MiB by default, checked by a new capability-ledger row for the largest hook payload seen.
- **Rationale:**
  - Multi-line text is normal (brief §4.1 S1).
  - v1 promises an OS-user boundary, and a same-user driver in another logon session (SSH, scheduled task) must still connect.
  - macOS is CI-only in v1 (brief O5).
  - A cryptographic hash removes the accidental-only caveat at no cost; same-user tampering stays outside v1.
  - The cap should be checked against measured data.
- **Impact:**
  - Sections changed: Input Validation (paste rule, constants), Authentication & Authorization (Windows DACL, client verification), Data Protection (code-bearing artefacts), Anti-Patterns (Input, Data Protection).
  - Arch amendments: new ledger row "largest hook payload seen" (CLI Version Compatibility), and `<hash>` defined as truncated SHA-256 (Deployment / Distribution).
- **By:** `/andromeda-security` Phase 3.5, user review

`2026-09-24` — Fuzz workspace exemption and unscanned CI uploads (chunk 2026-09-24-quality-gates)
- **Decision:** `fuzz/` is a separate cargo workspace (`viola-fuzz`, excluded from the root) with its own `fuzz/Cargo.lock` holding `libfuzzer-sys =0.4.13` (builds C++ via `cc`) and `arbitrary =1.4.2`. It is accepted outside `cargo deny`'s root graph as test-only. Its fuzz toolchain is `fuzz/rust-toolchain.toml` `nightly-2026-09-20`. The first target is `viola_name` (`ViolaName::try_new`), with a synthetic corpus only. Two CI uploads leave without the secret scan, admissible by content: `mutants-verdict-<os>.json` (repo-relative source locations and outcomes only, never absolute paths) and the nightly `fuzz/artifacts/` (fuzzer inputs from the synthetic corpus).
- **Rationale:** The crates run only in the CI fuzz jobs (`contents: read`) and are never linked into `viola`. A secret scan has nothing to match in the verdict file, and the mutants job has no homes to scan (`empty-scope`). The raw `mutants.out/` upload it replaces held absolute argv paths and test output.
- **Conditions:** `fuzz` never joins the root `[workspace]`; a non-synthetic corpus seed drops the fuzz upload first. The advisory and source audit of `fuzz/Cargo.lock` was carried to the "Workspace tree and code-graph planes" chunk (route CARRY), which wired it in `ci.yml` `supply-chain` (advisories, sources) and weekly `nightly.yml` `advisories`.
- **By:** the overseer (founder-delegated), wrap-session P2 escalation

`2026-09-25` — SQOS adoption spike, SHA-256 crate, 0BSD licence exceptions (chunk 2026-09-25-security-prerequisites)
- **Decision:**
  - **SQOS spike: passed.** On interprocess `=2.4.4` + windows-sys `=0.61.2`, `CreateFileW(GENERIC_READ | GENERIC_WRITE, …, OPEN_EXISTING, SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED)` adopted by `interprocess::os::windows::named_pipe::local_socket::Stream::try_from(OwnedHandle)` exchanges bytes both ways, and a same-user server reads `SecurityIdentification`. This is the Windows client open. No replacement is needed, and interprocess's default connect stays banned as a fallback or test double.
  - **SHA-256 crate: `sha2 =0.11.0`, `default-features = false`** (RustCrypto, MIT OR Apache-2.0, rust-version 1.85). `<hash>` = the first 8 digest bytes as 16 lowercase hex characters. FNV-1a / `DefaultHasher` stay banned for it.
  - **Licences:** interprocess's transitive `doctest-file` 1.1.1 and `recvmsg` 1.0.0 (both `0BSD`) are admitted by per-crate `[[licenses.exceptions]]`, never by adding `0BSD` to `allow`.
- **Rationale:**
  - SQOS, as measured at chunk 2026-09-25-security-prerequisites (research.md §Measured facts, scratch probe on 1.98.1-msvc):
    - the recipe reads level 1 (Identification);
    - the same open without the SQOS flags, and interprocess's default connect, read level 2 (Impersonation), so the check discriminates;
    - without `FILE_FLAG_OVERLAPPED` the adopted handle hangs its first exchange.
  - The level holds through interprocess's internal re-open: `try_from` calls `ReOpenFile(h, …, FILE_FLAG_OVERLAPPED)` with no SQOS flags (interprocess 2.4.4 `named_pipe/c_wrappers.rs:176-182`), and the measured level shows the re-open keeps the connection's connect-time level.
  - The measured safe-Rust equivalent, std `OpenOptions::security_qos_flags(SECURITY_IDENTIFICATION)` + `custom_flags(FILE_FLAG_OVERLAPPED)`, also reads level 1. It is recorded for the `viola-channel` chunk to weigh, not adopted.
  - sha2: its graph (cfg-if, cpufeatures, digest, block-buffer, crypto-common, hybrid-array, typenum; libc on aarch64-apple) has no `cc` on the three `[graph]` triples, passes `cargo deny check`, and builds on the 1.96 floor.
  - 0BSD is OSI-approved and more permissive than MIT. The per-crate form is the narrowest widening: any other 0BSD crate still fails the gate.
- **Conditions:**
  - `tests/channel_sqos_open.rs` (`#[cfg(windows)]`, the windows-2025 CI `test` leg) pins the recipe and its no-SQOS control. The `viola-channel` client reuses the recipe, and its own `security_negatives_*.rs` case lands with that crate.
  - `tests/contract_content_hash.rs` pins sha2 against published FIPS 180-2 vectors and the 16-hex truncation. `sha2` is a product dependency of `viola-state` (the pinned-copy key and re-hash); the root dev-dependency stays for this known-answer test.
  - Each further licence exception needs its own entry here.
  - Witness: CI run 36138441784 on `8e25ca7`, 15/15 success. The windows `test` leg PASSed both SQOS tests and all five hash cases.
- **By:** the operator (overseer, founder-delegated). The 0BSD exceptions were ratified at the phase P4 fork and again in the wrap P2 directive, as a boundary widening (playbook "Boundary widening", never routine). The spike and pick were recorded by `/andromeda-wrap-session` P2 from the chunk's evidence.

`2026-09-25` — R8 persistent-environment exemption and identity floor (chunk 2026-09-25-pty-wrapper-on-windows)
- **Decision:** The R8 strip removes every inherited `CLAUDE*` name (ASCII case-insensitive on Windows, exact on Unix) except the names the user defines persistently — Windows `HKCU\Environment` / `HKLM\…\Session Manager\Environment` value names, Unix `config.json` `claude_env_keep` (≤ 32 names matching `^CLAUDE[A-Z0-9_]*$`, rejected whole when bad). The identity floor `IDENTITY_FLOOR` — the 11 names measured on the Windows host (`CLAUDECODE`, `CLAUDE_CODE_BRIDGE_SESSION_ID`, `CLAUDE_CODE_CHILD_SESSION`, `CLAUDE_CODE_ENTRYPOINT`, `CLAUDE_CODE_EXECPATH`, `CLAUDE_CODE_MESSAGING_SOCKET`, `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_SESSION_ATTENDED`, `CLAUDE_CODE_SESSION_ID`, `CLAUDE_EFFORT`, `CLAUDE_PID`) — is stripped whatever the persistent set says. Names only: no value is read by the strip, the registry reader or any log; the start line names every stripped and kept name.
- **Why:** a user-defined `CLAUDE*` setting (for example a config dir) must reach the child as it would reach a bare `claude`, while the parent session's identity and messaging credentials never do. The S6 "14-name" list was never enumerated by any artifact; the measured inherited set is 11 (the chunk's research fact 7).
- **Boundary widening:** the child may now receive `CLAUDE*` names it previously never did, `config.json` admits a new key, and viola reads registry value names. Ratified as a boundary widening (playbook "Boundary widening", never routine): operator ruling 1 at the chunk's phase P4, confirmed at its wrap P2 (E1).
- **Witness:** `tests/tui_env_strip.rs` (11 floor canaries + an unknown name stripped; on Unix a `claude_env_keep` name survives), `plan_strip` unit tables, secret-scan 0 canaries; CI run 36167590761 on `7681c73`, all jobs green.
- **By:** the operator (overseer, founder-delegated); recorded by `/andromeda-wrap-session` P2.

`2026-09-27` — Test seam `FAKE_AGENT_PUMP_DELAY_MS` and the local pre-push gate (chunk 2026-09-26-local-linux-pre-push-gate)
- **Decision:**
  - A `viola` built with the test-only `fake-agent` cargo feature reads one environment variable outside the `VIOLA_*` plumbing: `FAKE_AGENT_PUMP_DELAY_MS`, parsed as `u64` milliseconds and capped at 5 000, which holds the `run` pump back after the child starts (`#[cfg(feature = "fake-agent")] fn hold_pump_start`, `src/cmd/run.rs`). It is absent from every release build, configures nothing, disables no control and widens no redaction. It carries neither the `VIOLA_` prefix (it is not product plumbing) nor `AGENT_RUN_` (that prefix means "never read by `viola`").
  - The local `viola-harness pre-push` gate runs every WSL call under `env -i` (HOME + PATH, plus the distro-derived constant `TMPDIR` on the Linux mutation leg; no host value) and installs its toolchain from CI's own pins (§Dependency Security).
- **Rationale:** the forced-window test `tui_host_resize_in_the_pump_start_window_reaches_the_child` needs a resize to land between the spawn sizing and the pump's first look. Sampling that race is luck; the seam makes the window certain (as measured at the chunk's `evidence/linux-red-investigation.md` §Experiment A: 6/6 red before the pump fix, 6/6 green after).
- **Boundary widening:** a test build of `viola` reads a new input. Ratified as a boundary widening (playbook "Boundary widening", never routine) by the operator's wrap directive: "record the test-only `FAKE_AGENT_PUMP_DELAY_MS` seam as the architecture + security-plan amendments — a carve-out behind cfg(feature=\"fake-agent\"), capped at 5 s, absent from release builds".
- **Conditions:** `scripts/release-check.sh` keeps judging that the release build carries `viola` only, built without `fake-agent`; any further env seam needs its own entry here (§Security Anti-Patterns → Universal).
- **Witness:** CI run 36282518379 on `054ebe4`, 15/15 success; local `pre-push` green ×3 plus the operator pass's run; the gate's `CLAUDE_CODE_MESSAGING_TOKEN` canary reads 0.
- **By:** the operator (overseer, founder-delegated); recorded by `/andromeda-wrap-session` P2.

`2026-09-27` — Test-only feature proven absent, host mutation scratch, `wsl-exec.sh` (chunk 2026-09-27-epoch-2-cleanup)
- **Decision:**
  - `viola-channel` gains the test-only cargo feature `test-support` (shared test helpers: the `JsonFields` tracing visitor and, on Windows, a pipe DACL read-back with its own SID lookup). Only the root package's `[dev-dependencies]` enables it. `scripts/release-check.sh` now makes the entry above's Condition executable: `judge` refuses any `compiler-artifact` record whose `features` hold `test-support` or `fake-agent` (`release-check: FAILED — test-only feature {feature} in {target}`), and `--probe` reads `5/5 refused, control clean`.
  - On a Windows host `run --mutants` wipes a directory outside the repository, the host mutation scratch `<repo parent>/viola-mutants-scratch`. A pure guard refuses unless the resolved path's final component is exactly `viola-mutants-scratch` and the repository is neither it nor inside it. A root has no final component, so the name check refuses a drive or filesystem root. A refusal is `scratch-refused` and a failed removal `scratch-wipe-failed`, never a fallback to `%TEMP%` or the repository. The path is never printed.
  - `scripts/wsl-exec.sh` is a second launcher into the WSL2 distro, for operator use only (§Secret Management).
- **Rationale:** the feature replaces three test-code clone pairs without a product-visible surface. The scratch keeps ~30 GB of cargo-mutants temp copies off C: and `mutants.out/` out of the repository. `wsl-exec.sh` removes a recurring hand-typed crossing (evolve P11).
- **Boundary widening:** `wsl-exec.sh` carries operator-typed argv and `--cd DIR` from the host into the distro, a new crossing of the `env -i` boundary. **Ratified live by the overseer at the 2026-09-27-epoch-2-cleanup wrap, under the founder's 2026-09-27 ruling** (the overseer's live word, not the founder's own). Merits as given: only what the operator types crosses, which is the tool's purpose, and `env -i` keeps every host environment value out.
- **Conditions:** no gate, harness command or plan entry may run a distro command through `wsl-exec.sh`; its own `--probe` self-test is exempt and stays a gate entry, as the proof of the ratified property (the overseer's clarification at the same wrap). A future use of it to run a command from a gate, harness or plan is a new widening that halts again. The scratch guard keeps its remove-the-guard pair (the chunk's `evidence/scratch-guard.md`: 3 refusal tests red with the guard neutralised, 13/13 green restored).
- **Witness:** `wsl-exec.sh --probe` green; `release-check --probe` 5/5 and `release-check` `viola only`; CI run 36333711860 on `f0e6dbc`, 15/15 success.
- **By:** the overseer (founder-delegated; E1 ratified live); recorded by `/andromeda-wrap-session` P2.

`2026-09-27` — The browser pipe's supply chain, and an operator-only root install in WSL (chunk 2026-09-27-browser-verdict-reachability)
- **Decision:**
  - The test-side npm graph `e2e-web/package-lock.json` (`@playwright/test` =1.63.0, 3 packages) is committed and gets its own audit, `scripts/npm-audit.sh` (advisories at every level + registry.npmjs.org-only sources), in `supply-chain` and weekly in `nightly.yml`. There is no exemption (P4 operator fork 3, founder ruling W125's pipe).
  - Node is a pinned official download (`scripts/install-node.sh`, sha256 from ci.yml's `NODE_PIN_*` lines), not a toolchain action and not runner-image Node (P4 operator fork 1). The WSL distro installs the same pin, and `pre-push` refuses `node` off-pin.
  - `scripts/wsl-provision.sh --install-deps <user home>` installs Chromium's system libraries as uid 0 through `wsl.exe -d Ubuntu -u root … env -i HOME=/root PATH=…` (never sudo).
- **Rationale:** one set of pins for the three CI legs and the WSL pre-push leg; an npm graph outside `cargo deny` still needs an advisory and sources gate; Chromium cannot launch in the distro without its system libraries, which only root can install.
- **Boundary widening:** the root launch is a new crossing into the distro, a new principal (uid 0) started from the host, and as shipped it runs the user-provisioned `node` and Playwright CLI as root. **Ratified live by the overseer at the 2026-09-27-browser-verdict-reachability wrap, under the founder's 2026-09-27 ruling** (the overseer's live word, not the founder's own), **operator-only**. Merits as given: root executing code the distro user can write turns a user-level compromise into root inside the VM.
- **Conditions:** only the operator launches `--install-deps` (by hand, or a `leg = 'operator'` plan entry), never the gate tool, a harness command or a pre-push stage. It ran once, on 2026-09-27 (this chunk's plan entry 42, 28 packages, recorded in the chunk's `evidence/wsl-chromium-deps.md`). Before any re-provision, `--install-deps` must stop running user-writable code as root: root runs only `apt-get install` over the package list an unprivileged dry run produced, checked against a committed allowlist, so apt installs only signed distro packages (a CARRY on the route).
- **Witness:** `install-node.sh --probe` and `npm-audit.sh --probe` green; `npm-audit.sh` `advisories 0, sources registry.npmjs.org only`; the pre-push `union` green with `linux.browser` playwright 1/0; CI run 36345175642 on `5f0a809`, 15/15 success.
- **By:** the overseer (founder-delegated; E1 ratified live, operator-only); recorded by `/andromeda-wrap-session` P2.

`2026-09-28` — `hook.event` served, and the hook's interim gap without server verification or strict-modes (chunk 2026-09-27-hooks-to-normalised-events)
- **Decision:**
  - The wrapper serves its first channel method, the id-less `hook.event` notification (§Standard Contracts in arch). It re-validates the event (the five hook kinds, an object `data`, `prompt-submitted` `text` a string and `origin` ∈ `harness` · `human`), appends it with `source:"hook"`, answers `-32603` on an append failure and `-32601` for every other method. The IPC controls it maps to are unchanged: the protected-DACL / 0700-directory listener, `accept_remote(false)`, `MAX_FRAME` framing and the `v` check.
  - `viola hook <event>` sends it to the `endpoint` recorded in `instances/<name>/snapshot.json` (through `VIOLA_DIR`, shape-checked only: absolute and ending `<home>/instances/<name>`, with `VIOLA_NAME` through `ViolaName::try_new`), through `viola-channel`'s `Client::connect_by` + `notify`, within the provisional 750 ms connect deadline, with NO client server verification (pid + start time / euid) and NO home strict-modes check.
- **Boundary widening:** prompt text, `last_assistant_message` and tool names reach whatever server answers at an endpoint read from a snapshot that has not passed strict-modes, against §Authentication's "before the first frame of any method, including `hook.event`". **Ratified by the founder, live, on 2026-09-28** (relay: the Viola overseer), after the widening was shown to him; the P4 overseer ruling (fork 2, founder-delegated) had not ratified it.
- **Rationale:** the Epoch 6 entries own the verifier and the strict-modes check; a hook must not grow a partial verifier first. Meanwhile the first-instance pipe and the exclusive-bind arbiter already stop a squatter from taking a live name.
- **Conditions:** the gap closes when "Server verification before any frame" and "Home and code-bearing file integrity" land (route CARRYs name the hook client and its fail-open-on-mismatch case). Every hook failure stays fail-open: exit 0, nothing on stdout or stderr. No other frame, verb or process may take this exception.
- **Witness:** CI run 36358593772 on `bb35d6e`, 15/15 success; `hook_fails_open_silently_within_the_spine_bound` (11 cases) and the wrapper's `methods_*` cases.
- **By:** the founder (live ratification, relay: the Viola overseer); recorded by `/andromeda-wrap-session` P2.

`2026-09-28` — Test seam `FAKE_AGENT_HOOK_PANIC`, and G2's exact-path exemption for its panic lines (chunk 2026-09-28-hook-perf-gate)
- **Decision:**
  - A `viola` built with the test-only `fake-agent` cargo feature reads a second environment variable outside the `VIOLA_*` plumbing: `FAKE_AGENT_HOOK_PANIC`. `hook()` calls `#[cfg(feature = "fake-agent")] fn panic_if_asked` (`src/cmd/hook/seam.rs`) right after `viola_obs_init`; it panics only when the value is exactly `1`, with the fixed synthetic payload `"forced-hook-panic ".repeat(256)`. The variable is named in product source only in that file, is absent from every release build, configures nothing, disables no control and widens no redaction.
  - CI's G2 (`scripts/g2-zero-panics.sh`, in the `test` and `perf` jobs) does not count a role-file `panic` line whose `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>`: the file compared as a whole string, never a prefix, suffix, regex or home path. The perf arm's own zero-panic check exempts nothing.
- **Rationale:** the fail-open contract is proven on its weakest path, a real panic in the real binary (exit 0, empty streams, a payload-free role line, a detail line over 4 KiB), and the same line closes obs D-28's concurrent over-4 KiB half. The forced-panic tests leave seam lines in the kept homes, so G2 without the exemption fails every run.
- **Boundary widening:** a test build of `viola` reads a new input, and a zero-panics gate admits one location. **Ratified by the founder, live**: the seam at 06:21 on 2026-09-28, and the seam with the G2 exemption (the exemption shown to him as new) at 09:52:07 the same day (relay: the Viola overseer).
- **Conditions:** `scripts/release-check.sh` keeps judging `viola only`; G2's `--probe` keeps proving that another file, a look-alike path (`x/…/seam.rs`, `seam.rs.bak`, `seam.rsx`), the seam path without a line, a missing location and an empty scope all read red, and runs before every G2 check; any further env seam, or any further G2 exemption, needs its own entry here (§Security Anti-Patterns → Universal).
- **Witness:** CI run 36390764600 on `5a693d6`, 18/18 success; `g2-zero-panics.sh --probe` all cases as expected; `release-check --probe` 5/5 and `release-check` `viola only`; the seam's remove-the-guard pair (the chunk's `evidence/seam-guard.md`).
- **By:** the founder (live ratification, relay: the Viola overseer); recorded by `/andromeda-wrap-session` P2.

`2026-09-28` — Stamps read without strict-modes until Epoch 6, and the hidden `hook --capture` arm (chunk 2026-09-28-capability-ledger-and-viola-verify)
- **Decision:**
  - Stamps read: `viola run`'s version gate reads `ledger/stamps.json` through `viola_state::stamps::read_stamps` (no lock, `take(MAX_FRAME + 1)`, over the cap an error), and `viola verify`'s `update_stamps` reads the current bytes for its locked read-modify-write, both without the home strict-modes check, until "Home and code-bearing file integrity" (Epoch 6) adds both to the strict-modes entry-point set.
  - Capture arm: `viola hook <event> --capture <DIR>`, called only by `viola verify`'s probe plugin, reads no `VIOLA_*`, does no obs init and opens no channel. It takes an absolute, existing `<DIR>`, reads stdin through `take(MAX_FRAME + 1)` and writes the bytes raw, unparsed, via `replace_private` (0600) to the first free `<DIR>/<PascalEvent>.<k>.json`; any failure writes nothing, and it always exits 0 with empty stdout and stderr.
- **Boundary widening:** a new process reads a trusted own-state file before strict-modes (§Authentication's entry-point set), and `hook` gains a new crossing: a raw payload written to an argv-named directory without the `VIOLA_NAME` / `VIOLA_DIR` checks (§Input Validation, Hook stdin).
- **Rationale:** no process runs the strict-modes check yet (the Epoch 6 entry owns it); an unreadable or malformed stamps file only degrades `run` to `cli_verified:false` with one codes-only `parse-rejected{parser:"ledger-stamps"}` line, and `verify` stays the only writer. The capture arm is how `verify` sees the four spine payloads without a new channel method (a new method needs its own entry here) or an env-var switch; it runs as the same user through the pinned exe, and its captures live in the 0700 probe dir that `verify` removes on every exit path.
- **Conditions:** "Home and code-bearing file integrity" (Epoch 6) adds `run`'s gate and `verify`'s `update_stamps` to the strict-modes entry points (route CARRY on that entry; PREREQ on "Dialog answers by dialog_id": strict-modes on `run`'s stamps read before any non-`null` decision). The capture arm stays hidden, silent and exit-0, and no other caller than verify's probe plugin may register it.
- **Witness:** CI run 36460408121 on `6486276`, 18/18 success; `tests/cli_version_gate.rs` (7 cases) and `tests/cli_verify.rs` (12 cases, incl. `hook session-start --capture` and the probe dir removed after every verify run).
- **By:** the founder (live ratifications, relay: the Viola overseer): the stamps read by `run` at 2026-09-28 12:39:40; its extension to `verify`'s read and the capture arm at 2026-09-28 20:24:32, after each was shown. Recorded by `/andromeda-wrap-session` P2.

`2026-09-28` — Mutation leaves CI and the pre-push: the distro `TMPDIR` carve-out and the unscanned leg-verdict upload retire (chunk 2026-09-28-mutation-testing-to-the-epoch-boundary)
- **Decision:** the `mutants (<os>)` and `mutants-verdict` CI jobs, the pre-push Linux and Windows mutation legs and their union are removed; mutation testing runs only through `agent-run run --mutants`, kept for the epoch-boundary code audit. With them go: the pre-push WSL `TMPDIR=<distro home>/viola-pre-push-scratch` assignment (every WSL call now carries exactly `HOME` + `PATH`), the unscanned `mutants-verdict-<os>.json` upload (the nightly `fuzz/artifacts/` is the one unscanned upload left), and the `actions/download-artifact` pin (the SHA-pinned set is checkout, rust-cache, install-action, upload-artifact).
- **Boundary narrowing:** it retires two earlier grants rather than adding one — the 2026-09-27 `TMPDIR` carve-out (the founder's retroactive ratification) and the 2026-09-24 admission of the leg-verdict upload. Their entries stay as history.
- **Rationale:** the founder's ruling of 2026-09-28 17:59 moves mutation testing to the epoch boundary; with no CI or pre-push leg, nothing uses the carve-out, the upload or the pin, and the overseer's P4 answer removes code without a consumer.
- **Witness:** CI run 36483042659 on `17b93c7`, 15/15 success; the operator pass's four pre-push runs (document keys `v cmd ok stage sync cache linux vm windows`, the chunk's `evidence/operator-pass.md`).
- **By:** the founder (ruling 2026-09-28 17:59, relay: the Viola overseer) and the overseer (P4 fork A); recorded by `/andromeda-wrap-session` P2.

`2026-09-29` — Vendored Microsoft ConPTY binaries, sideloaded; and the third interim gap, the companions loaded before Epoch 6's owner/DACL check (chunk 2026-09-29-sideloaded-conpty)
- **Decision:**
  - `conpty.dll` (109 920 B) and `OpenConsole.exe` (1 066 296 B) from nuget `Microsoft.Windows.Console.ConPTY` 1.24.260710001 (MIT, the Microsoft signer) are committed byte-identical to the official package under `vendor/conpty/1.24.260710001/x64/`, embedded in the Windows x64 `viola` and written write-if-absent to `bin/<version>-<hash>/conpty/`, from which `viola run` pre-loads `conpty.dll` by absolute path so the child is hosted by that `OpenConsole.exe` (§Input Validation, the DLL search order row). Their audit is their own gate, `scripts/conpty-vendor.sh --verify` / `--probe` (CI `ConPTY vendor verification`, `windows-2025`), never a `cargo deny` exemption; the signer is checked there, and at run time the SHA-256 pin carries it. `viola` makes no network call for them, at build or run time.
  - The control for the DLL planting vector this closes (a bare-name `conpty.dll` load picking a planted copy from the CWD or `PATH`, a hole present before this chunk): every `viola` process restricts its DLL search to System32 as the second statement of `main`.
  - Third interim gap: until "Home and code-bearing file integrity" (Epoch 6) adds the `conpty/` folder and both files to the Windows strict-modes set, `viola run` loads the companions under the `FILE_SHARE_READ`-only held handle and the full SHA-256 re-hash alone, without the owner/DACL check (§ `~/.viola/` access control).
- **Boundary widening:** a third-party prebuilt binary pair, outside every lockfile audit, is loaded into `viola` and hosts the child; and code-bearing files are loaded from `bin/` before the owner/DACL check. **Ratified by the founder, live, on 2026-09-29 at 10:41:12** (relay: the Viola overseer), both forks shown to him at the chunk's phase P4.
- **Rationale:** the H2 key-after-resize measurement in one windows-2025 run (ci#36563868040): sideloaded ConPTY 0 of 200 lost, inbox 14 of 200 (no rate claimed). The held-handle re-hash denies a concurrent writer from check to spawn; the owner/DACL check is the Epoch 6 entry's, and no process runs it yet.
- **Conditions:** the gap closes when "Home and code-bearing file integrity" lands (route CARRY on that entry). A version move is a re-vendor through the script, never a hand copy; a sideload failure never refuses, prints or changes the exit.
- **Witness:** CI run 36566391084 on `8f643f2`, 15/15 success (its `ConPTY vendor verification` step green); `conpty-vendor probe: 4/4 refused, control clean`; `pin_companions_holds_every_file_against_a_writer`; the `conpty_sideload` planted and tamper cases; `restrict_dll_search_keeps_planted_conpty_out_of_a_bare_name_load` (two-sided).
- **By:** the founder (live ratification, relay: the Viola overseer); recorded by `/andromeda-wrap-session` P2.

`2026-09-29` — The cross-session-message tag filed as harness origin (chunk 2026-09-29-fake-agent-drift-contract)
- **Decision:** `viola-agent-claude`'s compiled harness prefixes become four: `<agent-message from=`, `<task-notification>`, `<\cross-session-message` and `<cross-session-message`, matched on the UserPromptSubmit `prompt`'s raw start with no trim. A prompt starting with either cross-session form normalises to `origin:"harness"` and never flips the wheel; the same tag mid-prompt or after a leading space stays `human`.
- **Boundary widening:** the classifier's `harness` class admits a new input, and it includes an ESCAPED tag, which the escaped-means-typed design had kept `human`. Side effect, accepted: a human who types the cross-session tag at a prompt's start is filed `harness`. **Ratified by the founder, live, on 2026-09-29 at 15:21:44** (relay: the Viola overseer), after the widening and its side effect were shown at the chunk's plan fork.
- **Rationale:** a Claude SendMessage arrives in the driven session as a cross-session message; filed `human`, it flipped the wheel (three flips on andromeda-worker, 2026-09-28/29). The escaped injection form `<\cross-session-message from="…" from-name="…">` is a relayed measurement (overseer1's F115; the viola-lab prototype's `harness_injected`), not yet measured in this repository.
- **Conditions:** "First live test and self-drive" measures a real cross-session UserPromptSubmit `prompt` and records the harness-prefix ledger row (route CARRY on that entry). The list stays compiled; no runtime or upstream text builds a prefix (Code Patterns).
- **Witness:** `prompt_origin_files_the_cross_session_tag_as_harness` (5 cases; red on the unchanged two-entry list, then green); the census gate `! (grep -rl --include='*.rs' 'cross-session-message' crates src tests | grep -vx 'crates/viola-agent-claude/src/hook.rs')` green; CI run 36583175440 on `055adf4`, 15/15 success.
- **By:** the founder (live ratification, relay: the Viola overseer); recorded by `/andromeda-wrap-session` P2.
