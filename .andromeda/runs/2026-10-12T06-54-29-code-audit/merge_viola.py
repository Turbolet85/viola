"""Merge of the viola unit's two invocations, then mutsum.py over the merge (unchanged).

Inputs: mutants-viola-partial/mutants.out (the stopped first invocation, 678/1299, never written to) and
mutants-viola-iter/mutants.out (the --iterate resume). The resume must be COMPLETE by the tool's own markers
(end_time set AND total_mutants == len(its mutants.json)), else nothing is written. The merge keeps the first
invocation's baseline and its CaughtMutant / Unviable outcomes (the ones --iterate skipped) and every outcome of the
resume (the 621 untested and the 15 re-tested missed); the first invocation's 15 missed are superseded. Asserted:
the two kept sets are disjoint, their union equals the 1299 listing by (file, span, replacement, genre, function),
and the first invocation's mutants.json equals list-viola.json. Writes mutants-viola-merged/mutants.out/{outcomes,
mutants}.json, then c-mutation-viola.json by mutsum.main, its timing replaced by the two invocations' summed walls.
"""
import json
import os
import sys

RUN = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, RUN)
import mutsum  # noqa: E402

P = os.path.join(RUN, 'mutants-viola-partial', 'mutants.out')
I = os.path.join(RUN, 'mutants-viola-iter', 'mutants.out')
M = os.path.join(RUN, 'mutants-viola-merged', 'mutants.out')
FIRST_WALL, FIRST_CAP = 6596, 31737  # 07:17:31Z to the stop at 09:07:27Z; the unit's recorded cap


def load(p):
    return json.load(open(p, encoding='utf-8'))


def key(m):
    return (m['file'], m['span']['start']['line'], m['span']['start']['column'], m['span']['end']['line'],
            m['span']['end']['column'], m.get('replacement'), m.get('genre'), json.dumps(m.get('function'), sort_keys=True))


def mutant(o):
    sc = o.get('scenario')
    return sc['Mutant'] if isinstance(sc, dict) and 'Mutant' in sc else None


full = load(os.path.join(RUN, 'list-viola.json'))
po, pm = load(os.path.join(P, 'outcomes.json')), load(os.path.join(P, 'mutants.json'))
io_, im = load(os.path.join(I, 'outcomes.json')), load(os.path.join(I, 'mutants.json'))
if not (io_.get('end_time') and io_.get('total_mutants') == len(im)):
    sys.exit(f"resume incomplete: end_time={io_.get('end_time')} total_mutants={io_.get('total_mutants')} listed={len(im)}")
assert [key(m) for m in pm] == [key(m) for m in full], 'partial mutants.json differs from list-viola.json'
assert po['outcomes'][0]['scenario'] == 'Baseline' and po['outcomes'][0]['summary'] == 'Success'
kept = [o for o in po['outcomes'][1:] if o['summary'] in ('CaughtMutant', 'Unviable')]
new = [o for o in io_['outcomes'] if mutant(o)]
ks, ns = {key(mutant(o)) for o in kept}, {key(mutant(o)) for o in new}
assert len(ks) == len(kept) == 663 and len(ns) == len(new) == len(im) == 636, (len(kept), len(new), len(im))
assert not ks & ns, 'overlap'
assert ks | ns == {key(m) for m in full}, 'union differs from the listing'
outs = [po['outcomes'][0]] + kept + new
tally = {s: sum(o['summary'] == s for o in outs[1:]) for s in ('CaughtMutant', 'MissedMutant', 'Timeout', 'Unviable')}
merged = dict(po)
merged.update(outcomes=outs, total_mutants=len(outs) - 1, caught=tally['CaughtMutant'], missed=tally['MissedMutant'],
              timeout=tally['Timeout'], unviable=tally['Unviable'], success=po.get('success', 0),
              start_time=po['start_time'], end_time=io_['end_time'])
os.makedirs(M)
with open(os.path.join(M, 'outcomes.json'), 'w', encoding='utf-8', newline='\n') as f:
    json.dump(merged, f)
with open(os.path.join(M, 'mutants.json'), 'w', encoding='utf-8', newline='\n') as f:
    json.dump(pm, f)
mutsum.main('viola', 'mutants-viola-merged')
cp = os.path.join(RUN, 'c-mutation-viola.json')
res = load(cp)
iw = int((mutsum.ts(io_['end_time']) - mutsum.ts(io_['start_time'])).total_seconds())
res['timing'] = {'scope': 'unit', 'planned': len(full), 'tested': res['counts']['mutants'], 'wall_s': FIRST_WALL + iw,
                 'jobs': 2, 'cap_s': FIRST_CAP}
res['merged_from'] = {
    'first': {'output': 'mutants-viola-partial', 'start_time': po['start_time'], 'stopped': '2026-10-12T09:07:27Z',
              'wall_s': FIRST_WALL, 'outcomes': len(po['outcomes']) - 1, 'kept_caught_unviable': len(kept),
              'superseded_missed': sum(o['summary'] == 'MissedMutant' for o in po['outcomes'][1:]),
              'outcomes_sha256': '7439ee703ae08ec18b0e658728a8e3a4d7432bec9e9cb6d3fe55c575fa411c07'},
    'resume': {'output': 'mutants-viola-iter', 'flag': '--iterate', 'start_time': io_['start_time'],
               'end_time': io_['end_time'], 'wall_s': iw, 'total_mutants': io_['total_mutants'], 'listed': len(im),
               'baseline': io_['outcomes'][0]['summary'] if io_['outcomes'] and io_['outcomes'][0]['scenario'] == 'Baseline' else None},
    'note': 'start_time/end_time above the merge span the stop; wall_s is the two invocations summed'}
with open(cp, 'w', encoding='utf-8', newline='\n') as f:
    json.dump(res, f, ensure_ascii=False, indent=1)
    f.write('\n')
print('timing', res['timing'])
