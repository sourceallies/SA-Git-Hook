#!/bin/bash

RELEASE_DIR="Release-$(awk '/version/{print $3;exit}' Cargo.toml | tr -d '"')"

echo $RELEASE_DIR

if [ -d "${RELEASE_DIR}" ]
then
  rm -rf "${RELEASE_DIR}"
fi
mkdir "${RELEASE_DIR}"

echo "Compiling Windows"
cd "${RELEASE_DIR}" || exit
cross build --release --target=x86_64-pc-windows-gnu
mkdir "Windows"
cd ".."
cp "target/x86_64-pc-windows-gnu/release/installer.exe" "${RELEASE_DIR}/Windows"
cp "target/x86_64-pc-windows-gnu/release/post-commit.exe" "${RELEASE_DIR}/Windows"


echo "Compiling MacOS"
cd "${RELEASE_DIR}" || exit
cargo build --release
mkdir "MacOS"
cd ".."
cp "target/release/installer" "${RELEASE_DIR}/MacOS"
cp "target/release/post-commit" "${RELEASE_DIR}/MacOS"

zip -r "${RELEASE_DIR}.zip" "${RELEASE_DIR}"

rm -rf "${RELEASE_DIR}"