"""Reads ci#37627485806 attempt 1 as fetched into one directory and prints the facts `ci-attempt-1.md` records.

Usage: python -X utf8 read-attempt1.py <dir>, where <dir> holds `attempt.json`, `jobs.json`, `artifacts.json`,
`job.log`, `harness.zip`, `diag.zip` and `junit.zip` (the fetch commands are in `ci-attempt-1.md`).
"""
import io
import json
import sys
import zipfile
from datetime import datetime
from pathlib import Path
from xml.etree import ElementTree

PID = 10799
TEST = "start_opens_the_scenario_one_spans_under_run_start"
ARTIFACTS = (11485600497, 11484568161, 11484488518)
# The runner's checkout prefix, built so that this file holds no host path.
CHECKOUT = "/".join(["", "home", "runner", "work", "viola", "viola", ""])


def instant(text):
    return datetime.fromisoformat(text.replace("Z", "+00:00"))


def main(out):
    attempt = json.loads((out / "attempt.json").read_text(encoding="utf-8"))
    print("attempt", attempt["run_attempt"], attempt["head_sha"], attempt["status"], attempt["conclusion"])

    jobs = json.loads((out / "jobs.json").read_text(encoding="utf-8"))["jobs"]
    print("jobs", len(jobs), sorted(j["conclusion"] for j in jobs if j["conclusion"] != "success"))
    job = next(j for j in jobs if j["name"] == "test (ubuntu-latest)")
    for step in job["steps"]:
        print(f"  step {step['number']:>2} {step['conclusion']:<8} {step['name']}")

    for artifact in json.loads((out / "artifacts.json").read_text(encoding="utf-8"))["artifacts"]:
        if artifact["id"] in ARTIFACTS:
            print("artifact", artifact["id"], artifact["name"], artifact["size_in_bytes"], artifact["expires_at"])

    log = (out / "job.log").read_text(encoding="utf-8", errors="replace")
    for line in log.splitlines():
        if ("no profile can be merged" in line or "tests run:" in line or '"cmd":"run"' in line
                or '"cmd":"gate"' in line or (".profraw" in line and "warning" in line)):
            print("log", line.replace(CHECKOUT, ""))

    harness = zipfile.ZipFile(out / "harness.zip")
    names = harness.namelist()
    print("harness members", len(names), "profraw", sum("profraw" in n for n in names))
    for name in names:
        print("  ", name, harness.getinfo(name).file_size)

    diag = zipfile.ZipFile(out / "diag.zip")
    rows = []
    files = 0
    for name in diag.namelist():
        if not name.endswith(".ndjson"):
            continue
        files += 1
        for raw in diag.read(name).decode("utf-8", errors="replace").splitlines():
            try:
                record = json.loads(raw)
            except ValueError:
                continue
            rows.append((record.get("timestamp"), record.get("pid"), record.get("event"), record.get("process")))
    homes = {name.split("/")[0] for name in diag.namelist()}
    with_pid = sorted((r for r in rows if isinstance(r[1], int)), key=lambda r: r[1])
    print("diag files", files, "lines", len(rows), "test homes", len(homes), "lines with a pid", len(with_pid),
          sorted({r[2] for r in with_pid}), sorted({r[3] for r in with_pid}))
    print("lines of pid", PID, sum(r[1] == PID for r in with_pid))
    below = [r for r in with_pid if r[1] < PID][-1]
    above = [r for r in with_pid if r[1] > PID][0]
    print("below", below)
    print("above", above)
    between = sorted(r[0] for r in rows if r[0] and below[0] < r[0] < above[0])
    print("lines between the two", len(between), "the last at", between[-1] if between else None)

    junit = zipfile.ZipFile(out / "junit.zip")
    low, high = instant(below[0]).timestamp(), instant(above[0]).timestamp()
    overlap = {}
    for name in junit.namelist():
        for suite in ElementTree.parse(io.BytesIO(junit.read(name))).getroot().iter("testsuite"):
            for case in suite.iter("testcase"):
                if not case.get("timestamp"):
                    continue
                start = instant(case.get("timestamp"))
                end = start.timestamp() + float(case.get("time", "0"))
                if start.timestamp() <= high and end >= low:
                    overlap[suite.get("name")] = overlap.get(suite.get("name"), 0) + 1
                if TEST in case.get("name", ""):
                    ended = datetime.fromtimestamp(end, start.tzinfo).isoformat(timespec="milliseconds")
                    print("junit", suite.get("name"), case.get("name"), case.get("timestamp"), case.get("time"), ended)
    print("cases overlapping the window", sum(overlap.values()), overlap)


if __name__ == "__main__":
    main(Path(sys.argv[1]))
