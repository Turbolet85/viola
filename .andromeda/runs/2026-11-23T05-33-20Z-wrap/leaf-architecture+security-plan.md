@leaf_group group=architecture+security-plan
@leaf_read leaf=.claude/rules/api.md lines=41
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/api.md line=20 class=stale
leaf-text: its `ok` payload is `{items, skipped}` in the shapes of the `list` payload
source: ctr-t5xneaha «The `list` `ok` payload, CLI `--json` and MCP `list` alike, is `{items, links?, skipped}`; `list` is not a channel method and needs no live wrapper.»
replace-with: its `ok` payload is `{items, links?, skipped}` in the shapes of the `list` payload
@leaf_edit n=2 leaf=.claude/rules/api.md line=29 class=omission
leaf-text: 2 usage (clap, a second positional to `viola send` included; never from `viola hook`)
source: ctr-jtlmr6fa «viola run --parent <NAME> declares the instance's parent; a parent equal to the instance's own name is a usage error, exit 2, before anything is read;»
replace-with: 2 usage (clap, a second positional to `viola send` included, and a `viola run --parent` equal to the instance's own name, before anything is read; never from `viola hook`)
@leaf_edit n=3 leaf=.claude/rules/api.md line=30 class=omission
leaf-text: `ViolaName` arguments via `value_parser = parse_viola_name`;
source: dec-rbodyykv «viola run's --parent goes through the viola name check and --role must be a code word; a --parent equal to the instance's own name is a usage error, exit 2, before anything is read.»
replace-with: `ViolaName` arguments via `value_parser = parse_viola_name` (`viola run --parent` included, its `--role` a code word);
@leaf_stands n=1 leaf=.claude/rules/api.md line=24
because: The line states the `release` param faults and their `channel-response` details, and names neither a `viola list` column nor the dropped `in v1` qualifier, so the reworded columns row bears on nothing it says.
