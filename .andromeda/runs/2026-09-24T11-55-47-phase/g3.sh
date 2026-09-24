export PATH="$PWD/target/tools/ripgrep/bin:$PATH"; set +e; rg -n --hidden -g 'Cargo.toml' -g 'config.toml' -g '*.yml' -g '*.yaml' "(panic|_PANIC)\s*[:=]\s*[\"']?abort" .; test $? -eq 1
