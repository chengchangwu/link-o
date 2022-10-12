# Link a C .o file before a shared .so in Rust

My Rust main function depends on a C object file foo.o, which depends on a C shared library libbar.so.

I need to link foo.o before libbar.so in Rust,

I tried rust-link-lib=foo. It only works for libraries that begin with lib and end with .so or .a.

I tried rust-link-arg=foo.o and rust-link-lib=bar. It puts foo.o after -lbar, which results in an undefined reference error.

Compile the C .o and .so files,

```
gcc -c foo.c
gcc -fPIC -shared -o libbar.so bar.c
```

Linking in gcc succeeds,

```
gcc main.c -o main foo.o -L./ -lbar
LD_LIBRARY_PATH=./ ./main
```

Linking in rust,

```
cargo run
```

Failed with message,

```
 = note: /usr/bin/ld: foo.o: in function `foo':
          foo.c:(.text+0x10): undefined reference to `bar'
          collect2: error: ld returned 1 exit status
```
