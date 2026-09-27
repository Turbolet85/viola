# layouts extract

## Relevance
partial — the chunk adds no rendered surface; its layout stake is that the new `viola hook` verb (and the `viola run` plugin rewrite) must keep the cli surface's no-output / stream / help-table rules, and that the `prompt-submitted` / `session-start` events it writes are what the web-spa tape and readback box will later render.

## Constraints
- `viola hook` has no human surface: no banner, no version line, no output of any kind on normal paths (per layout-templates §Surface: cli › Component — Header / banner). This is the layout-side statement of the CLAUDE.md fail-open invariant (empty stdout and empty stderr on every path).
- `viola run` prints nothing while the child runs, so the atomic `hooks.json` / plugin rewrite on every start must not add any line to the terminal (per layout-templates §Surface: cli › Output structure — `viola run` and §Primary screens (commands)). The only permitted `run` output is the existing exit-1 start refusal plus its `hint:` line.
- The grouped `viola --help` verb table (board / traffic / wheel / handoff / setup) does not list `hook` (or `mcp`) (per layout-templates §Surface: cli › Output structure — `viola --help`). The new clap verb therefore must not show up as a human verb in the top-level help. Research should find out how the clap dispatch currently builds that table and whether a hidden verb is already supported.
- The verb model is flat, with one lower-case word per verb and no nested noun hierarchy (per layout-templates §Surface: cli › Component — Primary navigation (verb structure)). `hook` is one flat verb and is never a `hint:` target, because hints name only human or driver next steps.
- Stream separation: stdout and stderr are never mixed, and stdout carries results only (per layout-templates §Surface: cli › Component — Primary content block 2: refusal lines). For `hook`, stdout is reserved for the (here empty) hook decision body, and no refusal or `hint:` line is ever emitted.
- The events this chunk writes must carry the fields the web tape displays: the kind word, `origin` (rendered `prompt · human` / `prompt · harness`, with harness lines shown in tertiary ink) and `source` (rendered `source hook` in the expanded line body) (per layout-templates §Surface: web-spa › Component — Primary content block 3: the tower tape and §Wireframe — Tape line, expanded).

## Patterns to follow
- Silent verbs follow `viola run`'s passthrough shape: nothing is printed and the exit code is the terminator (per layout-templates §Surface: cli › Component — Footer / terminator). For `hook` that terminator is always 0.
- The origin vocabulary (`human` / `harness` / `driver`) should match the tape's kind-word pairing (`prompt · <origin>`), so no mapping layer is needed between the event line and the page (per layout-templates §Surface: web-spa › Component — Primary content block 3: the tower tape).
- A driver-origin `prompt-submitted` is the readback trigger that folds into its send line and gets no tape line of its own. The same role goes to `session-start` with cause `clear` for `/clear` (per layout-templates §Surface: web-spa › Signature placement and §Component — Hero / signature section). The origin field should therefore be written so a later consumer can match it, even if this chunk cannot yet decide `driver` (scope item 3).

## Anti-patterns to avoid
- Any stderr or stdout line from `viola hook`: no `error:` line, no `unable` line, no `hint:`, no spinner, no `done` (per layout-templates §Surface: cli › Component — Header / banner and §Component — Footer / terminator).
- Listing `hook` in the grouped `--help` verb table, or naming it in any `hint:` line (per layout-templates §Surface: cli › Output structure — `viola --help` and §Component — Primary navigation (verb structure)).

## Contract bindings
- layouts ↔ architecture §Standard Contracts (event line): the tape expects `prompt-submitted` to carry `origin` in {human, harness, driver} and a `source` field, and it expects `session-start` to carry a cause/source readable as `clear`. The kind and field names are fixed by architecture, and layouts only consumes them (per layout-templates §Surface: web-spa › Component — Primary content block 3: the tower tape).
- layouts ↔ the confirmed-send entry (:58) and CL-1: the readback box turns `read` on a driver-origin `prompt-submitted` from the driven instance (per layout-templates §Surface: web-spa › Signature placement). A `driver` origin this chunk leaves undecided becomes a route note on :58, not a layout gap.
- layouts ↔ a11y: none this chunk. No focusable element, modal or responsive behaviour is added.

## Acceptance criteria contributions
- (layouts) Running `viola hook` with valid, malformed and empty stdin, both inside and outside a wrapped session, leaves stdout and stderr byte-empty with exit 0 (per layout-templates §Surface: cli › Component — Header / banner).
- (layouts) After the chunk, `viola --help` still prints exactly the five groups (board, traffic, wheel, handoff, setup) and no `hook` entry (per layout-templates §Surface: cli › Output structure — `viola --help`).
- (layouts) A `viola run` start that rewrites the plugin `hooks.json` writes no line to the terminal before the child's screen (per layout-templates §Surface: cli › Output structure — `viola run`).
- (layouts) Every `prompt-submitted` line in `events.ndjson` carries an `origin` in {human, harness, driver}, so the tape can render `prompt · <origin>` without a fallback word (per layout-templates §Surface: web-spa › Component — Primary content block 3: the tower tape).
