@row_set n=1 row=decision/verify-child-set cell=rule seen=2 fact=F6 detector=D-arch-decisions
value: viola verify makes one print-mode probe, six interactive PTY runs under the R8 strip at a fixed 80×24, one agents --json read inside Run B and one print-mode sender, verify-sender, inside Run B's cross-session turn, pasting only compiled texts.
basis: fct-zjeenbpz
because: Verify now spawns a print-mode sender child in Run B's cross-session turn, so the bounded child set the row names is no longer whole.
@row_set n=2 row=decision/no-environment-configuration cell=except seen=3 fact=F3 detector=D-arch-resources
value: NO_COLOR, TERM, COLORTERM for colour; XDG_RUNTIME_DIR for the socket dir and, read once by verify, the sender's address; PATH for lookup; two fake-agent test seams; kept CLAUDE_CONFIG_DIR's value
basis: fct-zjeenbpz
because: viola verify now reads XDG_RUNTIME_DIR once to compose the cross-session sender's address, a ratified exception the row's list must carry.
@row_set n=3 row=contract/hook-capture-arm/payload-file cell=promise seen=1 detector=D-arch-resources
value: Without --names-only, stdin, capped at MAX_FRAME, is written raw to an exclusively claimed <PascalEvent>.<k>.json in the given absolute, existing folder.
basis: fct-zjeenbpz
because: The new hidden --names-only mode files no payload byte, so the raw-write promise now holds only for the plain capture arm.
@row_set n=4 row=decision/recorded-fixtures-scrubbed cell=except seen=2 fact=F7 detector=none
value: Rig-only --record-first-screens keeps a failed verify's scrubbed first screens short of input box, Run A's trust excepted; a recorded cross-session arrival keeps the sender's socket and from-name
basis: fct-wn36g3ew
because: The recorded cross-session arrival keeps, inside the CLI's own prefix, the recording host's runtime-dir path and sender pid, which the scrub and hygiene walk pass, so the row's exceptions must name it.
@row_add n=1 row=decision/cross-session-prefix-ledger-row area=established-decisions fact=F1 detector=none
name: Cross-session-prefix ledger row and its readings
rule: The cross-session-prefix relied ledger row reads pass only when a print-mode sender's message to verify's Run B, started with the crossSessionInbound accept settings, arrives opening with the CLI's cross-session harness prefix, else fail, never on the sender's words; no runtime dir runs no turn.
basis: fct-zjeenbpz
because: The chunk lands the 29th relied ledger row, a decision beside the other per-row ledger decisions that no row yet states.
@row_add n=2 row=decision/cross-session-send-by-uds-address area=established-decisions fact=F2 detector=D-arch-decisions
name: Cross-session sends addressed by inbox socket
rule: A local claude session is addressed for a cross-session send by its inbox socket address uds:<runtime>/cc-socks/<pid>.sock, never by a bare session id; the address is composed only from an absolute runtime dir with no . or .. component, holding only [A-Za-z0-9/._-], its path within 103 bytes.
why: The CLI resolves a bare session id to no local session.
basis: fct-vgvx3eex
because: The bare-id addressing read red live and was replaced by the composed uds address, a decision future cross-session work must respect.
@row_add n=3 row=contract/hook-capture-arm/names-only area=occupied-resources fact=F4 detector=D-arch-resources
boundary: viola hook --capture arm
item: --names-only
promise: A hidden mode valid only with --capture: on PreToolUse or PostToolUse it files only owner-only Tool.<k>.json files of closed tool and send-result words, no payload byte; a spine event or a relative dir files nothing.
basis: fct-zjeenbpz
because: The chunk adds a hidden --names-only mode to the capture arm, a new surface the arm's contract items do not name.
@row_add n=4 row=contract/ledger-stamps/cross-session-probe area=standard-contracts fact=F5 detector=D-arch-resources
boundary: ledger stamps envelope
item: measured.cross_session_probe
promise: Holds outcome, prefix, latency_ms, sender_tools and send_result from the cross-session probe, closed words and one number.
basis: fct-wvhiwmit
because: The stamp gains a new measured key beside env_probe and first_screen, a per-key contract of the envelope.
@row_add n=5 row=name/other/verify-sender area=occupied-resources detector=D-arch-resources
class: other
name: verify-sender
basis: fct-wvhiwmit
because: verify-sender is a new diag-line subject naming verify's print-mode sender child, an occupied name the register must hold.
@row_add n=6 row=decision/test-verify-owned-runtime-dir area=occupied-resources fact=F8 detector=none
name: Test verify runs given a test-owned runtime dir
rule: Every test or harness start that stamps the relied rows gives viola verify a short owner-only runtime dir the test side owns.
why: The pre-push's env -i leaves XDG_RUNTIME_DIR unset, and verify reads it to compose the sender's address.
except: The rig rehearsal sets its own runtime dir only where the host sets none.
basis: fct-m4yamtxm
because: Verify's one XDG_RUNTIME_DIR read makes every relied-row stamp depend on a runtime dir, so the test side must supply one under the pre-push's clean environment.
