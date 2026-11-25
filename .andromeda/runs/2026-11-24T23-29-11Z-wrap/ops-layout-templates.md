@row_add n=1 row=contract/viola-adopt-refusals/not-resumed area=surface-cli fact=F1 detector=D-design-cli-wording
boundary: viola adopt refusals
item: session not resumed
promise: A child ending before its session starts exits 1: `unable: <name>'s session did not resume` / `hint: check the session id - claude --resume <id> must open it by hand, under the same claude config`; under `--json` `{"v":1,"error":"not-resumed","detail":null}`, stderr empty; no line holds the id.
basis: fct-x4brbvla, fct-blcs6o5x, fct-isor5lni
because: The chunk adds adopt's third own refusal with a new fixed pair and --json code, and no row of the cli surface states its wording.
@row_add n=2 row=decision/adopt-not-resumed-written-after-child area=surface-cli fact=F2 detector=none
name: Adopt's not-resumed end written after the child
rule: Once a started claude child has held the terminal, viola writes nothing of its own past it.
except: viola adopt's not-resumed end writes its unable/hint pair, or its --json document, after the child has released the terminal; nothing of run or revive.
basis: fct-x4brbvla, fct-j2uatzhk, fct-uteui6y3
because: The chunk makes adopt's not-resumed end write after the child released the terminal, an exception scoped to that one case that the cli surface's rows do not name.
