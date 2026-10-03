# Operator pass — 2026-10-03

Run on the overseer's word ("run the operator pass now"), after /implement run `2026-10-02T19-38-47-implement`.

## Entry 23 — `gate.py hygiene`
- First reading: `refused 4 files — P1 4`. These were the phase run's dry-run captures
  (`.andromeda/runs/2026-10-02T12-57-04-phase/p4-dryrun.txt`, `p5-dryrun.txt`, `p5-dryrun-2.txt`, `p5-dryrun-3.txt`), each
  at line 2 with ×4 hits.
- Redacted on the overseer's word, with the files kept. Each file had 4 host paths outside the repository: the resolved
  shell (→ `<git-bash>`), the gate log dir under the user temp dir (→ `%TEMP%\`), and the two `leg = 'operator'` command
  texts naming the skills dir under the user profile (→ `~/.claude/skills/`). The rewrite was binary-mode: every other
  byte was kept, and the CRLF counts were unchanged (34 · 38 · 38 · 35).
- Re-read: `hygiene: clean — read 48 (runs 40 · evidence 8) · trails 11 not read · binary 0 not read by P1`.

## Entry 19 — `agent-run pre-push` (taken at /implement, on this tree's code)
- Stages:
  - `tools`, `sync` (63 files), `cache` (19.7 GB of a 40 GB cap, not cleaned): passed.
  - `linux-tests`: **green** (coverage 951 passed / 0 failed · Playwright 1/0 · `gate` ok).
  - `vm-release`: terminated.
  - `windows-tests`: **red**, coverage 907 passed / **87 failed**.
- **Basis:** every `windows-tests` failure is the M2 class, a `watch.rs` deadline or the hook's spine bound on D:. It is
  two-sided witnessed, same command, both orders, same day: D: 22 passed / 40 failed against the C: copy 62/0, then D:
  22/40 against C: 62/0 (`m2-diagnosis.md`). M2's cause is the D: volume's per-operation filesystem latency, outside the
  repository; the founder decides it. Per the host-reds CARRY on `working-route.md:66`, **CI is the acceptance leg**. On
  the overseer's word this red is recorded and not chased. Not this chunk's red.
- The code under the pre-push equals the committed code. After the pre-push, the only code edits were a `verify.rs` change
  that was reverted (byte-equal to HEAD) and `home.rs` swapped for the leak arms and restored (byte-equal to the gated
  file). Both were confirmed by `cmp` / `git diff --quiet`.

## Entries 24–25 — push and the CI read
Recorded below once taken.
