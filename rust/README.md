# Dev Instructions

## Tools needed

1. Install *cargo-llvm-cov* via `cargo install cargo-llvm-cov`
2. Run `cargo llvm-cov --open` to run tests and open the coverage report in the browser.
3. Install *cargo-mutants* `cargo install --locked cargo-mutants`

## Flow

1. add snapshots `cargo insta test --review`
2. code coverage in browser `cargo llvm-cov --open` or if you want a report `cargo llvm-cov --lcov > lcov.info`
3. mutation testing `cargo mutants`