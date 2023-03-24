**THIS REPO IS NOT MAINTAINED ANYMORE**

If you're willing to contribute please do on [this](https://github.com/lambdaclass/cairo_sierra_2_MLIR) repo

<div align="center">
  <img src="docs/images/logo/logo.png" height="200">
  <br />
  <a href="https://github.com/keep-starknet-strange/shenlong/issues/new?assignees=&labels=bug&template=01_BUG_REPORT.md&title=bug%3A+">Report a Bug</a>
  -
  <a href="https://github.com/keep-starknet-strange/shenlong/issues/new?assignees=&labels=enhancement&template=02_FEATURE_REQUEST.md&title=feat%3A+">Request a Feature</a>
  -
  <a href="https://github.com/keep-starknet-strange/shenlong/discussions">Ask a Question</a>
</div>

<div align="center">
<br />

[![GitHub Workflow Status](https://github.com/keep-starknet-strange/shenlong/actions/workflows/test.yml/badge.svg)](https://github.com/keep-starknet-strange/shenlong/actions/workflows/test.yml)
[![Project license](https://img.shields.io/github/license/keep-starknet-strange/shenlong.svg?style=flat-square)](LICENSE)
[![Pull Requests welcome](https://img.shields.io/badge/PRs-welcome-ff69b4.svg?style=flat-square)](https://github.com/keep-starknet-strange/shenlong/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)

</div>


<details open="open">
<summary>Table of Contents</summary>

- [Report a Bug](#report-a-bug)
- [Request a Feature](#request-a-feature)
- [Report a bug](#report-a-bug-1)
- [Request a feature](#request-a-feature-1)
- [About](#about)
  - [Built With](#built-with)
- [Architecture](#architecture)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
    - [Rust](#rust)
    - [LLVM](#llvm)
  - [Installation](#installation)
    - [shenlongup](#shenlongup)
    - [Environment variables](#environment-variables)
- [Usage](#usage)
  - [Command line interface](#command-line-interface)
    - [Compile Sierra to LLVM IR](#compile-sierra-to-llvm-ir)
- [Performance](#performance)
  - [Benchmarks](#benchmarks)
- [Roadmap](#roadmap)
- [Support](#support)
- [Project assistance](#project-assistance)
- [Contributing](#contributing)
- [Authors \& contributors](#authors--contributors)
- [Security](#security)
- [License](#license)
- [Acknowledgements](#acknowledgements)
- [Contributors ✨](#contributors-)

</details>

---

## Report a bug

If you find a bug, please open a
[GitHub issue](https://github.com/keep-starknet-strange/shenlong/issues/new?assignees=&labels=bug&template=01_BUG_REPORT.md&title=bug%3A+)!

## Request a feature

To request a new feature, please open an issue following
[this template](https://github.com/keep-starknet-strange/shenlong/issues/new?assignees=&labels=enhancement&template=02_FEATURE_REQUEST.md&title=feat%3A+).

## About

> Shenlong is a blazingly fast ⚡ tool to generate LLVM IR from Cairo, written in Rust 🦀

### Built With

- [Rust](https://www.rust-lang.org/)
- [inkwell](https://thedan64.github.io/inkwell/)
- [cairo](https://github.com/starkware-libs/cairo)

## Architecture

```text
               +--------------+
               |   Cairo code |
               +-------+------+
                       | Compile with Cairo compiler
                       |
               +-------v------+
               |   Sierra     |
               +-------+------+
                       | Compile with Shenlong
                       |
               +-------v------+
               |   LLVM IR    |
      +--------+-------+------+---------+
      | lli            |clang           | clang
      |                |                |
+-----v-------+  +-----v-----+  +-------v----+
|    JIT      |  |  Binary   |  |    WASM    |
+-------------+  +-----------+  +------------+
```

## Getting Started

### Prerequisites

#### [Rust](https://www.rust-lang.org/tools/install)

The project requires nightly, which the rust-toolchain.toml file on the repository takes care of.

#### LLVM

In order to build the project, you need to have LLVM 16+ installed on your system.

##### On debian/ubuntu:

You can use the follow repository (make sure to install LLVM 16): <https://apt.llvm.org/>

And then export the following env var:

```bash
export LLVM_SYS_150_PREFIX=/usr/lib/llvm-16
export LLI_PATH=/usr/lib/llvm-16/bin/lli # used for testing
```

##### On macos:

While the oficial brew repo doesn't have LLVM 16 right now, you can use this custom tap:

```
brew install -s edg-l/tap/llvm@16
```

Example of `.zshenv` file:

```bash
export LLVM_SYS_150_PREFIX=$(/opt/homebrew/bin/brew --prefix llvm@16)
export LLI_PATH=$(/opt/homebrew/bin/brew --prefix llvm@16)/bin/lli # used for testing
```

### Installation

#### shenlongup

To install with `shenlongup` run (shenlongup requires nightly rustup):

```bash
curl -sL https://raw.githubusercontent.com/keep-starknet-strange/shenlong/main/shenlongup | sh
```

#### Environment variables

Copy the `.env.example` file to a `.env` file and populate each variable:

```bash
cp examples/.env.example .env
```

Build from source:

```bash
cargo build --all --release
```

The binaries will be located in `target/release/`.
Specifically, the binary for the CLI is `target/release/shenlong`.

## Usage

### Command line interface

```bash
Shenlong, make me a LLVM compiler!
Shenlong is a blazingly fast ⚡ tool to generate LLVM IR from Cairo, written in Rust 🦀

Usage: shenlong [CONFIG] <COMMAND>

Commands:
  sierra  Sierra related subcommands
  help    Print this message or the help of the given subcommand(s)

Arguments:
  [CONFIG]

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

#### Compile Sierra to LLVM IR

```bash
shenlong sierra compile-to-llvm \
--program-path examples/sierra/add.sierra \
--output-path target/llvm/add.ir
```

## Performance

### Benchmarks

Run the benchmarks with:

```bash
cargo bench
```

Result on a MacBook Pro Apple M1 Max 2021 with 64GB of RAM:

```text
     Running benches/sierra_2_llvm_benchmark.rs (target/release/deps/sierra_2_llvm_benchmark-b2798c19234aee25)
Benchmarking sierra-2-llvm-simple-test: Collecting 100 samples in estimated 5.6913 s (10k iterasierra-2-llvm-simple-test
                        time:   [556.84 µs 559.02 µs 562.34 µs]
                        change: [-3.0928% -1.3163% +0.1790%] (p = 0.13 > 0.05)
                        No change in performance detected.
```

This benchmark is the compilation of the `resources/bench/sierra/simple_test.sierra` file.
This file is a simple program that do some additions, here is the Cairo code:

```cairo
func main(a: felt) -> felt  {
    let d = 70;
    let b = a + 1;
    let c = b + d;
    c
}
```

## Roadmap

See the [open issues](https://github.com/keep-starknet-strange/shenlong/issues) for
a list of proposed features (and known issues).

- [Top Feature Requests](https://github.com/keep-starknet-strange/shenlong/issues?q=label%3Aenhancement+is%3Aopen+sort%3Areactions-%2B1-desc)
  (Add your votes using the 👍 reaction)
- [Top Bugs](https://github.com/keep-starknet-strange/shenlong/issues?q=is%3Aissue+is%3Aopen+label%3Abug+sort%3Areactions-%2B1-desc)
  (Add your votes using the 👍 reaction)
- [Newest Bugs](https://github.com/keep-starknet-strange/shenlong/issues?q=is%3Aopen+is%3Aissue+label%3Abug)

## Support

Reach out to the maintainer at one of the following places:

- [GitHub Discussions](https://github.com/keep-starknet-strange/shenlong/discussions)
- Contact options listed on
  [this GitHub profile](https://github.com/keep-starknet-strange)

## Project assistance

If you want to say **thank you** or/and support active development of shenlong:

- Add a [GitHub Star](https://github.com/keep-starknet-strange/shenlong) to the
  project.
- Tweet about the shenlong.
- Write interesting articles about the project on [Dev.to](https://dev.to/),
  [Medium](https://medium.com/) or your personal blog.

Together, we can make shenlong **better**!

## Contributing

First off, thanks for taking the time to contribute! Contributions are what make
the open-source community such an amazing place to learn, inspire, and create.
Any contributions you make will benefit everybody else and are **greatly
appreciated**.

Please read [our contribution guidelines](docs/CONTRIBUTING.md), and thank you
for being involved!

## Authors & contributors

For a full list of all authors and contributors, see
[the contributors page](https://github.com/keep-starknet-strange/shenlong/contributors).

## Security

shenlong follows good practices of security, but 100% security cannot be assured.
shenlong is provided **"as is"** without any **warranty**. Use at your own risk.

_For more information and to report security issues, please refer to our
[security documentation](docs/SECURITY.md)._

## License

This project is licensed under the **MIT license**.

See [LICENSE](LICENSE) for more information.

## Acknowledgements

- Shout out to StarkWare for the amazing Cairo compiler and the Cairo language
  itself, specifically to the compiler team for their help and support and for the primitives on which shenlong is built.
## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/abdelhamidbakhta"><img src="https://avatars.githubusercontent.com/u/45264458?v=4?s=100" width="100px;" alt="Abdel @ StarkWare "/><br /><sub><b>Abdel @ StarkWare </b></sub></a><br /><a href="https://github.com/keep-starknet-strange/shenlong/commits?author=abdelhamidbakhta" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/LucasLvy"><img src="https://avatars.githubusercontent.com/u/70894690?v=4?s=100" width="100px;" alt="Lucas @ StarkWare"/><br /><sub><b>Lucas @ StarkWare</b></sub></a><br /><a href="https://github.com/keep-starknet-strange/shenlong/commits?author=LucasLvy" title="Code">💻</a></td>
    </tr>
  </tbody>
  <tfoot>
    <tr>
      <td align="center" size="13px" colspan="7">
        <img src="https://raw.githubusercontent.com/all-contributors/all-contributors-cli/1b8533af435da9854653492b1327a23a4dbd0a10/assets/logo-small.svg">
          <a href="https://all-contributors.js.org/docs/en/bot/usage">Add your contributions</a>
        </img>
      </td>
    </tr>
  </tfoot>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
