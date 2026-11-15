# layout-templates · the rows that bind 2026-11-14-status-line-interception-removed — picked 20 of 221 (9 by every draw) · rule any · pin 1206

## surface-cli · ## Surface: cli
contract/cli-verbs/help · CLI verbs · viola --help — The grouped verb table with the served verbs only: the board, traffic, wheel, handoff and setup rows, `handoff:` holding link, unlink and `setup:` holding plugin install.
contract/viola-help/verb-groups · viola --help output · verb group rows — Five rows `board:    list`, `traffic:  send, wait, last, answer`, `wheel:    pause, release`, `handoff:  link, unlink`, `setup:    run, revive, verify, plugin install`, served verbs only, plain ASCII, then a blank line, `Options:` and clap's options; no about line and no clap `Commands:` list.
contract/viola-list-json/document · viola list --json · document — One JSON document on stdout with no header line, no colour and no glyph; its item and skipped shapes are the list payload contract.
contract/viola-verify-output/exit-0 · viola verify output · exit 0 — Exit 0 when every relied row passes.
contract/viola-verify-output/exit-1 · viola verify output · exit 1 — Exit 1 on a failing row, a refusal (the unable: / hint: pairs) or a stop.
contract/viola-verify-output/failing-row · viola verify output · failing row — A run with a failing row exits 1 and still writes the stamp.
contract/viola-verify-output/step-line · viola verify output · step line — One stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|fail`, in ledger order, MM being the relied count; each appended once, with no progress bar and no redraw.
contract/viola-verify-output/summary · viola verify output · summary — `stamped <ver>  <n> pass  <m> fail` is the last stdout line; `fail` is never coloured.
contract/viola-verify/summary · viola verify output · summary line — `stamped <version>  <n> pass  <n> fail` closes a completed run; a run stopped before its rows are checked prints no summary.
decision/board-planning-forms-wording · Board planning forms as the ratified wording — The board's planning forms (bare skip counts, the empty board's three lines) are its wording. · why: confirmed as built by the founder, superseding the operator's word
decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output.
decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
contract/bay-line/cell-order · the BAY line of viola list · cell order — `BAY`, then `skipped a - b - c`, separated by the two-space gutter.
contract/cli-verbs/list · CLI verbs · viola list — The board: every session as one strip row, with the BAY context header.
contract/cli-verbs/revive · CLI verbs · viola revive <name> — Resumes a dead instance in place (its newest logged session, in the recorded directory) and is passthrough like viola run.
contract/cli-verbs/run · CLI verbs · viola run <name> -- claude — Passthrough; prints nothing while the child runs.
contract/viola-help/usage-line · viola --help output · usage line — `viola --help` and `viola help` print exactly `Usage: viola [--home <dir>] <verb> [args]` as the first line, then a blank line.
contract/viola-run/start-output · viola run start · passed start output — `viola run` prints nothing of its own; the terminal belongs to the wrapped claude TUI until it exits.
contract/viola-verb-help/usage · viola <verb> --help output · per-verb usage — `viola <verb> --help` gives clap's per-verb usage unchanged, its usage line naming the program by its executable.
contract/viola-verify-output/record · viola verify output · --record — --record adds no human line.

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
