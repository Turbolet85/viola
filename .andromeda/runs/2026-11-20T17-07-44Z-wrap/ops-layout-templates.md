@row_set n=1 row=contract/viola-run/start-output cell=promise seen=2 fact=F3 detector=D-design-cli-wording
value: Before the child, only a key-on start's automatic verify line and verify's own output reach the terminal (discarded under `revive --json`); the trust step adds no refusal, exit code, hint, `--json` code or terminal byte; then the terminal belongs to the wrapped claude TUI until it exits.
basis: fct-zfntgo5a
because: The start now runs viola verify after the bind with its output inherited and writes one fixed stderr line, so `viola run` no longer prints nothing of its own before the child.
@row_set n=2 row=contract/viola-revive/start-output cell=promise seen=1 detector=D-design-cli-wording
value: A `viola revive` start that passes its preflight prints nothing of its own while the resumed child holds the terminal; before the child, a key-on start's automatic verify prints as `viola run`'s does, and nothing under `--json`.
basis: fct-zfntgo5a
because: Revive's start reaches the same automatic verify through the shared start, so its pre-child output now holds the verify line and verify's output, discarded under `revive --json`.
@row_add n=1 row=contract/run-revive-start/automatic-verify area=surface-cli fact=F1 detector=D-layout-surface
boundary: viola run and viola revive starts
item: automatic verify
promise: With config.json `verify_new_cli` true (off by default), a start that meets an unstamped claude version runs `viola verify` once before the child; with the key off, no verify runs and nothing is printed.
basis: fct-xgc7qjiw
because: The chunk adds the `verify_new_cli` key and a verify step to the start, a new pre-child surface of both start verbs the master does not state.
@row_add n=2 row=contract/run-revive-start/automatic-verify-line area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: viola run and viola revive starts
item: automatic verify line
promise: Exactly one of five fixed stderr lines (verifying; did not verify; earlier red, no verify; verifying elsewhere, not waited for; no stamp with spawn-failed, unstamped or lock-refused), ASCII, one write, no SGR, only the version and a closed word; none under `revive --json`.
basis: fct-zfntgo5a
because: The chunk adds the L1 to L5 stderr lines of the start's verify, a new CLI wording the master must state.
@row_add n=3 row=contract/run-revive-start/automatic-verify-stop area=surface-cli fact=F3 detector=D-layout-surface
boundary: viola run and viola revive starts
item: automatic verify stop
promise: A SIGINT or SIGTERM during a start's automatic verify stops the verify and ends the start by that signal, with no session.
basis: fct-zfntgo5a
because: The chunk installs a stop for the start's verify wait that TERMs the verify child and re-raises the signal, a new start outcome the master does not state.
@row_add n=4 row=decision/automatic-verify-once-per-version area=surface-cli fact=F4 detector=none
name: Automatic verify once per version
rule: A red automatic verify is recorded once per claude version; later starts print the earlier-red line and run no verify until the stamp is cleared by a hand-run `viola verify`.
basis: fct-jupau6dq
because: The chunk makes an attempted unverified stamp end every later start's automatic verify with the earlier-red line, a rule against a verify loop that future work must respect.
