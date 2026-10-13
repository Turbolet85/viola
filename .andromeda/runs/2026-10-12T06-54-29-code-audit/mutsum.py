"""C1 summarizer: python mutsum.py <unit> [<output-dir-name>]  ->  c-mutation-<unit>.json

Reads <run_dir>/<output-dir>/mutants.out/{outcomes.json, mutants.json} only when the invocation is complete by
the tool's own markers (outcomes.json end_time set AND total_mutants == len(mutants.json)). Host-excluded
missed mutants by collectors.md's pinned cover recipe (verbatim below). Score = caught/(caught+missed).
"""
import datetime
import json
import os
import re
import subprocess
import sys

RUN = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(RUN, '..', '..', '..'))


# ---- collectors.md, Host-excluded mutants, pinned recipe (verbatim) ----
def host_cfg(repo):  # at the repo root: the project's pinned toolchain answers
    out = subprocess.run(['rustc', '--print', 'cfg'], cwd=repo, capture_output=True, text=True, check=True).stdout
    return {(k, v.strip('"') if v else None) for k, _, v in (l.partition('=') for l in out.splitlines())}


def ev(p, cfg):  # three-valued: True / False / None (UNKNOWN)
    p = p.strip()
    m = re.fullmatch(r'(all|any|not)\s*\((.*)\)', p, re.S)
    if m:
        args, depth, cur = [], 0, ''
        for ch in m.group(2):
            depth += (ch == '(') - (ch == ')')
            if ch == ',' and depth == 0: args.append(cur); cur = ''
            else: cur += ch
        vals = [ev(a, cfg) for a in args + [cur] if a.strip()]
        if m.group(1) == 'not': return None if len(vals) != 1 or vals[0] is None else not vals[0]
        if m.group(1) == 'all': return False if False in vals else None if None in vals else True
        return True if True in vals else None if None in vals else False
    m = re.fullmatch(r'([A-Za-z_]\w*)\s*(?:=\s*"([^"]*)")?', p)
    if not m: return None
    if (m.group(1), m.group(2)) in cfg: return True
    return False if m.group(1) in ('unix', 'windows') or m.group(1).startswith('target_') else None


def mask(src):  # comments, strings and char literals blanked; offsets and newlines kept
    out, i, n = list(src), 0, len(src)
    def blank(a, b):
        for k in range(a, b): out[k] = out[k] if src[k] == '\n' else ' '
    while i < n:
        if src.startswith('//', i): j = src.find('\n', i); j = n if j < 0 else j; blank(i, j); i = j
        elif src.startswith('/*', i):
            d, j = 1, i + 2
            while j < n and d: d += src.startswith('/*', j) - src.startswith('*/', j); j += 2 if src[j:j+2] in ('/*', '*/') else 1
            blank(i, j); i = j
        elif m := re.match(r'b?r(#*)"', src[i:i+300]) if (i == 0 or not (src[i-1].isalnum() or src[i-1] == '_')) else None:
            j = src.find('"' + m.group(1), i + m.end()); j = n if j < 0 else j + 1 + len(m.group(1)); blank(i, j); i = j
        elif src[i] == '"':
            j = i + 1
            while j < n and src[j] != '"': j += 2 if src[j] == '\\' else 1
            blank(i, j + 1); i = j + 1
        elif src[i] == "'" and (m := re.match(r"'(\\.[^']*|[^\\'])'", src[i:i+12])): blank(i, i + m.end()); i += m.end()
        else: i += 1
    return ''.join(out)


ITEM = {'pub', 'fn', 'impl', 'mod', 'struct', 'enum', 'union', 'trait', 'unsafe', 'async', 'const', 'static', 'type',
        'use', 'extern', 'macro_rules'}


def extents(src):  # [(start, end, P)]: the attribute .. the close of its node's first top-level `{`, or its `;`
    mk, res = mask(src), []
    for a in re.finditer(r'#(!?)\[\s*cfg\s*\(', mk):
        d, j = 1, a.end()
        while j < len(mk) and d: d += (mk[j] == '(') - (mk[j] == ')'); j += 1
        pred, j = src[a.end():j-1], mk.find(']', j) + 1
        if a.group(1): res.append((0, len(src), pred)); continue
        k = re.match(r'(\s*#\[[^\]]*\])*\s*(\w+)', mk[j:])
        item = bool(k) and k.group(2) in ITEM  # an item ends only at its block or `;` (a `,` may sit in `<…>`)
        d = 0
        while j < len(mk):
            c = mk[j]
            if c in '([': d += 1
            elif c in ')]': d -= 1
            elif d == 0 and c == '{':
                b = 1; j += 1
                while j < len(mk) and b: b += (mk[j] == '{') - (mk[j] == '}'); j += 1
                break
            elif d == 0 and (c == ';' or c == ',' and not item): j += 1; break
            elif d == 0 and c == '}': break
            j += 1
        res.append((a.start(), j, pred))
    return res


def off(src, line, col):  # cargo-mutants: 1-based line, 1-based char column
    starts = [0] + [k + 1 for k, c in enumerate(src) if c == '\n']
    return starts[line - 1] + col - 1


def file_excluded(repo, rel, cfg, seen=()):  # a false predicate on the `mod` bringing the file in
    d, stem = os.path.split(rel)
    name = os.path.basename(d) if stem == 'mod.rs' else stem[:-3]
    if name in ('lib', 'main') or rel in seen: return None
    parent_dir = os.path.dirname(d) if stem == 'mod.rs' else d
    cands = [os.path.join(parent_dir, x) for x in ('lib.rs', 'main.rs', 'mod.rs')] + [parent_dir + '.rs']
    for c in cands:
        try: src = open(os.path.join(repo, c), encoding='utf-8').read()
        except OSError: continue
        m = re.search(r'\bmod\s+' + re.escape(name) + r'\s*;', mask(src))
        if not m: continue
        for s, e, p in extents(src):
            if s <= m.start() and m.end() <= e and ev(p, cfg) is False: return p
        return file_excluded(repo, c.replace('\\', '/'), cfg, seen + (rel,))
    return None


def cover(repo, mutant, cfg):  # the predicate proving the host excludes the WHOLE span, else None
    try: src = open(os.path.join(repo, mutant['file']), encoding='utf-8').read()
    except OSError: return None
    fp = file_excluded(repo, mutant['file'], cfg)
    if fp: return fp
    sp = mutant['span']
    a = off(src, sp['start']['line'], sp['start']['column'])
    b = off(src, sp['end']['line'], sp['end']['column'])
    for s, e, p in extents(src):
        if s <= a and b <= e and ev(p, cfg) is False: return p
    return None
# ---- end of the pinned recipe ----


def ts(s):
    return datetime.datetime.fromisoformat(s.replace('Z', '+00:00')[:26] + '+00:00' if '.' in s else s.replace('Z', '+00:00'))


def main(unit, outdir):
    base = os.path.join(RUN, outdir, 'mutants.out')
    o = json.load(open(os.path.join(base, 'outcomes.json'), encoding='utf-8'))
    try:
        listed = len(json.load(open(os.path.join(base, 'mutants.json'), encoding='utf-8')))
    except OSError:
        listed = None
    sp = os.path.join(RUN, f"state-{unit}{'-x' if outdir.endswith('-x') else ''}.json")
    state = json.load(open(sp, encoding='utf-8')) if os.path.exists(sp) else {}
    complete = bool(o.get('end_time')) and (listed is None or o.get('total_mutants') == listed)
    res = {'unit': unit, 'output': outdir, 'complete': complete, 'total_mutants': o.get('total_mutants'),
           'listed': listed, 'unit_states_line': f"{o.get('total_mutants')}/{listed}",
           'start_time': o.get('start_time'), 'end_time': o.get('end_time')}
    outs = o['outcomes']
    base_out = outs[0] if outs and outs[0].get('scenario') == 'Baseline' else None
    res['baseline'] = None if not base_out else {
        'summary': base_out['summary'],
        'phases': [{'phase': p['phase'], 'duration_s': round(p['duration'], 1), 'status': p['process_status']}
                   for p in base_out.get('phase_results', [])]}
    if base_out and base_out['summary'] != 'Success' and o.get('total_mutants') == 0 and state.get('how') == 'exited':
        res['state'] = 'baseline-test-failure'  # an invocation that EXITED incomplete, classified by its exit
    elif not complete:
        res['state'] = 'incomplete'
    muts = [x for x in outs if isinstance(x.get('scenario'), dict) and 'Mutant' in x['scenario']]
    cfg = host_cfg(REPO)
    counts = {'mutants': len(muts), 'caught': 0, 'missed': 0, 'not_measured': 0, 'timeout': 0, 'unviable': 0}
    survivors, not_measured, timeouts = [], [], []
    for x in muts:
        m = x['scenario']['Mutant']
        site = f"{m['file']}:{m['span']['start']['line']}:{m['span']['start']['column']}"
        text = m['name'].split(': ', 1)[1] if ': ' in m['name'] else m['name']
        s = x['summary']
        if s == 'CaughtMutant':
            counts['caught'] += 1
        elif s == 'MissedMutant':
            p = cover(REPO, m, cfg)
            if p:
                counts['not_measured'] += 1
                not_measured.append([site, text, f'cfg({p.strip()})'])
            else:
                counts['missed'] += 1
                survivors.append([site, text])
        elif s == 'Timeout':
            counts['timeout'] += 1
            timeouts.append([site, text, [pr['phase'] for pr in x.get('phase_results', [])][-1:]])
        elif s == 'Unviable':
            counts['unviable'] += 1
    tool_missed = o.get('missed')
    assert counts['missed'] + counts['not_measured'] == tool_missed, (counts, tool_missed)
    assert len({(a, b) for a, b in survivors}) == len(survivors) == counts['missed']
    assert len({(a, b) for a, b, _ in not_measured}) == len(not_measured) == counts['not_measured']
    tested = len(muts)
    res['counts'] = counts
    res['tool_tallies'] = {k: o.get(k) for k in ('caught', 'missed', 'timeout', 'unviable')}
    res['survivors'] = sorted(survivors)
    res['not_measured'] = sorted(not_measured)
    res['timeouts'] = timeouts
    if complete and 'state' not in res:
        if counts['unviable'] > counts['caught'] + counts['missed'] + counts['not_measured'] + counts['timeout']:
            res['state'] = f"unviable-dominant {counts['unviable']}/{tested}"
        else:
            res['state'] = f'complete {tested}/{listed}'
            res['score'] = round(100 * counts['caught'] / (counts['caught'] + counts['missed']), 2) \
                if counts['caught'] + counts['missed'] else None
    if o.get('end_time') and o.get('start_time'):
        wall = int((ts(o['end_time']) - ts(o['start_time'])).total_seconds())
    else:
        wall = state.get('wall_s')
    res['timing'] = {'scope': 'unit', 'planned': state.get('planned', listed), 'tested': tested, 'wall_s': wall,
                     'jobs': state.get('jobs'), 'cap_s': state.get('cap_s')}
    res['host'] = subprocess.run(['rustc', '-vV'], cwd=REPO, capture_output=True, text=True).stdout.split(
        'host: ')[1].split()[0]
    res['score_formula'] = 'caught/(caught+missed)'
    with open(os.path.join(RUN, f'c-mutation-{unit}.json'), 'w', encoding='utf-8', newline='\n') as f:
        json.dump(res, f, ensure_ascii=False, indent=1)
        f.write('\n')
    print(json.dumps({k: res.get(k) for k in ('unit', 'state', 'score', 'counts', 'timing', 'baseline')}))
    print('survivors', len(survivors), 'not_measured', len(not_measured), 'timeouts', timeouts)


if __name__ == '__main__':
    main(sys.argv[1], sys.argv[2] if len(sys.argv) > 2 else f'mutants-{sys.argv[1]}')
