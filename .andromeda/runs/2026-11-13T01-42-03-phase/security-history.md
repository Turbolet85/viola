@history_item n=1 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — trust boundary, CI jobs, deny.toml additions, nightly.yml: made the ubuntu `supply-chain` job and the weekly uncached `nightly.yml` advisory run what they are now. That is the job this chunk must turn green, and nightly re-runs the same advisories on both manifests, so a fix must hold on the scheduled run as well as on the push.

@history_item n=2 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — fuzz lockfile audit wired: the `supply-chain` job also runs `cargo deny --manifest-path fuzz/Cargo.toml check advisories sources`, and so does nightly. A green job needs the fuzz lockfile clean too, so check whether `rustix 1.1.5` also sits in `fuzz/Cargo.lock` (decision/cargo-deny-gate's fuzz exemption).

@history_item n=3 marker=2026-09-25-security-prerequisites
**2026-09-25-security-prerequisites** — 0BSD per-crate exceptions, never through `allow`; each further exception needs its own entry. An rmcp bump past `<3.5` or a rustix bump can pull in a new licence, and decision/licence-allow-list forbids widening `allow`.

@history_item n=4 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — the `FAKE_AGENT_PUMP_DELAY_MS` seam carve-out (fake-agent only, compiled out of release). A footer-notice replay in the fake agent's screens must not add another env seam without its own Decisions Log entry (decision/test-seams-fake-agent-only, decision/no-control-switch-off).

@history_item n=5 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — release-check refuses `test-support` / `fake-agent` artifacts (probe 5/5). Any fake-agent screen change for the footer notice must stay behind the test-only feature (decision/release-build-without-test-features, decision/release-build-no-test-binaries).

@history_item n=6 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — the `FAKE_AGENT_HOOK_PANIC` seam, ratified by the founder, and the rule that another seam needs its own entry. This is the second admitted seam (decision/fake-agent-hook-panic-seam), and the precedent if the fake agent needs a new knob to show a transient footer notice.

@history_item n=7 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the hidden `hook --capture` arm, probe captures, fixture scrub; `MAX_FRAME` consumers and verify's CI split) — set the capture arm (decision/hook-capture-arm), the transient 0600 probe captures, the `--record` scrub and refusal, and "the real CLI runs only locally; verify runs in CI only against the fake agent". The proof run on the newest CLI is local-only, and CI replays fixtures.

@history_item n=8 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — vt100 feed degrade: the feed runs under `catch_unwind`, and a poisoned model reads `input-not-ready`. Any new input-box reading in `turns.rs` reads the same screen model, so it must treat a poisoned screen as not-ready (decision/vt100-feed-under-panic-catch, decision/vt100-inside-catch-unwind).

@history_item n=9 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (send's input validation; the feed bound closed) — set the bounded 256-message feed queue and the one-write bracketed paste with `validate_paste_text`. These are the vt100-feed-bounded-queue and send-text-is-one-bracketed-paste rows. The run pump's gate is out of scope unless it shares the literal.

@history_item n=10 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (run's strict stamps read) — `run`'s version gate reads `ledger/stamps.json` through the strict check, and a non-null decision flows only on a stamp covering the relied rows. The proof's stamps, written by verify, are what switch on dialog answering for the new CLI version (decision/version-gate-strict-stamps-read, decision/dialog-answers-need-verify-stamp).

@history_item n=11 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — Run B's typed-input PTY probe: a direct spawn under the R8 strip at 80×24, output capped at `MAX_FRAME` into a Screen under `catch_unwind`, read only against compiled literals. It sends no byte into any dialog, and a modal start is killed with no key. The entry also set the 0700 probe dirs, the named fixture refusal (`home-path` · `absolute-path` · `username` · `email`) and the transcript residual. This is the founder-ratified envelope a readiness change must stay inside.

@history_item n=12 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — Runs C and D join; the stamp grows; no key is typed into any dialog. The proof's all-pass reading spans these runs too (decision/verify-probes-confined-to-own-dir).

@history_item n=13 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B's added pastes each go only into rows that hold the input-box literal and no modal literal. After the long and tag-like turns Run B waits up to `PROBE_DEADLINE` (120 s), and a modal, a poisoned screen or the deadline ends the pastes with no key. This is the exact readiness rule this chunk rewrites, ratified by the founder as a boundary widening. Moving it off a footer literal changes what was shown to him, so weigh whether the change is a re-showing.

@history_item n=14 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — standing rule: a scratch measurement's answers are spent, and another probe needs its own showing. Any live probe of the footer notice is a new crossing that needs its own answer (scope: named operator steps under the founder's cap).

@history_item n=15 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (trailing CR and LF removed; empty typed text refused) — keeps "NEVER refuse LF, CR or TAB" (decision/never-refuse-line-breaks). It matters for the newline paste Run B types and for any pump-side send change, should the run pump's gate turn out to share the literal.

@history_item n=16 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — probe dirs are removed when verify passes, fails or is stopped by SIGINT/TERM, and guards remove only what their own run created. A stopped Run B is killed with no key and pastes nothing more. A new readiness wait in `box_wait` or `framing_turns` must keep reading the stop flag (decision/verify-probe-dirs-cleanup, decision/probe-artifacts-cleanup).

@history_item n=17 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — Run B's fifth compiled paste (`PROBE_NEWLINE_PASTE`), each added paste only into rows holding the input-box literal and no modal literal, plus the stamp counter rule (an older id set reads `cli_verified:false`). A readiness rewrite touches this paste's gate, and a new ledger row for the input box changes the stamp's relied set.

@history_item n=18 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — Run B's probe `--settings` and the statusline capture arm; `statusline-rate-limits` among the relied rows; the kept `workspace.repo` in fixtures. Run B's status line is the probe's own, which bears on what footer the probe sees. The fixture scrub's one exception stands (decision/fixture-scrub-and-refusal).

@history_item n=19 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E joins the verify child set, after Run B and using Run B's first SessionStart id. The proof's every-row-pass reading includes `resume-session`, and Run B must still reach its first turn.

@history_item n=20 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — Run B makes one bounded `claude agents --json` read once its first turn has settled, before the framing pastes. The "settled" point that a readiness change redefines also orders this read (decision/child-output-bounded, decision/child-output-tolerant-unknown).

@history_item n=21 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — Run B's sixth paste `/compact`. The five added pastes go only into the settled input box, `/compact`'s answer is waited up to `PROBE_DEADLINE`, and the stamp is twenty-four rows. A `/compact` turn is exactly where a transient footer notice may displace the literal.

@history_item n=22 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** (verify's print-mode probe residual) — accepted into the trusted-runs class as measured on 2.1.296 and 2.1.302. A proof on a newer CLI is covered for the known residual only, and a new residual class would need its own answer (decision/verify-real-cli-transcripts-accepted).

@history_item n=23 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (scratch footer reading accepted; Run B's pastes go into rows holding an input-box literal) — the compiled set gained a second input-box literal, the 2.1.302 resting footer `(shift+tab to cycle)`. This is the footer literal V178 says a notice displaces. The scratch footer reading was one start, no standing permission (decision/scratch-footer-reading-residual), so another footer reading needs its own answer.

@history_item n=24 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** (the founder's Epoch 5 batch) — Run B's probe `--settings`, its agents read, its newline and `/compact` pastes, the compaction residual and Run E stand as the founder's own word. These are the ratified baselines this chunk's readiness change rests on.

@history_item n=25 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the `Mcp.` exchange recorded by the test-only tap; Run E re-cited) — the fixture refusal codes are shared with the MCP tap, and the Runs A–E live in `src/cmd/verify/typed/runs.rs`, beside the `typed/turns.rs` this chunk edits. New recorded screens must pass the same scrub and refusal.

@history_item n=26 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — `[[bans.features]]` holds the rmcp ban (`transport-streamable-http-server`, `auth`) alone, and every ban is proven live. That ban is why the three rmcp advisories (OAuth client, Streamable HTTP) sit in disabled features: the ground for an ignore under decision/cve-sla-disabled-feature.

@history_item n=27 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the tokio ban's tree form and the rmcp release-graph gate) — rmcp is scoped to the release graph with exactly `server` + `transport-io`, checked by `scripts/rmcp-features.sh`, and the test-only `viola-e2e` takes `client` only. An rmcp bump must keep both checks green, and tokio and schemars stay exact pins (decision/rmcp-features-exact, decision/rmcp-feature-set, decision/no-tokio-in-sync-crates).

@history_item n=28 marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
**2026-10-26-tokio-rustsec-2026-0386-lock-bump** (viola-channel's Tokio pin; the RUSTSEC-2026-0386 supply-chain red closed) — precedent: a RustSec red on `supply-chain` was closed by an exact-pin and lock bump, `deny.toml` untouched, no ignore widened, the open-advisory item retired. It is the model for the `rustix` lockfile bump, and the contrast case for an rmcp ignore.

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (`ledger/stamps.json` checked before `ledger/` is made, and verify's home tree before its log; the home tree at run's start) — verify runs `check_home_tree` before its `cli` log, and `update_stamps` checks before the create and again under the lock. The ban: never check after a create, mode set or log open that narrows the path. The proof's stamp write rides this order (decision/stamps-checked-under-lock, decision/strict-check-before-narrowing, decision/verify-home-tree-check).

@history_item n=30 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (`viola mcp`'s stdin capped by `CappedLines`; deny-probes at 14/14; the serde_json ban; the bounded-reader census) — the stdin cap is handed to rmcp 3.4.1's `(reader, writer)` transport because rmcp reads lines unbounded. An rmcp bump to >=3.9 must keep that transport API and the cap, keep `deny-probes` at its counted verdict, and keep the reader census (decision/bounded-readers-census) green.

@history_item n=31 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (verify's refusal codes) — verify writes closed refusal codes on its `process-exit`, `unclean-fixture` and `strict-modes-failed` among them, and a failing row is no refusal and no code. A readiness failure must stay a row outcome, not a new refusal code (contract/verify-refusal-codes/*).

@history_item n=32 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (one licence exception, one deny target triple; advisories red on RUSTSEC-2026-0441, owned by Version done-check; the relied set on every build; verify's stop) — the licence exception for `doctest-file` alone, and the `deny-sync.toml` single triple. It set the precedent of carrying an advisory red with no ignore widened, the relied set at `/24` on every build, and verify's stop as SIGINT/SIGTERM.

@history_item n=33 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (advisories green with `ignore = []`; portable-pty pinned `=0.9.0`; `statusline-rate-limits` reads the account's plan) — `ignore = []` is the current baseline, and a new ignore must follow the CVE SLA with a reason naming the disabled feature (decision/no-ignored-advisories, decision/cargo-deny-gate-clean). The pin bump by `cargo update -p` is the model for rustix. `statusline-rate-limits` is account-dependent, so the proof excepts it.

@history_item n=34 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** (the Decisions Log leaves the body) — brought `release-check.sh` and the per-entry Decisions Log rule for exceptions into the body. An rmcp ignore, or a range widening past the ratified `>=3.4.1, <3.5`, needs its record (decision/exact-dependency-pins' except).

@history_item n=35 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the colour facts join the admitted env set beside the two seams. No new env read, for example one to detect a footer notice, may be added without its own Decisions Log entry (decision/controls-not-switchable, decision/no-stored-secrets except).

@history_item n=36 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters as rows: every picked row here was made by this conversion and verified against the code at 8834195b. Twin rows (verify-signature-only / verify-reads-only-signatures, the strict-modes rows) are a single claim each.
