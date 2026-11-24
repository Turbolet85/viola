@row_set n=1 row=contract/cli-verbs/help cell=promise seen=4 detector=D-design-cli-wording
value: The grouped verb table with the served verbs only: the board, traffic, wheel, handoff and setup rows, `board:` holding list and focus, `handoff:` holding link, unlink and `setup:` holding open, run, revive, adopt, up, verify and plugin install.
basis: fct-zdl3zlbi
because: The help's `setup:` row now reads `setup:    open, run, revive, adopt, up, verify, plugin install`, so the row that lists its verbs must name adopt.
@row_retire n=1 row=contract/cli-catch-site/internal-error seen=1 heir=contract/cli-catch-site-outside-revive-adopt/internal-error detector=D-design-cli-wording
why: The boundary names only the revive start as outside the catch site's printed line, while the adopt start is now silent there too.
basis: fct-zdl3zlbi
because: `adopt` is a wrapper's named role silent at the catch site, so the boundary cell, which the key is made from, must widen through an heir.
@row_add n=1 row=contract/cli-catch-site-outside-revive-adopt/internal-error area=surface-cli detector=D-design-cli-wording
boundary: CLI verbs outside a revive or adopt start
item: internal error
promise: A failed or panicked verb prints exactly `error: internal error` once, from the one catch site, with no hint; with `--json` before a `--` it prints `{"v":1,"error":"internal-error","detail":null}` on stdout instead, stderr empty.
basis: fct-zdl3zlbi
because: The adopt start joins the revive start as silent at the internal-error catch, so the catch-site promise holds for the verbs outside both.
@row_add n=2 row=contract/cli-verbs/adopt area=surface-cli fact=F1 detector=D-layout-surface
boundary: CLI verbs
item: viola adopt <SESSION_ID> --name <NAME>
promise: Resumes a plain claude session by its session id under viola with the given name and parent, in the current folder, with no window, logging as process run.
basis: fct-zdl3zlbi
because: The chunk adds the `viola adopt` verb to the CLI surface, and the verb list holds one row a verb.
@row_add n=3 row=contract/viola-adopt/usage area=surface-cli fact=F4 detector=D-design-cli-wording
boundary: viola adopt
item: usage
promise: `viola adopt <SESSION_ID> --name <NAME> [--parent <NAME>] [--model <MODEL>] [--effort <EFFORT>] [--json]`; the id is the session-id shape revive's `--id` takes, hex of either case; a malformed id, any word after `--`, or a `--parent` equal to `--name` is a usage exit 2.
basis: fct-zdl3zlbi
because: The new verb's arguments and usage exits are a contract at the CLI boundary that no row states yet.
@row_add n=4 row=contract/viola-adopt-refusals/session-live area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: viola adopt refusals
item: session open in another claude
promise: A session a live claude holds is refused `session-live`, exit 1, with `unable: that session is open in another claude` / `hint: close it there first, then adopt it again`.
basis: fct-zdl3zlbi
because: The chunk adds a new `unable:` / `hint:` pair for adopt's live-elsewhere refusal, and the masters state every refusal wording.
@row_add n=5 row=contract/viola-adopt/preflight-order area=surface-cli fact=F5 detector=none
boundary: viola adopt start
item: preflight order
promise: The start reads its preflight in a fixed order: the strict-modes check, the pinned exe, the wrapper log, the live-name holder, then the live-elsewhere check, then the resume launch.
basis: fct-zdl3zlbi
because: The adopt start's preflight order is a fixed contract beside revive's own, and no row states it.
@row_add n=6 row=contract/viola-adopt-json/strict-modes-failed area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola adopt --json error codes
item: strict-modes-failed
promise: The code for a refused home tree or instance state files.
basis: fct-spvlmkow
because: Adopt's `--json` refusal codes are a closed list at the CLI boundary, one row an item, and this is one of its own two.
@row_add n=7 row=contract/viola-adopt-json/session-live area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola adopt --json error codes
item: session-live
promise: The code for a session a live claude holds.
basis: fct-spvlmkow
because: Adopt's `--json` refusal codes are a closed list at the CLI boundary, one row an item, and this is one of its own two.
@row_add n=8 row=contract/viola-adopt-json/already-live area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola adopt --json error codes
item: already-live
promise: One of the codes a `viola adopt --json` refusal carries, shared with `viola run`.
basis: fct-spvlmkow
because: Adopt's `--json` refusal list includes run's shared codes, one row an item.
@row_add n=9 row=contract/viola-adopt-json/stale-heartbeat area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola adopt --json error codes
item: stale-heartbeat
promise: One of the codes a `viola adopt --json` refusal carries, shared with `viola run`.
basis: fct-spvlmkow
because: Adopt's `--json` refusal list includes run's shared codes, one row an item.
@row_add n=10 row=contract/viola-adopt-json/pinned-hash-mismatch area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola adopt --json error codes
item: pinned-hash-mismatch
promise: One of the codes a `viola adopt --json` refusal carries, shared with `viola run`.
basis: fct-spvlmkow
because: Adopt's `--json` refusal list includes run's shared codes, one row an item.
@row_add n=11 row=contract/viola-adopt-json/squatted-name area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola adopt --json error codes
item: squatted-name
promise: One of the codes a `viola adopt --json` refusal carries, shared with `viola run`.
basis: fct-spvlmkow
because: Adopt's `--json` refusal list includes run's shared codes, one row an item.
@row_add n=12 row=contract/viola-adopt-json/socket-dir-refused area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola adopt --json error codes
item: socket-dir-refused
promise: One of the codes a `viola adopt --json` refusal carries, shared with `viola run`.
basis: fct-spvlmkow
because: Adopt's `--json` refusal list includes run's shared codes, one row an item.
@row_add n=13 row=contract/viola-adopt-json/internal-error area=surface-cli fact=F3 detector=D-design-cli-wording
boundary: viola adopt --json error codes
item: internal-error
promise: One of the codes a `viola adopt --json` refusal carries, shared with `viola run`; an error of the start before the child runs prints this document.
basis: fct-zdl3zlbi
because: Adopt's `--json` refusal list includes run's shared internal-error code, printed for an error before the child pump.
@row_add n=14 row=contract/viola-help/setup-row area=surface-cli detector=D-design-cli-wording
boundary: viola --help output
item: setup row
promise: The `setup:` verb group row prints exactly `setup:    open, run, revive, adopt, up, verify, plugin install`, `adopt` after `revive`.
basis: fct-zdl3zlbi
because: The printed `setup:` row gained `adopt`, and the verb-groups row cannot take the new literal within its bound.
