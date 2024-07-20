# Rust Interpreter

## Overview

This project is an interpreter written in Rust for a simple programming language. It includes components for lexical analysis (tokenizing), parsing, and evaluating expressions and statements. The interpreter is designed to be extensible and modular, making it a suitable foundation for learning and experimenting with compiler and interpreter design.

## Features

- **Lexer**: Converts raw source code into a stream of tokens.
- **Parser**: Parses tokens into an Abstract Syntax Tree (AST) for further processing.
- **Evaluator**: Evaluates expressions and statements based on the AST.
- **REPL**: Provides a Read-Eval-Print Loop for interactive use.
- **Custom Error Handling**: Utilizes the `anyhow` crate for improved error reporting.

## Installation

To build and run the project, you need to have Rust installed on your system. You can install Rust from [rust-lang.org](https://www.rust-lang.org/).

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/rust-interpreter.git
   cd rust-interpreter
