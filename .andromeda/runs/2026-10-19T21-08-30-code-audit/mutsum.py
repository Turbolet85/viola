#!/usr/bin/env python3
"""C1 summarizer: one unit's complete mutants.out/ -> c-mutation-{unit}.json.

usage: mutsum.py <run_dir> <unit> <mutants.out dir> <scope> <jobs> <cap_s> [<files,...>]
Completion by the tool's own markers: outcomes.json end_time set AND total_mutants == len(mutants.json).
Score = caught/(caught+missed); missed whose whole span the host's build excludes = not_measured (collectors.md pinned cover()).
"""
import json, os, re, subprocess, sys
from datetime import datetime

REPO = os.getcwd()


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
            while j < n and src[j] != '"': j += 2 if src[j] == chr(92) else 1
            blank(i, j + 1); i = j + 1
        elif src[i] == "'" and (m := re.match(r"'(" + chr(92) * 2 + r".[^']*|[^" + chr(92) * 2 + r"'])'", src[i:i+12])): blank(i, i + m.end()); i += m.end()
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
        item = bool(k) and k.group(2) in ITEM
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


def file_excluded(repo, rel, cfg, seen=()):
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
        return file_excluded(repo, c.replace(chr(92), '/'), cfg, seen + (rel,))
    return None


def cover(repo, mutant, cfg):
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


def ts(s):
    return datetime.fromisoformat(s.rstrip('Z')[:26])


def summarize(run_dir, unit, mo, scope, jobs, cap_s, files=None, src_repo=REPO):
    o = json.load(open(os.path.join(mo, 'outcomes.json')))
    ml = json.load(open(os.path.join(mo, 'mutants.json')))
    if not o.get('end_time') or o['total_mutants'] != len(ml):
        return None  # incomplete by the tool's own markers
    base = o['outcomes'][0]
    if base['scenario'] != 'Baseline' or base['summary'] != 'Success':
        state = 'baseline-test-failure' if o['total_mutants'] == 0 or base['summary'] != 'Success' else None
    cfg = host_cfg(REPO)
    caught = missed = nm = tmo = unv = 0
    survivors, not_measured = [], []
    for x in o['outcomes'][1:]:
        m = x['scenario']['Mutant']
        site, _, text = m['name'].partition(': ')
        s = x['summary']
        if s == 'CaughtMutant': caught += 1
        elif s == 'Unviable': unv += 1
        elif s == 'Timeout': tmo += 1
        elif s == 'MissedMutant':
            p = cover(src_repo, m, cfg)
            if p:
                nm += 1; not_measured.append([site, text, f'cfg({p.strip()})'])
            else:
                missed += 1; survivors.append([site, text])
        else:
            raise SystemExit(f'unknown summary {s}')
    tested = len(o['outcomes']) - 1
    assert tested == o['total_mutants'] == len(ml), (tested, o['total_mutants'], len(ml))
    assert caught + missed + nm + tmo + unv == tested
    assert missed + nm == o['missed'], (missed, nm, o['missed'])
    assert len({(a, b) for a, b in survivors}) == len(survivors) == missed
    assert len({(a, b) for a, b, _ in not_measured}) == len(not_measured) == nm
    dominant = unv > caught + missed + nm + tmo
    score = None if dominant or caught + missed == 0 else round(100 * caught / (caught + missed), 2)
    wall = round((ts(o['end_time']) - ts(o['start_time'])).total_seconds())
    timing = {'scope': scope, 'planned': len(ml), 'tested': tested, 'wall_s': wall, 'jobs': jobs, 'cap_s': cap_s}
    if files:
        timing['files'] = sorted({m['file'] for m in ml})
    d = {'unit': unit, 'unit_state': (f'unviable-dominant {unv}/{tested}' if dominant else f"complete {o['total_mutants']}/{len(ml)}"),
         'score': score, 'counts': {'mutants': tested, 'caught': caught, 'missed': missed, 'not_measured': nm, 'timeout': tmo, 'unviable': unv},
         'survivors': sorted(survivors), 'not_measured': sorted(not_measured), 'timing': timing,
         'score_formula': 'caught/(caught+missed)', 'tool_missed_tally': o['missed']}
    json.dump(d, open(os.path.join(run_dir, f'c-mutation-{unit}.json'), 'w'), indent=1)
    return d


if __name__ == '__main__':
    rd, unit, mo, scope, jobs, cap = sys.argv[1:7]
    files = sys.argv[7].split(',') if len(sys.argv) > 7 else None
    d = summarize(rd, unit, mo, scope, int(jobs), int(cap), files)
    print(unit, 'INCOMPLETE' if d is None else (d['unit_state'], d['score'], d['counts']))
