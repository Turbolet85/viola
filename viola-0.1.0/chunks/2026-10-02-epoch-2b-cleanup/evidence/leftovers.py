"""Step 17: the viola test leftovers, by class. Dry-run by default; `--apply` is the founder's removal (P4 ruling).

usage (repository root):  python -X utf8 <this file> [--apply]

Classes, each by its measured shape, never by age:
  temp-git-repo       %TEMP%/.tmp*/ holding a git repo whose only tracked file is the harness fixture's `a.rs`
  temp-cargo-project  %TEMP%/.tmp*/ holding a throwaway cargo project whose package is named "viola"
  temp-empty          %TEMP%/.tmp*/ with nothing in it
  e2e-home-ownerless  target/e2e-home/viola-test-*/ without an owner.json (no sweep may ever take it)
Other `.tmp*` dirs in %TEMP% belong to other tools and are only counted, never touched.
"""
import os, stat, sys, shutil, subprocess, collections

apply = "--apply" in sys.argv
temp = os.environ.get("TMP") or os.environ.get("TEMP")


def tracked(repo):
    out = subprocess.run(["git", "-C", repo, "ls-files"], capture_output=True, text=True)
    return out.stdout.split() if out.returncode == 0 else None


def is_viola_cargo(d):
    for root, dirs, files in os.walk(d):
        dirs[:] = [x for x in dirs if x not in ("target", ".git")]
        if "Cargo.toml" in files:
            with open(os.path.join(root, "Cargo.toml"), encoding="utf-8", errors="replace") as f:
                if 'name = "viola"' in f.read():
                    return True
    return False


def classify(d):
    entries = os.listdir(d)
    if not entries:
        return "temp-empty"
    if ".git" in entries and tracked(d) == ["a.rs"]:
        return "temp-git-repo"
    if is_viola_cargo(d):
        return "temp-cargo-project"
    return "other"


def writable_then_remove(path):
    def onexc(func, p, _exc):
        os.chmod(p, stat.S_IWRITE)
        func(p)
    shutil.rmtree(path, onexc=onexc)


classes = collections.Counter()
removed = collections.Counter()
for e in os.scandir(temp):
    if not (e.name.startswith(".tmp") and e.is_dir(follow_symlinks=False)):
        continue
    try:
        c = classify(e.path)
    except OSError:
        c = "unreadable"
    classes[c] += 1
    if apply and c.startswith("temp-"):
        try:
            writable_then_remove(e.path)
            removed[c] += 1
        except OSError:
            removed[c + " (failed)"] += 1

home = os.path.join("target", "e2e-home")
for n in sorted(os.listdir(home)) if os.path.isdir(home) else []:
    d = os.path.join(home, n)
    if n.startswith("viola-test-") and os.path.isdir(d) and not os.path.exists(os.path.join(d, "owner.json")):
        classes["e2e-home-ownerless"] += 1
        if apply:
            try:
                writable_then_remove(d)
                removed["e2e-home-ownerless"] += 1
            except OSError:
                removed["e2e-home-ownerless (failed)"] += 1

print(("APPLY" if apply else "DRY-RUN") + " — counts by class:")
for k, v in sorted(classes.items()):
    print(f"  {k:22s} {v}")
if apply:
    print("removed:")
    for k, v in sorted(removed.items()):
        print(f"  {k:22s} {v}")
