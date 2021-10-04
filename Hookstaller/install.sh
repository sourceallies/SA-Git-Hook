#!/bin/bash

cargo build --release
CUR_DIR="$(pwd)"
cd target/release/ || exit

./installer

cd "${CUR_DIR}" || exit