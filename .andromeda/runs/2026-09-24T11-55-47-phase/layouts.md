# layouts extract

## Relevance
Partial. The chunk does not create or change any web-spa surface, and it does not change the wireframe or focus order of any cli surface. It touches the cli surface only indirectly: the print/raw-log bans and their `#[allow]` sites decide which modules may write each cli output line, and to which stream. My plan defines those lines and streams as a contract.

## Constraints
- Stream split is fixed: results go to stdout; `waiting:`, the `send` issue line, refusals, `hint:` lines, errors and the `viola ui` launch line go to stderr; the two are never mixed. Any code or `#[allow]` placement the print bans cause must keep this split (per layout-templates §Surface: cli › Component — Primary content block 2: refusal lines and the `unable` column, "Streams" bullet).
- Human columns and words are a stable contract that LLM drivers may read. Making the gates pass must not change any human-output character, column or word order (per layout-templates §Surface: cli › IA notes, "Output as a contract").
- `viola run` prints nothing of its own while the child runs. Its only viola-authored output is the exit-1 start refusal (`unable: <name> is already live` plus `hint: viola list`) on stderr. The "`println!` in a `run` path fails" proof must still leave that refusal output working through an output module (per layout-templates §Surface: cli › Output structure — `viola run`; §Primary content block 2, the fixed-message exceptions).
- `viola hook` / `viola mcp` have no human surface, so a print ban on the hook path matches the layout. The hook decision body is machine output, and my plan does not define it (per layout-templates §Surface: cli › Component — Header / banner, "No banner anywhere").
- `viola ui` prints exactly one unstyled launch line to stderr. Its credential content is deferred to the security plan. If that line exists at HEAD, it is one of the allowed stderr sites (per layout-templates §Surface: cli › Primary screens (commands)).
- `--json` outputs one document on stdout, with no stderr line and no hint text. The module that owns `--json` needs `print_stdout` allowed, and it must not start printing a stderr hint (per layout-templates §Surface: cli › Output structure — `viola list --json`; §Primary content block 2, "Exit codes as the typed tail"; Decisions Log overseer fix pass T4).
- TTY-only lines (the `send` issue line `[  ] open …`, `waiting: <name>`, `last  <name>  …`) go to stderr and only on a TTY. Moving print sites into output modules must keep them TTY-gated (per layout-templates §Surface: cli › Output structure — `viola send`; §Output structure — `viola wait` / `viola last`).

## Patterns to follow
- Keep all cli styling and column padding in the single hand-written SGR/padding module in the `viola` bin (per layout-templates §Surface: cli › Tooling context). That module and the stream writers are the natural places for a local `#[allow(clippy::print_stdout, clippy::print_stderr)]`. Which modules actually own output at HEAD is research's question.
- Append lines, never redraw. Output has no spinner, progress bar or cursor control, and the exit code is the terminator (per layout-templates §Surface: cli › Component — Footer / terminator; Decisions Log "Motion trigger placement", cli). Gate-related changes should add no new human output lines.
- Refusal form is `unable  <name>  <reason>  <detail>` followed by one `hint:` line as the last stderr line (per layout-templates §Surface: cli › Component — Primary content block 2). Refusal printing that is moved behind the ban should keep this two-line shape.

## Anti-patterns to avoid
- Spreading `#[allow(print_*)]` onto call sites in command logic, or onto `hook` / `run` / `mcp` paths, to get past the ban. My plan gives those paths no human output beyond the listed lines (per layout-templates §Surface: cli › Component — Header / banner; §Output structure — `viola run`).
- Adding status or terminator output (`done.`, `success`, a spinner, a version line on normal output) while restructuring output. It is banned outright (per layout-templates §Surface: cli › Component — Footer / terminator; §Component — Hero / signature output line, "What never happens").

## Contract bindings
- layouts cli streams ↔ obs-plan §11 Logs / §3 `obs-ci-gate-wire` print bans: the `#[allow]` allowlist of output modules (`--json`, human CLI stderr, `ui` launch line) should match the cli output sites my plan defines, per §Surface: cli › Primary content block 2 "Streams".
- layouts `viola ui` launch line ↔ security plan: the credential-bearing content is deferred to security. This chunk should only relocate or allow that line, not change its content (per layout-templates §Decisions Log, "Notable surface-specific deferrals").
- layouts output contract ↔ test-plan e2e stdout/stderr assertions: moving print sites must leave the stream-level expectations of existing tests unchanged (per layout-templates §Surface: cli › IA notes, "Output as a contract").

## Acceptance criteria contributions
- (layouts) For each cli output line that exists at HEAD (`list` header/captions/rows, `send` issue/outcome, `wait`/`last` results and TTY stderr lines, wheel/handoff/dialog verb lines, refusal plus `hint:`, `verify` step/summary lines, `ui` launch line, `run` start refusal), the stream and characters are the same before and after the lint bans land. Which of these exist at HEAD is research's question. (per layout-templates §Surface: cli › IA notes "Output as a contract"; §Primary content block 2 "Streams")
- (layouts) After the bans, `print_stdout`/`print_stderr` `#[allow]` appears only on modules that own a cli output stream, and none on `hook`/`run`/`mcp` command logic. The `run` exit-1 refusal still prints `unable: … is already live` followed by `hint: viola list` on stderr. (per layout-templates §Surface: cli › Output structure — `viola run`; §Component — Header / banner)
- (layouts) `--json` output stays one document on stdout with no stderr line or hint text after the output-module restructuring. (per layout-templates §Surface: cli › Output structure — `viola list --json`; §Primary content block 2 "Exit codes as the typed tail")

## Relevant amendment history
(none) The sidecar `D:/dev/projects/viola/.andromeda/layout-templates-amendments.md` does not exist, which is normal. The plan's own Decisions Log has one related item: overseer fix pass T4, which says `--json` carries no hint text. It is cited above under Constraints.
