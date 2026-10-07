## 2026-10-07-a-send-ending-in-a-newline-is-confirmed — `text_bytes` counts the typed text
**Section:** §4 Scenario: Confirmed `send` (CL-1) from driver to readback (Required span attributes, Required log fields) · §6 (the field catalog's `send-issued` / `send-confirmed` / `send-refused` row)
**Change:** `text_bytes` on the `pty.paste_write` span, on the `send-issued` line and in the catalog row is the length of the typed text: the sent text without its trailing LF characters. It was stated with no qualifier and read as the text as sent. The field's name and type are unchanged; for a send ending in newlines its value is smaller than before by the number of trailing LF.
**Why:** `send` types a validated text without its trailing LF (the founder's ruling, live, 2026-10-07T15:21Z, relayed by the overseer), and `text_bytes` reads that one typed text.
**Kept:** "Never text" and the lines saying only the count is logged; no schema, span or line was added.
**Ref:** .andromeda/runs/2026-10-07T19-12-23-wrap/
