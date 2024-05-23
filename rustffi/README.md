### Compile C files



1. make sure you have tool chain to compile either gcc or clang
2. on all unix based machines "lib" is prefixed to the actual library.Means libmylib.dylib on mac is nothing but mylib.dylib. 
3. libmylib.so on unix is nothig but mylib.so
    - generate shared library using 
```
gcc -shared -o libmylib.dylib -fPIC mylib.c
or 
clang -shared -o mylib.dylib -fPIC mylib.c
``` 
4. Make sure the .dylib or .so or .dll(windows) in the root directory of your rust application. If not set the below 
    - Library Path Environment Variable:
    - If the shared library is not in the same directory, you might need to add its location to the DYLD_LIBRARY_PATH environment variable

    ```export DYLD_LIBRARY_PATH=/path/to/library:$DYLD_LIBRARY_PATH```
5. create build.rs file in rust root directory. Follow the file in this directory for more details.
6. link and write a rapper as below either in main.rs if a binary create or in a lib.rs if a library crate.

```
// src/main.rs
#[link(name = "mylib")]
extern "C" {
    fn hello_from_c();
    fn add(a: i32, b: i32) -> i32;
}
```

7. call it as below . External linked library calls are always unsafe in rust

```
fn main() {
    unsafe {
        hello_from_c();
        let result = add(5, 3);
        println!("5 + 3 = {}", result);
    }
}
```