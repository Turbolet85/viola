# tests · standing conventions

- S1 · each new test function is named by the test naming rule · 2-test-strategy
  decision/test-function-naming · Test function naming — Test functions are named <subject>_<condition>_<expected> in snake_case. · except: critical-path scenarios are named path<N>_<slug>
- S2 · each new table of cases is written as an rstest case table with literal oracles · 2-test-strategy · 11-test-anti-patterns-never-do-these
  decision/cli-outputs-pinned-by-literal · CLI outputs pinned by literal asserts — CLI and hook integration tests run the built viola binary as a process and pin its outputs by literal asserts.
  decision/human-layouts-pinned-by-literal · Human-mode layouts pinned by literal — Human-mode layouts, the viola list board and viola --help, are pinned by literal asserts in their test files. · guard: HELP literal in cli_output_plain; cli_list board tests
  decision/human-output-pinned-by-literals · Human output pinned by literals — Fixed human output, such as the send mirror, the grouped help, the list board and verify's lines, is pinned by a literal in the test file itself.
  decision/rstest-case-tables · Table-driven unit cases in rstest — Table-driven unit cases, including the refusal-order, exit-code and fixture-replay matrices, use rstest case tables, each case named with a readable label. · why: so no manifest edge is added for rstest alone · except: viola-e2e and viola-core carry no rstest dev-dependency; their table-driven cases, and the own cases of a shared helper taken by path, which run in every taker, are labelled tables in one test.
  decision/rstest-fixtures · rstest for fixtures — rstest is what provides test fixtures and cases.
- S3 · each new test waits on events or bounded reads, never on sleeps, and is never retried · 2-test-strategy · 11-test-anti-patterns-never-do-these
  decision/boot-readiness-file-probe · Boot readiness is a file-state probe — boot polls file state at a bounded 100 ms interval against a deadline, in stages, never by sleep-based synchronisation.
  decision/ci-profile-no-retries · CI test profile without retries — The CI nextest profile runs with retries = 0, fail-fast off, a JUnit report, and a default per-test kill at 120 s (30 s period, four periods).
  decision/control-file-gating · control-file gating of fake agent steps — Each gated step of a fake agent turn script waits for the next line the test appends to the agent's control file, never for a sleep. · why: Deterministic ordering without sleeps.
- S4 · each new fixture file carries synthetic, scrubbed content only and passes the hygiene walk · 7-test-data-fixtures · test-data-fixtures
  contract/mcp-fixture-scrub-refusal/absolute-path · MCP fixture scrub refusal codes · absolute-path — A closed refusal code of the MCP fixture scrub, naming the file only.
  contract/mcp-fixture-scrub-refusal/email · MCP fixture scrub refusal codes · email — A closed refusal code of the MCP fixture scrub, naming the file only.
  contract/mcp-fixture-scrub-refusal/home-path · MCP fixture scrub refusal codes · home-path — A closed refusal code of the MCP fixture scrub, naming the file only.
  contract/mcp-fixture-scrub-refusal/username · MCP fixture scrub refusal codes · username — A closed refusal code of the MCP fixture scrub, naming the file only.
  decision/synthetic-fixture-text · synthetic fixture text only — Fixture prompts and checked-in fake agent scenario scripts hold synthetic probe text only, never real prompts.
  decision/synthetic-inputs-with-canary · Synthetic test inputs carry a canary — Every integration and E2E input is synthetic and embeds one fixed canary, which never appears in a home-level diagnostics line, only in events.ndjson and instance detail files.
- S5 · each new parser surface gets proptest coverage with committed seeds · unit-test-strategy · test-data-bootstrap
  decision/proptest-seeds-committed · Property tests replay from committed seeds — Property tests replay deterministically: proptest regression seeds persist in committed regression files.
  decision/seeded-generators · Seed-controlled generators — Non-deterministic generators are always seed-controlled, and `proptest-regressions/` is committed.
