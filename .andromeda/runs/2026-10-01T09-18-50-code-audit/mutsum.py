"""C1 per-unit summarizer (collectors.md C1 + §Host-excluded mutants, the pinned recipe verbatim).
usage: mutsum.py {unit} {output dir} {exit code} {jobs}  -> c-mutation-{unit}.json only for a COMPLETE invocation."""
import json, os, re, subprocess, sys

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

# ---- per-unit tally ------------------------------------------------------------------------------
def mutant_text(m):
    """the tool's own `name` minus its `file:line:col: ` site prefix."""
    name = m.get('name') or ''
    return re.sub(r'^[^:]+:\d+:\d+: ', '', name.replace('\\', '/'))

def site(m):
    return f"{m['file'].replace(chr(92), '/')}:{m['span']['start']['line']}:{m['span']['start']['column']}"

def summarize(repo, unit, out, rc, jobs):
    mo = os.path.join(out, 'mutants.out')
    oc = json.load(open(os.path.join(mo, 'outcomes.json'), encoding='utf-8'))
    planned = json.load(open(os.path.join(mo, 'mutants.json'), encoding='utf-8')) if os.path.isfile(os.path.join(mo, 'mutants.json')) else None
    total = oc.get('total_mutants')
    complete = bool(oc.get('end_time')) and (planned is None or total == len(planned))
    res = {'unit': unit, 'exit': rc, 'jobs': jobs, 'total_mutants': total,
           'planned_len': None if planned is None else len(planned), 'end_time': oc.get('end_time')}
    outs = oc.get('outcomes', [])
    base = outs[0] if outs else None
    base_ok = bool(base) and base.get('scenario') == 'Baseline' and base.get('summary') == 'Success'
    if not complete:
        res['state'] = 'incomplete'
        if not base_ok and not total:
            res['state'] = 'baseline-test-failure'
        print(json.dumps(res)); return None
    cfg = host_cfg(repo)
    c = {'mutants': 0, 'caught': 0, 'missed': 0, 'not_measured': 0, 'timeout': 0, 'unviable': 0}
    survivors, nm, timeouts = [], [], []
    for o in outs[1:]:
        m = o['scenario']['Mutant']
        s = o['summary']
        c['mutants'] += 1
        if s == 'CaughtMutant': c['caught'] += 1
        elif s == 'Unviable': c['unviable'] += 1
        elif s == 'Timeout': c['timeout'] += 1; timeouts.append([site(m), mutant_text(m)])
        elif s == 'MissedMutant':
            p = cover(repo, m, cfg)
            if p: c['not_measured'] += 1; nm.append([site(m), mutant_text(m), f'cfg({p})'])
            else: c['missed'] += 1; survivors.append([site(m), mutant_text(m)])
        else: raise SystemExit(f'unknown outcome summary {s!r}')
    tool_missed = oc.get('missed')
    assert c['missed'] + c['not_measured'] == tool_missed, (c, tool_missed)
    assert len({tuple(r) for r in survivors}) == len(survivors) == c['missed'], 'survivor key coarser than the tool'
    assert len({tuple(r) for r in nm}) == len(nm) == c['not_measured']
    tested_other = c['caught'] + c['missed'] + c['not_measured'] + c['timeout']
    unviable_dominant = c['unviable'] > tested_other
    scored = c['caught'] + c['missed']
    res.update({'state': 'unviable-dominant' if unviable_dominant else 'complete',
                'unit_state': (f"unviable-dominant {c['unviable']}/{c['mutants']}" if unviable_dominant
                               else f"complete {c['mutants']}/{len(planned) if planned is not None else total}"),
                'counts': c, 'score': None if unviable_dominant or not scored else round(100 * c['caught'] / scored, 2),
                'score_formula': 'caught/(caught+missed)', 'survivors': survivors, 'not_measured': nm,
                'timeouts': timeouts, 'tool_tally': {k: oc.get(k) for k in ('caught', 'missed', 'timeout', 'unviable', 'success', 'failure')},
                'host': next(v for k, v in cfg if k == 'target_arch') + '-' + next(v for k, v in cfg if k == 'target_vendor') + '-'
                        + next(v for k, v in cfg if k == 'target_os') + '-' + next(v for k, v in cfg if k == 'target_env'),
                'start_time': oc.get('start_time')})
    return res

if __name__ == '__main__':
    unit, out, rc, jobs = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4])
    repo = os.getcwd()
    r = summarize(repo, unit, out, rc, jobs)
    if r:
        R = os.path.dirname(out)
        json.dump(r, open(os.path.join(R, f'c-mutation-{unit}.json'), 'w', encoding='utf-8', newline='\n'), ensure_ascii=False, indent=1)
        print(json.dumps({k: r[k] for k in ('unit', 'unit_state', 'counts', 'score')}))
