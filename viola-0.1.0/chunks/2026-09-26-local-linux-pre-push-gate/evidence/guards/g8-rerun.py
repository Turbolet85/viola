"""g8 re-run after the sync test gained its stray-file case. The first g8 reading (readings.json,
`g8-clean`) was a broken pair: `reset --hard` already drops a file the clone's own `add -A` staged,
so the test never needed `clean`. Same cycle as guards.py; appended to readings.json."""
import json
import os

import guards

g8 = [g for g in guards.GUARDS if g[0] == "g8-clean"][0]
row = guards.cycle(*g8, suffix="-2")
path = os.path.join(guards.HERE, "readings.json")
readings = json.load(open(path))
readings.append(row)
json.dump(readings, open(path, "w", newline="\n"), indent=2)
print(f"{row['guard']}: neutralised exit {row['neutralised']['exit']} · restored exit "
      f"{row['restored']['exit']} · pair {'ok' if row['pair_ok'] else 'BROKEN'}")
