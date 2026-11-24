@row_set n=1 row=decision/exec-commands-absolute-pinned-path cell=rule seen=1 detector=none
value: Every exec-form command viola writes for the CLI is an absolute path, never a PATH lookup: the pinned bin/<version>-<hash>/viola path for run, revive, adopt and verify, and the installing exe's canonical path in plugin install's `.mcp.json`.
basis: fct-zdl3zlbi
because: Adopt's start runs run's start, which pins viola and rewrites plugin/ with the pinned path, so the starts the rule names gain adopt's.
