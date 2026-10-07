## 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host — a send whose text ends in a newline is delivered and not claimed
**Section:** §Established Decisions → [Delivery Confirmation] · [Human Takeover / Wheel] (the "never move it" parenthetical) · §Standard Contracts → `prompt-submitted` (the "`text` is the pasted text" clause)
**Change:**
- [Delivery Confirmation] carries one measured exception to "every `send` is confirmed", unfixed: the CLI drops a pasted text's last newline before UserPromptSubmit, wrapped or not, so for a sent text whose last byte is a newline `prompt-submitted`'s `text` is one byte short and the exact match fails. The text is delivered and runs a turn, its prompt is filed `human` and moves the wheel (`cause` `human-input`), and the send ends `not-delivered` / `no-prompt-submitted` when the window closes.
- `prompt-submitted`: `text` for a text that ended in a newline is one byte short of it and does not match the sent text.
- [Human Takeover / Wheel]: an in-flight send whose text ends in a newline is not relabelled, so its own prompt moves the wheel to the human.
- No remedy is built; it is carried on the working route.
**Why:** measured end to end on 2.1.287 on a verified and an unverified home. It was escalated at this wrap as a qualification of a locked decision; the overseer directed that all three sections say what the product does today, measured and unfixed, with the remedy left to its route card.
**Kept:** the exact-match claim itself: no trim was added.
**Ref:** .andromeda/runs/2026-10-07T10-53-41-wrap/
