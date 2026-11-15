# Relay 1 — the overseer's answer to the pty-host technical fork (implement P1, 2026-11-15)

Question (asked as a dialog): the plan hosts the wrapped and unwrapped sessions on the copied `live-pty.py`, which must
stay byte-identical (gate 1 `cmp`), but it writes nothing to the master and discards every drained byte — it cannot
type the trust keys or step 3(d)'s `x`/DEL, nor scan the drained bytes. How should live-hold.py host its sessions?

Selected: "Own pty loop (Recommended)"

Notes, verbatim:

The overseer's technical answer (no founder fork): live-hold.py opens its own pty in live-pty.py's exact form, importing its constants and declared_env, and adds only the key writes and the in-memory scan; live-pty.py stays byte-identical (gate 1's cmp). Bind it: every key write sits behind the founder-words.md line that grants it (trust; input-box) and refuses when the line reads unanswered or no; the trust keys only while the drained bytes hold the trust literal, at most 2 down-arrows and 1 Enter; the rehearsal under the fake agent covers both the granted and the refused arm. Record it as a P4 deviation for the wrap.
