extern crate oxipng;

use std::default::Default;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

#[test]
fn optimize_from_memory() {
    let mut in_file = File::open("tests/files/fully_optimized.png").unwrap();
    let mut in_file_buf: Vec<u8> = Vec::new();
    in_file.read_to_end(&mut in_file_buf).unwrap();

    let mut opts: oxipng::Options = Default::default();
    opts.pretend = true;
    let result = oxipng::optimize_from_memory(&in_file_buf, &opts);
    assert!(result.is_ok());
}

#[test]
fn optimize_from_memory_corrupted() {
    let mut in_file = File::open("tests/files/corrupted_header.png").unwrap();
    let mut in_file_buf: Vec<u8> = Vec::new();
    in_file.read_to_end(&mut in_file_buf).unwrap();

    let mut opts: oxipng::Options = Default::default();
    opts.pretend = true;
    let result = oxipng::optimize_from_memory(&in_file_buf, &opts);
    assert!(result.is_err());
}

#[test]
fn optimize_from_memory_apng() {
    let mut in_file = File::open("tests/files/apng_file.png").unwrap();
    let mut in_file_buf: Vec<u8> = Vec::new();
    in_file.read_to_end(&mut in_file_buf).unwrap();

    let mut opts: oxipng::Options = Default::default();
    opts.pretend = true;
    let result = oxipng::optimize_from_memory(&in_file_buf, &opts);
    assert!(result.is_ok());
}

#[test]
fn optimize() {
    let in_file = Path::new("tests/files/fully_optimized.png");
    let mut opts: oxipng::Options = Default::default();
    opts.force = true;
    opts.out_file = Some(in_file.with_extension("out.png"));
    let result = oxipng::optimize(in_file, &opts);
    assert!(result.is_ok());
}

#[test]
fn optimize_corrupted() {
    let in_file = Path::new("tests/files/corrupted_header.png");
    let mut opts: oxipng::Options = Default::default();
    opts.force = true;
    opts.out_file = Some(in_file.with_extension("out.png"));
    let result = oxipng::optimize(in_file, &opts);
    assert!(result.is_err());
}

#[test]
fn optimize_apng() {
    let in_file = Path::new("tests/files/apng_file.png");
    let mut opts: oxipng::Options = Default::default();
    opts.force = true;
    opts.out_file = Some(in_file.with_extension("out.png"));
    let result = oxipng::optimize(in_file, &opts);
    assert!(result.is_ok());
    let new_png = oxipng::png::PngData::new(&opts.out_file.unwrap(), false).unwrap();
    assert!(new_png.apng_headers.is_some());
    assert!(new_png.apng_data.is_some());
}
