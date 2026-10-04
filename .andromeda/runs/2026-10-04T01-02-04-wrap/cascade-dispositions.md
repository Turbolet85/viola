# Cascade dispositions — 2026-10-03-mutation-scoring-completion

**The search:** `cascade-patterns.toml`, 14 patterns, swept with `cascade.py sweep` over the seven masters, every
`.andromeda/registries/**` file, the three curation homes, the two judgment bases and the leaves (baseline `aa300a4`, the
pre-CI parent). Every control fired on the pre-pass text.

**Patterns, keyed on the retired claims' tokens AND their phrasings:**
- the WSL crossing: `WSL|wsl\.exe|wsl-` · `distro`;
- the retired stages and values: `pre-push-windows-only` · `windows-tests` · `vm-release` · `sync-mismatch|sync-failed` ·
  `CARGO_BUILD_JOBS`;
- the clone and its scratch: `viola-pre-push` · `target/pre-push`;
- the root launch: `install-deps`;
- the provisioning claim: `provisioning|viola-provision`;
- the two-host phrasing: `Linux leg`;
- the two-arm verdict phrasing: `counted or scoped|counted and scoped`;
- the verbatim-copy policy: `pending wording`.

A pattern `stay verbatim|never bring §1` was dropped, because its control never fired over the masters: that wording
lives only in `playbook.md`, which was edited directly and is controlled by hand below.

**Sections read in full besides the rows:**
- architecture §Stack rows 36–37 and §Occupied Resources (383, 403, 424, 434);
- the key files `ci-cd-approach.md` (every line; line 5, 3 320 chars, read by offset) and
  `project-directory-structure.md` (the `scripts/` subtree);
- security-plan Pinning (332–337), Development (444–446) and Anti-Patterns (590);
- test-plan 437, 1153 (by offset), 1209 and 1221;
- the test key files `5-command-implementation.md` (lines 24, 42–48, 56–57, 65, 151–165) and
  `bootstrap-phases-…md:48`;
- obs-plan 485 and 966–969.

The two sweep listings' raw captures were moved to the session scratchpad at P7.3c: they quote a retired leaf's host
path. Their rows are dispositioned below, and the tool's own trail is `cascade-2026-10-03-mutation-scoring-completion.json`.

## Rows of the first sweep (85 lines)
- **Master `standing` rows (stale text) — all amended this pass:**
  - architecture 36, 37, 383, 403, 424 (entry deleted), 434;
  - `ci-cd-approach.md` 4, 5 (the "WSL provisioning carry no perf step" clause, a site the detector missed, found by the
    `provision` / `wsl` rows), 21;
  - `project-directory-structure.md` 63–68 (rows removed);
  - security-plan 332, 337, 445, 590;
  - test-plan 437, 1153, 1221;
  - `5-command-implementation.md` 24, 44, 46–48, 56 (the `arms` row: "read only on the `counted` and `scoped`
    verdicts", amended), 57, 65, 151–156 (block rewritten), 161–164;
  - `bootstrap-phases-…md:48`;
  - obs-plan 967, 969.
- **Master rows left standing, a true claim sharing the token:**
  - `obs-plan.md:1009` and `registries/contracts/obs-plan/bootstrap-phases-…md:33`: `Linux leg` names the CI `lint`
    job's lint-probes leg, not pre-push. No change.
- **Leaf rows — every one re-derived from the amended master:**
  - `.claude/docs/commands.md` 12–14 (WSL setup lines removed, replaced by the native Node install line), 36 (pre-push
    rewritten), 37 (`wsl-exec.sh` removed);
  - `.claude/docs/gotchas.md` 126–129 (the `wsl.exe -- cmd` gotcha removed: no WSL call remains);
  - `.claude/docs/security-summary.md` 67;
  - `.claude/docs/stack.md` 32–33;
  - `.claude/docs/workflow.md` 11;
  - `.claude/docs/tests-summary.md` 15;
  - `.claude/rules/security.md` 35;
  - `.claude/rules/verification-harness.md` 24.
- **Curation row:** `.claude/rules/testing.md:72` (Session Additions, 2026-10-03: "Linux clippy in pre-push's WSL clone
  can, before the push"). Routed to P3 as an in-place extension; the cascade never edits a curation home.
- **Base row:** `.andromeda/playbook.md:40` (`pending wording`). The rule is superseded by directive 1 and kept verbatim;
  its `note:` now opens `SUPERSEDED 2026-10-04 (…)`. Controlled by hand: `:44` takes the same mark, and the new rule
  is appended. The propose → approve → append channel ran on the founder's ruling.

## Leaves outside the rows (enumerated by provenance and by the amended mechanisms)
- **Re-derived:**
  - `.claude/docs/commands.md` 30 and 35 (`[--package <member>]` added to the grammar), 45 (the package form added);
  - `.claude/docs/tests-summary.md` 42 (the Mutation row: `--package`, the not-measured rule, NOCOW `TMPDIR`);
  - `.claude/docs/obs-summary.md` 51–52 (obs-plan §8 item 6; the old wording "Windows repo/home path, Linux clone"
    carried none of the swept tokens and was found by reading the obs leaf);
  - `.claude/rules/verification-harness.md` 44 (`--package`, the prefix pin);
  - `.claude/rules/testing.md` 48 (`--package`).
- **No change:**
  - CLAUDE.md `GENERATED:setup:*` (recomputed: no block names pre-push, WSL or the scripts);
  - `.claude/docs/conventions.md`, `services/*.md`;
  - `a11y-summary.md`, `design-summary.md` (no hit, no amended source behind them).

## Rows of the final sweep — what still matches, and why
- **`5-command-implementation.md:165`** (`wsl`, `prewin`, `wintests`, `vmrel`, `sync`, `distro`): the closed-enum line's
  deliberate retired-values list ("Retired with the WSL gate by the mutation-scoring-completion chunk: …"). The line
  names each addition's chunk already, and a reader of an older document classifies a retired value by it. No change.
- **`architecture.md:403`, `security-plan.md:337`, `test-plan.md:1153`** (`provision`): this pass's own "no provisioning
  script exists / remains". Current truth. No change.
- **`testing.md:72`** (curation): routed to P3.
- **`playbook.md:40`** (base): superseded and kept verbatim, per the ruling.
- **`obs-plan.md:1009`, the obs bootstrap key `:33`** (`Linux leg`): a true claim. No change.
