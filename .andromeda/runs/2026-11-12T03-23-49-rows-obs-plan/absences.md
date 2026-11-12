absence/audit-log-compliance-trail · «(?i)audit» · in src/**/*.rs and crates/*/src/**/*.rs · place unread: it names no tracked file · fct-6eqwgp2k@1
decision/no-network-otlp-exporter · «(?i)otlp|opentelemetry|TcpListener|TcpStream|reqwest|hyper» · in src/**/*.rs and crates/*/src/**/*.rs · place unread: it names no tracked file · fct-4v6f5pir@1,fct-uv7mc5hr@1,fct-fxg4qazp@1
decision/no-error-reporter · «(?i)sentry» · in src/**/*.rs and crates/*/src/**/*.rs · place unread: it names no tracked file · fct-4v6f5pir@1,fct-uv7mc5hr@1
decision/no-env-log-levels · «EnvFilter|from_default_env|with_env_filter» · in src/**/*.rs and crates/*/src/**/*.rs · place unread: it names no tracked file · fct-ljftf7ri@1,fct-uv7mc5hr@1
decision/no-env-log-levels · «env::var» · in src/obs.rs and crates/*/src/**/*.rs · place unread: it names no tracked file · fct-ljftf7ri@1,fct-uv7mc5hr@1
decision/run-sync-instrumentation · «tokio» · in Cargo.toml [dependencies] · place unread: it names no tracked file · fct-ctz7espz@1,fct-bgcxnqje@1
decision/hook-stderr-forbidden · «eprintln!» · in src/cmd/hook.rs and src/cmd/hook/*.rs · place unread: it names no tracked file · fct-qlfp3c3h@1,fct-z66kg5vk@1
decision/hook-stderr-forbidden · «stderr(Stdio::null) is the child's stderr» · in src/cmd/hook/statusline.rs:228 · place unread: it names no tracked file · fct-qlfp3c3h@1,fct-z66kg5vk@1
decision/no-otel-sdk-exporter · «opentelemetry» · in Cargo.toml, src/**/*.rs, crates/*/Cargo.toml, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-hx3c7qre@1
decision/no-otel-sdk-exporter · «otel» · in Cargo.toml, src/**/*.rs, crates/*/Cargo.toml, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-hx3c7qre@1
decision/redaction-not-disableable · «(?i)redact|no_redact|VIOLA_DEBUG|VIOLA_LOG|verbose» · in src/**/*.rs, crates/**/*.rs · place unread: it names no tracked file · fct-ccfaed7l@1,fct-xvmeqbmc@1
decision/redaction-not-disableable · «CONFIG_KEYS» · in src/obs.rs · first hit src/obs.rs:32 · fct-ccfaed7l@1,fct-xvmeqbmc@1
decision/no-traceparent-on-local-socket · «traceparent» · in src/**/*.rs, crates/**/*.rs · place unread: it names no tracked file · fct-pcotmuct@1
decision/no-traceparent-on-local-socket · «trace_context» · in src/**/*.rs, crates/**/*.rs · place unread: it names no tracked file · fct-pcotmuct@1
decision/assistant-message-never-logged · «last_assistant_message» · in src/run/wait.rs, src/cmd/last.rs, src/run/send/slot.rs · place unread: it names no tracked file · fct-xsyteni6@1,fct-mutbrgup@1
decision/assistant-message-never-logged · «obs_event! field list» · in crates/viola-channel/src/server.rs:365-376,473-485 · place unread: it names no tracked file · fct-xsyteni6@1,fct-mutbrgup@1
decision/dialog-text-never-logged · «obs_event!(INFO, ObsEvent::DialogAnswered» · in src/run/dialog.rs:458-467 · place unread: it names no tracked file · fct-t4iplhxy@1,fct-berta6ar@1
decision/statusline-command-confined · «statusline_command» · in src/ · first hit src/cmd/hook.rs:673 · fct-z24xvxbu@1,fct-tlje6rwd@1
decision/statusline-command-confined · «statusline_command» · in src/cmd/hook/statusline.rs · first hit src/cmd/hook/statusline.rs:130 · fct-z24xvxbu@1,fct-tlje6rwd@1
decision/statusline-command-confined · «obs_event!» · in src/cmd/hook/statusline.rs · first hit src/cmd/hook/statusline.rs:102 · fct-z24xvxbu@1,fct-tlje6rwd@1
decision/channel-lines-carry-no-bodies · «obs_event!(INFO, ObsEvent::ChannelRequest» · in crates/viola-channel/src/server.rs:365-376 · place unread: it names no tracked file · fct-ernmap45@1
decision/channel-lines-carry-no-bodies · «ObsEvent::ChannelResponse» · in crates/viola-channel/src/server.rs:473-485 · place unread: it names no tracked file · fct-ernmap45@1
decision/send-text-confined · «ObsEvent::SendRefused» · in src/run/send.rs:280-292 · place unread: it names no tracked file · fct-w5cnwd2z@1
decision/send-text-confined · «ObsEvent::SendRefused» · in src/cmd/send.rs:239-242 and crates/viola-mcp/src/tools.rs:308-311 · place unread: it names no tracked file · fct-w5cnwd2z@1
decision/send-text-confined · «ObsEvent::SendIssued» · in src/run/send.rs:193-203 · place unread: it names no tracked file · fct-w5cnwd2z@1
decision/single-file-sink-no-os-exporter · «opentelemetry|syslog|journald|eventlog|oslog|tracing-appender» · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-dao4nifd@2
decision/single-file-sink-no-os-exporter · «set_global_default» · in src/obs.rs:145 · place unread: it names no tracked file · fct-dao4nifd@2
decision/normal-mode-not-inverted · «opentelemetry|otlp|tonic|prost» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-x2cjy5mn@1
decision/metrics-derived-by-jq · «opentelemetry|prometheus» · in {src,crates,tests,scripts,schemas,.github,.config}/** · place unread: it names no tracked file · fct-i24bcxka@1
decision/metrics-derived-by-jq · «opentelemetry-otlp banned» · in deny.toml:32 · place unread: it names no tracked file · fct-i24bcxka@1
decision/no-human-only-dashboards · «grafana|kibana|datadog|prometheus» · in {src,crates,tests,scripts,schemas,.github,.config}/** · place unread: it names no tracked file · fct-dpql65fn@1
decision/no-proprietary-apm-sdk · «datadog|newrelic|new_relic|appdynamics|sentry» · in {src,crates,tests,scripts,schemas,.github,.config}/** and Cargo.toml · place unread: it names no tracked file · fct-7dfhzxv4@1
decision/no-proprietary-apm-sdk · «sentry banned» · in deny.toml:34 · place unread: it names no tracked file · fct-7dfhzxv4@1
decision/span-kinds-not-emitted-v1 · «otel.kind» · in src/**, crates/**, schemas/** · place unread: it names no tracked file · fct-7n4mjpbn@1
decision/span-kinds-not-emitted-v1 · «tracing-opentelemetry» · in every Cargo.toml · place unread: it names no tracked file · fct-7n4mjpbn@1
decision/span-kinds-not-emitted-v1 · «SpanKind» · in src/**, crates/** · place unread: it names no tracked file · fct-7n4mjpbn@1
decision/uninstrumented-components · «instrument» · in config.json handling is src/obs.rs read only · place unread: it names no tracked file · fct-vjo2uyhn@1
absence/web-or-graphical-frontend · «^(egui|eframe|iced|tauri|axum|actix|hyper|warp|rocket|leptos|yew|winit|wry|slint|gtk|druid)[a-z_-]*\s*=» · in every Cargo.toml · place unread: it names no tracked file · fct-qh2p5n6e@2
decision/no-assistant-message-in-logs · «last_assistant_message» · in obs_event! calls under src/** and crates/** · place unread: it names no tracked file · fct-q67chfay@2
decision/verify-lines-redacted · «obs_event!» · in src/cmd/verify.rs · first hit src/cmd/verify.rs:123 · fct-bharnesn@1
decision/verify-lines-redacted · «obs_event!» · in src/cmd/verify/typed/run.rs · first hit src/cmd/verify/typed/run.rs:89 · fct-bharnesn@1
absence/database · «rusqlite|sqlx|diesel|sled|redb|sqlite|postgres» · in Cargo.toml, crates/*/Cargo.toml, src/**, crates/*/src/** · place unread: it names no tracked file · fct-oqqvdowc@1
absence/database · «libsqlite3-sys» · in deny.toml · first hit deny.toml:30 · fct-oqqvdowc@1
decision/local-error-capture-only · «sentry|ureq|rustls|reporter» · in Cargo.toml and crates/*/Cargo.toml · place unread: it names no tracked file · fct-nwrbbhk2@1,fct-bx64zmwr@1
decision/local-error-capture-only · «sentry|ureq|rustls» · in src/**/*.rs and crates/*/src/**/*.rs · place unread: it names no tracked file · fct-nwrbbhk2@1,fct-bx64zmwr@1
decision/tracing-panic-rejected · «tracing-panic|tracing_panic» · in **/Cargo.toml and src/**/*.rs and crates/*/src/**/*.rs · place unread: it names no tracked file · fct-hbeo3r33@1
decision/human-panic-rejected · «human-panic|human_panic» · in **/Cargo.toml and src/**/*.rs and crates/*/src/**/*.rs · place unread: it names no tracked file · fct-hbeo3r33@1
decision/error-correlation-without-trace-ids · «trace_id|span_id|TraceId» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-kd3a4grv@1
decision/secrets-never-logged · «Cookie|cookie|signing» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-btgnwmtl@1
decision/endpoint-and-pinned-bin-never-logged · «obs_event!\([^;]*(endpoint|pinned_bin)» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-qk7hj63u@1
decision/endpoint-and-pinned-bin-never-logged · «pinned_bin» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-qk7hj63u@1
decision/veil-toggle-feature-banned · «toggle» · in Cargo.toml · first hit Cargo.toml:244 · fct-nfaxfmcg@1
absence/sentry-error-reporting · «sentry|Sentry|opentelemetry» · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-xpgyosmo@1
absence/sentry-error-reporting · «sentry» · in deny.toml · first hit deny.toml:34 · fct-xpgyosmo@1
decision/local-run-outputs-never-uploaded · «pre-push» · in .github/workflows/*.yml · first hit .github/workflows/ci.yml:121 · fct-onsbpcol@1
decision/no-allow-list-widening · «RUST_LOG|EnvFilter» · in src/** and crates/*/src/** · place unread: it names no tracked file · fct-llmksgvm@1
decision/no-abort-panic-strategy · «abort» · in Cargo.toml, .cargo/*, crates/*/Cargo.toml · place unread: it names no tracked file · fct-2rxslar4@1
decision/no-abort-panic-strategy · «abort» · in .github/workflows/*.yml · first hit .github/workflows/ci.yml:298 · fct-2rxslar4@1
decision/no-ci-attributes-in-log-lines · «deployment\.environment|ci\.run\.id|git\.commit\.sha» · in **/*.{rs,yml,sh,toml,json} · place unread: it names no tracked file · fct-z7z7o2va@1
decision/no-request-ratio-slo · «(?i)\bslo\b|error.ratio|request.ratio|burn.rate|error.budget» · in src/**, crates/*/src/**, scripts/**, .github/**, .config/** · place unread: it names no tracked file · fct-d4bn7qgd@1
absence/on-call-paging · «on-call|oncall|pagerduty|paging (case-insensitive)» · in **/*.{rs,toml,sh,yml,yaml} · place unread: it names no tracked file · fct-e2fzkaej@1
decision/no-otel-without-decisions-log · «opentelemetry|otlp» · in Cargo.toml and crates/*/Cargo.toml · place unread: it names no tracked file · fct-n47oemak@1
decision/no-otel-without-decisions-log · «opentelemetry|otlp|Meter» · in src/**/*.rs crates/**/*.rs · place unread: it names no tracked file · fct-n47oemak@1
decision/transcripts-not-a-contract · «transcript» · in src/**/*.rs crates/**/*.rs · place unread: it names no tracked file · fct-gr6xdpkj@1
absence/telemetry-exporter-stage · «opentelemetry|otlp|exporter» · in Cargo.toml and crates/*/Cargo.toml · place unread: it names no tracked file · fct-om4ml4hz@1
absence/telemetry-exporter-stage · «tracing-appender|opentelemetry» · in src/**/*.rs crates/**/*.rs · place unread: it names no tracked file · fct-om4ml4hz@1
decision/no-bare-instrument · «#\[(tracing::)?instrument\]» · in src/**/*.rs crates/**/*.rs · place unread: it names no tracked file · fct-3n7snwrw@1
decision/no-bare-instrument · «instrument\]» · in src/**/*.rs crates/**/*.rs · place unread: it names no tracked file · fct-3n7snwrw@1
decision/no-instrument-err-ret-on-sensitive · «instrument\((err|ret)|, ?(err|ret)\b» · in src/**/*.rs crates/**/*.rs · place unread: it names no tracked file · fct-odnuwe7o@1
decision/low-cardinality-span-names · «name = format|name = &|name = concat» · in src/**/*.rs crates/**/*.rs · place unread: it names no tracked file · fct-zgyunnhf@1
decision/no-in-process-metrics · «opentelemetry|metrics» · in Cargo.toml and crates/*/Cargo.toml · place unread: it names no tracked file · fct-npbjgxck@1
decision/no-in-process-metrics · «opentelemetry|metrics::|counter!|histogram!|gauge!|Meter» · in src/**/*.rs crates/**/*.rs · place unread: it names no tracked file · fct-npbjgxck@1
decision/no-telemetry-endpoint · «/metrics|axum|hyper|tiny_http|reqwest» · in src/**/*.rs crates/**/*.rs Cargo.toml · place unread: it names no tracked file · fct-khkx5tob@1
decision/no-stringified-corr · «\?corr|%corr|corr = [?%]» · in src/**, crates/**, scripts/**, tests/** · place unread: it names no tracked file · fct-dhc5cdwv@1
decision/no-env-log-level · «from_default_env» · in **/*.rs, **/Cargo.toml, **/*.sh · place unread: it names no tracked file · fct-mk3zdfg7@1
decision/no-env-log-level · «RUST_LOG» · in **/*.rs · first hit src/obs.rs:180 · fct-mk3zdfg7@1
decision/no-non-blocking-appender · «non_blocking» · in **/*.rs, **/Cargo.toml, **/*.sh · place unread: it names no tracked file · fct-v6m4e2mq@1
decision/no-log-bridge · «LogTracer» · in **/*.rs, **/Cargo.toml · place unread: it names no tracked file · fct-vdlxpzho@1
decision/reporter-opt-in-only · «sentry» · in **/*.toml and src/**/*.rs · place unread: it names no tracked file · fct-fb7hrxpx@1
decision/reporter-opt-in-only · «before_send» · in src/**/*.rs, crates/**/*.rs · place unread: it names no tracked file · fct-fb7hrxpx@1
decision/reporter-never-log-floor · «before_send» · in src/**/*.rs, crates/**/*.rs · place unread: it names no tracked file · fct-fb7hrxpx@1
decision/claude-env-and-cookie-never-logged · «CLAUDE» · in src/run/env.rs · first hit src/run/env.rs:60 · fct-vnsyc5om@1
decision/no-retry-once · «retries|retry» · in .config/ , .github/ , scripts/ · place unread: it names no tracked file · fct-pardphhn@1
decision/no-proprietary-apm-only · «datadog|newrelic|appdynamics|ddtrace» · in Cargo.toml, Cargo.lock, src/**, crates/**, scripts/**, .github/** · place unread: it names no tracked file · fct-df7l3zmj@1
absence/network-egress · «TcpStream|TcpListener|UdpSocket|reqwest|ureq» · in src/**/*.rs, crates/**/*.rs, Cargo.lock · place unread: it names no tracked file · fct-df7l3zmj@1
absence/network-egress · «opentelemetry-otlp|sentry (banned)» · in deny.toml:32,34 · place unread: it names no tracked file · fct-df7l3zmj@1
decision/no-dashboards-only-obs · «grafana|kibana|datadog» · in Cargo.toml, src/**, crates/**, scripts/**, .github/** · place unread: it names no tracked file · fct-h4xijqlt@1
decision/product-is-not-a-telemetry-collector · «otlp|OtlpReceiver|opentelemetry» · in src/**, crates/**/*.rs, Cargo.toml · place unread: it names no tracked file · fct-gd3tf5wl@1
decision/env-vars-not-config-channel · «RUST_BACKTRACE» · in src/**/*.rs, crates/**/*.rs · place unread: it names no tracked file · fct-c5mu6ru4@2
decision/env-vars-not-config-channel · «force_capture» · in src/**/*.rs, crates/**/*.rs · place unread: it names no tracked file · fct-c5mu6ru4@2
absence/interactive-terminal-interface · «dialoguer|ratatui|crossterm|rustyline|inquire|cursive|termion|pager» · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-6ujsabka@1
absence/deployment-environments · «deployment|environment =|staging» · in Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-uwlbnhdg@1
decision/no-otel-exporter · «opentelemetry» · in Cargo.toml, crates/*/Cargo.toml · place unread: it names no tracked file · fct-foy4up7k@1
decision/no-otel-exporter · «tracing-appender|sentry» · in Cargo.toml, crates/*/Cargo.toml · place unread: it names no tracked file · fct-foy4up7k@1
decision/json-to-file-only-mode · «agent-mode|agent_mode|log-format|LogFormat» · in src/**/*.rs, crates/**/*.rs · place unread: it names no tracked file · fct-he3oql74@1
decision/detail-codes-no-paths · «detail\s*=\s*[^"a-z_]*(path|display\(\)|pid)» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-h2wm2goy@2
decision/messaging-secrets-never-logged · «CLAUDE_CODE_MESSAGING_TOKEN|CLAUDE_CODE_MESSAGING_SOCKET» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-rkannnby@1,fct-ja2bi6wz@1
decision/redaction-cannot-be-disabled · «RUST_LOG|EnvFilter|VIOLA_LOG|VIOLA_DEBUG|redact» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-rkannnby@1
decision/latency-aggregation-outside-process · «metrics|prometheus|opentelemetry|percentile|p95» · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs · place unread: it names no tracked file · fct-dijwakce@1
decision/no-formal-slo · «\bSLO\b|availability» · in src/**/*.rs, crates/**/*.rs, tests/**/*.rs · place unread: it names no tracked file · fct-rjubzxp3@1
decision/no-log-rotation-v1 · «rotate|rotation|logrotate» · in src/**/*.rs, crates/**/*.rs · place unread: it names no tracked file · fct-lmrmnkyn@1
decision/no-telemetry-from-screen · «obs_event!» · in src/run/gate.rs · first hit src/run/gate.rs:78 · fct-jz5zhprx@1
decision/signing-credentials-never-logged · «signing|codesign|secrets\.» · in src/**/*.rs, crates/**/*.rs · place unread: it names no tracked file · fct-j3d6gqtu@1
decision/endpoint-paths-never-logged · «\bendpoint\s*=|pinned_bin\s*=» · in src/**/*.rs, crates/**/*.rs · place unread: it names no tracked file · fct-5jqyxpdr@1
decision/endpoint-paths-never-logged · «endpoint_kind» · in src/run/mod.rs:37 · place unread: it names no tracked file · fct-5jqyxpdr@1
decision/fixtures-not-a-telemetry-source · «fixtures» · in src/**/*.rs obs_event! fields · place unread: it names no tracked file · fct-rp7rj7wv@1
decision/mutants-out-never-uploaded · «upload-artifact» · in .github · first hit .github/workflows/ci.yml:80 · fct-ydc7ueix@3
decision/link-names-not-in-process-log · «obs_event!\([^;]{0,400}(driver|driven)» · in src/**/*.rs crates/*/src/**/*.rs · place unread: it names no tracked file · fct-lohdixpq@2,fct-lnbq3lnm@1
decision/link-names-not-in-process-log · «driver|driven» · in src/obs.rs crates/viola-core/src/obs.rs schemas/diag-line.v1.json · place unread: it names no tracked file · fct-lohdixpq@2,fct-lnbq3lnm@1
decision/no-text-in-telemetry · «obs_event!\([^;]{0,500}(text\b|\btext:|last_assistant_message)» · in src/**/*.rs crates/*/src/**/*.rs · place unread: it names no tracked file · fct-xfymn57j@1,fct-gghzntp4@2
decision/no-text-in-telemetry · «fields\([^)]*\btext\b» · in src/**/*.rs crates/*/src/**/*.rs · place unread: it names no tracked file · fct-xfymn57j@1,fct-gghzntp4@2
decision/no-last-assistant-message · «last_assistant_message» · in src/obs.rs src/obs/ crates/viola-core/src/obs* schemas/diag-line.v1.json · place unread: it names no tracked file · fct-hluu43pb@2
decision/no-last-assistant-message · «obs_event!\([^;]{0,500}last_assistant_message» · in src/**/*.rs crates/*/src/**/*.rs · place unread: it names no tracked file · fct-hluu43pb@2
decision/no-dialog-content-in-logs · «^\s+(input|question|answer|text|plan|tool_input|response|keys|bytes) = » · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-ffumo3bh@1
decision/no-dialog-content-in-logs · «obs_event!` field lists of the dialog events» · in src/run/dialog.rs:312-318, 386-393 and src/cmd/hook.rs:365-407 · place unread: it names no tracked file · fct-ffumo3bh@1
decision/no-typed-keys-in-telemetry · «fields(» · in src/**/*.rs · first hit src/bin/viola-fake-agent.rs:177 · fct-yj7tr26f@1
decision/no-typed-keys-in-telemetry · «keys|typed` as a span or obs_event field» · in src/run/wheel.rs and src/human/*.rs · place unread: it names no tracked file · fct-yj7tr26f@1
decision/implicit-link-untraced · «ObsEvent::Link» · in src/**/*.rs and crates/viola-core/src/obs.rs · place unread: it names no tracked file · fct-46kjozmt@1
decision/link-methods-untraced · «params logged on channel lines» · in crates/viola-channel/src/server.rs:365-376 · place unread: it names no tracked file · fct-lnbq3lnm@1
decision/no-raw-snapshot-reads · «read_snapshot\(» · in src/**/*.rs outside test modules · place unread: it names no tracked file · fct-wev2ceaa@1
decision/no-raw-snapshot-reads · «read_snapshot_or_replay» · in src · first hit src/cmd/hook.rs:30 · fct-wev2ceaa@1
decision/verify-lines-carry-no-content · «obs_event!|ObsEvent::» · in src/cmd/verify.rs and src/cmd/verify/ · place unread: it names no tracked file · fct-f3gi2vhc@2
decision/verify-lines-carry-no-content · «session_id|prompt|cwd|path.display» · in src/cmd/verify.rs and src/cmd/verify/ obs_event! calls · place unread: it names no tracked file · fct-f3gi2vhc@2
decision/no-obs-gate-on-verify-lines · «verify» · in scripts · first hit scripts/agent-run.sh:11 · fct-f3gi2vhc@2
decision/events-stream-never-rotated · «logroller|tracing-appender|rotat» · in Cargo.toml, Cargo.lock, src/**/*.rs, crates/*/src/**/*.rs, crates/*/Cargo.toml · place unread: it names no tracked file · fct-mtgjpoqf@1,fct-kytmq3ju@1
decision/no-log-rotation-in-v1 · «logroller» · in Cargo.toml, Cargo.lock, src/**/*.rs, crates/*/src/**/*.rs, crates/*/Cargo.toml · place unread: it names no tracked file · fct-kytmq3ju@1
decision/no-log-rotation-in-v1 · «rotat (case-insensitive)» · in Cargo.toml, Cargo.lock, src/**/*.rs, crates/*/src/**/*.rs, crates/*/Cargo.toml · place unread: it names no tracked file · fct-kytmq3ju@1
decision/tracing-appender-rejected · «tracing-appender» · in Cargo.toml, Cargo.lock, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-kytmq3ju@1
decision/tracing-appender-rejected · «tracing_appender» · in src/**/*.rs, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-kytmq3ju@1
decision/no-agent-mode-flag · «agent-mode|agent_mode|--agent» · in src/**/*.rs, Cargo.toml, deny.toml · place unread: it names no tracked file · fct-brueynpb@3
decision/no-otel-sdk-v1 · «opentelemetry» · in **/*.rs and **/Cargo.toml · place unread: it names no tracked file · fct-56r4goln@1,fct-rrjwwqcp@2
decision/no-otel-sdk-v1 · «Resource::builder» · in **/*.rs and **/Cargo.toml · place unread: it names no tracked file · fct-56r4goln@1,fct-rrjwwqcp@2
absence/hosted-deployment · «deployment» · in src/**/*.rs, crates/**/*.rs, tests/**/*.rs · place unread: it names no tracked file · fct-ah3dlz7g@1
absence/pulse-class-receiver · «pulse» · in src/**/*.rs, crates/**/*.rs, tests/**/*.rs · place unread: it names no tracked file · fct-ehmrhyqq@1
decision/claude-internals-not-a-contract · «transcript» · in src/**/*.rs · first hit src/bin/viola-fake-agent.rs:1573 · fct-avrouaxx@1,fct-ngecoymf@1,fct-t3jwc6wb@1
decision/claude-internals-not-a-contract · «transcript» · in crates/viola-agent-claude/src/**/*.rs · first hit crates/viola-agent-claude/src/hook.rs:418 · fct-avrouaxx@1,fct-ngecoymf@1,fct-t3jwc6wb@1
absence/otlp-receiver · «opentelemetry|otlp|tonic|prost» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-q2zh72zo@2,fct-x2cjy5mn@1
decision/no-w3c-traceparent · «traceparent» · in {src,crates,tests,scripts,schemas,.github,.config}/** and *.rs, *.toml · place unread: it names no tracked file · fct-pcsvls55@1,fct-dhcjb4zv@1
absence/in-process-metrics-pipeline · «opentelemetry|metrics|prometheus|statsd» · in Cargo.toml, **/Cargo.toml, Cargo.lock, src/**/*.rs, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-kvmjumn6@1,fct-dijwakce@1
absence/in-process-metrics-pipeline · «metrics-exporter|metrics::» · in Cargo.toml, crates/**/Cargo.toml, **/*.rs · place unread: it names no tracked file · fct-kvmjumn6@1,fct-dijwakce@1
absence/grpc · «tonic|prost|grpcio|grpc» · in Cargo.toml, crates/*/Cargo.toml, src/**, crates/*/src/** · place unread: it names no tracked file · fct-oqqvdowc@1,fct-nvdezwhb@1
decision/no-sentry-or-otel-bridge · «sentry|opentelemetry» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-kd3a4grv@1
decision/no-sentry-or-otel-bridge · «trace_id|span_id» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-kd3a4grv@1
absence/otel-span-processing · «SpanProcessor» · in src/**, crates/*/src/**, Cargo.toml, crates/*/Cargo.toml, Cargo.lock, deny.toml · place unread: it names no tracked file · fct-xpgyosmo@1
absence/otel-span-processing · «opentelemetry» · in the same paths · place unread: it names no tracked file · fct-xpgyosmo@1
absence/otel-trace-export · «opentelemetry» · in src/**/*.rs, crates/**/*.rs, Cargo.toml, crates/**/Cargo.toml · place unread: it names no tracked file · fct-dajf6x5e@1
absence/otel-trace-export · «otel» · in src/**/*.rs, crates/**/*.rs, Cargo.toml, crates/**/Cargo.toml · place unread: it names no tracked file · fct-dajf6x5e@1
decision/no-secrets-in-log-lines · «Cookie» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-ogkwmfvl@1,fct-evvm5jqh@1
