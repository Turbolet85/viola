# Host Recipes

Shell mechanics of the Bash tool on this host: the recurring ways a recipe breaks. Every rule is measured on live
work, not hypothetical.

## Paths & the working directory
- The working directory MAY persist across calls (measured both ways on one host: persisted in one session,
  reset after every call in another) — rely on neither: anchor every cross-call path absolutely, or `cd` only
  inside a subshell `( cd X && … )` (setup's Bash guard denies a `cd` that would move the cwd); a
  `cd` inside a compound command can also trip a permission prompt.

## Probes & pattern tools
- A zero-is-healthy count probe (`grep -c` / `grep -q`) exits non-zero on no matches and aborts a
  `&&` chain — suffix `|| true`, or chain with `;`.

## Transports (JSON & documents)
- Never build JSON — or any multi-KB document — through shell quoting: `printf` collapses escapes
  content-dependently.
- Documents: the Write tool, whole-content (setup's Bash guard refuses a `cat`/`tee` heredoc with a file
  target). Ledger appends: a VALIDATED python append (`json.dumps(json.loads(...))` — a mangled payload fails
  loudly instead of landing).
- Inline `python -c` is fine for a SHORT, quote-free, single-expression probe; anything longer,
  quote-bearing, or document-carrying goes through a scratchpad file run by path (the measured failure
  modes are size and unquoted expansion, not inlining or quoted heredocs).

## Compound commands & permissions
- `rm -rf` + `mkdir` + launch compounds get denied by permission layers and abort mid-chain —
  granular steps, fresh unique dirs, no `rm` in a launch path.
- `;` over `&&` for optional probes: an optional probe's failure must not abort the chain.

## Processes
- Never `pgrep -f` / `pkill -f` on a pattern the invoking shell's own command line contains — it matches, and
  kills, that shell. Select by `ps -eo pid,comm,args` and act on the pid.

## Exit codes
- Read `$?` from the BARE command — `| tail` / `| head` report the pipe's LAST command's exit
  (measured green-masking red gates). Redirect output to a file and inspect the file.
- When a tool prints its own verdict, the printed text outranks an unisolated `$?`.

## Encoding & heredocs
- Heredoc terminators must be column-0 and whitespace-exact — a padded terminator silently
  swallows the rest of the script.
- The repo pins LF through `.gitattributes` (setup appends the lines): `git ls-files --eol` on a pipeline file
  reads `i/lf w/lf`. Never write CRLF into a pipeline file, and never fix a terminator reading with a
  terminator-agnostic script.

## Session Additions
- 2026-09-24: `grep` over a glob that matches ONE file prints no `file:` prefix, so an exemption keyed on `^[^:]+:[0-9]+:` never matches — use `grep -H` whenever a later stage parses the prefix.
- 2026-09-28: The Bash guard refuses a heredoc whose payload carries a doubled backslash, so a source edit holding escape sequences (a Rust string literal) goes through the Edit tool, never a shell heredoc. Extended 2026-09-29: it refuses ANY command carrying one — a sed expression, an inline regex, a JSON record — so such a payload goes in a Write-tool file run by path, or builds the backslash with `chr(92)`. Extended 2026-09-29: it also refuses a heredoc redirected to a file (`cat <<'EOF' > f`) — write the file with the Write tool.
- 2026-10-07: On the Linux dev host a process named `viola` is not this repository's by its name: the overseer pair's bridge is another tree's build of the same name, so `pgrep -x viola` reads it too — decide whose a process is by `/proc/<pid>/exe` against the repository root.
- 2026-10-07: A removal the permission layer denied is never done through another tool (no script, no re-spelled `rm`): move a scratch home this builder made by its full name into `target/e2e-home.disk/`, read back that `target/e2e-home` no longer holds it, and record where it stands; its deletion is the founder's at the desk (the overseer's direction).
- 2026-10-07: A backgrounded command's output file ends with the harness's own exit line, never the command's last line, so a wait that tests the file's last line for a summary runs to its bound: match the summary anywhere in the file, or wait on the completion notice.
- 2026-10-08: On the Linux dev host the desktop lock is the shell's own, so a search for a locker process and logind's `LockedHint` both read "not locked" while it holds, and no window takes keyboard focus under it: read `hyprctl locked` first before any step that focuses a window or types a key with `wtype`, and stop on `true` (unlocking is the founder's).
