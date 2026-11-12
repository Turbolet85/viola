@row_add n=1 row=decision/leading-slash-never-a-prompt area=1-test-scope-summary rec=1,2
name: Leading-slash argument never a prompt
rule: A leading-slash argument in the target or text position of send or answer is never taken as a prompt: it is a usage error, exit 2, and nothing is delivered.
guard: cli_slash_args
@row_add n=2 row=decision/tailing-ignores-symlinks area=1-test-scope-summary rec=3,4
name: Tailing ignores symlinks and invalid names
rule: Tailing ignores symlinks and invalid names, and an over-long ndjson line is counted in skipped.torn_lines.
@row_add n=3 row=decision/vt100-panic-contained area=1-test-scope-summary rec=5,6
name: vt100 panic contained
rule: A forced vt100 panic refuses send with not-delivered/input-not-ready while passthrough continues on the outer terminal: the child's own bytes reach it and no viola literal appears on it.
@row_add n=4 row=decision/linux-only area=1-test-scope-summary rec=7,8
name: Linux only
rule: The product is Linux only: the core on any current distribution, the full workspace and fake-agent E2E on ubuntu-latest, the window features on Hyprland; a test gate whose predicate names windows or macos is a fault even when a row names it.
guard: contract_os_gated_tests
@row_add n=5 row=contract/cli-exit-codes/14 area=1-test-scope-summary rec=9,10,11
boundary: CLI exit codes
item: 14
promise: A refusal whose code word is outside the known list falls back to RefusalReason unknown and exits 14 with a --json refusal document; it never panics and never maps to 20.
@row_add n=6 row=decision/no-secrets-in-logs area=1-test-scope-summary rec=12,13
name: No secrets in logs or diagnostics
rule: No log line and nothing in diagnostics/ holds a Cookie header or a stripped CLAUDE* value, and diagnostics/ is mode 0600.
guard: the secret-scan test
@row_add n=7 row=decision/mutation-at-epoch-boundary area=1-test-scope-summary rec=14,15,16
name: Mutation testing at the epoch boundary
rule: cargo-mutants runs at the epoch-boundary code audit and on demand through run --mutants, never in the default run or --all; there is no per-chunk, pre-push or CI mutation gate, and a missed or timed-out mutant fails that run.
@row_add n=8 row=decision/rstest-case-tables area=2-test-strategy rec=17,18,19,20
name: Table-driven unit cases in rstest
rule: Table-driven unit cases, including the refusal-order, exit-code and fixture-replay matrices, use rstest case tables, each case named with a readable label.
except: viola-e2e and viola-core carry no rstest dev-dependency; their table-driven cases are labelled literal tables in one test.
why: so no manifest edge is added for rstest alone
