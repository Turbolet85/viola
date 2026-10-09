
## 2026-10-09-inner-cr-and-crlf-in-a-sent-text — the text_bytes gloss follows the typed text's inner CR rule
**Section:** §4 Scenario: Confirmed `send` (CL-1) from driver to readback (`pty.paste_write`) · §6 Additive field catalog (the `send-issued` / `send-confirmed` / `send-refused` row)
**Change:** `text_bytes` is the length of the typed text, the sent text with every CR LF pair and every other CR as one LF and without its trailing CR and LF characters (was: "the sent text without its trailing CR and LF characters"). Both sites carry the one wording. Read live on 2.1.287: a 70-byte text with one inner CR LF logged `text_bytes` 69, an 82-byte text with two logged 80.
**Why:** the typed text's rule changed at this chunk (architecture [Delivery Confirmation]), and `text_bytes` is that text's length. No field, span, event or detail is added, renamed or removed.
**Ref:** .andromeda/runs/2026-10-09T20-50-14-wrap/
