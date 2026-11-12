design-system · fct-mfvo2or6@1 · A failed strict-modes check refuses the whole board with exit 21, in the fixed `unable:` / `hint:` form or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`.
security-plan · fct-cw3l7aky@1 · The send text is written into the PTY as one bracketed paste, so an embedded paste terminator or control sequence would end the paste early and reach the claude TUI as keystrokes.
security-plan · fct-a6h3q7dw@2 · An absent socket directory passes the check, so a start can create it, while an unstattable one is refused.
security-plan · fct-ohbg3pqy@1 · The bind and the three client seams share one check on a recorded endpoint's parent directory.
security-plan · fct-6oinutja@1 · The socket-directory check runs before any snapshot, child or bind at run and at the start of revive, a race's loser reading the same.
security-plan · fct-pf2l572y@1 · A dropped peer reads end-of-file, and no line and nothing on the terminal is written.
security-plan · fct-mhqlyrp7@2 · run and revive's start check the whole home tree, home first then instance, ledger, bin and plugin folders, before the wrapper log opens.
security-plan · fct-gfc5nchi@2 · Reading the connected stream's peer pid writes and logs nothing.
security-plan · fct-zpaxdj2d@2 · A peer whose euid cannot be read is refused, and reading it writes and logs nothing.
security-plan · fct-5ryfhirx@2 · Only a Matches verdict, from a parsed snapshot read after the strict-modes check, lets a frame be written.
security-plan · fct-p7bllbag@2 · link and unlink take the same order as every CLI frame verb: strict-modes check, liveness pre-check, then the socket-directory, euid and server checks before their one frame.
architecture · fct-y6frgi47@1 · A replayed snapshot carries no endpoint, pid, start time or statusline command, so its readers treat the instance as unreachable and the collision check judges liveness from the heartbeat alone.
architecture · fct-xcjb7ry2@1 · The plugin marketplace tree is rewritten whole on every plugin install, any other entry is left as found, and viola never reads the tree back.
architecture · fct-q7tym3q2@1 · run, revive's start, mcp and hook stay silent at the internal-error catch site.
architecture · fct-dhyimyqh@1 · A failed connect reads as an instance-dead cause and a closed or broken stream as an endpoint-lost cause, which the exit 21 detail words rest on.
architecture · fct-kbitf2mh@2 · The socket-directory check passes an absent path, refuses a symlink, a non-directory or an unreadable path, and never chmods, so a refused directory stays as found.
architecture · fct-kan7g7fj@1 · Every MCP frame tool refuses a home or target instance directory failing the strict-mode check with the `instance-unreachable` result and detail `strict-modes-failed`, sending no frame.
architecture · fct-au3pdba5@1 · Every unreachable cause a frame tool meets surfaces as the `instance-unreachable` result with no frame and no line.
architecture · fct-vwfyd52k@1 · A dialog hook whose server check fails sends no `hook.dialog` and exits 0 at once with no body, so the dialog renders for the human.
architecture · fct-4qhayrrs@1 · A hook whose instance check refuses or finds no instance exits 0 before reading config, stdin or writing any line.
architecture · fct-vag5jbae@1 · A strict-modes refusal of the home tree or of the stamps makes viola verify exit 1 with the home pair, reading and writing nothing.
architecture · fct-2iyz7ner@1 · A frame verb against a stale instance reads instance-dead, since no live endpoint is given for a stale wrapper.
architecture · fct-e4yo5lcc@1 · Without a live endpoint a client reads an existing instance directory as instance-dead and anything else as an unwrapped name.
architecture · fct-zgxijmsq@1 · Over a replayed snapshot the start's collision check decides from the heartbeat alone, refusing already-live within the stale bound.
architecture · fct-5kgumnrt@1 · A revive makes one snapshot replay and logs one state-recovered line.
architecture · fct-wwpgfv5r@1 · Every start refuses a strict-modes home-tree failure with exit 1 before any log, line or child.
architecture · fct-mefdhndu@1 · A failed socket-dir create and a refused check both read socket-dir-refused with exit 1, and a bind failure does not.
architecture · fct-5gkabtaw@2 · Program resolution has one refusal, ProgramNotFound, with a fixed message.
architecture · fct-5ijnay4v@1 · A start pins the very bytes it read once and keyed for its home-tree check.
architecture · fct-nbf2wdvs@2 · verify's Run E resumes the session id taken from Run B's first SessionStart, so Run E depends on Run B.
architecture · fct-2oafusjr@1 · A hook whose VIOLA_DIR names an instance folder that does not exist exits 0 and writes nothing.
architecture · fct-wdtionxy@2 · A refused diagnostics folder or file silences the log rather than failing the process, and a detail line is dropped.
architecture · fct-vs5zumck@2 · run's and revive's starts and verify check the home tree, refusing at home or instance level, before using it.
architecture · fct-sbmskf35@2 · A path under a regular file is a strict-modes refusal: verify exits 1 with the home pair, the frame verbs exit 21 with the instance pair.
architecture · fct-flit34us@1 · Harness cleanup reads ok after a force-killed wrapper, since a refused or missing socket reads gone and no leftover file is removed.
architecture · fct-w6vpbrjt@2 · Root tests freeze a wrapper with SIGSTOP through one shared guard so that it reads stale, resuming it on drop.
architecture · fct-vp2lxvsd@2 · Both channel clients read the serving process's pid and euid from the connected socket's peer credentials, the identity each frame-writing client checks before its first frame.
architecture · fct-e6lqqdby@1 · The serving process's start time read through sysinfo is the observed side of the server check each frame-writing client makes before its first frame.
architecture · fct-ekks4dfz@1 · The stamps writer runs the strict-modes check before creating the ledger directory and again under its lock, and a refusal reads and writes nothing.
architecture · fct-fr2pme2q@1 · The link set changes only on an implicit link and on a served link or unlink call.
architecture · fct-ng4l52uh@1 · Every exit-21 CLI document is written by one writer.
architecture · fct-a6y4blve@2 · A frame-writing client runs the strict-modes check, the liveness pre-check, the socket-directory check, the peer euid decision and then the server identity check, in that order.
architecture · fct-4yfg5mra@1 · A frame verb against a stale or dead instance reads instance-dead, while a name with no instance directory reads as unwrapped.
architecture · fct-ikwybol4@2 · A peer whose euid cannot be read is refused.
test-plan · fct-njspb5nv@3 · The child's exit is detected on the process handle and never on EOF.
test-plan · fct-xq6vpu6l@1 · The watch report carries a stalled wrapper's state at the readiness deadline, before the panic, and a passing boot adds nothing to it.
test-plan · fct-pb23i77m@1 · The boot command creates the home, stamps it via viola verify and starts the wrappers, which tests rely on to seed their own data.
test-plan · fct-wok64pug@1 · Per session, cleanup removes the harness home, session.json and the supervisor.
test-plan · fct-7tiauhj7@1 · The error-body check reads a hit of a planted value as class planted and reports place and class, never bytes.
obs-plan · fct-xpt2kzwy@1 · A panic before any role file is opened (clap parsing, home and instance checks) leaves no log record.
obs-plan · fct-bprhan45@2 · A refused diagnostics-path check is exactly a failed open: the role writer ends at a sink and the detail line is dropped.
obs-plan · fct-dujo6hv4@1 · Refusals of the instance and home-tree checks that run before a role file opens write no line in any log.
obs-plan · fct-25dvifnk@1 · Each MCP refusal maps to exactly one UnreachableCause word.
obs-plan · fct-wmupioom@1 · The NotADirectory refusal message is fixed and names no path.
obs-plan · fct-4gke2rkl@1 · The CLI and MCP clients screen a peer reply to code words and a sanitised fault before using it, adding no span and no line.
obs-plan · fct-474ly2yg@1 · Cross-process joins rely on conn being added to the channel client's params and server logging.
obs-plan · fct-rar73ens@1 · The subscriber flattens event fields to the top level of each line, and the Targets filter is the only configurable level gate.
