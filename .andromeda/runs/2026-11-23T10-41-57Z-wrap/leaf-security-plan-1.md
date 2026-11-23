@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=29 class=omission
leaf-text: an instance's lineage (parent and role names) | Typed fields in diagnostics; paths never in external errors; lineage kept only in the snapshot and one lineage line of the event log at start, with no process-log line, span or terminal byte |
source: dec-62b4vb7f «An instance's launch pair is kept only in the snapshot's launch field and the events log's launch line, log-only with no span, process-log line or terminal byte»
replace-with: an instance's lineage (parent and role names), its launch pair (model and effort words) | Typed fields in diagnostics; paths never in external errors; lineage kept only in the snapshot and one lineage line of the event log at start, with no process-log line, span or terminal byte; the launch pair kept only in the snapshot's launch field and the events log's launch line, with no span, process-log line or terminal byte, leaving only through `list --json` and the MCP list item |
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=64 class=omission
leaf-text: a role only when it is a code word and a link pair only when both names parse
source: dec-ajbksxgt «The board shows a recorded model only when it passes the model-word check and a recorded effort only when it passes the code-word check.»
replace-with: a role only when it is a code word, a recorded model only when it passes the model-word check, a recorded effort only when it passes the code-word check, and a link pair only when both names parse
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=65 class=omission
leaf-text: the founder ratified that second crossing too (live, 2026-10-10, relayed verbatim by the overseer).
source: dec-lvb2p7da «A ratified crossing; revive handing the recorded pair onto the resumed child's argv is a second ratified crossing.»
replace-with: the founder ratified that second crossing too (live, 2026-10-10, relayed verbatim by the overseer). It hands the recorded launch pair onto the resumed child's argv only as four separate argv elements of a direct spawn (`--model` and its word, then `--effort` and its word), never through a shell — a further ratified crossing.
