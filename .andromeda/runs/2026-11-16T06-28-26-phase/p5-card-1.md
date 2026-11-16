# P5 review — «Local-command delivery» (`2026-11-16-local-command-delivery`)

**The plan:** open `.andromeda/cache/docs/2026-11-16-local-command-delivery/plan.md` (its render, rev 3, 406 lines).
No plan stands in a chunk's folder; writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-16-local-command-delivery` is a
diff of the chunk's records.

## Goal
A driver's `send` of a local slash command the CLI runs reads as delivered, confirmed by the command's own echo. The
echo was measured live on claude 2.1.325 (live start 2): `/mcp disable viola-probe-absent` fires NO hook and writes
exactly one `<local-command-stdout>` tag to the session transcript; `/cost` (an alias of `/usage`) opens a panel and
writes none. Built on the founder's own answers (relayed verbatim by the overseer): the transcript tag as the echo,
classification by command name, two live starts, and Run B's seventh paste (`inputs#I2`–`I4`).

## Top acceptance criteria
- On a verified CLI, `viola send` of `/mcp disable viola-probe-absent` (arguments included) whose transcript gains the
  tag reads `[RB] read back`, exit 0, `{submitted_at, cursor}`, `send-issued` then `send-confirmed`.
- A tag already there before the send, no tag in the 10 s window, no remembered transcript, and an unverified CLI
  each confirm nothing (`no-prompt-submitted` exit 13 for the expiry, `unconfirmable` exit 0 for the other two).
- By name: exactly a listed name, or a name, one space and a non-empty rest; never a bare leading slash.
- The transcript path never reaches `data`, `events.ndjson`, a log, a span or a reply; the read is `O_NOFOLLOW`,
  regular-file, owner-checked, capped at `MAX_FRAME`, a count alone.
- `local-command-echo` is the 22nd relied ledger row; verify's Run B pastes it seventh; a 21-row stamp reads
  `cli_verified:false`.

## Touchpoints
New `src/run/send/echo.rs`. Modified: the ledger and hook modules of `viola-agent-claude`; `src/cmd/hook.rs`,
`src/cmd/run.rs`; `src/run/send.rs`, `classify.rs`, `slot.rs`, `relabel.rs`, `test_support.rs`;
`src/run/version_gate.rs`; `src/cmd/verify.rs` and `verify/typed/turns.rs`; the fake agent and its parse cases;
`crates/viola-e2e/src/harness/run.rs`; `tests/cli_send.rs`, `tests/cli_verify*`, `tests/contract_ledger_probes.rs`,
`tests/support/` (24 items, research's lists parsed).

## Test commands
16 entries: fmt · clippy · check · the adapter crate · the send/run/hook/verify/version-gate modules · the echo cases by
name · the fake agent · the two booted cases by name · the integration files · `LEDGER_ROWS` · the `fixtures/claude`
probe · `agent-run.sh run --unit` · `pre-push` · and the operator pass (hygiene, push, CI). No mutation run, no live
start, no smoke (each absence stated in the plan's prose). Four one-shot controls, each `--timeout 300` on its
filtered entry: `echo-offset`, `name-args`, `path-shape`, `echo-nofollow`.

## Concretization preview — the one capability claimed
- **v2-23** (integration). Current: «A send of a local slash command that runs reads as delivered, confirmed by the
  command's own echo.» Proposed: «On a verified CLI, a `viola send` of a listed local slash command that runs —
  `/mcp disable viola-probe-absent`, its arguments included — reads as delivered (exit 0, `{submitted_at, cursor}`),
  confirmed by the command's own echo: a `<local-command-stdout>` tag the session's transcript gains after the send
  (the shape measured live on claude 2.1.325, live start 2). `send_local_command_echo_is_read_back_on_a_verified_cli`
  proves it on the fake agent replaying that shape; a tag before the send, no tag, no transcript and an unverified CLI
  each confirm nothing.» Not weakened: the outcome stands and gains its boundary cases. No working-route line names
  v2-23 (`command grep -n "v2-23" viola-0.2.0/working-route.md`: none).

## Named for the founder (`inputs#I4`, and the live record)
- **The probe's residual.** Run B runs under the user's real Claude config. Whether `/mcp disable viola-probe-absent`
  writes a config key there for an absent server is unmeasured. Under live start 2's private config it wrote no key
  naming mcp or disabled (the overseer's reading). The paste adds its line to the CLI's input history, as every Run B
  paste does.
- **The live stamp is owed.** The 22nd relied row makes every existing stamp read `cli_verified:false` until
  `viola verify` re-stamps it. The live home is included: until then `/clear` and `/compact` read `unconfirmable`
  there. No live verify is made by this chunk (the cap is spent).
- **Live starts: 2 of 2 spent at P4** (`.andromeda/runs/2026-11-16T06-28-26-phase/live-starts.md`):
  - Start 1 stopped at the CLI's external-imports modal (its scratch folder sat inside this repository); no key was
    typed and nothing was read.
  - Start 2 is the reading above. It ended at `/cost`'s panel by the founder's condition, so `/context` was not sent.
  - `~/.claude/settings.json` was unchanged. `~/.claude.json`'s mtime moved, size unchanged; that is not attributable,
    since other live sessions write it.

## Directive item 7, in one block
- Union, as `rec.py rows-union` printed it:
  - `architecture: rows 944 · packs 10 · picked 219 (139 by every draw) · standing 24 · 62342 B`
  - `security-plan: rows 599 · packs 6 · picked 171 (115 by every draw) · standing 15 · 46737 B`
  - `design-system: rows 314 · packs 4 · picked 62 (44 by every draw) · standing 32 · 14749 B`
  - `layout-templates: rows 221 · packs 3 · picked 60 (45 by every draw) · standing 0 · 12162 B`
  - `test-plan: rows 1329 · packs 14 · picked 278 (164 by every draw) · standing 65 · 69016 B`
  - `obs-plan: rows 1021 · packs 11 · picked 259 (152 by every draw) · standing 124 · 61222 B`
  - `a11y-plan: rows 235 · packs 3 · picked 48 (26 by every draw) · standing 20 · 13729 B`
  - `rows-union: 7 master(s) · picked 1097 of 4663 · rule any`
- **How the rows were read.** By search, never whole: the sum is 279 957 B, past 250 000 B. Every row the plan cites
  (24 handles) stands in its master's rows file (`grep -c -F` per handle, each ≥ 1; standing lists 0). The history
  files (7) were read whole.
- **`[RETYPED]` lines, verbatim; there were no re-spawns:**
  - `arch-pick-p4 · [RETYPED] line 9: decision/fake-agent-test-seam-variables is no row of the pack — read as decision/fake-agent-test-seams, the pack's row 9`
  - `obs-pick-p1.d2 · [RETYPED] line 81: decision/dialog-text-never-logged-on-answer is no row of the pack — read as decision/dialog-text-never-logged, the pack's row 81`
  - `obs-pick-p1.d3 · [RETYPED] line 28: decision/assistant-message-never-logged-on-readback is no row of the pack — read as decision/assistant-message-never-logged, the pack's row 28`
  - `obs-pick-p8.d3 · [RETYPED] line 1: decision/endpoint-and-pinned_bin-never-logged is no row of the pack — read as decision/endpoint-and-pinned-bin-never-logged, the pack's row 1`
- **Non-zero exits and refusals:**
  - The Bash guard refused a `cd` into the run dir (re-issued in a subshell) and a pipe of `rec.py query` into a
    filter (re-issued as a bare `rec.py find`).
  - `rec.py find` without `--doc` exited 2 (usage).
  - The write guard refused the rig script under `target/`; it was re-written in the session scratchpad.
  - The baseline runs of entries 6 and 8 read red, exit 4 `error: no tests to run`: the cases are not written yet,
    the red the plan predicts.
  - Live start 1 exited by SIGTERM (status 15) at its modal; start 2 by SIGTERM at its stop.
  - No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line was printed by any tool this run.
- **Where a letter was wrong or silent:**
  - The P3 rule re-runs the row read when a scope correction adds a surface. The surfaces added at P5 validation-1
    (the hook frame, the run dispatch, the harness lists) came from the founder's fork answers, not from a P3
    correction, and no letter says whether that re-runs the read. It was not re-run. The hook-frame rows were picked
    anyway: security history item 1 reaches `hook.event` re-validation, and the arch rows hold
    `contract/channel-methods/hook-event`.
  - The directive's «inside a started session's turn» met a CLI that ran local commands without a model turn; the
    sends went in once `session-start` stood and the input box was drawn.
  - The live-round letter (verification-harness rules) holds no recipe for a private-config interactive start. The
    rig was built here, and its seeded `.claude.json` (onboarding done, folder trusted) is in the journal.
- **Validation-1:** aligned. The scope was amended twice: P3's premise closure (item 2), and P5's surfaces list
  (intent-incomplete, from `inputs#I2` and `inputs#I4`).

## Beyond the plan
- Owner for the live re-stamp of `local-command-echo` on claude 2.1.325: «The 2.1.325 recorded fixture set» (its clean
  `viola verify --record` on 2.1.325), plus the operator's live-home re-stamp at the desk.
- The operator's desk list gains `target/live-localcmd-20261116/` and `$TMPDIR/viola-live-localcmd-125iyA` (tmpfs),
  the two live starts' scratch roots, holding the CLI's transcripts and kept screens.
- `/context`'s echo shape on 2.1.325 stays unmeasured: a CARRY onto «Standing live rig with a screen model».

Apply? (yes / review / cancel)
