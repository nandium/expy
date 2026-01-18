# Expy

An Excel formula to Python code transpiler, written in Rust and compiled to WebAssembly for client-side browser execution. We have integrated this into our interactive webpage (WIP).

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

> **Note:** This project is currently under active development. The API and feature set may change.
> 
> It is currently not in a working state, but is actively being developed. Subscribe to see updates

<details open>

<summary><strong>Table of Contents</strong></summary>

- [Expy](#expy)
  - [Overview](#overview)
    - [Features](#features)
    - [Supported Excel Functions](#supported-excel-functions)
  - [Installation](#installation)
    - [Pre-built Binaries](#pre-built-binaries)
    - [From Source](#from-source)
  - [Usage](#usage)
    - [Browser](#browser)
    - [As a Rust Library](#as-a-rust-library)
  - [Development](#development)
    - [Requirements](#requirements)
    - [Building](#building)
    - [Testing](#testing)
    - [Build Targets](#build-targets)
  - [License](#license)

</details>

## Overview

Expy parses Microsoft Excel formulas and transpiles them into equivalent Python code. It runs entirely in the browser via WebAssembly, and powers our web demo.

### Features

- LALR(1) parser for Excel formula syntax
- Runs client-side in the browser (no server required)

### Supported Excel Functions

WIP

## Installation

### Pre-built Binaries

Download the latest release from the [Releases](https://github.com/nandium/expy/releases) page.

### From Source

See the [Development](#development) section below.

## Usage

### Browser

Include the generated WebAssembly module in your web application:

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Expy Demo</title>
</head>
<body>
    <script type="module">
        import init, { transpile } from './pkg/expy.js';

        async function run() {
            await init();
            const python = transpile("=SUM(A1:A10) + IF(B1 > 0, C1, D1)");
            console.log(python);
        }

        run();
    </script>
</body>
</html>
```

### As a Rust Library

Add Expy to your `Cargo.toml`:

```toml
[dependencies]
expy = { git = "https://github.com/nandium/expy" }
```

```rust
use expy::transpile;

fn main() {
    let excel_formula = "=SUM(A1:A10)";
    let python_code = transpile(excel_formula).unwrap();
    println!("{}", python_code);
}
```

## Development

### Requirements

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) for WebAssembly builds
- [Node.js](https://nodejs.org/) (optional, for running WASM tests)
- [Python 3](https://www.python.org/) (optional, for local development server)

The required Rust components and targets are specified in `rust-toolchain.toml` and will be installed automatically when you run any Cargo command.

### Building

Clone the repository:

```bash
git clone https://github.com/nandium/expy.git
cd expy
```

Install `wasm-pack` (if not already installed):

```bash
cargo install wasm-pack
```

Build the WebAssembly package:

```bash
wasm-pack build --target web --release
```

Serve locally and open `http://localhost:8080` in your browser:

```bash
python3 -m http.server 8080
```

### Testing

Run native Rust tests:

```bash
cargo test
```

Run WebAssembly tests in a headless browser:

```bash
wasm-pack test --headless --firefox
```

Or using Node.js:

```bash
wasm-pack test --node
```

### Build Targets

| Target      | Command                               | Output    | Use Case                     |
|-------------|---------------------------------------|-----------|------------------------------|
| `web`       | `wasm-pack build --target web`        | `pkg/`    | Native ES modules            |
| `bundler`   | `wasm-pack build --target bundler`    | `pkg/`    | Webpack, Rollup, Parcel      |
| `nodejs`    | `wasm-pack build --target nodejs`     | `pkg/`    | Node.js (CommonJS)           |

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
