#!/usr/bin/env bash
echo If the compilation fails with error 8, it\'s a bug in wasm-pack
echo https://github.com/rustwasm/wasm-pack/issues/974
echo Workaround: open req_anim_frame/Cargo.toml, disable the optimization step
echo and run it manually like:
echo wasm-opt pkg/req_anim_frame.wasm -o ../demo_server/web_app/req_anim_frame.wasm_bg.wasm -O4 -all -ffm --enable-simd                                                         
echo Cleaning up previous builds...
rm demo_server/web_app/req_anim_frame.wasm_bg.wasm demo_server/web_app/req_anim_frame.js
echo Compiling web application
cd web_app
wasm-pack build --target web --out-name req_anim_frame.wasm -- --features web --no-default-features || exit 1
echo Copying the compiled wasm to our demo server
cp pkg/req_anim_frame.js ../demo_server/web_app/
cp pkg/req_anim_frame.wasm ../demo_server/web_app/req_anim_frame.wasm_bg.wasm
cd -
echo
echo %%%%%%%% COMPILATION DONE. OPENING DEV SERVER: %%%%%%%
python3 demo_server/server.py
