#!/bin/bash
set -euo pipefail

# Determine configuration
if [ "$TRAVIS_OS_NAME" == osx ]; then
  FOREIGN_TARGET=i686-apple-darwin
else
  FOREIGN_TARGET=i686-unknown-linux-gnu
fi
export CARGO_EXTRA_FLAGS="--all-features"
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======
export RUSTC_EXTRA_FLAGS="-D warnings"
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

# Prepare
echo "Build and install miri"
./miri build --all-targets
./miri install
echo

# Test
function run_tests {
    ./miri test
    # "miri test" has built the sysroot for us, now this should pass without
    # any interactive questions.
    test-cargo-miri/run-test.py
}

echo "Test host architecture"
run_tests
echo

echo "Test foreign architecture ($FOREIGN_TARGET)"
MIRI_TEST_TARGET="$FOREIGN_TARGET" run_tests
echo
