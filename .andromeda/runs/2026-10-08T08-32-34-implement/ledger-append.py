"""Append validated JSON lines to an evidence ledger: each stdin line must parse as one JSON object; the lines
are written compact, each with its newline, in one write.

  ledger-append.py <ledger file>   (lines on stdin)
"""
import json
import sys


def main():
    path = sys.argv[1]
    out = []
    for raw in sys.stdin.read().splitlines():
        if not raw.strip():
            continue
        obj = json.loads(raw)
        if not isinstance(obj, dict):
            sys.exit("not an object")
        out.append(json.dumps(obj, separators=(",", ":"), ensure_ascii=False))
    if not out:
        sys.exit("no line")
    with open(path, "a", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(out) + "\n")
    print("appended %d line(s)" % len(out))


if __name__ == "__main__":
    main()
