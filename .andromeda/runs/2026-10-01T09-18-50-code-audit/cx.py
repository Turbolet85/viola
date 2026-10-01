"""A2 complexity summarizer (pinned per collectors.md): per-FUNCTION cyclomatic + cognitive from
rust-code-analysis `-m -O json`; cognitive = metrics.cognitive.sum, cyclomatic = metrics.cyclomatic.sum
of every space whose kind is `function` (closures, which rca names `<anonymous>`, included or not per
--named). Nearest-rank p50/p90; over_ceiling = cognitive > 15; top-10 by cognitive."""
import json, math, os, sys

def funcs(rca_dir, named_only):
    out = []
    for root, _, files in os.walk(rca_dir):
        for f in files:
            if not f.endswith('.json'):
                continue
            d = json.load(open(os.path.join(root, f), encoding='utf-8'))
            path = d['name'].replace('\\', '/')
            stack = [d]
            while stack:
                s = stack.pop()
                if s.get('kind') == 'function' and not (named_only and s.get('name') == '<anonymous>'):
                    out.append({'fn': s['name'], 'file': f"{path}:{s['start_line']}",
                                'path': path,
                                'cog': s['metrics']['cognitive']['sum'],
                                'cyc': s['metrics']['cyclomatic']['sum']})
                stack.extend(s.get('spaces', []))
    return out

def nr(vals, p):
    v = sorted(vals)
    return v[max(0, math.ceil(p / 100 * len(v)) - 1)] if v else None

def summary(fs):
    cog = [f['cog'] for f in fs]
    cyc = [f['cyc'] for f in fs]
    top = sorted(fs, key=lambda f: (-f['cog'], f['file']))[:10]
    return {'functions': len(fs),
            'cyclomatic_p50': nr(cyc, 50), 'cyclomatic_p90': nr(cyc, 90),
            'cognitive_p50': nr(cog, 50), 'cognitive_p90': nr(cog, 90),
            'over_ceiling': sum(1 for c in cog if c > 15),
            'max': {'fn': top[0]['fn'], 'file': top[0]['file'], 'val': top[0]['cog']} if top else None,
            'top': [{'fn': f['fn'], 'file': f['file'], 'value': f['cog']} for f in top]}

if __name__ == '__main__':
    rca_dir, named = sys.argv[1], sys.argv[2] == 'named'
    fs = funcs(rca_dir, named)
    s = summary(fs)
    if len(sys.argv) > 3:
        json.dump(s, open(sys.argv[3], 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
        # per-file max cognitive for B2 hotspots
        per = {}
        for f in fs:
            per[f['path']] = max(per.get(f['path'], 0), f['cog'])
        json.dump(per, open(sys.argv[3].replace('.json', '-perfile.json'), 'w', encoding='utf-8', newline='\n'), indent=0)
    print(json.dumps({k: v for k, v in s.items() if k != 'top'}, ensure_ascii=False))
