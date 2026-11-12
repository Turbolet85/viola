@row_none n=1 rec=1 type=S
@row_add n=1 row=decision/no-ignored-advisories area=dependency-security rec=2
name: No ignored advisories
rule: cargo-deny treats unmaintained and unsound advisories as failures, denies yanked crates and ignores no advisory; a new ignore follows the Critical CVE response SLA, its reason naming the disabled feature.
@row_add n=2 row=decision/crates-io-only area=dependency-security rec=3
name: crates.io as the only dependency source
rule: cargo-deny denies every unknown registry and every unknown git source, so dependencies come from crates.io only.
why: keeps CVE-2026-5222's sparse-registry class out of reach
@row_add n=3 row=decision/licence-allow-list area=dependency-security rec=4
name: Closed licence allow list
rule: The licence allow list stays exactly MIT, Apache-2.0, Zlib and Unicode-3.0; 0BSD is never added to the allow list.
why: keeping 0BSD out of the allow list makes any other 0BSD crate still fail the gate; ratified
guard: cargo deny check licenses
except: 0BSD admitted per crate for doctest-file, pulled in by interprocess; each new exception needs its own Decisions Log entry
@row_add n=4 row=decision/rmcp-http-server-and-auth-banned area=dependency-security rec=5
name: rmcp HTTP server and auth features banned
rule: cargo-deny denies the rmcp features transport-streamable-http-server and auth.
@row_add n=5 row=decision/serde-json-bounded-depth area=dependency-security rec=5
name: serde_json recursion limit kept
rule: cargo-deny denies the serde_json unbounded_depth feature, so every parse of external bytes keeps serde_json's default recursion limit.
guard: scripts/deny-probes.sh
@row_add n=6 row=decision/no-c-building-crates area=dependency-security rec=6
name: No C-building crates
rule: Crates that build C code are banned in the root dependency graph by cargo-deny.
guard: scripts/deny-probes.sh
@row_add n=7 row=decision/no-tokio-in-sync-crates area=dependency-security rec=6
name: No tokio in the sync crates
rule: tokio is banned from each sync crate's dependency graph, checked per sync crate as the sole root; viola-channel is covered by a cargo tree check instead.
guard: deny-sync.toml, scripts/channel-tokio-free.sh, scripts/deny-probes.sh
@row_add n=8 row=decision/fuzz-workspace-exemption area=dependency-security rec=7
name: Fuzz workspace outside the deny graph
rule: The fuzz crate is a separate test-only cargo workspace with its own committed lockfile, outside cargo-deny's root graph; its lockfile is audited separately for advisories and sources, and the C-build ban does not apply to it.
why: a ratified test-only exemption, holding only while the fuzz crate is never linked into a shipped binary and never joins the root workspace
@row_none n=2 rec=8 type=S
@row_add n=9 row=decision/exact-dependency-pins area=dependency-security rec=9
name: Exact pins for third-party crates
rule: Both lockfiles are committed, and every third-party version of the root graph and of the fuzz workspace is pinned exactly.
except: rmcp takes the minor range >=3.4.1, <3.5, and the lockfile must resolve rmcp >=3.4.1
@row_add n=10 row=decision/bytes-advisory-floor area=dependency-security rec=10
name: bytes version floor
rule: The lockfile must resolve bytes >=1.11.1.
why: RUSTSEC-2026-0007
@row_add n=11 row=decision/tracing-subscriber-floor area=dependency-security rec=10,49
name: tracing-subscriber version floor
rule: If obs selects tracing-subscriber, it must be >=0.3.20.
why: RUSTSEC-2025-0055
@row_add n=12 row=decision/notify-stays-on-8-2 area=dependency-security rec=11
name: notify held on 8.2.0
rule: notify stays on 8.2.0; no 9.0 pre-release is taken.
@row_add n=13 row=decision/product-toolchain-pin area=dependency-security rec=12
name: Product toolchain pinned exactly
rule: The product toolchain is pinned exactly to Rust 1.98.1 with rustfmt and clippy, installed through rustup on the dev host and in CI.
@row_add n=14 row=decision/msrv-floor area=dependency-security rec=12
name: Minimum supported Rust version
rule: The workspace rust-version floor is 1.96, checked in CI with the 1.96 toolchain.
guard: CI msrv job
@row_add n=15 row=decision/fuzz-nightly-pin area=dependency-security rec=12
name: Fuzz nightly toolchain pinned
rule: The fuzz workspace pins its own exact nightly toolchain, separate from the product toolchain, installed through rustup.
@row_add n=16 row=decision/cli-tool-version-floors area=dependency-security rec=13,32
name: Minimum floors for external CLI tools
rule: External CLI and audit tools are expressed as minimum version floors, not exact pins; exact pins are only for lock-pinnable library dependencies.
why: an exact pin triggers a false drift escalation when a patch-behind build is installed
@row_add n=17 row=decision/pre-push-checks-pins-installs-nothing area=dependency-security rec=14
name: pre-push checks tool pins, installs nothing
rule: viola-harness pre-push is never a new version source: it checks the host's cc, toolchain channel, cargo-nextest, cargo-mutants and cargo-llvm-cov against CI's own pins and installs nothing.
@row_add n=18 row=contract/pre-push-refusals/tool-pin-mismatch area=dependency-security rec=14
boundary: viola-harness pre-push refusals
item: tool-pin-mismatch
promise: Refused when an installed tool's version differs from its CI pin line.
@row_add n=19 row=contract/pre-push-refusals/tool-missing-cc area=dependency-security rec=14
boundary: viola-harness pre-push refusals
item: tool-missing: cc
promise: Refused when the host has no C toolchain.
@row_add n=20 row=decision/nothing-runs-as-root area=dependency-security rec=14
name: Nothing launched as root
rule: No harness command, gate or pre-push stage launches anything as uid 0, and no provisioning script exists.
@row_add n=21 row=decision/manual-dependency-updates area=dependency-security rec=15
name: Manual dependency update review
rule: Dependency updates are reviewed manually, triggered by the scheduled advisory run; no automated update bot is used.
why: no automated update bot was researched; the rmcp minor range makes the scheduled run the main signal for that crate
@row_none n=3 rec=16 type=S
@row_add n=22 row=decision/ci-cargo-deny-gate area=dependency-security rec=17
name: cargo-deny gate in CI
rule: CI runs cargo deny check over all four check families on the root lockfile and fails the build on any advisory, yanked crate or unknown source; the fuzz lockfile is audited for advisories and sources in its own step.
@row_add n=23 row=decision/scheduled-advisory-run area=dependency-security rec=18
name: Weekly scheduled advisory run
rule: A separate scheduled workflow runs cargo deny advisory checks weekly, without cache, on both the root and the fuzz manifests.
why: the advisory database changes without code changes
@row_add n=24 row=decision/scheduled-fuzzing area=dependency-security rec=18
name: Scheduled fuzzing
rule: The scheduled workflow runs each fuzz target for 120 seconds on the pinned fuzz nightly and uploads the fuzz artifacts on failure.
@row_add n=25 row=decision/no-workflow-concurrency area=dependency-security rec=19
name: No concurrency block in workflows
rule: The CI workflows carry no concurrency block; zizmor's pedantic concurrency-limits finding is declined and stays visible.
why: a concurrency group cancels pending runs even with cancel-in-progress false, which would drop a cancelled run's always() gate and upload chain
@row_add n=26 row=decision/release-build-without-test-features area=dependency-security rec=20
name: Release build free of test-only features
rule: The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent).
guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control
@row_add n=27 row=contract/release-check-output/test-only-feature area=dependency-security rec=20
boundary: release-check output
item: release-check: FAILED — test-only feature {feature} in {target}
promise: Printed when a release artifact's features include a test-only feature.
@row_add n=28 row=decision/test-support-dev-only area=dependency-security rec=20
name: test-support enabled only for development
rule: viola-channel's test-only test-support feature is enabled only by the root package's dev-dependencies.
@row_add n=29 row=decision/zizmor-workflow-gate area=dependency-security rec=21
name: zizmor workflow audit gate
rule: zizmor audits the workflows in CI and fails on unpinned actions, excessive permissions, template injection and cache poisoning.
@row_add n=30 row=decision/perf-uploads-scan-gated area=dependency-security rec=22
name: perf job uploads gated by secret scan
rule: The perf job's hyperfine exports and diagnostics are uploaded only when its own secret scan succeeds, the scan output only when it fails; every perf input is synthetic.
@row_add n=31 row=decision/g2-single-panic-exemption area=dependency-security rec=23
name: G2 zero-panics gate has one exemption
rule: CI's G2 zero-panics gate admits exactly one exemption, a role-file panic line whose panic_location equals the FAKE_AGENT_HOOK_PANIC seam's as a whole string; any further exemption needs its own Decisions Log entry.
guard: G2 --probe run before every G2 check
@row_add n=32 row=decision/actions-pinned-by-sha area=dependency-security rec=24
name: Actions pinned by commit SHA
rule: GitHub Actions are pinned by full commit SHA with a version comment, and checkout runs with persist-credentials false.
@row_add n=33 row=decision/toolchains-via-rustup area=dependency-security rec=25
name: CI toolchains installed through rustup
rule: CI toolchains come from rustup toolchain install steps; no toolchain action is used.
@row_add n=34 row=decision/event-values-through-env area=dependency-security rec=26
name: Event payload values only through env
rule: A value from the GitHub event payload reaches a workflow step only through env, never through an expression inside run.
@row_add n=35 row=decision/least-workflow-permissions area=dependency-security rec=27
name: Least workflow permissions
rule: Workflows set empty permissions at workflow level and contents read at job level.
@row_none n=4 rec=28 type=9
@row_none n=5 rec=29 type=S
@row_add n=36 row=decision/cve-sla-enabled-feature area=dependency-security rec=30,29
name: Critical CVE response for an enabled feature
rule: Within 7 days of a RustSec or GHSA advisory against a crate and feature viola enables, the crate is bumped.
@row_add n=37 row=decision/cve-sla-disabled-feature area=dependency-security rec=31,29
name: Critical CVE response for a disabled feature
rule: Within 7 days of an advisory confined to a disabled feature, a cargo-deny ignore entry is added with a reason naming the disabled feature.
@row_none n=6 rec=33 type=S
@row_add n=38 row=decision/release-artifact-attestations area=dependency-security rec=34,33
name: Release built with dist and attestations
rule: A v1.x release requires dist >=0.33.0 with GitHub Artifact Attestations.
@row_add n=39 row=decision/release-binary-audit area=dependency-security rec=35,33
name: Auditable release binaries
rule: A v1.x release requires cargo-auditable >=0.7.6, with cargo audit bin (cargo-audit >=0.22.2) run on the release binaries.
@row_add n=40 row=decision/signed-self-update area=dependency-security rec=36,33
name: Signed self-update
rule: A v1.x release requires self_update 1.3.0 built with signatures (zipsign).
@row_add n=41 row=decision/release-cache-poisoning-addressed area=dependency-security rec=37,33
name: Cache-poisoning findings addressed before release
rule: A v1.x release requires zizmor's cache-poisoning findings to be addressed.
@row_add n=42 row=decision/no-rust-cache-in-release area=dependency-security rec=38,33
name: No rust-cache in the release workflow
rule: The release workflow uses no rust-cache.
@row_none n=7 rec=39 type=S
@row_none n=8 rec=40 type=S
@row_none n=9 rec=41 type=S
@row_none n=10 rec=42 type=S
@row_add n=43 row=decision/local-boundary-no-auth-library area=bootstrap-phases-derive-for-route-setup-project rec=43
name: Local boundary instead of an auth library
rule: Authentication is narrowed to the local boundary with no auth library: Linux socket hardening (socket directory, mode 0600, peer euid check) and client verification of the server.
@row_add n=44 row=decision/owner-only-home-and-pinned-binary area=bootstrap-phases-derive-for-route-setup-project rec=43
name: Owner-only home and pinned binary re-hash
rule: The home is created owner-only, strict-modes checks apply, and the pinned binary's SHA-256 is re-hashed.
@row_add n=45 row=decision/code-files-rewritten-at-start area=bootstrap-phases-derive-for-route-setup-project rec=43
name: Code-bearing files rewritten at start
rule: Code-bearing files are rewritten at every start, and every exec-form command is the absolute pinned path.
@row_add n=46 row=decision/nutype-input-validation area=bootstrap-phases-derive-for-route-setup-project rec=44
name: nutype for input validation
rule: Input validation uses nutype, already in the stack; no new validation library is added, and validated names are enforced at the CLI, MCP handlers, channel link and unlink params and the notify path filter.
why: arch rejected garde and validator
@row_add n=47 row=decision/bounded-external-readers area=bootstrap-phases-derive-for-route-setup-project rec=44
name: Bounded external readers
rule: Every external reader is wrapped by a size bound at a maximum frame size.
@row_add n=48 row=decision/dependency-audit-tools area=bootstrap-phases-derive-for-route-setup-project rec=45
name: cargo-deny and zizmor as audit tools
rule: Dependency auditing uses cargo-deny >=0.20.2 and workflow auditing zizmor >=1.30.1; cargo-audit >=0.22.2 is deferred to v1.x binary scans.
@row_add n=49 row=absence/stored-secrets area=bootstrap-phases-derive-for-route-setup-project rec=46
capability: Stored secrets in v1
@row_add n=50 row=decision/no-repo-secret-scanner area=bootstrap-phases-derive-for-route-setup-project rec=47
name: No repository secret scanner
rule: v1 runs no repository secret scanner.
why: there are no secrets in the repo or CI
@row_add n=51 row=decision/scan-gated-ci-uploads area=bootstrap-phases-derive-for-route-setup-project rec=47
name: CI uploads gated by the artifact canary scan
rule: The CI test job runs the artifact canary scan against the NEVER-log floor, never printing or writing the matched bytes, and every scan-gated upload waits for it to pass.
except: the nightly fuzz job's crash inputs, mutated from the committed synthetic corpus, leave CI unscanned; a non-synthetic seed must first be dropped
@row_add n=52 row=decision/error-display-no-upstream-text area=bootstrap-phases-derive-for-route-setup-project rec=48
name: Error messages carry no upstream text
rule: The Display output of the product error enums never interpolates upstream text or absolute paths.
@row_add n=53 row=contract/mcp-structured-content/codes area=bootstrap-phases-derive-for-route-setup-project rec=48
boundary: MCP structuredContent
item: refusal, detail and error codes
promise: structuredContent carries only refusal, detail and error codes.
@row_add n=54 row=decision/serde-errors-mapped-to-fixed-message area=bootstrap-phases-derive-for-route-setup-project rec=48
name: Upstream-quoting errors mapped to fixed messages
rule: An error chain printed on the viola binary's human-mode stderr never includes a source whose Display can quote upstream text; such a source is mapped to a fixed message and the full error goes only to the instance diagnostics.
why: serde errors quote the offending value
@row_add n=55 row=decision/never-log-claude-values area=bootstrap-phases-derive-for-route-setup-project rec=49
name: CLAUDE values never logged
rule: CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every other stripped CLAUDE* value are never written to any log, diagnostic, snapshot, events.ndjson or fixture.
@row_add n=56 row=decision/user-content-only-to-diagnostics area=bootstrap-phases-derive-for-route-setup-project rec=49
name: User content only in diagnostics
rule: Outside the content-carrying contract payloads, user content, tool input, the statusline command and drift reports go only to the instance diagnostics, never to stderr, CLI --json errors, MCP isError results or channel error.data.
except: events.ndjson, wait and last results, hook.dialog and the statusline_command in snapshot.json carry content by design
@row_add n=57 row=decision/no-paths-in-error-output area=bootstrap-phases-derive-for-route-setup-project rec=49
name: No paths or type names in error output
rule: Absolute paths, which carry the OS username, and internal type names never go to CLI --json error output, MCP isError results or channel error.data.
why: the OS username is the only PII in scope
@row_add n=58 row=decision/diagnostics-owner-only area=bootstrap-phases-derive-for-route-setup-project rec=49
name: Diagnostics files owner-only
rule: Files under the instance diagnostics are mode 0600.
@row_add n=59 row=decision/human-output-escapes-controls area=bootstrap-phases-derive-for-route-setup-project rec=49
name: Human-mode output escapes control characters
rule: Human-mode wait and last output escapes C0 and C1 controls except newline and tab; human-mode list rows escape every control character as \xHH text so each row stays one line.
@row_none n=11 rec=50 type=9
@row_none n=12 rec=51 type=9
@row_none n=13 rec=52 type=0
what: instructions to downstream skills on how to order and materialize the bootstrap phases
@row_none n=14 rec=53 type=S
@row_none n=15 rec=54 type=S
@row_none n=16 rec=55 type=S
