# FileSentry

FileSentry is a fast open-source Rust CLI for file integrity checking, directory hashing, duplicate detection, and manifest verification. It streams SHA-256/SHA-512 checksums, identifies confirmed duplicates, and detects changed or missing files without modifying scanned data.

Created by arjunrenvon — GitHub: https://github.com/Arjunren

## Features

- Streamed SHA-256 and SHA-512 hashing for individual files and directory scans
- Size-then-hash duplicate detection with recoverable-space reporting; never deletes files
- JSON integrity manifests, verification exit codes, hidden-file and ignore controls

## Screenshots

This is a terminal application; command examples below show its output model.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
filesentry hash archive.zip --algorithm sha256
filesentry scan ./src --json
filesentry duplicates ./downloads --ignore node_modules
filesentry manifest ./project --output manifest.json
filesentry verify manifest.json
```

Exit codes: `0` passed, `1` verification differences, `2` invalid input or command error.

## Development

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

## Project Structure

`cli.rs` defines commands, `hashing.rs` streams file data, `scanner.rs` walks safely, `duplicates.rs` confirms duplicate hashes, and `manifest.rs`/`verify.rs` manage integrity checks.

## Security / Privacy

FileSentry never executes, deletes, or mutates scanned files. The only write operation is an explicitly requested manifest output.

## Contributing

Add tests for changed filesystem behavior and avoid logging private file contents.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Author

**arjunrenvon**

GitHub: [@Arjunren](https://github.com/Arjunren)
