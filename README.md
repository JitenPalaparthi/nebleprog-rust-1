# Rust 

### Create a crate 

```cargo new hello```

```cargo new 1-hello --name hello```

### Create a library crate

```cargo new hello_lib --lib```

### Published crates

https://crates.io/


### target compilation in rust

- target triplet

<arch>-<vendor>-<os>[-<env>]

aarch64-apple-darwin
aarch64-pc-windows-msvc 

```rustc --print target-list```

Go to website --> https://doc.rust-lang.org/nightly/rustc/platform-support.html
For Golang this is the website: https://tinygo.org/


### To add a crate 

```cargo add heapless```
```cargo add panic-halt```
```cargo add corted-m-rtic```
```cargo add corted-x```

### For concurrency in embedded

https://crates.io/crates/cortex-m-rtic


### publish to crate.io

```cargo build```
```cargo login```
```cargo publish```