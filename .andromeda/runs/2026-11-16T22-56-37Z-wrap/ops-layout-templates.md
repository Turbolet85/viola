@row_add n=1 row=contract/wait-turn-ended-json/scheduled-wakes area=surface-cli fact=F1 detector=none
boundary: viola wait --json and MCP wait turn-ended result
item: scheduled_wakes
promise: For a turn end, the count of the session's scheduled wakes beside background_tasks, one number and no wake content; null when the CLI sent none or an unreadable value, and null on an API-error turn end; viola last, viola list, the snapshot and the human wait output do not carry it.
basis: fct-wkxfnidr
because: The turn-ended data gains scheduled_wakes on the wait --json document and the MCP wait structuredContent, a new field a driver reads at the CLI and MCP boundary, so the surface needs its promise stated.
@row_add n=2 row=name/other/scheduled-wakes area=surface-cli fact=F2 detector=none
class: other
name: scheduled_wakes
basis: fct-wkxfnidr
because: The name scheduled_wakes is now occupied as a turn-ended data key on the --json and MCP surfaces, the CLI's own session_crons never surfacing by that name, so the register must hold it.
