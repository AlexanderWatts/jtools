use core::str;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use format::formatter::Formatter;
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
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(error) => {
            return eprintln!("{}", error);
        }
    };

    let formatter = Formatter::default();
    let _ = formatter.format(&ast);
}

fn read(path: &str) -> String {
    let bytes = fs::read(path).unwrap();

    str::from_utf8(&bytes).unwrap().to_string()
}

fn multi_source_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("json");

    let inputs = vec![
        ("64KB", read("json/64KB.json")),
        ("128KB", read("json/128KB.json")),
        ("256KB", read("json/256KB.json")),
        ("512KB", read("json/512KB.json")),
        ("1MB", read("json/1MB.json")),
        ("5MB", read("json/5MB.json")),
        ("92MB", read("json/92MB.json")),
    ];

    for (name, input) in inputs.into_iter() {
        group.bench_with_input(BenchmarkId::new("file", name), &input, |b, s| {
            b.iter(|| black_box(scan_to_format(&s)));
        });
    }

    group.finish();
}

criterion_group!(benches, multi_source_size);
criterion_main!(benches);
