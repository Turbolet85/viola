@row_set n=1 row=contract/help-output/verb-groups cell=promise seen=5 detector=D-design-cli-wording
value: `viola --help` lists the served verbs as `board: list, focus` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, adopt, up, switch, verify, plugin install`, then `Options:`.
basis: fct-vocxyji6
because: The `setup:` row of `viola --help` now lists `switch` after `up`, so the row printing that line must carry it.
@row_set n=2 row=contract/viola-help/verb-groups cell=promise seen=5 detector=D-design-cli-wording
value: `viola --help` lists the served verbs only, grouped `board: list, focus` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, adopt, up, switch, verify, plugin install`, then `Options:`.
basis: fct-vocxyji6
because: The `setup:` row of `viola --help` now lists `switch` after `up`, so the row printing that line must carry it.
@row_set n=3 row=contract/cli-exit-codes/13 cell=promise seen=2 detector=D-design-cli-wording
value: `unable  not-delivered` with `input-not-ready`, `no-prompt-submitted`, `turn-running`, `unknown-dialog`, `control-character`, `empty-text`, `paste-cut`, `dialog-pending`, `no-session`, `child-not-ended` or `resume-failed`; under `--json` the refusal object.
basis: fct-xvxcbpio
because: The switch verb adds four not-delivered details at exit 13, and the row listing exit 13's closed details must name them.
@row_set n=4 row=contract/cli-refusal-line/fixed-unable-form cell=promise seen=6 detector=none
value: The exit-1 refusals of `run`, `revive`, `adopt`, `open`, `focus`, `up` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch` print the fixed `unable: <text>` form.
basis: fct-pmolmu2p
because: `viola switch` runs the frame verbs' strict-modes refusal first, so it joins the verbs whose exit-21 strict-modes refusal prints the fixed form.
@row_set n=5 row=contract/cli-stderr/fixed-message-refusal cell=promise seen=6 detector=none
value: The exit-1 refusals of `viola run`, `revive`, `adopt`, `open`, `focus`, `up` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch` use `unable: <text>`.
basis: fct-pmolmu2p
because: `viola switch` runs the frame verbs' strict-modes refusal first, so it joins the verbs whose exit-21 strict-modes refusal uses the fixed form.
@row_set n=6 row=contract/cli-strict-modes-refusal/instance-state-pair cell=promise seen=1 detector=none
value: The exit-21 strict-modes refusal of `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch` prints the fixed instance-state `unable:` / `hint:` pair, not the `[/ ]` mirror.
basis: fct-pmolmu2p
because: `viola switch` refuses an untrusted instance through the same strict-modes check as the other frame verbs, before any frame.
@row_set n=7 row=contract/strict-modes-refusal/exit-21 cell=promise seen=1 detector=none
value: A refused strict-modes check of the driven instance by `viola link` / `unlink`, or of the instance of `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch`, is made before any frame, exits 21, and writes no frame and no line.
basis: fct-pmolmu2p
because: `viola switch` runs the strict-modes refusal before its endpoint read and its connect, so the promise now binds it too.
@row_set n=8 row=contract/frame-verb-refusals/strict-modes cell=promise seen=1 detector=none
value: A failed strict-modes check (`send` after reading its text; `wait`, `last`, `answer`, `pause`, `release`, `switch` first; each before any frame) prints the instance-state pair, no `unable  instance-unreachable` line or mirror; `--json`: the `strict-modes-failed` document.
basis: fct-pmolmu2p
because: `viola switch` runs the strict-modes refusal first, before any frame, as the listed frame verbs do.
@row_set n=9 row=contract/exit-21-json-cause/strict-modes-failed cell=promise seen=3 detector=none
value: The `list`, `link`, `unlink`, `focus`, `up` and `plugin install` refusals and the seven frame verbs' strict-modes refusals read `strict-modes-failed`.
basis: fct-pmolmu2p
because: `viola switch` is a seventh frame verb refusing an untrusted instance by strict modes, so the count the cell holds moved.
@row_set n=10 row=contract/exit-21-json-cause/server-verify-failed cell=promise seen=1 detector=none
value: A process on the endpoint that is not the recorded wrapper (pid or start time differs, the check cannot be made, or the peer's euid is not this user's or cannot be read) gets no frame; the nine frame-writing verbs read `server-verify-failed`.
basis: fct-pmolmu2p
because: The switch method is served only after the existing peer and server checks of the client connect, so `viola switch` is a ninth frame-writing verb reading that cause.
@row_add n=1 row=name/command/switch area=surface-cli fact=F1 detector=none
class: command
name: switch
basis: fct-xvxcbpio
because: A new CLI verb `viola switch` occupies the command name switch.
@row_add n=2 row=contract/cli-verb-output/switch area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: CLI verb output
item: viola switch
promise: `viola switch <name>` prints one ASCII line on stdout, `<name>  switched  <model> <effort>`, or under `--json` the pair document holding `model` and `effort`.
basis: fct-xvxcbpio
because: The new verb prints a new result line and document, and the masters state every verb's output wording.
@row_add n=3 row=decision/switch-refusal-order area=surface fact=F4 detector=D-design-cli-wording
name: Switch refusal order
rule: `viola switch` reads its refusals in a fixed order, the first winning, none ending the child: `human-typing` (exit 10), `turn-running`, `dialog-pending`, `unverified-cli` (exit 12), `no-session`, then at the respawn `child-not-ended` and `resume-failed`, each `not-delivered` at exit 13.
state: provisional
basis: fct-xvxcbpio
because: The switch verb brings a refusal ladder of its own, built provisional on fork F3, that future verbs and hints must respect.
@row_add n=4 row=contract/viola-send-refusal/switch-standing area=surface-cli fact=F6 detector=none
boundary: viola send refusals
item: turn-running while a switch stands
promise: A send is refused `not-delivered` `turn-running` from a switch's request until the new child's session start.
basis: fct-xvxcbpio
because: The send verb now refuses while a switch stands, a promise at its refusal boundary no row states.
@row_add n=5 row=decision/switch-cli-only area=surface-cli fact=F7 detector=none
name: Switch is CLI-only
rule: Switching a session's model and effort is the `viola switch` CLI verb alone; no MCP tool switches a session.
state: provisional
basis: fct-5xyy4zny
because: The chunk built the switch with no MCP tool, provisional on fork F5, and the board's callers must know the MCP surface lacks it.
@row_add n=6 row=decision/switch-keeps-the-slot area=brand-identity fact=F8 detector=none
name: A switch keeps the session's slot
rule: A switch replaces only the child: the name, the wrapper and the session's slot stay, viola writes nothing to the terminal at the respawn and nothing to the user's own settings, and a later revive replays the switched pair.
state: provisional
basis: fct-bs3a4rfe
because: The switch respawns the child in place under the same wrapper, provisional on fork F1, and the stable-slot and silent-wrapper direction must hold across it.
@row_none n=1 fact=F3 type=4
relied: the exit-2 row already promises clap usage text for a usage error; the verb's at-least-one-half argument group is the code's
@row_none n=2 fact=F5 type=5
what: the four added entries of the refusal hint table are an inventory the code holds; the rule that no hint names viola release already stands as its own rows
