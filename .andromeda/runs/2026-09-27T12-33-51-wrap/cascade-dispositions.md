# Cascade dispositions — 2026-09-27-wrapper-channel

**Search:** `cascade.py sweep` over `cascade-patterns.toml` (18 patterns, one per retired claim or mechanism of this
pass: `mode(0o600)`/`ListenerOptionsExt::mode`, `SecurityDescriptor::deserialize`, the `(A;;GA;;;` read-back literal,
the `$TMPDIR/viola-<h12>` path, "Wrapper channel" pending-bind wording, `endpoint_gone … null`, "next read returns 0",
the fd-count premise, the viola-pty exact dependency set, the one-fuzz-target wording, member lists without
`viola-channel`, tracing `std`-only, `corr` optional keys, the pre-push stage list, amendment 2 pending, the
`interprocess` connect check, the boot missing codes, `channel-*` corr always the id). Baseline `17c99c9` (the pre-CI
commit's parent); every control fired. Full listing: `cascade-sweep.txt`. Plus a hand grep of the leaves of the four
changed masters for `viola-channel|endpoint|squatted|sync crate|fuzz|pre-push|SDDL|veil|attributes|corr|srv_conn|E2|fd`.

## Master rows
| row | disposition |
|---|---|
| security :189, :498 (mode0600, sddeser) | new — this pass's text naming the mechanism as not used / banned |
| security :203 (mode0600, edited) | amended — the retired mechanism is named only as "not used" |
| security :207@c130, obs :583@c1475 | true — `OpenOptionsExt::mode(0o600)` on home files, a different API; no change |
| security :594 | history — Decisions Log record of the IPC decision as made; the mechanism is current in §Auth (IPC access control (Unix)); no change (prior-wrap precedent: Decisions Log history unchanged) |
| security :202 (ga-literal, new) | true — the SDDL string that is set; the read-back form sits beside it |
| test :46, :926 (ga-literal, edited) | amended — set literal and canonical read-back both stated |
| security :57 | rejected for the section — Threat Model Summary is a verbatim copy of threat-assessment.md (playbook "Verbatim upstream copy"); current truth sits in §Auth and arch IPC endpoints |
| security :497 | true — the ban names the old root path as forbidden |
| security :598 | history — Decisions Log arch amendment 2 as proposed ("moves from … to …"); now folded (bootstrap :371) |
| test :1709 | true — anti-pattern contrasting the old path |
| security :376 (bind-pending, new) | new — "landed with Wrapper channel" |
| test :599 (endpt-gone-null, new) | new — `port_free` / `url_file_removed` stay null |
| test :1203 (fd-premise, edited) | amended — the retired count quoted as not holding, with the measurement |
| arch :96, :389; test :434 (edited) | amended |
| obs :698 (edited) | amended — optional except the per-event required rules |
| obs :1659 | history — D-26 Decisions Log; no change |
| test :671 (new) | new — the enum with `vm-release`, `windows-tests` |
| obs :261, a11y :221 (chan-corr-id) | rejected for the section — obs §1 and a11y §1 are verbatim scope copies (playbook "Verbatim scope copy" / "Verbatim upstream copy"); §3 wins |
| next-read-zero, member-list, amend2-pending, ipc-notfound, boot-missing | 0 rows after the apply; each control fired on the baseline |

## Leaf rows (re-derived)
| leaf | disposition |
|---|---|
| `.claude/rules/security.md:13` | re-derived — chmod 0600 after the bind; `ListenerOptionsExt::mode` banned |
| `.claude/rules/security.md:12` | true — the SDDL set; no change |
| `.claude/docs/security-summary.md:14` | re-derived — chmod after the bind |
| `.claude/docs/services/viola-channel.md:11, :22, :28` | re-derived — deps as landed, SDDL conversion + canonical read-back, chmod, `.lock` arbiter, frame rules, `srv_conn`; "another environment" |
| `.claude/docs/services/viola-state.md:20` | true — `OpenOptionsExt::mode(0o600)` on home files |
| `.claude/rules/verification-harness.md:29` | re-derived — `endpoint` required, `<name>:endpoint` |
| `.claude/docs/services/viola.md:20` | re-derived — bind landed, `squatted-name` |
| `.claude/docs/services/viola-pty.md:11` | re-derived — tracing |
| `.claude/docs/stack.md:15, :21` | re-derived — Wrapper IPC server side; tracing `attributes`, veil |
| `.claude/docs/commands.md:33` | re-derived — `vm-release`, `windows-tests`, 16 jobs |
| CLAUDE.md `GENERATED:setup:*` | recomputed — modules (`viola-channel` line), architecture, warnings, pointer table: still true, no change |
| `.claude/docs/{conventions,gotchas,tests-summary,obs-summary}.md` | recomputed from the hand grep — no stale claim (obs-summary :67 already lists `squatted-name`; gotchas :49 per-user dir) |

## Binds
- tests §3 ↔ obs §3 Log format: the same `corr` rules and the same required-set bullet written into both.
- a11y ↔ obs schema: the a11y row schema never reads diag-line; a11y :220-225 is the §1 verbatim copy — no change.

## Curation homes / judgment bases
- 0 rows in `USER:session-learnings`, `## Session Additions`, `docs/session-learnings.md`, playbook, drift-base.

## Verification read (orchestrator, not a detector)
- `crates/viola-e2e/src/harness/pre_push.rs:209-215, :877-879` — the document keys `linux`, `vm`, `windows{run,gate}`
  in that order, read to write test-plan §3's shape exactly (the report names the keys, not the inner shape).
