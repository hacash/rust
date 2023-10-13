## settings
LIBNAME=hacash_sdk
LIBNAMEBG=hacash_sdk_bg
DIRNAME="./pkg/$LIBNAMEBG"
FILNAME="./pkg/$LIBNAME"

wasm-pack --log-level error build --target no-modules # or web

## reduce size (remove panic exception handling, etc.)
wasm-snip --snip-rust-fmt-code \
          --snip-rust-panicking-code \
          -o $DIRNAME.wasm $DIRNAME.wasm

# ## reduce size (remove all debugging information)
# wasm-strip $DIRNAME.wasm

# ## Further reduce size
# wasm-opt -o $DIRNAME.wasm -Oz $DIRNAME.wasm

## zip
cp $DIRNAME.wasm $FILNAME.wasm
rm -f $FILNAME.zip && zip -j -9 $FILNAME.zip $FILNAME.wasm
zip -j -9 $FILNAME"_wasm".zip $FILNAME"_wasm".js

## wasm to js
node ./script/park_sdk.js -web

## View final size
ls -lh $FILNAME.wasm
ls -lh $FILNAME.zip
ls -lh $FILNAME"_wasm".js
ls -lh $FILNAME"_wasm".zip