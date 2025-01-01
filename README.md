# jtools

CLI tools for working with JSON written in Rust.

**Aim**

Manage JSON without relying on an online tool, ensuring clear and concise error reporting to
make debugging easier.

**Features**

- Error reporting inspired by Rust for example:

```text
Unterminated string
  +|
   |
16 |"message": "Hello,
   |           ^___
  +|
```
- Handwritten scanner/lexical analyser
- Recursive descent parser
- Formatter
- Minifier
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

## Examples (zsh or bash)

A few useful commands that can be combined with shell features:

* Format data and copy the result to the clipboard

```bash
jtools format -w file "data.json" | pbcopy
```

* Minify data and redirect the output to a new file without overriding the original

```bash
jtools minify -w file -p "data.json" > "data-min.json"
```

* Format data from standard input and append the output to a file

```bash
jtools format -s 2 -w text '{ "message": "Hello, World!" }' >> "data.json"
```

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

Always take benchmark tests with a pinch of salt!

* Benchmarker
    * [Criterion](https://crates.io/crates/criterion/) is used to perform all benchmark tests
* Data
    * [Test JSON data](https://microsoftedge.github.io/Demos/json-dummy-data/) used in all of the tests

### MacBook Pro 2021

* Chip - Apple M1 Pro
* Memory - 16GB

#### Formatted JSON

| File size  | Scan (s) | Parse (s) | Format (s) | Minify (s) |
| :----------| :------: | :------:  | :------:   | ------:    |
| 64KB       | 0.0002   | 0.0003    | 0.0007     | 0.0006     |
| 128KB      | 0.0012   | 0.0016    | 0.0030     | 0.0025     |
| 256KB      | 0.0012   | 0.0016    | 0.0029     | 0.0026     |
| 512KB      | 0.0025   | 0.0034    | 0.0059     | 0.0058     |
| 1MB        | 0.0057   | 0.0073    | 0.0130     | 0.0108     |
| 5MB        | 0.0294   | 0.0389    | 0.0661     | 0.0575     |

#### Minified JSON

| File size  | Scan (s) | Parse (s) | Format (s) | Minify (s) |
| :----------| :------: | :------:  | :------:   | ------:    |
| 64KB       | 0.0002   | 0.0003    | 0.0007     | 0.0006     |
| 128KB      | 0.0010   | 0.0014    | 0.0027     | 0.0023     |
| 256KB      | 0.0010   | 0.0014    | 0.0028     | 0.0023     |
| 512KB      | 0.0022   | 0.0029    | 0.0056     | 0.0048     |
| 1MB        | 0.0043   | 0.0059    | 0.0115     | 0.0095     |
| 5MB        | 0.0227   | 0.0334    | 0.0628     | 0.0529     |

