# Rune

Rune is an experimental programming language built from scratch in Rust, focused on simplicity, performance, and compiler design.

> Rune is currently in early development. The syntax, architecture, and features may change frequently.

## Example

```rune
print("Hello, ");
println("world!");

println(10 + 20);
println(2 + 3 * 4);
println((2 + 3) * 4);
```

Output:

```text
Hello, world!
30
14
20
```

## Current Features

Rune currently supports:

- String literals
- Integer and decimal number literals
- Identifiers
- `print` and `println`
- Arithmetic operators: `+`, `-`, `*`, `/`
- Operator precedence
- Grouped expressions with parentheses
- Division-by-zero runtime errors
- Semicolon-terminated statements
- Source location tracking with line and column information
- `.rune` source files

## Architecture

Rune currently follows this pipeline:

```text
Source Code
    ↓
Lexer
    ↓
Tokens
    ↓
Parser
    ↓
AST
    ↓
Interpreter
```

### Lexer

Converts Rune source code into tokens, including identifiers, strings, numbers, punctuation, and arithmetic operators.

### Parser

Transforms tokens into an Abstract Syntax Tree and handles arithmetic precedence and grouped expressions.

### AST

Represents Rune programs as statements and expressions, including binary arithmetic expressions.

### Interpreter

Executes the AST and currently acts as a reference implementation while Rune's compiler architecture is developed.

## Arithmetic

Rune respects standard arithmetic precedence:

```rune
println(2 + 3 * 4);
```

Output:

```text
14
```

Parentheses can change the evaluation order:

```rune
println((2 + 3) * 4);
```

Output:

```text
20
```

## Building

Rune requires Rust and Cargo.

Clone the repository:

```bash
git clone https://github.com/MateusSoaresL/rune.git
cd rune
```

Build:

```bash
cargo build
```

## Running

Create a `.rune` file:

```rune
println("Hello, Rune!");
println(10 + 5 * 2);
```

Run it with:

```bash
cargo run -- examples/main.rune
```

Rune only accepts source files using the `.rune` extension.

## Project Structure

```text
src/
├── main.rs
├── token.rs
├── lexer.rs
├── ast.rs
├── parser.rs
└── interpreter.rs
```

## Roadmap

Planned features include:

- Variables and assignment
- Booleans
- Comparison operators
- `if` / `else`
- Loops
- Blocks
- Functions
- Semantic analysis
- Intermediate representation
- Optimization
- Native code generation

## Status

Rune is in the early stages of development.

The current focus is building a solid language frontend and defining its semantics before moving toward native compilation.

## Contributing

Issues, ideas, discussions, and contributions are welcome while Rune continues to evolve.

## License

Rune is licensed under the MIT License.