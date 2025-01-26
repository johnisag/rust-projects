# Setup Project

## Init Project

```
mkdir state-machine
cd state-machine
cargo init
cargo run
```

## Tooling

### rustfmt - formating
```
rustup toolchain install nightly-x86_64-pc-windows-msvc
rustup component add rustfmt --toolchain nightly
touch rustfmt.toml
```

add content to rustfmt.toml
```
# Basic
edition = "2021"
hard_tabs = true
max_width = 100
use_small_heuristics = "Max"
# Imports
imports_granularity = "Crate"
reorder_imports = true
# Consistency
newline_style = "Unix"
# Misc
chain_width = 80
spaces_around_ranges = false
binop_separator = "Back"
reorder_impl_items = false
match_arm_leading_pipes = "Preserve"
match_arm_blocks = false
match_block_trailing_comma = true
trailing_comma = "Vertical"
trailing_semicolon = false
use_field_init_shorthand = true
# Format comments
comment_width = 100
wrap_comments = true
```
and now run

```
cargo +nightly fmt
```

### Rust Analyzer

Another popular tool in the Rust community is [Rust Analyzer](https://rust-analyzer.github.io/).