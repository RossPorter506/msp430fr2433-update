#!/bin/bash
# svd files generated useding msp430_svd:
# cargo run -- msp430fr2433

./svd2rust --nightly -g --target=msp430 -i $1

rm -rf src/

form -i lib.rs -o src/ && rm lib.rs

mv generic.rs src/

cargo fmt
