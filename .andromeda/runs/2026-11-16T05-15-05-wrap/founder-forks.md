# Forks — 2026-11-16-send-confirmation, wrap P2 (Escalate)

The operator is away (the wrap directive `inputs#I4`, item 1): no dialog is raised. Q1 is the founder's: a boundary
widening under the playbook's escalate rules `rul-r74hshif` and `rul-p2zl2kgl` (a validated surface — the send's
delivery confirmation — admits a new input class: a submitted text equal to the typed text less a trailing whitespace
run). Fit has judged first; no operation listed reads `not_master`. The delegate's answer, by relay, lands
PROVISIONAL; nothing is ratified; the founder's own word stays owed.

## Q1 (founder's) — the masters' rows for the send's trailing-whitespace tolerance

The code at `76257d01` confirms a send when the submitted prompt equals the typed text, or is non-empty and equals it
less its trailing run of ASCII space, TAB, CR and LF (`viola_agent_claude::hook::is_submitted_form`, called by
`Flight::claim`; report Changes → Symbols / APIs). Today four masters say the match is exact. The operations below
bring each row to the built rule, the tolerance marked provisional where the sheet takes a `state`:

- `ops-architecture.md · contract/hook-event/driver-relabel` · fit: fits type 3
- `ops-architecture.md · contract/channel-methods/hook-event-driver-origin` · fit: fits type 3
- `ops-architecture.md · decision/exact-prompt-match` (retire, heir decision/submitted-form-prompt-match) · fit: —
- `ops-architecture.md · decision/submitted-form-prompt-match` (add, provisional) · fit: fits type 1
- `ops-design-system.md · contract/send-readback/prompt` · fit: fits type 3
- `ops-layout-templates.md · contract/viola-send-output/read-back` · fit: fits type 3
- `ops-obs-plan.md · contract/confirm-window-close/ordinary-send` · fit: fits type 3
- `ops-security-plan.md · decision/typed-text-line-ending-normalisation` (why) · fit: fits type 1
- `ops-security-plan.md · decision/delivery-confirmation-exact-match` (retire, heir decision/delivery-confirmation-trailing-tolerance) · fit: —
- `ops-security-plan.md · decision/delivery-confirmation-trailing-tolerance` (add, provisional) · fit: fits type 1
- `ops-test-plan.md · decision/send-confirmation-exact-match` (retire, heir decision/send-confirmation-trailing-trim-match) · fit: —
- `ops-test-plan.md · decision/send-confirmation-trailing-trim-match` (add, provisional) · fit: fits type 1
- `ops-test-plan.md · contract/viola-send/records` · fit: fits type 3

13 lines.

1. **(Recommended) Apply as proposed — trailing whitespace only, PROVISIONAL.** The rows state the rule the code
   holds; the three decision rows carry `state: provisional`. Why: the masters must match the built code (drift = 0),
   and the narrowest widening is the one the delegate already chose at P4 (`inputs#I2`).
2. **Apply «either end».** The rows would state a tolerance at both ends. Trade-off: the code holds trailing only, so
   the rows would be false until a chunk changes `is_submitted_form`.
3. **Reject — keep the exact match in the masters.** Trade-off: four masters then contradict the shipped code; the
   chunk's v2-18 claim rests on the tolerance.

## Answer

Taken by relay, per the wrap directive (`inputs#I4`, item 1: «where the letter halts for his word take the delegate's
answer by relay — PROVISIONAL, the rows marked so, nothing ratified»). The delegate's answer to this widening, given at
P4 and relayed by the operator (`inputs#I2`), verbatim: «Q1 = trailing whitespace only. PROVISIONAL. It is the smallest
widening that closes the MEASURED instance (a send ending in a space, the CLI submitting it trimmed, 2026-10-19 on
2.1.296); what the CLI does to LEADING whitespace is not measured on any version, and a tolerance for it is the
founder's to give.»

Resolution: option 1, apply as proposed. Authority: `delegate overseer, 2026-11-16 — relayed by the operator,
inputs#I2, inputs#I4` — PROVISIONAL; the founder's own word owed.

## For the founder's return

- **Candidate: «either end».** The claim takes `S` when `T` is `S` with whitespace only before and after it. The
  overseer leans to it once a live reading shows what the CLI trims at submit; the rule lives in one function, the
  change one line, additive.
- **Owner of that live reading:** «Standing live rig with a screen model»: read live, on the interactive input box,
  exactly which whitespace the CLI trims at submit (leading, trailing, inside a paste).
