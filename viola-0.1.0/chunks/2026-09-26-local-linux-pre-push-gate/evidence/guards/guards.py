"""Remove-the-guard runs (plan step 6): neutralise each guard, run its named test red, restore the
original bytes (asserted identical), run it green. Readings in readings.json, logs beside it."""
import json
import os
import subprocess
import sys

ROOT = r"D:\dev\projects\viola"
HERE = os.path.dirname(os.path.abspath(__file__))
PP = r"crates\viola-e2e\src\harness\pre_push.rs"
MU = r"crates\viola-e2e\src\harness\run\mutants.rs"

GUARDS = [
    ("g1-tree-id", PP, "if synced.trim() != tree {", "if false && synced.trim() != tree {",
     "pre_push_sync_mismatch_is_red"),
    ("g2-base", PP, "if bases[0].is_none() || bases[0] != bases[1] {", "if false {",
     "pre_push_base_mismatch_is_red"),
    ("g3-uncommitted-promotion", MU, "&& !uncommitted_promotion(repo) {", "{",
     "chunk_base_of_an_uncommitted_promotion_is_the_flip_at_head"),
    ("g4-cache-cap", PP, "let cleaned = bytes > CACHE_CAP_BYTES;",
     "let cleaned = false && bytes > CACHE_CAP_BYTES;",
     "pre_push_cache_over_cap_is_cleaned_and_reported"),
    ("g5-pin", PP, "if version != Some(pin.as_str()) {",
     "if false && version != Some(pin.as_str()) {", "pre_push_pin_mismatch_names_the_tool"),
    ("g6-env-i", PP, 'cmd.args(["--exec", "/usr/bin/env", "-i"])',
     'cmd.args(["--exec", "/usr/bin/env"])', "pre_push_every_wsl_call_uses_exec_and_a_clean_env"),
    ("g7-fail-fast", PP, "if !green {", "if false && !green {",
     "pre_push_linux_test_red_stops_before_the_legs"),
    ("g8-clean", PP, '&["clean", "-fdq"]', '&["status", "-s"]',
     "pre_push_sync_reproduces_a_dirty_tree"),
]


def test(name, log):
    env = dict(os.environ, CARGO_TARGET_DIR=os.path.join(ROOT, "target", "check-pp"))
    with open(log, "wb") as out:
        proc = subprocess.run(["cargo", "test", "-p", "viola-e2e", "--lib", "--", name],
                              cwd=ROOT, env=env, stdout=out, stderr=subprocess.STDOUT)
    text = open(log, encoding="utf-8", errors="replace").read()
    result = [l for l in text.splitlines() if l.startswith("test result:")]
    ran = [l for l in text.splitlines() if l.startswith("test ") and name in l]
    return proc.returncode, (result[-1] if result else "no test result line"), ran


def cycle(gid, rel, anchor, neutral, name, suffix=""):
    """One neutralise → red → restore → green pair; returns its reading."""
    path = os.path.join(ROOT, rel)
    original = open(path, "rb").read()
    a, n = anchor.encode(), neutral.encode()
    assert original.count(a) == 1, (gid, "anchor count", original.count(a))
    open(path, "wb").write(original.replace(a, n, 1))
    try:
        assert open(path, "rb").read().count(n) >= 1, (gid, "edit did not land")
        red = test(name, os.path.join(HERE, f"{gid}.neutralised{suffix}.log"))
    finally:
        open(path, "wb").write(original)
    assert open(path, "rb").read() == original, (gid, "restore mismatch")
    green = test(name, os.path.join(HERE, f"{gid}.restored{suffix}.log"))
    return {"guard": gid + suffix, "file": rel.replace("\\", "/"), "test": name,
            "neutralised": {"exit": red[0], "result": red[1], "ran": red[2]},
            "restored": {"exit": green[0], "result": green[1], "ran": green[2]},
            "pair_ok": red[0] != 0 and green[0] == 0}


def main():
    readings = []
    for guard in GUARDS:
        row = cycle(*guard)
        readings.append(row)
        print(f"{row['guard']}: neutralised exit {row['neutralised']['exit']} · restored exit "
              f"{row['restored']['exit']} · pair {'ok' if row['pair_ok'] else 'BROKEN'}")
        sys.stdout.flush()
    json.dump(readings, open(os.path.join(HERE, "readings.json"), "w", newline="\n"), indent=2)
    print(f"pairs ok: {sum(r['pair_ok'] for r in readings)}/{len(readings)}")


if __name__ == "__main__":
    main()
