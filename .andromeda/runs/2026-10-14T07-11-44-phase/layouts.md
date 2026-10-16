# layouts extract

## Relevance
partial — the chunk touches three CLI text surfaces (`viola verify`'s step lines and count, `viola send`'s outcome line for a local command, `viola revive --list`'s cause word); no web-spa surface is built or changed, and no wireframe, focus order, breakpoint or modal applies.

## Constraints
- `viola verify` prints one stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|fail`, in ledger order, and `MM` is `LedgerRow::relied()`'s length per OS; the plan's enumerated ledger-order list "grows as owning chunks land their rows", so each new row needs its id, its row words and its place in that order stated (per layout-templates §Output structure — `viola verify`).
- `viola verify`'s summary `stamped <ver>  <n> pass  <m> fail` is the last stdout line; plain ASCII, no colour (`fail` never coloured), each step line appended once with no progress bar and no redraw (per layout-templates §Output structure — `viola verify`).
- A completed `verify` run closes on that summary, and no terminator word or banner is added (per layout-templates §Component — Footer / terminator).
- `viola send`'s outcome line is `[RB] read back …` on stdout, exit 0, when the send is confirmed by the matching `prompt-submitted` or, for `/clear` on a verified CLI version, by its new-session post-condition; an unmeasured local command takes `[  ] unconfirmable …` on stdout, exit 0; a wrapper refusal takes `[/ ] unable …` on stderr with its exit code. The plan names only `/clear`'s post-condition (cause `clear`, a NEW session id); a `/compact` confirmed by a same-id `session-start` of cause `compact` is not yet in the plan and needs its sentence amended (per layout-templates §Surface: cli, Signature placement).
- The mirror's vocabulary is fixed (`open` / `read back` / `unable` / `unconfirmable`), uncoloured, and `turn-ended` is never reported as a send outcome — a `/compact` confirmation reuses `[RB] read back`, adding no new state word (per layout-templates §Component — Hero / signature output line (the `viola send` readback mirror)).
- `viola revive --list` prints `<ts>  <cause>  <id>` rows whose cause is `startup`, `clear`, `resume` or `compact` as logged, `unknown` otherwise; `revive` takes no `--json`, and its refusal pairs and hints are fixed — a `--resume` row adds no line here (per layout-templates §Output structure — `viola revive`).
- The human columns and words are a stable contract that LLM drivers may read; a change goes into `--json` first, never as a new human column (per layout-templates §IA notes).

## Patterns to follow
- The 18th–21st rows' landing: each appended its own `[NN/MM] <row id> <row words>` example line to the verify block and its id to the ledger-order list, with the per-OS relied count explained beside `MM` (layout-templates §Output structure — `viola verify`, the `statusline-*` rows and "21 on Unix, 18 on Windows").
- The `local-command-clear` row's words (`/clear starts a new session and submits no prompt`) are the model for a `/compact` row's words, stating the post-condition the row measures (layout-templates §Output structure — `viola verify`).
- The `/clear` confirmation path in the send mirror — a ledger post-condition replacing `prompt-submitted` as the read-back trigger — is the shape a `/compact` post-condition follows (layout-templates §Surface: cli, Signature placement).

## Anti-patterns to avoid
- No spinner, progress bar, redraw or colour in `verify` (a long-running live probe such as a compaction or a resumed start still prints its step line once, appended) (per layout-templates §Output structure — `viola verify`).
- No `Sending…`, `✓` or `done` for a local command, and no green/red on `read back` / `unable` (per layout-templates §Component — Hero / signature output line (the `viola send` readback mirror)).
- No refusal line or hint that names a path, a pid or a logged session id (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).

## Contract bindings
- verify count ↔ ledger: `MM` binds to `LedgerRow::relied()` (`crates/viola-agent-claude/src/ledger.rs`); where each new row is relied (per OS) moves `MM` on that OS, and the Windows count waits on a Windows stamp — the layout states the count's rule, research confirms the per-OS set.
- send mirror ↔ web readback: the web readback box's trigger list names `session-start` cause `clear` for `/clear` only (layout-templates §Component — Hero / signature section, web-spa); a `/compact` post-condition binds there too, though the web page is not built yet (Epoch 8) — whether to amend that trigger sentence now is a P4/drift question.
- send mirror ↔ architecture/obs: the `session-start` event's cause `compact` and the send slot's claim (`src/run/send/slot.rs`) feed the `[RB]` line; whether the code already confirms a same-id `compact` session-start is research's question.

## Acceptance criteria contributions
- `viola verify` prints one `[NN/MM] <row id> <row words>  pass|fail` line per relied row including each new row in its stated ledger position, `MM` equal to `LedgerRow::relied()`'s length on the host OS, and ends with `stamped <ver>  <n> pass  <m> fail` as the last stdout line, plain ASCII (per layout-templates §Output structure — `viola verify`).
- On a verified CLI, `viola send` of `/compact` prints `[RB] read back  <name>  …` on stdout with exit 0 once its post-condition is seen, and no `unable` / `no-prompt-submitted` line (per layout-templates §Surface: cli, Signature placement).
- The plan's verify example block and ledger-order list, and the Signature-placement sentence naming `/clear`'s post-condition, are amended at wrap to carry the new rows and the `/compact` post-condition (per layout-templates §Output structure — `viola verify` and §Surface: cli, Signature placement).
