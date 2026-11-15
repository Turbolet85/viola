# relay-1 — the overseer's answer to implement's technical fork (fixture key names)

Question (AskUserQuestion, header «Fixture keys»): Plan step 3 names the new keys in
fixtures/viola-0.1.0/retired-words.json `statusline_snapshot_key`, `statusline_ledger_rows`, `statusline_hook_word`,
`statusline_source_file`, `statusline_diag_subject`. The tests that read them (steps 1, 2, 4) must spell the key, and
that spelling sits under crates/ and tests/, where step 13's identifier probe (`grep -rn -E 'statusline|…' src crates
tests …`, expect exit 1) reads it. The two steps can't both hold as written. How should the keys be named?

Answer selected: «Neutral keys (Recommended)»

Notes, verbatim: The overseer's technical answer (the operator is away; no founder fork): neutral keys, the values
unchanged, the probe exactly as planned — never a split string, which would game the probe. Record it as a deviation
from step 3's key names for the wrap.
