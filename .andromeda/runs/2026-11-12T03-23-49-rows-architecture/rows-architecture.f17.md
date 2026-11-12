@row_add n=1 row=decision/permission-request-question-stays-null area=established-decisions rec=1
name: A question first raised by PermissionRequest stays null
rule: A question first raised by PermissionRequest stays null to the human.
why: A PermissionRequest allow with updatedInput would satisfy it, but that body has no ledger row; adding one needs a 25th row, its viola verify probe and a re-stamp of every stamped version.

@row_add n=2 row=contract/viola-revive-json/error-document area=established-decisions rec=2,3
boundary: viola revive --json
item: error document
promise: Every refusal of the start, a refused strict-modes check and a preflight refusal among them, prints {"v":1,"error":"<code>","detail":null}, the code one of the ten codes, in place of its unable/hint pair, with stderr empty and exit 1; a pump_child error prints no document.

@row_add n=3 row=decision/fake-agent-pump-delay-seam area=occupied-resources rec=4
name: FAKE_AGENT_PUMP_DELAY_MS test seam
rule: FAKE_AGENT_PUMP_DELAY_MS is read only by a viola built with the test-only fake-agent feature, parsed as milliseconds capped at 5 000, and holds the pump back after the child starts; it disables no control and widens no redaction.
why: Ratified as a security-plan Security Anti-Patterns carve-out: a test seam, not a configuration channel.

@row_add n=4 row=decision/end-to-end-homes-kept-after-each-job area=occupied-resources rec=5,7
name: Homes kept after each job's end-to-end run
rule: Each CI job keeps its harness and test homes after its end-to-end run, whatever the outcome (AGENT_RUN_KEEP_HOMES "1" on the job, if: always()), the viola-chaos-* home out.
why: The homes are kept in CI for the obs gates.
