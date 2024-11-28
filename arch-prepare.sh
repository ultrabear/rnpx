#!/bin/bash

set -euo pipefail

cargo b --release --locked
cp ./target/release/rnpx ./arch-pkg/rnpx
