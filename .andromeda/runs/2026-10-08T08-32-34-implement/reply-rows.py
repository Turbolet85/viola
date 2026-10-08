"""Step 5c: the printed lines of evidence/reply-probe.sh, as `reply-probe` rows in the row form of
evidence/reply-probe.ndjson, plus build_sha256.

  reply-rows.py <the probe's printed output> <build sha256, 16 hex digits>   (rows on stdout)
"""
import json
import re
import sys

LINE = re.compile(r"^(\S+) window_exit=(\d+) wheel_human_input=(\d+) kinds=\[([^\]]*)\] (.*)$")


def shown(raw):
    return raw.decode("ascii", "replace").replace("\x1b", "ESC ")


def main():
    out_path, sha = sys.argv[1], sys.argv[2]
    rows = []
    with open(out_path, encoding="utf-8") as f:
        for line in f:
            m = LINE.match(line.rstrip("\n"))
            if not m:
                continue
            qid, window_exit, moved, kinds, side = m.groups()
            if side == "no-side-line":
                sys.exit("no side line for " + qid)
            s = json.loads(side)
            assert s["id"] == qid, qid
            raw = bytes.fromhex(s["reply_hex"])
            rows.append(
                {
                    "build": "release-check",
                    "build_sha256": sha,
                    "child": "reply-probe-child.py",
                    "id": qid,
                    "key_typed": False,
                    "kind": "reply-probe",
                    "query_hex": s["query_hex"],
                    "read_as_typing": int(moved) > 0,
                    "reply_bytes": s["reply_bytes"],
                    "reply_hex": s["reply_hex"],
                    "reply_shown": shown(raw),
                    "terminal": "foot 1.28.0",
                    "wheel_human_input": int(moved),
                    "window_exit": int(window_exit),
                }
            )
    for r in rows:
        print(json.dumps(r, sort_keys=True))


if __name__ == "__main__":
    main()
