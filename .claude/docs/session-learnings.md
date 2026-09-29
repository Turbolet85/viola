# Session Learnings

_This file is curated by `/wrap-session`. Learnings captured here are too detailed or specific for CLAUDE.md but worth preserving as reference material for future sessions._

_Entries are added in reverse chronological order (newest first). Each entry has an ISO date, short title, and body._

_This file is entirely wrap-session's territory. `/setup-project` creates it if missing but NEVER regenerates it. Manual edits are preserved across all Andromeda skill runs._

---

## 2026-09-29 — A gate's freshness check must name a file, never a directory
A plan gate's `artifact` key is judged by the named path's own mtime. `agent-run.sh run` overwrites its JUnit reports in place, and on NTFS overwriting a file does not move its directory's mtime, so an `artifact` naming `target/agent-run/artifacts/` reads STALE on every run while the run's files are fresh. Name a file the entry writes on every run (for the default `run`, `target/agent-run/artifacts/junit-nextest-integration.xml`), never the directory that holds it; a freshness red on a directory is an instrument defect to retarget, not a standing red to accept.

---

## 2026-09-29 — gate.py hygiene reads `/home/<x>/` in prose as a POSIX user home
The run-dir hygiene read (`gate.py hygiene`, P1) matches the POSIX form `/home/{user}/…` anywhere in a committed run dir or chunk evidence, including a repo-relative path to a kept test home written as `…/viola-test-X/home/instances/builder/…`, and refuses the file. In evidence prose, name such a file by its parts ("the builder instance's `detail-run.ndjson` in the kept home `viola-test-X`, under `ci/diag-windows-2025/`") rather than as a slash path through `home/`.

---

## 2026-09-29 — cargo-llvm-cov refuses `--no-report` with `--no-clean`
cargo-llvm-cov 0.9.1 rejects the pair (`--no-report may not be used together with --no-clean`): a `--no-report` run already keeps the previous build and profiles, so a loop that repeats `cargo llvm-cov nextest --no-report …` needs no `--no-clean` and must not pass it. A loop that runs the command many times should fail closed on an iteration that fails without the evidence it expects (here, no kept test report), so a usage error reads as a broken measurement rather than as a pass.

---

## 2026-09-29 — Pin `TMP`/`TEMP` in a Windows-runner `shell: bash` step that must find what a test wrote there
A Rust test's `std::env::temp_dir()` reads `TMP` first on Windows, while Git Bash keeps its own temp mapping, so a CI step that later looks for files a test wrote under the temp dir should export `TMP` and `TEMP` itself as a `cygpath -w` path (for example under `$RUNNER_TEMP`) and read from that same directory. Measured on the windows-2025 runner: every kept report the H2 loop printed was found there.

---

## 2026-09-28 — A plan whose test and probe contradict: narrow the probe
When a plan's own test must name a retired token to prove it is refused (a CLI test passing a removed flag) while the same plan's grep probe forbids that token anywhere in the tree, the two cannot both hold. The operator's ruling: narrow the probe to exclude exactly the refusal test (every other hit still reds), recorded in the plan's `note`, at the wrap. Never split the token in the test to dodge the grep, and never carry the probe as a standing red: a permanently red probe stops discriminating, and a dodged token hides the very reference the test exists to make.

---

## 2026-09-24 — Three pattern probes that match what they were not aimed at
A sweep or guard is only as good as its pattern, and three natural patterns in this repo match more, or less, than they appear to.
- A bare `grep owed` over the masters also hits `allowed`, `followed` and `showed`. Word-bound it (`grep -w owed`) before counting stale "owed" clauses.
- An absolute-path probe built on `[A-Za-z]:/` matches the `s:/` inside every `https://` URL. Exclude URL schemes, or match drive letters only at a token start, before calling a report path-free.
- A git pathspec `dir/**/name` does not match `dir/name`: without `:(glob)` magic, `**/` needs a directory level between. The fnmatch form `dir/*name` (where `*` crosses `/`) catches both.

The third was a guard that could never fire on its main case. Only a planted known-positive control exposed it, so run every new guard against a planted positive before trusting its silence.

---

## 2026-09-24 — A gate atom must not match what a green run can print
A `lacks` / `contains` atom on a test runner's log reads the whole log, and test NAMES are part of it. A `lacks failed` atom over a green nextest run went red because two passing tests are named `…_build_failed…`, and a `contains 0 failed` atom went red because nextest's green summary never prints a failed count at all. Assert on the runner's own summary token (nextest prints `N failed,` only on a red run), and read the token from a recorded log of the real run before writing the atom.

---

## 2026-09-24 — Reproduce a folded CI red on the host before planning its fix
A CI red folded into a chunk is closed against its recorded run. Still, reproduce its mechanism locally before the plan is written: the host can carry a second face of the same defect that the runner cannot show. The mutation gate's Rust-free-diff failure read as `outcomes-missing` on the clean CI runner. On the dev host, the same tool exit left an earlier run's `mutants.out/` in place, and the harness would have reported the stale counts as a pass.

The fix therefore had to cover residue, not only absence. Applies to any gate that reads an artifact a tool may silently not write: check the artifact is this run's, or remove it before the run.

---

---

## Entry format

Each entry follows this structure:

```
## {ISO-date} — {short title}
{1-3 paragraphs describing what was learned, why it matters, and where it applies. Reference specific files or documented decisions when relevant.}

See: `.claude/docs/services/{service}.md` (or similar cross-reference)
```

## Tier classification

This file is **Tier 3 — on-demand**. Claude reads it when explicitly needed (debugging, planning, reviewing patterns), not at session start.

Other tiers:
- **Tier 1** (always loaded) — universal safety rules in `CLAUDE.md` `USER:session-learnings` section (critical, short)
- **Tier 2** (path-triggered) — directives in `.claude/rules/*.md` `## Session Additions` sections (loaded when matching files touched)
- **Tier 3** (on-demand) — this file (detailed reference, lazy-read)

wrap-session classifies each learning into its tier automatically during curation (per its curation-tier-decision contract).

## Promotion

When this file grows beyond ~200 lines, `/wrap-session` suggests promoting some entries to topic-specific files (e.g., `.claude/docs/services/{service}.md` if the learning is about a specific service). Promotion is a user action, not automatic — wrap-session never moves entries without approval.

## Demotion from CLAUDE.md

If `CLAUDE.md` `USER:session-learnings` section gets too large (≥ 180 lines total CLAUDE.md), wrap-session suggests promoting old Tier 1 entries down to this file (Tier 3) to keep CLAUDE.md within size budget. This is also a user action.
