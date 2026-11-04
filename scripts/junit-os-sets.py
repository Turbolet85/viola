"""The per-OS JUnit reading of one CI run: the operator pass's reading, never a CI step.

`--run <id>` downloads the run's `junit-windows-2025`, `junit-macos-latest` and
`junit-ubuntu-latest` artifacts with `gh run download` into a temporary directory removed on exit,
parses each `junit.xml`, and prints one line per OS — `{os}: {n} cases · {f} failed · {s} skipped`
— then four set differences of the cases, keyed `classname::name`, each a `## {label}: {count}`
heading and one case per line, sorted. Exit 0 once all three reports were read, 1 when one is
missing or unparsable (named), 2 on usage. It reads no environment variable and writes nothing in
the tree.
"""

import argparse
import shutil
import subprocess
import sys
import tempfile
import xml.etree.ElementTree as ET
from pathlib import Path

OSES = ("windows-2025", "macos-latest", "ubuntu-latest")
DIFFERENCES = (
    ("ubuntu-not-macos", "ubuntu-latest", "macos-latest"),
    ("macos-not-ubuntu", "macos-latest", "ubuntu-latest"),
    ("windows-not-ubuntu", "windows-2025", "ubuntu-latest"),
    ("ubuntu-not-windows", "ubuntu-latest", "windows-2025"),
)


def read_report(run, os_name, scratch):
    """The report's cases as `{key: outcome}`, outcome `passed` · `failed` · `skipped`; None when
    the artifact is missing or its `junit.xml` does not parse."""
    dest = scratch / os_name
    got = subprocess.run(
        ["gh", "run", "download", str(run), "--name", f"junit-{os_name}", "--dir", str(dest)],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=False,
    )
    report = dest / "junit.xml"
    if got.returncode != 0 or not report.is_file():
        return None
    try:
        root = ET.parse(report).getroot()
    except ET.ParseError:
        return None
    cases = {}
    for case in root.iter("testcase"):
        key = f"{case.get('classname', '')}::{case.get('name', '')}"
        if case.find("failure") is not None or case.find("error") is not None:
            cases[key] = "failed"
        elif case.find("skipped") is not None:
            cases[key] = "skipped"
        else:
            cases[key] = "passed"
    return cases


def main():
    parser = argparse.ArgumentParser(description="the per-OS JUnit reading of one CI run")
    parser.add_argument("--run", required=True, type=int, help="the CI run id")
    args = parser.parse_args()

    scratch = Path(tempfile.mkdtemp(prefix="junit-os-sets-"))
    try:
        reports = {}
        missing = []
        for os_name in OSES:
            cases = read_report(args.run, os_name, scratch)
            if cases is None:
                missing.append(os_name)
            else:
                reports[os_name] = cases
    finally:
        shutil.rmtree(scratch, ignore_errors=True)

    for os_name in OSES:
        if os_name in reports:
            outcomes = list(reports[os_name].values())
            print(
                f"{os_name}: {len(outcomes)} cases · {outcomes.count('failed')} failed"
                f" · {outcomes.count('skipped')} skipped"
            )
    if missing:
        for os_name in missing:
            print(f"missing or unparsable: junit-{os_name}")
        return 1
    for label, left, right in DIFFERENCES:
        only = sorted(set(reports[left]) - set(reports[right]))
        print(f"## {label}: {len(only)}")
        for key in only:
            print(key)
    return 0


if __name__ == "__main__":
    sys.exit(main())
