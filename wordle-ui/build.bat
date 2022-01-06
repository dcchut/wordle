pushd %~dp0
wasm-pack build --target no-modules --out-name index --out-dir ./pkg --no-typescript --release || (popd && exit /b)
cd %~dp0\static\
copy * %~dp0\pkg
cd %~dp0\pkg
popd
C:\Python310\python.exe -m http.server -d %~dp0\pkg 8080
