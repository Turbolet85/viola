decision/no-compliance-verification · «compliance (case-insensitive)» · in src/**, crates/**, tests/**, scripts/**, .github/** · place unread: it names no tracked file · fct-pvam5xzl@1
decision/no-switch-disables-a-control · «(?i)(disable|insecure|skip_strict|no_strict|unsafe_)[a-z_]*["\)]|VIOLA_[A-Z_]*(DISABLE|SKIP|INSECURE|NO_)» · in src/ non-test files · place unread: it names no tracked file · fct-engo5ir7@2
decision/no-switch-disables-a-control · «var_os\(|env::var\(|"VIOLA_[A-Z_]+"» · in src/ non-test files · place unread: it names no tracked file · fct-engo5ir7@2
decision/no-switch-disables-a-control · «VEIL_DISABLE_REDACTION» · in deny.toml · first hit deny.toml:41 · fct-engo5ir7@2
decision/no-screen-scraping-in-e2e · «screen_text|contents\(\)|\.screen\(\)|rendered screen» · in tests/** and crates/viola-e2e/**/*.rs · place unread: it names no tracked file · fct-shszmhpb@1
decision/no-human-in-loop-verification · «Percy|Chromatic|exploratory» · in crates, tests, src, scripts, .github, fuzz/fuzz_targets · place unread: it names no tracked file · fct-gcer5i4i@2
decision/insta-check-mode-only · «INSTA_UPDATE» · in crates, tests, src, scripts, .github, .config · place unread: it names no tracked file · fct-gcer5i4i@2
decision/insta-check-mode-only · «insta::» · in crates, tests, src, scripts, .github, .config · place unread: it names no tracked file · fct-gcer5i4i@2
decision/no-release-in-driver-hints · «release» · in src/human/hints.rs · first hit src/human/hints.rs:127 · fct-x4oyim7i@1,fct-jtonrpse@2,fct-ndx7lm2p@2
decision/env-per-child · «set_var» · in **/*.rs · first hit crates/viola-e2e/src/harness/boot.rs:496 · fct-io5pl4i6@1
decision/no-http-mocking · «wiremock|httpmock|mockito|reqwest|hyper|ureq|isahc» · in Cargo.toml, crates/*/Cargo.toml, deny.toml · place unread: it names no tracked file · fct-yqmk3k5s@1,fct-qta73xkp@1
decision/no-mocks-in-e2e · «mockall|automock» · in crates/viola-e2e, tests/ · place unread: it names no tracked file · fct-dlsufsua@1,fct-kjqvkfwd@1
decision/no-mocks-in-e2e · «wiremock|httpmock|mockito» · in Cargo.toml, crates/*/Cargo.toml · place unread: it names no tracked file · fct-dlsufsua@1,fct-kjqvkfwd@1
decision/no-http-fake-servers · «TcpListener|http://|https://|axum|warp|tiny_http» · in src/**/*.rs, crates/**/*.rs, tests/**/*.rs · place unread: it names no tracked file · fct-qull53ri@1
decision/mutation-not-a-ci-gate · «mutants» · in crates/viola-e2e/src/harness/pre_push.rs · first hit crates/viola-e2e/src/harness/pre_push.rs:103 · fct-fo2372wn@2
decision/mutation-output-never-uploaded · «mutants» · in .github/workflows/ci.yml upload-artifact paths · place unread: it names no tracked file · fct-2ymlhohm@2
decision/boot-deadline-not-a-budget · «the perf rows list (five hook rows, no boot)» · in crates/viola-e2e/src/harness/run/perf.rs:22-26 · place unread: it names no tracked file · fct-6txqcbhi@1,fct-umz6vque@1
decision/literal-test-oracles · «viola_agent_claude::ledger::LedgerRow» · in tests/support/home.rs:13 is a type import for planting rows, not an expected-value oracle · place unread: it names no tracked file · fct-jfqpalye@1
decision/no-debug-string-comparison · «format!("{:?}" compared to a reason or exit code» · in tests/ and tests/support/ · place unread: it names no tracked file · fct-ee3ranmx@1
decision/endpoint-from-snapshot-only · «try_overwrite» · in tests/ and crates/*/tests · place unread: it names no tracked file · fct-aqwq345t@1
decision/no-process-env-mutation-in-tests · «set_var» · in tests/ crates/*/tests crates/viola-e2e · place unread: it names no tracked file · fct-ieax375m@1
decision/no-shared-mutable-state · «static mut|lazy_static|thread_local!|serial_test|#\[serial\]» · in **/*.{rs,toml,yml} · place unread: it names no tracked file · fct-x76wnqv5@1
decision/no-committed-test-secrets · «.env» · in .gitignore · first hit .gitignore:30 · fct-ejn23r66@1
decision/no-committed-test-secrets · «**/.env*» · in repository · place unread: it names no tracked file · fct-ejn23r66@1
decision/no-runtime-monkey-patching · «monkey|set_var» · in {src,crates,tests}/**/*.rs · place unread: it names no tracked file · fct-sw7m3jb3@1
decision/no-tokio-failpoint-crate · «fail-parallel|failpoint|fail_point» · in repository tracked files incl. Cargo.toml, Cargo.lock, crates/*/Cargo.toml · place unread: it names no tracked file · fct-mbozqxei@1
decision/no-tokio-failpoint-crate · «mockall» · in Cargo.toml · first hit Cargo.toml:214 · fct-mbozqxei@1
decision/tests-run-in-parallel · «test-threads» · in .github/**, Cargo.toml, .config/** · place unread: it names no tracked file · fct-yijholqq@1
decision/no-human-reviewed-visual-regression · «Percy|Chromatic|Applitools» · in tracked source outside .claude/.andromeda/viola-0.x · place unread: it names no tracked file · fct-neagcwig@1
decision/no-recorder-authored-tests · «codegen|Selenium|Playwright» · in tests/, crates/*/tests, crates/*/src, src/, scripts/, .github/ · place unread: it names no tracked file · fct-xjawgeec@1
decision/no-human-screenshot-review · «screenshot» · in tests/, crates/*/tests, crates/*/src, src/, scripts/, .github/ · place unread: it names no tracked file · fct-dx5w5yd6@1
decision/no-real-network-in-tests · «TcpStream|TcpListener|reqwest|ureq|UdpSocket|https?://[a-z0-9]» · in tests/**/*.rs, crates/*/tests/**/*.rs, crates/viola-e2e/src/**/*.rs · place unread: it names no tracked file · fct-bom5r3of@1
decision/no-in-process-clock-across-processes · «mock_instant|MockClock» · in tests/, crates/*/tests, crates/*/src, src/, Cargo.toml, crates/*/Cargo.toml · place unread: it names no tracked file · fct-v6in7u4x@1
decision/no-interactive-snapshot-review · «cargo insta|insta review» · in .github/, scripts/, crates/viola-e2e/src, tests/, src/ · place unread: it names no tracked file · fct-qxrlql3x@2
decision/no-blanket-approval-tests · «approve|auto-approve|auto_answer|auto-answer|blanket» · in tests/, crates/*/tests, crates/viola-e2e/src, src/ · place unread: it names no tracked file · fct-m765nh6l@1
decision/no-flaky-test-retries · «retries» · in .config/nextest.toml · first hit .config/nextest.toml:2 · fct-jpy6mjkf@1
decision/no-flaky-test-retries · «retry» · in crates/viola-e2e/tests/zero_retries.rs · first hit crates/viola-e2e/tests/zero_retries.rs:1 · fct-jpy6mjkf@1
decision/module-acyclicity-not-gated · «acyclic» · in .github/** and scripts/*.sh · place unread: it names no tracked file · fct-pk32flwi@2
decision/module-acyclicity-not-gated · «cargo modules dependencies» · in .github/** and scripts/*.sh · place unread: it names no tracked file · fct-pk32flwi@2
decision/mutation-testing-off-ci · «run --mutants» · in .github/** · first hit .github/workflows/ci.yml:32 · fct-ixlw7x3t@2
decision/mutation-testing-off-ci · «cargo mutants» · in .github/** and scripts/*.sh · place unread: it names no tracked file · fct-ixlw7x3t@2
decision/no-cache-in-release-workflow · «rust-cache» · in .github · first hit .github/workflows/ci.yml:29 · fct-2qyicniz@2
decision/hints-never-suggest-release · «viola release|`release`|run release|, release» · in src/**/*.rs crates/**/*.rs · place unread: it names no tracked file · fct-imixgk6n@1,fct-3r2a46vd@2
decision/secrets-never-in-logs · «Cookie|cookie» · in src/**/*.rs crates/*/src/**/*.rs · place unread: it names no tracked file · fct-ja62ftpj@3,fct-iplgfuqy@2
decision/secrets-never-in-logs · «env_stripped_count» · in crates/viola-pty/src/lib.rs · first hit crates/viola-pty/src/lib.rs:128 · fct-ja62ftpj@3,fct-iplgfuqy@2
decision/error-bodies-held-by-surface-witnesses · «assert_error_body_clean|error_body_hit» · in tests/**/*.rs, crates/**/*.rs, src/**/*.rs · place unread: it names no tracked file · fct-iplgfuqy@2
decision/internal-error-held-at-unit-layer · «failpoint|fail_point|fail::|FAILPOINT|PANIC_AFTER|panic_after|PANIC_COUNT» · in tests/**/*.rs, crates/**/*.rs, src/**/*.rs · place unread: it names no tracked file · fct-iplgfuqy@2
decision/internal-error-held-at-unit-layer · «-32603» · in tests/ · first hit tests/channel_error_bodies.rs:5 · fct-iplgfuqy@2
decision/no-cli-snapshot-crates · «tests/cmd/**» · in the repository tree · place unread: it names no tracked file · fct-lzdm6e7w@6
decision/profraw-census-dev-host-only · «profraw-census» · in .github/**, .config/**, crates/**/*.rs, src/**/*.rs, tests/**/*.rs · place unread: it names no tracked file · fct-s6rhzqu4@1
decision/profraw-census-dev-host-only · «profraw-census» · in scripts/agent-run.sh and the harness pre_push stages · place unread: it names no tracked file · fct-s6rhzqu4@1
decision/mutation-gate-at-epoch-boundary · «--mutants» · in .github/workflows/ci.yml · first hit .github/workflows/ci.yml:32 · fct-ocdr3xdn@3
decision/mutation-gate-at-epoch-boundary · «--mutants» · in crates/viola-e2e/src/harness/pre_push.rs and pre_push/linux.rs · place unread: it names no tracked file · fct-ocdr3xdn@3
absence/windows-and-macos-support · «cfg(windows)» · in src/**, crates/**, tests/** · place unread: it names no tracked file · fct-ze6ljnvw@1
absence/windows-and-macos-support · «os: [ubuntu-latest]» · in .github/workflows/ci.yml · first hit .github/workflows/ci.yml:18 · fct-ze6ljnvw@1
decision/mcp-no-control-tools · «"release"» · in crates/viola-mcp/src · first hit crates/viola-mcp/src/tools.rs:483 · fct-elurmfjv@1
decision/mcp-no-control-tools · «"pause"» · in crates/viola-mcp/src · first hit crates/viola-mcp/src/tools.rs:483 · fct-elurmfjv@1
decision/mcp-no-control-tools · «"link"» · in crates/viola-mcp/src · first hit crates/viola-mcp/src/tools.rs:483 · fct-elurmfjv@1
decision/mcp-no-control-tools · «"unlink"» · in crates/viola-mcp/src · first hit crates/viola-mcp/src/tools.rs:483 · fct-elurmfjv@1
decision/mcp-no-control-tools · «TOOL_NAMES: [&str; 5] = ["send", "wait", "last", "answer", "list"]» · in crates/viola-mcp/src/tools.rs:37 · place unread: it names no tracked file · fct-elurmfjv@1
absence/browser-suite · «playwright» · in src/**, crates/**, tests/**, .github/**, scripts/** · place unread: it names no tracked file · fct-67zmkx76@3
absence/browser-suite · «selenium|cypress|webdriver|chromium|puppeteer» · in src/**, crates/**, tests/**, .github/**, scripts/** · place unread: it names no tracked file · fct-67zmkx76@3
decision/harness-never-reads-cargo-bin-exe · «CARGO_BIN_EXE» · in crates/viola-e2e/src · first hit crates/viola-e2e/src/harness/run/mutants.rs:29 · fct-zxaumenb@5
decision/harness-never-reads-cargo-bin-exe · «bin_dir» · in crates/viola-e2e/src/harness/boot.rs · first hit crates/viola-e2e/src/harness/boot.rs:77 · fct-zxaumenb@5
decision/doc-examples-never-ignore · «ignore» · in **/*.rs · first hit crates/viola-agent-claude/src/dialog.rs:344 · fct-zujg4opw@3
decision/no-ci-mutants-gate · «gate --require» · in .github/workflows/ci.yml · first hit .github/workflows/ci.yml:117 · fct-47aec6c3@3
decision/no-ci-mutants-gate · «mutants» · in .github/workflows/ci.yml · first hit .github/workflows/ci.yml:32 · fct-47aec6c3@3
decision/pre-push-runs-no-mutation · «mutants» · in crates/viola-e2e/src/harness/pre_push.rs and crates/viola-e2e/src/harness/pre_push/linux.rs · place unread: it names no tracked file · fct-gvvrytf4@2
absence/outbound-http · «reqwest|hyper::|ureq::|TcpStream|https?://» · in src/**/*.rs and crates/*/src/**/*.rs · place unread: it names no tracked file · fct-jnilykpz@2,fct-yqmk3k5s@1,fct-qta73xkp@1,fct-b4mgg72y@2,fct-iyneo4vq@1,fct-pwscsozw@2
