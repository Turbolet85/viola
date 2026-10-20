# security extract

## Relevance
partial — no product trust boundary moves, but four items touch security-governed test and gate surfaces: the Windows mutation scratch (3), a newly committed live recording (7), the `pre-push` coverage launcher (8) and the stamped-home gates (9). The source splits (5) also carry the escaping and compiled-literal bans with the code they move.

## Constraints
- Item 3: the Windows mutation scratch keeps its guard. The plan requires a wipe only of `<repo parent>/viola-mutants-scratch`, re-created owner-only through `viola_state::fs::create_private_dir` (the protected user + SYSTEM DACL), never through a plain `create_dir_all`, and never a loosened strict-modes check to make an instance tree inside it pass (per security-plan §Security Anti-Patterns → Code Patterns). Whether the `Err(Writable)` failure of `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not` comes from the re-creation path or from the check is research's question. The fix must land on the creation side, never on the check.
- Item 7: a recorded live `tools/list` exchange committed as a fixture follows the repository-fixture rule. Synthetic prompts only; home prefix → `~` and the user word → `<user>`, scrubbed before the write; refused on a surviving drive path, `/home/`, `/Users/`, `\Users\`, the user word or an email-shaped token; reviewed for home paths, usernames and tool `input` before it is committed (per security-plan §Data Protection, Repository fixtures). Two things are research's question: whether a `tools/list` / `initialize` exchange recorded from a live start carries the canonical exe or home path (the `plugin install` `.mcp.json` args name both), and whether `viola verify --record`'s scrub reaches this new recording or a separate path needs the same checks.
- Item 7: no R8-stripped `CLAUDE*` value (`CLAUDE_CODE_MESSAGING_TOKEN` / `_SOCKET` above all) may reach the recorded fixture (per security-plan §Bootstrap phases, `logging-redaction-wire`).
- Item 7: the E2E replay case stays inside `viola-e2e`'s rmcp feature set. It may use `client` only, never `transport-child-process`, and `transport-streamable-http-server` / `auth` stay banned (per security-plan §API Security).
- Item 8: the profraw investigation keeps the `pre-push` launcher's environment exactly `HOME` + `PATH` under `/usr/bin/env -i`. Any added assignment that takes a host value (an `LLVM_PROFILE_FILE` or a similar coverage variable passed through) is a boundary widening that needs its own Decisions Log entry, and no uid-0 launch or second launcher may be added (per security-plan §Secret Management, Development).
- Items 1, 2 and 8: a "faster replay" or a profile-writer fix adds no new environment read or test seam in any `viola` build. The only seams are `FAKE_AGENT_PUMP_DELAY_MS` (capped at 5 s) and `FAKE_AGENT_HOOK_PANIC` (exact `1`), both behind `cfg(feature = "fake-agent")`. Another seam needs a Decisions Log entry (per security-plan §Security Anti-Patterns → Universal).
- Item 5: splitting `src/human/board.rs` keeps every human `list` row going through `escape_row` (each control character, `\n`, `\t`, DEL and C1 as `\xHH`, one line per row), the unwrapped row's NAME included (per security-plan §Bootstrap phases, `logging-redaction-wire`).

## Patterns to follow
- The test-side keepers (`prepare_home_base` in `tests/support/home.rs`, `Workspace::ensure_e2e_home` in the harness) create the backing with mode 0700 through the directory builder and remove nothing. A refusal is a fixed message that names no path. Item 9's headroom read should take the same read-only stance (per security-plan §Security Anti-Patterns → Code Patterns).
- Fixture refusals name the file and a closed code (`home-path` · `absolute-path` · `username` · `email`), never the content. A new recording path reuses that form (per security-plan §Data Protection, Repository fixtures).
- The CI artifact canary scan (`viola-harness secret-scan`) reads all of `target/agent-run/` and gates every scan-gated upload. New harness capture lands where it is scanned (per security-plan §Secret Management, Secret scanning in CI).
- `src/cmd/verify/typed.rs`'s pastes and every screen signature stay compiled `viola-agent-claude` literals when the file is split by concern, never built from runtime text (per security-plan §Security Anti-Patterns → Code Patterns).

## Anti-patterns to avoid
- NEVER let a test-side removal under `target/e2e-home`, or item 9's headroom step, take any dir the test side did not create. Other live sessions' `/tmp` task output is not the chunk's to remove (per security-plan §Security Anti-Patterns → Code Patterns).
- NEVER commit a `fixtures/` recording made from a non-synthetic session, or one not yet checked for home paths and usernames (per security-plan §Security Anti-Patterns → Data Protection).
- NEVER add a `skip` / `allow` / ignore to `deny.toml`, or silence zizmor, to get a new `viola-e2e` dependency or feature through for item 7 (per security-plan §Dependency Security, `deny.toml` additions).

## Contract bindings
- Fixture scrub (item 7) ↔ tests: committed fixtures hold no real PII, and the replay is the fake/E2E harness's (security-plan §Data Protection, Repository fixtures ↔ test-plan §3).
- Artifact canary scan ↔ obs: the `secret-scan` classes are obs-owned, obs-plan §9 step 3 (security-plan §Secret Management, Secret scanning in CI).
- `pre-push` launcher (item 8) ↔ the test harness: `viola-harness pre-push`'s `tools` stage and the `env -i` launch are the harness's, and the security floor binds them (security-plan §Dependency Security, Pinning ↔ test-plan §3).
- Mutation scratch (item 3) ↔ the strict-modes check in `viola-state`: the scratch's owner-only creation is what lets the check accept an instance tree (security-plan §Security Anti-Patterns → Code Patterns ↔ §Authentication & Authorization).

## Acceptance criteria contributions
- `cargo deny check` passes, `bash scripts/rmcp-features.sh` reads rmcp features exactly `server` + `transport-io` in `viola`'s release graph, and `viola-e2e` takes no rmcp feature beyond `client` (per security-plan §API Security).
- The item-7 fixture file holds no `/home/`, `/Users/`, `\Users\`, drive-path string start, OS user word, email-shaped token or `CLAUDE_CODE_MESSAGING_*` value. A grep of the committed file reads zero hits (per security-plan §Data Protection, Repository fixtures).
- The `pre-push` launcher's child environment is still exactly `HOME` + `PATH`: the canary pair reads 0 `CLAUDE*` names through the launcher and 1 without `-i` (per security-plan §Secret Management, Development).
- The scratch guard still refuses (`scratch-refused`) any path whose final component is not exactly `viola-mutants-scratch`, and the scratch re-creation goes through `create_private_dir`. No strict-modes code path is loosened, as read in the diff (per security-plan §Security Anti-Patterns → Code Patterns).
