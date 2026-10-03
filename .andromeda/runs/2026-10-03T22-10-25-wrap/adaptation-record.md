# Adaptation record — 0-pending wrap 2026-10-03T22-10-25 (session 33)

Source: the Viola overseer's relay `linux-route-adaptation.md` (2026-10-04; founder-delegated; item C2 the
founder's own ruling). Dispositions taken in this wrap's route-resolve dialogue, the overseer answering for the
founder. The dev host moved from Windows to Linux (Omarchy, btrfs) on 2026-10-03; the operator lifted the Viola pause.

## Measured at this wrap (before any edit)
- CI `ci` green on `d60f3d6` (run 37120646288) and `9e3b850` (run 37107107417), read with `gh run list`.
- `target/e2e-home/` exists and is empty.
- Host git config: `diff.mnemonicprefix=true`; `crates/viola-e2e/src/harness/run/mutants/base.rs` `diff_paths`
  matches only the literal `diff --git a/` prefix (C3's mechanism; the 25/13 vs 38/0 witness is the relay's).
- The host-reds CARRY that item A asks to retire is already absent: the only `host-reds` mention on the route was
  the M2 CARRY's own retirement sentence (premise out of date, nothing to retire).
- The relay's `:76` "First live test" reference is stale: that entry is `:80` (`:76` is Dialog answers).
- The sweep (markerless tail) found two more Windows-host assumptions: `:118` Linux live confirmation ("the Linux
  laptop") and `:131` Web test toolchain's pre-push `windows-tests` stage item.

## Dispositions (working-route.md, markerless tail only)
| Item | Disposition | Where |
|---|---|---|
| A — close M2 | M2 CARRY removed (the D: volume is gone; witness set 63/0 on Linux; CI green). Leak B closed (`e2e-home` empty after every Linux run). In-repo coverage gate closed (96.95 / 97.21 / 96.97 vs 85/95/80). The in-repo TUI boundary cases KEPT as a CARRY: the Linux 5/0 covers openpty only, the ConPTY cases stay CI-witnessed (overseer). The two-sided-control rule for local reds retires with M2 | `:68` |
| B — local-Windows claims | `:85` tab-close leg: founder-attended in a Linux terminal; Windows only through CI's wrapper-termination test; a Windows tab-close recorded not measurable. `:80` annotated: no interactive Windows host; where the live proof runs is the founder's live call at its phase (a credential on a runner = widening; a Linux live run precedes the Unix hardening). `:118` named in that annotation. Pre-push `windows-tests` / WSL stage → item D | `:80`, `:85` |
| B — `host-win32.md` | Not route freight: setup-rendered (registry U04). Routed to an `/andromeda-setup-project` re-run through the handoff | handoff |
| C1 | The 12 `cfg(unix)` mutants run natively; the WSL leg and its `TMPDIR` carve-out question retired; the 13th stays not measurable (no Windows non-x86_64 host or runner) | `:68` |
| C2 (founder ruling) | Own entry right after `:68`: "Windows boundary mutation workflow" (overseer: keeps `:68` within one window; the Epoch 3 boundary audit is its first consumer). Its Decisions Log / test-plan text lands with that entry's own wrap | new `:70` |
| C3 | Folded into `:68` (its gates hit it first) | `:68` |
| D | Folded into `:68`: the native Linux stage keeps the `env -i` HOME+PATH boundary; anything wider halts for the founder live. The WSL-provision root CARRY retires with the WSL tooling (kept if any WSL path stays). `:131`'s `windows-tests` mirror item re-judged with it | `:68` |

`:68` title re-scoped to "Mutation scoring completion — viola-e2e harness and twelve cfg(unix) mutants scored
natively at the boundary tier; pre-push and gate tools moved to the Linux host" (the chunk title, up to ` — `,
unchanged). Anchors held: `:68` stays the next entry; epoch headers unchanged; the 0.2.0 incubator untouched.

## Boundary widenings
None applied. Three are named on the route for the founder live: dropping `env -i` from a native pre-push stage
(`:68`), a Claude credential on a CI runner and a Linux live run before `Unix endpoint and home hardening` (`:80`).

## Not done here
- No spec body edited: the bodies naming the WSL distro or pre-push stages (architecture, security-plan,
  test-plan, obs-plan, `.claude/docs/commands.md`, rules `testing.md` and `verification-harness.md`) reconcile at
  `:68`'s wrap, by its D CARRY.
- P3 curation: no corrections in this conversation; skipped.
- Epoch 3 holds 9 entries after the insert (under the ~10 growth valve).
