### Service identity

- **service.name:** hardcoded `"viola"` as `viola_core::SERVICE_NAME`. This is the arch-fixed binary name, already served as `"name":"viola"` by `/health` and `/api/info`. It is never read from `OTEL_SERVICE_NAME` / `SERVICE_NAME`: arch says env vars are not a configuration channel.
- **service.version:** compile-time `pub const VERSION: &str = env!("CARGO_PKG_VERSION");` in viola-core. Every crate uses `version.workspace = true` and reads this one constant, so the log `version`, channel `sender`, snapshot `writer` and `/health.version` cannot drift apart.
- **deployment.environment:** N/A. viola is local-only with no hosting (arch Surfaces). No env-var fallback exists: env vars are not a configuration channel.
- **Resource attributes:** no OTel Resource is built in v1. The identity is emitted as fields on `process-start`: `service_name`, `version`, `os` (`windows|macos|linux` from `std::env::consts::OS`), `pid`. Every line carries the role (`process`) and the instance (`instance`). If the deferred bridge is ever adopted, use `Resource::builder_empty()` with these same values.
