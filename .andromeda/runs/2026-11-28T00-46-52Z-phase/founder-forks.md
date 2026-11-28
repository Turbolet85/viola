# Founder forks — «Console scaffold» (`2026-11-28-console-scaffold`, viola-0.2.0, Epoch 6)

## P3 block — written 2026-11-28T01:23:24Z (the overseer's directive, inputs#I1, item 5)

**Window.** One builder window, taken up at `dbb0c40e` (CI ci#46087983798 green 7/7). P1 promoted the chunk (tx 5247),
P2's row read picked 1364 of 5003 rows over the seven masters (no master at `picked 0`; 3 picker re-spawns, 1 dead
draw, `tests-pick-p1.d3`, its pack read on its two standing draws), seven history files, P3 research put
(`doc:2026-11-28-console-scaffold/research`, rev 1; lists parsed, 14 items). No live start, window, compositor call,
real `claude`, live script arm or mutation run; nothing of `~/.claude*` or `~/.viola` read.

### Measurements (each a number or a closed word, its source beside it)
1. **Terminal surface in the tree.** Raw mode: `viola_pty::HostTerminal` (libc `cfmakeraw` on fd 0, restored on drop,
   `crates/viola-pty/src/lib.rs:226-254`). Size: `viola_pty::host_size` (TIOCGWINSZ on stdout, `lib.rs:261`), polled —
   no SIGWINCH handler (`pump.rs:553`). Screen reader: vt100 `=0.16.2`, used only for the wrapped claude screen.
   **Lacks for a console:** a frame writer (alternate screen, cursor placement, clear), a key decoder, a quit path
   restoring the terminal, its role. Nothing in the tree draws.
2. **Redraw rows** — see «Reached rows» below; four reached, three already reconciled, three not reached.
3. **Draw / key candidates** (measured 2026-11-28 against viola's `Cargo.lock`, viola's advisory policy offline on
   the 2026-11-24 DB, `cargo tree -i cc`):
   | candidate | new crates | licences vs `deny.toml` | highest MSRV (ws 1.96) | advisories | `cc` |
   |---|---|---|---|---|---|
   | none — hand-written, over `HostTerminal` | 0 | — | — | — | no |
   | `termion` 4.0.6 | 2 (`termion` MIT, `numtoa`) | inside | 1.65 (libc) | ok | no |
   | `crossterm` 0.29.0 | 7 (+ `rustix`, `mio`, `signal-hook` in its graph) | inside | 1.85 | ok | no |
   | `ratatui` 0.30.2, crossterm backend only | 40 | inside | 1.88 | ok | no |
   | `ratatui` 0.30.2 default | 89 | 3 OUTSIDE (Apache-2.0 WITH LLVM-exception, WTFPL, Unicode-DFS-2016) | 1.89 | — | — |
4. **State, never the screen.** The pattern stands: `tests/tui_passthrough.rs` drives an outer portable-pty and judges by
   records (`test-plan · decision/tui-verdicts-from-records`). The console's state read = its own codes-only role-file
   lines (a frame verdict with cols × rows, a key class, its `process-exit`) and its exit status. Fake side: a
   test-owned home; the fake agent on PATH only where a board read is made.
5. **Its own diagnostics role.** There is NO `ui` role: the roles are `run · hook · mcp · cli` (`ObsProcess`,
   `crates/viola-core/src/obs.rs:81-99`), pinned by 11 master rows, one test (`obs.rs:196`) and two schemas
   (`schemas/diag-line.v1.json:14`, `diag-detail.v1.json:16`). The harness `logs` verb reads every
   `diagnostics/*.ndjson` (`crates/viola-e2e/src/harness/logs.rs`) and G2 globs the same: a new role file needs no
   change in either.
6. **chrono (`cry-h6vzhlce`).** No safe upgrade: crates.io's newest chrono is 0.4.45, the pin (read
   2026-11-28T00:48:36Z); RustSec `RUSTSEC-2026-0543` `patched = []`, «all versions are affected». An ignore that
   matches nothing is `warning[advisory-not-detected]`, `advisories ok`, exit 0 (cargo-deny 0.20.2, measured on a scratch
   crate); while viola stays on 0.4.45 the ignore keeps matching, so a future patched release would raise nothing by
   itself. chrono reaches the graph through rmcp 3.4.1 and schemars 1.2.2 as well as viola and viola-state
   (`cargo tree -i chrono`): dropping viola's own use would not drop the advisory.
7. **`cry-boujwuyr`.** M1: `framing_turns` (`src/cmd/verify/typed/turns.rs:152`, 2 callers) repeats guard → count →
   paste → wait → `box_wait` five times by hand; `send_through` (`src/run/send.rs:90`, 2 callers). M3: the relied set
   IS `LedgerRow::ALL` (`ledger.rs:184`), yet 21 test sites hard-code `29` (`src/cmd/verify.rs:833-908`,
   `ledger/test_cases_stamps.rs:82,205,918,1108`; re-derived by `grep -rn "\b29\b|twenty-nine"`).

### Reached rows (terms searched: `redraw` · `console|tui|deferred to 0.2.0` · `run, hook, mcp` · the help verb rows · `terminal size|resize|width source` · `(only|never) … stdin|keystroke|raw mode` · `licence allow|one OS-binding`)
- **Reached by the redraw / interactive input:** `a11y-plan · decision/static-linear-output` («viola's own output is
  static and linear, with no spinner or redraw»); `a11y-plan · decision/cli-not-keyboard-interactive` («The CLI has no
  prompt, TUI mode or pager; its input comes only from stdin or --file»); `a11y-plan · decision/cli-no-interactive-input`
  («no interactive prompt, pager or redraw»); `obs-plan · absence/interactive-terminal-interface` («An interactive
  terminal interface: prompt, TUI or pager»).
- **Read, already reconciled:** `design-system · decision/fleet-console-not-verb-output`; `layout-templates ·
  decision/no-prompt-or-pager`; `layout-templates · decision/shared-captions-fixed` (the view's, not this entry's).
- **Read, not reached** (scoped to a verb's output or the CLI's tokens, which the console is not):
  `a11y-plan · decision/no-spinners`, `decision/linear-static-cli-output`; `design-system ·
  decision/cli-tty-static-expression`, `decision/no-spinners-in-verb-output`; `layout-templates ·
  contract/cli-token-rendering/motion-tokens`, `decision/send-lines-appended-never-redrawn`.
- **Reached by the role list:** `architecture · decision/codes-only-process-logs`; `obs-plan · contract/diag-line/process`,
  `contract/log-line/process`, `contract/process-log/process`, `contract/process-log-line/process`,
  `decision/structured-json-process-logs`, `decision/zero-unlogged-panics`, `contract/diagnostics-line/role-file-line-size`;
  `test-plan · decision/structured-json-process-logs`, `decision/process-logs-via-tracing-subscriber`,
  `contract/log-line/process`.
- **Reached by the verb list:** `layout-templates · contract/viola-help/verb-groups` («Five rows … served verbs only»),
  `contract/viola-help/setup-row`.
- **Reached by a dependency (only on a fork arm that adds one):** `architecture · decision/licence-allowlist`,
  `decision/libc-only-os-binding` (crossterm's graph takes `rustix`), `decision/workspace-dependency-pins`,
  `decision/no-c-building-crates`; `security-plan · decision/licence-allow-list`; `design-system ·
  decision/hand-written-sgr-module` («adds no dependency»).
- **Read, not reached** (the run passthrough's own): `architecture · decision/human-key-never-held-behind-resize`;
  `a11y-plan · decision/human-keystrokes-never-blocked`, `decision/wheel-ignores-terminal-events`; `security-plan ·
  decision/stdin-closed-non-editing-list`; `design-system · contract/human-tables/width-source` (the board's; the
  console reads the same `host_size`).
- **Playbook escalate rows read:** `rul-r74hshif`, `rul-p2zl2kgl`, `rul-o2tr55aj`, `rul-bznresq5` — the redraw fork's
  option (a new input class, keystrokes read interactively, and output that redraws) is the «Boundary widening» class.

### His forks — each with the arm taken PROVISIONAL meanwhile
**F1 — what draws and reads the terminal** (a technology choice; the 0.1.0 masters left none).
1. **No new dependency (Recommended) — PROVISIONAL.** A small hand-written frame writer and key decoder in the `viola`
   bin over `HostTerminal` and `host_size`. Cost: 0 crates; roughly the size of the SGR module again, owned by us;
   keeps `decision/hand-written-sgr-module` and `decision/libc-only-os-binding` whole. Its decoder reads only the
   scaffold's keys.
2. `termion` 4.0.6 — +2 crates, libc-only, MIT. Cost: a dependency for what the bin already half-holds.
3. `crossterm` 0.29.0 — +7 crates; reaches `decision/libc-only-os-binding` through `rustix`.
4. `ratatui` (crossterm backend only) — +40 crates, MSRV 1.88; widgets the view may want later.
**F2 — the four reached redraw / interactive-input rows** (a boundary widening: rows `rul-r74hshif`/`rul-p2zl2kgl`).
1. **An exception NAMED for the console's own surface on each reached row, the rule never generalised
   (Recommended) — PROVISIONAL.** Each row gains `except: the terminal fleet console (`viola console`), its own
   surface`; the obs absence leaves for a console decision row. Cost: four row amendments; the wrap halts on them as
   his.
2. One console-wide rule superseding the four. Cost: a generalised rule; rejected by the directive's conservative
   reading.
**F3 — what starts the console, and its name.**
1. **A new verb `viola console`, in the `board:` help row beside `list, focus` (Recommended) — PROVISIONAL.** Cost: the
   help's verb-group rows amended; one more `Command` variant.
2. A flag on `viola list` (`--live`). Cost: list's static-output rows would then be reached too.
3. Another name or group — his word.
**F4 — `cry-h6vzhlce`, chrono RUSTSEC-2026-0543 (no safe upgrade exists).**
1. **Keep the ignore one more epoch, to the Epoch 7 boundary (Recommended) — PROVISIONAL.** Cost: none; `deny.toml`'s
   reason line unchanged («re-read at each epoch boundary»).
2. Remove viola's and viola-state's own use of chrono (to `time` or by hand). Cost: a real refactor of the timestamp
   code; rmcp and schemars still bring chrono, so the advisory and its ignore stay.
3. Drop the ignore. Cost: `cargo deny check` red (`unmaintained = "all"`) until rmcp and schemars leave chrono.
**F5 — one window or a split.** The scaffold (a verb, a frame and key module, a role, two schemas, one outer-PTY test
binary) and `cry-boujwuyr` (M1 one function, M3 21 literal sites) fit one implement window by this read; the plan puts
M1 and M3 last, so a stop after the scaffold leaves them whole for a later window.
1. **One chunk, the carries last (Recommended) — PROVISIONAL.**
2. Split M1/M3 to a new entry «Epoch 6 cleanup» after «Console actions». Cost: a route insertion.

### Technical, asked of the overseer
- **Q1** — the founder's quarter of the screen in cells (cols × rows): no master row or fixture holds it. Meanwhile the
  frame's stays-whole floor is 80×24 (`Size::DEFAULT`), PROVISIONAL.
- **Q2** — M3's «the stamped-set table held once»: no second hand-kept table of stamped rows was found beside
  `LedgerRow::ALL`; which table is meant? Meanwhile M3 is the 21 literals only.
- **Q3** — the role's word: `console` (its file `console.ndjson`, home-level like `mcp`), PROVISIONAL.

### Builds whole on those arms?
**Yes.** Every arm taken PROVISIONAL (F1 no dependency, F2 named exceptions, F3 `viola console`, F4 keep, F5 one
chunk, Q1 80×24, Q3 `console`) builds the whole entry; the phase goes on to P4 without waiting.
