# layout-templates · the rows that bind 2026-11-27-epoch-5-cleanup-2 — picked 31 of 238 (10 by every draw) · rule any · pin 5179

## surface-cli · ## Surface: cli
decision/board-vs-strip · Board versus strip verbs — `viola list` shows the whole bay, `viola up` re-opens the whole fleet or the instances it names, `verify` and `plugin install` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`.
contract/cli-exit-codes/21 · CLI exit codes · 21 — `unable  instance-unreachable`; for `viola list`, `viola plugin install`, the board refusal of `viola up` and the strict-modes refusal of the frame verbs and `viola focus` the fixed state pair instead.
contract/cli-verbs/adopt · CLI verbs · viola adopt <SESSION_ID> --name <NAME> — Resumes a plain claude session by its session id under viola with the given name and parent, in the current folder, with no window, logging as process run.
contract/cli-verbs/open · CLI verbs · viola open <name> — Starts a wrapped claude session in a new foot terminal window beside its parent; its flags are a closed allowlist (--parent, --role overseer|builder|worker, --cwd, --workspace, --model, --effort, --json), the only words allowed after `--` are `claude` alone, and no shell runs.
contract/cli-verbs/revive · CLI verbs · viola revive <name> — Resumes a dead instance in place (its newest logged session, in the recorded directory, which also keys the trust record its start writes) and is passthrough like viola run.
contract/cli-verbs/run · CLI verbs · viola run <name> -- claude — Passthrough; on a verified CLI the start writes the claude CLI's trust record for an untrusted session folder, flag only, so no trust dialog stands and no key is typed; prints nothing while the child runs, nor at a switch's respawn of the child.
contract/cli-verbs/switch · CLI verbs · viola switch <name> [--model <MODEL>] [--effort <EFFORT>] [--json] — Switches a running wrapped session to a new model and effort in place, resuming the same session id; at least one of --model and --effort is required, else a usage error, exit 2; it exits 0, 10, 12, 13, 14, 20 or 21 and writes nothing to the user's own settings.
contract/cli-verbs/up · CLI verbs · viola up [<NAME>...] [--dry-run] [--json] — Re-opens the fleet after a reboot: each candidate instance is revived in its own terminal at its recorded working directory, parents before children, and instances already live are skipped; it is CLI-only, with no MCP tool and no channel method.
contract/viola-revive/no-window · viola revive · window placement — viola revive places no window unless given its hidden `--origin <cli|agent>`, under which the revived window moves to the workspace a real snapshot recorded, or reads unplaced when none was recorded.
contract/viola-run/start-output · viola run start · passed start output — Before the child, only a key-on start's automatic verify line and verify's own output reach the terminal (discarded under `revive --json`); the trust step adds no refusal, exit code, hint, `--json` code or terminal byte; then the terminal belongs to the wrapped claude TUI until it exits.
decision/hint-for-every-cause · A hint for every refusal cause — Directly under each refusal is `hint: <one plain instruction>`, keyed by reason or reason and detail, every cause with its own hint. · why: Exit 21 and exit 1 each have several causes, so an agent can tell the causes apart from the last stderr line. · except: No hint for the `unknown` refusal, for `wrapper fault` or for `internal error`.
decision/hint-last-on-stderr · Hint is the last stderr line — On a refusal, the `hint:` line is the last line on stderr.
decision/hints-name-next-verb · Hints name the next verb — `hint:` lines point to the next verb by name, as the CLI's breadcrumbs.
decision/hints-never-quote-or-locate · Hints never quote text or name paths — A hint never quotes the sent text or any upstream text, and never names a path or a pid.
decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output.
decision/json-on-every-data-verb · --json on every data verb — `--json` is the machine view of every data verb, `viola revive` included in both its refusal document and `--list`.
decision/no-hint-names-release · No hint names viola release — Every hint is distinct and no hint names `viola release`; `release` is a human verb reached through `--help`. · why: The refusals that carry hints reach drivers, and a driver's `release` is refused.
decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
decision/shared-refusal-word-column · Shared refusal word column — The word half of the struck box (`unable` plus `reason  detail`) is the first-word column of every other verb's refusal line, while the struck glyph itself belongs to viola send alone.
decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
contract/cli-exit-codes/10 · CLI exit codes · 10 — `human-typing`.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unverified-cli`.
contract/cli-exit-codes/13 · CLI exit codes · 13 — `not-delivered`.
contract/cli-exit-codes/14 · CLI exit codes · 14 — `unknown`; an older wrapper's `budget-paused` reply also reads as `unknown`.
contract/cli-exit-codes/20 · CLI exit codes · 20 — Wrapper fault, `error: wrapper fault  <code>`.
contract/cli-exit-codes/json-form · CLI exit codes · --json form — Under `--json` the same outcome is one document on stdout, with no stderr line and no hint text.
contract/cli-json-mode/one-document · CLI --json mode · one document — Under --json each verb prints exactly one document on stdout (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and nothing on stderr.
contract/cli-refusal-line/form · CLI refusal line · line form — `unable  <name>  <reason>  <detail>` on stderr, fields separated by two spaces; `viola send` pads `unable` into the mirror's word column after `[/ ]`.
contract/viola-revive/start-output · viola revive · passed start output — A `viola revive` start that passes its preflight prints nothing of its own while the resumed child holds the terminal, as `viola run`.
contract/viola-switch-output/json-ok · viola switch output · --json ok — Under --json a switch that succeeds prints one stdout document whose `ok` carries the `model` and `effort` of the new pair.
contract/viola-switch-output/switched · viola switch output · switched — A switch that succeeds prints `<name>  switched  <model> <effort>` on stdout, exit 0.

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
