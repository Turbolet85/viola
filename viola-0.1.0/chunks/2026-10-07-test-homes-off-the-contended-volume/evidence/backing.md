# The backing of `target/e2e-home` on the Linux dev host

## Authority
The link rests on the founder's own live answer of 2026-10-07T07:25Z, relayed by the overseer (inputs#I6): the
link with the keeper, the widening shown to him first (the harness creates and deletes outside the working
directory through the link). inputs#I6 was read at step 0, 2026-10-07T07:41Z: present in
`inputs/andromeda-inputs.json`, and its copy `I6-relay-2.md.txt` hashes to the manifest's `sha256` (`4d53891a…`).

## Before anything moved (step 0, 2026-10-07T07:41Z)
- `findmnt -n -o FSTYPE -T target/e2e-home/`: `btrfs`.
- `du -sh target/e2e-home`: 545M.
- `ls -la target/e2e-home` (the owner and group columns read "the user"): 13 entries, the path a real directory
  (`drwxr-xr-x`).

```
drwx------ viola-record-20261005T094211Z
drwx------ viola-record-20261005T100046Z
drwx------ viola-record-20261005T102055Z
drwx------ viola-record-20261005T102140Z
drwx------ viola-record-20261005T131207Z
drwx------ viola-record-20261005T131243Z
drwx------ viola-record-20261006T203444Z
drwx------ viola-record-20261006T211429Z
drwxr-xr-x viola-session-UIPQKC
drwx------ viola-test-SZkaMS
drwxr-xr-x viola-test-WYNVH7
drwx------ vt-21w84iv1
drwx------ vt-i7sy6r2a
```

## The backing as set (step 5, 2026-10-07T07:47Z)
The operator was told in one line before the move (a notice, not a question).

### The quiet-host precondition, as it read
- `pgrep -x viola` found 9 processes; `pgrep -x viola-harness` and `pgrep -x viola-fake-agen` found none.
- All 9 run another tree's `viola` build (the executable path of each, read from `/proc/<pid>/exe`, lies outside
  this repository): they are the bridge the overseer pair itself runs on, one of them hosting this session. None of
  this repository's test or harness processes was alive: 0 of the 9 has its executable under this repository's
  root, and `lsof +D target/e2e-home` listed no open file.
- So the plan's check by process name could not read empty on this host, and was read by executable path instead:
  this repository's 0, other trees' 9.
- `bash scripts/agent-run.sh cleanup`: exit 0, `{"v":1,"cmd":"cleanup","ok":true,"cleaned":[]}`.

### The three commands
1. `mv target/e2e-home target/e2e-home.disk`: exit 0 (the 13 entries, nothing deleted).
2. `mkdir -m 700 /tmp/viola-e2e-home-<uid>`, with the user's numeric id: exit 0.
3. `ln -s /tmp/viola-e2e-home-<uid> target/e2e-home`: exit 0.

### Read back
- The link: `lrwxrwxrwx`, a symbolic link, to `/tmp/viola-e2e-home-<uid>`.
- Its target: `drwx------` (700), a directory, owned by the user.
- `findmnt -n -o FSTYPE -T target/e2e-home/`: `tmpfs`.
- `target/e2e-home.disk`: 13 entries, 545M.
- `git status --short`: no line names the link or `target/e2e-home.disk`; `git check-ignore -v` answers both with
  `.gitignore:15:target/`.

## What the wrap needs to carry (step 9)
- **The old entries.** `target/e2e-home.disk` holds the 13 entries that were in `target/e2e-home`,
  `viola-test-WYNVH7` among them: the handoff's operator desk names it at its old path
  (`target/e2e-home/viola-test-WYNVH7`), and it now sits at `target/e2e-home.disk/viola-test-WYNVH7`. Nothing was
  deleted; the directory is the operator's to remove.
- **After a `cargo clean`.** The link goes with `target/`, and the next start would put a plain directory back on
  the shared volume with no signal. The one-line recipe, from the repository root:
  `ln -s /tmp/viola-e2e-home-<uid> target/e2e-home` (the user's numeric id; `mkdir -p target` first if `target/`
  is gone). No `mkdir` of the link's target is needed: the keeper makes it, owner-only, at the next start. The gate
  entry `test -L target/e2e-home && findmnt -n -o FSTYPE -T target/e2e-home/` reads red until the link is back.
- **After a reboot.** The tmpfs target is gone and the link dangles. The next test or harness start re-creates
  the target with mode 0700 (the keepers: `prepare_home_base` in `tests/support/home.rs`,
  `Workspace::ensure_e2e_home` in `crates/viola-e2e/src/harness/mod.rs`); nothing is done by hand.
- **Kept homes.** A home kept for a red reading (`AGENT_RUN_KEEP_HOMES=1`, `AGENT_RUN_KEEP_FAILED=1`) now lives in
  memory and no longer survives a reboot; `/tmp` also ages out what is untouched for ten days. A home worth keeping
  longer is copied out of the backing by hand.
- **A move to another tmpfs** is a re-link and no code: the keepers follow any absolute target that is an
  owner-only real directory.
