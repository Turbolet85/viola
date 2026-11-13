@task_prose part=chunk
Epoch 8 cleanup, no change to product behaviour: one shared, separately tested source-masking scanner in test support replaces the three hand scanners of the error-body, bounded-reader and OS-gated-test census contract tests, and each caller keeps its verdict.
Two duplicated product function pairs merge into one each (inside the hook command, and across run links and run snapshot). The dead strict-modes presence check is deleted with its doc mention, or wired and tested. The dead non-unix constants in the e2e run harness are removed.
The three largest over-800-line files (the agent ledger's test cases, the fake agent binary, the verify CLI tests) are split to at most 800 lines each, with moved bodies byte-identical and every test name kept.
Named mutant survivors are closed by new tests, never by a mutation run in this chunk: the test-only MCP tap (tested, or kept out of the mutants scope with a recorded reason), the e2e harness free-bytes check, the viola-state fs and pin error-kind guards, the MCP parse-rejected WARN line and the stdin short-buffer case. The stdin CHUNK constant is marked equivalent.
New test helpers: one that makes a test home the way viola makes one (0700), one remove-the-guard control, and the host's quota'd temp root named in one place. The e2e secret scan drops its cookie class (the cookie header and the GUI cookie pair) and still catches every other class. Coordinates are re-read at HEAD.
@task_file n=1 path=tests/contract_error_bodies.rs mark=edits
what: the contract test that every product error writer is named and error bodies interpolate nothing — edits: its hand scanner `blanked`, replaced by the shared scanner
@task_file n=2 path=tests/contract_bounded_readers.rs mark=edits
what: the bounded-readers census check: every product reader is bounded or named — edits: its hand scanner `code_only`, replaced by the shared scanner
@task_file n=3 path=tests/contract_os_gated_tests.rs mark=edits
what: the census of OS-gated tests (cfg attribute readers) — edits: its census scanner, replaced by the shared scanner
@task_file n=4 path=tests/support/mod.rs mark=edits
what: the shared test-support module that holds the test helpers — edits: the module list, which gains the shared source-masking scanner, the remove-the-guard control helper and the temp-root setting
@task_file n=5 path=tests/support/home.rs mark=edits
what: the test-home helper: test homes made the way viola makes its home, owner-only 0700, used by the 19 standing home tests
@task_file n=6 path=src/cmd/hook.rs mark=edits
what: the viola hook command, which is silent, exits 0 and checks the instance first — edits: the duplicated function pair, merged into one
@task_file n=7 path=src/run/links.rs mark=edits
what: run's link names and their restore — edits: the function duplicated with the snapshot one, merged into one
@task_file n=8 path=src/run/snapshot.rs mark=edits
what: the instance snapshot that the wrapper writes and revive reads — edits: the function duplicated with the links one, merged into one
@task_file n=9 path=crates/viola-state/src/strict.rs mark=edits
what: the strict-modes check on the viola home tree and trusted state — edits: the dead presence check `check_present`
@task_file n=10 path=.claude/docs/services/viola-state.md mark=edits
what: the viola-state service doc — edits: its one mention of the dead presence check
@task_file n=11 path=crates/viola-state/src/fs.rs mark=edits
what: viola-state's owner-only file writes and replace helper — edits: its error-kind guard (a survivor), closed by tests
@task_file n=12 path=crates/viola-state/src/pin.rs mark=edits
what: the pinned copy keyed and checked by SHA-256 — edits: its error-kind guard (a survivor), closed by tests
@task_file n=13 path=crates/viola-mcp/src/lib.rs mark=edits
what: the viola mcp server and its stdin refusal log — edits: tests asserting the WARN parse-rejected line
@task_file n=14 path=crates/viola-mcp/src/stdin.rs mark=edits
what: the MCP stdin line reader, capped at the frame limit — edits: a test with a buffer smaller than the pending line; the CHUNK constant marked equivalent
@task_file n=15 path=crates/viola-e2e/src/bin/viola-mcp-tap.rs mark=edits
what: the test-only MCP tap through which MCP fixtures are recorded — edits: tests, or a recorded exclusion from the mutants scope
@task_file n=16 path=crates/viola-e2e/src/harness/mod.rs mark=edits
what: the e2e harness and its free-bytes check — edits: `free_bytes`, closed by tests
@task_file n=17 path=crates/viola-e2e/src/harness/run.rs mark=edits
what: the e2e harness's run of viola and its children — edits: the three dead non-unix constants
@task_file n=18 path=crates/viola-e2e/src/harness/secret_scan.rs mark=edits
what: the end-to-end secret scan over artefacts (the canary scan classes) — edits: the cookie class (cookie header and GUI cookie pair), removed
@task_file n=19 path=crates/viola-agent-claude/src/ledger/test_cases.rs mark=edits
what: the agent ledger's test cases, split into files of at most 800 lines with bodies and test names unchanged
@task_file n=20 path=src/bin/viola-fake-agent.rs mark=edits
what: the test-only fake agent binary and its test seams, split into files of at most 800 lines with bodies unchanged
@task_file n=21 path=tests/cli_verify.rs mark=edits
what: the viola verify CLI tests, split into files of at most 800 lines with every test name kept
@task_item n=1 part=needs
The artifact canary scan's classes: the cookie header class leaves the scan, and every other class (content canary, stripped CLAUDE values, diagnostics file mode) is still caught.
@task_item n=2 part=needs
The canary scan never echoes what it matches, and CI uploads stay gated by the scan once its cookie class is gone.
@task_item n=3 part=needs
Mutation testing happens at the epoch audit and not in this chunk. Survivors are closed by tests, mutants output is not uploaded, and any exclusion from the mutants scope is recorded with a reason.
@task_item n=4 part=needs
The test-only MCP tap through which MCP fixtures are recorded. It must stay test-only and out of release builds, whether it gets tests or a scope exclusion.
@task_item n=5 part=needs
The MCP stdin line cap on the reader and the parse-rejected refusal log line. New tests assert the WARN line and the short-buffer case, and the cap behaviour stays unchanged.
@task_item n=6 part=needs
The strict-modes check on the home tree and trusted state, through the dead presence check. If the check is wired instead of deleted, it runs before any trusted read and its refusal leaves no trace.
@task_item n=7 part=needs
Owner-only modes for the viola home (0700). The new test-home helper makes homes the way viola does, and the 19 home tests use it.
@task_item n=8 part=needs
Tests use their own home and keep test homes out of git. The shared home helper must keep each test's home private to that test.
@task_item n=9 part=needs
Test-side removal only touches directories the test created. The quota'd temp root and the refused recursive rm must not make the harness remove anything outside its own directories, and must not touch the e2e-home backing or a linked base.
@task_item n=10 part=needs
The remove-the-guard control helper. It must restore the guard it removes, and no config, env var or flag may switch off a product control.
@task_item n=11 part=needs
The hook command's merged function: hook stays silent, exits 0, fails open to the human, checks the instance and strict modes before anything, and writes no frame when refused.
@task_item n=12 part=needs
The links and snapshot merge: link restore still trusts only names it wrote, link names stay out of the process log, only the wrapper writes the snapshot, and recorded cwd stays in the snapshot.
@task_item n=13 part=needs
The pinned copy's error-kind guards (SHA-256 keyed, re-hashed before reuse, no-clobber placement). New tests pin the error-kind behaviour without changing it.
@task_item n=14 part=needs
viola-state's owner-only file writes and the one shared replace helper. The fs error-kind guard is closed by tests.
@task_item n=15 part=needs
The bounded-readers census and the every-error-writer-named check keep their verdicts after the shared scanner replaces their hand scanners.
@task_item n=16 part=needs
The fake agent's test seams (hook panic, pump delay) stay under the fake-agent feature and compiled out of release when the binary is split. The release build still refuses test-only binaries and features.
@task_item n=17 part=needs
The tree is Linux-only (the pre-push gate is Linux only), which is why the dead non-unix constants in the e2e run harness are removed.
@task_item n=18 part=needs
No live claude start in this chunk: the real claude CLI runs only locally, under the founder's per-chunk cap.
@task_standing n=1
convention: every new test makes and uses its own owner-only home and never the user's real viola or Claude home
home: secret-management
@task_standing n=2
convention: every new test or helper removes only directories it created itself
home: security-anti-patterns-never-do-these · security-anti-patterns
@task_standing n=3
convention: every new test, fixture or helper holds no credential literal, secret or local home, and nothing of a test home is committed to git
home: security-anti-patterns-never-do-these
@task_standing n=4
convention: every new test seam or test-only code stays under the test-only feature and is compiled out of release
home: security-anti-patterns · input-validation
@task_item n=1 part=not_needed
Revive's JSON codes, list documents and refusal words: no revive output changes.
@task_item n=2 part=not_needed
The budget.json and config.json fields, including dialog_hold_s. The dialog-hold ceiling is not this entry's work.
@task_item n=3 part=not_needed
The rmcp feature set and API Security: no MCP transport or dependency feature changes.
@task_item n=4 part=not_needed
Dependency CVE response times, advisory version floors, licence allow list and the weekly scheduled advisory and fuzzing runs: no dependency is added or updated.
@task_item n=5 part=not_needed
The absences of a database, file upload, webhook, hosting, websocket and remote access: none is approached.
@task_item n=6 part=not_needed
The statusline shell-out, its sources and its deadline, and the terminal colour facts read for list: neither is touched.
@task_item n=7 part=not_needed
The channel's closed types and frame param contracts for send, answer, pause, release, wait and last: no channel frame changes.
@task_item n=8 part=not_needed
The paste and dialog free-text character sets and the typed-text normalisation: no typed-text rule changes.
