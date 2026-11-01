proposals:
  - detector: D-design-cli-wording
    severity: warning
    section: Component — Primary content block 2 → Line form (stderr)
    change: 'Add to the fixed-message exceptions the socket-directory refusal of the `viola run` and `viola revive` starts, exit 1, made before any snapshot, child or bind, stdout empty and the directory left as found: `unable: the socket directory for <name> cannot be trusted` / `hint: viola''s socket directory must be a directory you own, readable only by you; viola does not change its permissions`, a planning form with the founder''s wording owed at Epoch 7''s boundary batch.'
    sidecar: 'Primary content block 2: the fixed-message exceptions gain the socket-directory pair of the `run` and `revive` starts, a planning form (chunk 2026-11-01-unix-endpoint-and-home-hardening).'
    rationale: 'The report''s Symbols / APIs adds `refuse_socket_dir` at `run` and `revive`''s start, and Coverage of new surfaces states the new pair; Expected amendments names this site and the wording a planning form; this document states neither line (report.md:14, report.md:66, report.md:76).'
    basis: src/cmd/run.rs:822-829
  - detector: D-design-cli-wording
    severity: warning
    section: Component — Primary content block 2 → Hint line
    change: 'Add the frame verbs'' socket-directory cause: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` or `unlink` meeting a refused socket directory writes no frame and exits 21 with `unable  <name>  instance-unreachable` (`send` in its `[/ ] unable` mirror) and `hint: viola''s socket directory must be a directory you own, readable only by you; viola does not change its permissions` as the last stderr line; under `--json` it prints `{"v":1,"error":"instance-unreachable","detail":"socket-dir-refused"}` with stderr empty.'
    sidecar: 'Primary content block 2: the Hint line gains the frame verbs'' exit-21 socket-directory hint and its `--json` document (chunk 2026-11-01-unix-endpoint-and-home-hardening).'
    rationale: 'The report''s Symbols / APIs maps `CallError::SocketDirRefused` to `Unreachable::SocketDir` with hint cause `socket-dir`, and Coverage of new surfaces has the check at the eight CLI frame verbs; the Hint line gives every exit-21 cause its own hint and does not state this one (report.md:15, report.md:19, report.md:71).'
    basis: tests/security_negatives_socket_dir.rs:36-37
  - detector: D-design-cli-wording
    severity: warning
    section: Component — Primary content block 2 → Hint line
    change: 'Replace "On Linux and Windows (macOS on the recorded gap, its server check making no comparison)" with: on Linux, Windows and macOS a frame verb reads `server-verify-failed` with this line, hint and document, and on macOS that means an euid mismatch alone, its server check comparing no peer pid (the named carve-out, owned by no route entry).'
    sidecar: 'Primary content block 2: the server check''s `server-verify-failed` now reaches macOS, as an euid mismatch alone; the no-pid comparison stays as the named carve-out (chunk 2026-11-01-unix-endpoint-and-home-hardening).'
    rationale: 'Counts / qualifiers moved: `server-verify-failed`''s reach becomes Linux, Windows and macOS, each row changes, and macOS''s pid gap stays named but owned by no route entry; the founder''s own word on Q2 (report.md:31, report.md:33, report.md:93).'
    basis: src/cmd/client.rs:177-179
  - detector: D-design-cli-wording
    severity: warning
    section: Component — Primary content block 2 → Exit codes as the typed tail
    change: 'The machine view''s exit-21 `<cause>` list reads six closed words, `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, `socket-dir-refused`.'
    sidecar: 'Primary content block 2: exit 21''s closed causes extend from five to six with `socket-dir-refused` (chunk 2026-11-01-unix-endpoint-and-home-hardening).'
    rationale: 'Counts / qualifiers moved: exit 21''s causes five → six, and a row holding a list of the five extends; this bullet holds that list without the sixth word (report.md:10, report.md:28).'
    basis: crates/viola-core/src/lib.rs:201
  - detector: D-design-cli-wording
    severity: warning
    section: Output structure — `viola revive`
    change: 'Extend the shared-start refusal sentence: under `--json` `<code>` also takes `socket-dir-refused`, so `{"v":1,"error":"socket-dir-refused","detail":null}` with exit 1 and stderr empty, and without `--json` that refusal is the socket-directory pair of Primary content block 2.'
    sidecar: 'Output structure — `viola revive`: the shared start''s `--json` codes gain `socket-dir-refused`, and its human form is the socket-directory pair (chunk 2026-11-01-unix-endpoint-and-home-hardening).'
    rationale: 'Counts / qualifiers moved: `revive --json`''s exit-1 codes ten → eleven, each list holding all ten extends, and Expected amendments counts one `revive-codes` site in layout-templates; `revive`''s start shares the refusal (report.md:14, report.md:29, report.md:66).'
    basis: src/cmd/run.rs:822-829
note: 'D-layout-surface holds: the chunk adds refusal lines and no surface or region. D-retired has no site: this document names neither the route entry nor two home rules, and its `viola list` home refusal stays true under the read rule. The new pair and `socket-dir-refused` stand in neither master, so design-system needs the same wording from its own pass.'
