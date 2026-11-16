# Live starts — chunk 2026-11-16-local-command-delivery (cap 1, inputs#I2)

Each start is entered here with the clock's time (`date -u`) BEFORE it is fired.

## Start 1 — 2026-11-16T07:24:23Z (entered before firing)
- What: ONE `viola --home <root>/home run probe -- ~/.local/share/mise/installs/claude/2.1.325/claude` (claude
  2.1.325 by its install path, never the mise shim), `<root>` = `target/live-localcmd-20261116/` (gitignored, 0700
  dirs), the pinned product build `<root>/bin/viola` (sha256 prefix `251d99b81e42e980`, a default-feature
  `cargo build --bin viola` of HEAD `c80fe742`), cwd `<root>/work`, inside one pty the rig opens
  (`$TMPDIR/{scratchpad}/rig.py`, 120×40), which drains the screen bytes to `<root>/screen.raw` (kept local, never
  committed) and types NO key into the pty.
- The child's environment: every inherited `CLAUDE*` and `ANTHROPIC*` name removed, then `CLAUDE_CONFIG_DIR` = the
  private 0700 `<root>/cfg` (kept through the R8 strip by the scratch home's `config.json` `claude_env_keep`),
  `ANTHROPIC_AUTH_TOKEN` = the literal `viola-scratch-not-a-token`, `ANTHROPIC_BASE_URL` = `http://127.0.0.1:9`
  (a closed local port: nothing is sent), `CLAUDE_CODE_MAX_RETRIES` = `0`, `TERM` = `xterm-256color`.
- The private config dir is seeded by the rig with a `.claude.json` holding onboarding done and the scratch folder
  trusted, so no dialog needs a key (viola types into no dialog). Nothing under `~/.claude`, nor `~/.claude.json`,
  is read or written by the rig; both are compared by stat facts only (mtime, size) before and after:
  `~/.claude.json` 1794813824 · 282976 B, `~/.claude/settings.json` 1794639652 · 5271 B (before).
- The sends: once `events.ndjson` holds the start's `session-start`, three compiled local commands through
  `viola --home <root>/home send probe` (stdin), one at a time, each after the previous send returned:
  `/mcp disable viola-probe-absent`, `/cost`, `/context`.
- Reads per command, kinds and counts only: the send's `--json` document (closed words), the event kinds appended to
  `events.ndjson` since the pre-send offset, the count of `<local-command-stdout>` tags in the private config dir's
  transcripts and its delta, and whether a compiled input-box footer literal appears in the screen bytes after it.
- Stop: the rig's `stop` file (SIGTERM to the pty's process group, SIGKILL after 10 s).
- Result: the start did NOT reach a session. `events.ndjson` held the `wheel` start line alone; no `session-start`
  in 100 s; no send was made. The CLI drew its own modal «Allow external CLAUDE.md file imports?» (read from the
  rig's local screen bytes only, 1 419 B): the scratch folder `<root>/work` lies inside this repository, so the CLI
  read the repository's `CLAUDE.md` above it and its `@.claude/session-handoff.md` import. The rig typed no key;
  viola types into no dialog. Stopped by the `stop` file at 2026-11-16T07:26:06Z, the rig's child ended by SIGTERM
  (status 15), no process of the root left (`ps -eo pid,comm,args`, none).
- After (stat facts): `~/.claude/settings.json` unchanged (1794639652 · 5271 B). `~/.claude.json` mtime moved
  (1794813824 → 1794813966), size unchanged (282 976 B): not attributable to this start — other live `claude`
  sessions on the host (this phase's own session included) write that file; the child ran with its config dir
  private and its own `<root>/cfg/.claude.json` grew (208 → 742 B), the CLI's writes there.
- Reading: the cap (1) is spent; none of the three local commands was read. A second start needs the founder's
  word anew (asked through the overseer, not taken).

## Start 2 — 2026-11-16T07:47:47Z (entered before firing; the cap raised to 2 by the founder's own word, inputs#I3; the last)
- What: start 1 exactly, but for the scratch root: `<root>` = `$TMPDIR/viola-live-localcmd-125iyA` (a fresh 0700
  dir made by `mktemp -d` under `/tmp`, a tmpfs; `/tmp` and `/` hold no `CLAUDE.md`; `git rev-parse` in `<root>/work`
  reads «not a git repository»), the same pinned build `<root>/bin/viola` (sha256 prefix `251d99b81e42e980`, copied
  from start 1's), the same rig, environment, seeded private config dir, three sends and reads.
- The founder's condition (inputs#I3): if a modal stands, nothing is typed; the rig reads which modal it is (local
  screen bytes) and the start is ended; that reading is recorded, and no third start is asked.
- Before (stat facts): `~/.claude.json` 1794815262 · 282976 B, `~/.claude/settings.json` 1794639652 · 5271 B.
- Result: the session started (`events.ndjson`: `wheel`, `session-start`, at 07:47:58Z); no modal stood: the input
  box, its rule rows and the footer literal `(shift+tab to cycle)` drawn (local screen bytes, escapes stripped).
  The wrapper ran on an unverified CLI (a fresh home, no stamp): transport-only.
  - `/mcp disable viola-probe-absent` (sent 07:48:11Z): `{"v":1,"refusal":"not-delivered","detail":"no-prompt-submitted"}`,
    exit 13, after the 10 s window. Event kinds appended: `send-issued`, `send-refused` — NO hook kind (no
    `prompt-submitted`, no `session-*`, no `turn-ended`). Transcript `<local-command-stdout>` tags 0 → 1 (delta 1).
    The input-box footer literal drawn again after it: yes.
  - `/cost` (sent 07:48:38Z): the same document, exit 13. Event kinds: `send-issued`, `send-refused` — no hook kind.
    Tags 1 → 1 (delta 0). Footer literal after it: no. On 2.1.325 `/cost` is an alias of `/usage`, which opened its
    settings panel (the menu row «/usage ( cost ) Show session cost …», then the panel's tabs and «Esc to cancel»):
    a modal stood.
  - `/context`: NOT sent. By the founder's condition (inputs#I3), with a modal standing nothing was typed and the
    start was ended.
  - The private config dir's transcripts at the end (counts only): 1 file; `<local-command-stdout>` 1,
    `<local-command-stderr>` 0, `<command-name>` 1, `<command-message>` 1, `<command-args>` 1; 6 lines of types
    `mode`, `permission-mode`, `file-history-snapshot`, `user` ×3.
- Stopped by the `stop` file at 2026-11-16T07:49:04Z (status 15, SIGTERM), no process of the root left
  (`ps -eo pid,comm,args`, none). After (stat facts): `~/.claude/settings.json` unchanged; `~/.claude.json` mtime
  moved (→ 1794815344), size unchanged — not attributable, as at start 1.
- Reading: on 2.1.325 a local command that prints and returns fires NO hook, and its run is written to the session
  transcript as one `<local-command-stdout>` tag (with its `<command-name>`); a panel command (`/cost` → `/usage`)
  writes no stdout tag while its panel stands and leaves a modal. The cap (2) is spent; no third start is asked.
  The scratch root `$TMPDIR/viola-live-localcmd-125iyA` (tmpfs) and `target/live-localcmd-20261116/` are left for
  the operator's desk.
