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

## Examples (zsh or bash)

A few useful commands that can be combined with shell features:

* Format a JSON file and copy the result to the clipboard

```bash
jtools format -p file "data.json" | pbcopy
```

* Minify data from a JSON file and redirect the output to a new file

```bash
jtools minify -p file "data.json" > "data-min.json"
```

* Parse data from standard input and append the output to an existing file

```bash
jtools parse -p stdin '{ "message": "Hello, World!" }' >> "data.json"
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
