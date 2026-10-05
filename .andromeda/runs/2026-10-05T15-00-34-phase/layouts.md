# layouts extract

## Relevance
partial — no web-spa surface is touched; the `permission` end-to-end case exercises two cli-surface outputs (the `viola wait` dialog result line and the `viola answer` result/refusal lines), and `viola hook` has no human surface (per layout-templates §Component — Header / banner (the BAY context line)).

## Constraints
- A `viola wait` that is woken by a dialog prints one stdout result line of the form `<kind>  <name>  dialog <id>  cursor <n>`. For this chunk the kind word is `permission`. `DIALOG` is not coloured in that line because it is a result, not the board. A dialog event that carries no `dialog_id` reads `dialog unknown` (per layout-templates §Output structure — `viola wait` / `viola last`).
- `waiting:` goes to stderr only when stderr is a TTY and `--json` is not set. There is no spinner and no elapsed counter. Under `--json`, each verb prints exactly one `{"v":1,"ok":…}` document on stdout and nothing on stderr (per layout-templates §Output structure — `viola wait` / `viola last`).
- A successful `viola answer <target> <dialog_id>` prints `answered  <name>  dialog <id>` as its last line, with no terminator word. An unknown or non-pending id prints `unable  <name>  not-delivered  unknown-dialog` on stderr, exits 13, and is followed by its `hint:` line as the last stderr line (per layout-templates §Output structure — wheel, handoff and dialog verbs; §Component — Footer / terminator).
- `viola answer` takes its arguments in the order `<target> <dialog_id>`, mirroring the channel `params`. `<target>` is always a `ViolaName` (per layout-templates §Component — Primary navigation (verb structure)).
- Results go to stdout. `waiting:`, refusals, hints and errors go to stderr, and the two are never mixed. A failed or panicked `answer` / `wait` prints exactly `error: internal error` once, exits 1, and has no hint (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- `viola hook` (including the `permission-request` arm) has no human surface. The decision body it prints on stdout is a hook contract, not a layout output (per layout-templates §Component — Header / banner (the BAY context line)).

## Patterns to follow
- `tests/cli_answer.rs` already has `question` / `plan` cases. The new `permission` case asserts the same `wait` and `answer` line shapes and changes only the kind word. Whether those existing cases assert the human lines or the `--json` documents is research's question (per layout-templates §Output structure — `viola wait` / `viola last`).
- Output as a contract: human columns and words stay stable because LLM drivers read them. Any new field goes into `--json` first, and the human line is not widened (per layout-templates §IA notes).
- Refusal vocabulary is shared across verbs: `unable  <name>  <reason>  <detail>` plus one `hint:`. The hint names no path, no pid and no upstream text (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).

## Anti-patterns to avoid
- No colour on the `permission` kind word, or on `DIALOG`, in `wait` output. Amber `DIALOG` belongs only to `viola list` (per layout-templates §Surface: cli, signature placement paragraph).
- No `done` / `success` / `✓` terminator, and no extra human column or field on the `wait` / `answer` lines. A permission-specific field such as the tool name or `input` goes to `--json` or the event, never into the human line (per layout-templates §Component — Footer / terminator; §IA notes).
- No hint names `viola release`. No hint quotes upstream text: a PermissionRequest's tool `input` is upstream content (per layout-templates §Component — Primary navigation (verb structure); §Component — Primary content block 2).

## Contract bindings
- cli `wait` dialog line ↔ architecture §Standard Contracts (`wait` result, `dialog_id` on the event line): the layout's `dialog <id>` / `dialog unknown` word reads from the wrapper-assigned `dialog_id`.
- cli `answer` refusal ↔ architecture §Conventions exit codes (13 `not-delivered`) and design-system cli pattern 2 (hint keyed by reason · detail).
- `viola hook` stdout decision body ↔ architecture [Hook Contract]: hook-owned, outside this layout.

## Acceptance criteria contributions
- (layouts) A parked `viola wait`, woken by the replayed PermissionRequest, prints exactly one stdout line `permission  <name>  dialog <id>  cursor <n>`, with no SGR bytes. Under `--json` it prints exactly one `{"v":1,"ok":…}` document on stdout and empty stderr (per layout-templates §Output structure — `viola wait` / `viola last`).
- (layouts) `viola answer <name> <id>` on the pending permission dialog exits 0, and its last stdout line is `answered  <name>  dialog <id>` (per layout-templates §Output structure — wheel, handoff and dialog verbs; §Component — Footer / terminator).
- (layouts) If the case covers a re-answer or an unknown id: exit 13, stderr `unable  <name>  not-delivered  unknown-dialog`, then a `hint:` line as the last stderr line, and nothing on stdout (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
