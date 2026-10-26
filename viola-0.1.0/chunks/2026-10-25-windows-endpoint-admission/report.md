# Report — 2026-10-25-windows-endpoint-admission

**Chunk:** the Windows pipe admits only the owning user and SYSTEM, rejects remote clients and keeps first-instance, each asserted on the Windows leg
**Date:** 2026-10-26
**Commits:** since `last_wrap` 2026-10-20T20:26:43Z (`git log --since`): `1862fcfb` feat(2026-10-20-epoch-5-cleanup-ii) · `bda69648` chore(records) post-push gates row · `24eef748` chore(records) four citations re-cited for the U55 door · `e4aeb25b` chore(setup-project) U56 · `6467439b` `b8014bf4` `3b1d9e28` `d6298fbd` `d08eeada` `b6740d59` the U55 door (a11y, design, obs, security, tests, architecture split into body + category files) · `f1086a2b` chore(records) playbook rule · `e2bb59c8` chore(2026-10-25-windows-endpoint-admission): operator pre-CI commit, for the run this chunk's verdict reads. This chunk's own: `e2bb59c8` alone, on base `b6740d59` (Setup 4).

## Changes (structured — detectors read this)
- **Files:** `tests/security_negatives_channel.rs` · `Cargo.toml` · `src/main.rs` · `src/run/send/test_support.rs` · `src/run/dialog.rs` · `src/run/budget.rs` · `src/run/gate.rs` · `src/obs.rs` · `src/cmd/run.rs` · `src/cmd/hook/statusline.rs` (10 files, `git diff --stat b6740d59 -- src tests Cargo.toml`: 234 insertions, 12 deletions). Every edit is test code (`#![cfg(windows)]` integration binary, `#[cfg(test)]` modules) or a dev-dependency table: **no product file changed**, no product behaviour, wire contract, exit code, refusal word or output line moved.
- **Symbols / APIs:** all test-only, none public.
  - `tests/security_negatives_channel.rs` (added 1-5 · 119-272, the New text listing): five `#[test]` cases —
    `security_negative_viola_listener_rejects_remote_clients` (218-227): binds `viola_channel::Server::bind` on
    `\\.\pipe\viola-test-chan-<pid>-remote`, opens a client end with `CreateFileW(FILE_READ_ATTRIBUTES)`, reads
    `NtQueryInformationFile(…, FilePipeLocalInformation)` and asserts `NamedPipeType & 0x2 != 0`
    (`FILE_PIPE_REJECT_REMOTE_CLIENTS`, the literal 2); its controls on test-owned `CreateNamedPipeW` pipes,
    `security_negative_control_accept_remote_pipe_reads_the_bit_clear` (229-235, `PIPE_ACCEPT_REMOTE_CLIENTS`, bit
    clear) and `security_negative_control_reject_remote_pipe_reads_the_bit_set` (237-243, `PIPE_REJECT_REMOTE_CLIENTS`,
    bit set); `security_negative_viola_listener_denies_an_anonymous_open` (245-259): on a spawned thread binds the
    listener on `viola-test-chan-<pid>-anon`, then under `ImpersonateAnonymousToken(GetCurrentThread())` opens it
    `GENERIC_READ` and asserts `Err(5)` (`ERROR_ACCESS_DENIED`), reverting (`RevertToSelf`) before the thread ends; its
    control `security_negative_control_default_pipe_admits_an_anonymous_read` (261-272): the same anonymous open of a
    test-owned pipe with null security attributes (the default descriptor) succeeds. Helpers `wide` (120-122),
    `test_pipe` (124-126), `listen_in` (128-152), `open` (154-177), `pipe_type` (179-202), `anonymous_read` (204-216).
    Test pipe names stay in the `viola-test-chan-<pid>-<label>` namespace (labels `remote`, `accept-remote`,
    `reject-remote`, `anon`, `anon-default`).
  - `src/main.rs` `test_support::with_scoped_default<S, R>(subscriber: S, f: impl FnOnce() -> R) -> R` (277-288, head
    281): registers once per process a pinned `tracing::Dispatch` over a bare `tracing_subscriber::registry()` (no
    layer, no filter) in a `static OnceLock<tracing::Dispatch>`, never the global default, then returns
    `tracing::subscriber::with_default(subscriber, f)`. Its witness `tests::scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first`
    (639-651, head 640), with the private emitter `emit_witness_line` (634-637, one `obs_event!(INFO,
    ObsEvent::StateRecovered, detail = "torn-line-healed")` site) and a field-capture layer `Seen` (615-632).
  - Nine root-bin scoped captures now call `crate::test_support::with_scoped_default(` in place of
    `tracing::subscriber::with_default(`, nothing else changed: `src/cmd/hook/statusline.rs:375`, `src/cmd/run.rs:772-775`
    and `:912`, `src/run/gate.rs:465`, `src/run/budget.rs:859`, `src/run/send/test_support.rs:421` (`capture_lines`,
    case_4's capture), `src/run/dialog.rs:902` and `:1270-1272`, `src/obs.rs:624` (coordinates from the New text
    listing). `src/run/gate.rs:429`'s `set_global_default` (the binary's one global install) is untouched. Census:
    `grep -rn 'subscriber::with_default' src | wc -l` reads `1`, the helper's own (`src/main.rs:287`).
  - Remaining callers: `viola_channel::Server::bind` keeps its 22 call sites (research.md, Graph impact); the new cases
    add two test callers in `tests/security_negatives_channel.rs`; no signature changed.
- **Crates / modules:** none added or removed. The root package's test code only.
- **Dependencies:** none added or bumped; `Cargo.lock` unchanged. A NEW root dev-dependency table
  `[target.'cfg(windows)'.dev-dependencies] windows-sys = { workspace = true, features = ["Wdk_Storage_FileSystem", "Win32_System_IO"] }`
  (`Cargo.toml` 168-171) — two features on the pinned windows-sys `=0.61.2`, not one as the plan's step 5 wrote
  (Spec claims disproved below). Both features already reached the Windows graph by unification (`cargo tree -e features
  -i windows-sys@0.61.2 --target x86_64-pc-windows-msvc`: `Wdk_Storage_FileSystem` from mio 1.2.4, `Win32_System_IO` from
  interprocess 2.4.4, mio 1.2.4 and socket2 0.6.5), so the lock does not move; under `resolver = "3"` a dev-dependency's
  feature reaches no release build. `cargo deny check bans licenses sources` green; `bash scripts/channel-tokio-free.sh`
  green.
- **Schema / config:** none.
- **Spec-master edits:** none (this chunk edited no master; the wrap's P2 owns the amendments below).
- **Retired:** none.
- **Counts / qualifiers moved:**
  - The root bin's scoped captures through `subscriber::with_default`: 9 → 0, the helper's own 1 left (`grep -rn
    'subscriber::with_default' src | wc -l`, 9 at `b6740d59` → 1). Pattern `scoped-cap`: no master site (control
    unfired — the refusal line below).
  - Windows listener admission controls asserted on CI: remote-client rejection and the DACL's effect, 0 → asserted on
    `test (windows-2025)` (ci#40247500584). Patterns `remote-rej`, `dacl-effect`, `untestable`.
  - Sweep reach: 5 patterns · 11 moved facts, 11 with a pattern (the report's own count: `wsys-dev`, `remote-rej`,
    `dacl-effect`, `untestable`, `neg-file`, `partly`, `scoped-cap`, `case4`, `plain-test`, `tokio-adv`, `crate-caps`) ·
    owner pattern 1 standing · by line 0 lines · 0 offsets (`cascade.py sweep --pre-pass`, baseline `e2bb59c8`; the
    `total` line `total (5 patterns) · 12 rows over 9 files`).
  - Zeros, by the tool's refusal line (first firing): `control: wsys-dev, partly, scoped-cap, case4, plain-test,
    tokio-adv, crate-caps, owner-gen never fired over the seven masters' pre-pass text, bodies and sets (e2bb59c8)` —
    no master or registry text names the windows-sys dev-dependency features, a "partly landed" status, the scoped
    capture, case_4, the plain-`cargo test` reds, the tokio advisory, the crate captures or a generic "owned by a route
    entry" phrase.
  - The sweep's standing rows, each dispositioned:
    - `remote-rej` (3 standing): `.andromeda/security-plan-detail/authentication-authorization.md:15` ×2 (`Keep
      accept_remote(false) (the default, PIPE_REJECT_REMOTE_CLIENTS)`) — true, now asserted: the Windows clause gains its
      witness; `.andromeda/test-plan.md:46` (`the cross-user parts (… remote pipe clients, the Windows pipe DACL) cannot
      be driven on hosted runners`) — stale for remote pipe clients and the DACL; `.andromeda/test-plan-detail/test-scope-summary.md:11`
      ×2 (`rejecting remote pipe clients, and denying access through the Windows pipe DACL … cannot be driven on hosted
      GitHub runners`) — stale for those two.
    - `dacl-effect` (2 standing): `.andromeda/security-plan.md:434` (`The default DACL grants read to Everyone and
      anonymous`) — true, measured by the control `security_negative_control_default_pipe_admits_an_anonymous_read` on
      windows-2025; `test-scope-summary.md:11` — stale (as above).
    - `untestable` (2 standing): `.andromeda/test-plan.md:86` (`- **Untestable zones:**`) — label, no site (its items,
      :87-90, name a second local user, ACL-less volumes and macOS local runs — none moved); `test-scope-summary.md:11` —
      stale (as above).
    - `neg-file` (1 standing): `.andromeda/architecture.md:261` (test-only endpoints `viola-test-chan-<pid>-<label>`,
      created by … `tests/security_negatives_channel.rs` …) — true: the five new cases' pipes are in that namespace.
    - `owner-title` (1 standing): `.andromeda/architecture-detail/established-decisions.md:125` («"Explicit session
      links", after "Windows endpoint admission" and "Server verification before any frame"») — true: the tokio entry's
      insertion after this one leaves that order.
    - Leaf rows (`remote-rej`, 3): `.claude/rules/security.md:10`, `.claude/docs/security-summary.md:14`,
      `.claude/docs/services/viola-channel.md:22` — `accept_remote(false)` restated, true; P2's cascade owns them.
- **Dev-tool versions:** none — cargo-nextest, rustc 1.98.1 and cargo-deny re-read unchanged on the dev host.
- **Harness / gate surface:** none changed (no script, xtask, CI step or status shape edited). The new Windows cases
  run inside CI's existing `test (windows-2025)` instrumented nextest run (the `coverage` suite); the JUnit artifact
  `junit-windows-2025` carries them.
- **Cross-project / external claims:**
  - CI run read: **ci#40247500584**, sha `e2bb59c8973d` (the pre-CI commit; this wrap's commit adds no source to it),
    `test (windows-2025)` `verdict: green`; full row `verdict: red · checks 15/15`, its one red `supply-chain`
    (`unsound` RUSTSEC-2026-0386, tokio 1.53.1) — `chunks/…/evidence/operator-pass.md`.
  - The `junit-windows-2025` artifact of that run (downloaded to a scratch dir): `nextest-run` 2988 tests, 0 failures,
    every case this chunk names `passed`.
  - The pinned crate sources read for the mechanism and the feature gate: tracing-core 0.1.36 / tracing 0.1.44
    (research.md, P4 additions), windows-sys 0.61.2 `src/Windows/Wdk/Storage/FileSystem/mod.rs` (`NtQueryInformationFile`
    under `#[cfg(feature = "Win32_System_IO")]`).
  - Inputs (`inputs.py verify`, P1): `inputs: 8 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
    unreachable 0 · n/a 6 · uncited 3 · unparsed 0` — no DRIFT.
    - I1 · `../additional/viola-overseer/phase-windows-endpoint-directive.md` · copy no-repo · unchanged — the phase directive.
    - I2 · message (the overseer, founder-delegated, via the operator, 2026-10-25T21:10Z) · copy message · n/a — the
      supply-chain red's owner, a new route entry "tokio RUSTSEC-2026-0386 lock bump", no ignore widened.
    - I3 · message (the operator, 2026-10-25T21:08Z) · copy message · n/a — the phase invocation.
    - I4 · message (the operator, 2026-10-25T23:32Z) · copy message · n/a — the P4 dialog re-asked live.
    - I5 · message (the founder, live by dialog in the overseer session, 2026-10-25T23:34Z) · copy message · n/a — the
      four P4 forks: (a) the pipe-type read-back, (c) the anonymous-token open, case_4 at its root cause, the three
      plain-run reds recorded with no owner (plain `cargo test` unsupported).
    - I6 · message (the founder, live by dialog in the overseer session, 2026-10-25T23:46Z) · copy message · n/a — cited
      here: the founder's own yes on the P5 review card `.andromeda/runs/2026-10-25T21-08-26-phase/p5-card-1.md`, the
      whole card including both Beyond-the-plan rows. Its `## Beyond the plan` rows, verbatim, with the founder's own
      authority:
      - **Route insertion (wrap P5, route-contract §Write row 17).** A new working-route entry, "tokio RUSTSEC-2026-0386 lock
        bump", placed right after "Windows endpoint admission" so it runs next. It owns the `supply-chain` red: a lock bump
        of `tokio` to `>=1.53.3`, no ignore-list widening. This is the overseer's founder-delegated answer at this P1
        (`inputs#I2`).
      - **Same-class captures left out, no owner.** The crate-level scoped captures `crates/viola-state/src/lib.rs:87`,
        `crates/viola-channel/src/test_support.rs:184` and `crates/viola-channel/src/lib.rs:199` share the mechanism. They
        have never been observed red, so no route entry is proposed. They are recorded in the plan and in the wrap's report.
    - I7 · `../additional/viola-overseer/wrap-windows-endpoint-directive.md` · copy no-repo · unchanged — cited here: the
      operator's wrap directive (overseer, founder-delegated): forced sample at P2 (`detail.py sample --side wrap
      --force`); keep every transcript and the three run dirs intact; a hard cascade failure stops the wrap; report at
      P2 validated (check's first-try lines per kind, the compare headers, the tally, the run dir); carry out the two
      founder-approved card items; the architecture amendment names both windows-sys features; run whole.
    - I8 · message (the operator, this session, 2026-10-26 00:17Z) · copy message · n/a — cited here: the wrap
      invocation, «wrap … whole; at P2 the operator's word is forced sample».
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - test-plan §1 Test Scope Summary, the `viola-channel` entity (`.andromeda/test-plan.md:46`,
    `.andromeda/test-plan-detail/test-scope-summary.md:11`): «rejecting remote pipe clients, and denying access through
    the Windows pipe DACL … cannot be driven on hosted GitHub runners». Measured false for those two on CI's
    `test (windows-2025)` (ci#40247500584, JUnit `passed`): the remote-client rejection is witnessed by the pipe-type
    read-back with an accept-remote and a reject-remote control telling the bit apart, and the DACL's denial by an
    anonymous-token open denied on the viola listener beside the same open admitted on a default-descriptor pipe. The
    rest of the clause (a peer with a different euid; a second OS account) stands. Patterns `remote-rej`, `dacl-effect`,
    `untestable`.
  - The plan's step 5 and research.md P4 additions: «`Wdk_Storage_FileSystem` … no new feature is needed» for the
    pipe-type read-back — measured false: windows-sys 0.61.2 gates `NtQueryInformationFile` on `Win32_System_IO` too
    (its `IO_STATUS_BLOCK` argument). The landed dev-dependency declares both. No master site (pattern `wsys-dev`, control
    unfired) — a chunk-artifact claim.
  - The folded CARRY's mechanism («its log capture found no `send-refused` line» under libtest threads): not reproduced
    at P3 (case_4 ok 3/3) and now explained and witnessed — the loss is a callsite first registered on a
    subscriber-less thread while the scoped capture is the only registered dispatcher (tracing-core 0.1.36), reproduced
    deterministically by the witness under nextest (red, capture `[]`) and removed by the helper (green). Pattern `case4`:
    no master site.
- **Expected amendments (from plan):**
  - test-plan §1 Test Scope Summary (the `viola-channel` untestable zone) — carried: Spec claims disproved, first bullet;
    sites `remote-rej`/`dacl-effect`/`untestable` → `.andromeda/test-plan.md:46` (1 hit) and its key/category file
    `.andromeda/test-plan-detail/test-scope-summary.md:11` (×2 `remote-rej`, 1 `dacl-effect`, 1 `untestable`).
  - test-plan §6 Security control negatives: the two Windows listener negatives and their controls named — carried:
    Symbols / APIs, the first bullet; pattern `neg-file` reads 0 sites in test-plan (its 1 hit is architecture.md:261,
    the test-only endpoint registry), so §6's naming is a new fact for the test-plan detector to place.
  - test-plan §4 Unit Test Strategy: the root bin's scoped captures go through `test_support::with_scoped_default` —
    carried: Symbols / APIs, second and third bullets; pattern `scoped-cap`: 0 master sites (control unfired), a new fact.
  - architecture §Infrastructure Patterns → Crate dependency direction: the root package's Windows dev-dependency
    features — carried: Dependencies (both `Wdk_Storage_FileSystem` and `Win32_System_IO`, per `inputs#I7` item 6);
    pattern `wsys-dev`: 0 master sites (control unfired), a new fact.
  - security-plan §Authentication & Authorization (IPC access control (Windows)): the remote-client rejection and the
    DACL's effect asserted on the Windows leg, as landed — carried: Counts / qualifiers moved, second bullet, and Spec
    claims disproved; pattern `remote-rej` → `.andromeda/security-plan-detail/authentication-authorization.md:15` (×2);
    pattern `partly` (the plan's «partly landed»): 0 master sites (control unfired) — no master states the status in
    those words.
  - `matrix#v1-44` notes — already written at phase P5 (`git diff b6740d59 -- viola-0.1.0/verification-matrix.json`: one
    `notes` field added to `v1-44`, «2026-10-25 (2026-10-25-windows-endpoint-admission, phase P5): not claimed — …»);
    no master site — a ledger entry; nothing owed at P7.3.
  - Curation candidate (wrap's, not a master amendment): `.claude/rules/testing.md` 2026-09-27 — carried to P3 under
    Decisions & corrections.
- **Coverage of new surfaces:**
  - `security_negative_viola_listener_rejects_remote_clients` + 2 controls → validation n/a (test) · instrumentation n/a ·
    PII n/a · tests integ (root `security_negatives_*`, Windows only, CI `test (windows-2025)`) · a11y n/a · tokens n/a
  - `security_negative_viola_listener_denies_an_anonymous_open` + 1 control → validation n/a · instrumentation n/a · PII
    n/a · tests integ (Windows only, CI `test (windows-2025)`) · a11y n/a · tokens n/a
  - `test_support::with_scoped_default` (test helper) → validation n/a · instrumentation n/a (it routes captures; every
    captured line still goes through `obs_event!`) · PII n/a · tests unit (the witness, every OS) · a11y n/a · tokens n/a

## Deviations from intent
- **Step 5 declares two windows-sys features, not one.** `NtQueryInformationFile` is behind `Win32_System_IO` as well as
  the module's `Wdk_Storage_FileSystem`; the plan's one-feature entry would have compiled only through unification from
  mio and interprocess. Same listed file; the lock and the release graph unchanged (Dependencies). Justification: the
  test names `IO_STATUS_BLOCK` directly, so the entry declares what it uses.
- **The anonymous cases bind before impersonating, on one spawned thread** (`anonymous_read` impersonates, opens and
  reverts in one call), as step 7 orders it; the default-pipe control's pipe also carries `PIPE_REJECT_REMOTE_CLIENTS`,
  so its only difference from the viola listener is the descriptor.
- **The remote-client read-back reads a client end** (`CreateFileW`, `FILE_READ_ATTRIBUTES`) of all three pipes, one
  instrument for the subject and both controls; the controls decided on CI that NPFS reports the bit there.
- **P3 smoke not re-driven:** no product file changed (no boot path moved); the plan's smoke entries ran green as P2
  gates.
- scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 10 · listed 10 · recorded 0
  (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 441`).

## Decisions & corrections
- The operator's word at implement's invocation (2026-10-25T23:49Z, this session): run the chunk through its operator
  pass and the CI read; `supply-chain` red expected; the `windows-2025` leg is the witness; journal each long run in the
  run dir before waiting on it; keep every subagent and workflow transcript (none was spawned at implement). Evidence:
  `.andromeda/runs/2026-10-25T23-49-29-implement/journal.md`.
- The operator pass was made by the session on that word: hygiene clean → `pre-push` re-run green (100.13 s) → the
  pre-CI commit `e2bb59c8` → the push through `gate.py run --operator 22` (`history moved:
  refs/remotes/origin/build/viola-0.1.0 b6740d59→e2bb59c8`) → the two CI reads fired as written.
- The wrap directive (`inputs#I7`) and invocation (`inputs#I8`): forced sample, keep the three run dirs and every
  transcript, a hard cascade failure stops the wrap, carry out the card's two items, both windows-sys features named.
- The founder's P4 forks (`inputs#I5`) and P5 yes (`inputs#I6`) — the card's two Beyond-the-plan rows above.
- Learning candidate (testing.md 2026-09-27 extension): a scoped tracing subscriber installed as the only dispatcher
  also loses an ON-thread line whose callsite another, subscriber-less thread registered first; the root bin's
  `test_support::with_scoped_default` keeps a second, always-interested dispatcher registered so no scoped capture is
  ever the only one. Evidence: `evidence/capture-witness.md` (red `left: []` → green `1 test run: 1 passed`).
- Learning candidate (host / crate facts): windows-sys gates a function on every feature its signature's types need,
  not only its own module's — `NtQueryInformationFile` (Wdk module) needs `Win32_System_IO` for `IO_STATUS_BLOCK`; a
  feature can compile by unification from another crate and still be undeclared. Evidence: windows-sys 0.61.2
  `FileSystem/mod.rs`, `cargo tree -e features`.
- Learning candidate (Windows test design): an anonymous-token open test binds its pipe BEFORE `ImpersonateAnonymousToken`
  on the same thread and reverts inside the same call — creating the listener under the anonymous token would build the
  owner-only descriptor from the wrong token. Evidence: `tests/security_negatives_channel.rs` `anonymous_read`, green on
  windows-2025.
- Hook refusal: the `stamp-ahead` PostToolUse hook refused an implement journal line stamped 23:58Z at a 23:51Z clock;
  re-read `date -u`, corrected (standing rule: a stamp comes from the clock).
- Guard refusal: the session's Bash guard refused a `cd` into a skill references dir (`moves the session cwd`);
  re-issued with absolute paths (a standing rule in `host-linux.md`, recurring).
- Sweep hazard: the site sweep's patterns for the windows-sys features, the scoped capture, case_4, plain `cargo test`,
  the tokio advisory and the crate captures read 0 master sites — each is a NEW fact for the masters, not an absence of
  a stale one.

## Outcome
- Acceptance, re-asserted against the diff:
  - (security) remote-client rejection on `test (windows-2025)`: **met** — `security_negative_viola_listener_rejects_remote_clients`
    `passed`, its two controls `passed` (bit clear on accept-remote, set on reject-remote): the controls tell the two
    apart, so nothing is owed to "Windows-only live measurements" (ci#40247500584).
  - (security) anonymous-token open denied with `ERROR_ACCESS_DENIED`: **met** — `security_negative_viola_listener_denies_an_anonymous_open`
    `passed`; the control `security_negative_control_default_pipe_admits_an_anonymous_read` `passed` (ci#40247500584).
  - (security) DACL text read-back, non-inheritance, first-instance and `squatted-name`: **met**, unedited — on windows-2025
    `bound_pipe_dacl_reads_back_protected_user_and_system`, `owner_only_sddl_is_protected_user_and_system`,
    `pipe_handles_on_both_ends_are_not_inheritable`, `bind_of_a_live_endpoint_is_taken_until_it_drops`,
    `channel_endpoint_second_bind_is_taken_while_the_first_lives`, `run_refuses_a_squatted_endpoint`,
    `path1_run_start_one_home_in_order` all `passed`; `test (ubuntu-latest)` and `test (macos-latest)` `success`.
  - (tests) the capture witness red on a bare `with_default`, green on `with_scoped_default`: **met** (`evidence/capture-witness.md`).
  - (tests) the witness by name under nextest reads `1 test run: 1 passed`: **met** (gate entry, green).
  - (tests) plain `cargo test --bin viola`: case_4 `ok` and the witness `ok`: **met**; the run reads `FAILED. 1037 passed;
    2 failed` on two of the three known-cause reds (`run::gate::tests::feed_panic_writes_one_parse_rejected_line_until_the_size_changes`,
    `obs::tests::log_config_rejection_writes_one_warn_line`; `case_2_sigterm` ok this run), recorded with no owner
    (`evidence/plain-run.md`, `inputs#I5`).
  - (tests) the census `grep -rn 'subscriber::with_default' src | wc -l` reads `1`: **met**.
  - (tests) `run --unit` `"ok":true`, `pre-push` `"ok":true` at `linux-tests`: **met**.
  - (arch) `cargo deny check bans licenses sources` and the tokio-free tree green; the features on a root Windows
    dev-dependency only, no ignore added: **met** — two features where the criterion named one (Deviations; the
    criterion's substance — a dev-dependency only, no lock move, no ignore — holds).
  - (obs) the witness's line and every capture's lines through `obs_event!`; G2 zero panics; G4 conformant: **met**.
  - (design · layouts · a11y) no human output, refusal text or SGR byte changed: **met** — no product file edited.
  - (CI) `ci.py conclusion --sha HEAD --name 'test (windows-2025)'` `verdict: green`, run id named; the full row recorded,
    its only red `supply-chain`: **met** — ci#40247500584.
- Gates (implement's first full firing, trail `.andromeda/runs/2026-10-25T23-49-29-implement/gate-2026-10-25-windows-endpoint-admission.json`,
  `entries 24 · green 19 · red 0 · recorded 1 · timeout 0 · not-run 4`):
  - `df -h /tmp` — recorded (26G avail; `evidence/tmp-reading.md`)
  - `cargo fmt --all --check` — green
  - `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green
  - `CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc -- -D warnings` — green (checked `viola v0.1.0 (.)`)
  - `rg -n -U --pcre2 --type rust '#\[(tracing::)?instrument\b(?!\(\s*skip_all\b)' .` — green (exit 1, no output)
  - `bash scripts/lint-probes.sh` — green
  - `bash scripts/agent-run.sh run --unit` — green (`"ok":true`)
  - `cargo nextest run … -E 'test(/(^|::)tests::scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first$/)'` — green (`1 test run: 1 passed`); its first firing red (exit 100), the step-2 control
  - `grep -rn 'subscriber::with_default' src | wc -l` — green (`last line 1`)
  - `cargo test --bin viola` — green by its two `contains … ok` atoms (exit 101, no exit atom)
  - `cargo deny check bans licenses sources` — green
  - `bash scripts/channel-tokio-free.sh` — green
  - `gate.py guard … --base b6740d59…` — green (`guard: clean`, allowed 10, excluded 0)
  - `bash scripts/agent-run.sh cleanup --session p-wea-smoke` — green
  - `bash scripts/agent-run.sh boot --session p-wea-smoke --instance builder` — green
  - `bash scripts/agent-run.sh status --session p-wea-smoke` — green
  - `bash scripts/g2-zero-panics.sh` — green
  - `bash scripts/agent-run.sh schema-check` — green
  - `bash scripts/agent-run.sh cleanup --session p-wea-smoke` — green (`processes_gone`, `endpoint_gone` true)
  - `bash scripts/agent-run.sh pre-push` — green (114.8 s; re-run in the operator pass, green 100.13 s)
  - `gate.py hygiene` (operator) — `hygiene: clean — read 427` (`evidence/operator-pass.md`)
  - `git diff --quiet && git diff --cached --quiet && git push origin HEAD` (operator) — green, `history moved:
    refs/remotes/origin/build/viola-0.1.0 b6740d59→e2bb59c8`
  - `ci.py conclusion --sha HEAD --wait 2400 --name 'test (windows-2025)'` (operator) — `verdict: green` · ci#40247500584
  - `ci.py conclusion --sha HEAD --wait 2400` (operator, report-only) — recorded: `verdict: red · checks 15/15`, `failed 1:
    supply-chain` → `red — not this chunk's: every run since the last wrap's flip fails supply-chain alone on
    RUSTSEC-2026-0386 against the unchanged tokio 1.53.1 (ci#40208779387 at b6740d59, the parent, before this chunk's
    edits; scope.md, CI since the last wrap) → the route entry "tokio RUSTSEC-2026-0386 lock bump" (inputs#I2, inputs#I6)`;
    the other 14 checks `success`.
  - No deferral (rust delta 10 files). No live leg. Smoke: the plan's smoke entries green in-block; no boot path moved.
- Watches: the macOS readiness stall (`wrapper <name> not ready`, `process-start{subject:"self"}` and nothing more) · 1
  green run [ci#40247500584 `test (macos-latest)` success] this chunk (from 2/3 at the last wrap) · not recurred.
- Outcome basis: this window holds implement's run whole (`.andromeda/runs/2026-10-25T23-49-29-implement/`, its
  `outcome-2026-10-25-windows-endpoint-admission.md`) and the operator pass; Setup 4's commit list is `e2bb59c8` alone;
  the final HEAD's CI run is ci#40247500584, recorded in `chunks/…/evidence/operator-pass.md`.
- Process hygiene: implement's census — the smoke wrapper and fake agent terminated (`processes_gone:true`), no
  viola-tree process left; two `cargo-nextest` processes of another project's build left running, not this run's.
  Re-measured here (`gate.py census --comm fake-agent --args projects/viola/target/debug/viola --args p-wea-smoke`,
  this run dir): `census: none running`.
## Addendum — resumed 2026-10-26
- inputs#I9 · message (the operator, this session, 2026-10-26 01:26Z) · copy message · n/a — the halt's answer: option 2; fanout v7 (f8827d5b, F350) re-spawns from the batch's rendered manifest; the wrap resumes in this run dir at the newdetail re-spawn, then compare, tally, Validate, Apply and the entry batches; nothing earlier re-runs; the directive (inputs#I7) still stands
  - no fact above rests on it
- inputs.py verify, re-fired: inputs: 9 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 7 · uncited 1 · unparsed 0; moved since the report: uncited 3 → 1 (I6, I7, I8 cited by the report; I9 cited here)

## Addendum — resumed 2026-10-26 (second halt)
- inputs#I10 · ../additional/viola-overseer/wrap-windows-endpoint-ruling-f351.md · copy no-repo · unchanged — overseer1's operator ruling F351: the architecture apply writer's malformed label in the key file Crate dependency direction is fixed by the orchestrator in exactly two writes (the label line; the subject line), then the three read-backs; no writer can correct a label it wrote
  - no fact above rests on it
- inputs#I11 · message (the operator, this session, 2026-10-26 01:41Z) · copy message · n/a — option 1 on that ruling; stop again if a read-back fails
  - no fact above rests on it
- inputs.py verify, re-fired: inputs: 11 entries — unchanged 3 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 8 · uncited 2 · unparsed 0; moved since the first addendum: uncited 1 → 2 → 0 (I10, I11 cited here)

## New text, by line
Generated by `cites.py added` (cites v1.7); pasted by `splice.py`. No line of this section is typed or edited.
The diff: b6740d59 (the parent of the oldest pre-CI commit e2bb59c8) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### Cargo.toml — added 4 line(s) in 1 range(s)
added: 168-171
### src/cmd/hook/statusline.rs — added 1 line(s) in 1 range(s)
added: 375
### src/cmd/run.rs — added 5 line(s) in 2 range(s)
added: 772-775 · 912
### src/main.rs — added 57 line(s) in 3 range(s)
added: 273-274 · 277-289 · 610-651
  - 277-288 @281 «pub(crate) fn with_scoped_default<S, R>(subscriber: S, f: impl FnOnce() -> R) -> R»
  - 615-622 «impl tracing::field::Visit for Seen {»
  - 624-632 «impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for Seen {»
  - 634-637 @635 «fn emit_witness_line() {»
  - 639-651 @640 «fn scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first() {»
### src/obs.rs — added 1 line(s) in 1 range(s)
added: 624
### src/run/budget.rs — added 1 line(s) in 1 range(s)
added: 859
### src/run/dialog.rs — added 4 line(s) in 2 range(s)
added: 902 · 1270-1272
### src/run/gate.rs — added 1 line(s) in 1 range(s)
added: 465
### src/run/send/test_support.rs — added 1 line(s) in 1 range(s)
added: 421
### tests/security_negatives_channel.rs — added 159 line(s) in 2 range(s)
added: 1-5 · 119-272
- 120-122 «fn wide(path: &str) -> Vec<u16> {»
- 124-126 «fn test_pipe(label: &str) -> String {»
- 128-152 @129 «fn listen_in(path: &str, mode: u32) -> OwnedHandle {»
  - 131-143 @132 «let handle = unsafe {»
  - 144-149 «assert_ne!(»
- 154-177 @155 «fn open(path: &str, access: u32) -> Result<OwnedHandle, u32> {»
  - 158-170 @160 «let handle = unsafe {»
  - 171-174 «if handle == INVALID_HANDLE_VALUE {»
- 179-202 @180 «fn pipe_type(path: &str) -> u32 {»
  - 181-183 «use windows_sys::Wdk::Storage::FileSystem::{»
  - 190-199 @191 «let nt = unsafe {»
- 204-216 @206 «fn anonymous_read(path: &str) -> Result<(), u32> {»
- 218-227 @219 «fn security_negative_viola_listener_rejects_remote_clients() {»
  - 222-226 «assert_ne!(»
- 229-235 @230 «fn security_negative_control_accept_remote_pipe_reads_the_bit_clear() {»
- 237-243 @238 «fn security_negative_control_reject_remote_pipe_reads_the_bit_set() {»
- 245-259 @246 «fn security_negative_viola_listener_denies_an_anonymous_open() {»
  - 247-251 «let opened = thread::spawn(|| {»
  - 254-258 «assert_eq!(»
- 261-272 @262 «fn security_negative_control_default_pipe_admits_an_anonymous_read() {»
  - 264-268 «let opened = thread::spawn(|| {»
