# Classified — security-plan · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · no · the session changes only the pinned copy's absent-copy write, so the events.ndjson lines and their no-truncation rule are not reached
threat-model-summary#1.2 · no · dialog free text and hook stdin are untouched, so the tool-call arguments and the plan and question text stay as they are
threat-model-summary#1.3 · no · the pinned path bin/<version>-<hash>/viola(.exe) and its 0700 mode are unchanged, so the line holds on re-read
threat-model-summary#1.4 · no · the session adds no env read and no credential path, so the R8-stripped environment lines hold
threat-model-summary#1.5 · no · the session names no change to snapshot fields or budget.json, so the operational metadata lines hold
threat-model-summary#2.1 · no · IPC access control and server verification are untouched, and strict.rs keeps its product verdict behind check_instance on the changed file
threat-model-summary#3.1 · no · paste text validation and the send path are untouched, so the typed-text and paste boundary holds
threat-model-summary#4.1 · no · the session names no change to the human's stdin or the wheel, so the terminal boundary holds
threat-model-summary#5.1 · no · hook stdin parsing and the hook statusline arm are unchanged, and strict.rs keeps the product verdict every hook's check runs
threat-model-summary#6.1 · no · the MCP tool set, its input checks and the check_instance order are unchanged, and mcp_tools.rs changes only the test's boot pair
threat-model-summary#7.1 · no · the CLI verbs, VIOLA_DIR and the home-tree check order are unchanged, and strict.rs keeps the plain-home refusal verdict on the changed file
threat-model-summary#8.1 · yes · add-to · the pinned binary is placed once and never replaced under concurrent starts, so the exec race fact F4 (ETXTBSY, 4 of 24 boots failed before and 0 of 24 after) joins this boundary's code-bearing artefacts
threat-model-summary#9.1 · no · run.rs and verify.rs are unchanged and the pinned copy's folder stays first on the child's PATH, so the child-spawn lines hold
threat-model-summary#10.1 · no · no dependency is added or bumped, Cargo.lock, deny.toml and the deny-sync bans are unchanged, and tempfile 3.27.0 already provides the new calls
threat-model-summary#11.1 · no · CI jobs and release-check.sh are unchanged, the fake agent stays test-only with only a receipt field added, and the release gate's refusal holds
