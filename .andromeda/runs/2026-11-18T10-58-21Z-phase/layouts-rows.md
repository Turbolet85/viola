# layout-templates · the rows that bind 2026-11-18-session-folder-trusted-by-viola — picked 10 of 222 (6 by every draw) · rule any · pin 2658

## surface-cli · ## Surface: cli
contract/cli-verbs/revive · CLI verbs · viola revive <name> — Resumes a dead instance in place (its newest logged session, in the recorded directory) and is passthrough like viola run.
contract/cli-verbs/run · CLI verbs · viola run <name> -- claude — Passthrough; prints nothing while the child runs.
contract/run-revive-start-json/internal-error · viola revive --json start refusals shared with run · internal-error — A program not found or an error of the start itself prints one exit-1 document with this code, stderr empty; a child-pump error prints no document, the child having held the terminal.
contract/viola-revive/preflight-order · viola revive start · preflight order — The start reads its preflight in a fixed order: the holder, the logged session, session-live, then the recorded directory.
contract/viola-revive/start-output · viola revive · passed start output — A `viola revive` start that passes its preflight prints nothing of its own while the resumed child holds the terminal, as `viola run`.
contract/viola-run/start-output · viola run start · passed start output — `viola run` prints nothing of its own; the terminal belongs to the wrapped claude TUI until it exits.
decision/hint-for-every-cause · A hint for every refusal cause — Directly under each refusal is `hint: <one plain instruction>`, keyed by reason or reason and detail, every cause with its own hint. · why: Exit 21 and exit 1 each have several causes, so an agent can tell the causes apart from the last stderr line. · except: No hint for the `unknown` refusal, for `wrapper fault` or for `internal error`.
decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
contract/cli-catch-site/internal-error · CLI verbs outside a revive start · internal error — A failed or panicked verb prints exactly `error: internal error` once, from the one catch site, with no hint; with `--json` before a `--` it prints `{"v":1,"error":"internal-error","detail":null}` on stdout instead, stderr empty.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unverified-cli`.

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
