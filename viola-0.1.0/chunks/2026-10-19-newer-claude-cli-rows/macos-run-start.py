"""Plan step 6(b)'s scratch reader: the gap from a wrapper's process-start{subject:"self"} to its
process-start{subject:"version-probe"} in every diagnostics/run-*.ndjson under the given dirs.

Usage: python3 -I macos-run-start.py <label>=<dir> [<label>=<dir> ...]

Prints per label: the count of self starts, the gap distribution in ms, every gap over 1 s, and every
self start with no version-probe start after it (a stall), with the last event after it. Each of
those rows carries `near`: how many other self starts (any home) fall within 1 s of its own, and the
summary gives that count's median for the boots whose gap was 1 s or less. Names are
the home's own directory name under the dir and the pid; no path is printed.
"""

import json
import pathlib
import sys
from datetime import datetime


def ts(line):
    return datetime.fromisoformat(line["timestamp"].replace("Z", "+00:00"))


def lines_of(path):
    out = []
    for raw in path.read_bytes().split(b"\n"):
        if not raw.strip():
            continue
        try:
            out.append(json.loads(raw))
        except ValueError:
            pass
    return out


def boots(lines):
    """Each self start with the version-probe start that follows it before the next self start."""
    found = []
    current = None
    for line in lines:
        if line.get("event") == "process-start" and line.get("subject") == "self":
            if current is not None:
                found.append(current)
            current = {"self": line, "probe": None, "after": []}
        elif current is not None:
            if (
                current["probe"] is None
                and line.get("event") == "process-start"
                and line.get("subject") == "version-probe"
            ):
                current["probe"] = line
            current["after"].append(line)
    if current is not None:
        found.append(current)
    return found


def pct(sorted_ms, p):
    if not sorted_ms:
        return None
    k = round((len(sorted_ms) - 1) * p)
    return sorted_ms[k]


def main(args):
    for arg in args:
        label, _, root = arg.partition("=")
        root = pathlib.Path(root)
        gaps, stalls, slow, starts, fast = [], [], [], [], []
        for path in sorted(root.glob("*/**/diagnostics/run-*.ndjson")):
            home = path.relative_to(root).parts[0]
            for boot in boots(lines_of(path)):
                pid = boot["self"].get("pid")
                start = ts(boot["self"])
                starts.append(start)
                if boot["probe"] is None:
                    last = boot["after"][-1]["event"] if boot["after"] else "none"
                    stalls.append((home, path.name, pid, boot["self"]["timestamp"], last, start))
                    continue
                gap = (ts(boot["probe"]) - start).total_seconds() * 1000
                gaps.append(gap)
                if gap > 1000:
                    slow.append((home, path.name, pid, boot["self"]["timestamp"], round(gap), start))
                else:
                    fast.append(start)
        gaps.sort()

        def near(at):
            return sum(1 for s in starts if abs((s - at).total_seconds()) <= 1.0) - 1
        print(f"== {label}")
        print(f"self starts {len(gaps) + len(stalls)} · with a version-probe start {len(gaps)} · without {len(stalls)}")
        if gaps:
            print(
                "gap ms min {:.0f} · p50 {:.0f} · p90 {:.0f} · p99 {:.0f} · max {:.0f}".format(
                    gaps[0], pct(gaps, 0.5), pct(gaps, 0.9), pct(gaps, 0.99), gaps[-1]
                )
            )
        fast_near = sorted(near(at) for at in fast)
        if fast_near:
            print(f"near, gap 1 s or less: median {fast_near[len(fast_near) // 2]} · max {fast_near[-1]}")
        for row in slow:
            print("over 1 s: home {} file {} pid {} self at {} gap {} ms near {}".format(*row[:5], near(row[5])))
        for row in stalls:
            print("no version-probe: home {} file {} pid {} self at {} last event {} near {}".format(*row[:5], near(row[5])))


if __name__ == "__main__":
    main(sys.argv[1:])
