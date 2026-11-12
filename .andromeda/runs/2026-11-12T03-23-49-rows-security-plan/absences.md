decision/cwd-only-in-snapshot · «cwd =» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-l2iemwjy@1
decision/cwd-only-in-snapshot · «(println|eprintln|obs_event).*cwd» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-l2iemwjy@1
absence/release-and-deploy · «publish = false» · in Cargo.toml · first hit Cargo.toml:190 · fct-tm2e2x5g@1,fct-ju2d5wlo@2
absence/public-network-listener · «TcpListener|TcpStream|SocketAddr|0.0.0.0» · in src/**/*.rs, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-k62evpgk@1,fct-odkkhtym@1
absence/public-network-listener · «axum|hyper|reqwest|tiny_http» · in Cargo.toml, crates/*/Cargo.toml · place unread: it names no tracked file · fct-k62evpgk@1,fct-odkkhtym@1
absence/websocket · «WebSocket|tungstenite» · in src/**/*.rs, crates/*/src/**/*.rs, Cargo.toml, crates/*/Cargo.toml · place unread: it names no tracked file · fct-k62evpgk@1
absence/hosting · «Dockerfile|docker» · in src/**/*.rs, crates/*/src/**/*.rs, scripts/*, .github/** · place unread: it names no tracked file · fct-k62evpgk@1,fct-nj4c3hrg@1
absence/hosting · «Dockerfile*» · in repository root glob · place unread: it names no tracked file · fct-k62evpgk@1,fct-nj4c3hrg@1
absence/remote-access · «TcpListener|0.0.0.0|tungstenite|axum|hyper» · in src/**/*.rs, crates/*/src/**/*.rs, Cargo.toml, crates/*/Cargo.toml · place unread: it names no tracked file · fct-c4azjngh@1
decision/plaintext-state · «encrypt|cipher|aes|chacha|keyring|crypto (case-insensitive)» · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-y7373awj@1
absence/third-party-personal-data · «TcpListener|TcpStream|reqwest|ureq|api.anthropic|UdpSocket» · in Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs, crates/*/Cargo.toml · place unread: it names no tracked file · fct-wmp6i3hm@1
absence/user-accounts-passwords · «password|passwd|login|user_account|bcrypt|argon2 (case-insensitive)» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-uy3rjgvj@1
decision/no-statusline-command-in-logs · «command =» · in src/cmd/hook/statusline.rs, src/cmd/run.rs, src/obs.rs, src/cmd/hook.rs · place unread: it names no tracked file · fct-2fgozo5r@2
decision/no-statusline-command-in-logs · «statusline_command» · in src/** · first hit src/cmd/hook.rs:673 · fct-2fgozo5r@2
decision/no-tls-local-transport · «Tls|TLS|rustls» · in src/**/*.rs and crates/**/src · place unread: it names no tracked file · fct-6ikkebgm@2
decision/no-keys-at-rest · «Sha256» · in crates/viola-state/src · first hit crates/viola-state/src/pin.rs:11 · fct-7roiu3gz@1
decision/viola-writes-nothing-in-claude-dir · «\.claude\.json» · in src/**/*.rs and crates/**/src · place unread: it names no tracked file · fct-j3zmg7hp@1,fct-vaineopj@1
decision/verify-never-answers-import · «Yes, allow external imports» · in src/cmd/verify · first hit src/cmd/verify/typed/turns.rs:251 · fct-tjw7g733@1
decision/verify-never-answers-import · «external import» · in src/cmd/verify/**, -i · place unread: it names no tracked file · fct-tjw7g733@1
decision/cargo-audit-not-gated · «cargo-audit|cargo audit» · in .github/**, scripts/**, Cargo.toml, crates/*/Cargo.toml, deny*.toml · place unread: it names no tracked file · fct-ioqfwukn@1
decision/nothing-runs-as-root · «sudo|doas|geteuid|uid 0» · in **/*.{rs,sh,py,yml} · place unread: it names no tracked file · fct-2c53xvmz@1
decision/manual-dependency-updates · «dependabot.yml|renovate.json|.renovaterc» · in repository root and .github/ · place unread: it names no tracked file · fct-27vpn33i@1
absence/stored-secrets · «keyring|credential|password» · in crates/**/*.rs, src/**/*.rs, Cargo.toml, crates/**/Cargo.toml · place unread: it names no tracked file · fct-gh3kajmo@1
decision/env-not-config-channel · «#[arg(env» · in src/** and crates/*/src/** · place unread: it names no tracked file · fct-ugohyrrh@2
decision/env-not-config-channel · «env = "» · in src/** and crates/*/src/** · place unread: it names no tracked file · fct-ugohyrrh@2
decision/env-not-config-channel · «RUST_LOG» · in src/** and crates/*/src/** · place unread: it names no tracked file · fct-ugohyrrh@2
decision/no-uid-0-launch · «.uid(» · in crates/viola-e2e/src/harness/pre_push.rs and pre_push/ · place unread: it names no tracked file · fct-z4tolcdo@1
decision/no-credential-literals · «sk-ant» · in src/**, crates/*/src/**, plugin/**, .github/** · place unread: it names no tracked file · fct-kmhnhsns@1
decision/no-credential-literals · «ghp_» · in src/**, crates/*/src/**, plugin/**, .github/** · place unread: it names no tracked file · fct-kmhnhsns@1
decision/no-credential-literals · «BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY» · in src/**, crates/*/src/**, plugin/**, .github/** · place unread: it names no tracked file · fct-kmhnhsns@1
decision/no-credential-literals · «Bearer [A-Za-z0-9]{10}» · in src/**, crates/*/src/**, plugin/**, .github/** · place unread: it names no tracked file · fct-kmhnhsns@1
absence/secret-manager · «keyring» · in Cargo.toml, crates/*/Cargo.toml, src/**, crates/**/src/** · place unread: it names no tracked file · fct-rijajpwh@1
absence/secret-manager · «secret.?manager» · in Cargo.toml, crates/*/Cargo.toml, src/**, crates/**/src/** · place unread: it names no tracked file · fct-rijajpwh@1
absence/secret-manager · «vault» · in Cargo.toml, crates/*/Cargo.toml, src/**, crates/**/src/** · place unread: it names no tracked file · fct-rijajpwh@1
absence/external-error-reporting · «sentry» · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-6tg2dujf@1
absence/external-error-reporting · «reqwest|ureq|isahc|TcpStream» · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-6tg2dujf@1
decision/no-abstract-namespace-sockets · «GenericNamespaced» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-2g5b5nut@1
decision/no-abstract-namespace-sockets · «to_ns_name» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-2g5b5nut@1
decision/no-abstract-namespace-sockets · «abstract» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-2g5b5nut@1
absence/user-accounts · «login|password|oauth» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-32h3f5um@1
absence/user-accounts · «account» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-32h3f5um@1
decision/non-inheritable-channel-handles · «set_inheritable|inheritable|HANDLE_FLAG_INHERIT» · in **/*.rs, **/*.toml · place unread: it names no tracked file · fct-iftwwapx@2
decision/non-inheritable-channel-handles · «CLOEXEC» · in **/*.rs, **/*.toml · place unread: it names no tracked file · fct-iftwwapx@2
decision/no-messaging-secrets-in-outputs · «CLAUDE_CODE_MESSAGING» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-vbcb52sx@1
decision/no-secrets-in-git · «.env» · in .gitignore · first hit .gitignore:30 · fct-jo3zjurs@1
decision/no-secrets-in-git · «**/.env, **/.env.*, **/*.pem, **/*.p12, **/*.pfx, **/*.keystore» · in the repository · place unread: it names no tracked file · fct-jo3zjurs@1
decision/no-hardcoded-credentials · «(password|api_key|secret) = "..." assignment» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-rnwhyrn3@1
decision/no-hardcoded-credentials · «include_str!» · in src · first hit src/obs.rs:354 · fct-rnwhyrn3@1
decision/no-parser-error-chains-outward · «serde_path_to_error» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-ilzyulw7@1
decision/hook-silent-and-exit-zero · «eprint|stderr» · in src/cmd/hook.rs and src/cmd/hook/ · place unread: it names no tracked file · fct-tnu5bfz7@1
decision/hook-silent-and-exit-zero · «(Role::Hook, _) => ExitCode::SUCCESS» · in src/main.rs:133 · place unread: it names no tracked file · fct-tnu5bfz7@1
decision/no-interprocess-try-overwrite · «try_overwrite(true» · in src/**/*.rs, crates/**/*.rs · place unread: it names no tracked file · fct-3nboftz7@1,fct-2pxk6g7w@2
decision/no-interprocess-try-overwrite · «try_overwrite» · in crates/viola-channel/src/server.rs · first hit crates/viola-channel/src/server.rs:98 · fct-3nboftz7@1,fct-2pxk6g7w@2
decision/viola-ignores-e2e-home-backing · «e2e-home» · in src/**/*.rs · first hit src/cmd/run.rs:1612 · fct-4fkoyw3x@1
decision/viola-ignores-e2e-home-backing · «e2e-home» · in crates/viola-*/src · place unread: it names no tracked file · fct-4fkoyw3x@1
decision/controls-not-switchable · «std::env::var|env::var_os|env::var\(» · in src/** and crates/*/src/** · place unread: it names no tracked file · fct-7ackeyph@2
decision/controls-not-switchable · «VIOLA_» · in src/cmd/hook.rs, src/cmd/mod.rs, src/cmd/client.rs · place unread: it names no tracked file · fct-7ackeyph@2
absence/accounts-and-passwords · «password|passwd|argon|bcrypt|login (case-insensitive)» · in src/**/*.rs, crates/*/src/**/*.rs, Cargo.toml, crates/*/Cargo.toml · place unread: it names no tracked file · fct-riad4wv3@1
decision/no-writes-under-claude-home · «claude\.json» · in src/**/*.rs, crates/**/*.rs · place unread: it names no tracked file · fct-zvqsbdzq@1,fct-uz6mlfv5@2
decision/no-writes-under-claude-home · «Command::new\(.claude» · in src/**/*.rs, crates/**/*.rs · place unread: it names no tracked file · fct-zvqsbdzq@1,fct-uz6mlfv5@2
decision/no-writes-under-claude-home · «\.join\("\.claude"\)» · in src/**/*.rs, crates/**/*.rs · place unread: it names no tracked file · fct-zvqsbdzq@1,fct-uz6mlfv5@2
absence/key-management · «keyring|encrypt|chacha|ed25519|hmac|secret_key|private_key|api_key|rustls» · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-uz6mlfv5@2
decision/no-stale-license-exceptions · «name = "doctest-file"» · in Cargo.lock · first hit Cargo.lock:250 · fct-qfxjrvyr@1
decision/no-stale-license-exceptions · «[[licenses.exceptions]]» · in deny.toml, deny-sync.toml · place unread: it names no tracked file · fct-qfxjrvyr@1
decision/internal-error-held-at-unit-layer · «failpoint» · in tests/**/*.rs and crates/viola-e2e/** · place unread: it names no tracked file · fct-iwcvxbkd@1
decision/internal-error-held-at-unit-layer · «-32603» · in tests/**/*.rs · first hit tests/channel_error_bodies.rs:5 · fct-iwcvxbkd@1
decision/internal-error-held-at-unit-layer · «counted panic» · in crates/viola-e2e/** and tests/channel_error_bodies.rs · place unread: it names no tracked file · fct-iwcvxbkd@1
decision/no-width-or-colour-override-env · «"COLUMNS"» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-lj2i7oks@2
decision/no-width-or-colour-override-env · «"CLICOLOR"» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-lj2i7oks@2
decision/no-width-or-colour-override-env · «"CLICOLOR_FORCE"» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-lj2i7oks@2
decision/no-width-or-colour-override-env · «viola_pty::host_size» · in src/cmd/list.rs · first hit src/cmd/list.rs:63 · fct-lj2i7oks@2
decision/classified-snapshot-read-only · «read_snapshot(» · in src/**/*.rs outside test modules · place unread: it names no tracked file · fct-wkbf5hxn@1
decision/classified-snapshot-read-only · «read_snapshot(» · in crates/**/*.rs outside test modules · place unread: it names no tracked file · fct-wkbf5hxn@1
decision/refusals-hold-no-paths-pids-ids · «Refusal::pair texts (cwd, pid, id interpolation)» · in src/cmd/revive.rs:79-102 · place unread: it names no tracked file · fct-22lqecbw@1
decision/refusals-hold-no-paths-pids-ids · «is_session_id» · in src/cmd/revive.rs:321 · place unread: it names no tracked file · fct-22lqecbw@1
decision/no-control-switch-off · «FAKE_AGENT_» · in src/**/*.rs · first hit src/cmd/hook/seam.rs:2 · fct-bg5w767t@2
decision/no-control-switch-off · «env::var» · in src/**/*.rs and crates/*/src/**/*.rs · place unread: it names no tracked file · fct-bg5w767t@2
decision/no-control-switch-off · «peer_admitted» · in src/**/*.rs and crates/*/src/**/*.rs · place unread: it names no tracked file · fct-bg5w767t@2
decision/no-seam-forcing-peer-refusal · «peer_admitted» · in src/**/*.rs and crates/*/src/**/*.rs · place unread: it names no tracked file · fct-lvhmwli7@2
decision/no-seam-forcing-peer-refusal · «serve_admitting» · in crates/viola-channel/src/server.rs · first hit crates/viola-channel/src/server.rs:69 · fct-lvhmwli7@2
decision/no-seam-forcing-peer-refusal · «VIOLA_.*EUID|FORCE_REFUS|PEER_EUID» · in src/**/*.rs and crates/*/src/**/*.rs · place unread: it names no tracked file · fct-lvhmwli7@2
absence/model-api-calls · «reqwest|ureq|TcpStream|api.anthropic» · in Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-jy2pv6x7@1
absence/secrets · «encrypt|keyring|crypto (case-insensitive)» · in Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-jy2pv6x7@1,fct-v2lh3e2i@3
decision/nothing-written-under-claude-home · «USER_SETTINGS» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-uemrwfo5@2
decision/nothing-written-under-claude-home · «".claude"» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-uemrwfo5@2
absence/release-stage · «dist-workspace.toml» · in repository root · place unread: it names no tracked file · fct-dvr22577@1,fct-v2lh3e2i@3
decision/mutation-testing-at-audit · «mutants» · in .github/workflows · first hit .github/workflows/ci.yml:32 · fct-v2lh3e2i@3
decision/pinned-artefacts-never-deleted · «remove_dir_all|remove_file|remove_dir» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-2i3lkhzs@2,fct-feql22h5@2
absence/outbound-network-calls · «TcpStream|TcpListener|UdpSocket|reqwest|hyper|ureq|isahc» · in src/**/*.rs and crates/**/*.rs · place unread: it names no tracked file · fct-2jp5ovui@2,fct-6tg2dujf@1
absence/file-upload · «upload|multipart» · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-h4t4qrfe@1,fct-5v55psto@1
absence/webhook · «webhook|axum|hyper|warp» · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-h4t4qrfe@1,fct-5v55psto@1
absence/database · «rusqlite|sqlx|diesel|sqlite|postgres» · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs · place unread: it names no tracked file · fct-h4t4qrfe@1,fct-5v55psto@1,fct-cy2wvz77@1,fct-c3b3fd57@1
decision/signed-self-update · «self_update» · in the tracked tree outside .claude/ .andromeda/ viola-0.1.0/ viola-0.2.0/ · place unread: it names no tracked file · fct-w6kcw2wp@1,fct-qhcowchx@1,fct-heudg7au@1
decision/no-repo-secret-scanner · «gitleaks|trufflehog|detect-secrets|secretlint|git-secrets» · in .github/**, scripts/**, Cargo.toml, deny.toml, .pre-commit-config.yaml, crates/**/*.rs, src/**/*.rs · place unread: it names no tracked file · fct-rwpm3oj4@1,fct-ypiftj4r@1,fct-f326yvjk@1
