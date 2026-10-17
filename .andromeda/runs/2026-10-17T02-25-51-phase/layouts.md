# layouts extract

## Relevance
relevant — the chunk shapes the cli surface's machine view (`--json` per verb), its exit tail, its `unable:` / `hint:` pairs and the global `--home`; its `config.json` and `state-recovered` field bullets (scope §5, §6) are outside layout.

## Constraints
- layout-templates §Component — Primary navigation (verb structure) requires the global flag `--home` to be the same on every verb, and `--json` to be the machine view of every data verb; it names `viola revive`'s missing `--json` (its `--list` rows human-only) as owed to the working-route entry "CLI machine contract". This conflicts with layout-templates §Output structure — `viola revive` ("The verb takes no `--json`") and with the scope's CARRY 2 wording; which reading stands is P4's fork, and the losing section is a layout amendment.
- layout-templates §Component — Primary content block 2: refusal lines and the `unable` column requires, under `--json`, the same outcome as one document on stdout with no stderr line and no hint text, keeping arch's shape (`"detail":null` for exit 21) until arch names the detail codes (that naming is "Exit-cause code catalogue", not this chunk).
- layout-templates §Component — Primary content block 2: refusal lines and the `unable` column requires the exit tail 10 `human-typing` · 11 `budget-paused` · 12 `unverified-cli` · 13 `not-delivered` · 14 `unknown` · 20 wrapper fault (`error: wrapper fault  <code>`) · 21 `unable  instance-unreachable` (`viola list` takes its fixed state pair instead), and exit 1 with exactly `error: internal error`, no hint, from the one catch site for a failed or panicked `cli` verb.
- layout-templates §Component — Primary content block 2: refusal lines and the `unable` column requires one `hint:` per cause directly under each refusal and last on stderr, so exit-1 / exit-21 causes are distinguishable from the last stderr line; no hint for `unknown`, wrapper fault or `internal error`; a hint never quotes sent or upstream text and never names a path or pid. layout-templates §Component — Primary navigation (verb structure) adds: no hint names `viola release`.
- layout-templates §Component — Primary content block 2: refusal lines and the `unable` column requires results on stdout and `waiting:`, the issue line, refusals, hints and errors on stderr, never mixed.
- layout-templates §Output structure — `viola revive` requires the `--list` row form `<ts>  <cause>  <id>` (stdout, no header, no colour, empty stderr) and lists six exit-1 `unable:` / `hint:` pairs (state files · already live · no logged session · no logged session with that id · open in another claude (`session-live`) · recorded directory missing), stdout empty; the scope counts five refusal codes, so whether the unknown-`--id` pair carries its own code is research's question.
- layout-templates §Surface: cli (Expression level) requires expression 0.0 for `--json`: no colour, no non-ASCII glyph, no cursor control.

## Patterns to follow
- `viola list --json`: one document on stdout, no header line; a failed strict-modes check prints `{"v":1,"error":"instance-unreachable","detail":null}`, exit 21, stderr empty (per layout-templates §Output structure — `viola list --json`).
- `viola send --json`: only the typed document (`ok` with `submitted_at` + `cursor`, `ok` with `confirmed:false` + `unconfirmable`, or `refusal` + `detail`), typed exit, no glyph, no issue line, no hint (per layout-templates §Component — Hero / signature output line (the `viola send` readback mirror) and §Output structure — `viola send`); a client-side refusal before any frame is likewise one stdout document with no stderr line (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- `viola wait` / `viola last --json`: exactly one document on stdout (`ok`, or the exit-21 / wrapper-fault document), nothing on stderr, no `waiting:` line (per layout-templates §Output structure — `viola wait` / `viola last`).
- `viola verify`'s human form: step lines plus the `stamped <ver>  <n> pass  <m> fail` summary last on stdout, exit 0 / 1, refusal and stop pairs on stderr; layout documents no `verify` machine document, so any one P4 plans is a new layout form (per layout-templates §Output structure — `viola verify`).

## Anti-patterns to avoid
- A glyph, hint, issue line, `waiting:` line or any stderr byte alongside a `--json` document (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- Changing a human column or word to carry new data: human output is a contract read by LLM drivers, new fields go into `--json` first, and adding a human column is a design change (per layout-templates §IA notes).
- A terminator word, banner or summary line added to close a verb's output: the exit code is the terminator (per layout-templates §Component — Footer / terminator).

## Contract bindings
- layouts §Output structure — `viola list --json` / §Component — Hero / signature output line ↔ architecture §Standard Contracts "CLI `--json` output" (the document shapes the layout forms print).
- layouts §Component — Primary content block 2 (exit tail, per-cause hint) ↔ architecture §Conventions "CLI exit codes" and design-system cli pattern 2 (hint per cause).
- layouts §Component — Primary content block 2 (`"detail":null` until codes are named) ↔ the later route entry "Exit-cause code catalogue" (v1-18).
- layouts §Output structure — `viola --help` (`Usage: viola [--home <dir>] <verb> [args]`) ↔ architecture §Cross-cutting Config management (the `--home` resolution order).

## Acceptance criteria contributions
- Every data verb given `--json` prints exactly one document on stdout with an empty stderr, and its exit is the one the documented exit tail maps its outcome to (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- Each human refusal cause a verb can reach ends stderr with its own distinct `hint:` line; none names `viola release`, a path or a pid; `unknown`, wrapper fault and `internal error` print no hint (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- `--home` is accepted in the same form by every verb (per layout-templates §Component — Primary navigation (verb structure)).
- If `viola revive` gains a machine document, its human `--list` rows and its six `unable:` / `hint:` pairs stay byte-identical, and §Output structure — `viola revive`'s "takes no `--json`" sentence is amended in the same chunk (per layout-templates §Output structure — `viola revive`).
