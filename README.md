# rust-ocpp

[![crates.io](https://img.shields.io/crates/v/rust-ocpp.svg)](https://crates.io/crates/rust-ocpp)
[![workflow](https://img.shields.io/github/actions/workflow/status/codelabsab/rust-ocpp/rust.yml)](https://github.com/codelabsab/rust-ocpp/actions)
[![codecov](https://codecov.io/gh/codelabsab/rust-ocpp/branch/main/graph/badge.svg?token=23C458RC3S)](https://codecov.io/gh/codelabsab/rust-ocpp)

The `rust-ocpp` libs implements the Open Charge Point Protocol
used in charging stations. You can read more on the official [Open Charge Alliance](https://www.openchargealliance.org/) website.

OCPP versions v1.6, v2.0.1, and v2.1 are implemented and validated using the official json schemas from Open Charge Alliance.

You can find the tests in `schema_validation.rs` for all supported versions.

## repo structure

`src/` : library files for v1.6, v2.0.1, and v2.1

`docs/` : official ocpp specification

## How to Use

Add `rust-ocpp` as a dependency in your `Cargo.toml`. Note that there is no default version - you must explicitly specify which OCPP version(s) you want to use via feature flags.

```toml
[dependencies]
rust-ocpp = "3.0"
```

To use a specific version, specify it with a feature flag:

```toml
[dependencies]
rust-ocpp = { version = "2.0", features = ["v1_6"] }  # For OCPP 1.6
rust-ocpp = { version = "2.0", features = ["v2_0_1"] }  # For OCPP 2.0.1
rust-ocpp = { version = "2.0", features = ["v2_1"] }  # For OCPP 2.1
```

You can also use multiple versions:

```toml
[dependencies]
rust-ocpp = { version = "2.0", features = ["v2_0_1", "v2_1"] }
```

## How to Build

To build the `rust-ocpp` library, you need to have Rust and Cargo installed on your system. You can install them by
following the instructions provided at the official [Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust and Cargo installed, you can build the library using the following steps:

1. Clone the `rust-ocpp` repository:

   ```bash
   git clone https://github.com/codelabsab/rust-ocpp.git
   ```

2. Change into the `rust-ocpp` directory:

   ```bash
   cd rust-ocpp
   ```

3. Build the library using Cargo for all versions:

   ```bash
   cargo build --all-features
   ```

   This command will compile the library and its dependencies. If the build is successful, you will find the compiled
   artifacts in the `target/debug` directory.

4. Run the tests on all versions:

   ```bash
   cargo test --all-features

   ```

   This command will execute the tests for all OCPP versions. If all tests pass, it means that the library is
   functioning correctly.

5. Build a specific version:

   To build a specific version of `rust-ocpp`, you can use the appropriate feature flag when running the build command.
   For example, to build `v1_6`:

   ```bash
   cargo build --features v1_6
   ```

   To build `v2_0_1`:

   ```bash
   cargo build --features v2_0_1
   ```

   To build `v2_1`:

   ```bash
   cargo build --features v2_1
   ```

6. (Optional) Build for release:

   If you want to build the library for release, with optimizations enabled, you can use the following command:

   ```bash
   cargo build --features v2_0_1 --release
   ```

   The release build will produce optimized artifacts in the `target/release` directory.

7. (Optional) Install the library:

   If you want to install the library globally on your system, you can use the following command:

   ```bash
   cargo install --path .
   ```

   This command will compile the library and its dependencies and install it in the Cargo binary directory, so you can
   use it as a dependency in other projects.

That's it! You have successfully built the `rust-ocpp` library. If you encounter any issues during the build process,
please check the project's issue tracker on GitHub or open a new issue for assistance.

## Testing

`rust-ocpp` provides testing against json schemas for all supported OCPP versions. To run the tests, you can use
Cargo's built-in test runner.

### Running Tests

To run the tests for a specific version, use the appropriate feature flag:

```bash
cargo test --features v1_6      # For OCPP 1.6 tests
cargo test --features v2_0_1    # For OCPP 2.0.1 tests
cargo test --features v2_1      # For OCPP 2.1 tests
```

To run all tests for all versions:

```bash
cargo test --all-features
```

### Test Coverage

The test coverage for rust-ocpp is measured using Codecov. You can find the current test coverage report
on [codecov](https://codecov.io/gh/codelabsab/rust-ocpp).

### Contributing to Tests

Contributions to the test suite are very much appreciated. If you encounter any bugs, discover edge cases, or have ideas
for
additional test cases, feel free to open an issue or submit a pull request.
We will be happy to review and incorporate your contributions.

Please ensure that you run the tests and maintain or improve the overall test coverage before submitting any changes.
Additionally, adhere to the existing testing conventions and follow the code style guidelines to maintain consistency.

## Contribute

Use `rustfmt` before you PR.

pre-commit config is available. You can read more about it at [pre-commits](https://pre-commit.com) website and checkout their repo on [github](https://github.com/pre-commit/pre-commit)

## Releasing a new version

1. Update the version of the library and push the changes to the main branch.
2. Create a [new release](https://github.com/codelabsab/rust-ocpp/releases/new) on GitHub with the new version number and some release notes (optional).

This will trigger the [publish workflow](./.github/workflows/publish.yml) which will publish the new version to crates.io.
