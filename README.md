# 🦀 My Rust Learning Journey

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Status](https://img.shields.io/badge/status-active-success.svg?style=for-the-badge) ![GitHub license](https://img.shields.io/github/license/Slozzyondul/rust-projects-)

Welcome to my **Rust Learning Journey** repository! This project serves as a central hub for my experiments, notes, and mini-projects as I master the Rust programming language.

## 📚 About This Repository

The goal of this repository is to document my progress from "Hello World" to building complex applications. It is structured to follow my personal learning path, including side projects and algorithm challenges.

## 🚀 Getting Started

To get a local copy up and running, follow these simple steps.

### 1. Prerequisites

You need to have the Rust toolchain installed on your machine. If you don't have it, you can install it via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone the Repository

```bash
git clone https://github.com/Slozzyondul/rust-projects-.git
cd rust-projects-
```

## 📂 Project Structure

Currently, the main focus is on the `learning_project` directory, which contains various exercises:

- `learning_project/`
  - `src/main.rs`: Entry point with basic examples (variables, mutability, shadowing).
  - `src/bin/`: A collection of standalone programs covering specific Rust features:
    - `arrays.rs`: Working with fixed-size arrays.
    - `enum.rs`: Enums and pattern matching.
    - `print.rs`: Formatting and printing to the console.
    - `scalar_values.rs`: Core primitive types (integers, floats, bools, chars).
    - `scalar_extras.rs`: Further exploration of scalar types and their properties.
    - `string.rs`: String manipulation (String vs &str).
    - `tuples.rs`: Working with tuples.
    - `struct.rs`: Custom data structures.

## 🛠️ Usage

Navigate to the project directory:

```bash
cd learning_project
```

### Run the Main Program
```bash
cargo run
```

### Run a Specific Binary/Exercise
You can run any of the files in `src/bin` using the `--bin` flag followed by the filename (without extension):

```bash
# Example: running the enums exercise
cargo run --bin enum

# Example: running the strings exercise
cargo run --bin string
```

### Check for Errors
```bash
cargo check
```

## 🛣️ Roadmap

- [x] Basic Syntax & Data Types
- [x] Ownership & Borrowing
- [ ] Error Handling
- [ ] Generics & Traits
- [ ] Concurrent Programming

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
