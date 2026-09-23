# epoch

A tiny CLI to instantly convert between unix timestamps and human-readable dates in local time or UTC. Fills the gap left by cryptic `date` command flags across different operating systems.

## Status

**built, untested**: the core timezone and parsing logic is built.

## Installation

```sh
cargo install --path .
```

## Usage

```sh
# Print current time
epoch

# Convert timestamp
epoch 1700000000

# Convert back from RFC3339
epoch "2023-11-14T22:13:20Z"
```
