@leaf_group group=test-plan
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=116
last: - 2026-11-14: A wheel move taken through `human_input` write
@leaf_read leaf=.claude/rules/verification-harness.md lines=73
last: - 2026-11-14: A test-only recorder that replaces every off-l
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=21 class=retired
leaf-text: and the `StatusLine.default-1.json` / `StatusLine.rate-limits-1.json` status line variants and the `SessionStart.resume-1.json` resume variant
source: dec-d5bfbqzd «The recorded agent hook, dialog, framing, resume and screen fixtures are refreshed only by a live viola verify run, locally»
replace-with: and the `SessionStart.resume-1.json` resume variant
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=21 class=retired
leaf-text: the framing tier by Run B's five added pastes, the status line variants by Run B's probe status line (Unix), the resume variant by Run E
source: dec-d5bfbqzd «The recorded agent hook, dialog, framing, resume and screen fixtures are refreshed only by a live viola verify run, locally»
replace-with: the framing tier by Run B's five added pastes, the resume variant by Run E
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=21 class=retired
leaf-text: `session_id`) and, with an absolute `--settings` status line command, the set's `StatusLine.default-1.json` after the launch SessionStart and `StatusLine.rate-limits-1.json` once after the first Stop, for verify's five interactive runs
source: ctr-r2ad2s23 «--settings <file> is accepted, its value consumed and ignored.»
replace-with: `session_id`) for verify's five interactive runs
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=21 class=retired
leaf-text: six framing variants, two status line variants, one resume variant
source: dec-d5bfbqzd «The recorded agent hook, dialog, framing, resume and screen fixtures are refreshed only by a live viola verify run, locally»
replace-with: six framing variants, one resume variant
@leaf_edit n=5 leaf=.claude/docs/tests-summary.md line=21 class=retired
leaf-text: the `StatusLine.*.json` status line variants against `schemas/claude-statusline-fixture.v1.json`, the `Agents.*.json`
source: dec-d5bfbqzd «The recorded agent hook, dialog, framing, resume and screen fixtures are refreshed only by a live viola verify run, locally»
replace-with: the `Agents.*.json`
@leaf_edit n=6 leaf=.claude/docs/tests-summary.md line=21 class=retired
leaf-text: replays every dialog variant and every framing variant byte for byte, and compares the `statusline` receipts' `stdin_hex` byte for byte, in order and count, with each recorded set's `StatusLine.default-1.json` then `StatusLine.rate-limits-1.json` under `--framing` (Unix), and replays the resume variant
source: dec-fxotyct5 «A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, resume and compact variants»
replace-with: replays every dialog variant and every framing variant byte for byte, and replays the resume variant
@leaf_edit n=7 leaf=.claude/docs/tests-summary.md line=21 class=retired
leaf-text: (the last two, `--settings <file>` and `--statusline-stdin <file>`, run the settings file's `statusLine.command` with the stdin file's bytes, only with both and only when it is absolute, or under `--framing` without `--statusline-stdin` with the set's recorded `StatusLine` variants, and file a `statusline` receipt; beside the options two mode words, `statusline-echo` and `agents`, a first word `agents`
source: ctr-r2ad2s23 «--settings <file> is accepted, its value consumed and ignored.»
replace-with: (`--settings <file>` accepted, its value consumed and ignored; beside the options the mode word `agents`, a first word `agents`
@leaf_edit n=8 leaf=.claude/docs/tests-summary.md line=30 class=retired
leaf-text: **Path 6 — statusline pass-through** — the statusline output passes through byte for byte, as root tests on a planted `<home>/statusline-source.json` (`tests/hook_statusline.rs`, `tests/cli_instance_state.rs`); the budget governor
source: ctr-wdo4kiwe «A start hands its child no --settings and writes no settings.json in the instance directory; the user's settings are left byte- and mtime-equal»
replace-with: **Path 6 — no status-line interception** — a start hands its child no `--settings` and writes no `settings.json` in the instance directory, the user's settings left byte- and mtime-equal; the budget governor
@leaf_edit n=9 leaf=.claude/rules/testing.md line=35 class=retired
leaf-text: keep-list; a `workspace.repo` naming this repository's own remote is kept (the `StatusLine` variants' `workspace.repo.owner`, neither rewritten by the scrub nor refused by the hygiene walk; nothing else widens); every input
source: dec-izq2ll4y «Tests never use production data; fixtures come only from `viola verify` probe text or a test-only recorder's capture, scrubbed of paths and usernames.»
replace-with: keep-list; every input
@leaf_edit n=10 leaf=.claude/rules/verification-harness.md line=28 class=retired
leaf-text: → under `--statusline-echo` (clap-conflicts with `--unstamped`) `<home>/statusline-source.json` planted at 0600 with the command `<bin dir>/viola-fake-agent statusline-echo <home>/fake/statusline.marker` (no env var) → the `--agents-mode
source: ctr-etd5ao5s «--statusline-echo conflicts with --unstamped and plants an owner-only statusline settings source in the session home that viola only reads; it sets no env var.»
replace-with: → the `--agents-mode
@leaf_edit n=11 leaf=.claude/rules/verification-harness.md line=41 class=retired
leaf-text: (each hook, and the status line command)
source: dec-kdiw3jql «On Unix every command the fake agent spawns, each hook, runs in a process group of its own.»
replace-with: (each hook)
@leaf_edit n=12 leaf=.claude/rules/verification-harness.md line=41 class=retired
leaf-text: and no UserPromptSubmit; with an absolute `--settings` `statusLine.command` and no `--statusline-stdin`, the set's `StatusLine.default-1.json` after the launch SessionStart's hooks and `StatusLine.rate-limits-1.json` once after the first Stop, each a `statusline` receipt with `stdin_hex`, the command spawned directly)
source: ctr-r2ad2s23 «--settings <file> is accepted, its value consumed and ignored.»
replace-with: and no UserPromptSubmit)
@leaf_edit n=13 leaf=.claude/rules/verification-harness.md line=41 class=retired
leaf-text: Built for the statusline tests: `--settings <file>` + `--statusline-stdin <file>` (the settings file's `statusLine.command`, split on ASCII white space, run with both only when absolute, with the stdin file's bytes, and under `--framing` without `--statusline-stdin` with the set's recorded `StatusLine` variants; a `statusline` receipt) and the mode word `statusline-echo` (prints its output, files its stdin, exits with the code after the marker path).
source: ctr-r2ad2s23 «--settings <file> is accepted, its value consumed and ignored.»
replace-with: `--settings <file>` is accepted, its value consumed and ignored.
@leaf_edit n=14 leaf=.claude/rules/verification-harness.md line=40 class=retired
leaf-text: `step`, `cwd`, `hold` (the `--exit-no-eof` holder, at its start), `statusline` (the `hook` kind's fields without `event`).
source: ctr-iwrkqo2g «Each receipt line has a kebab kind, one of start, size, cwd, hold, env, fds, key, prompt, hook, step.»
replace-with: `step`, `cwd`, `hold` (the `--exit-no-eof` holder, at its start).
@leaf_edit n=15 leaf=.claude/rules/verification-harness.md line=45 class=retired
leaf-text: the framing variants (`paste-<n>`, `compact-<n>`, `clear-<n>`), the status line variants (`StatusLine.default-1.json`, `StatusLine.rate-limits-1.json`) and the resume variant
source: dec-fxotyct5 «A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, resume and compact variants»
replace-with: the framing variants (`paste-<n>`, `compact-<n>`, `clear-<n>`) and the resume variant
@leaf_edit n=16 leaf=.claude/rules/verification-harness.md line=45 class=retired
leaf-text: byte for byte (contract suite; the status line variants by the `statusline` receipts' `stdin_hex`, in order and count, under `--framing`, Unix);
source: dec-fxotyct5 «A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, resume and compact variants»
replace-with: byte for byte (contract suite);
