## 2026-10-04-confirmed-send-with-cl-1-records — Path 2 as landed; paste_text joins the fuzz targets
**Section:** §1 Test Scope Summary (the confirmed-`send` path) · §2 Test pyramid (Property-based row) · §6 Scenario Path 2 (Surfaces; the `local` and Playwright bullets) · §6 Property suite · §7 Fake agent (Modes) · §3 → `5-command-implementation`
**Change:**
- Path 2 as landed: `path2_send_confirms_with_cl1_events` covers cli, the wrapper channel and the receipt on three OSes; MCP `send` is owed to `:102`, SSE to `:131`, the web half and Playwright to `:139`.
- `local` was exit 0 `{confirmed:false, detail:"unconfirmable", cursor}`; now exit 13 `not-delivered`/`no-prompt-submitted` while no local-command row is compiled, never presumed delivered; `unconfirmable` (and the Playwright local line's `data-rb="unconfirmable"`) is owed to `:82`. §1's "a local command yields `unconfirmable`" says the same.
- Fuzz targets: `paste_text` joins (`validate_paste_text` against a per-char oracle over lossy UTF-8; 8 synthetic seeds, byte-exact under `.gitattributes`); the `validate_paste_text` property landed at 512 cases.
- Fake agent: `--vt100-panic-bytes` was "lands with confirmed `send`'s chaos case"; now built (after its `start` receipt it writes `e4 b8 ad` once and receipts nothing new).
**Why:** what confirmed `send` landed; the `local` outcome follows F2, the overseer's hold of the local-command rows with the typed probe.
**Ref:** .andromeda/runs/2026-10-04T06-44-39-wrap/
