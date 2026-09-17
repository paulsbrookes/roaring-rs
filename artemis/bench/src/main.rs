//! Artemis reviewer-study benchmark for roaring-rs.
//!
//! Loads every bitmap in the vendored `census1881` and `census1881_srt`
//! datasets, then times a fixed number of passes over all consecutive pairs
//! (i, i+1) of each dataset computing `&a & &b`, `&a | &b`, `&a - &b` and
//! `&a ^ &b`.  The cardinalities of the results are folded into a checksum
//! that is compared against a hard-coded expected value, so the benchmark
//! doubles as a semantic check.  Deterministic: no randomness, no threads.
//!
//! Usage: artemis-bench <output.json>

use roaring::RoaringBitmap;
use std::io::Read;
use std::time::Instant;

const ITERATIONS: u32 = 3000;
const EXPECTED_CHECKSUM: u64 = 126_338_400_000; // computed on the unmodified v0.11.5 tree
const DATASETS: [(&str, &[u8]); 2] = [
    ("census1881", include_bytes!("../../data/census1881.zip")),
    ("census1881_srt", include_bytes!("../../data/census1881_srt.zip")),
];

fn load_dataset(name: &str, bytes: &[u8]) -> Vec<RoaringBitmap> {
    let mut archive =
        zip::ZipArchive::new(std::io::Cursor::new(bytes)).expect("valid vendored zip archive");
    let mut entries: Vec<(usize, String)> = (0..archive.len())
        .map(|i| (i, archive.by_index(i).expect("zip entry").name().to_string()))
        .filter(|(_, n)| n.ends_with(".txt"))
        .collect();
    // Order files by their numeric suffix (name.csvN.txt) so pairing is stable.
    entries.sort_by_key(|(_, n)| {
        let stem = n.trim_end_matches(".txt");
        let digits: String = stem.chars().rev().take_while(|c| c.is_ascii_digit()).collect();
        digits.chars().rev().collect::<String>().parse::<u64>().unwrap_or(u64::MAX)
    });

    let mut bitmaps = Vec::with_capacity(entries.len());
    let mut buf = String::new();
    for (index, _) in entries {
        buf.clear();
        archive.by_index(index).expect("zip entry").read_to_string(&mut buf).expect("utf-8 text");
        let values = buf
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().expect("u32 value"));
        let bitmap = RoaringBitmap::from_sorted_iter(values).expect("sorted values");
        bitmaps.push(bitmap);
    }
    eprintln!("artemis-bench: loaded {} bitmaps from {}", bitmaps.len(), name);
    bitmaps
}

fn main() {
    let out_path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: artemis-bench <output.json>");
        std::process::exit(2);
    });

    let datasets: Vec<Vec<RoaringBitmap>> =
        DATASETS.iter().map(|(name, bytes)| load_dataset(name, bytes)).collect();

    let mut checksum: u64 = 0;
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        for bitmaps in &datasets {
            for pair in bitmaps.windows(2) {
                let (a, b) = (&pair[0], &pair[1]);
                checksum = checksum.wrapping_add((a & b).len());
                checksum = checksum.wrapping_add((a | b).len().wrapping_mul(3));
                checksum = checksum.wrapping_add((a - b).len().wrapping_mul(5));
                checksum = checksum.wrapping_add((a ^ b).len().wrapping_mul(7));
            }
        }
    }
    let elapsed = start.elapsed();
    let pairwise_total_ms = elapsed.as_secs_f64() * 1000.0;

    println!("checksum: {checksum}");
    eprintln!("artemis-bench: {ITERATIONS} iterations, pairwise_total_ms = {pairwise_total_ms:.3}");

    if checksum != EXPECTED_CHECKSUM {
        eprintln!("artemis-bench: CHECKSUM MISMATCH: got {checksum}, expected {EXPECTED_CHECKSUM}");
        std::process::exit(1);
    }

    let json = format!("{{\"pairwise_total_ms\": {pairwise_total_ms:.3}, \"checksum\": {checksum}}}\n");
    std::fs::write(&out_path, json).expect("write results file");
}
