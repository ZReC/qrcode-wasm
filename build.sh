#!/bin/bash

wasm-pack build --release --no-typescript --target web
cd pkg
rm .gitignore
mv qrcode_wasm.js qr.js