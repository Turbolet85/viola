
## 2026-10-13-paste-newline-ledger-row — eighteen ledger rows, `paste-newline`, Run B's fifth paste, the owed rows moved to "Owed ledger rows"
**Section:** §Established Decisions → [CLI Version Compatibility] · [Delivery Confirmation] · §Cross-cutting Patterns (Capability ledger) · §Standard Contracts (`list`) · §Occupied Resources (Repository · Binary, subcommands and exit codes) · §Infrastructure Patterns → CI/CD approach · Crate dependency direction
**Change:**
- Was seventeen rows; now eighteen, the members of `LedgerRow::ALL`, the last `paste-newline` (an LF typed inside a bracketed paste stays inside the one prompt submitted). It is checked by `pasted(&probes.trusted, PROBE_NEWLINE_PASTE)`. Stamped on live 2.1.287 on the Linux dev host only; Windows owed to "Windows-only live measurements".
- `viola verify` prints `[NN/MM]` step lines, `MM` = `LedgerRow::ALL.len()`, 18.
- A stamp of the fourteen or the seventeen older ids, all `pass`, reads unverified. Every home stamped before this chunk reads `cli_verified:false` until its next `viola verify`.
- The hypothetical PermissionRequest `updatedInput` body row is now the 19th.
- Run B: was four compiled pastes; now five. `PROBE_NEWLINE_PASTE` (113 bytes, one inner LF, no CR) goes before `/clear` under the same per-paste guards. `FRAMING_TURNS` `paste-1` · `paste-2` · `paste-3` · `clear-1`, and `box_wait` holds after each added text turn.
- [Delivery Confirmation] and Capability ledger: the inner-LF shape is rowed. Was three relied-on dependences without a row; now two, `viola revive`'s `--resume` and the statusline's three shapes.
- The rows still owed (and the `claude agents --json` join-field row in `list`) moved from "Paste newline ledger row" to "Owed ledger rows". `cli_verified` is eighteen-row.
- Repository: `2.1.287` holds 5 framing variants (29 files), stamped at eighteen rows (`18 pass  0 fail`); `--framing` replays `paste-3` through `framing_stem`, with no option added.
- CI/CD: verify covers eighteen rows. Crate dependency direction: `ledger` holds eighteen rows and `PROBE_NEWLINE_PASTE`.
**Why:** The newline paste is a boundary widening of the ratified verify child set. Ratified by the founder's own answer to P4 fork 1, relayed verbatim by the operator (the chunk's `inputs#I2`). The wrap's card confirm of the proposals inside it is PROVISIONAL (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary. The owed rows' move is his own answer to fork 3 (`inputs#I2`), minted on the operator's word (`inputs#I4`). Counts carry their rule (`LedgerRow::ALL`).
**Kept:** the `stamped 2.1.287  17 pass  0 fail` of chunk 2026-10-08's re-verify round stands as a dated record.
**Ref:** .andromeda/runs/2026-10-13T21-52-39-wrap/
