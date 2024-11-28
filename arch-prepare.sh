#!/bin/bash

set -euo pipefail

cargo b --release
cp ./target/release/rnpx ./arch-pkg/rnpx
