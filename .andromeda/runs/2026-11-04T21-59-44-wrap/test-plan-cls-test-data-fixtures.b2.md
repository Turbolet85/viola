@answer n=1 entity=test-data-fixtures#7.1 v=no
why: the session moved no fixture, schema or walk file and tests/contract_fixture_hygiene.rs is not among its changed files, so the fixture walk claims still hold (section 7 untouched).
@answer n=2 entity=test-data-fixtures#7.2 v=no
why: the Ended::Refused and unclean-fixture reading is verify product behaviour in src/ and schemas/diag-line.v1.json, which the session did not move, so the claim still holds.
