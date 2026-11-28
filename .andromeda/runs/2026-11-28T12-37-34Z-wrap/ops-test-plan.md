@row_set n=1 row=contract/strict-modes-refusal/no-line cell=promise seen=5 detector=none
value: A refused check at run, viola revive, viola adopt, viola verify, viola focus, viola console's start, viola up's board scan, the frame verbs, the hook and the MCP frame tools writes no line.
basis: fct-tol6uebc
because: The console now writes one process-exit line, detail strict-modes-failed, on a refusal mid-run, so only its start's refusal stays line-free.
@row_set n=2 row=decision/home-read-rule cell=rule seen=2 detector=none
value: A home another user may read is refused: viola run exits 1 with the home pair and creates no diagnostics, viola list refuses the whole board, opening nothing, and viola console exits 1 with its home pair, writing no line at its start and one process-exit line when refused mid-run.
basis: fct-aspdcuf3
because: A home widened while the console runs now ends it with exit 1, the home pair and one process-exit strict-modes-failed line, so the console's writing no line holds at its start alone.
@row_set n=3 row=contract/strict-modes-refusal/console cell=promise seen=1 detector=none
value: At its start, exit 1 with the home pair, made before any log opens: no line, the mode left as found; on a board scan refused mid-run, exit 1 with the home pair after the terminal's restore and one process-exit line, detail strict-modes-failed.
basis: fct-tol6uebc
because: The console now re-reads the board under the strict-modes scan while it runs, and a refusal there is logged by one process-exit line after the log has opened.
@row_set n=4 row=decision/tests-never-read-host-sessions cell=rule seen=1 fact=F1 detector=none
value: A test's own list child, and every start of a verb that reads the claude agents list in the background, runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.
basis: fct-cdtfr2rm
because: The console's new agents thread made every existing console test start a reader of the host's claude through the inherited PATH, so the rule now binds every start of such a verb, not only its new cases.
@row_set n=5 row=decision/fake-agent-on-test-path cell=rule seen=3 detector=none
value: The fake agent stands as claude in a test-scoped bin dir, or as hyprctl or as foot, the terminal stand-in, by its own file name, answering from files beside its exe with no env var, and every test-started list, revive or console runs under a test-scoped PATH.
basis: fct-ezqgpxkh
because: Every tui_console start now runs with PATH set to an empty test-scoped bin dir, so the console joins the verbs started under a test-scoped PATH.
@row_set n=6 row=decision/agents-stub-from-files-no-env cell=rule seen=1 detector=none
value: claude agents --json is answered in tests by the fake agent placed as claude in a test-scoped bin dir, from files beside its exe and with no env var, and every test-started list, revive or console runs under a test-scoped PATH.
basis: fct-ezqgpxkh
because: The console now reads the claude agents list, and every one of its test starts runs under a test-scoped PATH.
@row_add n=1 row=decision/fleet-console-proven-live area=e2e-test-strategy fact=F2 detector=none
name: Real fleet console use proven only live
rule: The fleet console's view is proven only over the fakes; a tiled window usable at a quarter screen, the fleet kept current on the real home, the cost of the real agents read at its interval and real log sizes against the board re-read interval are proven only by the gated live reading v2-14.
basis: fct-vecccir6
because: The chunk proves the view over the fakes alone and leaves its live half unproven until the gated reading v2-14, a standing limit future work must respect.
@row_add n=2 row=contract/list-json/last-activity area=e2e-test-strategy fact=F4 detector=none
boundary: list --json
item: last-activity
promise: No list --json item and no MCP list item carries the board item's last-activity time; the board item never serialises it.
basis: fct-xjbaemp6
because: The board item gains a last-activity time kept off serialisation, and the list and MCP documents promise outward that no such key appears.
@row_none n=1 fact=F3 type=4
relied: The console's skip of an unchanged log relies on a board read once relived matching the full board read; the re-read interval's injection through the console's sources is how the loop is built, held by the code.
