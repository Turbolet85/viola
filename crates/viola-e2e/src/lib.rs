//! The headless harness behind `scripts/agent-run.{sh,ps1}` (test-plan §3): `boot · run · status ·
//! cleanup · logs` plus the internal `supervise`. Every command yields one JSON document and a typed
//! exit; the `viola-harness` bin only parses, prints and exits.

pub mod harness;
