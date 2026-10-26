# Cascade dispositions — 2026-10-25-windows-endpoint-admission · wrap

The step-2 sweep, ONE call: `cascade.py sweep --patterns-file apply-architecture.md --patterns-file apply-security-plan.md
--patterns-file apply-test-plan.md --patterns-file cascade-patterns.toml`, baseline `b6740d59` (the parent of the oldest
pre-CI commit `e2bb59c8`), over the seven masters' bodies and category sets, every `.andromeda/registries/**` file, the
three curation homes, the two judgment bases and the leaf bodies.

`total (6 patterns) · 36 rows over 15 files` · `per class · new 14/7 · standing 18/9 · leaf 4/3 · curation 0/0 · base 0/0`.

## The search
- The writers' patterns: `tp-01` (`remote pipe clients, the Windows pipe DACL) cannot be driven on hosted runners`,
  the retired test-plan.md:46 wording) and `tp-02` (`rejecting remote pipe clients, and denying access through the
  Windows pipe DACL`, the retired test-scope-summary.md:11 wording); architecture's and security-plan's writers wrote
  `(none)`. Both read 0 rows with their control fired: the retired wording stands nowhere.
- Main's own (`cascade-patterns.toml`), for the retired claim's verbs and its mechanism, not only its name:
  `main-hosted` (`cannot be (driven|exercised|tested|asserted) on hosted`, `hosted (GitHub )?runners?,? which have no
  second`, `no second OS account`), `main-remote` (`remote (pipe )?clients?`, `PIPE_REJECT_REMOTE_CLIENTS`,
  `accept_remote`), `main-dacl` (`pipe DACL`, `DACL'?s effect`, `partially testable with stubs`, `anonymous`),
  `main-tokio-adv` (`RUSTSEC-20NN-NNNN`, any advisory id). Every control fired.
- No pattern was written for main's own subject-line write (`architecture.md:459`, a label naming a subject, claiming
  nothing).

## Rows
main-hosted (new 1 · standing 1)
- `.andromeda/test-plan.md:46` · standing, edited — re-derived: the pass's own broad line; «cannot be driven on hosted runners» now holds only for a peer of another euid, true.
- `.andromeda/test-plan-detail/test-scope-summary.md:11` ×2 · new — re-derived: the pass's text, the other-euid peer and the absent second OS account, true.

main-remote (new 6 · standing 3 · leaf 3)
- `.andromeda/security-plan.md:108` · standing — no change: «remote clients stay rejected» is the body's broad line, now witnessed in its detail group, true.
- `.andromeda/security-plan-detail/authentication-authorization.md:15` ×2 · standing — no change: «Keep `accept_remote(false)` (the default, `PIPE_REJECT_REMOTE_CLIENTS`)», the implementation, true; its witness is the new group below it.
- `.andromeda/security-plan-detail/authentication-authorization.md:49` ×2 · new — re-derived: the pass's remote-client witness.
- `.andromeda/test-plan.md:46` · new — re-derived: the pass's broad line.
- `.andromeda/test-plan.md:606` · new — re-derived: the pass's §6 broad line.
- `.andromeda/test-plan-detail/e2e-test-strategy.md:295` · new — re-derived: the pass's remote-client case.
- `.andromeda/test-plan-detail/e2e-test-strategy.md:296` ×3 · new — re-derived: the pass's two controls.
- `.andromeda/test-plan-detail/integration-test-strategy.md:18` · new — re-derived: the pass's pointer to §6.
- `.andromeda/test-plan-detail/test-scope-summary.md:11` · standing, edited — re-derived: the pass's text.
- `.claude/rules/security.md:10` · leaf — added to step 3's set (`accept_remote(false)` restated; the comparer reads it against the amended sources).
- `.claude/docs/security-summary.md:14` · leaf — added to step 3's set.
- `.claude/docs/services/viola-channel.md:22` · leaf — added to step 3's set.

main-dacl (new 6 · standing 8)
- `.andromeda/architecture-detail/stack-and-technologies.md:15` · standing — no change: the Wrapper IPC row's «the server's pipe DACL is converted from …», the descriptor's mechanism, true.
- `.andromeda/security-plan.md:434` · standing — no change: «The default DACL grants read to Everyone and anonymous», measured true by `security_negative_control_default_pipe_admits_an_anonymous_read` on windows-2025.
- `.andromeda/security-plan-detail/authentication-authorization.md:25` · standing — no change: the second interim gap's residual names the pipe DACL and the 0700 socket dir as what bounds a squatter to the same user, true.
- `.andromeda/security-plan-detail/authentication-authorization.md:51` · new — re-derived: the pass's group broad line.
- `.andromeda/security-plan-detail/authentication-authorization.md:52` ×4 · new — re-derived: the pass's DACL-effect witness.
- `.andromeda/security-plan-detail/threat-model-summary.md:48` · standing — no change: per-OS enforcement (pipe DACL, socket file mode) handed to security, true.
- `.andromeda/test-plan.md:46` ×2 · new — re-derived: the pass's broad line.
- `.andromeda/test-plan.md:51` · standing — no change: its «partially testable with stubs for the Windows DACL cases» is the HOME strict-modes cases (a foreign-owner SID, a volume without persistent ACLs) the security excerpt classifies, another control from the pipe DACL, true.
- `.andromeda/test-plan.md:421` · standing — no change: the Module ↔ IPC row's DACL read-back in canonical form, true.
- `.andromeda/test-plan.md:606` · new — re-derived: the pass's §6 broad line.
- `.andromeda/test-plan-detail/e2e-test-strategy.md:297` ×3 · new — re-derived: the pass's DACL-denial case and control.
- `.andromeda/test-plan-detail/integration-test-strategy.md:18` ×2 · standing, edited, mixed — no change to the standing clause (the DACL's canonical read-back); the pass's appended pointer, re-derived.
- `.andromeda/test-plan-detail/mocking-stubbing-discipline.md:19` · new — re-derived: the pass's `ImpersonateAnonymousToken` stand-in.
- `.andromeda/test-plan-detail/test-scope-summary.md:11` ×3 · standing, edited, mixed — re-derived the pass's text; no change to its last sentence, «The security excerpt (data classifications) notes the Windows DACL cases are only "partially testable with stubs"», a quotation of the excerpt whose stub cases are the home strict-modes ones (test-plan.md:51), true as a quotation.

main-tokio-adv (new 1 · standing 6 · leaf 1)
- `.andromeda/security-plan.md:250` · standing — no change: the `[advisories]` ignore list holds only RUSTSEC-2017-0008, true (no ignore widened).
- `.andromeda/security-plan.md:262` ×2 · standing — no change: another advisory (RUSTSEC-2026-0007, bytes), unrelated.
- `.andromeda/security-plan.md:481` · standing — no change: another advisory (RUSTSEC-2025-0055), unrelated.
- `.andromeda/security-plan-detail/dependency-security.md:58` ×3 · new — re-derived: the pass's supply-chain line naming its owner.
- `.andromeda/test-plan.md:226` · standing — no change: «only ignore RUSTSEC-2017-0008», true.
- `.andromeda/obs-plan-detail/obs-scope-summary.md:243` · standing — no change: another advisory, unrelated.
- `.andromeda/registries/contracts/obs-plan/log-file-location.md:10` · standing — no change: another advisory (RUSTSEC-2026-0009), unrelated.
- `.claude/rules/security.md:31` · leaf — added to step 3's set (the ignore line, true; the comparer reads it).

## Leaves
`cascade.py leaves` (pass mode, the step-2 leaf rows added: `.claude/rules/security.md`, `.claude/docs/security-summary.md`,
`.claude/docs/services/viola-channel.md`): `total · pass · 9 group(s) · 23 leaves · 325153 B · unmapped 2`. The leaf
batch: `leaf: 9 agents · 9 pass`. The apply, last call: `total · blocks 6 · applied 2 · already applied 1 · held 3`
(the first `--only` call wrote `testing.md:17#1`; `--only` takes several ids after one switch).

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| architecture-1 | pass | 0 | 0 | 0 | 0 |
| architecture-2 | pass | 1 | 0 | 0 | 1 — `services/viola.md:17#1`: the root's dev-dependency features are a detail its master holds (the key file Crate dependency direction); no reader of the service leaf acts against it |
| architecture-3 | pass | 1 | 0 | 0 | 1 — `stack.md:47#1`: the same detail, stated once in the key file |
| architecture+security-plan | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan+test-plan (CLAUDE.md) | pass | 0 | 0 | 0 | 0 |
| architecture+test-plan | pass | 1 | 1 — `commands.md:39#1` (a reader would run plain `cargo test` as a supported runner) | 0 | 0 |
| security-plan | pass | 1 | 0 | 0 | 1 — `rules/security.md:31#1`: the rule already bans widening an ignore to reach green, so no reader acts against the source; the red is the route entry's to remove, its status stated once at Dependency Security |
| test-plan-1 | pass | 2 | 2 — `tests-summary.md:15#1` (the runner), `rules/testing.md:17#1` (a new scoped capture would take a bare `with_default`) | 0 | 0 |
| test-plan-2 | pass | 0 | 0 | 0 | 0 |

Unmapped: `.claude/rules/host-linux.md` (the host's own statements, no master source) and `.claude/rules/testing-src.md`
(pointers into `testing.md`, no master source) — no comparer is the leaf's.

## Not looked for
The scoped-capture helper, the windows-sys dev-dependency features and the plain-`cargo test` runner are new facts
with 0 master sites before this pass (the P1 site sweep's unfired controls), so no retired wording of theirs exists
to sweep; their new text is this pass's own.
