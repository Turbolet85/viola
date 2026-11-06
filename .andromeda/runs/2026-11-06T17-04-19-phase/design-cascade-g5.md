@carried n=1 label=design-system/surface fact=fct-5r2kfdmj
why: the founder's owed word on the board's read rule (scope item 7) rules on this as-built rule, and the master text must state whichever he rules, while v1-36's «link set on the board» is deferred or re-worded against this board.
quote: A wrapped row joined to a `claude agents --json` row (by child pid or agent session id) reads STATUS `idle` or `busy`, else `unknown`.
@carried n=1 run=2 label=design-system fact=fct-mfvo2or6
quote: As built: a wrapped row joined to a `claude agents --json` row reads STATUS `idle` or `busy`, else `unknown`; unwrapped sessions follow the `-- UNWRAPPED - READ-ONLY --` line;
@carried n=1 run=3 label=design-system fact=fct-glxbi6ql
quote: (an existing pair prints `link    overseer -> builder  already linked`)
@carried n=1 run=4 label=design-system fact=fct-ybxjcc2w
quote: `viola release builder --budget` → `builder  budget gate lifted until <time>  wheel human` when a pause stood, else `builder  budget gate open  wheel human`, one unstyled stdout line.
@carried n=1 run=5 label=design-system fact=fct-7z5o76y5
quote: Linux terminals (the core on any current distribution, CI on `ubuntu-latest`).
@carried n=1 run=6 label=design-system fact=fct-3n5f23up
quote: clap 4.6.7 (derive) built without its `color` feature, Rust stable: clap's help and usage output is plain in every mode, and no dependency reads a colour or terminal variable, so every styled byte of human output is viola's own SGR module's.
@carried n=1 run=7 label=design-system fact=fct-qu7ywsq4
quote: Styling is a small hand-written SGR module in the `viola` bin. It adds no dependency.
@carried n=2 label=design-system/surface fact=fct-kn56sdzi
why: the `portable-pty` move reaches crates/viola-pty whole, so `viola_pty::host_size()` must keep reading stdout's own terminal size by `TIOCGWINSZ`, never `COLUMNS`, and `viola run` must keep its silent passthrough.
quote: The width is stdout's own terminal size, read by `viola_pty::host_size()` (a `TIOCGWINSZ` ioctl); `COLUMNS` is not read, and a piped stdout has no width
@carried n=2 run=2 label=design-system fact=fct-rhkxdzp7
quote: Where stdout's own terminal width is known (`COLUMNS` is never read, a piped stdout has none), human tables truncate only the NAME of unwrapped rows (`...`); the CLI has no tape.
@carried n=2 run=3 label=design-system fact=fct-w4pl2fni
quote: **`run`:** prints nothing once the child starts.
@carried n=2 run=4 label=design-system fact=fct-jphnedpy
quote: `viola run`'s terminal belongs to the child's screen; a passed `viola revive` is the same passthrough.
@carried n=3 label=design-system/surface fact=fct-nmng4ymy
why: «the board's read rule» the founder's owed word rules on may read as this strict-modes read rule, a fork the scope leaves open, so the plan must bring both readings and the master must agree with his word.
quote: The board's strict-modes scan holds the home to the read rule and `instances/` to the write rule, then each instance's set: a home another user may read refuses the whole board with the home-state pair, with or without an instance; the order and the no-open rule hold.
@carried n=3 run=2 label=design-system/surface fact=fct-t547mw3y
quote: a failed strict-modes check of the home, `instances/` or any one instance refuses the whole board, nothing from state printed, in the fixed `unable:` / `hint:` form of the start refusals (pattern 2): the home → `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`
@carried n=4 label=design-system/surface fact=fct-bmfbeece
why: the founder's owed word on the socket-directory pair's wording (scope item 7) closes this OPEN, and the pair's lines must then read his wording at every place it prints.
quote: OPEN — the socket-directory pair's wording is a planning form · owner: the Epoch 7 boundary batch
@carried n=4 run=2 label=design-system/surface fact=fct-pbvocw2a
quote: `unable: the socket directory for <name> cannot be trusted` (before any snapshot, child or bind) → `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`
@carried n=4 run=3 label=design-system/surface fact=fct-fjfzfyz5
quote: A refused socket directory at `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` or `unlink` exits 21 before any frame: `unable  <name>  instance-unreachable` (`send` in its mirror's form), then the hint of cause `socket-dir`, no strict-modes pair; under `--json` the `instance-unreachable` document with `detail` `socket-dir-refused`, stderr empty.
@carried n=5 label=design-system/surface fact=fct-pi5kme4c
why: item 6's folds or ratifications of the error enums must leave every refusal's stderr form, its typed exit code and its `--json` document exactly as stated, with no stack trace.
quote: stderr: context lines (`waiting: <name>`, the issue line of a send), refusals (`unable  <name>  <reason>  <detail>`, two-space separated;
@carried n=5 run=2 label=design-system fact=fct-tj4jq2zv
quote: Each typed exit code has its human stderr form and, where it has one, its `--json` document:
@carried n=5 run=3 label=design-system fact=fct-zumogwyu
quote: **Stack traces** never print. Errors are fixed messages, and full detail goes only to `instances/<name>/diagnostics/`.
@carried n=6 label=design-system/surface fact=fct-4dwulxx5
why: a fold of `ServeError` beside `McpError` reaches the MCP client, which must keep screening a peer's refusal and detail through `is_code_word` as the CLI client does.
quote: Code word: 1 to 64 bytes of lower-case letters, digits and hyphens, a lower-case letter first (`viola_core::is_code_word`); both clients, `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`), screen a peer's `refusal` and `detail` through it.
@carried n=6 run=2 label=design-system/surface fact=fct-y6uokfat
quote: A peer's `refusal` that is no code word reads `unknown`; a string `detail` that is no code word reads as no detail, null under `--json`; a non-string `detail` is no known reply.
@carried n=6 run=3 label=design-system/surface fact=fct-kr6nq63b
quote: A code word outside the closed refusal list stays visible, by design, in the document and in `refused: <word>`, and exits 14 as `unknown` does (`reason_of`).
@carried n=7 label=design-system/surface fact=fct-ypei347w
why: «every gate green on Linux» re-reads this ubuntu-latest witness after the pty dependency moves, so it must pass at the wrap with exactly this escape sequence at the 256-colour depth its `TERM` selects.
quote: The stale row's colour layer is witnessed end to end on `ubuntu-latest`, the one CI runner, by `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (`tests/cross_list_rows.rs`, ungated), passing as measured at ci#42030025050: `viola list` under an outer PTY with `TERM=xterm-256color` prints a stale row whose escape sequences are exactly DIM, BOLD, RESET, DIM, RESET
@carried n=7 run=2 label=design-system/surface fact=fct-qgesfouf
quote: NAME on a stale row is dim and bold, and every span inside the row opens dim again after its reset.
@carried n=7 run=3 label=design-system/surface fact=fct-23ap43hv
quote: | callsign | `\x1b[1m` (bold) | `\x1b[1m` | `\x1b[1m` | NAME column values |
@carried n=7 run=4 label=design-system/surface fact=fct-qe6n467m
quote: | reset | `\x1b[0m` | `\x1b[0m` | `\x1b[0m` | After every styled span |
@carried n=7 run=5 label=design-system fact=fct-vzhczpnh
quote: 256-colour if `TERM` contains `256color`;
@carried n=7 run=6 label=design-system fact=fct-w37zqv3v
quote: **Colour decision order** (the first match wins, and results in no colour, no glyph and no SGR;
@binding n=1
**cli-list (surface) ↔ the founder's Epoch 7 batch and the verification matrix**: the board's read rule the founder rules on is either the agents-join rule (design-system/surface fct-5r2kfdmj) or the strict-modes read rule (design-system/surface fct-nmng4ymy); the plan brings the fork, and the master states his word; v1-36's «link set on the board» is deferred or re-worded against a board that carries no link set (design-system fct-mfvo2or6), v1-13's clause against link's and the budget release's fixed lines (design-system fct-glxbi6ql) (design-system fct-ybxjcc2w).
@binding n=2
**surface (`viola_pty::host_size()`, `run` passthrough) ↔ the dependency move (`Cargo.toml` `portable-pty` pin, crates/viola-pty)**: the release `portable-pty` moves to must keep the width read by `TIOCGWINSZ` on stdout with `COLUMNS` unread (design-system/surface fct-kn56sdzi) and `viola run` printing nothing once the child starts (design-system fct-w4pl2fni), and the outer-PTY stale-row witness green on ubuntu-latest (design-system/surface fct-ypei347w).
@binding n=3
**cli-send socket-directory pair (surface) ↔ obs-plan's `socket-dir-refused` detail code and the founder's word**: the founder's wording replaces the planning form (design-system/surface fct-bmfbeece) at `run`'s and `revive`'s start pair (design-system/surface fct-pbvocw2a) and at the eight verbs' exit-21 hint (design-system/surface fct-fjfzfyz5), the `--json` detail word `socket-dir-refused` unchanged.
@binding n=4
**cli-output and exit-code phraseology (surface) ↔ architecture's one-enum-per-crate rule (item 6)**: each fold or ratification of `ServeError`, `McpError`, `StampsError`, `StateError`, `PinError`, `StampsReadError` and `Refused` keeps every typed exit code, its stderr form and `--json` document (design-system/surface fct-pi5kme4c) (design-system fct-tj4jq2zv), the code-word screen in both clients (design-system/surface fct-4dwulxx5), and no stack trace (design-system fct-zumogwyu).
@binding n=5
**colour decision order (surface) ↔ the plain-output tests (`tests/cli_output_plain.rs` lines 104 and 116, `plain_drive` in `src/cmd/mod.rs`)**: removing those Windows remnants leaves every plain-mode output with no colour, glyph or SGR, the first match of the decision order winning (design-system fct-w37zqv3v), and the depth chosen by `TERM` unchanged (design-system fct-vzhczpnh).
