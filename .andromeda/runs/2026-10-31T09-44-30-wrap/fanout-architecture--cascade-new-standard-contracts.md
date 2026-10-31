proposals:
  - detector: D-new-detail
    severity: warning
    fact: F6
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 81
    section: "## Standard Contracts"
    change: "- `viola_channel::ProtocolError::cause(self) -> Option<&'static str>` names the three `-32602` faults `unsupported-version`, `invalid-params` and `release-from-driver`, and is `None` for the other four."
    sidecar: "Standard Contracts, Channel methods: added `ProtocolError::cause`, the three `-32602` cause words and `None` for the other four faults."
    rationale: "The report's Changes, Symbols / APIs, states `ProtocolError::cause` at `crates/viola-channel/src/lib.rs:102-110`; no standing text of the category holds it, and the contract-channel-methods entity is where the `-32602` faults of the methods stand."
    basis: crates/viola-channel/src/lib.rs:103
