@row_add n=1 row=decision/no-interactive-terminal-interface area=obs-scope-summary detector=none
name: No interactive terminal interface
rule: viola offers no interactive terminal interface: no prompt, TUI or pager.
except: the terminal fleet console (`viola console`), its own surface, which reads keys in raw mode and redraws its frame
basis: fct-xexxpxch
because: The chunk built viola console, so the standing absence of an interactive terminal interface is false whole; it leaves for this decision carrying the console as its one named exception.
@row_set n=1 row=decision/structured-json-process-logs cell=rule seen=1 detector=none
value: Every process role (run, hook, mcp, cli, console) writes structured JSON process logs under diagnostics/, which harness logs merges with events.ndjson.
basis: fct-twjnmpkt
because: The proposer's set was read not_master for its added count; the standing row's role list is false without console, so this shorter set adds the one word.
@row_retire n=1 row=absence/interactive-terminal-interface seen=1 heir=decision/no-interactive-terminal-interface
why: viola console is an interactive terminal interface, so the capability is no longer absent; the decision row that replaces it names the console as its one exception.
basis: fct-xexxpxch
because: The chunk built viola console, which reads keys and redraws, so the absence row is false.
