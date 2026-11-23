@history_item n=1 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — introduced the `waiting:` line (stderr, terminal only, never under `--json`), the one-`--json`-document rule and the once-printed `error: internal error` from the one catch site. Why it matters: `viola open` waits up to 30 s for liveness; this is the precedent (contract/viola-wait-output/waiting-line, decision/no-spinner) for how a blocking verb may show it is waiting, and for joining the catch site.

@history_item n=2 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — a new verb (`answer`) was added to the catch-site verb list printing `error: internal error` once, no hint. Why it matters: the register `viola open` extends as a new verb (contract/cli-catch-site/internal-error, contract/cli-internal-error/human); later entries (wheel, list, plugin install, link/unlink, revive --list) grew it the same way.

@history_item n=3 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — gave `viola revive` its output structure, its refusal pairs (including `run`'s collision pair `unable: <name> is already live` / `hint: viola list`) and put it in the `setup:` help group. Why it matters: `open` reads `already-live` / `stale-heartbeat` before any spawn with the same pair (contract/viola-run/already-live, contract/viola-revive-refusals/already-live) and joins the same `setup:` row as `setup:  open, run, revive, verify, plugin install`.

@history_item n=4 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — built `viola list` and its `--json` refusal document, its widths and its catch-site entry. Why it matters: `list --json` gains `window?` / `origin?` in this chunk while the six-column human board stays unchanged (contract/viola-list-json/document, contract/viola-list/columns, decision/human-output-is-contract: new fields go into `--json` first).

@history_item n=5 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — cut web-spa to a stub; `list --json`'s item and skipped shapes became the `list` payload contract (architecture §Standard Contracts); shared vocabulary fixed the six captions and slot order. Why it matters: the `window?` / `origin?` additions to list items are a change of the list payload contract, not of the layout; the six captions stay fixed (decision/shared-captions-fixed).

@history_item n=6 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `viola revive` took `--json` (each refusal one stdout document `{"v":1,"error":"<code>","detail":null}`, exit 1, empty stderr, human pairs byte-identical) and `--json` became the machine view of every data verb. Why it matters: the exact pattern `open`'s `--json` error document follows (contract/viola-revive-json/refusal-document, decision/json-on-every-data-verb, contract/cli-json-mode/one-document).

@history_item n=7 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built `viola --help` as the grouped verb table over served verbs only, pinned by the `HELP` literal and `help_lists_only_served_verbs`; `viola <verb> --help` stays clap's per-verb usage. Why it matters: adding `open` to `setup:` must update the `HELP` literal and the served-verbs test (contract/viola-help/verb-groups, contract/cli-verbs/help, contract/viola-verb-help/usage); plain-output modes come from here too.

@history_item n=8 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` is a hidden verb whose tools are the driver's machine view of the CLI verbs' `--json` documents less `v`, naming no wheel or handoff verb. Why it matters: the MCP `list` tool's items gain `window?` / `origin?` in this chunk (contract/mcp-tools/list, contract/mcp-tool-results/success); whether `open` is an MCP tool is outside this scope, and the help literal stays unchanged for hidden verbs.

@history_item n=9 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — `plugin install` was served into the `setup:` row, read by `help_lists_only_served_verbs` as one entry; it joined the catch-site verbs. Why it matters: the most recent precedent for adding a verb to the `setup:` row and the catch site (decision/flat-verbs exception, contract/viola-help/verb-groups).

@history_item n=10 marker=2026-10-20T06-58-31-wrap
**2026-10-20T06-58-31-wrap** — the founder ratified `--help`'s served-verbs-only rule and the board's width source as built. Why it matters: `open` appears in `--help` only once served; the rule is the founder's, not a delegate's.

@history_item n=11 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the `handoff:  link, unlink` help row built; catch-site and exit-21 lists grown) — the help table went from four to five rows and every line naming the pending owner was retired; `link`/`unlink` joined the catch site. Why it matters: the precise sites (Primary screens, `--help` As built, Discoverability) a new help-row word touches; argument order mirrors channel params (decision/argument-order-mirrors-channel).

@history_item n=12 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — `run` and `revive` starts check the home tree before the wrapper log opens: the home pair or the instance state-files pair, exit 1, no log, no child; `revive --json` `strict-modes-failed`. Why it matters: `open`'s exit 1 `strict-modes-failed` before any log is this refusal (contract/run-revive-start/home-tree-refused, contract/viola-revive-json/strict-modes-failed, contract/viola-revive-refusals/state-files-writable, contract/viola-verify-refusals/home-not-private); trap: a check placed after a role file opens passes a widened home.

@history_item n=13 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (all four sub-entries) — exit 21's document carries its cause in `detail` while exit 1's `{"v":1,"error":"<code>","detail":null}` keeps its null; the catch site prints `{"v":1,"error":"internal-error","detail":null}` under `--json` before a `--`; `stale-heartbeat` split from `already-live` for revive; `revive --json` became the machine road to `run`'s start causes. Why it matters: `open`'s exit-1 codes (`terminal-not-found`, `not-live`, `already-live`, `stale-heartbeat`, `strict-modes-failed`) take the null-detail document; the `--json`-before-`--` rule matters since `open` takes `-- claude`.

@history_item n=14 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the unwrapped-name and endpoint-lost hints) — each refusal cause got its own hint, last on stderr, plain ASCII. Why it matters: decision/hint-for-every-cause — `open`'s new refusals (`terminal-not-found`, `not-live`) each need a distinct hint naming the next verb, quoting no text and naming no path or pid, and never `viola release`.

@history_item n=15 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory refusal of the `run` and `revive` starts) — a start refusal shared by `run` and `revive` with its pair and `socket-dir-refused` code. Why it matters: the started `viola run` inside `open`'s window carries these start refusals; `open` must decide which it reads before the spawn and which surface only in the window (left standing, `not-live`).

@history_item n=16 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the per-OS wording of help and plugin install) — the usage line is the override literal; `viola <verb> --help` names the program by its executable; the Windows VT enable left the colour surface. Why it matters: `open` is Linux/Hyprland-only by design with no per-OS wording (contract/viola-verb-help/usage, decision/std-isterminal-tty-detection).

@history_item n=17 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0: no web surface; the keyboard-driven terminal fleet console tiled on Hyprland takes its own surface at «Console scaffold»; no verb has an interactive prompt or pager and the console is no verb's output; the six captions are reused by the console. Why it matters: `open` places a terminal window on Hyprland but is a plain verb, not the console (decision/no-prompt-or-pager, decision/flat-verbs, absence/web-surface); the scope excludes console work.

@history_item n=18 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows (decision / contract / absence), each verified against the code. Why it matters: every layout change of this chunk lands as row operations (`+` / `~` / `−` by handle), e.g. new `open` contract rows beside contract/cli-verbs/run.

@history_item n=19 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (both sub-entries) — exit 11 retired and never reused; the `list` payload lost its `budget` member (now `{items, skipped}`). Why it matters: `open` maps no refusal to 11 (contract/cli-exit-codes/11), and the list document `window?` / `origin?` extend is the `{items, skipped}` form (contract/viola-list-json/document).

@history_item n=20 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `viola run`'s start writes the CLI's trust record for an untrusted session folder, flag only, no trust dialog, no key typed. Why it matters: `open` starts `viola run` in a new foot window with `--cwd`; that start's trust step stands unchanged and adds no refusal or byte (contract/cli-verbs/run).

@history_item n=21 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — before the child, a key-on start's automatic verify line and verify's output reach the terminal. Why it matters: the `run` started inside `open`'s window may print verify output there and take longer to go live, bearing on `open`'s 30 s liveness bound.

@history_item n=22 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (the delegate's confirm ratified — 2026-11-20-a-new-cli-version-stamped-automatically, contract/viola-run/start-output) — the automatic-verify start output stands on the founder's own word. Why it matters: `open` builds on that start behaviour as ratified, not provisional.

@history_item n=23 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (4 rows, 1 of 2) — the board draws wrapped children under their parent as a tree; `decision/mcp-hides-wheel-and-handoff-verbs` widened so the MCP list result carries held links as data (PROVISIONAL, founder-forks W1). Why it matters: `open`'s `--parent` / `VIOLA_NAME` default ties to this lineage; the MCP list item gains `window?` / `origin?` under the same rule, naming no wheel or handoff verb.

@history_item n=24 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (1 row, decision/no-link-column) — the six board columns, captions and slot order stay unchanged, links shown in a block of their own after the rack. Why it matters: the precedent for this chunk keeping the six-column board unchanged while window / origin data goes to `--json` only (decision/no-link-column, contract/viola-list/columns).

@history_item n=25 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — `viola revive` gained `--model` / `--effort`; added contract/run-revive-usage/pair-after-dashes (a launch pair after the dashes is a usage refusal, exit 2, one fixed stderr line echoing none of the caller's words); PROVISIONAL, founder-forks W2. Why it matters: `open` takes `--model` / `--effort` and its own fixed exit-2 line for a word after `--` other than `claude` sits beside `PAIR_AFTER_DASHES`, with the same no-echo form.
