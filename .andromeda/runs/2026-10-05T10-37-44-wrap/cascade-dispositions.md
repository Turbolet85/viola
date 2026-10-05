# Cascade dispositions — wrap of 2026-10-05-real-cli-verify-probes

**The search.**
- **Patterns:** `cascade-patterns.toml`, 16 patterns, each control fired on the pre-pass masters (baseline `caae9eca`,
  the pre-CI parent).
- **Claims swept for:**
  - the six-row ledger, by its count and by the words `six rows`, `six-row`, `six spine ledger` and `six literal row`;
  - the `/06` counter and the `6 pass` summary;
  - the `:84` owner and the "Real-CLI verify probes" owner;
  - the fixed refusal text;
  - PROVISIONAL screen constants;
  - the `2.1.283` stamped set;
  - the six-value `subject` enum;
  - verify's "two spawns";
  - "print mode only" in CI (`single probe`, `ONE print-mode probe`);
  - "signatures not built" (`no signature set`, `not built yet`, `every build today`, `no signature row is compiled`);
  - "the one exception is verify";
  - "never used to read content";
  - `cli_verify` "12 cases";
  - "the window is an open item".
- **Sections read by hand:**
  - architecture: §Design Philosophy, §Stack, [Screen Model], [Delivery Confirmation], [Human Takeover / Wheel],
    [CLI Version Compatibility] (rows list and body), [Plugin Scope], §Standard Contracts → Ledger stamps envelope,
    §Occupied Resources (Binary, integration names, Filesystem, Repository) and Cross-cutting → Capability ledger;
  - the architecture keyed contracts: CI/CD approach, Project directory structure, Crate dependency direction;
  - security-plan: Threat Model → Child process spawning, Input Validation (child output row, Constants), Data
    Protection, Anti-Patterns → Universal;
  - design-system: §Surface: cli `verify` and the exit-code table;
  - layout-templates: §Surface: cli `verify` and the refusal lines;
  - test-plan: §2 Contract row, §4 viola-agent-claude, §5 CLI, §7 Fake agent and Fixture hygiene, and the
    `:84`-owner sites at `:233`, `:408`, `:749`, `:750`, `:757`, `:779` and `:781`;
  - the test-plan keyed contracts: 5-command (boot steps 4 and 5, `--local-live`) and Bootstrap phases (nextest);
  - obs-plan: §4 Edge flows `verify` and §6 (event table, child spawns, schema).
- **Not looked for:** the working-route line numbers of `:86` and later. P5 mints two entries before `:86`, and that
  renumbering is P5's manifest.

**Pass 1 listing** (`sweep-1.txt`). Every master row was dispositioned:

**Master rows:**
- test-plan `:233`, `:408`, `:749`, `:750`, `:757`, `:779` and `:781` (`:84` owner): **amended**. The local-command
  sites are re-pointed to "Local-command and paste-framing rows", and the dialog and permission sites to "Dialog rows
  and re-probe". These were stale citations of the amended owner claim, folded into this pass.
- architecture `:139` (the fixed refusal, missed by the detector): **amended** to the named form.
- design-system `:815` (the fixed refusal): **amended** to the named form.
- security-plan `:278` (the fixed refusal): **amended**. The new text quotes the retired form once as history ("It was
  …"), and pass 2 lists that as the `new` row at `:280`, a true quotation.
- These rows are true claims that share the token, so **no change**:
  - `provisional`, standing at architecture `:64`, `:65` and `:69`, test-plan `:1220`, obs-plan `:419` and `:1081`
    ×2, and a11y-plan `:823`: these are the spine, connect and dialog deadlines, not the screen constants;
  - architecture `:251`: the stamps-envelope example keeps `2.1.283`, a valid illustrative version key;
  - test-plan `:510`: "not built yet" is about the harness surfaces.
- These rows are this pass's own true text, so **no change**:
  - architecture `:48` and `:49` (`provisional` new, "no longer provisional");
  - architecture `:412` and test-plan `:663` / `:1084` (`2.1.283` drift-only);
  - architecture `:91` and test-plan `:653` ("ONE print-mode probe" plus the interactive runs).

**Leaf rows:** each is **re-derived** (step 3).
- CLAUDE.md `:43` (the warnings block): the dated exception now rides the ten-row stamp until "Dialog rows and
  re-probe". The overview's fixtures line gains the screens.
- `.claude/docs/commands.md:25`: verify's runs, the ten rows, `[NN/10]`, the screens, the named refusal, the dev-host
  folder and CI's modes.
- `.claude/docs/gotchas.md:75` and `:79`: the owner, and the full gate on a verified CLI.
- `.claude/docs/security-summary.md:38` and `:70`: the ten-row stamp and its owner, and the cross-session owner. A new
  bullet records the ratified probe widening, the residual and the prerequisite.
- `.claude/docs/services/viola-agent-claude.md:11`, `:23` and `:25`: `SIGNATURES`, `rows()`, the compiled constants,
  the ten rows and the new helpers.
- `.claude/docs/services/viola.md:6` and `:31`: verify's two interactive runs and four spawn pairs.
- `.claude/docs/stack.md:14`: the Screen model row.
- `.claude/docs/tests-summary.md:21`, `:26`, `:27`, `:28` and `:61`: the fake-agent modes, the sets and the screen
  walk, the owners, and the window.
- `.claude/docs/obs-summary.md:31`, `.claude/docs/services/viola-core.md:6` and `.claude/docs/tests-summary.md:44`
  (`provisional`): spine, dialog and connect deadlines, so **no change**.
- `.claude/docs/commands.md:5` ("not built yet"): unrelated, so **no change**.
- Beyond the sweep, by provenance (the leaves whose source sections changed):
  - `.claude/rules/verification-harness.md`: boot step 4's verify flags and supervise's flags;
  - `.claude/rules/security.md`: one bullet for verify's PTY children, the no-key rule, the dirs, `~/.claude` and
    the named refusal.

**Pass 2 listing** (`sweep-2.txt`, after the apply): `six-row`, `slash06`, `six-pass`, `route84`, `real-cli-entry`,
`subject-enum`, `two-spawns`, `one-exception`, `never-content`, `cli-verify-12` and `window-open` read 0 rows, each
with its control fired. Every remaining row is one dispositioned above as a true claim or a quotation.

**Curation homes and judgment bases:** 0 rows on every pattern in both passes.
