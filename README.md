# jtools

CLI tools for working with JSON written in Rust

**Aim**

Manage JSON without relying on an online tool, focusing on clear error reporting
and strict validation to simplify debugging

**Features**

- Error reporting inspired by Rust for example:

```text
Unterminated string
  +|
   |
16 |"message": "Hello,
   |           ^---Column=14
  +|
```
- Formatter
- Minifier
- Recursive descent parser
- Handwritten scanner/lexical analyser
---

## Installation

1) Download a compatible binary from the [jtools GitHub releases page](https://github.com/AlexanderWatts/jtools/releases "jtools GitHub releases page")
2) Extract and move the `jtools` binary to a directory included in your `$PATH`
    - On macOS, for example, you can move it to `/usr/local/bin`

## Usage

To see all available commands run:

```bash
jtools -h

# OR

jtools --help
```

Output:

```text
Usage: jtools <COMMAND>

Commands:
  parse   Parse
  format  Format
  minify  Minify
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Examples

```bash
# Parsing
jtools parse text '{ "message": "Hello, 🌎!" }'
jtools parse file "data.json"
jtools parse --verify text '[1, 2, 3, 4]'

# Formatting
jtools format text '{ "title": "json", "tags": [] }'
jtools format file "data.json"
jtools format file --prevent-override "data.json"
jtools format --spacing 2 text '["hello", 1e10]'

# Minification
jtools minify text '[{}, [100, "😀", "🚀"]]'
jtools minify file "data.json"
jtools minify file --prevent-override "data.json"
```

## Comparison

Notable differences between `jtools parse` and JavaScript's `JSON.parse(...)`

### Properties

```javascript
JSON.parse('{"language":"Rust", "language": null}')
```
* Passes - Removes the first duplicate property

```bash
jtools parse text '{"language":"Rust", "language": null}'
```
* Fails - Duplicate properties not allowed

### Numbers

```javascript
JSON.parse('10e1000')
```
* Passes - Returns Infinity

```bash
jtools parse text '10e1000'
```
* Fails - Follows RFC 8259 and only supports binary64

## Parser Design

The parser was built from the following custom Context Free Grammar (CFG):

```
json := literal ;
object := "{" ( property ( "," property )* )* "}" ;
property := string ":" literal ;
array := "[" ( literal ( "," literal )* )* "]" ;
literal := string | number | "true" | "false" | "null" | object | array ;
```

Note: Both string and number have not been expanded for readability and whitespace is ignored. See
[RFC 8259](https://datatracker.ietf.org/doc/html/rfc8259#section-7) for the JSON specification

## Performance

Benchmark tests should be taken with a pinch of salt!

* Test data
    * Formatted and minified [JSON files](https://microsoftedge.github.io/Demos/json-dummy-data/)
    from 64KB to 5MB
    * This data is fairly basic and lacks a variety of features but serves as a good default.
    Keep this in mind when reviewing the benchmark results below
* Benchmark Overview
    * [Criterion](https://crates.io/crates/criterion/) is used to perform benchmarking with a default
    sample size of 100 and a warm-up time of 500ms
    * The test data is passed separately to each pipeline such as `Scanner->Parser->Formatter`
    where the performance is measured at each stage and reported

### MacBook Pro 2021

* Chip - Apple M1 Pro
* Memory - 16GB

#### Formatted JSON

| File size  | Scan & Parse (s)  | Format (s) | Minify (s) |
| :--------- | :---------------: | :--------: | :--------: |
| 64KB       | 0.0003            | 0.0007     | 0.0006     |
| 128KB      | 0.0014            | 0.0020     | 0.0025     |
| 256KB      | 0.0015            | 0.0027     | 0.0023     |
| 512KB      | 0.0030            | 0.0057     | 0.0048     |
| 1MB        | 0.0067            | 0.0120     | 0.0101     |
| 5MB        | 0.0345            | 0.0623     | 0.0510     |

#### Minified JSON

| File size  | Scan & Parse (s)  | Format (s) | Minify (s) |
| :--------- | :---------------: | :--------: | :--------: |
| 64KB       | 0.0003            | 0.0007     | 0.0005     |
| 128KB      | 0.0014            | 0.0027     | 0.0022     |
| 256KB      | 0.0014            | 0.0027     | 0.0022     |
| 512KB      | 0.0028            | 0.0055     | 0.0046     |
| 1MB        | 0.0056            | 0.0109     | 0.0090     |
| 5MB        | 0.0307            | 0.0588     | 0.0490     |

---

### Lenovo ThinkPad X13 Gen 1

* Chip - AMD Ryzen™️ 5 Pro 4650U
* Memory - 8GiB

#### Formatted JSON

| File size  | Scan & Parse (s) | Format (s) | Minify (s) |
| :--------- | :--------------: | :--------: | :--------: |
| 64KB       | 0.0005           | 0.0009     | 0.0007     |
| 128KB      | 0.0019           | 0.0036     | 0.0029     |
| 256KB      | 0.0019           | 0.0036     | 0.0028     |
| 512KB      | 0.0038           | 0.0074     | 0.0059     |
| 1MB        | 0.0078           | 0.0150     | 0.0119     |
| 5MB        | 0.0398           | 0.0750     | 0.0586     |

#### Minified JSON

| File size  | Scan & Parse (s) | Format (s) | Minify (s) |
| :--------- | :--------------: | :--------: | :--------: |
| 64KB       | 0.0004           | 0.0009     | 0.0007     |
| 128KB      | 0.0018           | 0.0035     | 0.0027     |
| 256KB      | 0.0018           | 0.0036     | 0.0028     |
| 512KB      | 0.0038           | 0.0073     | 0.0057     |
| 1MB        | 0.0075           | 0.0150     | 0.0119     |
| 5MB        | 0.0396           | 0.0743     | 0.0586     |

### Improvements

It helps to see a breakdown of each algorithm in terms of percentage take for example formatting 5MB of
test data:

** Data taken from tables above **

| Machine     | File size   | Parse (s)    | Format (s)   |
| :---------- | :---------: | :----------: | :----------: |
| ThinkPad    | 5MB         | 0.0398 (53%) | 0.0750 (47%) |
| ThinkPad    | 5MB-min     | 0.0396 (53%) | 0.0743 (47%) |
| MacBook     | 5MB         | 0.0345 (55%) | 0.0623 (45%) | 
| MacBook     | 5MB-min     | 0.0307 (52%) | 0.0588 (48%) |


The formatting algorithm recursively traverses the AST and rebuilds the input, not surprisingly this
is computationally expensive and takes almost half of the runtime. Alternative algorithms should be
explored to improve this

## Running jtools locally

1) Install the latest stable version of Rust

### Run

To run jtools without building a binary use `cargo run`

```bash
# cargo run -- <arguments>

# This is like running -> jtools parse text '[1, 2, 3]'
cargo run -- parse text '[1, 2, 3]'
```
See [examples](#examples) for some alternative arguments (just replace jtools with `cargo run -- `)

OR

```bash
cargo build --release
```

This builds a binary at `/target/release/jtools` from the root directory. For example, the following can then
be run:


```bash
./target/release/jtools parse text '[1, 2, 3]'
```

### Documentation

Create and open the documentation

```bash
cargo doc --open
```

### Test

Run all tests

```bash
cargo test
```

### Benchmarking

Before running `cargo bench`:

Download the [JSON files](https://microsoftedge.github.io/Demos/json-dummy-data/) with the following
file names into `/benches/json`

* 1MB-min.json
* 1MB.json
* 5MB-min.json
* 5MB.json
* 64KB-min.json
* 64KB.json
* 128KB-min.json
* 128KB.json
* 256KB-min.json
* 256KB.json
* 512KB-min.json
* 512KB.json

To see the graphs produced by Criterion open the HTML report it generates after doing `cargo bench` at
`/target/criterion/report/index.html`
