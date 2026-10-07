## 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host — the cross-session form and the typed task-notification as measured
**Section:** §Established Decisions → [CLI Version Compatibility] (Harness prompt prefixes · Tag escaping) · [Human Takeover / Wheel] (the `HARNESS_PREFIXES` parenthetical and the side-effect sentence)
**Change:**
- On 2.1.287 on the Linux dev host a real cross-session message reaches UserPromptSubmit unescaped, `<cross-session-message from="…" from-name="…" from-mode="…">`, a newline, the text, a newline and `</cross-session-message>`, and the plain prefix files it `harness`. Was "the two cross-session forms rest on a relayed measurement, not yet measured in this repository" and "the escaped form is the one the CLI injects".
- The escaped form rests on the relayed measurement alone; it was not seen and its prefix stays compiled. The four compiled prefixes are unchanged.
- A `<task-notification>` typed at the very start of a prompt arrives as typed, so a human who types it first is filed `harness`. Was "the start-of-prompt position … is unmeasured". The wheel entry states it as measured, not as a new ratification.
- On PATH `claude` (the builder's own CLI, counts only) the `<agent-message from=` and `<task-notification>` forms reach the hook starting at the tag, with no preface.
- The readings came from a scratch probe, not a `viola verify` probe: the harness-prefix ledger row stays owed.
**Why:** one fixed synthetic peer message into a scratch 2.1.287 session, and one typed paste, measured both; the forward reference to this chunk is spent.
**Kept:** the founder's 2026-09-29 ratification of the escaped prefix and its side effect on a human who types that tag.
**Ref:** .andromeda/runs/2026-10-07T10-53-41-wrap/
