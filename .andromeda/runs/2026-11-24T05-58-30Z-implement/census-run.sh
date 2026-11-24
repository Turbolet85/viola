#!/usr/bin/env bash
# The profraw census over this chunk's cases that end a fake-agent process, then two cli_open cases as the control
# (inputs#I11). 192 runs over 16 workers per case, every case alike. Each census is named
# target/profraw-census/up-<n>-<test>; nothing is removed.
set -u
cd "$(dirname "$0")/../../.." || exit 2
runs=192
workers=16
n=0
census() {
  n=$((n + 1))
  echo "== $1 $2 · $(date -u +%FT%TZ)"
  env -u VIOLA_NAME -u VIOLA_DIR -u VIOLA_BIN bash scripts/profraw-census.sh --case "$1" "$2" "$runs" "$workers" "up-$n-$2"
  echo "== exit $?"
}
for t in up_reopens_each_pre_boot_instance_parents_first up_moves_each_window_to_its_recorded_workspace \
  up_opens_a_hand_started_instance_unplaced up_skips_a_live_and_a_stale_instance \
  up_leaves_out_an_instance_ended_on_purpose_or_this_boot up_names_no_session_cwd_missing_and_session_live \
  up_dry_run_prints_the_plan_and_spawns_nothing up_json_is_one_document_of_closed_words \
  up_refuses_a_home_another_user_may_read_with_no_line up_without_foot_refuses_terminal_not_found_and_spawns_nothing \
  up_writes_neither_snapshot_nor_stamps; do
  census cli_up "$t"
done
census cli_revive revive_origin_moves_its_window_to_the_recorded_workspace
census cli_revive revive_without_origin_runs_no_window_step
census tui_channel_fds tui_channel_fds_the_child_holds_nothing_of_violas
echo "== control (not this chunk's)"
census cli_open open_spawns_the_terminal_with_the_closed_argv
census cli_open open_records_the_window_found_by_the_terminal_pid
echo "== done $(date -u +%FT%TZ)"
