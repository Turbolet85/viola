# Cascade dispositions — 2026-09-29-fake-agent-drift-contract

**The search:** `cascade.py sweep --patterns-file cascade-patterns.toml` (13 patterns, every control fired over the seven
masters' pre-pass text at `2d8bc53`; `escapedtyped` first refused at a 60-char window, re-controlled at 120 on its
architecture :89 known positive). Patterns, one per retired claim or its mechanism: the prefix literals `agent-message
from=` · `<task-notification>`; the class `harness[- ](prompt )?prefix` · `harness[- ](injected|turn)`; the retired
"escaped/typed tag never classifies" mechanism `typed.{0,120}escap|escap.{0,120}typed|never classif`; insta as the
fake-agent pin `\binsta\b` · `fake-agent transcript|hook sequences?`; the matcher owner `matchers?\b` · `drift contract`;
the S8 owner `annotations`; the resize-oracle mechanism `resize oracle|size changed|before a byte read|re-read its size`;
the hook receipt field list `stdout_hex`; the UserPromptSubmit payload construction `only the top-level .prompt.|with
.prompt. set|only .prompt. set`. Scope: the seven masters, the three curation homes, the two judgment bases, the leaves.
The listing is `.sweep-listing.txt` beside this file. Long lines (architecture :49, :70, :91) resolved by the `@c`
offsets the listing printed.

## Masters — `new` rows (this pass's own text): 20 — no change
architecture :70 (agentmsg, tasknotif), :80 (agentmsg, tasknotif), :89 (harnessprefix); security-plan :573 (agentmsg,
tasknotif), :774-777 (agentmsg, tasknotif, harnessprefix ×2, escapedtyped); test-plan :1973-1979 (insta ×2, matcher ×3,
driftcontract, annotations ×2, resizeoracle) — the applied amendments themselves, re-read.

## Masters — `standing` rows
- **Amended this pass (edited):** architecture :70 (harnessturn ×3 @c502, c545, c2953 — the new sentence and "The
  prototype paused twice on harness turns before the classifier was fixed", true), :80, :89 (escapedtyped mixed ×4 —
  re-read whole: the typed-tag escaping statement stays true, the exception appended; no intra-line duplicate of a
  retired claim); security-plan :573 (harnessprefix mixed ×2 — the ban stands, the set appended); test-plan :270, :438,
  :872, :1336, :1337, :1359, :1366, :1372 (×2), :1379 — no swept text left standing against the amendment.
- **True claims sharing the token — no change:**
  - architecture :91 @c1939 "tag escaping, harness prefixes and the R8 identity floor with the first live test" — the
    ledger ROW still lands with the first live test (route CARRY this wrap).
  - test-plan :57, :60 — M2 harness prefixes / real harness-injected turns as live-test subjects, unchanged.
  - test-plan :1123, :1382 — `--inject-harness-turn` fires `<agent-message …>` / `<task-notification>` prompts: the
    mode's behaviour, unchanged. :1132 — harness turns log `origin:"harness"`, no `wheel`: true.
  - test-plan :1382 promptkey "UserPromptSubmit/default with `prompt` set" — print mode, true.
  - test-plan :1801 (§12 history) "matchers deferred", `--inject-harness-turn` — history, true.
  - design-system :107 "harness turn" — the Info token row, unrelated.
  - architecture :49 @c1062, test-plan :887 — the send-confirmation matcher, a different subject; architecture :63,
    :357 — the PreToolUse matcher `AskUserQuestion|ExitPlanMode` registration, true.
  - insta: architecture :499 (`snapshots/` dir), test-plan :442, :463, :768 (dev-deps catalog), :839, :866, :1099,
    :1347, :1641 — the decision-body / check-mode tooling, still the dialog tier's; none states the fake-agent half.
  - annotations: architecture :76, :277; test-plan :252, :866, :1090, :1100 — the S8 dialog path, owned by dialog
    answers, true; test-plan :408 — the contract-suite REQUIREMENT including S8 forwarding stands (its owner stated at
    :1337); obs-plan :1253 "CI annotations" — unrelated.

## Curation homes (`curation`): 0 rows. Judgment bases (`base`): 0 rows.

## Leaves (`leaf`) — added to step 3's set, recomputed
- `.claude/rules/events.md:22` — re-derived: the four prefixes + the escaped cross-session exception.
- `.claude/docs/services/viola-agent-claude.md:23` — re-derived (prompt order: the four raw prefixes, no trim);
  `:41` — re-derived (the contract: byte for byte through `stdin_hex`, spine literal, no insta); `:22` (ledger-row list
  naming "harness prefixes"), `:20` (decision bodies incl. `annotations`), `:37` (insta-pinned decision bodies) — true,
  no change.
- `.claude/rules/verification-harness.md:39` — re-derived (matcher owner; the kept trailing newline); `:41` receipt
  kinds — re-derived (`hook` `stdin_hex`, `size` by a watcher); `:42` `--inject-harness-turn` — true.
- `.claude/docs/tests-summary.md:21` — re-derived by recompute (the drift contract joins the fake-agent line); `:28`
  insta-pinned decision bodies, `:29` harness turns never flip the wheel — true.
- `.claude/docs/services/viola.md:22` harness-injected prompts never move the wheel — true; `:41` confirmation matcher —
  unrelated.
- `.claude/rules/testing.md:18`, `.claude/docs/stack.md:50` — insta 1.48.0 in the framework catalog — true (dialog tier).
- Recomputed by provenance with no swept hit: `.claude/docs/security-summary.md` §Critical decisions gains the harness-
  origin decision (the Decisions Log `2026-09-29` entry); `.claude/rules/security.md`, CLAUDE.md `GENERATED:setup:*`
  (overview · modules · warnings · pointer-table · architecture) — no statement of the classifier, the fake-agent pin,
  the resize oracle or matchers; no change.
