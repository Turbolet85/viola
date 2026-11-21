@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'select
@leaf_edit n=1 leaf=.claude/docs/commands.md line=24 class=omission
leaf-text: no start added; in CI the fake agent's `agents` mode answers it), Run C's dialogs in `-dialogs/`
source: dec-uczlwirj «viola verify makes one print-mode probe, six interactive PTY runs under the R8 strip at a fixed 80×24, one agents --json read inside Run B and one print-mode sender, verify-sender, inside Run B's cross-session turn»
replace-with: no start added; in CI the fake agent's `agents` mode answers it) and one print-mode sender, `verify-sender`, inside Run B's cross-session turn (Run B started with the crossSessionInbound accept settings, the sender addressing it by `uds:<runtime>/cc-socks/<pid>.sock`, never a bare session id; the cross-session-prefix row passes only when the message arrives opening with the CLI's cross-session harness prefix, else fails; no runtime dir runs no turn), Run C's dialogs in `-dialogs/`
@leaf_edit n=2 leaf=.claude/docs/commands.md line=25 class=omission
leaf-text: names only, every kept name removed)
source: ctr-jgzfav2v «Holds outcome, prefix, latency_ms, sender_tools and send_result from the cross-session probe, closed words and one number.»
replace-with: names only, every kept name removed; `measured.cross_session_probe` holds the cross-session probe's `outcome`, `prefix`, `latency_ms`, `sender_tools` and `send_result`, closed words and one number)
@leaf_edit n=3 leaf=.claude/docs/commands.md line=24 class=omission
leaf-text: under a test-owned user home whose config file is seeded empty and owner-only and named to the fake agent by `--trust-config`
source: dec-3nvt7nvr «Every test or harness viola verify that stamps runs with a short owner-only runtime dir the test or harness owns for the stamp's life.»
replace-with: under a test-owned user home whose config file is seeded empty and owner-only and named to the fake agent by `--trust-config`, with a short owner-only runtime dir (`XDG_RUNTIME_DIR`) the test owns for the stamp's life
@leaf_edit n=4 leaf=.claude/docs/commands.md line=28 class=omission
leaf-text: stamp via `viola verify` against the fake agent, start wrappers
source: dec-3nvt7nvr «Every test or harness viola verify that stamps runs with a short owner-only runtime dir the test or harness owns for the stamp's life.»
replace-with: stamp via `viola verify` against the fake agent (with a short owner-only runtime dir the harness owns for the stamp's life), start wrappers
