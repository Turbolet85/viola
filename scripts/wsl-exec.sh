#!/usr/bin/env bash
# Runs one command inside WSL2 `Ubuntu` with the environment `pre-push` gives its distro calls
# (crates/viola-e2e/src/harness/pre_push/linux.rs, `cmd_env`): `--exec` so argv is passed verbatim
# (the `--` form re-parses it through the distro shell), `env -i` with only HOME and a Linux PATH so
# nothing of this host's environment crosses (no CLAUDE* value, no Windows PATH), and
# MSYS2_ARG_CONV_EXCL so Git Bash never rewrites a leading-slash argument into a Windows path.
# An operator aid for fix loops; `pre-push` keeps its own commands.
#   wsl-exec.sh [--cd DIR] CMD [ARG...]   run CMD in the distro (DIR is a distro path)
#   wsl-exec.sh --probe                   prove exactly HOME and PATH cross, no CLAUDE* name does,
#                                         and a leading-slash argument arrives unconverted
set -euo pipefail

distro=Ubuntu

if ! command -v wsl.exe >/dev/null 2>&1; then
  echo "tool-missing: wsl.exe"
  exit 1
fi

distro_home() {
  local home
  home=$(MSYS2_ARG_CONV_EXCL='*' wsl.exe -d "$distro" --exec /usr/bin/printenv HOME | tr -d '\r') || {
    echo "wsl-exec: FAILED — distro $distro did not answer"
    exit 1
  }
  case "$home" in
    /*) printf '%s' "$home" ;;
    *)
      echo "wsl-exec: FAILED — distro $distro gave no absolute HOME"
      exit 1
      ;;
  esac
}

run() {
  local home cd=()
  if [ "${1:-}" = "--cd" ]; then
    if [ $# -lt 2 ]; then
      echo "usage: wsl-exec.sh [--cd DIR] CMD [ARG...] | --probe"
      exit 2
    fi
    cd=(--cd "$2")
    shift 2
  fi
  if [ $# -eq 0 ]; then
    echo "usage: wsl-exec.sh [--cd DIR] CMD [ARG...] | --probe"
    exit 2
  fi
  home=$(distro_home)
  MSYS2_ARG_CONV_EXCL='*' wsl.exe -d "$distro" "${cd[@]}" --exec /usr/bin/env -i \
    "HOME=$home" "PATH=$home/.cargo/bin:/usr/local/bin:/usr/bin:/bin" "$@"
}

probe() {
  local env names echoed failing=()
  env=$("$0" /usr/bin/printenv | tr -d '\r') || failing+=("printenv did not run")
  names=$(printf '%s\n' "$env" | sed -n 's/=.*//p' | sort | tr '\n' ' ')
  if [ "$names" != "HOME PATH " ]; then
    failing+=("names crossed: ${names:-none}")
  fi
  if printf '%s\n' "$env" | grep -q '^CLAUDE'; then
    failing+=("a CLAUDE* name crossed")
  fi
  echoed=$("$0" /bin/echo /usr/bin/printenv | tr -d '\r') || failing+=("echo did not run")
  if [ "$echoed" != "/usr/bin/printenv" ]; then
    failing+=("argv converted: $echoed")
  fi
  if [ "${#failing[@]}" -eq 0 ]; then
    echo "wsl-exec probe: HOME and PATH only, no CLAUDE* name, argv unconverted"
    return 0
  fi
  printf 'wsl-exec probe: FAILED — %s\n' "${failing[@]}"
  return 1
}

case "${1:-}" in
  --probe) probe ;;
  *) run "$@" ;;
esac
