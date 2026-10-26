# security · standing conventions

- S1 · every third-party version of the root graph pinned exactly in `[workspace.dependencies]` (rmcp's minor range the one exception) and `Cargo.lock` committed · .andromeda/security-plan-detail/dependency-security.md
- S2 · an audit or gate tool is expressed as a minimum floor, and a gate command is measured at the exact version CI pins · .andromeda/security-plan-detail/dependency-security.md
- S3 · any test the chunk adds runs under its own `--home`, with no secret or real-session content in a fixture · .andromeda/security-plan-detail/secret-management.md
  (.andromeda/security-plan-detail/data-protection.md:26) data-rest
  (.andromeda/security-plan-detail/secret-management.md:15) secrets
