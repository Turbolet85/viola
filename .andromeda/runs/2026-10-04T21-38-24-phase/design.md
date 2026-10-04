# design extract

## Relevance
partial: the chunk is wrapper state with no new rendered surface. Design binds only the wording of the `turn-running` refusal that reaches the human CLI (and, later, the web readback). The tokens, typography, spacing and motion sections do not apply.

## Constraints
- The refusal is phraseology. design-system §Brand Identity ("Standard phraseology and 'unable' plus a reason") requires it to print as the fixed words `not-delivered` + `turn-running`, with no filler and no new word. The §Semantic Colors enum table already lists `RefusalDetail::turn-running` as `not-delivered · turn-running`. The chunk adds no new detail word, so no new enum row is owed. Whether the CLI already renders this detail for a refusal that no longer comes from the flight slot alone is research's question.
- design-system §Surface: cli → Component Patterns 2 (`viola send`: the readback mirror) requires a human-mode refusal to print on stderr as the `[/ ] unable` mirror with `<name>  not-delivered  turn-running`, two-space separated, followed by exactly one `hint:` line. The hint for this detail is fixed: `a turn is running; viola wait <name> first`. Per §Exit-code phraseology it exits 13. Under `--json`, the same section requires the refusal object to be the whole answer, with no hint line.
- design-system §Surface: cli → Component Patterns 2 (T3) says no hint ever names `viola release`. A turn left running with no `turn-ended`, cleared only by a human `release`, must not lead to a hint that tells the driver to release. The `turn-running` hint stays the `viola wait` one.
- design-system §Surface: cli → Component Patterns 4 (Wheel, link and gate verbs) fixes the `viola release` output: `<name>  wheel driver  you have control`. Clearing the running-turn state adds no word, line or field to that output. Whether a `release` with the wheel already at the driver prints anything is architecture's and research's question, not design's.
- design-system §Surface: web-spa → Component Patterns 2 (`<viola-readback>` sources) bases "at most one send is in flight per instance" on "a send during a running turn is refused `turn-running`". It also says `turn-ended` never fills a readback box (§Brand Identity signature element; §Semantic Colors `EventKind::turn-ended`). The running-turn state must not be shown as, or turned into, a readback outcome.
- design-system §Surface: web-spa → Component Patterns 1 (STATUS: `idle` | `busy` | `unknown`) and §Semantic Colors `SessionStatus::*`: the strip's STATUS word comes from `claude agents --json`. This chunk does not redefine it from the wrapper's running-turn state. Adding a WHEEL / STATUS word for "turn running" would be a design amendment, not this chunk's work.

## Patterns to follow
- Each refusal is one fixed-column stderr line: mirror, `unable`, name, reason, detail. Then comes its keyed `hint:` line, and the typed exit code follows. The existing `human-typing` / `manual-pause` and `not-delivered · *` refusals use the same shape (per design-system §Surface: cli → Component Patterns 2 and §Exit-code phraseology).
- The stream split is fixed: refusal and hint on stderr; results on stdout; one JSON document on stdout under `--json` (per design-system §Surface: cli → Streams).
- A hint never quotes the sent text or any upstream text, such as the `<task-notification>` prompt that started a harness turn (per design-system §Surface: cli → Component Patterns 2, "A hint never quotes…").

## Anti-patterns to avoid
- No spinner, progress line, queued-for-later notice or "retrying…" text for a refused send. A refusal is final and is not queued (per design-system §Anti-Patterns → Per-Surface Bans → cli: spinner, and "done/success" without context; §Rejected Defaults: the optimistic "Sending…" spinner).
- No colour on the refusal line. CLI colour is reserved for `DIALOG` (amber) and `stale` rows (dim) only (per design-system §Anti-Patterns → Per-Surface Bans → cli).
- No hint naming `viola release` (per design-system §Surface: cli → Component Patterns 2, T3).

## Contract bindings
- design ↔ architecture §Conventions (the `send` refusal order, exit codes): the CLI words and exit 13 here mirror architecture's typed `not-delivered` / `turn-running` refusal. design-system §Exit-code phraseology defers to architecture.md's typed codes.
- design ↔ security-plan §Error Handling and the NEVER-log floor: the hint and refusal line carry no upstream text, path or pid (design-system §Anti-Patterns → Per-Surface Bans → cli, "NEVER print upstream text, paths, pids…").
- design ↔ test-plan: if the end-to-end witness asserts human-mode output, it asserts the fixed words and hint above. A `--json` witness asserts the refusal object with no hint text.

## Acceptance criteria contributions
- (design) A CLI `viola send` refused because a turn is running, including a turn the driver did not start (harness or human origin), prints on stderr the `[/ ] unable` mirror with `<name>  not-delivered  turn-running`, then exactly `hint: a turn is running; viola wait <name> first`, and exits 13 (per design-system §Surface: cli → Component Patterns 2 / §Exit-code phraseology).
- (design) Under `--json`, the same refusal is one stdout document `{"v":1,"refusal":"not-delivered","detail":"turn-running"}` with no `hint:` text and no SGR bytes (per design-system §Surface: cli → Streams / Colour decision order).
- (design) No hint or refusal line added or changed by this chunk names `viola release` or quotes prompt text (per design-system §Surface: cli → Component Patterns 2, T3 and "A hint never quotes…").
- (design) `viola release <name>` human output stays `<name>  wheel driver  you have control`, with no added word for the cleared turn state (per design-system §Surface: cli → Component Patterns 4).
