@task_prose part=chunk
Pins, by tests only, the exact-frame case on both MAX_FRAME bounds of the viola-hyprland seam: a reply of exactly MAX_FRAME bytes is accepted, MAX_FRAME + 1 bytes is oversize.
The bounded read is driven through the fake hyprctl; the dispatch parse is given the bytes directly (exactly MAX_FRAME bytes give Refused, not oversize).
Each new guard test carries a one-off remove-the-guard control that mutates its site (> to >=) and must see the test fail.
No product line changes unless a test proves one wrong; the clients parse bound is already pinned and is not re-pinned.
@task_file n=1 path=crates/viola-hyprland/src/read.rs mark=edits
what: the Hyprland seam's bounded read of a hyprctl reply, refusing a reply over MAX_FRAME as oversize — edits: the test module, new exact-frame tests for read_within
@task_file n=2 path=crates/viola-hyprland/src/reply.rs mark=edits
what: the Hyprland seam's reply parsers, refusing a reply over MAX_FRAME as oversize — edits: the test module, new exact-frame tests for parse_dispatch
@task_item n=1 part=needs
The MAX_FRAME bound on the Hyprland seam's bounded read of a hyprctl reply: exactly MAX_FRAME bytes accepted, one byte more refused as oversize.
@task_item n=2 part=needs
The MAX_FRAME bound on the dispatch reply parse: exactly MAX_FRAME bytes not oversize (refused as not the success text), one byte more refused as oversize.
@task_item n=3 part=needs
The oversize error of the Hyprland seam as the refusal both bounds return, and the refused outcome the dispatch parse returns for a reply that is not its success text.
@task_item n=4 part=needs
The read's tests run only through the fake hyprctl the seam's tests already use: no real hyprctl, no live desktop step, no live claude start.
@task_item n=5 part=needs
Each new guard test owes a one-off remove-the-guard control on its site (> to >=), named by the test's name alone, in which that test runs and fails.
@task_item n=6 part=needs
Test-only: no product behaviour of the Hyprland seam changes, no requirement is added and no capability claimed; a defect a test proves folds in as its own item.
@task_item n=1 part=not_needed
The cli surface's decisions on human output, styling, hints, tokens and the help, board, send, wait, revive, run, verify and link outputs: the chunk adds no output and changes no verb.
@task_item n=2 part=not_needed
The CLI exit codes, refusal lines, --json documents and exit-21 details: no refusal or exit path reaches a user through this chunk.
@task_item n=3 part=not_needed
The viola mcp tools and their results: the chunk touches no MCP tool.
@task_item n=4 part=not_needed
The web-spa surface and its absence of a web page, and the absence of deep links in the CLI: the chunk renders nothing.
@task_item n=5 part=not_needed
The release --budget output and the wheel verb outputs: the budget gate is not this chunk's work.
