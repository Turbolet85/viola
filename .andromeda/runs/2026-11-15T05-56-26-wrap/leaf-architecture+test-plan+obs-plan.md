@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=89
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'selec
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=retired
leaf-text: Run B trusted in `<cwd>/.viola-verify-<pid>/` (for shell-plain paths, also one probe `--settings <probe dir>/typed/statusline.json` whose status line the hidden `hook statusline --capture` arm files) with six compiled pastes
source: ctr-zgw4cnv3 «contract/hook-to-kind-map/hook-statusline · Hook to kind map · hook statusline — The retired hook word `statusline` maps to no hook event: `viola hook statusline` takes the unknown-event exit 0 and reads and writes nothing.»
replace-with: Run B trusted in `<cwd>/.viola-verify-<pid>/` with six compiled pastes
@leaf_edit n=2 leaf=.claude/docs/commands.md line=23 class=retired
leaf-text: Run B's `paste-<n>` / `compact-<n>` / `clear-<n>` framing variants, Run B's status line variants `StatusLine.default-1.json` / `StatusLine.rate-limits-1.json`, Run E's first SessionStart
source: dec-d5bfbqzd «decision/fixtures-recorded-by-live-verify · Fixtures refreshed by live verify — The recorded agent hook, dialog, framing, resume and screen fixtures are refreshed only by a live viola verify run, locally;»
replace-with: Run B's `paste-<n>` / `compact-<n>` / `clear-<n>` framing variants, Run E's first SessionStart
@leaf_edit n=3 leaf=.claude/docs/commands.md line=27 class=retired
leaf-text: [--agents-mode recorded|oversize|malformed] [--statusline-echo]` — build, create a home, stamp via `viola verify` against the fake agent, start wrappers, wait for readiness (both built: `--statusline-echo`, which conflicts with `--unstamped`, runs after the stamp and before the wrappers start, writing `<home>/statusline-source.json` (0600) whose command is `<bin dir>/viola-fake-agent statusline-echo <home>/fake/statusline.marker`, for Path 6; `--agents-mode`
source: ctr-etd5ao5s «contract/harness-boot/statusline-echo · harness boot · --statusline-echo — --statusline-echo conflicts with --unstamped and plants an owner-only statusline settings source in the session home that viola only reads; it sets no env var.»
replace-with: [--agents-mode recorded|oversize|malformed]` — build, create a home, stamp via `viola verify` against the fake agent, start wrappers, wait for readiness (`--agents-mode`
@leaf_edit n=4 leaf=.claude/docs/commands.md line=33 class=retired
leaf-text: [--agents-mode recorded|oversize|malformed] [--statusline-echo]` · `run
source: ctr-etd5ao5s «contract/harness-boot/statusline-echo · harness boot · --statusline-echo — --statusline-echo conflicts with --unstamped and plants an owner-only statusline settings source in the session home that viola only reads; it sets no env var.»
replace-with: [--agents-mode recorded|oversize|malformed]` · `run
