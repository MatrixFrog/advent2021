#!/bin/bash

set -e

cargo build --release --bin=level14
flamegraph target/release/level14
google-chrome flamegraph.svg &
