# Advent of Code in Rust

## Commands

### Print outputs for each day (currently sequentially)

`cargo run`

### Print both outputs for given day

`cargo run --bin implementation_{day_num}`

### Test all days

`cargo bench`

### Test specific day

`cargo bench benchmark "DAY {day_num} :: PART {A|B}"`

### Generate files for day N

`cargo run --bin day_template -- {day_num}`

The above creates files at:

- `src/solution_{day_num}.rs`
- `src/bin/implementation_{day_num}.rs`

It also modifies the files:

- `main.rs` -> Adds execution of the 

## TODOS
