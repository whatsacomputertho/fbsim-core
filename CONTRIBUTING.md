# Contributing to fbsim-core

> Thank you for your interest in contributing into `fbsim-core`!  Here are some basic instructions and background information on how to do so.

## Contents

- Tooling setup
- Version control
- Roadmap

## Tooling Setup

> Discusses how to install required project dependencies, and run the project's various make recipes

### Required Dependencies

This project only requires the rust toolchain, which can be installed following these directions:
- https://rustup.rs/

### Make Recipes

You may want to execute the project's make recipes when contributing to the project, such as
- `make build`: To make sure the library compiles without errors or warnings
  - Equivalent to `cargo build`
- `make test`: To make sure the library's unit tests & doc examples compile and run without errors or warnings, and all test assertions pass
  - Equivalent to `cargo test`
- `make sec`: To make sure project dependencies contain no known vulnerabilities
  - Equivalent to `cargo audit`
  - If you do not have `cargo-audit` installed, you can run `make sec-dependencies` to install it, or simply run `cargo install cargo-audit`
- `make lint`: To make sure the library is compliant with rust style standards
  - Equivalent to `cargo clippy`

## Version Control

> [!TIP]
> Any `fbsim-core` contributor should be familiar with the project's version control practices to ensure they contribute into the correct branches of the project, and understand how to include their changes in a project release

The `fbsim-core` project makes use of the following version control strategy.
1. The `main` branch represents the latest developments that are considered potentially shippable
    - The `main` branch MAY incur API breaking changes at any time
    - Releases of `fbsim-core` MUST NOT be produced from the `main` branch
2. For each release, we create a `release-x.y` branch, where `x.y` is the Major.Minor version of the release
    - The `release-x.y` branch MUST NOT incur API breaking changes
    - Any release of `fbsim-core` with Major.Minor version `x.y` MUST be produced from the `release-x.y` branch
        - Including the initial `x.y.0` release, `x.y.z` patch releases, and pre-releases (`x.y.z-prerelease`)
    - Any release of `fbsim-core` MUST be accompanied by a git tag and GitHub release
3. All new development MUST first be contributed into the `main` branch, then cherry-picked (using `git cherry-pick`) into the `release-x.y` branch for its corresponding release
    - Any exception to this rule (such as backports to significantly old versions) MUST be approved by project maintainers

## Roadmap

See [the issues tab of this repository](https://github.com/whatsacomputertho/fbsim-core/issues) for the current set of enhancements we have planned. For now, consider this our public roadmap for `fbsim-core`.

### Reporting bugs

Feel free to [raise a new issue in this repository](https://github.com/whatsacomputertho/fbsim-core/issues/new/choose) to report a bug you encounter with `fbsim-core`, and add the `bug` label. Feel free to ping me (`whatsacomputertho`) if I don't respond to your bug report in a timely manner.
