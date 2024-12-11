use core::str;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use format::formatter::Formatter;
use parser::parser::Parser;
use scanner::scanner::Scanner;
use std::fs::{self};

fn test(c: &mut Criterion) {
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

    let bytes = fs::read("json/test.json").unwrap();
    let source = str::from_utf8(&bytes).unwrap();

    c.bench_with_input(BenchmarkId::new("test", 1), &source, |b, s| {
        b.iter(|| scan_to_format(&s));
    });
}

criterion_group!(benches, test);
criterion_main!(benches);
