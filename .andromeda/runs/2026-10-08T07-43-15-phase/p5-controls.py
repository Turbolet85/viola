"""P5 of the second revision: the known-verdict controls of the two reply-probe reader entries.
It mints control ledgers beside itself from the committed pre-fix record, reads each through the entry's own
`run` text (taken from the plan's fence, the ledger path swapped), and prints the exits. Run from the repo root.
"""
import json
import os
import re
import subprocess
import sys
import tomllib

CHUNK = "viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive"
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "p5-controls")
os.makedirs(OUT, exist_ok=True)

plan = open(os.path.join(CHUNK, "plan.md"), encoding="utf-8").read()
sec = plan.split("## Test Commands", 1)[1]
fence = re.search(r"```toml\n(.*?)\n```", sec, re.S).group(1)
gates = tomllib.loads(fence)["gate"]
pre = next(g for g in gates if g["run"].endswith("evidence/reply-probe.ndjson'") or g["run"].endswith("evidence/reply-probe.ndjson"))
fixed = next(g for g in gates if "evidence/reply-probe-fixed.ndjson" in g["run"])

rows = [json.loads(l) for l in open(os.path.join(CHUNK, "evidence/reply-probe.ndjson"), encoding="utf-8")]


def write(name, lines):
    p = os.path.join(OUT, name)
    with open(p, "w", encoding="utf-8", newline="\n") as f:
        for r in lines:
            f.write(json.dumps(r, sort_keys=True) + "\n")
    return p


def read(gate, ledger, path):
    run = gate["run"].replace(CHUNK + "/evidence/" + ledger, os.path.relpath(path))
    return subprocess.run(["bash", "-o", "pipefail", "-c", run], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode


def key(k, **over):
    r = {"kind": "key-control", "key": k, "receipt_key_hex": format(ord(k), "x"), "guard": True,
         "wheel_holder": "human", "wheel_cause": "human-input", "send_exit": 10, "compositor": "own", "build": "harness"}
    r.update(over)
    return r


def fixed_rows(**over):
    out = []
    for r in rows:
        x = dict(r, read_as_typing=False, wheel_human_input=0, build_sha256="0000000000000000")
        out.append(x)
    return out


results = []
# the pre-fix reader: green on the record; its control is a record with six shapes typing
six = [dict(r, read_as_typing=False) if r["id"] == "dsr" else r for r in rows]
eight = [dict(r, read_as_typing=True) if r["id"] == "da1" else r for r in rows]
results.append(("pre-fix reader · the committed record", 0, read(pre, "reply-probe.ndjson", os.path.join(CHUNK, "evidence/reply-probe.ndjson"))))
results.append(("pre-fix reader · six shapes typing", 1, read(pre, "reply-probe.ndjson", write("pre-six.ndjson", six))))
results.append(("pre-fix reader · an eighth reply typing", 1, read(pre, "reply-probe.ndjson", write("pre-eight.ndjson", eight))))
results.append(("pre-fix reader · 22 rows", 1, read(pre, "reply-probe.ndjson", write("pre-22.ndjson", rows[:-1]))))

good = fixed_rows() + [key("n"), key("t"), key("m")]
results.append(("fixed reader · the pass file", 0, read(fixed, "reply-probe-fixed.ndjson", write("fixed-pass.ndjson", good))))
cases = {
    "one reply still typing": [dict(r, read_as_typing=True) if r.get("id") == "winops-16" else r for r in good],
    "a shape the terminal did not answer": [dict(r, reply_bytes=0) if r.get("id") == "theme-996" else r for r in good],
    "the DA1 control not answered": [dict(r, reply_bytes=0) if r.get("id") == "da1" else r for r in good],
    "22 probe rows": [r for r in good if r.get("id") != "osc4"],
    "the harness build in a probe row": [dict(r, build="harness") if r.get("id") == "dsr" else r for r in good],
    "a key typed by the probe": [dict(r, key_typed=True) if r.get("id") == "cpr" else r for r in good],
    "two key controls": fixed_rows() + [key("n"), key("t")],
    "a key control with the guard failed": fixed_rows() + [key("n"), key("t"), key("m", guard=False)],
    "a key that did not move the wheel": fixed_rows() + [key("n"), key("t"), key("m", wheel_holder="driver")],
    "a key control with the send not refused": fixed_rows() + [key("n"), key("t"), key("m", send_exit=0)],
    "a key in the desktop compositor": fixed_rows() + [key("n"), key("t"), key("m", compositor="desktop")],
    "another key in place of m": fixed_rows() + [key("n"), key("t"), key("x")],
}
for i, (name, lines) in enumerate(cases.items(), 1):
    results.append(("fixed reader · " + name, 1, read(fixed, "reply-probe-fixed.ndjson", write("fixed-fail-%02d.ndjson" % i, lines))))

bad = 0
for name, want, got in results:
    ok = "ok" if want == got else "MISMATCH"
    bad += want != got
    print("%-58s want exit %d · got %d · %s" % (name, want, got, ok))
print("controls: %d · mismatches: %d" % (len(results), bad))
sys.exit(1 if bad else 0)
