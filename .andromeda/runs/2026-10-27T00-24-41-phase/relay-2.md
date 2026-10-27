# P4 fork of chunk 2026-10-27-pinned-copy-race-and-reproducible-builds — red 2's owner

Asked by question dialog at P4 (one question, the recommendation marked first):

## Q1 — Red 2 owner
Question: Who owns CI red 2? On ci#40425866096 (sha 99977116, test (windows-2025)), viola-state's strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry failed at strict.rs:778. The product verdict was right: check_instance returned Err(Writable) as asserted. The test's second assertion requires the inherited write grant to come from Users (S-1-5-32-545). On that runner (same image 20261021.278.1, region northcentralus; the green run's was westus3), Users held read only and Authenticated Users (S-1-5-11) held the write. The red does not touch what this chunk builds.
Options: 1. Fold here (Recommended) · 2. Record it unowned · 3. You mint its owner

Answer, received in the session at 2026-10-27T01:48Z, verbatim: «Fold here (Recommended)», with the notes:

> The overseer's technical answer (founder-delegated; his night word «продолжают работать автономно», overseer1 in charge), 01:48Z: a red met during a chunk folds into it; this is a test-only oracle fix in a crate the chunk already edits, the product verdict was right, and the remove-the-guard control keeps its red. Also from overseer1 for P3/P4: test-plan.md:718 is a blank line; the quoted rule stands at :717, so cite :717.

Authority: delegate overseer1 (founder-delegated, the founder away), PROVISIONAL.
