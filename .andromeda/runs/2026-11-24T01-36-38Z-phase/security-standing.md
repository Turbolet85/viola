# security · standing conventions

- S1 · each new test uses its own home, never the operator's, and removes only the directories it created · secret-management · security-anti-patterns
  decision/e2e-home-removal-own-dirs-only · Test-side removal only of its own dirs — A test-side removal under target/e2e-home never takes anything but a dir the test side itself created.
  decision/test-homes-gitignored · Test homes kept out of git — Local --home test directories are git-ignored so recorded state never lands in git.
  decision/test-removal-own-dirs-only · Test-side removal only of test-created dirs — A test-side removal under the e2e home takes only a directory the test side itself created, by its owner record, its own temp dir or its session name prefix, or another build's stamp dir whose lock it takes with try_lock.
- S2 · each new recorded fixture is scrubbed or refused whole, reviewed before commit and kept under the fixture hygiene walk · data-protection
  contract/fixture-refusal-codes/absolute-path · verify's unclean-fixture refusal codes · absolute-path — A closed check code of the fixture refusal, for a surviving home, drive-letter or user-folder path.
  contract/fixture-refusal-codes/email · verify's unclean-fixture refusal codes · email — A closed check code of the fixture refusal, for a surviving email-shaped token.
  contract/fixture-refusal-codes/home-path · verify's unclean-fixture refusal codes · home-path — A closed check code of the fixture refusal, for a surviving home path.
  contract/fixture-refusal-codes/username · verify's unclean-fixture refusal codes · username — A closed check code of the fixture refusal, for a surviving user word.
  contract/fixture-refusal-line/form · verify's unclean-fixture refusal · line form — The refusal reads `unable: a recorded fixture is not clean: <file> <code>`, or `<file> row <n>[ seam] <code>` for a screen, naming the file and a closed code, never the content.
  decision/fixture-recording-scrubbed · Fixture recordings scrubbed — verify --record scrubs the home prefix and user word before any write and refuses the whole recording when a home, drive-letter or user-folder path, the user word or an email-shaped token survives, naming file and closed code, never content.
  decision/fixture-scrub-and-refusal · Recorded fixtures scrubbed or refused whole — Before any write, verify --record scrubs the home prefix to ~ and the user word to <user>, and refuses the whole recording with exit 1 and nothing written when a home, drive-letter or user-folder path, the user word or an email-shaped token survives.
  decision/fixtures-reviewed-before-commit · Fixtures reviewed before commit — Recorded fixture payloads are reviewed for home paths, usernames and tool input before committing.
  decision/viola-format-fixtures-hygiene · viola-format fixtures under the hygiene walk — Committed viola-format fixtures, the old state shapes and retired words, are held to the same fixture hygiene walk as recorded fixtures: no path, user or email. · guard: viola_format_fixtures_pass_hygiene
- S3 · each new test seam lives only under the fake-agent test feature and is compiled out of the release build · security-anti-patterns
  decision/release-build-no-test-binaries · Release build refuses test-only binaries — The CI release build refuses any test-only binary. · guard: scripts/release-check.sh
  decision/release-build-without-test-features · Release build free of test-only features — The release build is judged from its own compiler-artifact records and refused if it holds any executable other than viola or any artifact built with a test-only feature (test-support, fake-agent). · guard: scripts/release-check.sh, its --probe run first reading 5/5 refused with a clean control
  decision/test-seams-compiled-out · Test seams compiled out of release — Fake-agent test seams exist only in builds with the test-only fake-agent feature, are compiled out of every release build, configure nothing, disable no control and widen no redaction.
- S4 · each new seam or environment read gets its log entry · security-anti-patterns-never-do-these
- S5 · each new error writer is named in the error writer census and its display message is fixed, interpolating nothing · error-handling
- S6 · each new reader of external bytes is bounded or named in the bounded-readers census · input-validation
