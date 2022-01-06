wasm-pack build --target no-modules --out-name index --out-dir ./pkg --no-typescript --release || exit /b
cd %~dp0\static\
copy * %~dp0\pkg
cd %~dp0\pkg
C:\Python310\python.exe -m http.server 8080