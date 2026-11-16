//! The one transcript read `send` makes (obs-plan · decision/transcripts-not-a-contract, its one
//! exception): a local command's own [`ECHO_TAG`] counted in the session transcript the CLI
//! declared, from the send's pre-paste end, bounded at `MAX_FRAME`. Only the count leaves: no
//! byte of the file is kept, logged or returned, and a failed read confirms nothing.

use std::fs::File;
use std::io::Read;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::Path;

use viola_agent_claude::ledger::ECHO_TAG;
use viola_core::MAX_FRAME;

/// How much of the file one read takes at a time.
const CHUNK: usize = 64 * 1024;

/// The file opened read-only without following a symbolic link at `path`, only when it is a
/// regular file this user owns.
fn open(path: &Path) -> Option<File> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
        .ok()?;
    let meta = file.metadata().ok()?;
    // SAFETY: `geteuid` takes no arguments, cannot fail and touches no memory.
    let euid = unsafe { libc::geteuid() };
    (meta.file_type().is_file() && meta.uid() == euid).then_some(file)
}

/// The transcript's end: the offset a send's echo is counted from.
pub(crate) fn end_of(path: &Path) -> Option<u64> {
    open(path)?.metadata().ok().map(|meta| meta.len())
}

/// How many [`ECHO_TAG`]s stand wholly past `offset`, reading at most `MAX_FRAME` bytes from it.
pub(crate) fn tags_since(path: &Path, offset: u64) -> Option<usize> {
    let file = open(path)?;
    let mut reader = std::io::BufReader::new(file);
    std::io::Seek::seek(&mut reader, std::io::SeekFrom::Start(offset)).ok()?;
    let mut reader = reader.take(MAX_FRAME);
    let tag = ECHO_TAG.as_bytes();
    let mut window: Vec<u8> = Vec::with_capacity(CHUNK + tag.len());
    let mut chunk = vec![0; CHUNK];
    let mut count = 0;
    loop {
        let read = reader.read(&mut chunk).ok()?;
        if read == 0 {
            return Some(count);
        }
        window.extend_from_slice(&chunk[..read]);
        let mut at = 0;
        while let Some(found) = window[at..].windows(tag.len()).position(|w| w == tag) {
            count += 1;
            at += found + tag.len();
        }
        // Keep only a tail too short to hold a whole tag, so a tag split across two reads counts
        // once.
        let keep = window.len().saturating_sub(at).min(tag.len() - 1);
        window.drain(..window.len() - keep);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn written(bytes: &[u8]) -> (tempfile::TempDir, std::path::PathBuf) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("t.jsonl");
        std::fs::write(&path, bytes).expect("transcript");
        (tmp, path)
    }

    #[test]
    fn echo_counts_the_tags_past_the_offset() {
        let line = format!("{{\"content\":\"{ECHO_TAG}x</local-command-stdout>\"}}\n");
        let (_tmp, path) = written(format!("{line}{line}").as_bytes());
        assert_eq!(end_of(&path), Some(2 * line.len() as u64));
        assert_eq!(tags_since(&path, 0), Some(2));
        assert_eq!(tags_since(&path, line.len() as u64), Some(1));
        assert_eq!(tags_since(&path, 2 * line.len() as u64), Some(0));
        assert_eq!(tags_since(&path, 10 * line.len() as u64), Some(0));
    }

    /// A tag that begins before the offset is the earlier send's or the human's, never this one's.
    #[test]
    fn echo_a_tag_split_across_the_offset_is_not_counted() {
        let (_tmp, path) = written(format!("ab{ECHO_TAG}cd").as_bytes());
        assert_eq!(tags_since(&path, 3), Some(0));
        assert_eq!(tags_since(&path, 2), Some(1));
    }

    #[test]
    fn echo_a_tag_split_across_two_reads_counts_once() {
        let mut bytes = vec![b'x'; CHUNK - 5];
        bytes.extend_from_slice(ECHO_TAG.as_bytes());
        bytes.extend_from_slice(ECHO_TAG.as_bytes());
        let (_tmp, path) = written(&bytes);
        assert_eq!(tags_since(&path, 0), Some(2));
    }

    #[test]
    fn echo_a_symlink_is_refused() {
        let (tmp, path) = written(ECHO_TAG.as_bytes());
        let link = tmp.path().join("link.jsonl");
        std::os::unix::fs::symlink(&path, &link).expect("symlink");
        assert_eq!(end_of(&link), None);
        assert_eq!(tags_since(&link, 0), None);
        assert_eq!(tags_since(&path, 0), Some(1));
    }

    #[test]
    fn echo_a_non_regular_file_is_refused() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(end_of(tmp.path()), None);
        assert_eq!(tags_since(tmp.path(), 0), None);
        assert_eq!(end_of(&tmp.path().join("absent")), None);
        assert_eq!(tags_since(&tmp.path().join("absent"), 0), None);
    }

    /// A tag wholly past `MAX_FRAME` bytes from the offset is not read.
    #[test]
    fn echo_a_read_is_capped_at_max_frame() {
        let cap = usize::try_from(MAX_FRAME).expect("fits");
        let mut bytes = vec![b'x'; cap - ECHO_TAG.len()];
        bytes.extend_from_slice(ECHO_TAG.as_bytes());
        bytes.extend_from_slice(ECHO_TAG.as_bytes());
        let (_tmp, path) = written(&bytes);
        assert_eq!(tags_since(&path, 0), Some(1));
        assert_eq!(
            tags_since(&path, 1),
            Some(1),
            "the second tag ends one past the cap"
        );
        assert_eq!(tags_since(&path, ECHO_TAG.len() as u64), Some(2));
    }
}
