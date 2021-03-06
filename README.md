# Payments Engine in Rust

## Efficiency

Size of `amount` is not specified. This code opts for `u64` to allow for large amounts. (It's also assumed that `amount` and all balances fit `u64`.)

## Safety

### Account

The `Account` struct was made largely mutable. This is because an immutable `Account` would require allocting memory for an `Account` per each transaction which would be time-consuming.

One way to mitigate the risk of accounts being illegally mutated would be to have a another `Account` struct that's immutable and only expose that outside of the `Engine` module.
