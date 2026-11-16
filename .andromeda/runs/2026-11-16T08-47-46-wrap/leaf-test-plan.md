@leaf_group group=test-plan
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=119
last: - 2026-11-16: A CI red that is not the chunk's is measured f
@leaf_read leaf=.claude/rules/verification-harness.md lines=73
last: - 2026-11-14: A test-only recorder that replaces every off-l
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=26 class=omission
leaf-text: (`/remote-control` on any version, or `/compact` and `/clear` on an unverified CLI) → exit 0 `unconfirmable`
source: ctr-fguybmtp «A listed local command with no measured post-condition, any listed command on an unverified CLI, or an echo-confirmed command with no remembered transcript or no readable transcript end, yields ok»
replace-with: (`/remote-control` on any version, or `/compact` and `/clear` on an unverified CLI), or an echo-confirmed command with no remembered transcript or no readable transcript end → exit 0 `unconfirmable`
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=26 class=omission
leaf-text: (`send_clear_on_a_verified_cli_is_confirmed_by_its_new_session`, on the recorded 2.1.287 variants);
source: dec-g2cvvbdj «/compact only by one of cause compact with the unchanged id, and /mcp only by a new stdout tag in the transcript past its pre-paste end; none by a prompt-submitted, a missed one is not-delivered.»
replace-with: (`send_clear_on_a_verified_cli_is_confirmed_by_its_new_session`, on the recorded 2.1.287 variants); `/mcp` on a verified CLI is confirmed only by a new stdout tag in the transcript past its pre-paste end, else `not-delivered`;
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: and writes an ndjson `--receipt`;
source: ctr-mz2pisu3 «The argv-only --transcript creates <exe dir>/transcripts/ at 0700 at start and gives the launch SessionStart a transcript_path to a seeded 0600 file named by its session id; replayed framing variants»
replace-with: and writes an ndjson `--receipt`; its argv-only `--transcript` creates `<exe dir>/transcripts/` at 0700 at start and gives the launch SessionStart a `transcript_path` to a seeded 0600 file named by its session id (replayed framing variants not rewritten), and every stamping `viola verify` against it (harness `boot` and the stamped-home helpers) passes it `--transcript` beside `--framing` (the framing-free verify helper passes none);
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: with the last fired SessionStart's `session_id`)
source: ctr-6gfalgaz «Under --local-command-mode a submitted local command whose post-condition is the stdout tag, and under --framing the exact verify echo probe command, append one stdout tag line to the last started session»
replace-with: with the last fired SessionStart's `session_id`; the exact verify echo probe command, like a stdout-tag local command under `--local-command-mode`, one stdout tag line appended to the last started session's transcript and no hook fired)
@leaf_edit n=5 leaf=.claude/rules/verification-harness.md line=28 class=omission
leaf-text: --trusted-root <workspace root> --dialogs --framing`
source: dec-pxitz5rr «Every stamping viola verify against the fake agent, in the harness boot and the stamped-home helpers, passes the fake agent --transcript beside --framing.»
replace-with: --trusted-root <workspace root> --dialogs --framing`, the fake agent given `--transcript` beside `--framing` (without it Run B's echo probe is skipped and the `local-command-echo` row fails)
@leaf_edit n=6 leaf=.claude/rules/verification-harness.md line=38 class=stale
leaf-text: whose `session_id` is set to the last SessionStart the agent fired.
source: dec-d5qj3lkd «the launch SessionStart's source and session_id under --resume, its transcript_path under --transcript, and the compact replay's session_id.»
replace-with: whose `session_id` is set to the last SessionStart the agent fired, and under `--transcript` the launch SessionStart's `transcript_path`.
@leaf_edit n=7 leaf=.claude/rules/verification-harness.md line=41 class=omission
leaf-text: `--settings <file>` is accepted, its value consumed and ignored.
source: ctr-6gfalgaz «Under --local-command-mode a submitted local command whose post-condition is the stdout tag, and under --framing the exact verify echo probe command, append one stdout tag line to the last started session»
replace-with: `--transcript` (argv-only) creates `<exe dir>/transcripts/` at 0700 at start and gives the launch SessionStart a `transcript_path` to a seeded 0600 file named by its session id (replayed framing variants not rewritten); under `--local-command-mode` a submitted local command whose post-condition is the stdout tag, and under `--framing` the exact verify echo probe command, append one stdout tag line to the last started session's transcript and fire no hook. `--settings <file>` is accepted, its value consumed and ignored.
@leaf_edit n=8 leaf=.claude/rules/verification-harness.md line=42 class=omission
leaf-text: taken against the fake agent at 2.1.287 over `fixtures/claude` once per pair of built binaries
source: dec-pxitz5rr «Every stamping viola verify against the fake agent, in the harness boot and the stamped-home helpers, passes the fake agent --transcript beside --framing.»
replace-with: taken against the fake agent at 2.1.287 over `fixtures/claude`, the fake agent given `--transcript` beside `--framing`, once per pair of built binaries
