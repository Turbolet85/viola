@answer n=1 entity=test-data-fixtures#7.1 v=no
why: section 7 is untouched and no fixture or schema moved, and the walk lives in tests/contract_fixture_hygiene.rs, which the change did not reach
@answer n=2 entity=test-data-fixtures#7.2 v=no
why: the unclean-fixture refusal is verify's product behaviour and schemas/diag-line.v1.json, neither of which moved, so the change does not alter it
