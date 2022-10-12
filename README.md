# Link a C .o file before a shared .so in Rust

Compile the C .o and .so files,

```
gcc -c foo.c
gcc -fPIC -shared -o libbar.so bar.c
```

Linking with gcc succeeds,

```
gcc main.c -o main foo.o -L./ -lbar
LD_LIBRARY_PATH=./ ./main
```

Linking with rust failed.

```
cargo run
```
