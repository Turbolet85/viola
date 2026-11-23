@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=46
last: - 2026-11-21: A promise that a value reaches no fixture cove
@leaf_edit n=1 leaf=.claude/rules/security.md line=19 class=omission
leaf-text: `from` is self-reported, never identity.
source: dec-otvnh2p3 «Self-reported parent authenticates nothing — An instance's parent is read as the parent only — never the instance, never an identity, never joined into a path — taken from --parent or else the inherited VIOLA_NAME»
replace-with: `from` is self-reported, never identity; likewise an instance's parent, taken from `--parent` or else the inherited `VIOLA_NAME` and dropped when it is the instance's own name, is read as the parent only — never the instance, never an identity, never joined into a path — and authenticates nothing.
@leaf_edit n=2 leaf=.claude/rules/security.md line=22 class=omission
leaf-text: (client checks and schemars schemas are advisory).
source: dec-rbodyykv «viola run's lineage flags checked before anything is read — viola run's --parent goes through the viola name check and --role must be a code word; a --parent equal to the instance's own name is a usage error, exit 2»
replace-with: (client checks and schemars schemas are advisory). `viola run`'s `--parent` goes through the viola name check and `--role` must be a code word; a `--parent` equal to the instance's own name is a usage error, exit 2, before anything is read.
@leaf_edit n=3 leaf=.claude/rules/security.md line=14 class=omission
leaf-text: the whole board refused, exit 21;
source: dec-ihvcipcw «Board lineage and links shown only when they re-parse — The board shows a recorded parent only when it re-parses as a viola name other than the row's own, a role only when it is a code word, and a link pair only when both names parse»
replace-with: the whole board refused, exit 21; the board shows a recorded parent only when it re-parses as a viola name other than the row's own, a role only when it is a code word, and a link pair only when both names parse, a gone instance giving no links;
@leaf_edit n=4 leaf=.claude/rules/security.md line=41 class=omission
leaf-text: when an instance resolves.
source: dec-cngvoifp «Lineage kept to the snapshot and the event log — An instance's lineage (parent and role names) is kept only in the snapshot and one lineage line of the event log at start, with no process-log line, span or terminal byte.»
replace-with: when an instance resolves. An instance's lineage (parent and role names) is kept only in the snapshot and one lineage line of the event log at start, with no process-log line, span or terminal byte.
@leaf_stands n=1 leaf=.claude/rules/security.md line=6
because: The line's "v1" scopes the secrets list, not the `viola list` columns, and ctr-q5dspexy's dropped "in v1" bears on no claim the line makes.
@leaf_stands n=2 leaf=.claude/rules/security.md line=25
because: The line's `events.ndjson` mentions concern StopFailure turn-end data and option text crossings, which none of the changed rows (revive keeps lineage, replay adds lineage, start order adds a lineage line, link names wording narrowed) contradicts.
@leaf_stands n=3 leaf=.claude/rules/security.md line=26
because: Revive's strict-modes check over `snapshot.json` and `events.ndjson`, its cwd, resume id and preflight are untouched by dec-zmmfuujl now adding that a revive keeps the recorded parent and role, nor by the other rows' lineage additions.
