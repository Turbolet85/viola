# WSL Ubuntu provisioning (implement, 2026-09-26)

- Distro: `Ubuntu` 26.04.1 LTS, kernel 6.6.87.2-microsoft-standard-WSL2 (research M1). Default distro `docker-desktop`.
- **build-essential** — plan entry 6, `wsl.exe -d Ubuntu -u root --exec /usr/bin/apt-get install -y build-essential`.
  - First run RED, exit 100: three security-pocket packages 404 at `security.ubuntu.com`
    (`libc-dev-bin_2.43-2ubuntu2.3`, `linux-libc-dev_7.0.0-30.30`, `libc6-dev_2.43-2ubuntu2.3`), apt: "Unable to fetch
    some archives, maybe run apt update". The P5 hypothesis (a stale distro needs `apt-get update` first) is now MEASURED.
  - Remedy, run once by hand as root (`.andromeda/runs/2026-09-26T21-57-01-implement/apt-update.log`, exit 0):
    `wsl.exe -d Ubuntu -u root --exec /usr/bin/apt-get update`. Entry 6 then GREEN (17.45 s).
  - For the wrap: the entry should carry the update, e.g. a preceding `-u root --exec /usr/bin/apt-get update` entry.
  - `cc --version` (entry 7): `cc (Ubuntu 15.2.0-16ubuntu1) 15.2.0`; build-essential `12.12ubuntu2.26.04.2` (the
    candidate read at P5).
- **rustup-init** 1.29.1, `x86_64-unknown-linux-gnu`, sha256
  `dda7234360b7f578ca8b0ddcb80145646fa61a67c1720a5abc7051b35c9fcb71`, read from the published
  `https://static.rust-lang.org/rustup/archive/1.29.1/x86_64-unknown-linux-gnu/rustup-init.sha256` and embedded as a
  literal in `scripts/wsl-provision.sh`.
- **Entry 8** (`wsl-provision.sh`, 147.0 s): `cargo install --locked` of `cargo-nextest v0.9.146`, `cargo-mutants
  v27.1.0`, `cargo-llvm-cov v0.9.1` (the ci.yml `test`-job line); the check line
  `wsl-provision: pins ok rustc 1.98.1 cargo-nextest 0.9.146 cargo-mutants 27.1.0 cargo-llvm-cov 0.9.1`.
- **Entry 9** (`--probe`): `wsl-provision --probe: 2/2 refused, control clean`.
- The live `pre-push` tools stage read the same five versions from the tools themselves (`rustc 1.98.1 (48a229cea
  2026-09-01)`, `cargo-nextest 0.9.146 (8af696ddc 2026-09-21)`, `cargo-mutants 27.1.0`, `cargo-llvm-cov 0.9.1`) and,
  before provisioning, refused with `{"reason":"tool-missing","detail":"cc","stage":"tools"}`.
- Sizes after the runs: `~/.rustup` 800 MB, `~/.cargo` 612 MB, the clone's `target/` 6.6 GB (under the 40 GiB cap).
