## 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host — the long-paste frame as measured beside typed text and between two pairs
**Section:** §Established Decisions → [CLI Version Compatibility] (the long-paste wrapper) · [Delivery Confirmation] (the normalisation parenthetical) · §Standard Contracts → `prompt-submitted` (the frame parenthetical)
**Change:**
- The frame: two newlines before the open tag; after the close tag one newline at the prompt's end, two when typed text follows, three in all between two adjacent pairs. Was "two before and one after".
- `hook::unwrap_pastes` removes the two before, the one after, and a second after the close when text follows that is not the next pair's own two-newline frame. A third newline before, a second after at the prompt's end and a lone one before stay. Was "a second after … stay[s]" without the condition.
- The id is 4 hex characters, one for every pair of a session (the two pairs of one prompt included), different between sessions. Was "differs per paste".
- A pasted text's own last newline never reaches the hook: a wrapped text ending in a newline gets none added before the close tag (the static reading, now measured), and an unwrapped text loses it too.
- The frame beside typed text and between two pairs is compiled on one measurement (2.1.287) and held by `hook.rs` unit cases only; no `viola verify` run types those shapes, so the `long-paste-wrapper` row re-validates only the lone-paste frame. Whether they get a probe is open and the founder's.
- "Unmeasured" now names only a long or a repeated text pasted under the paste hint. The wrap threshold and the feature flag stay read statically.
**Why:** three live shapes were measured on `claude` 2.1.287 and the paste-then-typed one falsified the base unwrap, which this chunk fixed inside `unwrap_pastes`. The probe gap was escalated at this wrap; the overseer directed it recorded and brought as a route card, since a new Run B paste is a widening and an accepted limit is a ruling on the ledger rule, both the founder's.
**Kept:** the exact-match claim and the unwrap's "nothing wider" rule.
**Ref:** .andromeda/runs/2026-10-07T10-53-41-wrap/
