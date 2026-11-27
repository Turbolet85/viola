use super::*;

#[test]
fn paste_bound_a_paste_the_child_takes_is_written_whole_with_enter() {
    let rig = rig();
    let text = "line one\nline two ".repeat(600);
    rig.paste_when_open(&text, TAKEN).expect("taken");
    let bytes = rig.wait_for_bytes("the Enter", |b| b.ends_with(b"\r"));
    assert_eq!(
        bytes,
        [PASTE_START, text.as_bytes(), PASTE_END, b"\r"].concat()
    );
    rig.finish();
}

#[test]
fn paste_bound_a_child_that_stops_reading_cuts_the_paste() {
    let rig = gated(2);
    let handle = rig.paste.clone();
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let started = Instant::now();
        let outcome = paste_when_open(&handle, &long_text(), BOUND);
        let _ = tx.send((outcome, started.elapsed()));
    });
    let (outcome, took) = rx
        .recv_timeout(Duration::from_secs(1))
        .expect("the paste returned within 1 s");
    assert!(
        matches!(outcome, Err(PtyError::PasteCut)),
        "a paste the child never took read {outcome:?}"
    );
    assert!(took >= BOUND, "cut after {took:?}, before its bound");
    rig.finish();
}

#[test]
fn paste_bound_a_cut_paste_closes_the_bracket_without_enter() {
    let rig = gated(2);
    let text = long_text();
    let outcome = rig.paste_when_open(&text, BOUND);
    assert!(matches!(outcome, Err(PtyError::PasteCut)), "{outcome:?}");
    rig.release.store(true, Ordering::SeqCst);
    // A key queued behind the cut paste reaches the writer only once the paste's job is done.
    rig.keys.send(b"k".to_vec()).expect("key");
    let bytes = rig.wait_for_bytes("the key after the cut", |b| b.ends_with(b"k"));
    let body = bytes.strip_prefix(PASTE_START).expect("the start marker");
    let end = find(body, PASTE_END).expect("the closing marker");
    let prefix = &body[..end];
    assert!(
        prefix.len() < text.len() && prefix.len().is_multiple_of(CHUNK),
        "{} text bytes before the closing marker",
        prefix.len()
    );
    assert!(prefix.iter().all(|&b| b == b'a'));
    assert_eq!(
        &body[end..],
        [PASTE_END, b"k"].concat(),
        "an Enter after a cut"
    );
    assert!(!bytes.contains(&b'\r'));
    rig.finish();
}

#[test]
fn paste_bound_a_human_key_waits_behind_at_most_one_chunk() {
    let rig = gated(2);
    let handle = rig.paste.clone();
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(paste_when_open(&handle, &long_text(), BOUND));
    });
    rig.paste_started
        .recv_timeout(WAIT)
        .expect("the writer blocked");
    rig.keys.send(b"k".to_vec()).expect("key");
    rig.key_read.recv_timeout(WAIT).expect("key read");
    let outcome = rx.recv_timeout(WAIT).expect("the paste returned");
    assert!(matches!(outcome, Err(PtyError::PasteCut)), "{outcome:?}");
    rig.release.store(true, Ordering::SeqCst);
    let bytes = rig.wait_for_bytes("the key", |b| b.contains(&b'k'));
    let key = bytes.iter().position(|&b| b == b'k').expect("the key");
    let ahead = &bytes[..key];
    assert!(
        ahead.ends_with(PASTE_END),
        "the key landed inside the bracket"
    );
    let text_ahead = ahead.iter().filter(|&&b| b == b'a').count();
    // The chunk written before the block, and the one in flight at the cut.
    assert!(
        text_ahead <= 2 * CHUNK,
        "{text_ahead} text bytes ahead of the key"
    );
    rig.finish();
}

#[test]
fn pump_returns_with_the_writer_blocked_on_a_child_that_never_reads() {
    let rig = gated(0);
    let handle = rig.paste.clone();
    let paster = std::thread::spawn(move || {
        let _ = paste_when_open(&handle, "x", BOUND);
    });
    rig.paste_started
        .recv_timeout(WAIT)
        .expect("the writer blocked");
    let exited = Instant::now();
    rig.stop.store(true, Ordering::SeqCst);
    while !rig.pump.is_finished() {
        assert!(
            exited.elapsed() < Duration::from_secs(3),
            "the pump did not return"
        );
        std::thread::sleep(Duration::from_millis(1));
    }
    assert!(
        rig.dropped.load(Ordering::SeqCst),
        "the writer thread still runs after the pump returned"
    );
    paster.join().expect("paster");
    assert!(matches!(rig.pump.join().expect("pump"), PumpEnd::Exited(_)));
}

#[test]
fn paste_human_bytes_during_a_paste_land_wholly_after_it() {
    let rig = rig();
    rig.release.store(false, Ordering::SeqCst);
    std::thread::scope(|s| {
        let handle = rig.paste.clone();
        let paste = s.spawn(move || paste_when_open(&handle, "canary-chain-value-5c1e", TAKEN));
        rig.paste_started.recv_timeout(WAIT).expect("paste started");
        rig.keys.send(b"k".to_vec()).expect("key");
        rig.key_read.recv_timeout(WAIT).expect("key read");
        rig.release.store(true, Ordering::SeqCst);
        paste.join().expect("paste").expect("taken");
    });
    let bytes = rig.wait_for_bytes("the key", |b| b.ends_with(b"k"));
    assert_eq!(
        bytes,
        [
            b"\x1b[200~canary-chain-value-5c1e\x1b[201~\r".as_slice(),
            b"k".as_slice()
        ]
        .concat()
    );
    rig.finish();
}

#[test]
fn paste_before_the_pump_starts_is_refused() {
    let paste = PasteHandle::default();
    assert!(matches!(
        paste.paste("early", TAKEN),
        Err(PtyError::NoInput)
    ));
}

#[test]
fn paste_after_the_pump_returns_is_refused() {
    let rig = rig();
    rig.paste_when_open("x", TAKEN).expect("taken");
    let paste = rig.paste.clone();
    rig.finish();
    assert!(matches!(paste.paste("late", TAKEN), Err(PtyError::NoInput)));
}

#[test]
fn paste_human_keys_reach_the_child_through_the_shared_writer() {
    let rig = rig();
    rig.keys.send(b"typed".to_vec()).expect("key");
    assert_eq!(rig.wait_for_bytes("the keys", |b| !b.is_empty()), b"typed");
    rig.finish();
}

#[test]
fn paste_a_failed_write_is_a_write_error() {
    struct Broken;
    impl Write for Broken {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("gone"))
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    let (paste, keys, _, stop, pump) = start(Box::new(Broken), Arc::default());
    assert!(matches!(
        paste_when_open(&paste, "x", TAKEN),
        Err(PtyError::Write(_))
    ));
    stop.store(true, Ordering::SeqCst);
    drop(keys);
    pump.join().expect("pump");
}
