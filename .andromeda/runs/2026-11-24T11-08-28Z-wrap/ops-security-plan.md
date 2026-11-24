@row_set n=1 row=decision/state-tree-strict-modes-before-use cell=rule seen=4 fact=F4 detector=none
value: run, revive's start, adopt's start, open and verify hold the home tree to the strict-modes check before their first read or open under the home and before any log opens; a refusal exits 1.
basis: fct-zdl3zlbi, fct-gnvnr36o, fct-spvlmkow
because: viola adopt's start checks the instance strict-modes first and refuses a widened home or instance dir before any log, so it joins the named list of starts.
@row_set n=2 row=decision/home-tree-strict-modes-before-log cell=rule seen=1 detector=none
value: viola run, revive's start, adopt's start and verify run the home tree's strict-modes check before their log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
basis: fct-zdl3zlbi, fct-gnvnr36o
because: adopt's start runs the strict-modes check before its wrapper log opens and refuses with no child, so the list of starts must name it.
@row_set n=3 row=decision/run-refuses-socket-dir-before-start cell=rule seen=1 detector=none
value: viola run, revive's start and adopt's start refuse a bad socket directory before any snapshot, child or bind: exit 1 and one process-exit with detail socket-dir-refused.
basis: fct-zdl3zlbi
because: adopt launches through run's start and carries run's shared socket-dir-refused code, so the row naming the starts must include it.
@row_set n=4 row=decision/claude-agents-resolved-from-path cell=rule seen=1 detector=D-security-input
value: list, revive's preflight, adopt's live-elsewhere check and the MCP list tool resolve claude for claude agents --json from the calling process's PATH and current directory; its output is untrusted and bounded by MAX_FRAME.
basis: fct-zdl3zlbi
because: revive's live-elsewhere check is now shared with adopt with its body unchanged, so adopt also spawns claude agents --json resolved from the caller's PATH.
@row_set n=5 row=decision/launch-flags-checked cell=rule seen=1 detector=D-security-input
value: viola run's, viola revive's and viola adopt's --model must be a model word, 1 to 64 bytes, a lowercase letter first, then lowercase letters, digits, dots and dashes, and --effort one of the five effort words of claude --help; a bad value is a usage error, exit 2, before anything is read.
basis: fct-zdl3zlbi
because: viola adopt takes --model and --effort through run's own model and effort parsers, so the launch-pair check now holds for adopt too.
@row_set n=6 row=contract/cli-json-error/internal-error cell=promise seen=1 fact=F6 detector=none
value: Under --json the internal-error line is replaced by {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty; run, revive's start, adopt's start, mcp and hook stay silent there.
basis: fct-zdl3zlbi, fct-pwskfqkr
because: adopt is a named wrapper role silent at the internal-error catch site and prints the internal-error document under --json, so the silent list must name it.
@row_add n=1 row=decision/adopt-session-id-argv-crossing area=input-validation fact=F1 detector=D-security-input
name: Adopt's typed session id onto the child's argv
rule: viola adopt takes an operator-typed session id only in the closed session-id shape, ASCII hex of either case, and hands it to the claude child as the one argv element after --resume of a direct spawn; a malformed id is a usage error, exit 2, before anything is read.
why: A boundary widening: an operator-typed value, not a logged one, crosses onto a child's command line.
state: provisional
basis: fct-jtbdy5p2, fct-zdl3zlbi, fct-tc34su7x
because: adopt is a new external-input surface whose typed id crosses onto the claude child's argv, a widening that stands on the delegate's word (W2).
@row_add n=2 row=decision/adopt-refuses-live-session area=authentication-authorization fact=F2 detector=D-security-auth
name: Adopt refuses a session a live claude holds
rule: viola adopt refuses a session a live claude holds, an id a live adopted instance holds included, with session-live, exit 1, one fixed unable:/hint: pair or its --json document, and no child started.
state: provisional
basis: fct-k2nhjvap, fct-zdl3zlbi
because: adopt's live-elsewhere refusal is a new guard on which session a start may take, standing on the delegate's word (W1, W4).
@row_add n=3 row=decision/adopt-agents-read-fails-open area=input-validation fact=F3 detector=D-security-input
name: Adopt starts when its agents read fails
rule: When viola adopt's claude agents read for its live-elsewhere check fails, adopt starts anyway: the no-signal arm fails open.
state: provisional
basis: fct-k2nhjvap, fct-spvlmkow
because: the plan asks W1's no-signal arm as a row of its own, a fail-open choice standing on the delegate's word.
@row_add n=4 row=decision/adopt-arguments-checked area=input-validation fact=F5 detector=D-security-input
name: viola adopt's parent and dashes checked
rule: A viola adopt --parent equal to --name is a usage error, exit 2, as is any word after the dashes; with no --parent, the parent is the inherited VIOLA_NAME unless it equals the name.
basis: fct-zdl3zlbi
because: adopt is a new CLI start surface whose lineage and trailing words are checked as usage errors before anything is read.
@row_add n=5 row=contract/strict-modes-refusal/adopt area=authentication-authorization detector=none
boundary: strict-modes refusal
item: viola adopt
promise: Refused before any log with no child started and the home or instance mode left as found: the instance-state unable:/hint: pair on stderr, or under --json the strict-modes-failed refusal code.
basis: fct-zdl3zlbi, fct-gnvnr36o
because: adopt adds a strict-modes refusal of its own beside run and revive's, whose standing row names only those two.
@row_add n=6 row=decision/r5-adopt-script-ends-own-processes area=authentication-authorization fact=F7 detector=none
name: R5 adopt script ends only its own processes
rule: The test-only R5 adopt live-reading script ends only the wrapper it recorded, while that process's exe is the given viola, and its own script host: TERM, then 10 s, then KILL only if the start time is unchanged; its verdicts keep words only.
basis: fct-zdl3zlbi, fct-slqf7dkf
because: the new live-reading script acts on processes, and the bound on which processes it may end is a decision future rigs must respect.
