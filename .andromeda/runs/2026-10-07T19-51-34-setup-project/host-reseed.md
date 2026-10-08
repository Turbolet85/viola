# Host leaf re-seed: the proposed sort

`.claude/rules/host-win32.md` (rendered for win32, 10 467 B, `leaf_md5 3a7d7ee5fd4397db346ae6d134cac067`) becomes
`.claude/rules/host-linux.md`. This host is linux (`upgrade.py host`). Nothing is moved until the card's word. The
machine form of this list is `host-reseed.json` beside this file; an item moves byte for byte or not at all.

The dry run's numbers (`upgrade.py apply --id U04 --sort host-reseed.json --dry-run`, exit 0):

- 10 467 B → 4 506 B loaded on every turn;
- 12 items, 3 988 B: keep 5 · learnings 4 · `verification-harness.md` 1 · drop 2;
- `.claude/rules/verification-harness.md` (`paths:` 6 globs, 18 515 B) gains 1 line;
- `.claude/docs/session-learnings.md` (Tier 3, read on demand, 12 353 B) gains 20 lines;
- one line still names `host-win32.md` afterwards, `CLAUDE.md:105`, which the P7.5 re-render rewrites.

## The 12 items below `## Session Additions`

| n | line | bytes | dated | to | what it says | why |
|---|---|---|---|---|---|---|
| 1 | 90 | 340 | 2026-09-24 | learnings (mixed) | a running `.exe` cannot be relinked on Windows; a tool that runs cargo while its own exe runs uses a separate `CARGO_TARGET_DIR` | the relink refusal is Windows-only, the rule still binds the windows-2025 CI leg; the harness rule's body already carries the `target/harness` form, so the rule-file route would duplicate it |
| 2 | 91 | 677 | 2026-09-24 | drop | a stopped Monitor leaves `tail.exe` / `grep.exe` holding files open, which blocks a directory rename; poll with `cat`, never `tail -F` | old host only: Windows handle semantics, and its correction describes the Windows host's mutation scratch |
| 3 | 92 | 196 | 2026-09-24 | keep | `grep` over a one-file glob prints no `file:` prefix; use `grep -H` | live on any host |
| 4 | 93 | 156 | 2026-09-24 | learnings (mixed) | under Git Bash a slash-mapping parameter expansion deletes forward slashes; use `tr` | names Git Bash; the repository's scripts still run under Git Bash on the windows-2025 CI leg |
| 5 | 94 | 225 | 2026-09-24 | learnings (mixed) | a path a native Windows tool reads is written as `pwd -W`, falling back to `pwd` | names Git Bash and native Windows tools; live in `ci.yml` and two scripts that run on the windows-2025 CI leg |
| 6 | 95 | 212 | 2026-09-25 | drop | stop a process by its exact `ExecutablePath` (`Get-CimInstance Win32_Process`) | old host only: a PowerShell recipe; the linux leaf's template carries this host's form (`pgrep -f` / `pkill -f` self-match) |
| 7 | 96 | 241 | 2026-09-27 | learnings (mixed) | in the interactive Git Bash `rg` is a shell function; probe a tool with `type`, put a pinned tool on PATH inside the command | names the interactive Git Bash; the directive is live on any host |
| 8 | 97 | 545 | 2026-09-28 | keep | the Bash guard refuses a doubled backslash and a heredoc redirected to a file; use the Edit or Write tool | live on this host (met again at the last chunk), names no host |
| 9 | 98 | 493 | 2026-10-03 | `.claude/rules/verification-harness.md` | cargo-mutants' tree copy: a short `TMP` on Windows; on the Linux dev host a NOCOW btrfs `TMPDIR` | a mutation leg recipe (harness driver class); its sibling `TMPDIR` rule already stands in that file's Session Additions |
| 10 | 99 | 279 | 2026-10-07 | keep | on the Linux dev host a process named `viola` is not this repository's by name; decide by `/proc/<pid>/exe` | a Linux dev host fact, live |
| 11 | 100 | 355 | 2026-10-07 | keep | a removal the permission layer denied is never done through another tool; move the scratch home into `target/e2e-home.disk/` | live on any host |
| 12 | 101 | 269 | 2026-10-07 | keep | a backgrounded command's output file ends with the harness's own exit line; match the summary anywhere | live on any host |

Titles for the four Tier-3 entries (`.claude/docs/session-learnings.md`):

- 1: A tool that runs cargo while its own exe runs uses a separate CARGO_TARGET_DIR
- 4: Git Bash parameter expansion deletes forward slashes: normalise separators with tr
- 5: A path a native Windows tool reads is written as pwd -W
- 7: Probe a tool with type: a gate's non-login bash holds no shell function

## Judgment calls in this sort

- **Items 4, 5 and 7 go to learnings, not to drop.** Each names only Git Bash, which the letter's second rule
  sends to `drop`. They are sorted as mixed because `ci.yml` still runs the repository's scripts with
  `shell: bash` on `windows-2025` (and `pwd -W` stands in `ci.yml:105`, `scripts/install-ripgrep.sh` and
  `scripts/lint-probes.sh`), so the lessons still bind script authoring. `host 4 drop` (or 5, 7) reverses any one.
- **Item 1 goes to learnings, not to `verification-harness.md`.** Its rule covers any tool (the harness, a live
  supervisor, a wrapper), and that file's generated body already states the harness's own form at line 28.
- **Item 9 goes to `verification-harness.md`.** It carries the one Linux dev host fact about a mutation run
  (the NOCOW `TMPDIR`), next to the 2026-10-04 `TMPDIR` rule at that file's line 60. It loads when a harness file
  is read, not on every turn. `host 9 learnings` sends it to Tier 3 instead.
- **Items 2 and 6 are dropped into the run dir**, `host-reseed-dropped.md`, which the commit carries.
  `host drop→learnings` sends both to Tier 3 instead.

## `USER:session-learnings` bullets that name the old host (listed, never edited)

One bullet, `CLAUDE.md:116`:

> Prompt text reaches viola only from stdin or `--file`, never from a leading-slash argument (Git Bash rewrites
> `/skill` into a Windows path).

It reads as a fact about viola's own users on Windows, the product's live-supported target, so it may need no
correction. If it does, the door is a 0-pending `/andromeda-wrap-session`, asked for the correction; setup edits
no `USER:*` bullet.

## Template content the new leaf gains or loses (the tool's render, not this sort)

- Gains the `linux` section: `## Processes`, the `pgrep -f` / `pkill -f` self-match rule.
- Loses the `win32` sections: the Git Bash preamble, MSYS path conversion, `/tmp` against the Windows temp dir,
  `grep -P` on the host locale, the PowerShell redirect, `## Processes & ports`, forced UTF-8 on python, the
  7.5 KB command cut, and the `core.autocrlf` re-checkout recipe.
- The old leaf's `## Long single-line files` section stands in no host section of the installed template
  (`rules-templates/host.md`), so the rendered `host-linux.md` will not carry it. It is body text above
  `## Session Additions`, not one of the 12 items, and the old leaf is backed up to `.claude/backup/` before the
  re-seed. Its anchored-Edit rule and the four-assertion python write are measured on LF-pinned files on any
  host. Named here so the loss is a decision, not a side effect.
