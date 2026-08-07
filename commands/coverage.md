# Code Coverage Report

Run code coverage analysis and report gaps. CIM requires ≥ 80% coverage. ValueObjects, state machines, and law tests must be 100%.

## Usage

`/coverage [target]`

Where `[target]` is optional: `lib` (default), `all`, or a specific module path.

## What It Does

1. Run `cargo tarpaulin --lib --out json` (or `cargo llvm-cov` if tarpaulin unavailable)
2. Parse coverage results
3. Report overall percentage
4. Identify uncovered lines by module
5. Flag modules below 80%
6. Flag ValueObjects, state machines, or law tests below 100%
7. Suggest what tests to write for uncovered code

## Commands

```bash
# Library coverage
cargo tarpaulin --lib --skip-clean

# Full coverage with HTML report
cargo tarpaulin --all-features --out Html

# LLVM coverage
cargo llvm-cov --html

# Specific module
cargo tarpaulin --lib --skip-clean -- --test-threads=1
```

## Report Format

```
Coverage Report
═══════════════
Overall: 85.2% (target: ≥ 80%) ✅

Module Coverage:
  src/value_objects/money.rs      100.0% ✅
  src/value_objects/interest_rate.rs  100.0% ✅
  src/aggregate/lead.rs            92.3% ✅
  src/aggregate/borrower.rs        88.1% ✅
  src/saga/deal_file.rs            75.4% ❌ BELOW 80%

Uncovered Lines:
  src/saga/deal_file.rs:142 — compensation path not tested
  src/saga/deal_file.rs:158 — timeout handling not tested

Required Tests:
  1. DealFile compensation scenario (line 142)
  2. DealFile timeout scenario (line 158)
```
