# Rune

> **Version:** 0.0.2  
> **Status:** Early Development  
> **Implementation Language:** Rust  
> **Compilation:** Native / AOT

Rune is an experimental compiled programming language built from scratch with a focus on learning compiler design, native code generation, and programming language architecture.

The compiler itself is written primarily in **Rust** and uses **Cranelift** as its native code-generation backend. Rune also includes a small runtime written in **x86-64 Assembly**.

Rune source files use the `.rune` extension.

---

## 🚀 Features

Rune is currently in a very early stage of development.

### println

Rune provides the native `__println` instruction, and have a line break:

```rune
__println("Hello, world!");
```

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