"""Build evidence/survivors.ndjson from the three witness outcome files: eighteen rows, one object a line.

A `test` row's witness is the outcome line found by file and mutation text (the text as the witness prints it, the
function renamed where a split moved the expression). A `restated` row's witness is a reading: the audit's
mutation text is absent from that function's lines, and the lines the function does hold are quoted.
"""
import json
import re
import sys

E = 'viola-0.1.0/chunks/2026-10-09-epoch-3-cleanup-ii/evidence/'
LINE = re.compile(r'^(\w+)\s+(\S+?):(\d+):(\d+): (.*?)(?: in \d+s build(?: \+ \d+s test)?)?$')
STATE, CLAUDE, ROOT = 'viola-state', 'viola-agent-claude', 'viola'
S = 'crates/viola-state/src/'
L = 'crates/viola-agent-claude/src/ledger.rs'
F = 'src/bin/viola-fake-agent.rs'

# n, unit, file, audit site, audit mutation text, disposition, test, witness text now, function now, obs
ROWS = [
    (1, STATE, S + 'events.rs', '96:19',
     'replace match guard e.kind() == std::io::ErrorKind::NotFound with true in current_len', 'test',
     'events::tests::events_current_len_of_a_log_that_cannot_be_statted_is_an_error', None, None, False),
    (2, STATE, S + 'events.rs', '149:19',
     'replace match guard e.kind() == ErrorKind::NotFound with true in read_within', 'test',
     'events::tests::events_read_of_a_log_that_cannot_be_opened_is_an_error', None, None, False),
    (3, STATE, S + 'events.rs', '209:24', 'replace < with > in LoggedLines::next_line', 'restated', None,
     None, 'LoggedLines::next_line', False),
    (4, STATE, S + 'fs.rs', '290:19',
     'replace match guard fs::read(path).ok().as_deref() != Some(bytes) with true in replace_private_shared',
     'test', 'fs::tests::replace_private_shared_in_a_read_only_dir_is_done_only_over_the_same_bytes', None, None,
     False),
    (5, STATE, S + 'stamps.rs', '28:19',
     'replace match guard e.kind() == io::ErrorKind::NotFound with true in read_capped', 'test',
     'stamps::tests::read_stamps_of_a_file_that_cannot_be_opened_is_an_error', None, None, False),
    (6, STATE, S + 'strict.rs', '34:23',
     'replace match guard e.kind() == io::ErrorKind::NotFound with true in check_stamps', 'test',
     'strict::tests::check_stamps_of_a_path_that_cannot_be_statted_is_unreadable', None, None, False),
    (7, CLAUDE, L, '647:81', 'replace == with != in both_parallel_answered', 'test',
     'ledger::tests::check_dialog_concurrency_with_a_post_for_a_third_id_is_not_answered',
     ('647:81', 'replace == with != in both_parallel_answered'), None, False),
    (8, CLAUDE, L, '670:33', 'replace < with <= in parallel_both_before_first_post', 'restated', None, None,
     'parallel_both_before_first_post', False),
    (9, CLAUDE, L, '685:51', 'replace + with * in clear_start', 'restated', None, None, 'clear_start', False),
    (10, CLAUDE, L, '900:76', 'replace && with || in dialog_variants', 'test',
     'ledger::tests::dialog_variants_with_a_post_of_an_unknown_id_beside_an_identified_call_records_no_post',
     ('898:68', 'replace && with || in joined_call'), None, False),
    (11, CLAUDE, L, '1172:5', 'replace is_local_char -> bool with true', 'test',
     'ledger::tests::has_email_reads_local_at_domain_dot_tld::case_09_space_before_the_at', None, None, False),
    (12, CLAUDE, L, '1181:74', 'replace == with != in has_email', 'test',
     'ledger::tests::has_email_reads_local_at_domain_dot_tld::case_10_hyphen_in_the_domain',
     ('1196:74', 'replace == with != in has_email'), None, False),
    (13, ROOT, F, '328:9', 'replace Agent::close_hooks with ()', 'test',
     'tests::run_hook_after_close_hooks_runs_nothing', None, None, False),
    (14, ROOT, F, '437:74', 'replace == with != in Agent::run_hook', 'test',
     'tests::run_hook_of_a_stop_with_the_receipt_hold_takes_at_least_the_hold', None, None, False),
    (15, ROOT, F, '473:46', 'replace && with || in Agent::submit', 'restated', None, None, 'Agent::end_turn',
     False),
    (16, ROOT, F, '473:57', 'replace == with != in Agent::submit', 'test',
     'tests::submit_of_the_long_paste_with_a_paste_hint_takes_at_least_the_hint',
     ('495:28', 'replace == with != in Agent::end_turn'), None, False),
    (17, ROOT, 'src/cmd/mod.rs', '133:5', 'replace cli_sink -> Option<DetailSink> with None', 'test',
     'wait_in_a_home_whose_diagnostics_is_a_file_keeps_the_chain_in_the_detail_file', None, None, True),
    (18, ROOT, 'src/cmd/hook.rs', '214:72', 'replace > with >= in handle_dialog', 'test',
     'cmd::hook::tests::handle_dialog_with_a_payload_at_the_frame_bound_prints_the_decision_body', None, None,
     False),
]


def outcomes(unit):
    rows = []
    for raw in open(E + 'witness-' + unit + '-outcomes.txt', encoding='utf-8'):
        raw = raw.rstrip('\n')
        m = LINE.match(raw)
        if not m:
            sys.exit('UNPARSED ' + raw)
        rows.append((m.group(1).lower(), m.group(2), m.group(3) + ':' + m.group(4), m.group(5), raw))
    return rows


def main():
    cache, out, problems = {}, [], []
    for n, unit, file, site, mutation, disposition, test, moved, function, obs in ROWS:
        lines = cache.setdefault(unit, outcomes(unit))
        in_file = [r for r in lines if r[1] == file]
        if disposition == 'test':
            text = moved[1] if moved else mutation
            hits = [r for r in in_file if r[3] == text and (not moved or r[2] == moved[0])]
            if len(hits) != 1:
                problems.append('row {}: {} lines for {!r}'.format(n, len(hits), text))
                continue
            word, raw = hits[0][0], hits[0][4]
            if word != 'caught':
                problems.append('row {}: reads {}'.format(n, word))
            record = {'outcome': 'caught', 'witness': raw}
        else:
            audit_text = mutation.rsplit(' in ', 1)[0]
            suffix = ' in ' + function
            held = [r for r in in_file if r[3].endswith(suffix)]
            same_site = [r for r in held if r[2] == site and r[3].startswith(audit_text)]
            bad = [r for r in held if r[0] != 'caught' and r[0] != 'unviable']
            if same_site or bad or not held:
                problems.append('row {}: same_site {} bad {} held {}'.format(n, len(same_site), len(bad), len(held)))
            record = {'outcome': 'gone',
                      'witness': 'the witness run holds {} lines of {}, none at {} and none missed or timed out: {}'
                      .format(len(held), function, site, ' | '.join(r[4] for r in held))}
        out.append({'n': n, 'site': file + ':' + site, 'mutation': mutation, 'disposition': disposition,
                    'test': test, 'outcome': record['outcome'], 'obs': obs, 'witness': record['witness']})
    for problem in problems:
        print('PROBLEM', problem)
    if problems:
        sys.exit(1)
    with open(E + 'survivors.ndjson', 'w', encoding='utf-8', newline='\n') as f:
        for row in out:
            f.write(json.dumps(row, ensure_ascii=False) + '\n')
    print('wrote {} rows'.format(len(out)))


main()
