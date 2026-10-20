# Footer reading — claude 2.1.302 (plan step 8)

Written by `footer-reading.sh`. One `viola run` start of `2.1.302 (Claude Code)` under a tmpfs `--home`, its cwd a fresh
`.viola-footer-<pid>/` under the repository root, in a detached tmux pane 80×24, with no key typed
(the founder's Fork 2 option 1, `inputs#I6`). Only the pane's bottom three rows are kept, scrubbed of the
home and the user word.

- Start: 2026-10-20T05:05:37Z.
- `viola list --json` at ~2 s: 2026-10-20T05:05:39Z · exit 0 · the m302 row: wrapped=True · liveness=live · status=idle.

## ~15 s — 2026-10-20T05:05:52Z

```text
────────────────────────────────────────────────────────────────────────────────

  ⏵⏵ auto mode on (shift+tab to cycle)
```

`for agents` on these rows: no

## ~25 s — 2026-10-20T05:06:02Z

```text
────────────────────────────────────────────────────────────────────────────────

  ⏵⏵ auto mode on (shift+tab to cycle)
```

`for agents` on these rows: no

- Killed: 2026-10-20T05:06:02Z, with no key typed.

## Reading (implement, after the script; plan step 9)

- The literal is `(shift+tab to cycle)`. The resting 2.1.302 input box keeps it at both readings, and it is on no modal
  row. Implement read the shortest-text rule as the shortest phrase on the row that also holds three properties:
  - It is not tied to the permission mode. `auto mode on` is the user's mode word, and so is 2.1.287's `manual mode on`.
  - It is not on the rule row above.
  - It is not in another text the 2.1.302 bundle holds. The bundle has a tip, "Hit shift+tab to cycle between manual
    mode, auto-accept edit mode, and plan mode", and a Skills dialog hint, "enter/space to cycle, / to search, …". The
    parentheses keep the literal off both. The bundle was read by path, with no start.
- **Did `viola list` expire the hint? Not separable by this reading.**
  - The `list` read at ~2 s joined the session (`status=idle`, so its `claude agents --json` listed it).
  - The hint was gone at ~15 s. Both candidate timers had run out by then: the bundle's 10 s value counted from the
    `list` read (expiring ~12 s) and counted from the start (~10 s).
  - **`send`'s exposure:** a 2.1.302 session that one `viola list` has read shows no `for agents` 15 s after its
    start. On a verified 2.1.302 with only the old literal, `run`'s pre-send gate would wait to `GATE_MAX_WAIT` and
    refuse `input-not-ready`. With the measured literal it reads the resting footer as the input box.
