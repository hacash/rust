## Settings
LIBNAME=hacash_sdk
TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/$LIBNAME.wasm

## Build WASM
RUSTFLAGS="$RUSTFLAGS -A dead_code -A unused_imports -A unused_variables" \
cargo build --target $TARGET --release --lib

## Reduce size (remove panic exception handling, etc.)
wasm-snip --snip-rust-fmt-code \
          --snip-rust-panicking-code \
          -o $BINARY $BINARY

## Reduce size (remove all debugging information)
wasm-strip $BINARY

## Further reduce size
mkdir -p dist
wasm-opt -o dist/$LIBNAME.wasm -Oz $BINARY

## View Function & Bytecode
# wasm-objdump -d dist/$LIBNAME.wasm

## Park sdk to js & zip
rm -f dist/hacash_sdk.zip && zip -j -9 ./dist/hacash_sdk.zip ./dist/hacash_sdk.wasm

## if can npm i wasm2js -g
node ./park_sdk.js

## View final size
ls -lh dist/$LIBNAME.wasm
