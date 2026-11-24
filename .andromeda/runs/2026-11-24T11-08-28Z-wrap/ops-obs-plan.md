@row_set n=1 row=contract/strict-modes-refusal-exit/1 cell=promise seen=3 detector=none
value: run, revive's start, adopt's start, verify, open and up's home-tree check exit 1 on a strict-modes refusal; revive, adopt, open and up under --json print the strict-modes-failed document.
basis: fct-zdl3zlbi, fct-spvlmkow, fct-gnvnr36o
because: viola adopt's start refuses a widened home or instance tree with exit 1 and carries strict-modes-failed among its --json refusal codes, so the closed list of exit-1 strict-modes refusers must name it.
@row_set n=2 row=decision/home-tree-check-before-log-init cell=rule seen=1 detector=none
value: run, revive's start arm, adopt's start and viola verify check the home tree's modes before their log opens; a refusal writes no line, starts no child, leaves modes as found and exits 1.
basis: fct-zdl3zlbi, fct-gnvnr36o
because: adopt runs its strict-modes check before open_wrapper_log and a widened home is refused before any log with modes left as found, so the list of starts holding this rule gains adopt's.
@row_set n=3 row=decision/home-tree-check-before-start-log cell=rule seen=1 detector=none
value: The start of run, of viola revive and of viola adopt checks the home tree (the home, instance set, ledger, bin, plugin, pinned exe and plugin hooks) before its log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
basis: fct-zdl3zlbi, fct-gnvnr36o
because: adopt's start is a new start that checks the tree before its run log opens and refuses with exit 1 and no line, so the row naming the starts must include it.
@row_set n=4 row=decision/instance-check-before-log-init cell=rule seen=3 detector=none
value: Commands acting on an instance (revive's start arm, adopt's start, the hook arms, link, unlink, send, wait, last, answer, pause, release, focus) run the instance strict-modes check before opening any log; a refusal opens and writes nothing and leaves modes as found.
basis: fct-zdl3zlbi, fct-gnvnr36o
because: adopt's preflight runs check_instance before open_wrapper_log, so its start joins the commands that check the instance before any log.
@row_set n=5 row=contract/exit-1-detail/session-live cell=promise seen=1 detector=none
value: session-live is a distinct detail code for the exit-1 cause of viola revive and of viola adopt, naming no path or pid.
basis: fct-zdl3zlbi, fct-k2nhjvap
because: adopt now refuses a session open elsewhere with session-live, exit 1, so the code is no longer revive's alone.
@row_set n=6 row=contract/run-exit-1-detail/session-live cell=promise seen=1 detector=none
value: On a start made by viola revive or viola adopt, a distinct detail code.
basis: fct-zdl3zlbi, fct-k2nhjvap
because: adopt's start, logging as run, writes session-live on its refusal, so the row naming the starts that carry it must include adopt.
@row_set n=7 row=decision/session-live-check-fails-open cell=rule seen=1 fact=F3 detector=none
value: A failed, oversize or malformed claude agents read passes the session-live check of revive's and adopt's starts; the check is skipped under revive's --fork.
basis: fct-zdl3zlbi, fct-k2nhjvap
because: adopt shares revive's live_elsewhere check and starts when its agents read fails, so the fail-open rule now covers both starts.
@row_set n=8 row=contract/diag-subject/agents-probe cell=promise seen=1 detector=none
value: Subject of the process-start and process-exit pair of the claude agents read, written by viola list, the session-live reading of viola revive and viola adopt, verify's Run B and the list tool of viola mcp into their role files.
basis: fct-slqf7dkf, fct-zdl3zlbi
because: adopt's session-live reading writes run's agents-probe lines, so the list of writers of this subject gains it.
@row_set n=9 row=contract/cli-internal-error/human-line cell=promise seen=1 fact=F5 detector=none
value: Without --json the cli catch site prints exactly error: internal error; run, revive's start, adopt's start, mcp and hook print neither.
basis: fct-zdl3zlbi
because: the role word adopt is now classified before clap and silent at the internal-error catch site, so the list of roles that print nothing there gains adopt's start.
@row_set n=10 row=decision/launch-line-untraced cell=rule seen=1 detector=none
value: The launch line written at every run, revive and adopt start is log-only product audit data: it wakes no wait and opens no span, writes no process-log line and no terminal byte of its own.
basis: fct-k3h7lbx2, fct-zdl3zlbi
because: adopt's start writes the launch line through run::start and adds no span or line of its own, so the starts the rule names gain adopt.
@row_set n=11 row=decision/lineage-line-untraced cell=rule seen=1 detector=none
value: The lineage line written at every run, revive and adopt start is log-only product audit data: it wakes no wait and writes no process-log line, span or terminal byte of its own.
basis: fct-k3h7lbx2, fct-zdl3zlbi
because: adopt's start writes the lineage line of its given or inherited parent and adds no line or span of its own, so the starts the rule names gain adopt.
@row_add n=1 row=decision/adopt-start-logs-as-run area=span-trace-coverage fact=F1 detector=D-obs-instrumentation
name: viola adopt start logs as run
rule: The start of viola adopt is not a cli verb and logs as process run into the run log of its name, through run's own lines and spans, adding no event, detail word, field, subject or span of its own.
basis: fct-zdl3zlbi, fct-slqf7dkf
because: viola adopt is a new start whose instrumentation is run's own process-start, process-exit and agents-probe lines and run.* spans, and no row says which role and file it logs under.
@row_add n=2 row=contract/process-exit-detail/exit-1-adopt area=log-coverage fact=F4 detector=none
boundary: process-exit detail
item: exit 1 from viola adopt
promise: viola adopt's preflight ends exit 1 with one process-exit detail session-live and no child started; its strict-modes-failed refusal comes before any log and writes no line.
basis: fct-zdl3zlbi, fct-k2nhjvap, fct-gnvnr36o
because: adopt writes session-live on its own exit-1 process-exit while its strict-modes-failed refusal writes no line, and the standing exit-1 list cannot take it within its bound.
@row_add n=3 row=decision/adopt-preflight-order area=log-coverage fact=F2 detector=none
name: Adopt start preflight order
rule: Adopt's start checks the home tree's strict modes, then the pinned exe, then opens its run log, then reads the name holder, then the session-live check, then launches, and stops at the first refusal.
basis: fct-zdl3zlbi, fct-56rsq5py
because: adopt's preflight is a new ordered sequence, held by its adopt-order control, that decides which refusals are logged, and only revive's order stands as a row.
@row_add n=4 row=decision/adopt-session-id-never-logged area=pii-scrubbing-compliance fact=F6 detector=D-obs-pii
name: Adopt's typed session id never logged
rule: The session id an operator types to viola adopt never appears in any log line, field pair or detail.
basis: fct-slqf7dkf
because: adopt brings a new operator-typed id as external input, and the report holds it absent from every line, which no standing row says.
