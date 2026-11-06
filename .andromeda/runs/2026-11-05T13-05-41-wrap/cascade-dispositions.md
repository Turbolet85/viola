# Cascade dispositions — 2026-11-05-linux-only-code-and-ci (cascade step 2)

total (551 patterns) · 655 rows over 62 files
per class · new 11/6 · standing 105/32 · leaf 513/26 · curation 26/4 · base 0/0

## The search
- One `cascade.py sweep` call (cascade v1.26, baseline `ecbe7ab3`, the parent of the oldest pre-CI commit `d3690534`)
  over every apply writer's `@pattern` records — `apply-a11y-plan.p1`–`p3`, `apply-architecture.p1`–`p6`,
  `apply-design-system.p1`–`p2`, `apply-layout-templates`, `apply-obs-plan.p1`–`p4`, `apply-security-plan.p1`–`p5`,
  `apply-test-plan.p1`–`p7` (ids `a11y1-*` … `tp7-*`) — and main's `cascade-patterns.toml` (ids `m-*`: the OS target,
  the retired «Windows-only live measurements», `batch-script-child`, the sideload / ConPTY words, the `os` enum, the
  `*_REPORTED` constants, the Windows relied set, the `{sh,ps1}` pair, the Windows CI legs and scripts, the pipe / SDDL
  / DACL words, the `.cmd` / `.bat` shim, the case fold, the replace retry, the macOS gap, the VT enable, `recvmsg`, the
  Windows PTY APIs, the e2e-home floor, the `process-exit` detail count, verify's codes, the nextest kill line, the
  `WITHIN` waits, zizmor's suppressions, the registry reader, the deny triples, the console-control stop,
  `seed_conpty`). Every pattern's control fired (no refusal). Sections read: every master body, category file and key
  file, as the store renders them; the rows were read at their facts (`rec.py get`, `--at-char` for a long line) and
  each named file, symbol, job or count checked against the tree (`.github/workflows/ci.yml` one leg
  `[ubuntu-latest]`, `.config/nextest.toml:24` 26 `binary(` names, `grep -rn 'Instant::now() + WITHIN' tests/` 31 in
  23 files, `grep -rcE 'human::refuse\(' src` run 7 · verify 5, `deny.toml` one triple and one 0BSD exception).
- Re-fired at this wrap's resume (window 3) with the same argument list; the listing above is that call's.
- Not looked for: wording of the retired mechanisms no pattern names. A zero-row pattern is a statement about that
  pattern only.

## Master rows (new 11 · standing 105) — by fact
Amended rows name their edit in `edits-main-7.md` (TX 339; the replaced originals retired TX 340–341).

### Stale → amended (routine, this pass)
- fct-y2h27pfd (obs key `bootstrap-phases-derive-for-route-setup-project`:20, obs2-12) — «on all three targets» → its
  one target `x86_64-unknown-linux-gnu` · E1.
- fct-ad44nn2e (architecture established-decisions:14, m-sideload) — the reason «because ConPTY does not close the output
  stream on exit» named the retired backend; the rule stands, the reason dropped · E2.
- fct-4u2vf5lk (architecture.md:373, m-win-ci) — the present-tense `windows-2025` drive reason dropped; the placement
  stands · E3.
- fct-mw7zqeom (obs span-trace-coverage:69, m-sideload) — «(ConPTY chaos class)» dropped, the handle-wait rule
  stands · E4.
- fct-z43sni6d (test key `bootstrap-phases-derive-for-route-setup-project`:43, m-win-ci) — the retired runners' jq
  readings dropped, the ubuntu reading kept · E5.
- fct-lzdm6e7w (test integration-test-strategy:108, sec4-01) — «(the plain drive form on Windows)» dropped, matching
  security-plan's sibling fact fct-l7hy7pzk re-worded this pass (the units `plain_drive_*` still exist) · E6.
- fct-mos7j36w (test-plan.md:770, a11y1-03) — «the per-OS `test` job» → «the `test` job» · E7.
- fct-47aec6c3 (test key `5-command-implementation`:127 ×2, a11y1-03) — the same, twice · E8.
- fct-jv5gqhbl (architecture occupied-resources:232: a11y3-01, tp4-10) — «green on the three CI runners» → «green on a
  run of the retired three-OS matrix» (the e2e facts' wording this pass) · E9. Its m-within-waits ×3 rows: true (31 in
  23, re-measured).
- fct-hgewdmj2 (test key `5-command-implementation`:27: a11y3-01, tp4-10) — the same · E10. Its m-within-waits rows
  (×3, @c1951 and @c+2030,2087): true (the count and its rule, 31 / 23).
- fct-nuclnamd (security input-validation:45: arch1-02, arch5-14, arch6-01) — «(the Unix R8 pass-list)» → «(the R8
  pass-list)» · E11.
- fct-2j6rht5l (test key `bootstrap-phases-derive-for-route-setup-project`:11: arch1-06, m-os-target) — «on three OSes
  under `ci` (as measured at ci#39383206773)» → the reading kept, «a run of the retired three-OS matrix» · E12. Its
  m-kill-line rows (:10 ×3 @c4059 / @c+4444 / @c+4847, :12): true (dated «no kill line moved» readings; the mutants
  profile's kill line, nextest.toml:33, :41–59).
- fct-73ei4s3r (obs-plan.md:65: arch1-08, tp1-25) — «npm-shim resolution» → «child program resolution» · E13.
- fct-enn2bam2 (security-plan.md:493: arch1-26, arch3-33, arch5-15) — `viola(.exe)` → `viola` · E14.
- fct-5akis2u7 (test-plan.md:249: arch1-32, m-os-target) — a quotation of architecture text fct-fa2klbw5 no longer
  holds; re-quoted to its current words · E15.
- fct-zxaumenb (test key `5-command-implementation`:4, arch5-33) — the four `[.exe]` sites (one row; the other three
  found by `rec.py find --doc test-plan --text '[.exe]'`, the same fact) dropped · E16. Its m-msrv-floor row (:6): true
  (`Workspace::ensure_e2e_home` stands).
- fct-smfiay44 (obs span-trace-coverage:53: obs2-18, obs3-06, m-sideload ×4, m-pipe-acl) — the `run.conpty_sideload`
  (Windows) bullet removed: a line inside a fact, so the fact re-written whole without it (`as=s1`, E17), the original
  retired (TX 340).
- fct-jzj7djty (security data-protection:10, :11, :18: sec4-18, sec4-19, m-sideload ×8, m-win-ci, m-pipe-acl ×2 ×2) —
  the two Windows DACL bullets and the `conpty/` companions bullet removed the same way (`as=s2`, E18; the original
  retired, TX 341).

### Already the post-chunk text (amended this pass) — true
- fct-7wuxnmfc (architecture.md:388: tp1-43, m-win-ci), fct-xtqblyzi (occupied-resources:309, new: tp1-43, m-win-ci),
  fct-ocdr3xdn (quality-gates:16, :18 ×2, :20: tp1-43, m-win-ci), fct-ydc7ueix (pii-scrubbing:47: tp1-43, m-win-ci) —
  `viola-mutants-scratch` named as the operator's Linux NOCOW `TMPDIR`, an environment fact; the Windows leg named as
  retired; the run-cited `windows-2025` reading at :16 a dated history line.
- fct-v2lh3e2i (security threat-model-summary:181, new: m-sideload, m-win-ci) and fct-vf6cfvel (architecture key
  `ci-cd-approach`:2: m-win-ci, m-zizmor) — the retirement statements (tag `multi-os-final`), and two suppressions,
  no `concurrency:` block.
- fct-kylcwwrw, fct-5eczp6gf, fct-53es7vlo, fct-fbkvffnw (test e2e-test-strategy:187, :206, :231, :250, new:
  m-os-target) — «a run of the retired three-OS matrix»: dated history.
- fct-fa2klbw5 (architecture.md:417), fct-gldxhydz (test-strategy:73), fct-t2ak7i7a (obs ci-integration:26)
  (m-os-target) — «every OS-gated test» names the census `tests/contract_os_gated_tests.rs` (it exists): a different
  subject sharing the token.
- fct-qwpklckh (established-decisions:234 new, :247, :258: m-cmd-bat, m-relied-win) — `shim-resolution` reads
  `claude --version` answering; :247 the row id list (a label of `LedgerRow::ALL`); :258 «twenty-one» the stamp id-set
  history, not the Windows relied set.
- fct-62qgonvf (standard-contracts:33, :35: m-cmd-bat, m-relied-win) — :33 an example `stamps.json` (quotation, the id
  stands); :35 the stamp-history counts beside «all twenty-four, on every build».
- fct-zvkglgpg (security-anti-patterns:26), fct-tuv4h7zt (test key `5-command-implementation`:43 @c2296; its
  m-msrv-floor @c528 true too) (m-relied-win) — stamp-history counts and a dated 21-row live firing.
- fct-sx3u7aez (design surface:127 ×2), fct-hk2gtsxu (layout-templates:176), fct-pf5uzbca (layout-templates:206)
  (m-cmd-bat) — the `[01/24] shim-resolution …` literal (as pinned by `tests/cli_verify.rs:27`) and the ledger order.
- fct-32d4ty7r (security-plan.md:246) and fct-cotc6dxt (architecture key `build-system`:8) (m-recvmsg) — `doctest-file`
  the one 0BSD exception; no `recvmsg` (deny.toml:21–23).
- fct-26ufyipy (obs-plan.md:311) and fct-hfdtlj6j (obs-anti-patterns:8) (m-verify-codes) — the counts read run 7 ·
  verify 5, re-measured equal.
- fct-pxt7iprv (security-plan.md:270) and fct-k7t43itf (test ci-integration:61) (m-zizmor) — 2 suppressions, one per
  workflow, as measured at implement entry 22.
- fct-7d3yogoz (test ci-integration:62, m-kill-line) — «over 26 binaries» (nextest.toml:24 re-counted 26).

### True claims sharing the token
- fct-lzdm6e7w (test integration-test-strategy:96, arch1-05) — «read as `viola` or `viola.exe`» describes
  `tests/cli_output_plain.rs:104,116` as it stands (the `.exe` trim is still in the test; see *Code remnants*).
- fct-va43dnf2 (established-decisions:38), fct-lhvxskln (test-scope-summary:145) (m-casefold) — the local-command
  match's «no case folding».
- fct-26k2fw6o (security-plan.md:369), fct-cthr64ip (pii-scrubbing:10), fct-5z3qj22n (test key
  `5-command-implementation`:104) (m-casefold) — the secret scan's case-insensitive `cookie:` class
  (`secret_scan.rs:129`).
- fct-cnkeyzag (data-protection:70, m-casefold) — the scrub: the home matched exactly, the user word folded (`ledger.rs`
  `replace_word`).
- fct-swegdoif (architecture.md:344, m-macos-gap) — «the one named carve-out to the test-data rule» (the chaos home), not
  the macOS carve-out.
- fct-65crhg3m (authn-authz:54) and fct-uv5qlev7 (error-handling:24) (m-verify-codes) — name `human::refuse`, no count.
- fct-7mqjkvmt (test-plan.md:762, m-zizmor) — no count, no workflow named.
- fct-qdldsdqk, fct-iplgfuqy, fct-jaqoqtdh, fct-hhwc4gim, fct-hdfefjry (e2e-test-strategy:39, :266, :279, :326, :340)
  and fct-cz5jn5fs (test-strategy:29) (m-kill-line) — the `ci` profile's 20 s kill line (each binary on
  nextest.toml:24), and the default profile's absence of one («27 minutes», not a binary count).
- fct-5pbk7sp3 (occupied-resources:262), fct-xib5fpty (:289, :290 ×2), fct-y573epy2 (security-anti-patterns:17),
  fct-qmrx4mno (integration-test-strategy:40), fct-msk3nz6p (unit-test-strategy:104, new ×4), fct-p6be5x2e (test key
  `test-data-bootstrap`:12 ×4) (m-msrv-floor) — `Workspace::ensure_e2e_home`, `HEADROOM_FLOOR_MIB` and
  `headroom_refusal` unchanged (the report's msrv fix).

### Quotations and labels
- fct-njspb5nv (test-plan.md:42) and fct-b2j3wr36 (test-scope-summary:64) (m-sideload) — trace citations naming a
  creator-excerpt Must-Work item by its title; the claim they trace (exit on the handle, never EOF) stands.
- fct-s6rhzqu4 (quality-gates:8, m-sideload) — a chunk name inside a dated measurement citation: label.
- fct-pk63c4dw (security-plan.md:504, m-sideload) — the master's subject line (`dep-conpty` among the subjects): label;
  `classify.py check --doc security-plan --mode standing` PASS.

### Owned elsewhere
- fct-irhgihut (architecture key `project-directory-structure`:20 new, :63, :92 ×2, :107, :124: m-sideload, m-win-ci) —
  the fenced tree; the deleted paths re-worded as pointers (Q2, the overseer's technical answer, 2026-11-06 15:20Z;
  `edits-main-6.md`, TX 338).
- fct-yb5jpaot (obs log-coverage:10: obs3-10, obs3-11, m-sideload ×7) — the `process-start` row, left standing (Q3,
  2026-11-06 15:20Z), owned by a CARRY on «Version done-check»: target text `pty_backend` (`openpty`) alone,
  `sideload_fallback` gone.

## Leaf rows (513 over 26 files)
Every leaf the listing names enters step 3. This chunk closes Epoch 7 (the `epoch` view: `markerless 0 · pending 1`),
so step 3 runs `cascade.py leaves --all` (WHOLE mode): every leaf is compared against its sources whole, these rows
included; none is written from a row.

## Curation rows (26 over 4 files) → P3
`.claude/rules/testing.md` :54, :55 ×2, :56 (×2 patterns), :58, :62, :70 (×2 patterns), :85, :107 · `.claude/rules/ci.md`
:30, :31, :33 · `.claude/rules/verification-harness.md` :55, :56, :68 ×2 · `.claude/docs/session-learnings.md` :12,
:135 (×5 patterns), :200, :209, :232, :242. Preserve-verbatim homes: never edited here; P3 weighs each as an in-place
extension of the stale entry or surfaces it.

## Base rows (0)
None.

## The pass's own text
`edits-main-7.md` re-words rows of standing patterns and coins no mechanism; the sweep was not re-fired over it (its
subjects are the listed patterns' own). Read-backs after TX 339–341: `detail.py check --all --growth` VERDICT 0 defects
(GROWTH: 7 flag(s) · 17 warning(s), unchanged); `classify.py check --mode standing` PASS on the obs-plan, security-plan,
test-plan and architecture bodies and the three edited key files.

## Leaves
Mode: WHOLE for every group (`cascade.py leaves --all`; this chunk closes Epoch 7). The set call: `total · whole · 16
group(s) · 27 leaves · 441587 B · unmapped 2`. The batch `leaf`: 16 comparers, `fanout.py check` VERDICT 16 of 16 pass.
The apply dry run: `total · blocks 296 · applied 293 · preserved 1 · reshaped 1 · escaped 1`; each finding's source side
read against the masters (`rec.py find` per finding, a read-only verifier pass under main's ruling); the write: `total ·
blocks 296 · applied 289 · preserved 1 · reshaped 1 · escaped 1 · held 4` (25 leaves written, each read back).
- Groups and findings (applied by the tool unless named below): a11y-plan 9 · architecture-1 41 · architecture-2 50 ·
  architecture-3 39 · architecture-4 4 · architecture+design-system+layout-templates 2 · architecture+security-plan 11 ·
  architecture+test-plan+obs-plan 22 · design-system+layout-templates 2 · masters 18 · obs-plan 9 · security-plan 40 ·
  security-plan+obs-plan 0 · test-plan-1 30 · test-plan-2 13 · test-plan+obs-plan 6.
- Declined, written by hand instead (TX 374–379, each drafted whole and landed by `rec.py harness land`):
  - `leaf-architecture-1.md/.claude/docs/gotchas.md:28#1` — its text («a search viola does not own») no master states; the
    line now records the npm `.cmd` case as having left with the Windows code, recoverable at `multi-os-final`.
  - `leaf-architecture-2.md/.claude/docs/services/viola-channel.md:26#1` — «a Unix socket has no busy state» rests on a code
    comment, no master; the line keeps `connect_by`'s deadline, re-worded «a busy deadline».
  - `leaf-architecture-2.md/.claude/docs/services/viola-pty.md:27#1` — it dropped the psmux swap trigger fct-ad44nn2e still
    holds; the line names the owed move off `=0.8.1` (owner «Version done-check») and keeps the trigger.
  - `leaf-architecture-2.md/.claude/docs/services/viola-pty.md:30#1` — its replacement dropped the bullet's current half;
    the line keeps «the input writer until the child exits», the ConPTY reason gone.
- Not written by the tool, written by hand with the comparer's own text: `viola-agent-claude.md:22#1` (escaped: `[&str;
  3]` reads as an entity), `stack.md:14#1` (reshaped: the row's escaped pipes), `commands.md:26#1` (preserved: a heading).
- Unmapped: `.claude/rules/host-linux.md` — the host leaf, no source states it, no comparer (its one sweep row none);
  `.claude/rules/testing-src.md` — no master names it; its one sweep row (`:20`, m-kill-line) is a dated Session Additions
  entry on the nextest kill line, a true claim, preserve-verbatim: left.

## Residual census (after the leaves; a cascade miss folded into this pass)
- The doc agents' `@remark` facts (`fanout-results.md` §Remarks carried) were ruled at Apply (`fanout-results.md` §Apply
  rulings): six edited, routine and Q4, in `edits-main-8.md` (TX 380); the rest true.
- Then one regex over all seven masters' text, `\b(Windows|macOS|windows|macos)\b` by `rec.py find --regex` per doc
  (116 hits over 61 facts: architecture 43 · security-plan 8 · design-system 8 · layout-templates 1 · test-plan 45 ·
  obs-plan 8 · a11y-plan 3), plus every `per-OS` hit (`rec.py find --text 'per-OS'` per doc) and the Windows relink
  reason. Each hit read at its fact (a read-only reader under main's ruling): 104 kept — retirement and recoverability
  statements, dated readings on a retired leg or host, the typography table's Windows / macOS columns, «per-OS-user»
  (a different subject), and two facts whose Windows code still stands in the tree (fct-k6htnrfj `plain_drive`;
  fct-hgewdmj2 @18033, `host.rs`'s `windows` key) — and 19 stale sites in 17 facts. 16 facts amended in
  `edits-main-9.md` (TX 381): the «per-OS» job wording (`perf`, `lint`, `release`, the coverage floors), present-tense
  Windows / macOS reasons dated as from a past target (fct-ak4xjyyx, fct-zyth7okk, fct-zxoe3ae7, fct-zxaumenb), the
  Windows Defender example (fct-lqftgban), the orphans-check control (fct-eesxehzs, one `cfg(target_os = "linux")`
  file), the counted-tests gate words (fct-zujg4opw); fct-ahjpxago (the root bin's windows-sys dev-dependency, its
  whole fact) retired (TX 382). fct-yb5jpaot's two pairs held: Q3 left that row standing, its CARRY owning the text.
- Leaves restating the retired wording, written by hand (TX 383–390): `obs-summary.md:31`, `tests-summary.md:44`,
  `stack.md:31`, `verification-harness.md:28`, `commands.md:75`, `rules/security.md:11`, `rules/ci.md:13`,
  `services/viola-state.md:38`. Curation homes holding it (`rules/testing.md:56`, `:70`; `docs/session-learnings.md`
  :50–55, :175) → P3. `gotchas.md:32`'s heading (append + exclusive lock fails on Windows) is a true fact about Windows,
  kept. Read-backs after TX 380–382: `detail.py check --all --growth` VERDICT 0 defects (GROWTH unchanged);
  `classify.py check --mode standing` PASS on the architecture, security-plan, test-plan and obs-plan bodies and the
  edited key files.

## Code remnants seen (not master drift; for P5)
Read while ruling, outside the masters: `tests/cli_output_plain.rs:104,116` (the `.exe` trim and its Windows comment),
`crates/viola-agent-claude/src/lib.rs:2` (an «npm shim» doc comment), `crates/viola-e2e/src/harness/session.rs:52` (a
«`.exe` on Windows» comment), the `EXE_SUFFIX` joins in `crates/viola-e2e/src/harness/` (`mod.rs:190,361`,
`boot.rs:877`, `run/perf.rs:109,390`; empty on Linux). The wrap touches no source: P5 carries them.
