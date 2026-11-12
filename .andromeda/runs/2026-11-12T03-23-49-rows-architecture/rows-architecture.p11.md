@row_add n=1 row=contract/fault-document/known-code-message area=standard-contracts rec=1
boundary: The fault document on --json and MCP
item: message of a known fault code
promise: For each of the five known fault codes, `message` is that fault's fixed text; a fault without an integer `code` reads as no fault.
@row_add n=2 row=contract/fault-document/unsupported-version area=standard-contracts rec=1
boundary: The fault document on --json and MCP
item: -32602 unsupported version
promise: A `-32602` fault with an integer `data.supported` and a version-text `data.wrapper` (1 to 64 ASCII letters, digits, `.`, `-` or `+`) reads the unsupported-version body with those two values.
@row_add n=3 row=contract/fault-document/release-from-driver area=standard-contracts rec=1
boundary: The fault document on --json and MCP
item: -32602 release-from-driver
promise: A `-32602` fault whose `data.reason` is exactly `release-from-driver` reads the release-from-driver body.
@row_add n=4 row=contract/fault-document/invalid-params area=standard-contracts rec=1
boundary: The fault document on --json and MCP
item: other -32602
promise: Any other `-32602` fault reads `invalid params` with `data` null; every other known code reads `data` null.
@row_add n=5 row=contract/fault-document/wrapper-fault area=standard-contracts rec=2
boundary: The fault document on --json and MCP
item: wrapper fault
promise: A fault code outside the five known codes reads `{"code": <code>, "message": "wrapper fault", "data": null}`; the literal `wrapper fault` is a fixed output word.
@row_add n=6 row=contract/human-error-line/wrapper-fault area=standard-contracts rec=2
boundary: The human error line
item: wrapper fault
promise: A fault code outside the five known codes prints the human line `error: wrapper fault  <code>`.
@row_add n=7 row=contract/revive-json/refusal-document area=standard-contracts rec=3
boundary: viola revive --json
item: start refusal document
promise: The refusals `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused` print the refused document in place of the human pair, before any child is spawned.
@row_add n=8 row=contract/revive-json/program-not-found area=standard-contracts rec=3
boundary: viola revive --json
item: program not found
promise: A program not found prints the `internal-error` document, before any child is spawned.
@row_add n=9 row=contract/revive-json/exit-1-document area=standard-contracts rec=4
boundary: viola revive --json
item: exit-1 document
promise: Every refusal of the start prints one exit-1 document; an internal error of the start prints the `internal-error` document and the error chain is still logged.
@row_add n=10 row=contract/revive-json/pump-error area=standard-contracts rec=4
boundary: viola revive --json
item: error after the child holds the terminal
promise: An error while pumping the child prints no document, because the child has held the terminal.
@row_add n=11 row=decision/statusline-wrapper-not-a-hook area=standard-contracts rec=5
name: Statusline wrapper is not a hook event
rule: `viola hook statusline` is a hidden arm of `hook` taken on the word `statusline` before the event is mapped, so the hook event set keeps its nine events and the arm is exempt from the hook rules.
why: The statusline arm is the statusline wrapper, not a Claude Code hook.
@row_add n=12 row=contract/hook-statusline/capture area=standard-contracts rec=5
boundary: viola hook statusline
item: --capture <DIR>
promise: With `--capture <DIR>` the arm takes `viola verify`'s probe capture: no instance check, no `budget.json` write, no command run, nothing printed; without it the pass-through is unchanged.
@row_add n=13 row=decision/statusline-source-in-viola-home area=standard-contracts rec=5
name: Statusline command source
rule: The user's statusline command is read at start from one named source that viola only reads: the statusline source file in the viola home whenever it exists, else only in the default home the user's Claude settings, else none.
why: Two files deciding a command viola later runs is a ratified boundary widening, choosing the file in the viola home.
@row_add n=14 row=contract/statusline-source/command area=standard-contracts rec=5
boundary: The statusline source settings
item: statusLine command
promise: A command is taken only from a `statusLine` of type `command`, non-empty, with no NUL.
@row_add n=15 row=decision/statusline-check-before-log area=standard-contracts rec=5
name: Statusline instance check before the log
rule: The statusline arm runs the instance check before it opens its log or reads anything under the home.
why: Opening the log sets an existing home to 0700, so a check made after it passes a home at 0770.
@row_add n=16 row=contract/hook-statusline/refused-check area=standard-contracts rec=5
boundary: viola hook statusline
item: refused instance check
promise: A refused check ends the arm at once with exit 0: nothing under the home read or opened, no stdin read, no command run, nothing printed, `budget.json` untouched and the home's mode left as found.
@row_add n=17 row=decision/statusline-only-shell-out area=standard-contracts rec=5
name: Statusline command is the only shell-out
rule: The user's statusline command is the only shell-out in viola: one argument of `/bin/sh -c`, with the same stdin, environment and working directory unchanged, stderr discarded, bounded by a deadline.
why: The command is a user-written shell string; a ratified boundary widening.
guard: ledger row statusline-shell
@row_add n=18 row=decision/statusline-deadline area=standard-contracts rec=5
name: Statusline command deadline
rule: The statusline command is bounded by a 5 s deadline.
state: provisional
@row_add n=19 row=contract/hook-statusline/stdout area=standard-contracts rec=5
boundary: viola hook statusline
item: stdout
promise: Prints the command's stdout unchanged, bounded by the frame cap; prints nothing when the command fails or passes the deadline, when no command is recorded, and on oversize or malformed stdin.
@row_add n=20 row=contract/hook-statusline/exit area=standard-contracts rec=5
boundary: viola hook statusline
item: exit status
promise: Always exits 0 with empty stderr.
@row_add n=21 row=contract/board/status area=standard-contracts rec=9
boundary: The board (CLI list and MCP list)
item: status
promise: Each row carries `status` (idle/busy) from `claude agents --json`, and unwrapped sessions are listed as read-only rows.
@row_add n=22 row=contract/board/agents-read-failure area=standard-contracts rec=9
boundary: The board (CLI list and MCP list)
item: agents read failure
promise: If `claude agents` is not found, fails, is over the cap or cannot be parsed, every row's `status` is `unknown`, no unwrapped rows are listed, and the outcome is never an error.
@row_add n=23 row=contract/board/wrapped-merge area=standard-contracts rec=9
boundary: The board (CLI list and MCP list)
item: wrapped instance join
promise: An agents row matching a wrapped instance's `child_pid` or `agent_session_id` is merged into that instance's row, never listed separately.
@row_add n=24 row=contract/mcp-list/refusal area=standard-contracts rec=9
boundary: The MCP list tool
item: board refusal
promise: The board's refusal is returned as `instance-unreachable`.
@row_add n=25 row=decision/mcp-list-reads-agents area=standard-contracts rec=9
name: MCP list reads claude agents on each call
rule: The MCP server's `list` tool makes the same one `claude agents` read as CLI `list` on each call.
why: A ratified widening.
@row_add n=26 row=decision/workspace-dependency-pins area=build-system rec=11
name: Third-party versions pinned in one place
rule: Every third-party dependency version is pinned once in the workspace dependency table and shared by the members that take it.
@row_add n=27 row=decision/msrv area=build-system rec=11
name: Minimum supported Rust version 1.96
rule: The workspace declares rust-version 1.96, and dependency resolution prefers versions compatible with it.
@row_add n=28 row=decision/fmt-and-clippy-gate area=build-system rec=12,25
name: Formatting and clippy with warnings denied
rule: Code passes the cargo fmt check and clippy over the whole workspace, all targets and the fake-agent feature, with every warning denied.
@row_add n=29 row=decision/no-print-macros area=build-system rec=12
name: No print macros
rule: The workspace lints deny `print_stdout`, `print_stderr` and `dbg_macro`, and every product member inherits them.
guard: tests/contract_lints.rs
except: viola-e2e denies only `dbg_macro`, because the harness prints its JSON document; the fake agent carries a crate-level print allow.
@row_add n=30 row=decision/single-human-writer area=build-system rec=12
name: One human writer in the root bin
rule: The root bin writes human output only through its human module, whose writers lock the stream and swallow a closed pipe instead of panicking; hook and obs never call it.
@row_add n=31 row=contract/human-stderr/refusal area=build-system rec=12
boundary: Human stderr
item: refusal
promise: A refusal writes the two fixed `unable:` / `hint:` lines as one write.
@row_add n=32 row=contract/human-stderr/internal-error area=build-system rec=12
boundary: Human stderr
item: internal error
promise: An internal error writes exactly `error: internal error` and a newline, for every cli verb without `--json`.
@row_add n=33 row=contract/cli-json/internal-error area=build-system rec=12
boundary: CLI --json output
item: internal-error
promise: Under `--json` an internal error prints the `internal-error` document on stdout, with stderr empty.
@row_add n=34 row=decision/obs-event-only-emitter area=build-system rec=12
name: obs_event! is the only event emitter
rule: The tracing level macros are banned, so `obs_event!` is the only sanctioned emitter; raw `event!` is caught by a fail-closed grep.
why: Clippy cannot exempt a macro's inner expansion, so raw event! needs the grep.
guard: clippy.toml disallowed-macros; scripts/lint-probes.sh
@row_add n=35 row=decision/lint-bans-proven area=build-system rec=12
name: Every lint ban proven to fire
rule: Every lint ban is proven to fire and every control proven to pass.
guard: scripts/lint-probes.sh
@row_add n=36 row=decision/licence-allowlist area=build-system rec=14
name: Licence allowlist
rule: Dependency licences allowed are exactly MIT, Apache-2.0, Zlib and Unicode-3.0.
why: The 0BSD exception was ratified.
guard: cargo deny check
except: 0BSD only through the per-crate exception for doctest-file, never through the allowlist.
@row_add n=37 row=decision/crates-io-only area=build-system rec=14
name: Sources from crates.io only
rule: Every dependency comes from crates.io only.
guard: cargo deny check
@row_add n=38 row=decision/no-c-building-crates area=build-system rec=14
name: No C-building crates
rule: C-building crates (cc, libsqlite3-sys, openssl-sys) are banned from the dependency graph.
guard: cargo deny check
except: The fuzz workspace's libfuzzer-sys builds C++ via cc, a ratified test-only exemption: never linked into viola, and fuzz never joins the root workspace.
@row_add n=39 row=decision/no-telemetry-crates area=build-system rec=14
name: No telemetry crates
rule: Telemetry crates (opentelemetry-otlp, opentelemetry-stdout, sentry, tracing-appender) are banned.
guard: cargo deny check
@row_add n=40 row=decision/banned-features area=build-system rec=14
name: Banned dependency features
rule: The features veil/toggle, rmcp auth and transport-streamable-http-server, and tracing-subscriber/env-filter are banned.
guard: cargo deny check
@row_add n=41 row=decision/serde-json-recursion-limit area=build-system rec=14
name: serde_json keeps its recursion limit
rule: The serde_json unbounded_depth feature is banned.
why: Every parse of external bytes keeps serde_json's default recursion limit.
guard: probe row serde_json-unbounded-depth
@row_add n=42 row=decision/deny-single-target area=build-system rec=14
name: Dependency policy for one target triple
rule: The dependency policy is evaluated for the one target triple x86_64-unknown-linux-gnu.
@row_add n=43 row=decision/sync-crates-tokio-free area=build-system rec=14
name: Sync crates are tokio-free
rule: Tokio is banned anywhere, direct or transitive, in the normal dependency graph of viola-core, viola-pty, viola-state and viola-agent-claude, checked once per crate as the sole root.
why: A direct-parent allowlist would let Tokio arrive through a third-party feature; excluding over the workspace false-fails under feature unification.
guard: deny-sync.toml per crate in scripts/sync-crates.txt
@row_add n=44 row=decision/channel-tokio-only-behind-feature area=build-system rec=14,32
name: viola-channel tokio only behind its feature
rule: viola-channel's normal dependency graph without its tokio feature holds no tokio; the Tokio client exists only behind that feature.
why: As its own root viola-channel false-fails the sole-root ban once viola-mcp enables the feature, so a tree check replaces it.
guard: scripts/channel-tokio-free.sh
@row_add n=45 row=decision/deny-bans-proven area=build-system rec=14
name: Every dependency ban proven live
rule: Every dependency ban is proven live: a throwaway project per ban must fail with that ban's own diagnostic, and a clean control must pass.
guard: scripts/deny-probes.sh
@row_add n=46 row=decision/fuzz-lockfile-audited area=build-system rec=14
name: Fuzz lockfile audited separately
rule: The fuzz workspace's own lockfile is audited separately for advisories and sources.
guard: CI supply-chain step Fuzz lockfile audit; nightly advisories job
@row_add n=47 row=decision/no-orphan-modules area=build-system rec=15
name: No orphan modules
rule: No lib or bin target holds an orphan module; the check is a CI gate and is proven to fire on planted orphans.
guard: scripts/orphans-check.sh
@row_add n=48 row=decision/module-graph-review-not-gate area=build-system rec=15
name: Module dependency graph is a review, not a gate
rule: The acyclic module dependency graph stays an on-demand review, never a gate.
why: cargo-modules reports every type and its own inherent method as a cycle, whatever the filters.
@row_add n=49 row=decision/rmcp-features-exact area=build-system rec=15,35
name: rmcp with exactly server and transport-io
rule: rmcp is present in viola's release graph with default features off, and the features requested of it are exactly server and transport-io.
guard: scripts/rmcp-features.sh
@row_add n=50 row=decision/install-from-source area=build-system rec=17
name: Install with cargo install
rule: viola is installed with cargo install from the repository root.
@row_add n=51 row=decision/no-crates-io-publishing area=build-system rec=18
name: Nothing published to crates.io
rule: Every workspace member, the root viola bin included, is unpublished; nothing goes to crates.io in v1.
@row_add n=52 row=decision/dual-licence area=build-system rec=19
name: MIT OR Apache-2.0 licence
rule: The project is licensed MIT OR Apache-2.0, set once for the workspace and inherited by every member; the README names both licence texts.
why: Ratified.
@row_add n=53 row=decision/github-actions-ci area=ci-cd-approach rec=21
name: GitHub Actions CI
rule: CI runs on GitHub Actions: one push and pull-request pipeline on ubuntu, and a weekly scheduled workflow.
@row_add n=54 row=decision/weekly-advisories area=ci-cd-approach rec=21
name: Weekly advisory audit
rule: A weekly job audits advisories over both the root and the fuzz lockfile.
why: The advisory database moves without code changes.
@row_add n=55 row=decision/weekly-fuzz area=ci-cd-approach rec=21
name: Weekly fuzzing
rule: A weekly job fuzzes every target for a bounded time and uploads the fuzz artifacts on failure.
@row_add n=56 row=decision/no-workflow-concurrency area=ci-cd-approach rec=21
name: No workflow concurrency groups
rule: No workflow has a concurrency block.
why: A concurrency group cancels pending runs, which would drop a cancelled run's always() gate and upload chain.
@row_add n=57 row=decision/workflow-permissions area=ci-cd-approach rec=22
name: Least-privilege workflow permissions
rule: Every workflow sets empty permissions at the top and read-only contents per job.
guard: zizmor
@row_add n=58 row=decision/actions-pinned-by-sha area=ci-cd-approach rec=22
name: Actions pinned by commit SHA
rule: Every action used in a workflow is pinned by full commit SHA with a version comment.
guard: zizmor
@row_add n=59 row=decision/event-payload-via-env area=ci-cd-approach rec=22
name: Event payload only through env
rule: Event-payload values reach a workflow step only through environment variables.
guard: zizmor
@row_add n=60 row=decision/ci-tools-pinned area=ci-cd-approach rec=23
name: CI tools at pinned versions
rule: Every tool CI installs is version-pinned and checkout keeps no credentials; a release asset installed by hand is checked against its published sha256.
@row_add n=61 row=decision/coverage-floors area=ci-cd-approach rec=24
name: Coverage floors
rule: The test job runs the instrumented suite with coverage floors 85/95/80, then doctest, and its gate requires coverage and doctest.
guard: gate --require coverage,doctest
@row_add n=62 row=decision/uploads-gated-on-secret-scan area=ci-cd-approach rec=24
name: Uploads gated on the secret scan
rule: CI artifacts are uploaded only when the secret scan succeeds; on a scan failure only the secret-scan report is uploaded.
guard: secret-scan step
@row_add n=63 row=decision/perf-gate area=ci-cd-approach rec=24
name: Perf runs in its own job
rule: Performance runs only in the perf job, gated on its perf verdict; the test job and the pre-push carry no perf step.
guard: gate --require perf
@row_add n=64 row=decision/mutation-at-epoch-boundary area=ci-cd-approach rec=24
name: Mutation testing at the epoch boundary
rule: CI runs no mutation job; mutation testing runs only through the harness mutants run, kept for the epoch-boundary code audit.
@row_add n=65 row=decision/msrv-ci area=ci-cd-approach rec=24
name: MSRV checked in CI
rule: A CI job checks the workspace and runs the unit tests on Rust 1.96.
guard: gate --require nextest-unit
@row_add n=66 row=decision/fuzz-replay-ci area=ci-cd-approach rec=24
name: Fuzz corpus replayed in CI
rule: A CI job replays the fuzz corpus on every push and pull request.
guard: gate --require fuzz-replay
@row_add n=67 row=decision/gate-steps-fail-closed area=ci-cd-approach rec=24
name: Gate steps fail closed
rule: Every CI gate step runs fail-closed in bash.
@row_add n=68 row=decision/sync-crates-compile-without-tokio area=ci-cd-approach rec=25
name: Sync crates compile without tokio
rule: Each listed sync crate, the hook path included, is checked to compile on its own with no tokio; an empty list or an absent package fails.
@row_add n=69 row=decision/tests-against-fake-agent area=ci-cd-approach rec=25
name: Tests run against the fake agent
rule: The workspace test suite runs against the fake agent, replaying recorded Claude fixtures.
@row_add n=70 row=decision/release-build-viola-only area=ci-cd-approach rec=25
name: Release build yields viola only
rule: The release build yields only the viola executable, and no artifact of it carries the test-only features test-support or fake-agent.
why: The check judges the build's own artifacts, because a shared or cached target directory keeps stale test-only executables.
guard: scripts/release-check.sh
@row_add n=71 row=decision/real-claude-local-only area=ci-cd-approach rec=26
name: Real claude CLI only locally
rule: The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent, and its real-CLI probe and record run only locally.
guard: tests/cli_verify.rs
@row_add n=72 row=decision/local-pre-push-gate area=ci-cd-approach rec=27
name: Local pre-push gate
rule: A local pre-push gate precedes every operator push and a red pre-push stops the pass; CI's run on the pushed sha stays the verdict of record.
@row_add n=73 row=decision/pre-push-clean-environment area=ci-cd-approach rec=27
name: Pre-push children in a clean environment
rule: The pre-push runs natively in the working tree, every child with an emptied environment holding only HOME from the passwd entry and a fixed PATH; the harness's own HOME is never read.
@row_add n=74 row=contract/pre-push/linux-only area=ci-cd-approach rec=27
boundary: viola-harness pre-push
item: pre-push-linux-only
promise: On a host other than Linux the pre-push refuses `pre-push-linux-only` with exit 2.
@row_add n=75 row=contract/pre-push/passwd-home area=ci-cd-approach rec=27
boundary: viola-harness pre-push
item: tool-missing passwd-home
promise: A failed passwd probe or a non-absolute home field is `tool-missing` `passwd-home` at stage `tools`.
@row_add n=76 row=absence/deploy-stage area=ci-cd-approach rec=28
capability: A deploy stage
@row_add n=77 row=decision/release-via-dist area=ci-cd-approach rec=28
name: Release workflow generated by dist
rule: The v1.x release path is a dist-generated release workflow with cargo-auditable.
@row_add n=78 row=decision/core-depends-on-no-viola-crate area=crate-dependency-direction rec=30
name: viola-core depends on no viola crate
rule: viola-core depends on no other viola crate.
@row_add n=79 row=decision/pty-knows-no-agent area=crate-dependency-direction rec=31
name: viola-pty depends on no viola crate and knows no agent
rule: viola-pty depends on no other viola crate, knows no agent, and takes no thiserror.
@row_add n=80 row=decision/claude-shapes-in-one-crate area=crate-dependency-direction rec=34
name: Claude payload shapes in one crate
rule: viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json.
@row_add n=81 row=decision/agent-claude-pure area=crate-dependency-direction rec=34
name: viola-agent-claude does no file I/O
rule: viola-agent-claude takes no viola-state dependency and writes no file; stamps, budget and settings files are written by viola-state and the root bin.
why: So the crate stays pure.
@row_add n=82 row=decision/mcp-dependencies area=crate-dependency-direction rec=35
name: viola-mcp takes channel with tokio
rule: viola-mcp depends on viola-channel with its tokio feature and on tokio, and takes no serde.
@row_add n=83 row=decision/e2e-test-only area=crate-dependency-direction rec=37
name: viola-e2e is test-only
rule: viola-e2e is test-only, and no product crate depends on it.
@row_add n=84 row=decision/root-bin-consumes-normalised-events area=crate-dependency-direction rec=38
name: Root bin consumes only normalised events
rule: The root bin depends on all members and holds the subcommand dispatch, the run pump, the wheel and the budget governor, all of which consume only normalised events.
@row_add n=85 row=decision/local-only-deployment area=deployment-model rec=42
name: Local-only deployment
rule: viola is deployed local-only.
@row_add n=86 row=absence/container-or-serverless-deployment area=deployment-model rec=42
capability: Container, orchestration or serverless deployment (Docker, Compose, Kubernetes, serverless)
@row_add n=87 row=decision/single-binary-on-path area=deployment-model rec=43
name: Single binary on PATH
rule: viola ships as a single binary installed on PATH.
@row_add n=88 row=decision/run-pins-self-copy area=deployment-model rec=43
name: run pins a copy of itself and its plugin
rule: viola run pins a copy of itself per version and hash, writes the embedded plugin pointing at that copy, and launches the child with that plugin directory.
@row_none n=1 rec=6 type=4
relied: A frame-writing client runs the strict-modes check, the liveness pre-check, the socket-directory check, the peer euid decision and then the server identity check, in that order.
@row_none n=2 rec=7 type=4
relied: A frame verb against a stale or dead instance reads instance-dead, while a name with no instance directory reads as unwrapped.
@row_none n=3 rec=8 type=4
relied: A peer whose euid cannot be read is refused.
@row_none n=4 rec=10 type=S
@row_none n=5 rec=13 type=6
@row_none n=6 rec=16 type=5
@row_none n=7 rec=20 type=S
@row_none n=8 rec=29 type=S
@row_none n=9 rec=33 type=5
@row_none n=10 rec=36 type=5
@row_none n=11 rec=39 type=S
@row_none n=12 rec=40 type=S
@row_none n=13 rec=41 type=S
@row_none n=14 rec=44 type=S
