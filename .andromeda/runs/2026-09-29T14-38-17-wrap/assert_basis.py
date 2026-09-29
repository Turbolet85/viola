"""P7.1 ASSERT basis: every failure of the light gate's red entries (7, 8, 12) against the union of the same-day
2d8bc53 control runs, plus pre-push's linux leg. Reads logs outside the tree; prints codes and names only."""
import json, sys

SCRATCH = sys.argv[1]
LOGS = sys.argv[2]


def doc(path, cmd):
    raw = open(path, "rb").read().decode("utf-8", "replace")
    return json.loads([l for l in raw.splitlines() if l.startswith('{"v":1,"cmd":"' + cmd + '"')][-1])


def fails(d):
    return {f for s in d["suites"] for f in s.get("failures", [])}


control = set()
for name in ("ctl-e7-r1.log", "ctl-e8-r1.log", "ctl-cov-r1.log", "ctl-cov-r2.log"):
    control |= fails(doc(SCRATCH + "/" + name, "run"))
print("control union (4 runs):", len(control))

for n in ("7", "8"):
    d = doc(LOGS + "/" + n + ".log", "run")
    f = fails(d)
    print("entry", n, [(s["suite"], s["passed"], s["failed"]) for s in d["suites"]],
          "| not in control union:", sorted(f - control))

pp = doc(LOGS + "/12.log", "pre-push")
lin = pp["linux"]
print("entry 12 stage", pp.get("stage"), "| linux run ok", lin["run"]["ok"],
      [(s["suite"], s["passed"], s["failed"]) for s in lin["run"]["suites"]],
      "| browser ok", lin["browser"]["ok"], "| gate ok", lin["gate"]["ok"], "| vm", pp.get("vm", {}).get("terminated"))
w = pp["windows"]["run"]["suites"][0]
print("entry 12 windows", w["suite"], w["passed"], w["failed"], "| not in control union:", sorted(set(w["failures"]) - control))
