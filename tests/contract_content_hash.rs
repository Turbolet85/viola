//! The pinned SHA-256 crate behind the `<hash>` of `bin/<version>-<hash>/` (security-plan §Data Protection,
//! Code-bearing artefacts): published FIPS 180-2 vectors and the 16-hex truncation.

use rstest::rstest;
use sha2::{Digest, Sha256};

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[rstest]
#[case::empty(
    b"",
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
)]
#[case::abc(
    b"abc",
    "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
)]
#[case::two_blocks(
    b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
    "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
)]
fn sha256_published_vector_matches_digest(#[case] message: &[u8], #[case] digest: &str) {
    assert_eq!(hex(&Sha256::digest(message)), digest);
}

#[rstest]
#[case::empty(b"", "e3b0c44298fc1c14")]
#[case::abc(b"abc", "ba7816bf8f01cfea")]
fn sha256_truncated_to_eight_bytes_yields_sixteen_hex(
    #[case] message: &[u8],
    #[case] prefix: &str,
) {
    let truncated = hex(&Sha256::digest(message)[..8]);
    assert_eq!(truncated, prefix);
    assert_eq!(truncated.len(), 16);
    assert!(
        truncated
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    );
}
