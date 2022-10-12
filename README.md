```
gcc -c foo.c
gcc -fPIC -shared -o libbar.so bar.c
gcc main.c -o main foo.o -L./ -lbar
LD_LIBRARY_PATH=./ ./main
```
