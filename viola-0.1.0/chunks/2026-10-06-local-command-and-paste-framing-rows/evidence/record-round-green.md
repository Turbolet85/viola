# Step 14 — the second record round on `claude` 2.1.287: GREEN, fired once

**Outcome: `stamped 2.1.287  17 pass  0 fail`, exit 0. The four new variants were recorded and copied into
`fixtures/claude/2.1.287/`. The round was fired once.**

## How it ran
- The five ledger rows (8-12 of `live-sessions.md`) were written at 2026-10-06T21:14:12Z, before the round.
- Before it, with no session: step 13's thirteen entries read green (the two lint entries, the unit entry, the
  four named unit entries, step 12's case and the five guard probes), and the tree's three framing arms and
  `--record`'s scrub were run over the rehearsal's own ten captures by the scratch helper (`rehearsal-shapes.md`):
  the three rows read `true` and the four variants read clean.
- Fired once through `gate.py run --live-legs`, 21:14:29Z to 21:15:17Z: entry 8, the version probe, green
  (`2.1.287 (Claude Code)`, 0.01 s); entry 9, the record entry, green after 47.8 s, its artifact fresh. Its lines
  and the summary are in `round-211429Z.txt`: `round: COMPLETE · legs fired 1/1`. No survivor was reported and
  no ref moved.
- The record home is `target/e2e-home/viola-record-20261006T211429Z/` (git-ignored). Its stamp names 2.1.287 with
  17 rows, all `pass`; no home outside it was written.
- All seventeen step lines read `pass`, `[15/17] long-paste-wrapper`, `[16/17] tag-escaping` and
  `[17/17] local-command-clear` among them; the last stdout line is `stamped 2.1.287  17 pass  0 fail`.

## What was recorded
`--record` wrote 23 files into the record home's `out/2.1.287/`: the spine, the three screens, the twelve dialog
variants and the four new ones. The `cp` took only the four new ones; each is byte for byte its recorded file.

| file | bytes | what it holds |
|---|---|---|
| `UserPromptSubmit.paste-1.json` | 2 071 | the long text's prompt, 1 558 chars: two newlines, the pair with id `7602`, one newline |
| `UserPromptSubmit.paste-2.json` | 712 | the tag-like text's prompt, 202 chars: both typed `pasted_content` tags escaped, `<task-notification>` as typed |
| `SessionEnd.clear-1.json` | 475 | `reason` `clear`, with `prompt_id` |
| `SessionStart.clear-1.json` | 426 | `source` `clear`, no `model` key |

- All four passed `--record`'s scrub with no refusal (`home-path` · `absolute-path` · `username` · `email`): the
  recording was written whole.
- The paste id is `7602`: a fourth id for the same text (`7ccf` at step 0, `31a3` in the first round, `dead` in
  the rehearsal), and the same frame each time.
- No committed fixture was modified or deleted: `git status --short fixtures` lists the four files as new and
  nothing else. `git ls-files --eol` reads `w/lf` for each.

## After the round (2026-10-06T21:15:42Z)
- No probe dir of this chunk is left at the repository root. `.viola-verify-2095228/` predates the chunk (the
  operator desk, the founder's word: leave it). No `viola-verify-*` dir is left under the OS temp dir.
- No `claude` process has a probe-dir cwd (`pgrep -x claude`, each cwd read: none under a record home or a
  `.viola-verify-*` dir).
- Residual: under `~/.claude/projects/` the CLI wrote 2 transcripts for Run B (the session and the one `/clear`
  opened), 1 for Run C and 1 for Run D, the accepted class. Run B's is one higher than a plain Run B, as inputs#I2
  says.
- Sessions: 12 of the cap of 12 are used. None is left and none is needed.
