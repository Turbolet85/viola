@history_item n=1 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — Surface: cli, 5 rows: `~ contract/cli-verbs/help` and `~ contract/viola-help/verb-groups` (the `setup:` row became `open, run, revive, verify, plugin install`), `+ contract/cli-verbs/open`, `+ contract/viola-open/parent-default`, `+ contract/viola-revive/no-window`. Why it matters: this is the direct precedent for `viola focus`. A new served verb has to edit both help rows and add its own `contract/cli-verbs/<verb>` row. The open verb's window record (origin, placement) is the identity source (a) that focus reads.

@history_item n=2 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — four sub-entries: link/unlink joined the exit-21 `unable` lines, the server check (frame verbs went from six to eight) and the catch site; the `handoff:` help row was built; the result lines; the strict-modes refusal of the driven instance. Why it matters: this is the full checklist of tables a new verb joins (word column, not-running hint, catch-site list, help row, `help_lists_only_served_verbs`). It also records the trap: opening a role file narrows a 0770 home to 0700, so the strict-modes check must run before any `client::start`, log or snapshot read. `link_verbs_refuse_a_widened_instance_before_any_log` holds that order. Focus reads instance state under strict modes, so it inherits the trap.

@history_item n=3 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — the frame verbs' exit-21 instance pair (`unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`), made before any role file or log opens, mode left as found, no log line. Also the home-tree refusals of verify and the run/revive starts. Why it matters: this is the standing refusal form (`contract/frame-verbs/strict-modes-refused`, `contract/viola-revive-refusals/state-files-writable`) that focus's strict-modes read must reuse or deliberately fork from. Exit 21 (frame verbs, list) and exit 1 (revive) both stand as precedents.

@history_item n=4 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — four sub-entries. (1) Exit 21's `--json` document carries its cause in `detail`, from a closed list (`strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`); `detail: null` is retired for exit 21. (2) The catch site prints `{"v":1,"error":"internal-error","detail":null}` under `--json`. (3) `revive --json` start codes. (4) The unwrapped-name and endpoint-lost hints, each cause getting its own hint. Why it matters: focus's no-window and `unsupported` refusals need distinct codes and distinct hints (`decision/hint-for-every-cause`, `contract/instance-unreachable-detail/*`). The founder ruled the catalogue after the widening was shown to him, so a new cause word or a new detail code is a widening that needs a ruling, not an implementation choice.

@history_item n=5 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — exit 21's sixth cause `socket-dir-refused`, with its own hint, at the frame verbs; `server-verify-failed` reached via the euid check; the socket-directory refusal of the run/revive starts. Why it matters: this is the most recent extension of the exit-21 cause list (`contract/instance-unreachable-detail/socket-dir-refused`) and shows the form: a new cause word goes in the typed-tail list, gets its own hint, keeps `error` = `instance-unreachable` and `v` = 1, and the founder confirms the code.

@history_item n=6 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — added the server-check cause and its hint (`the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`) for the frame verbs, with no frame written. Why it matters: it sets the rule behind `contract/frame-verb-hints/server-verify-failed`. If focus contacts the wrapper, it has to decide whether it is a frame verb under this check. If it reads only state, it stays outside the check.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — sub-entries: the Windows VT enable left the colour surface; per-OS help wording dropped (`viola <verb> --help` names the program by its executable); the server-check hint became unscoped; the `.cmd`/`.bat` refusal left with its emitter ("a closed word no build can emit leaves with its emitter"). Why it matters: Linux is the only target, which bears on `contract/viola-verb-help/usage` and `decision/plain-output-modes`. The rule that a refusal word exists only with a build that emits it applies to focus's `unsupported`, which is Hyprland-only, not OS-scoped.

@history_item n=8 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0: no web surface (`absence/web-surface`). "No interactive prompt and no pager in a verb" was scoped to verbs, and the keyboard-driven fleet console tiled on Hyprland takes its own surface owed by "Console scaffold". Why it matters: `decision/no-prompt-or-pager` states that the terminal fleet console is no verb's output. `viola focus` is a CLI verb that moves Hyprland focus, so it must stay a one-shot verb and must not grow into console behaviour.

@history_item n=9 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `viola --help` was built as the grouped table: exact usage line, served verbs only, pinned by the `HELP` literal, no about line, no `Commands:` list. Why it matters: these are the rules behind `contract/viola-help/usage-line`, `contract/viola-help/verb-groups` and `contract/cli-verbs/help`. Adding `focus` means placing it in a group row and updating the `HELP` literal and `help_lists_only_served_verbs`.

@history_item n=10 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch: help with served verbs only and the board width from `host_size()` stand as built; the provisional marks are gone. Why it matters: the served-verbs-only help rule is the founder's word, so `focus` joins the help table only once it is served.

@history_item n=11 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — `viola plugin install` was served: it joined the `setup:` row, got a new output-structure section, gained its exit-21 home pair among the fixed-message exceptions, and joined the catch-site verbs. Its lines were a planning form, with the founder's wording owed (later ratified). Why it matters: it is a precedent for a new verb's wording landing as a planning form pending the founder's word, and for a non-frame verb taking exit 21 with a fixed state pair (`contract/cli-exit-codes/21`, `contract/viola-plugin-install/internal-error`).

@history_item n=12 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` became a hidden verb run only by the plugin. Its tools are the driver's machine view (each tool's result is the CLI verb's `--json` document minus `v`), and no tool names release/pause/link/unlink. Why it matters: this governs `decision/mcp-verb-hidden`. The scope keeps an MCP `focus` tool out (CLI-only, PROVISIONAL); adding one would be a widening of this surface.

@history_item n=13 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — `~ decision/mcp-hides-wheel-and-handoff-verbs` (the list tool carries links as data; PROVISIONAL, founder-forks W1); the board tree and the LINKS block. Why it matters: this is the lineage/ancestry chunk whose walk focus reuses for hand-started sessions. Its MCP decision is the precedent for any driver-facing focus surface.

@history_item n=14 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `--json` became the machine view of every data verb, `viola revive` included (it was the exception before); `revive --list` joined the catch site. Why it matters: `decision/json-on-every-data-verb` and `contract/cli-json-mode/one-document`. Focus must take `--json` with one document per outcome, its refusals included.

@history_item n=15 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a new verb's output structure, its place in the command list and the `setup` help group, and its refusals as `unable:`/`hint:` pairs with exit 1. The help group placement was the operator's answer. Why it matters: it is a precedent for a new verb's refusal shape (`contract/viola-revive/refusal-shape`) and shows that a verb's help-group placement is a fork for a ruling.

@history_item n=16 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — wait/last got the exit-21 pair `unable  <name>  instance-unreachable` plus its hint, one `--json` document, and `error: internal error` once from the one catch site; they joined the shared `unable` word column. Why it matters: this is the origin of `contract/cli-refusal-line/form`, `decision/shared-refusal-word-column` and `contract/cli-catch-site/internal-error`, which focus's refusal lines must follow.

@history_item n=17 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — `answer` joined the catch-site verb list. Why it matters: a new verb has to be added to the catch-site list (`contract/cli-catch-site/internal-error`).

@history_item n=18 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — `pause`/`release` joined the catch-site verbs. Why it matters: same as n=17, a new verb joins the catch-site list.

@history_item n=19 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — list's exit-21 strict-modes refusal: the human state pairs, the `--json` document, no log opened, mode left as found; list joined the catch-site verbs. Why it matters: it is the source of `contract/viola-list/strict-modes-refused` and `contract/viola-list-json/strict-modes-failed`, a precedent for a non-frame verb refusing on strict modes before any log opens.

@history_item n=20 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder confirmed list's exit-21 refusal with no log opened and the human refusal pairs in the line form. Why it matters: the refusal forms focus would reuse are the founder's own word.

@history_item n=21 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the `unknown` refusal's detail became a code word or nothing; `unknown`, `wrapper fault` and `internal error` still take no hint. Why it matters: it bounds `decision/hint-for-every-cause` (the "except" clause) and `decision/hints-never-quote-or-locate`. Focus's hints must not quote compositor or upstream text such as a window title or address.

@history_item n=22 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder ratified the provisional records of 2026-10-31-exit-cause-code-catalogue (the unwrapped-name and endpoint-lost hints, the detail catalogue). Why it matters: the exit-21 cause list and its hints that focus extends are settled law, not provisional.

@history_item n=23 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ contract/cli-exit-codes/11` (retired, never reused) and `~ contract/cli-exit-codes/14`. Why it matters: if focus needs a new exit code, 11 cannot be used, and the exit map must be extended explicitly.

@history_item n=24 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — the cli surface stands alone, with the board-versus-strip rule and the streams. Why it matters: it is the origin of `decision/board-vs-strip`, under which focus is a strip verb acting on one `ViolaName`.

@history_item n=25 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — client-side refusals made before any request (`control-character`, `empty-text`). Why it matters: it is a precedent, behind `contract/viola-send-output/unable`, for refusing on the client side with a named cause before contacting anything. That bears on focus's `unsupported` refusal outside Hyprland.

@history_item n=26 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's prose became rows (226). Why it matters: every picked row dates from this migration, so any focus additions land as `+`/`~` row operations, as the later entries show.

@history_item n=27 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — `~ contract/viola-revive/usage`, `+ contract/run-revive-usage/pair-after-dashes`; a new CLI refusal wording with its exit code landed as a delegate-word widening, PROVISIONAL (founder-forks W2). Why it matters: this is the latest precedent for how new refusal wordings and exits are priced and marked when the founder's word is not yet given. Focus's refusal words are such forks.
