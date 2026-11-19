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

```text
cargo run --locked -- source.apcsp
```

The runner reads one UTF-8 source file, reads input from standard input, and writes `DISPLAY` output to standard output.

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
