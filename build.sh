#! /bin/bash

clear
cargo check && cargo build && cargo run --color=always --package incrementer --bin incrementer --profile dev -- $1 $2 $3
ls src/
