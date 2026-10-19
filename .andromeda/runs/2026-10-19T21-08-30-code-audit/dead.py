#!/usr/bin/env python3
"""A5 summary: c-dead-raw.json (stages 1-2) + the stage-3 hand read at HEAD + cargo machete -> c-dead.json."""
import json, re, sys

R = sys.argv[1]
raw = json.load(open(f'{R}/c-dead-raw.json'))
PFX = re.compile(r'^rust-analyzer cargo \S+ \S+ ')
# Stage 3: each residual read at HEAD; family + witness line.
WIT = {
    'crates/viola-mcp/src/tools.rs': ('derive/attr-invoked', 'crates/viola-mcp/src/tools.rs:76 #[derive(JsonSchema)] (advertised MCP input schema; never deserialised into, line 74)'),
    'crates/viola-state/src/events.rs': ('derive/attr-invoked', 'crates/viola-state/src/events.rs:35 #[serde(serialize_with = "kind_str")]'),
    'crates/viola-state/src/lib.rs': ('derive/attr-invoked', 'crates/viola-state/src/lib.rs:26 Encode(#[from] serde_json::Error) (#[from] variant)'),
    'src/cmd/send.rs': ('derive/attr-invoked', 'src/cmd/send.rs:41 #[arg(hide = true, value_parser = refuse_text_argument)] (clap derive)'),
    'src/human/mirror.rs': ('inline format-string capture', 'src/human/mirror.rs:9 format!("{word:<MIRROR_WORD$}")'),
}
reviewed = []
for r in raw['residual']:
    fam, w = WIT[r['file']]
    reviewed.append({'symbol': PFX.sub('', r['symbol']), 'file': f"{r['file']}:{r['def_line']}", 'family': fam, 'witness': w})
left = [x for x in reviewed if not x['family']]
d = {
    'unused_deps': [['viola-mcp (crates/viola-mcp/Cargo.toml)', 'serde'], ['viola-fuzz (fuzz/Cargo.toml, own workspace)', 'arbitrary']],
    'zero_ref_candidates': len(left), 'top': left[:20],
    'classing': {'raw': raw['raw'], 'stage1_tests_excluded': raw['stage1_tests_excluded'], 'stage2_classes': raw['stage2_classes'],
                 'residual_reviewed_fp': reviewed},
    'note': 'candidates, never dead; recipe = recipes.dead (the baseline record), reused verbatim',
}
assert raw['raw'] == raw['stage1_tests_excluded'] + sum(raw['stage2_classes'].values()) + len(raw['residual'])
json.dump(d, open(f'{R}/c-dead.json', 'w'), indent=1)
print('dead raw', raw['raw'], 'reviewed fp', len(reviewed), 'candidates', len(left), 'unused deps', d['unused_deps'])
