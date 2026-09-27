"""Plan step 7, the Unix-only witness (overseer direction 6). Plant a failing #[cfg(unix)] test in
pre_push.rs's test module; show this Windows host cannot see it (the filter selects nothing:
nextest-exit-4) and `pre-push` stops red at linux-tests naming it, before any leg; then remove the
plant, the original bytes asserted identical. The green remove-the-plant reading is the three
consecutive green pre-push runs over the same tree (evidence/linux-red-investigation.md)."""
import json
import os
import subprocess

ROOT = r"D:\dev\projects\viola"
HERE = os.path.dirname(os.path.abspath(__file__))
REL = r"crates\viola-e2e\src\harness\pre_push.rs"
ANCHOR = "    #[test]\n    fn pre_push_host_gate_is_a_windows_const() {"
PLANT = ("    #[cfg(unix)]\n    #[test]\n    fn pre_push_planted_unix_red() {\n"
         "        panic!(\"planted unix red\");\n    }\n\n")
BASH = r"C:\Program Files\Git\bin\bash.exe"
RA = r"C:\Users\turbo\AppData\Local\Temp\claude\D--dev-projects-viola\29707c17-24df-41cf-a283-d674c38e8bf0\scratchpad\ra-stop.ps1"


def run(argv, log):
    with open(os.path.join(HERE, log), "wb") as f:
        proc = subprocess.run(argv, cwd=ROOT, stdout=f, stderr=subprocess.STDOUT)
    return proc.returncode, open(os.path.join(HERE, log), encoding="utf-8", errors="replace").read()


path = os.path.join(ROOT, REL)
original = open(path, "rb").read()
assert original.count(ANCHOR.encode()) == 1
open(path, "wb").write(original.replace(ANCHOR.encode(), (PLANT + ANCHOR).encode(), 1))
try:
    host_code, host = run([BASH, "scripts/agent-run.sh", "run", "--unit", "--filter",
                           "test(/pre_push_planted_unix_red/)"], "witness-host.log")
    subprocess.run(["powershell.exe", "-NoProfile", "-File", RA], capture_output=True)
    pp_code, pp = run([BASH, "scripts/agent-run.sh", "pre-push"], "witness-pre-push.log")
finally:
    open(path, "wb").write(original)
assert open(path, "rb").read() == original, "restore mismatch"

doc = [json.loads(l) for l in pp.splitlines() if l.startswith('{"v":1,"cmd":"pre-push"')][-1]
host_doc = [json.loads(l) for l in host.splitlines() if l.startswith('{"v":1,"cmd":"run"')][-1]
reading = {
    "host": {"exit": host_code, "nextest_unit_failures": [s["failures"] for s in host_doc["suites"]
                                                          if s["suite"] == "nextest-unit"][0]},
    "pre_push": {"exit": pp_code, "ok": doc["ok"], "stage": doc["stage"],
                 "failures": doc["linux"]["run"]["suites"][0]["failures"],
                 "legs_ran": "legs" in doc, "sync": doc["sync"]},
}
json.dump(reading, open(os.path.join(HERE, "witness-reading.json"), "w", newline="\n"), indent=2)
print(json.dumps(reading, indent=1))
