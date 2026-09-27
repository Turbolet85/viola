# WSL Chromium system libraries (plan step 11; overseer review direction 2026-09-27)

## How they were installed
- The user provision ran first (gate entry 41). The distro user ran `scripts/wsl-provision.sh` under `env -i` with a distro-derived
  HOME. It installed Node v24.21.0 into `~/.local/viola-node/` through `install-node.sh linux-x64` and ran `npm ci` in
  `~/.cache/viola-provision/e2e-web/`. It then ran `npx --no playwright install chromium` there, and printed
  `wsl-provision: pins ok … node v24.21.0`.
- Then the root install (gate entry 42) ran as
  `wsl.exe -d Ubuntu -u root … env -i HOME=/root PATH=… bash scripts/wsl-provision.sh --install-deps <distro user home>`. That is uid
  0, with no password and no sudo. It ran the user's locked Playwright:
  `<home>/.local/viola-node/bin/node <home>/.cache/viola-provision/e2e-web/node_modules/@playwright/test/cli.js install-deps`.
  - It ran `--dry-run chromium` first, then `chromium`.
  - Its last line was `wsl-provision: install-deps ok` (gate entry 42 green, the re-run).
- **Measured on the first run:** Playwright 1.63.0's `install-deps --dry-run` prints `Missing system dependencies (28):` and exits
  1 while any package is missing. So `set -e` stopped the script before the install. The dry run now carries `|| true` (with that
  reason in a comment), and the install's own exit is the verdict.

## The printed package list (`Missing system dependencies (28):`, Ubuntu 26.04 "resolute")
fonts-freefont-ttf · fonts-ipafont-gothic · fonts-liberation · fonts-noto-color-emoji · fonts-tlwg-loma-otf · fonts-unifont ·
fonts-wqy-zenhei · libasound2-data · libasound2t64 · libfontenc1 · libice6 · libnspr4 · libnss3 · libsm6 · libunwind8 · libxaw7 ·
libxfont2 · libxkbfile1 · libxmu6 · libxpm4 · libxt6t64 · x11-xkb-utils · xfonts-cyrillic · xfonts-encodings · xfonts-scalable ·
xfonts-utils · xserver-common · xvfb

## `dpkg-query -W` after the install (2026-09-27T19:20:02Z, the distro user under `env -i`; exit 0)

| package | version | status |
|---|---|---|
| fonts-freefont-ttf | 20211204+svn4273-4build1 | installed |
| fonts-ipafont-gothic | 00303-23ubuntu1 | installed |
| fonts-liberation | 1:2.1.5-3build1 | installed |
| fonts-noto-color-emoji | 2.051-1build1 | installed |
| fonts-tlwg-loma-otf | 1:0.7.3-1build1 | installed |
| fonts-unifont | 1:16.0.04-1build1 | installed |
| fonts-wqy-zenhei | 0.9.45-8build1 | installed |
| libasound2-data | 1.2.15.3-1ubuntu1.1 | installed |
| libasound2t64 | 1.2.15.3-1ubuntu1.1 | installed |
| libfontenc1 | 1:1.1.8-1build2 | installed |
| libice6 | 2:1.1.1-1build1 | installed |
| libnspr4 | 2:4.38.2-1ubuntu1 | installed |
| libnss3 | 2:3.120-1ubuntu2.1 | installed |
| libsm6 | 2:1.2.6-1build1 | installed |
| libunwind8 | 1.8.3-0ubuntu1 | installed |
| libxaw7 | 2:1.0.16-1build1 | installed |
| libxfont2 | 1:2.0.6-2ubuntu0.2 | installed |
| libxkbfile1 | 1:1.1.0-1build5 | installed |
| libxmu6 | 2:1.1.3-4 | installed |
| libxpm4 | 1:3.5.17-1ubuntu0.26.04.1 | installed |
| libxt6t64 | 1:1.2.1-1.3build1 | installed |
| x11-xkb-utils | 7.7+9build1 | installed |
| xfonts-cyrillic | 1:1.0.5+nmu1build1 | installed |
| xfonts-encodings | 1:1.0.5-0ubuntu3 | installed |
| xfonts-scalable | 1:1.0.3-1.3build1 | installed |
| xfonts-utils | 1:7.7+7build1 | installed |
| xserver-common | 2:21.1.22-1ubuntu1.2 | installed |
| xvfb | 2:21.1.22-1ubuntu1.2 | installed |

28 of 28 are installed. The wrap judges whether a root install by a plan entry is a boundary widening that needs the founder's live
word (plan, Expected amendments → security-plan Pinning (WSL2)).
