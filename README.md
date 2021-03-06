# Payments Engine in Rust

## Efficiency

Size of `amount` is not specified. This code opts for `u64` to allow for large amounts. (It's also assumed that `amount` and all balances fit `u64`.)

## Safety

### Account

The `Account` struct was made largely mutable. This is because an immutable `Account` would require allocting memory for an `Account` per each transaction which would be time-consuming.

## Correctness

Typing is used for correctness as far as possible. One instance to mention is `Transaction.amount`.
It might have been an `Option` as not all transactions have an `amount`.
However, for `deposit` and `withdraw` having 0 amount is a no-op in this implementation so we interpret omitted amount as 0.

Several places have unit tests, most notably `Engine`. It would be nice to also have a few E2E tests that feed a CSV to the program.
As this code is to be used with large datasets it would be important to test performance of the code also.

## Maintainability

The code is well maintainable. One thing to improve could be having both the transaction and its dipute status in a single struct in `Engine`. Now these are in two maps accessed using transaction ID, often one after another.
