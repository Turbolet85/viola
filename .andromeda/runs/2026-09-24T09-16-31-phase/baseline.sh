cd /d/dev/projects/viola
B=.andromeda/runs/2026-09-24T09-16-31-phase/baseline
mkdir -p "$B"
b() { n=$1; shift; bash -o pipefail -c "$1" > "$B/$n.log" 2>&1; echo "$n exit=$? · $(tail -1 "$B/$n.log" | cut -c1-160)"; }
b 1 'cargo deny --version'
b 2 'zizmor --version'
b 3 'cargo deny check'
b 4 'test -s scripts/sync-crates.txt && while read -r c; do cargo deny --manifest-path "crates/$c/Cargo.toml" check bans -c deny-sync.toml || exit 1; done < scripts/sync-crates.txt'
b 5 "test -s scripts/sync-crates.txt && cargo check \$(sed 's/^/-p /' scripts/sync-crates.txt)"
b 6 'bash scripts/deny-probes.sh'
b 7 'zizmor .github/workflows/'
b 8 "grep -HnE '\\\$\\{\\{\\s*github\\.event' .github/workflows/*.yml | grep -vE '^[^:]+:[0-9]+:\\s+[A-Z][A-Z0-9_]*:\\s'"
# control for 8: a planted run: body line must be caught
mkdir -p "$B/ctl"; printf 'jobs:\n  x:\n    steps:\n      - run: |\n          echo ${{ github.event.before }}\n' > "$B/ctl/planted.yml"
b 8c "grep -HnE '\\\$\\{\\{\\s*github\\.event' $B/ctl/planted.yml .github/workflows/*.yml | grep -vE '^[^:]+:[0-9]+:\\s+[A-Z][A-Z0-9_]*:\\s'"
# check 8 mechanism: cargo check on an absent package fails
b m1 'cargo check -p viola-absent-probe'
