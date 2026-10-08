"""Step 5c: the RESULT line of each evidence/key-probe-own.sh run, as a `key-control` row.

  key-rows.py <log of key n> <log of key t> <log of key m>   (rows on stdout)
"""
import json
import re
import sys


def main():
    for path in sys.argv[1:]:
        with open(path, encoding="utf-8") as f:
            lines = f.read().splitlines()
        result = [l for l in lines if l.startswith("RESULT ")]
        assert len(result) == 1, path
        kv = dict(p.split("=", 1) for p in result[0].split()[1:])
        key_line = [l for l in lines if " key: guard ok immediately before the key" in l]
        assert len(key_line) == 1, path
        at = re.match(r"^(\d\d:\d\d:\d\d)\.\d+Z ", key_line[0]).group(1)
        print(
            json.dumps(
                {
                    "kind": "key-control",
                    "at": "2026-10-08T" + at + "Z",
                    "key": kv["key"],
                    "receipt_key_hex": kv["receipt_keys"],
                    "guard": kv["guard"] == "true",
                    "focus_by_itself": kv["focus_self"] == "true",
                    "focus_wheel_delta": int(kv["focus_wheel_delta"]),
                    "new_wheel_records": int(kv["new_wheel"]),
                    "wheel_holder": kv["wheel_holder"],
                    "wheel_cause": kv["wheel_cause"],
                    "send_exit": int(kv["send_exit"]),
                    "send_issued": int(kv["send_issued"]),
                    "compositor": "own",
                    "build": "harness",
                    "agent": "fake",
                    "typed_into": "probe",
                    "cols": int(kv["cols"]),
                    "rows": int(kv["rows"]),
                    "font_size": int(kv["font"]),
                },
                sort_keys=True,
            )
        )


if __name__ == "__main__":
    main()
