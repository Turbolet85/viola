2026-09-28 (2026-09-28-capability-ledger-and-viola-verify, phase P5): not claimed. The concretization fails for this chunk. The chunk is the head of a split ruled at its P4, and it proves six print-mode ledger rows (shim resolution, spine hooks, SessionStart fields, prompt verbatim, Stop message, largest hook payload), `viola verify` as the sole `stamps.json` writer with the step counter and `stamped` summary, scrubbed `--record`, and `run`'s `cli_verified` degrade. The acceptance also needs rows the print mode cannot reach, and each lands with the chunk that owns it:
- the typed-input rows (long-paste wrapper, tag escaping, harness prefixes) → working-route :58/:68
- the local-command rows → :60
- the dialog rows, withholding every dialog answer on an unstamped version, and the `unverified-cli` driver refusal → :62
The cap stays pooled for the chunk that completes it.
