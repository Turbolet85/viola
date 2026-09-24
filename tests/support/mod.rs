//! Sync fixtures shared by the root `tests/` binaries (test-plan §3 `run` step 2): the chain
//! `home → fake_agent_path → stamped_home → booted_wrapper`, fake-agent helpers and the fixture
//! hygiene checker. Each binary declares `mod support;` and uses what it needs.

pub mod fake;
pub mod home;
pub mod hygiene;
