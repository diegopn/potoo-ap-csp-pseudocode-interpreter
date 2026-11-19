<p align="center">
  <img src="https://diegopn.github.io/potoo-site/src/assets/images/language-interpreter-icon.svg?v=1c651ca7" alt="Potoo IDE language interpreter icon" width="120" />
</p>

<h1 align="center">AP CSP Pseudocode Interpreter</h1>

<p align="center">A Rust interpreter for the College Board's 2026 AP Computer Science Principles pseudocode reference sheet.</p>

<p align="center">
  <a href="https://github.com/diegopn/potoo-ap-csp-pseudocode-interpreter/releases">Download</a> ·
  <a href="https://diegopn.github.io/potoo-site/manual/ap-csp-pseudocode/">Website</a> ·
  <a href="https://diegopn.github.io/potoo-site/manual/ap-csp-pseudocode/">User manual</a>
</p>

<p align="center">
  <a href="https://www.rust-lang.org/"><img alt="Rust" src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white"></a>
  <a href="https://doc.rust-lang.org/cargo/"><img alt="Cargo" src="https://img.shields.io/badge/Cargo-DEA584?style=for-the-badge&logo=rust&logoColor=white"></a>
  <img alt="Rust library" src="https://img.shields.io/badge/Rust_library-4A86CF?style=for-the-badge">
  <img alt="CLI" src="https://img.shields.io/badge/CLI-6A4C93?style=for-the-badge">
</p>

This Rust package is part of the Potoo IDE ecosystem and is distributed independently from the desktop interface. Integration with the IDE has not been implemented or verified. AP CSP pseudocode is an assessment notation, not a general-purpose programming language.

## Scope

The interpreter supports assignments, numeric and Boolean expressions, strings, lists, selection, repetition, procedures and returns, display/input, random integer generation, and the sheet's robot commands. Lists use one-based indices and assignment copies list values. Invalid list indices produce a runtime error. A blocked robot move leaves the robot in place and terminates execution.

See [reference coverage](REFERENCE-COVERAGE.md) for the construct-by-construct mapping and behaviors the exam sheet leaves unspecified.

Robot behavior is provided by the embedding host through `RobotGrid`; the command-line runner does not define an exam grid. The CLI reports a clear runtime error if a program uses a robot command without a configured grid.

## Use the library

Call `parse(source)` and `check(program)`, then pass the checked program to `Interpreter::new(host).run(&program)`. Implement `Host` for input, display, random values, and optionally `robot()` for a `RobotGrid` implementation. `Execution` returns the final global variables, statement count, and whether robot movement terminated the program.

## Command line

Save this program as `example.apcsp`:

```text
DISPLAY("Hello, AP CSP!")
```

Build or install the CLI from a source checkout with Rust 1.93 or later:

```sh
cargo build --release --locked --bin potoo-ap-csp
cargo install --path . --locked --bin potoo-ap-csp
```

Run a source file with the installed command or directly from the checkout:

```sh
potoo-ap-csp example.apcsp
cargo run --locked --bin potoo-ap-csp -- example.apcsp
```

The example displays `Hello, AP CSP!`. The runner reads one UTF-8 source file, reads input from standard input, and writes `DISPLAY` output to standard output.

## Install a prebuilt CLI

The release workflow is configured to publish native v0.1.0 packages for Linux x86_64 GNU, Windows x86_64 MSVC, Intel macOS x86_64, and Apple silicon macOS arm64. After publication, download your system's archive and `SHA256SUMS` from the [GitHub Releases page](https://github.com/diegopn/potoo-ap-csp-pseudocode-interpreter/releases). Each archive contains `potoo-ap-csp` (or `potoo-ap-csp.exe` on Windows) and the repository license. Run the commands below from the folder containing both downloads.

### Linux x86_64 GNU

```sh
set -euo pipefail
archive="potoo-ap-csp-pseudocode-interpreter-v0.1.0-x86_64-unknown-linux-gnu.tar.gz"
grep -F "  $archive" SHA256SUMS | sha256sum --check -
tar -xzf "$archive"
mkdir -p "$HOME/.local/bin"
install -m 755 "${archive%.tar.gz}/potoo-ap-csp" "$HOME/.local/bin/potoo-ap-csp"
export PATH="$HOME/.local/bin:$PATH"
potoo-ap-csp example.apcsp
```

The Linux build is produced on Ubuntu 22.04 and may require a sufficiently recent glibc. Add the `export PATH="$HOME/.local/bin:$PATH"` line to your shell profile to keep the command available in future sessions.

### Windows x86_64 MSVC (PowerShell)

```powershell
$archive = "potoo-ap-csp-pseudocode-interpreter-v0.1.0-x86_64-pc-windows-msvc.zip"
$line = Get-Content .\SHA256SUMS | Where-Object { $_.EndsWith("  $archive") } | Select-Object -First 1
if (-not $line) { throw "No SHA256SUMS entry found for $archive" }
$expected = ($line -split '\s+')[0]
$actual = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
if ($actual -ne $expected) { throw "SHA-256 mismatch for $archive" }
$installDir = Join-Path $env:USERPROFILE ".local\bin"
New-Item -ItemType Directory -Force $installDir | Out-Null
Expand-Archive -LiteralPath $archive -DestinationPath $installDir
$exe = Join-Path $installDir "potoo-ap-csp-pseudocode-interpreter-v0.1.0-x86_64-pc-windows-msvc\potoo-ap-csp.exe"
& $exe .\example.apcsp
```

To run `potoo-ap-csp.exe` from any folder, add `$env:USERPROFILE\.local\bin\potoo-ap-csp-pseudocode-interpreter-v0.1.0-x86_64-pc-windows-msvc` to your user `Path` environment variable in Windows Settings.

### Intel macOS x86_64

```sh
set -euo pipefail
archive="potoo-ap-csp-pseudocode-interpreter-v0.1.0-x86_64-apple-darwin.tar.gz"
grep -F "  $archive" SHA256SUMS | shasum -a 256 -c -
tar -xzf "$archive"
mkdir -p "$HOME/.local/bin"
install -m 755 "${archive%.tar.gz}/potoo-ap-csp" "$HOME/.local/bin/potoo-ap-csp"
export PATH="$HOME/.local/bin:$PATH"
potoo-ap-csp example.apcsp
```

### Apple silicon macOS arm64

```sh
set -euo pipefail
archive="potoo-ap-csp-pseudocode-interpreter-v0.1.0-aarch64-apple-darwin.tar.gz"
grep -F "  $archive" SHA256SUMS | shasum -a 256 -c -
tar -xzf "$archive"
mkdir -p "$HOME/.local/bin"
install -m 755 "${archive%.tar.gz}/potoo-ap-csp" "$HOME/.local/bin/potoo-ap-csp"
export PATH="$HOME/.local/bin:$PATH"
potoo-ap-csp example.apcsp
```

On macOS, add the `export PATH="$HOME/.local/bin:$PATH"` line to `~/.zprofile` to keep the command available in future sessions.

## Reference

- [College Board AP CSP 2026 Exam Reference Information](https://apcentral.collegeboard.org/media/pdf/ap-computer-science-principles-exam-reference-sheet.pdf)
- [Potoo AP CSP pseudocode manual](https://diegopn.github.io/potoo-site/manual/ap-csp-pseudocode/)

## Support

<p align="left">
  <a href="https://github.com/sponsors/diegopn"><img alt="Become a sponsor" src="https://img.shields.io/badge/Become_a_sponsor-EA4AAA?style=for-the-badge&logo=githubsponsors&logoColor=white"></a>
</p>

## Project policies

- [Code of conduct](CODE_OF_CONDUCT.md)
- [Contributing](CONTRIBUTING.md)
- [Security policy](SECURITY.md)
- [GPL-3.0-or-later license](LICENSE)
