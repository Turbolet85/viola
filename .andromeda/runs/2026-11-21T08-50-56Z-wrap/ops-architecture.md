@row_retire n=1 row=decision/identity-floor-eleven-names seen=1 heir=decision/identity-floor-names detector=D-arch-resources
why: The identity floor no longer holds eleven names, so the row's key and name are false; its heir names the floor without a count.
basis: fct-yw2wvymn
because: IDENTITY_FLOOR grew by CLAUDE_CODE_CHILD_NOTES_DIR and CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS, and a key made from the old count cannot carry the new floor.
@row_add n=1 row=decision/identity-floor-names area=occupied-resources fact=F1 detector=D-arch-resources
name: Identity floor names
rule: The identity floor is CLAUDECODE, CLAUDE_EFFORT, CLAUDE_PID and the CLAUDE_CODE_ names BRIDGE_SESSION_ID, CHILD_NOTES_DIR, CHILD_SESSION, ENTRYPOINT, EXECPATH, LAUNCHED_BY_SESSION_IDS, MESSAGING_SOCKET, MESSAGING_TOKEN, SESSION_ATTENDED and SESSION_ID.
why: CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS stands on a reading of the CLI bundle's strings alone, never met live; the plugin and project names a hook sees are not identity names.
basis: fct-yw2wvymn, fct-wi3sokcx
because: The floor now holds two more parent-identity names, one of them never met live, and the floor row must name every member.
@row_add n=2 row=decision/identity-floor-ledger-row area=established-decisions fact=F3 detector=none
name: Identity-floor ledger row and its readings
rule: The identity-floor relied ledger row passes when every CLAUDE name the CLI adds to a SessionStart hook is an identity-floor name or a known non-identity name, and fails on an unknown name or an unread or off-shape env-names file; kept names are dropped before the check.
basis: fct-yw2wvymn, fct-d6aupwop
because: The chunk lands identity-floor as a new relied ledger row with its own verify probe, and no row states when it passes or fails.
@row_add n=3 row=contract/hook-capture-arm/env-names-file area=occupied-resources fact=F4 detector=D-arch-resources
boundary: viola hook --capture arm
item: env-names file
promise: On SessionStart alone the capture arm also writes Env.<k>.json owner-only beside the payload, holding the CLAUDE names of its own process with every other name only counted; no value is ever read or written.
basis: fct-yw2wvymn, fct-cyougjyd
because: The capture arm now occupies a second file name, Env.<k>.json, which verify reads, and the arm's contract names only the payload file.
@row_add n=4 row=contract/ledger-stamps/env-probe area=standard-contracts fact=F5 detector=D-arch-resources
boundary: ledger stamps envelope
item: measured.env_probe
promise: Holds read, added, unknown, other_count and unshaped from the identity-floor probe, names only, with every kept name removed.
basis: fct-yw2wvymn, fct-y2ah5a4y
because: The stamp gains a measured.env_probe block, and the envelope's measured items stand one a row.
@row_add n=5 row=contract/recorded-env-fixture/env-recorded-1-json area=occupied-resources fact=F6 detector=D-arch-resources
boundary: recorded env-names fixture
item: Env.recorded-1.json
promise: viola verify --record writes the file through the record scrub as {v:1, added:[names]}, names only; the hygiene walk checks it as its own env fixture class, and an env name carrying the user word is refused.
basis: fct-yw2wvymn, fct-y2ah5a4y
because: A new recorded fixture class with its own format lands, read by the fake agent and the hygiene walk, and no row registers it.
@row_add n=6 row=decision/fake-agent-replays-env-names area=conventions fact=F7 detector=none
name: Fake agent replays recorded env names
rule: The fake agent hands each name of a fixture set's recorded env-names file to every SessionStart hook child, its value a compiled fake-agent word, never a recorded value.
basis: fct-yw2wvymn, fct-v4enyx24
because: The fake agent now replays the recorded env names so the identity-floor row reads in CI, and its values are never recorded ones.
@row_set n=1 row=decision/plugin-env-vars-unused cell=rule seen=1 fact=F2 detector=D-arch-decisions
value: viola never uses CLAUDE_PLUGIN_ROOT or CLAUDE_PLUGIN_DATA to locate its binary and reads no value of those, CLAUDE_ENV_FILE or CLAUDE_PROJECT_DIR; the four are known non-identity hook names, read by name only by the identity-floor ledger check.
basis: fct-yw2wvymn
because: NON_IDENTITY_HOOK_NAMES now holds the four names and the identity-floor check reads them by name, so the row's «reads none» no longer holds whole.
@row_set n=2 row=decision/r8-identity-floor cell=rule seen=1 detector=none
value: The identity-floor CLAUDE* parent-identity environment names are stripped from the child even when the persistent environment defines them; every other CLAUDE* name follows the prefix rule.
basis: fct-wi3sokcx
because: One floor name, CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS, stands on a bundle reading never met live, so the floor is no longer the measured names alone.
@row_set n=3 row=decision/probe-capture-arms-inert cell=rule seen=2 detector=D-arch-resources
value: The hidden hook `--capture` arm takes an absolute, existing directory only, reads no `VIOLA_*` variable and no environment value, starts no observability, opens no channel, and always exits 0 with empty stderr.
basis: fct-yw2wvymn, fct-cyougjyd
because: The capture arm now reads its own process's environment names to write the env-names file, so the inert rule must say it reads names only, never a value.
