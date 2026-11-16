# security · standing conventions

- S1 · each new test runs under its own viola home, never the user's real home · secret-management
  decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.
  decision/tests-use-own-home · Tests use their own home — Development handles secrets as production does, and tests use their own --home directory.
- S2 · each new test seam lives only under the fake-agent feature and is compiled out of release · security-anti-patterns
  decision/fake-agent-hook-panic-seam · Hook panic test seam — FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction. · why: Ratified.
- S3 · each new or re-recorded fixture is scrubbed of paths, usernames and emails and reviewed before commit · data-protection
  contract/fixture-refusal-codes/absolute-path · verify's unclean-fixture refusal codes · absolute-path — A closed check code of the fixture refusal, for a surviving absolute path.
  contract/fixture-refusal-codes/email · verify's unclean-fixture refusal codes · email — A closed check code of the fixture refusal, for a surviving email-shaped token.
  contract/fixture-refusal-codes/home-path · verify's unclean-fixture refusal codes · home-path — A closed check code of the fixture refusal, for a surviving home path.
  contract/fixture-refusal-codes/username · verify's unclean-fixture refusal codes · username — A closed check code of the fixture refusal, for a surviving user word.
  decision/fixture-hygiene · Recorded fixture hygiene — Recorded claude fixtures are never committed from sessions with real prompts, nor before being checked for home paths and usernames.
  decision/fixture-recording-scrubbed · Fixture recordings scrubbed — verify --record scrubs the home prefix and user word before any write and refuses the whole recording when a path, the user word or an email-shaped token survives, naming file and closed code, never content.
  decision/fixture-scrub-and-refusal · Recorded fixtures scrubbed or refused whole — Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home or absolute path, the user word or an email-shaped token survives.
  decision/fixtures-reviewed-before-commit · Fixtures reviewed before commit — Recorded fixture payloads are reviewed for home paths, usernames and tool input before committing.
  decision/mcp-fixtures-via-test-only-tap · MCP fixtures recorded through a test-only tap — MCP exchange fixtures are recorded by a test-only tap that spawns viola mcp directly with no shell, caps each line, keeps its raw capture 0600 in a private directory never committed, and applies the same scrub and closed refusal codes. · guard: schemas/claude-mcp-fixture.v1.json
  decision/probe-prompts-synthetic · Probe prompts are synthetic — Probe prompts used to record committed fixtures must be synthetic.
  decision/screen-fixture-signature-rows-only · Screen fixtures keep only signature rows — A screen fixture keeps only signature rows (a row holding a compiled footer or modal literal whole, a box rule row as its rule chars alone, a prompt row as the prompt char alone), the rest blanked; another box row, a kept row the scrub would rewrite or a username split across a row seam is refused.
  decision/viola-format-fixtures-hygiene · viola-format fixtures under the hygiene walk — Committed viola-format fixtures, the old state shapes and retired words, are held to the same fixture hygiene walk as recorded fixtures: no path, user or email. · guard: viola_format_fixtures_pass_hygiene
- S4 · each new seam or environment read needs a log entry · security-anti-patterns-never-do-these
