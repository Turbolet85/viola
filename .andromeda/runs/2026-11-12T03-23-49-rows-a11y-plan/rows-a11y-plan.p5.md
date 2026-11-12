@row_add n=1 row=decision/cli-screen-reader-test-setup area=screen-reader-test-pattern rec=1
name: CLI screen-reader test setup
rule: The cli surface is screen-reader tested with Orca on GNOME Terminal.
@row_add n=2 row=decision/tui-screen-reader-test-setup area=screen-reader-test-pattern rec=1
name: TUI screen-reader test setup
rule: The tui surface is screen-reader tested with Orca on foot under Hyprland.
@row_add n=3 row=decision/wrapped-tui-reads-as-unwrapped area=screen-reader-test-pattern rec=1
name: Wrapped TUI reads as unwrapped
rule: The wrapped claude TUI must read to a screen reader the same as the unwrapped claude TUI.
why: Must-Work O1
@row_add n=4 row=contract/structured-violation-json/cli-tui-a11y-rows area=structured-violation-json-schema rec=2
boundary: Structured violation JSON
item: a11y rows for cli and tui
promise: The cli and tui surfaces emit no a11y rows.
@row_add n=5 row=contract/structured-violation-json/suites-failures area=structured-violation-json-schema rec=2
boundary: Structured violation JSON
item: suites[].failures[]
promise: Output-discipline failures of cli and tui surface only as nextest failures in suites[].failures[].
