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
  parse
  format
  minify
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

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

| File size  | Scan (s) | Parse (s) | Format (s) | Minify (s) |
| :----------| :------: | :------:  | :------:   | :--------: |
| 64KB       | 0.0002   | 0.0003    | 0.0007     | 0.0006     |
| 128KB      | 0.0012   | 0.0016    | 0.0030     | 0.0025     |
| 256KB      | 0.0012   | 0.0016    | 0.0029     | 0.0026     |
| 512KB      | 0.0025   | 0.0034    | 0.0059     | 0.0058     |
| 1MB        | 0.0057   | 0.0073    | 0.0130     | 0.0108     |
| 5MB        | 0.0294   | 0.0389    | 0.0661     | 0.0575     |

#### Minified JSON

| File size  | Scan (s) | Parse (s) | Format (s) | Minify (s) |
| :----------| :------: | :------:  | :------:   | :--------: |
| 64KB       | 0.0002   | 0.0003    | 0.0007     | 0.0006     |
| 128KB      | 0.0010   | 0.0014    | 0.0027     | 0.0023     |
| 256KB      | 0.0010   | 0.0014    | 0.0028     | 0.0023     |
| 512KB      | 0.0022   | 0.0029    | 0.0056     | 0.0048     |
| 1MB        | 0.0043   | 0.0059    | 0.0115     | 0.0095     |
| 5MB        | 0.0227   | 0.0334    | 0.0628     | 0.0529     |

---

### Lenovo ThinkPad X13 Gen 1

* Chip - AMD Ryzen™️ 5 Pro 4650U
* Memory - 8GiB

#### Formatted JSON

| File size  | Scan (s) | Parse (s) | Format (s) | Minify (s) |
| :----------| :------: | :------:  | :------:   | :--------: |
| 64KB       | 0.0002   | 0.0004    | 0.0008     | 0.0006     |
| 128KB      | 0.0012   | 0.0017    | 0.0035     | 0.0027     |
| 256KB      | 0.0010   | 0.0017    | 0.0035     | 0.0028     |
| 512KB      | 0.0021   | 0.0035    | 0.0071     | 0.0055     |
| 1MB        | 0.0043   | 0.0070    | 0.0142     | 0.0112     |
| 5MB        | 0.0236   | 0.0366    | 0.0714     | 0.0560     |

#### Minified JSON

| File size  | Scan (s) | Parse (s) | Format (s) | Minify (s) |
| :----------| :------: | :------:  | :------:   | :--------: |
| 64KB       | 0.0002   | 0.0003    | 0.0008     | 0.0006     |
| 128KB      | 0.0009   | 0.0015    | 0.0032     | 0.0025     |
| 256KB      | 0.0008   | 0.0015    | 0.0033     | 0.0025     |
| 512KB      | 0.0019   | 0.0033    | 0.0070     | 0.0053     |
| 1MB        | 0.0038   | 0.0065    | 0.0139     | 0.0107     |
| 5MB        | 0.0184   | 0.0319    | 0.0829     | 0.0518     |

### Improvements

It helps to see a breakdown of each algorithm in terms of percentage take for example formatting 5MB of
test data:

** Data taken from tables above **

| Machine     | File size   | Scan (s)     | Parse (s)    | Format (s)   |
| :---------- | :---------: | :----------: | :----------: | :----------: |
| ThinkPad    | 5MB         | 0.0236 (33%) | 0.0366 (18%) | 0.0714 (49%) |
| ThinkPad    | 5MB-min     | 0.0184 (22%) | 0.0318 (16%) | 0.0829 (62%) |
| MacBook     | 5MB         | 0.0294 (45%) | 0.0389 (14%) | 0.0661 (41%) | 
| MacBook     | 5MB-min     | 0.0227 (36%) | 0.0334 (17%) | 0.0628 (47%) |

The results show that scanning and formatting are computationally expensive compared to parsing and highlight
the areas where improvements can be made

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
