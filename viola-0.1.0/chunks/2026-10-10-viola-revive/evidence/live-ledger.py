#!/usr/bin/env python3
"""The one writer of the start ledger (plan.md steps 1, 2, 13 and 15).

It appends one row to `live-sessions.ndjson`: the row is read from a JSON file, parsed and re-serialised, so a
mangled row fails here and never lands, and `written_at` is the UTC time read from the clock at the write, never a
time typed by hand. A `start` row is written before its start is made.

  live-ledger.py <ledger> <row file>

Run from the repository root.
"""
import datetime
import json
import sys


def main():
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    ledger, row_file = sys.argv[1], sys.argv[2]
    with open(row_file, encoding="utf-8") as f:
        row = json.load(f)
    if not isinstance(row, dict) or not isinstance(row.get("kind"), str):
        sys.exit("a row is an object with a kind")
    row["written_at"] = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    line = json.dumps(row, sort_keys=False)
    with open(ledger, "a", encoding="utf-8") as f:
        f.write(line + "\n")
    print(line)


if __name__ == "__main__":
    main()
