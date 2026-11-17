import sys
p = sys.argv[1]
t = open(p, encoding="utf8").read()
B = {
    "run = 'cargo deny check'\nrole = 'probe'\nnew = true\n":
        "baseline = 'red — exit 1, error[unmaintained] RUSTSEC-2026-0543, advisories FAILED (the subject step 1 builds); must-pass green in a scratch clone with the ignore, run dir p5-controls.md'\n",
    "run = 'cargo deny --manifest-path fuzz/Cargo.toml check advisories sources'\nrole = 'probe'\nnew = true\n":
        "baseline = 'green — exit 0, advisories ok, sources ok, three advisory-not-detected warnings; green in the clone with the ignore too'\n",
    "run = '''python3 -c \"import tomllib; print(' '.join(sorted(e['id'] for e in tomllib.load(open('deny.toml','rb'))['advisories']['ignore'])))\"'''\nrole = 'probe'\nnew = true\n":
        "baseline = 'red — last line RUSTSEC-2026-0457 RUSTSEC-2026-0461 RUSTSEC-2026-0462 (the id step 1 adds absent); must-pass green in the clone with the ignore, run dir p5-controls.md'\n",
    "run = 'bash scripts/deny-probes.sh'\nrole = 'probe'\nnew = true\n":
        "baseline = 'green — deny-probes: 14/14 banned, control clean'\n",
    "run = 'cargo nextest run -p viola-e2e --lib'\nrole = 'unit'\nnew = true\n":
        "baseline = 'green — exit 0, the harness library unit cases passed'\n",
    "run = '''bash scripts/agent-run.sh run --integration --filter 'binary(harness_tied)' '''\nrole = 'integration'\nnew = true\n":
        "baseline = 'green — nextest-integration 2 passed, 0 failed (the standing killed-session witness and its victim entry; steps 5 and 6 add their cases, whose control is implement red-before-green)'\n",
    "run = '''bash scripts/agent-run.sh run --integration --filter 'package(viola-e2e) & (binary(harness_lifecycle) | binary(cli))' '''\nrole = 'integration'\nnew = true\n":
        "baseline = 'green — nextest-integration 20 passed, 0 failed (the boots untied today)'\n",
    "run = '''bash scripts/agent-run.sh run --e2e --filter 'binary(mcp_tools) | binary(mcp_recorded_exchange)' '''\nrole = 'e2e'\nnew = true\n":
        "baseline = 'green — nextest-e2e 11 passed, 0 failed'\n",
    "run = '''bash scripts/agent-run.sh run --e2e --filter 'binary(contract_lints)' '''\nrole = 'e2e'\nnew = true\n":
        "baseline = 'green — nextest-e2e 7 passed, 0 failed'\n",
}
for k, v in B.items():
    assert t.count(k) == 1, k
    t = t.replace(k, k + v)
open(p, "w", encoding="utf8").write(t)
print("baselines written:", len(B))
