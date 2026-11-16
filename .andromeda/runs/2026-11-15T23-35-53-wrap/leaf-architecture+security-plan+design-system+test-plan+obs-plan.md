@leaf_group group=architecture+security-plan+design-system+test-plan+obs-plan
@leaf_read leaf=CLAUDE.md lines=133
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=43 class=stale
leaf-text: is a capability-ledger row in `viola-agent-claude` with a `viola verify` probe, never an unconditional assumption;
source: dec-udlphze4 «except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs; StopFailure is registered, provisionally, with no ledger row and no verify probe.»
replace-with: is a capability-ledger row in `viola-agent-claude` with a `viola verify` probe, never an unconditional assumption (except StopFailure, registered provisionally with no ledger row and no verify probe);
@leaf_edit n=2 leaf=CLAUDE.md line=85 class=stale
leaf-text: Every behaviour of the `claude` CLI it relies on is a measured, version-stamped capability-ledger row;
source: dec-zzhuidvp «except: StopFailure is registered, provisionally, with no ledger row and no verify probe.»
replace-with: Every behaviour of the `claude` CLI it relies on is a measured, version-stamped capability-ledger row (StopFailure alone is registered, provisionally, with no ledger row and no verify probe);
