# Fan-out results — 2026-10-09-epoch-3-cleanup

Seven doc-agents, one batch; 15 detectors over the seven prompts (arch 2 · security-plan 3 · design-system 1 ·
layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2), the sum of the drift-base's `doc:` names. Every return
parsed; no entity was found in any return, and no return needed a raw twin. Each proposal is recorded by its
detector, its section, its own `sidecar` line and its basis; its `change` text was re-derived at Apply from the
report's fact, never pasted. 45 proposals; 0 rejected for a source the report does not carry.

Dispositions use the checks of amendment-flow.md §Validate: 1 playbook · 2 cross-contradiction · 3
intent-consistency · 4 absence needs evidence · 5 expected amendments · 6 disproved claims.

## architecture — 16 proposals
| # | detector | section | sidecar line (the agent's) | dependent-of | disposition |
|---|---|---|---|---|---|
| 1 | D-arch-decisions | §Established Decisions → [Delivery Confirmation] | `typed_text` strips every trailing CR and LF (was trailing LF only); two root-bin callers | — | apply · check 1 "Accurate this-chunk addition", check 5 (the plan's entry names it) |
| 2 | D-arch-decisions | same | the matching sentence: typed text without trailing CR and LF | 1 | apply · check 1 |
| 3 | D-arch-decisions | same | the ruling sentence widened from trailing LF to trailing CR and LF | 1 | apply · check 1 |
| 4 | D-arch-decisions | same | a send ending in CR, LF or CRLF is confirmed like any other | 1 | apply · check 1 |
| 5 | D-arch-decisions | same | an empty typed text is refused `not-delivered` / `empty-text` on both sides | 1 | apply · checks 1, 6 (the first disproved claim) |
| 6 | D-arch-decisions | same | the newline-only live reading of 2026-10-08 kept as the earlier build's record; `after-empty-text` added | 1 | apply · check 6 |
| 7 | D-arch-decisions | same | the trailing-CR case fixed and measured; the open residual is the inner CR / CR LF case | 1 | apply · check 6 (the second disproved claim). Applied without the relayed ruling's content: the body states the measured inner readings and that the inner case is carried on the working route (the ruling reached this wrap by relay only, `inputs#I9`) |
| 8 | D-arch-decisions | same | a listed local command followed by CR or CRLF is that command | 1 | apply · check 1 |
| 9 | D-arch-decisions | §Established Decisions → [Human Takeover / Wheel] | in-flight send typed without trailing CR and LF | 1 | apply · check 1 |
| 10 | D-arch-decisions | §Standard Contracts → Channel methods → `hook.event` | typed text is the sent text without trailing CR and LF | 1 | apply · check 1 |
| 11 | D-arch-decisions | §Standard Contracts → Event `data` per kind → `prompt-submitted` | the same | 1 | apply · check 1 |
| 12 | D-arch-decisions | same | "as typed, not as sent" covers a CR / CRLF ending | 1 | apply · check 1 |
| 13 | D-arch-resources | §Conventions → Error handling schema | the `not-delivered` detail set gains `empty-text` (5 → 6) | — | apply · check 1 |
| 14 | D-arch-resources | same | the `send` refusal order gains `empty-text` second | 13 | apply · check 1 |
| 15 | D-arch-resources | §Infrastructure Patterns → Project directory structure (key file) | the self-healing route entry cited by title, the stale `:93` dropped | — | apply · check 5 (the plan's citation item; the operator's direction at this wrap, `inputs#I9` item two). The title is the one `route.py markerless` prints for the entry |
| 16 | D-arch-resources | §Established Decisions → [Database / State Store] | the same entry cited by title, the bare `working-route.md:109` dropped | 15 | apply · check 1, on the operator's recorded direction ("every site found", `inputs#I3`, `inputs#I9`): the number is not stale at this read, and this wrap's own P5 insertion above the entry moves it |

## security-plan — 8 proposals
The seven D-security-input proposals carry the detector's own severity `escalate`. None is a boundary widening:
the detector found the boundary validated on both sides, and each proposal brings the row's description to the
shipped control. Both changes were ruled by the founder live with the options shown (the strip:
`.andromeda/runs/2026-10-09T14-44-30-wrap/adaptation-record.md`; the `empty-text` name, condition and hint:
`inputs#I3`) before the chunk built them, and the plan's P5-approved list names each change. Resolved on those
recorded words, no halt (check 1's recorded-direction branch).

| # | detector | section | sidecar line (the agent's) | dependent-of | disposition |
|---|---|---|---|---|---|
| 1 | D-security-input | §Input Validation → Paste text row, the trailing-LF item | the typed text is the text without its trailing CR and LF characters; `/clear` plus CR or CRLF is `/clear` | — | apply · check 1 (recorded direction), check 5 |
| 2 | D-security-input | same row, `Where it runs` and `Wire outcome` | the `not-delivered` / `empty-text` refusal of an empty typed text on both sides | — | apply · the same |
| 3 | D-security-input | §Threat Model Summary → CLI input vector → Entry point | typed text without trailing CR and LF; an empty typed text refused | 1 | apply · playbook "Verbatim upstream copy kept current" |
| 4 | D-security-input | same vector → Trust boundary | the compared typed text is without trailing CR and LF | 1 | apply · the same |
| 5 | D-security-input | §Security Anti-Patterns → Input, the ban's sentence | the allowed-character removal is trailing CR and LF | 1 | apply · check 1 |
| 6 | D-security-input | same ban, `NEVER refuse LF, CR or TAB` | `empty-text` worded so the sentence stands | 1 | apply · check 5 (the plan's own wording) |
| 7 | D-security-input | §Input Validation → CLI arguments / stdin row | the client's empty-typed-text refusal follows `validate_paste_text` | 1 | apply · check 1 |
| 8 | D-security-auth | §Authentication & Authorization → IPC client-side server verification row | `:125` / `:127` (four times) replaced by the two Epoch 6 entry titles | — | apply · check 5 (the citation item) |

## design-system — 0 proposals
`proposals: []`. The return's commentary, stripped: no `hardcoded✗` flag in the report; the plan's `empty-text`
entries for this document are outside D-design-tokens. Raised by the orchestrator (below).

## layout-templates — 2 proposals
| # | detector | section | sidecar line (the agent's) | dependent-of | disposition |
|---|---|---|---|---|---|
| 1 | D-layout-surface | §Surface: cli → Component — Primary content block 2 | the block gains the `not-delivered` / `empty-text` refusal pair, decided by the client before any request, with its hint | — | apply · checks 1, 5 |
| 2 | D-layout-surface | §Surface: cli → Signature placement (item 2) | a client-side refusal before any frame draws the struck line too | 1 | apply · check 1 (the applied text names both client-side refusals; the `control-character` one predates this chunk and the sentence never named it) |

## test-plan — 13 proposals
| # | detector | section | sidecar line (the agent's) | dependent-of | disposition |
|---|---|---|---|---|---|
| 1 | D-tests-coverage | §6 Path 2 → Verification signal, the trailing-CR sentence | the trailing-CR outcome is fixed and pinned; the unpinned remainder is an inner CR / CR LF | — | apply · check 6. Applied without the relayed ruling's content, as architecture 7 |
| 2 | D-tests-coverage | same bullet, the only-newlines sentence | a newline-only text is refused `empty-text` at once; two cross-process cases named | 1 | apply · check 6 |
| 3 | D-tests-coverage | same bullet, the case list and the local-command sentence | the newline-tail case gains CR, CRLF and two-CR tails | 1 | apply · check 1 |
| 4 | D-tests-coverage | §1 Critical paths → confirmed `send` (path statement) | the typed text drops trailing CR and LF; the `empty-text` refusal added | 1 | apply · playbook "Verbatim upstream copy kept current" |
| 5 | D-tests-coverage | §1, same path → Verification signal | the receipt carries no trailing CR or LF | 1 | apply |
| 6 | D-tests-coverage | §4 → viola-agent-claude → `typed_text` | trailing CR and LF, seventeen cases (was nine), two product callers | 1 | apply · check 1 |
| 7 | D-tests-coverage | §4 → root bin → the send-confirmation matcher | six `trailing_newline` tests take LF, CR and CRLF tails | 1 | apply · check 1 |
| 8 | D-tests-coverage | §4 → root bin → Refusal ordering | `empty-text` after `control-character`, ahead of both wheel reads; five unit cases named | 1 | apply · check 1 |
| 9 | D-tests-coverage | §5 → Wrapper channel → wrapper-side paste validation | the `empty-text` refusal's wrapper and client cross-process cases | 1 | apply · check 1 |
| 10 | D-tests-framework | §3 → Bootstrap phases, `[profile.mutants]` bullet (key file) | `verify_window_` first, ahead of `package(viola-e2e)`; the renamed harness test joins the class; the order lint named | — | apply · checks 1, 5 |
| 11 | D-tests-framework | same key, `[profile.ci]` bullet | the `ci` class holds three cases; the renamed test moves from 120 s to 60 s | 10 | apply · check 1 |
| 12 | D-tests-framework | same key, "The first failure stops scheduling" | kill lines under `mutants`: 10 s, viola-e2e 30 s, the class 45 s | 10 | apply · check 1 |
| 13 | D-tests-framework | same key, "The `viola-e2e` override keeps a 30 s kill" | the 30 s kill has one named exception | 10 | apply · check 1 |

## obs-plan — 6 proposals
| # | detector | section | sidecar line (the agent's) | dependent-of | disposition |
|---|---|---|---|---|---|
| 1 | D-obs-instrumentation | §6 → `detail` code catalog | refusal details gain `empty-text` | — | apply · checks 1, 5 |
| 2 | D-obs-instrumentation | §4 Scenario: Confirmed `send` → Required log fields | the validation-refusal line admits `detail:"empty-text"` on both sides | 1 | apply · check 1 |
| 3 | D-obs-instrumentation | §1 Critical paths → Confirmed `send` → Required log fields | the `detail` list gains `empty-text` | 1 | apply · playbook "Verbatim upstream copy kept current" |
| 4 | D-obs-instrumentation | §1 Telemetry triggers → Vector 2 | the text-free `send-refused` signal names `empty-text` | 1 | apply · the same |
| 5 | D-obs-instrumentation | §6 → Additive field catalog | `text_bytes` is the length without trailing CR and LF | — | apply · checks 1, 5 |
| 6 | D-obs-instrumentation | §4 Scenario: Confirmed `send` → Required span attributes | the `pty.paste_write` gloss follows the wider strip | 5 | apply · check 1 |

## a11y-plan — 0 proposals
`proposals: []`. The return's commentary, stripped: no interactive UI element was added; neither schema changed;
the plan and its nine key files carry no detail list and no typed-text claim.

## Raised by the orchestrator (check 5 — a plan entry no detector proposed; check 4's searches in the report)
| # | doc | section | change | disposition |
|---|---|---|---|---|
| R1 | design-system | §Color Palette → Domain status colors | a `RefusalDetail::empty-text` row beside `control-character`, the same three tokens | apply · routine (the report substantiates it: Symbols / APIs) |
| R2 | design-system | §Surface: cli → Component Patterns 2 (the hint list) | the `not-delivered · empty-text` hint, the founder's exact wording | apply · routine |
| R3 | design-system | §Surface: cli → Exit-code phraseology (exit 13 row) | `empty-text` joins the exit-13 detail list | apply · routine |
| R4 | obs-plan | §4 Scenario: `wait` / `last` | the stale `:125` / `:127` replaced by the two Epoch 6 entry titles | apply · the citation item |
| R5 | test-plan | §6 Security control negatives | `:127` (twice) replaced by the entry "Home and code-bearing file integrity" | apply · the citation item |
| R6 | security-plan | §Input Validation → the `events.ndjson` reader row | the bare `working-route.md:109` replaced by the entry's title | apply · as architecture 16 |

## Check results over the whole set
- Check 2: no two proposals edit one section in opposing directions.
- Check 3: the report's one divergence from the working-route entry is the split, justified by the founder's word (`inputs#I3`); the scope record holds no line.
- Check 6: both disproved claims are disposed (architecture 5 to 7, test-plan 1 and 2).
- Escalations: 7 (the security-plan D-security-input group), resolved on recorded words, no halt. No playbook rule is proposed: the group matched "Accurate this-chunk addition" once the widening class was ruled out.
