// const wasm2js = require("wasm2js")
const fs = require("fs")

/* to base64 data */

const wasmBuffer  = fs.readFileSync("dist/hacash_sdk.wasm");
const jscon = wasm2js(wasmBuffer); // to js content
fs.writeFile("dist/hacash_sdk_wasm.js", jscon, (err) => {}); // to file


/* simple wasm2js */

function wasm2js(buf) {
    
    let b64str = Buffer.from(buf).toString('base64');

    return `

var hacash_sdk_wasm_buffer = toUint8Array("${b64str}");

function toUint8Array (s) {
    if (typeof atob === 'function') return new Uint8Array(atob(s).split('').map(charCodeAt))
    return (require('buffer').Buffer).from(s, 'base64')
}

function charCodeAt (c) {
    return c.charCodeAt(0)
}

`
  
}
