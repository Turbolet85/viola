# Task — design-system · 2026-10-26-home-and-code-bearing-file-integrity

## The session
Built the home's strict-modes check before every read or open at every entry point: a read rule (Unix: no group or other bit on the home, `instances/<name>/` and `diagnostics/`; Windows: no read grant to another SID), a home-tree check (`check_home_tree`), a diagnostics check, and owner/DACL checks on the pinned copy, `plugin/<key>/` and the Windows `conpty/` companions.
Added refusals: the six frame verbs `send`, `wait`, `last`, `answer`, `pause` and `release` run the instance check first through a shared helper (`refuse_untrusted`, which `link` / `unlink` now use too). A refusal exits 21 with `link`'s instance-state `unable:` / `hint:` pair, or under `--json` the `instance-unreachable` document. It sends no frame and writes no line.
Added refusals: `viola run` and `revive`'s start check the home tree before any log (exit 1, the home pair or the instance pair). `viola verify` checks the home, `ledger/`, `bin/` and `plugin/` before its log (exit 1, the home pair, no step line, no summary). The `viola hook` event and dialog paths and the `viola mcp` frame tools refuse silently (hook: exit 0, nothing printed; mcp: `instance-unreachable`, `detail: null`).
Retired the eight dated strict-modes gaps. Moved the remaining snapshot readers onto the replay, so `read_snapshot` has no product callers left. Added `strict-modes-failed` to the `sideload_fallback` words. No crate, dependency, config key, env var, flag or verb was added.
Measured on `windows-2025`: CI green 15/15 on all three OSes. A protected owner-only home DACL reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`. A home made with `create_dir_all` before viola is refused `Writable`.

## Reached files
- crates/viola-state/src/strict.rs
- crates/viola-state/src/stamps.rs
- crates/viola-state/src/liveness.rs
- crates/viola-state/src/pin.rs
- crates/viola-mcp/src/tools.rs
- src/cmd/client.rs
- src/cmd/link.rs
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/hook.rs
- src/cmd/hook/statusline.rs
- src/cmd/run.rs
- src/cmd/revive.rs
- src/cmd/verify.rs
- src/obs.rs
- schemas/diag-line.v1.json
- tests/security_negatives_strict_modes.rs
- tests/chaos_snapshot_replay.rs
- crates/viola-mcp/tests/handlers.rs
- tests/hook_fail_open.rs
- tests/hook_statusline.rs
- tests/run_cli.rs
- tests/cli_verify.rs
- tests/conpty_sideload.rs
- tests/cli_controls_not_disableable.rs
- tests/support/home.rs
- tests/cli_revive.rs
- tests/cli_wait_last.rs
- tests/contract_diag_schema.rs
- tests/cli_machine_contract.rs
- tests/tui_env_strip.rs
- viola-0.1.0/verification-matrix.json
- viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/scope-record.md
- viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/evidence/remove-the-guard.md
- viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/evidence/operator-pass.md

## Touches
- Surface: cli, Streams (cli-output): the fixed `unable:` / `hint:` form is listed only for the exit-1 refusals of `run`, `revive` and `verify` and the exit-21 strict-modes refusals of `list`, `link` and `unlink`. The six frame verbs `send`, `wait`, `last`, `answer`, `pause` and `release` now join the exit-21 list.
- Exit-code phraseology (cli-exit-codes), the 21 row: exit 21 now also covers a refused strict-modes check of the six frame verbs. It prints the instance-state pair naming the instance, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty.
- Exit-code phraseology (cli-exit-codes), the `link` / `unlink` paragraph: this paragraph names the verbs that print the instance-state pair, so the six frame verbs join it. All eight verbs now share one helper, and `link`'s behaviour is unchanged.
- Exit-code phraseology (cli-exit-codes), the 1 row: the fixed-message `unable:` refusals now include `run`'s and `revive`'s start refusing a widened home tree, and `verify` refusing its home tree, each before any log.
- `viola verify` (cli-verify-run): a refused home-tree check (the home, `ledger/`, `bin/`, `plugin/`) prints only the home-state pair, with no step line and no summary, exit 1. A stamps write refused by strict-modes prints the same pair. The standing text names only the Ctrl-C / TERM stop as printing no step line.
- `viola run` (cli-verify-run): the start refusals gain the strict-modes refusals. A home-level refusal prints the home pair and an instance-level refusal prints the instance pair, both exit 1, before any log, with no child. The per-cause refusal lines and hints change with them.
- `viola revive` (cli-verify-run): its start now checks the whole home tree (`ledger/`, `bin/`, `plugin/`, the pinned copy) behind the same strict-modes refusal, and under `--json` it keeps `strict-modes-failed`.
- `viola send` (cli-send): send has a new refusal that is not drawn as the readback mirror. It is the fixed instance-state `unable:` / `hint:` pair, exit 21, and comes after the text is read but before any frame. The send hint paragraph lists the per-cause lines.
- Wait, link and gate verbs (cli-verbs): `wait`, `last`, `answer`, `pause` and `release` each refuse a widened instance first, with the instance pair at exit 21, writing no frame and no line.
- `viola list` (cli-list), the strict-modes board refusal: `board.rs` is unchanged, but the per-instance check now holds the home to the read rule. A 0750 home therefore passes an empty board, and on a board with an instance it refuses naming that instance. Which pair the whole-board refusal prints now depends on whether an instance exists.
- The pattern counts from the report reach design-system. `instance-pair` has 3 hits, at surface.md :37, :67 and :122, each listing the verbs that print the instance pair. `home-pair` has 4 hits, among them surface.md :65 («strict-modes at…») and :122, which name where the home pair stands.
- Unstattable home: on Unix, a home under a regular file now gets a strict-modes refusal (`verify`: the home pair, exit 1; frame verbs: the instance pair, exit 21) and no longer reaches the `cli` catch-site. On Windows such a path stats `NotFound`, so the old behaviour stands there.
- Error and hint rules: the new refusals print fixed pairs with no path, pid or upstream text. Re-read the cli bans and the hint rule (no hint names `viola release`) against the new pairs.
- Diagnostics: `diagnostics/` and its role and detail files now pass a symlink and mode check before opening. A refused open drops the detail line (the writer ends at a sink). This reaches the surface note that full detail goes only to `instances/<name>/diagnostics/`.

## New facts
- F1 · surface · `viola send`, `wait`, `last`, `answer`, `pause` and `release` run the instance strict-modes check first. A refusal exits 21 with the instance-state `unable:` / `hint:` pair naming the instance, or under `--json` the `instance-unreachable` document with stderr empty. No frame or line is written. This is the same form as `link` / `unlink`.
- F2 · surface · `viola verify` checks the home, `ledger/`, `bin/` and `plugin/` before its log. A refusal prints the home-state pair with no step line and no summary, exit 1. A stamps update refused by strict-modes prints the same pair.
- F3 · surface · `viola run` and `revive`'s start check the home tree before any log: the home, the instance set, `ledger/`, `bin/`, `plugin/`, the pinned copy and its `plugin/<key>/`. A home-level refusal prints the home-state pair and an instance-level refusal prints the instance-state pair, both exit 1, with no child.
- F4 · surface · `viola list` on a home that group or other may read (0750) passes when there is no instance. With an instance, it refuses naming that instance. The board's refusal pair therefore depends on whether an instance exists. This inconsistency is owed to a CARRY on «Unix endpoint and home hardening».
- F5 · surface · On Unix, a home under a regular file is a strict-modes refusal (`verify`: the home pair, exit 1; the frame verbs: the instance pair, exit 21). On Windows such a path stats `NotFound` and keeps its earlier outcome.

## Untouched
- Brand Identity, Color Palette, Typography, Spacing, Depth Strategy, Border Radius, Motion, Iconography and the Token Test: no visual value, token, glyph or 0.2.0 frontend input moved.
- The colour decision order and depth selection (cli-tokens): the new refusals print unstyled stderr in the existing fixed form, and the SGR module did not change.
- `viola list`'s table: its columns, BAY line, ordering, control escaping and empty-board line are unchanged, because the board read and the list command were not edited.
- `viola plugin install`'s output lines and its exit-21 home-pair refusal: the plugin command was not changed.
- Navigation pattern and the `viola --help` rows: no verb was added, renamed or hidden.
- Width and ASCII-only rules: no new output exceeds or changes them.
- The `sideload_fallback` word, the diag-line schema and the `state-recovered` span placement: these are obs vocabulary, not design-system content.
