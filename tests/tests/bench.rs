use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion, PlotConfiguration,
};
use tests::{read, Action, Runner};

fn parse_minified_json(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_minified_json");

    group
        .plot_config(PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic));

    let inputs = vec![
        ("64KB", read("json/64KB-min.json")),
        ("128KB", read("json/128KB-min.json")),
        ("256KB", read("json/256KB-min.json")),
        ("512KB", read("json/512KB-min.json")),
        ("1MB", read("json/1MB-min.json")),
        ("5MB", read("json/5MB-min.json")),
    ];

    let runner = Runner;

    for (name, input) in inputs.into_iter() {
        group.bench_with_input(BenchmarkId::new("scan", name), &input, |b, s| {
            b.iter(|| black_box(runner.run(Action::Scan, &s)));
        });

        group.bench_with_input(BenchmarkId::new("parse", name), &input, |b, s| {
            b.iter(|| black_box(runner.run(Action::Parse, &s)));
        });

        group.bench_with_input(BenchmarkId::new("format", name), &input, |b, s| {
            b.iter(|| black_box(runner.run(Action::Format, &s)));
        });

        group.bench_with_input(BenchmarkId::new("minify", name), &input, |b, s| {
            b.iter(|| black_box(runner.run(Action::Minify, &s)));
        });
    }

    group.finish();
}

fn parse_formatted_json(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_formatted_json");

    group
        .plot_config(PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic));

    let inputs = vec![
        ("64KB", read("json/64KB.json")),
        ("128KB", read("json/128KB.json")),
        ("256KB", read("json/256KB.json")),
        ("512KB", read("json/512KB.json")),
        ("1MB", read("json/1MB.json")),
        ("5MB", read("json/5MB.json")),
    ];

    let runner = Runner;

    for (name, input) in inputs.into_iter() {
        group.bench_with_input(BenchmarkId::new("scan", name), &input, |b, s| {
            b.iter(|| black_box(runner.run(Action::Scan, &s)));
        });

        group.bench_with_input(BenchmarkId::new("parse", name), &input, |b, s| {
            b.iter(|| black_box(runner.run(Action::Parse, &s)));
        });

        group.bench_with_input(BenchmarkId::new("format", name), &input, |b, s| {
            b.iter(|| black_box(runner.run(Action::Format, &s)));
        });

        group.bench_with_input(BenchmarkId::new("minify", name), &input, |b, s| {
            b.iter(|| black_box(runner.run(Action::Minify, &s)));
        });
    }

    group.finish();
}

criterion_group!(benches, parse_formatted_json, parse_minified_json);
criterion_main!(benches);
