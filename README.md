# Rune

> **Version:** 0.0.3-alpha.1
> **Status:** Early Development
> **Implementation Language:** Rust
> **Compilation:** Native / AOT

Rune is an experimental compiled programming language built from scratch with a focus on learning compiler design, native code generation, and programming language architecture.

The compiler itself is written primarily in **Rust** and uses **Cranelift** as its native code-generation backend. Rune also includes a small runtime written in **x86-64 Assembly**.

Rune source files use the `.rune` extension.

---

## 🚀 Features

Rune is currently in a very early stage of development.

### Variables

Rune provides the native variables:

`let` for imutable variables.

`var` for mutable variables.

### Type supported: String

`String` is the unique type Rune support now.

## 📦 Installation

### Requirements

Before building Rune, make sure the following tools are installed:

- Rust and Cargo
- NASM
- GNU linker (`ld`)
- Git

### Build from source

Clone the repository:

```bash
git clone https://github.com/MateusSoaresL/rune.git
cd rune
```
