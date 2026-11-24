@row_set n=1 row=contract/start-refusal/home-strict-modes cell=promise seen=1 detector=none
value: Over a home tree failing the strict-modes check, `viola run`, a `viola revive` or `viola adopt` start and `viola verify` refuse before any log opens: exit 1, stdout empty, the home-state pair on stderr with `hint:` last, no child; `verify` prints no step line and no summary.
basis: fct-spvlmkow
because: adopt's start now refuses a widened home before any log with exit 1 and no child, so the list of starts this refusal covers gains adopt.
@row_set n=2 row=contract/start-refusal/socket-dir cell=promise seen=1 detector=none
value: Over a failing socket directory, `viola run` and a `viola revive` or `viola adopt` start refuse with exit 1 before any snapshot, bind or child: the socket-directory pair on stderr, `hint:` last, plain ASCII, no SGR, naming only the parsed name; the directory is left as found.
basis: fct-spvlmkow
because: adopt's start runs run's shared start, socket-dir-refused among its refusals, so the starts this refusal names gain adopt.
@row_set n=3 row=decision/strict-modes-refusal-before-any-write cell=rule seen=2 detector=none
value: A home or instance failing the strict-modes check is refused first, before any log opens, frame is sent or child starts, with no line written: exit 21 for frame verbs, link/unlink and focus, exit 1 for run, revive's and adopt's start and verify.
basis: fct-gnvnr36o
because: adopt's start refuses a widened home or instance dir before any log with exit 1, so the exit-1 list gains it.
@row_set n=4 row=contract/json-fault/internal-error cell=promise seen=1 detector=none
value: Where a verb prints error: internal error, --json prints the internal-error document with null detail on stdout instead, stderr empty; run, revive's and adopt's start, mcp and hook stay silent there.
basis: fct-spvlmkow
because: adopt's start joins the roles silent at the internal-error catch site, so the silent list gains it.
@row_set n=5 row=contract/tui-passthrough/entry cell=promise seen=1 detector=none
value: The passthrough is entered by viola run, by a viola revive whose preflight passed and by a viola adopt whose preflight passed.
basis: fct-zdl3zlbi
because: adopt's start runs run's start and then pumps the child, so adopt is a new way into the passthrough.
@row_set n=6 row=decision/socket-dir-refusal-before-any-frame cell=rule seen=1 detector=none
value: A refused socket directory stops the verb before any frame, snapshot, child or bind and leaves the directory as found: exit 21 for the frame verbs, exit 1 for the run, revive and adopt start.
basis: fct-spvlmkow
because: adopt's start shares run's socket-dir-refused refusal, so the exit-1 starts gain adopt.
@row_set n=7 row=decision/claude-agents-read-fails-open cell=rule seen=1 detector=none
value: A failed, oversize, malformed or timed-out claude agents --json read leaves the list board as the board read gave it and passes revive's and adopt's preflight.
basis: fct-k2nhjvap
because: adopt reads claude agents for its live-elsewhere check and a failed read starts, so the fail-open rule covers adopt's preflight too.
@row_set n=8 row=contract/cli-refusal/unable-line cell=promise seen=1 detector=none
value: A refusal prints `unable  <name>  <reason>  <detail>` on stderr, the plain word followed by plain detail; `viola revive`'s exit-1 refusals, `viola adopt`'s two own refusals, `viola run`'s socket-directory refusal and `viola verify`'s interrupted pair print one `unable: <text>` line instead.
basis: fct-zdl3zlbi
because: adopt's strict-modes-failed and session-live refusals are fixed unable:/hint: pairs, so the list of one-line forms gains them.
@row_set n=9 row=contract/cli-fixed-message/form cell=promise seen=1 detector=none
value: viola revive's exit-1 refusals, viola adopt's strict-modes-failed and session-live refusals and viola run's socket-directory refusal print one unable: <text> line, then one hint: line last.
basis: fct-zdl3zlbi
because: adopt's two own refusals print the fixed unable:/hint: pair form, so the verbs this form names gain adopt.
@row_set n=10 row=contract/log-identity/process cell=promise seen=1 detector=none
value: `process:"cli"` for short-lived verbs with a resolved instance; `process:"run"` with `instance:<ViolaName>` for `viola run` and a `viola adopt` start.
basis: fct-zdl3zlbi
because: adopt opens run's wrapper log and logs as process run, so the process tag's promise must name it.
@row_set n=11 row=contract/start-json-refusal/document cell=promise seen=1 fact=F2 detector=none
value: Every refusal of the start revive and adopt share with run, and adopt's own strict-modes-failed and session-live, gives one {"v":1,"error":"<code>","detail":null} document with no hint and empty stderr.
basis: fct-spvlmkow
because: adopt's --json refusals are its two own codes plus run's shared start codes, each one hint-free document, so the row that says this for revive must cover adopt.
@row_add n=1 row=decision/adopt-session-live-pair area=cognitive-accessibility fact=F1 detector=none
name: Adopt's session-live refusal pair
rule: viola adopt refuses a session that a live claude holds elsewhere with session-live, exit 1, as the fixed pair `unable: that session is open in another claude` then `hint: close it there first, then adopt it again` last on stderr.
state: provisional
basis: fct-zdl3zlbi
because: adopt brings a new fixed refusal pair in its planning form on the delegate's word, which the cognitive-accessibility rows on fixed refusal copy must hold, marked provisional.
@row_none n=1 fact=F3 type=4
what: adopt's usage exits (a parent equal to the name, a malformed id, words after --) are clap usage errors the standing exit-2 usage contract already covers; the id's case is the parser's, held in the code.
@row_none n=2 fact=F4 type=5
what: the setup group's verb list is an inventory of served verbs pinned by the help literal; the group-label structure a11y relies on is unchanged.
