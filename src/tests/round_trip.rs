//! Round-trip integration test: encode → decode → assert equality.

use crate::{DEFAULT_MEM_SIZE, DEFAULT_ORDER};

#[test]
fn ppmd_round_trip_small() {
    use crate::tests::fixtures;
    let input = fixtures::random_seeded(1024);
    let compressed = crate::encode(&input, DEFAULT_ORDER, DEFAULT_MEM_SIZE).expect("encode");
    let decompressed = crate::decode(
        &compressed,
        input.len() as u64,
        DEFAULT_ORDER,
        DEFAULT_MEM_SIZE,
    )
    .expect("decode");
    assert_eq!(input, decompressed, "round-trip must be byte-identical");
}

#[test]
fn ppmd_round_trip_64k() {
    use crate::tests::fixtures;
    let input = fixtures::random_seeded(64 * 1024);
    let compressed = crate::encode(&input, DEFAULT_ORDER, DEFAULT_MEM_SIZE).expect("encode");
    let decompressed = crate::decode(
        &compressed,
        input.len() as u64,
        DEFAULT_ORDER,
        DEFAULT_MEM_SIZE,
    )
    .expect("decode");
    assert_eq!(input, decompressed, "round-trip must be byte-identical");
}

#[test]
fn ppmd_zeros_compresses_well() {
    use crate::tests::fixtures;
    let input = fixtures::zeros(64 * 1024);
    let compressed = crate::encode(&input, DEFAULT_ORDER, DEFAULT_MEM_SIZE).expect("encode");
    let decompressed = crate::decode(
        &compressed,
        input.len() as u64,
        DEFAULT_ORDER,
        DEFAULT_MEM_SIZE,
    )
    .expect("decode");
    assert_eq!(input, decompressed, "round-trip must be byte-identical");
    assert!(
        compressed.len() < input.len() / 10,
        "PPMd should compress zeros heavily; got {} bytes",
        compressed.len()
    );
}

#[test]
fn ppmd_empty_input() {
    let input: Vec<u8> = vec![];
    let compressed = crate::encode(&input, DEFAULT_ORDER, DEFAULT_MEM_SIZE).expect("encode");
    let decompressed =
        crate::decode(&compressed, 0, DEFAULT_ORDER, DEFAULT_MEM_SIZE).expect("decode");
    assert_eq!(decompressed, input);
}
