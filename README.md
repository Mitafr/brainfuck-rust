# Brainfuck Interpreter in Rust

Build current : 
```console
cargo build
```
Build current release : 
```console
cargo build --release
```
Build cross-compile : 
```console
cross build --target $ARCH (exemple: x86_64-pc-windows-gnu)
```
Build cross-compile release : 
```console
cross build --release --target $ARCH ($ARCH exemple: x86_64-pc-windows-gnu)
```
Run :
```console
./target/$TARGET/brainfuck-interpreter samples/mandelbrot.bf ($TARGET exemple: release)
```


Author: MitaFR