@row_retire n=1 row=decision/identity-floor-eleven-names seen=1 detector=D-arch-resources
why: The identity floor no longer holds eleven names, which the row's key, name and member list are made from.
basis: fct-yw2wvymn
because: IDENTITY_FLOOR grew by CLAUDE_CODE_CHILD_NOTES_DIR and CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS, so a row keyed on eleven cannot carry the floor.
@row_add n=1 row=decision/identity-floor-names area=occupied-resources detector=D-arch-resources
name: Identity floor names
rule: The identity floor is CLAUDECODE, CLAUDE_EFFORT, CLAUDE_PID and the CLAUDE_CODE_ names BRIDGE_SESSION_ID, CHILD_NOTES_DIR, CHILD_SESSION, ENTRYPOINT, EXECPATH, LAUNCHED_BY_SESSION_IDS, MESSAGING_SOCKET, MESSAGING_TOKEN, SESSION_ATTENDED and SESSION_ID.
why: The plugin and project names a hook sees are not identity names, and the prefix rule already removes them when inherited.
basis: fct-yw2wvymn, fct-wi3sokcx
because: The floor now holds two more parent-identity names, and the floor row must name every member while keeping the standing why whole.
@row_set n=1 row=decision/r8-identity-floor cell=rule seen=1 detector=none
value: The identity-floor CLAUDE* parent-identity environment names, measured or read from the CLI bundle, are stripped from the child even when the persistent environment defines them; every other CLAUDE* name follows the prefix rule.
basis: fct-wi3sokcx
because: One floor name, CLAUDE_CODE_LAUNCHED_BY_SESSION_IDS, stands on a reading of the CLI bundle's strings, never met live, so «measured» alone no longer holds for the whole floor.
