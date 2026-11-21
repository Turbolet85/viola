@row_set n=1 row=decision/scrubbed-fixtures cell=rule seen=1 detector=none
value: Fixtures never come from real prompts and are checked for home, drive-letter and user-folder paths and usernames.
basis: fct-wn36g3ew
because: The check flags only home, drive-letter and user-folder paths, so «absolute paths» is false: the committed cross-session arrival's runtime-dir path passes it.
@row_set n=2 row=decision/fixtures-walked-for-hygiene cell=rule seen=2 detector=none
value: Every committed fake script, Claude fixture and Hyprland fixture is walked against its schema and for home, drive-letter and user-folder paths, usernames and emails, with planted cases proving each class red.
basis: fct-wn36g3ew
because: The same measured reach of the walk; «absolute paths» is false.
