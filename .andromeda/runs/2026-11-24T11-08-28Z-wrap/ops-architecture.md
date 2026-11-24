@row_add n=1 row=decision/adopt-refuses-live-session area=established-decisions fact=F1 detector=none
name: Adopt refuses a session a live claude holds
rule: viola adopt resumes under viola, with an operator-given name, a plain claude session named by its session id, and refuses with session-live, exit 1, when a claude agents --json row carries that id; a failed read passes.
state: provisional
basis: fct-zdl3zlbi, fct-k2nhjvap, fct-spvlmkow
because: The chunk lands adopt with the live-elsewhere refusal revive already makes, on the delegate's word, and no row says adopt refuses a held session.
@row_add n=2 row=contract/cli-json-output/adopt-refusal-codes area=standard-contracts fact=F2 detector=D-arch-resources
boundary: CLI `--json` output
item: `viola adopt --json` refusal codes
promise: `<code>` is one of `strict-modes-failed` · `session-live` · `already-live` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`; only `strict-modes-failed` and `session-live` are adopt's own closed details.
basis: fct-zdl3zlbi, fct-spvlmkow
because: A new verb's --json refusal codes are a contract at the CLI boundary, and only revive's list stands registered.
@row_add n=3 row=contract/viola-adopt-refusals/session-live area=standard-contracts detector=D-arch-resources
boundary: viola adopt refusals
item: session-live
promise: A session a live claude holds exits 1 with the pair unable: that session is open in another claude / hint: close it there first, then adopt it again, or under --json the session-live document, and no child is started.
basis: fct-zdl3zlbi, fct-k2nhjvap
because: Adopt's own closed refusal detail carries its own fixed human pair, a promise at the CLI boundary no row holds.
@row_add n=4 row=contract/cli-verbs/adopt area=conventions fact=F3 detector=D-arch-resources
boundary: CLI verbs
item: adopt
promise: `viola adopt <SESSION_ID> --name <NAME> [--parent <NAME>] [--model <MODEL>] [--effort <EFFORT>] [--json]` takes the id in the closed session-id shape, hex of either case, and no words after `--`; a `--parent` equal to `--name` is a usage error, exit 2.
basis: fct-zdl3zlbi, fct-tc34su7x
because: The chunk adds a CLI verb whose arguments no row of the CLI verbs contract names.
@row_add n=5 row=name/command/adopt area=occupied-resources fact=F4 detector=D-arch-resources
class: command
name: adopt
basis: fct-zdl3zlbi
because: The command name adopt is now occupied and the register of command names lacks it.
@row_add n=6 row=decision/typed-session-id-on-child-argv area=occupied-resources fact=F5 detector=D-arch-decisions
name: Typed session id on the child's command line
rule: An operator-typed session id may be placed on the claude child's command line after --resume, only in the session-id shape, as one argv element of a direct spawn.
state: provisional
basis: fct-zdl3zlbi, fct-jtbdy5p2, fct-spvlmkow
because: The standing crossing admits only a session id read from the instance's log, and adopt widens it to a typed id on the delegate's word.
@row_add n=7 row=decision/adopt-lineage-without-role-or-window area=established-decisions fact=F6 detector=none
name: Adopted start's lineage, no role, no window
rule: An adopted start declares as parent its --parent, else the inherited VIOLA_NAME unless that equals its own name; it declares no role and launches with no window.
basis: fct-zdl3zlbi
because: Adopt's lineage and window choices bind every adopted start, and no row states them.
@row_set n=1 row=decision/silent-roles-at-internal-error-catch cell=rule seen=1 detector=none
value: Run, revive's start, adopt's start, mcp and hook print nothing at the internal-error catch site; only a CLI verb prints the internal-error line or document there.
basis: fct-zdl3zlbi, fct-spvlmkow
because: The root bin's role mapping now keeps adopt's start silent at the catch site, so the list of silent roles must name it.
@row_set n=2 row=contract/cli-json-output/internal-error cell=promise seen=1 detector=none
value: `{"v":1,"error":"internal-error","detail":null}` with exit 1 on stdout, stderr empty, when `--json` comes before any `--`; without `--json` the human line `error: internal error` stands, and `run`, `revive`'s start, `adopt`'s start, `mcp` and `hook` stay silent.
basis: fct-zdl3zlbi
because: Adopt's start joins the roles silent at the internal-error catch site, so the contract's silent list must name it.
@row_set n=3 row=contract/child-argv/resume cell=promise seen=1 detector=none
value: A revived or adopted start launches `claude` and adds `--resume <id>`, the id one argv element of a direct spawn.
basis: fct-zdl3zlbi, fct-jtbdy5p2
because: An adopted start now also hands the child --resume with its id, so the promise naming only a revived start is incomplete.
@row_set n=4 row=contract/claude-child-argv/resume cell=promise seen=1 detector=none
value: A revived start adds --resume <id> after those flags, the id one argv element: the newest logged session id, or --id when the instance's log holds it; an adopted start adds --resume with the operator-typed id, in the session-id shape.
basis: fct-zdl3zlbi, fct-jtbdy5p2
because: An adopted start now puts the typed id after --resume, which the promise naming only a revived start leaves out.
@row_set n=5 row=decision/launch-pair-handed-not-verified cell=rule seen=1 detector=none
value: Every start, run, revive and adopt, hands the claude child a model and effort pair on its command line; the pair is never read back or verified as the one the session runs on.
basis: fct-zdl3zlbi, fct-vqqjyoqi
because: Adopt's start hands and records its launch pair like every start, and the rule's list of starts names only run and revive.
@row_set n=6 row=decision/no-start-over-live-instance cell=rule seen=2 detector=none
value: A `viola run`, `viola revive`, `viola adopt` or `viola open` whose `ViolaName` is `live` or `stale` refuses to start.
basis: fct-zdl3zlbi, fct-kzl62unm
because: Adopt runs run's collision check and refuses a live or stale name, so the rule's list of starts must name it.
@row_set n=7 row=decision/run-refuses-held-name cell=rule seen=1 detector=none
value: viola run, viola revive and viola adopt refuse a name that is live or stale, or whose endpoint answers, with exit 1 and a message naming the instance.
basis: fct-zdl3zlbi, fct-kzl62unm
because: Adopt's holder check is run's own and prints run's collision pair, so the rule naming run and revive alone is incomplete.
@row_set n=8 row=contract/diagnostics-role-files/run-name cell=promise seen=1 fact=F8 detector=none
value: The wrapper's codes-only process lines, viola revive's start arm and viola adopt's start included.
basis: fct-zdl3zlbi
because: Adopt logs as process run in run-<name>.ndjson, which the role file's promise does not yet name.
@row_none n=1 fact=F7 type=4
relied: An adopted start spawns in the caller's current directory and records it as the snapshot cwd, which the standing contract/instance-snapshot/cwd and decision/child-cwd-explicit already hold for every start.
