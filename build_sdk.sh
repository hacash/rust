## Settings
LIBNAME=hacash_sdk
TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/$LIBNAME.wasm

## Build WASM
cargo build --target $TARGET --release

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

## View final size
ls -lh dist/$LIBNAME.wasm



