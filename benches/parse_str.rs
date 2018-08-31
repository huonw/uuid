#![feature(test)]
#[cfg(feature = "slog")]
#[macro_use]
extern crate slog;
extern crate test;
extern crate uuid;

#[cfg(feature = "slog")]
use slog::Drain;
use test::Bencher;
use uuid::prelude::*;

#[bench]
fn bench_parse(b: &mut Bencher) {
    b.iter(|| {
        let _ = Uuid::parse_str("");
        let _ = Uuid::parse_str("!");
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-B6BF-329BF39FA1E45");
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-BBF-329BF39FA1E4");
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-BGBF-329BF39FA1E4");
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-B6BFF329BF39FA1E4");
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa");
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faaXB6BFF329BF39FA1E4");
        let _ = Uuid::parse_str("F9168C5E-CEB-24fa-eB6BFF32-BF39FA1E4");
        let _ = Uuid::parse_str("01020304-1112-2122-3132-41424344");
        let _ = Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c88");
        let _ = Uuid::parse_str("67e5504410b1426f9247bb680e5fe0cg8");
        let _ = Uuid::parse_str("67e5504410b1426%9247bb680e5fe0c8");

        // Valid
        let _ = Uuid::parse_str("00000000000000000000000000000000");
        let _ = Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8");
        let _ = Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8");
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-B6BF-329BF39FA1E4");
        let _ = Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c8");
        let _ = Uuid::parse_str("01020304-1112-2122-3132-414243444546");
        let _ =
            Uuid::parse_str("urn:uuid:67e55044-10b1-426f-9247-bb680e5fe0c8");

        // Nil
        let _ = Uuid::parse_str("00000000000000000000000000000000");
        let _ = Uuid::parse_str("00000000-0000-0000-0000-000000000000");

        // Test error reporting
        let _ = Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c");
        let _ = Uuid::parse_str("67e550X410b1426f9247bb680e5fe0cd");
        let _ = Uuid::parse_str("67e550-4105b1426f9247bb680e5fe0c");
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-B6BF1-02BF39FA1E4");
    });
}

#[bench]
fn bench_parse_invalid_len(b: &mut Bencher) {
    b.iter(|| {
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-BBF-329BF39FA1E4");
    })
}

#[bench]
fn bench_parse_invalid_character(b: &mut Bencher) {
    b.iter(|| {
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-BGBF-329BF39FA1E4");
    })
}

#[bench]
fn bench_parse_invalid_group_len(b: &mut Bencher) {
    b.iter(|| {
        let _ = Uuid::parse_str("01020304-1112-2122-3132-41424344");
    });
}

#[bench]
fn bench_parse_invalid_groups(b: &mut Bencher) {
    b.iter(|| {
        let _ = Uuid::parse_str("F9168C5E-CEB2-4faa-B6BFF329BF39FA1E4");
    });
}

#[bench]
fn bench_valid_urn(b: &mut Bencher) {
    let uuids = [
        "urn:uuid:550e8400-e29b-41d4-a716-446655440000",
        "urn:uuid:67e55044-10b1-426f-9247-bb680e5fe0c8",
        "urn:uuid:F9168C5E-CEB2-4faa-D6BF-329BF39FA1E4",
        "urn:uuid:f81d4fae-7dec-11d0-7765-00a0c91e6bf6",
    ];
    b.iter(|| uuids.iter().all(|s| Uuid::parse_str(s).is_ok()))
}
#[bench]
fn bench_valid_hyphenated(b: &mut Bencher) {
    let uuids = [
        "550e8400-e29b-41d4-a716-446655440000",
        "67e55044-10b1-426f-9247-bb680e5fe0c8",
        "F9168C5E-CEB2-4faa-D6BF-329BF39FA1E4",
        "f81d4fae-7dec-11d0-7765-00a0c91e6bf6",
    ];
    b.iter(|| uuids.iter().all(|s| Uuid::parse_str(s).is_ok()))
}
#[bench]
fn bench_valid_short(b: &mut Bencher) {
    let uuids = [
        "550e8400e29b41d4a716446655440000",
        "67e5504410b1426f9247bb680e5fe0c8",
        "F9168C5ECEB24faaD6BF329BF39FA1E4",
        "f81d4fae7dec11d0776500a0c91e6bf6",
    ];
    b.iter(|| uuids.iter().all(|s| Uuid::parse_str(s).is_ok()))
}

#[cfg(feature = "slog")]
#[bench]
fn bench_log_discard_kv(b: &mut Bencher) {
    let root = slog::Logger::root(slog::Discard.fuse(), o!());
    let u1 = Uuid::parse_str("F9168C5E-CEB2-4FAB-B6BF-329BF39FA1E4").unwrap();
    b.iter(|| {
        crit!(root, "test"; "u1" => u1);
    });
}
