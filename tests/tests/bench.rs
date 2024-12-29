use core::str;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use parser::parser::Parser;
use scanner::scanner::Scanner;
use std::fs::{self};

fn scan_to_format(source: &str) {
    let mut scanner = Scanner::new(source);

    let tokens = match scanner.scan() {
        Ok(tokens) => tokens,
        Err(error) => {
            return eprintln!("{}", error);
        }
    };

    let parser = Parser::new(source, tokens);

    let _ = match parser.parse() {
        Ok(ast) => ast,
        Err(error) => {
            return eprintln!("{}", error);
        }
    };
}

fn read(path: &str) -> String {
    let bytes = fs::read(path).unwrap();

    str::from_utf8(&bytes).unwrap().to_string()
}

fn minified_json(c: &mut Criterion) {
    let mut group = c.benchmark_group("minified_json");

    let inputs = vec![
        ("64KB", read("json/64KB-min.json")),
        ("128KB", read("json/128KB-min.json")),
        ("256KB", read("json/256KB-min.json")),
        ("512KB", read("json/512KB-min.json")),
        ("1MB", read("json/1MB-min.json")),
        ("5MB", read("json/5MB-min.json")),
    ];

    for (name, input) in inputs.into_iter() {
        group.bench_with_input(BenchmarkId::new("size", name), &input, |b, s| {
            b.iter(|| black_box(scan_to_format(&s)));
        });
    }

    group.finish();
}

fn formatted_json(c: &mut Criterion) {
    let mut group = c.benchmark_group("formatted_json");

    let inputs = vec![
        ("64KB", read("json/64KB.json")),
        ("128KB", read("json/128KB.json")),
        ("256KB", read("json/256KB.json")),
        ("512KB", read("json/512KB.json")),
        ("1MB", read("json/1MB.json")),
        ("5MB", read("json/5MB.json")),
    ];

    for (name, input) in inputs.into_iter() {
        group.bench_with_input(BenchmarkId::new("size", name), &input, |b, s| {
            b.iter(|| black_box(scan_to_format(&s)));
        });
    }

    group.finish();
}

criterion_group!(benches, minified_json, formatted_json);
criterion_main!(benches);
