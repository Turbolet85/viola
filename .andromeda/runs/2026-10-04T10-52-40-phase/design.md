# design extract

## Relevance
partial — the chunk's human-facing surface is CLI only (`viola answer` result and refusal lines, the `wait` dialog-kind result line, and the hints and exit phraseology for `unverified-cli` / `unknown-dialog` / `control-character`). The web strip's cocked `DIALOG <kind>` is Epoch 8 (scope Boundaries), and `viola hook` has no human design surface (per design-system §Surface: cli · Platform-Specific Notes).

## Constraints
- `viola answer <name> <id>` (response on stdin, as in the sample `viola answer builder 7 < response.json`) prints the result `answered  <name>  dialog <id>` on stdout. Its refusal is `unable  <name>  not-delivered  unknown-dialog`, with fields separated by two spaces (per design-system §Surface: cli · Component Patterns 4). The sample's stdin delivery bears on scope §2's open `[inferred]` question of how `response` is supplied, and P3/P4 should weigh it with the architecture CLI conventions.
- Streams: the `answered` result goes to stdout. Each `unable …` refusal goes to stderr with its one `hint:` line directly after it. No hint is printed under `--json`, where the result, refusal or error is one JSON document on stdout with the typed exit code (per design-system §Surface: cli · Streams).
- Exit phraseology that `answer` must follow, per design-system §Surface: cli · Exit-code phraseology:
  - exit 0: stdout `answered`, `{"v":1,"ok":{…}}` under `--json`;
  - exit 10: `unable  human-typing`;
  - exit 12: `unable  unverified-cli`;
  - exit 13: `unable  not-delivered  unknown-dialog` / `control-character`;
  - exit 21: `unable  instance-unreachable` plus the hint for its cause;
  - every refusal: `{"v":1,"refusal":…,"detail":…}`.
- Hint texts are fixed and keyed by reason · detail (per design-system §Surface: cli · Component Patterns 2):
  - `unverified-cli`: `run viola verify for this CLI version`;
  - `not-delivered · unknown-dialog`: `that dialog is not pending; viola list shows the current DIALOG`;
  - `not-delivered · control-character`: `the text contains a control character (only LF, CR, TAB are allowed)`;
  - `human-typing`: the wheel hint, which never names `viola release` (T3).
- The `wait` dialog result line is `<kind>  <name>  dialog <id>  cursor <n>` (for example `question  builder  dialog 7  cursor 49310`). It prints `dialog unknown` when the event carries no `dialog_id`. `DIALOG` is not coloured here, because this line is a result, not the board (per design-system §Surface: cli · Component Patterns 3).
- Colour, per design-system §Surface: cli · Tokens and the Colour decision order:
  - The only CLI colours are amber on the `DIALOG` word in `viola list`, followed by the uncoloured kind word, and SGR dim on `stale` rows.
  - The `answered` line, the refusals, the hints and the `wait` dialog line stay unstyled at every depth.
  - Nothing is styled under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or `viola hook`.
- Human output is ASCII only. Two spaces separate columns, and a refusal's reason from its detail. ` - ` replaces `·` inside a field (per design-system §Iconography · Separator; §Surface: cli · Width).

## Patterns to follow
- Use the refusal shape `unable  <name>  <reason>  <detail>` and then one `hint:` line, as already specified for `send`. `answer` takes the plain form. The `[/ ]` readback mirror and the `unable` padding belong only to `send`'s sample (per design-system §Surface: cli · Streams; Component Patterns 2 and 4). Whether the landed send/wait/last writers (the handoff names `src/human.rs`) already factor out a reusable refusal + hint printer is a question for research.
- Use one fixed `error: internal error` line for an exit-1 fault, with no hint and no chain (per design-system §Surface: cli · Exit-code phraseology, row 1). The handoff reports a single catch-site printer for `send` / `wait` / `last`. Whether `answer` can join it is a question for research.
- On the board, `pending_dialog` surfaces as DIALOG `none` | `DIALOG question` / `DIALOG permission` / `DIALOG plan`, and the `DIALOG` word is never shown without its kind word (per design-system §Color Palette · Semantic Colors rows DialogPending / PendingDialogKind; §Surface: cli · Component Patterns 1). The unknown-dialog hint sends the user to `viola list`. Whether `viola list` renders the DIALOG column from the snapshot's `pending_dialog` yet is a question for research. If it does not, it is a fork about scope for P4.
- On an unverified CLI build, the state reads `<version> unverified-cli`: transport only, dialog answers held back, no colour (per design-system §Color Palette · Semantic Colors row CliVerified::false; §Brand Identity · Verified aircraft type).

## Anti-patterns to avoid
- Never print green `✓` / red `✗`, emoji or any glyph, never colour the `answered` line, and never print a bare "done" / "success". The result line names what happened, to whom, and which dialog (per design-system §Anti-Patterns · Per-Surface Bans cli).
- Never print a spinner, progress bar or elapsed counter while `answer`, `wait` or the hook awaits a dialog. `wait` keeps its one static `waiting:` line, printed on a TTY only (per design-system §Anti-Patterns · Per-Surface Bans cli; §Brand Identity · Expression level, cli 0.2 / 0.0).
- Never print upstream text in a result, refusal, hint or error. That covers question text, plan text, tool `input` and the answer's free text. Never mix data and messages, and never write anything from `viola hook` to stderr (per design-system §Anti-Patterns · Per-Surface Bans cli; §Surface: cli · Platform-Specific Notes).

## Contract bindings
- design ↔ architecture: the exit codes (10 / 12 / 13 / 21), the refusal reason and detail words, the `answer` → `{}` result shape and the refusal order come from architecture §Conventions. Design only supplies how they are spoken. A mismatch is an arch amendment, not a design choice (design-system §Surface: cli · Exit-code phraseology cites architecture.md).
- design ↔ security: hints and errors never quote upstream text, and `control-character` is a refusal that never strips. The human-mode escaper applies only to `wait` / `last` message text, and the dialog result line carries no message text (per design-system §Surface: cli · Component Patterns 3; §Anti-Patterns cli "NEVER print upstream text…").
- design ↔ obs/architecture: design-system §Color Palette ("Not visualised in v1") states that the `answer` behaviours (`allow` / `deny`, `approve` / `revise`) are not logged as events and have no tape line. Whether the obs `dialog-answered` record is a log-only process event or an `events.ndjson` kind is for architecture/obs to settle. If it lands as an events kind, this design row needs an amendment.
- design ↔ a11y: on the board (`viola list`), amber is only ever a second cue beside the `DIALOG` word and the kind word (not colour alone, SC 1.4.1). The web strip's cock motion and contrast are Epoch 8's.
- design ↔ test-plan: the human-mode line shapes above are the expected strings for the CLI tests (§9's `<kind>  <name>  dialog <id>  cursor <n>` witness, and the `answer` unstamped exit-12 row in `cli_controls_not_disableable.rs`).

## Acceptance criteria contributions
- A successful `viola answer <name> <id>` prints exactly `answered  <name>  dialog <id>` on stdout and exits 0. It emits no SGR or other escape bytes even on a TTY. Under `--json`, its one stdout document is `{"v":1,"ok":{…}}` and stderr is empty (per design-system §Surface: cli · Component Patterns 4 / Exit-code phraseology).
- An answer for a dialog that is not pending exits 13. It prints `unable  <name>  not-delivered  unknown-dialog` on stderr, then exactly `hint: that dialog is not pending; viola list shows the current DIALOG`. On an unverified CLI it exits 12 with `unable  <name>  unverified-cli`, then `hint: run viola verify for this CLI version`. Under `--json` neither case prints a hint line, and the result is the refusal object on stdout (per design-system §Surface: cli · Component Patterns 2 / Streams).
- A `wait` woken by each of `question` / `permission` / `plan` prints `<kind>  <name>  dialog <id>  cursor <n>` on stdout, with no colour on any byte, including on a TTY (per design-system §Surface: cli · Component Patterns 3).
- No `answer` / `wait` result, refusal, hint or error line contains the question text, plan text, tool `input` or the answer's free text, and every human line is ASCII only (per design-system §Anti-Patterns · Per-Surface Bans cli; §Surface: cli · Width).
