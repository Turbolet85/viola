"""g9: the pump's baseline is the spawned size. Neutralise it back to a second host read (the defect),
run both regression tests red — the unit test (viola-pty) and the forced-window tui test — restore the
bytes (asserted identical), run both green. Appended to readings.json."""
import json
import os
import subprocess

import guards

REL = r"crates\viola-pty\src\lib.rs"
ANCHOR, NEUTRAL = "let mut last = Some(spawned);", "let mut last = host_size();"
RUNS = [
    ("unit", ["cargo", "nextest", "run", "-p", "viola-pty", "-E",
              "test(/pump_forwards_a_resize_that_lands_before_its_first_look/)"]),
    ("forced", ["cargo", "nextest", "run", "--workspace", "--features", "viola/fake-agent", "-E",
                "test(/tui_host_resize_in_the_pump_start_window_reaches_the_child/)"]),
]


def run_all(tag):
    env = dict(os.environ, CARGO_TARGET_DIR=os.path.join(guards.ROOT, "target", "check-pp"))
    out = {}
    for name, argv in RUNS:
        log = os.path.join(guards.HERE, f"g9-{name}.{tag}.log")
        with open(log, "wb") as f:
            proc = subprocess.run(argv, cwd=guards.ROOT, env=env, stdout=f, stderr=subprocess.STDOUT)
        text = open(log, encoding="utf-8", errors="replace").read()
        summary = [l.strip() for l in text.splitlines() if "Summary [" in l]
        out[name] = {"exit": proc.returncode, "summary": summary[-1] if summary else "no summary",
                     "signature": "the resize never reached the child" in text or "the resize was lost" in text}
    return out


path = os.path.join(guards.ROOT, REL)
original = open(path, "rb").read()
assert original.count(ANCHOR.encode()) == 1
open(path, "wb").write(original.replace(ANCHOR.encode(), NEUTRAL.encode(), 1))
try:
    red = run_all("neutralised")
finally:
    open(path, "wb").write(original)
assert open(path, "rb").read() == original
green = run_all("restored")
row = {"guard": "g9-pump-baseline", "file": REL.replace("\\", "/"),
       "tests": [argv[-1] for _, argv in RUNS], "neutralised": red, "restored": green,
       "pair_ok": all(r["exit"] != 0 and r["signature"] for r in red.values())
       and all(g["exit"] == 0 for g in green.values())}
readings_path = os.path.join(guards.HERE, "readings.json")
readings = json.load(open(readings_path))
readings.append(row)
json.dump(readings, open(readings_path, "w", newline="\n"), indent=2)
for name in red:
    print(f"g9 {name}: neutralised exit {red[name]['exit']} signature {red[name]['signature']} · restored exit {green[name]['exit']}")
print(f"g9 pair {'ok' if row['pair_ok'] else 'BROKEN'}")
