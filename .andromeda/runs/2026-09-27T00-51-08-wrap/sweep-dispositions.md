# Cascade sweep dispositions — wrap of 2026-09-26-local-linux-pre-push-gate

Tool: `cascade.py sweep` (cascade v1.0 · 798c29e2), baseline a69c5efb (the pre-CI commit's parent), patterns
`cascade-patterns.toml` (23 run). Four patterns were refused by the tool (control never fired over the seven masters
at a69c5efb) and controlled by hand over the leaves, curation homes and bases (`grep -rn -E` over CLAUDE.md,
`.claude/rules`, `.claude/docs`, playbook, drift-base):
- `host_size` — 0 hits anywhere (the retired second host read is named in no doc).
- `\b(five|5) (internal|harness)` — 0 hits (no doc states the internal-subcommand count).
- `schema-check.{0,40}secret-scan` — 2 leaf hits: `rules/verification-harness.md:23` (re-derived: `pre-push` added),
  `docs/commands.md:32` (re-derived: `pre-push` added to the internal list).
- `\b399\b` — 0 hits (the ubuntu coverage test count is stated nowhere).

Every pattern: curation 0 · base 0 (no curation-home or judgment-base row).

| pattern | rows | disposition |
|---|---|---|
| base-head-itself | test-plan:553 | amended (the ×2 are the rule's two branches, both new text) |
| base-head-caret / base-wrap-push / base-last-flip | arch:364, arch:529; test-plan:1878; leaves verification-harness:43, commands:41, workflow:10 | arch:364 (registry) and arch:529 (CI job) describe the base CI derives — true, CI never checks out an uncommitted promotion: no change. test-plan:1878 is a dated §12 entry (history): no change. verification-harness:43 and commands:41 re-derived (the uncommitted-promotion clause added). workflow:10 (CI) true, no change; a new "Before the push" bullet added |
| resize-propag | test-plan:926 | amended |
| resize-any | arch:46/69, test-plan:40/125/1366, obs:61, a11y:100/958/1210; leaves CLAUDE.md:22, services/viola-pty.md:6/34, services/viola.md:21 | all state PTY resize capability or pass-through (true) or a11y SC 1.4.4 (unrelated token): no change. New sites arch:368, security-plan:232/687, test-plan:1890-1898 are this pass's text. `docs/gotchas.md` re-derived (new "A resize before the pump's first look was lost") |
| version-source | test-plan:1452 | amended (WSL provisioning sentence) |
| rustup-install / rustup-tc-inst | arch:13/36/528, security-plan:131/324/341, test-plan:794/1447/1448/1452/1457/1812; leaves commands.md:8, stack.md:9/32 | security-plan:324 amended ("the host and CI both" → host, CI and the WSL2 distro). All others state a CI/host install step that still holds: no change. stack.md:32 re-mirrored verbatim from arch:36 |
| cargo-locked | arch:37/528, security-plan:332, test-plan:755/791/794/1448/1452; leaves commands.md:9-10, stack.md:33 | true install statements: no change; new sites arch:36/494, security-plan:326 are this pass's text; commands.md gained the two WSL provisioning lines |
| internal-subcmd | test-plan:611/805/1307/1751/1755; leaf verification-harness:23 | test-plan:611 amended (the `pre-push` entry sits under it); the rest are cross-references: no change. verification-harness:23 re-derived |
| closed-enums | test-plan:665/1804/1820/1872 | 665 amended (the `pre-push` bullet); the rest are §12 impact cross-refs: no change |
| viola-prefix | arch:553, security-plan:580, test-plan:962, obs:1405; leaves CLAUDE.md:38, security-summary:39 | arch:553 and security-plan:580 amended (carve-out). test-plan:962 (the config table covers every `VIOLA_*` var) and obs:1405 (no `VIOLA_*` var may widen fields) stay true — the seam is not `VIOLA_*` and widens nothing. CLAUDE.md:38 recomputed: still true, no change. security-summary:39 re-derived |
| every-var-prod / never-read-viola | arch:171, arch:363; leaf conventions.md:13 | arch:171 amended; arch:363 (the harness-only list) true; conventions.md:13 re-derived |
| not-config-chan / never-reads-env | obs:39/249/604/606/1139/1279/1385/1415/1565; leaf CLAUDE.md:38 | expected amendment #17: each read — every one states that env is not read AS CONFIGURATION (a level, a service name, a backtrace switch, CI metadata); none is exhaustive over what a `viola` build reads, and the seam configures nothing: no change. arch:553 and security-plan:422/580 amended |
| agent-run-prefix | 15 master rows, 7 leaf rows | registry, override and keep-homes statements: all true, no change |
| mutation-gate / mutants-legs | test-plan:509/644-662/1444/1507/1879 + §12 cross-refs, arch:529, obs:1254; leaves testing.md:48, tests-summary.md:42, verification-harness:43, commands.md:31 | test-plan:1507 and obs:1254 amended; the §3 gate spec and CI job rows stay true; testing.md:48 and tests-summary.md:42 re-derived (the local union); verification-harness:43 and commands.md:31 true (+ `pre-push`) |
| operator-pass | arch:529, test-plan:553/1445/1876; leaf workflow.md:10 | true (CI reads the whole chunk per push): no change; workflow.md re-derived |
| target-registry | arch:395; leaf commands.md:71 | true; arch:396 `target/pre-push/` is this pass's text |
| install-ripgrep | arch:37/401/492/528, test-plan:793/1452; leaves commands.md:11/65, stack.md:33/48 | arch:492 amended (tree); the rest true: no change |

Leaves re-derived (step 3): `docs/commands.md`, `docs/stack.md` (CI/CD row), `docs/conventions.md`, `docs/gotchas.md`,
`docs/workflow.md`, `docs/tests-summary.md`, `docs/security-summary.md`, `docs/obs-summary.md`,
`rules/verification-harness.md`, `rules/testing.md`, `rules/security.md` (all above `## Session Additions`).
CLAUDE.md `GENERATED:setup:*` recomputed from arch §Stack, §Established Decisions, §Conventions, §Cross-cutting, the
module map and the pointer targets: no block states a changed fact (the warnings env line stays true) — no change.
`rules/observability.md`, `services/viola-pty.md`, `services/viola.md`: read, no amended claim stated — no change.
Binds: test-plan §3 ↔ obs-plan §3 — obs §3 lists no internal harness subcommand, so `pre-push` needs no obs §3 line;
a11y ↔ obs schema untouched.
