# Proof of Work

A simple command-line tool for generating and solving proof-of-work challenges.

## Installation

```bash
cargo build --release
```

## Usage

Generate a challenge.

```bash
pow generate <difficulty>
```

Solve a challenge.

```bash
pow solve <challenge>
```

Check a solution.

```bash
pow check <challenge> <solution>
```

Run command after solving.

```bash
pow run <difficulty> -- <command>
```

## License

Copyright (c) Cnily03 All rights reserved.

Licensed under the [MIT License](LICENSE).
