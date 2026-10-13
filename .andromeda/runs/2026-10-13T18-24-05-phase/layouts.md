# layouts extract

## Relevance
partial — the chunk renders on the cli surface only through `viola verify`'s output on a Ctrl-C / TERM stop; it adds no web-spa element, no column, no focusable element and no dialog; the two literal moves (program name, `--settings`) have no layout footprint.

## Constraints
- layout-templates §Surface: cli, Output structure — `viola verify` requires each step line `[NN/MM] <row id> <row words>  pass|fail` to be appended once, in ledger order, with no progress bar and no redraw; an interrupted run may therefore stop mid-sequence but never rewrites or erases a step line already printed. What the plan says, if anything, about the lines of an interrupted run is not stated there: the interrupted-run output form is unplanned, and research / P2 must say what (if anything) is printed.
- layout-templates §Surface: cli, Output structure — `viola verify` makes the `stamped <ver>  <n> pass  <m> fail` summary the last stdout line and ties it to a written stamp ("the stamp still written" on a failing row); since the scope expects an interrupted run to write no stamp, a `stamped` summary line on an interrupted run would contradict the plan's form — whether the code prints it on the stop path is research's question.
- layout-templates §Surface: cli, Output structure — `viola verify` fixes the exit set at 0 (every row passes) and 1 (a failing row or a refusal); an interrupted run's exit status is not among the planned forms, so any status chosen (from architecture §Conventions) is a new verify output shape, not a reuse of an existing one.
- layout-templates §Component — Primary content block 2: refusal lines and the `unable` column requires every `viola verify` exit-1 refusal to be an `unable: <text>` line followed by its own `hint:` line, naming no name, path or pid, and a failed or panicked verify to print exactly `error: internal error` once from the one catch site with no hint; any line the stop path prints must take one of these forms or be a planned new one.
- layout-templates §Component — Primary content block 2: refusal lines and the `unable` column (Streams) requires results on stdout and refusals, hints and errors on stderr, never mixed; a stop-path line, if any, keeps that split.
- layout-templates §Surface: cli (Expression level) requires `viola verify` output to stay plain ASCII with no colour and no cursor control; no stop-path line may add SGR or an in-place redraw.
- layout-templates §IA notes (Output as a contract) holds the human words and lines stable because LLM drivers read them: a new human line for an interrupted verify is a layout amendment at wrap, never a phase edit, and the uninterrupted run's lines (step lines, summary, `--help` paragraph) must stay byte-identical.

## Patterns to follow
- The exit-1 `unable: <text>` / `hint: <one plain instruction>` pair with fixed text and no path, pid or name, as verify's existing four refusal pairs use (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- Append-only line output: each line written once, nothing redrawn, the terminal scrollback as history (per layout-templates §Component — Footer / terminator).
- The exit code as the terminator, with no `done.` / `success` / summary banner word (per layout-templates §Component — Footer / terminator).
- `viola verify --help`'s one static ASCII paragraph naming no path stays as written; the stop behaviour adds no help text unless amended at wrap (per layout-templates §Surface: cli, Output structure — `viola verify`).

## Anti-patterns to avoid
- No `stamped …` summary line, no spinner, no `Interrupted…` progress redraw and no colour on the stop path (per layout-templates §Surface: cli, Output structure — `viola verify` and §Surface: cli Expression level).
- No stop-path line that names a removed directory, a path or a pid (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- No hint naming `viola release` (per layout-templates §Component — Primary navigation (verb structure)).

## Contract bindings
- layout ↔ architecture §Conventions: the interrupted run's exit code is read from architecture's exit-code table; layout-templates §Surface: cli, Output structure — `viola verify` plans only 0 and 1, so a different code binds a layout amendment.
- layout ↔ obs-plan event catalog: any `obs_event!` line the stop path writes goes to the process log, never to stdout/stderr, keeping the layout's stream split (layout-templates §Component — Primary content block 2: refusal lines and the `unable` column, Streams).
- layout ↔ security-plan §Error Handling: the no-path, no-pid rule on stop-path lines is shared (layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).

## Acceptance criteria contributions
- (layouts) An uninterrupted `viola verify` prints the same step lines and the same last-line `stamped <ver>  <n> pass  <m> fail` summary as before the chunk, byte for byte (per layout-templates §Surface: cli, Output structure — `viola verify`).
- (layouts) A Ctrl-C / TERM-stopped verify prints no `stamped` summary line, rewrites no step line already printed, and emits no SGR or cursor-control byte (per layout-templates §Surface: cli, Output structure — `viola verify`).
- (layouts) Every line the stop path prints on stderr names no path, pid or removed directory, and stdout and stderr carry no mixed content (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- (layouts) Any new human line or exit status for the interrupted run is recorded as a wrap amendment to layout-templates §Surface: cli, Output structure — `viola verify`, not left as an unplanned shape (per layout-templates §IA notes).
