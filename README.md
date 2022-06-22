## 1. Getting Started

```sh
# update rust
$ rustup update

# uninstall rust
$ rustup self uninstall

$ rustc --version

# 将main.rs编译成可执行文件 app
$ rustc main.rs -o app

# 新建项目并创建一个目录
$ cargo new hello_cargo

# 在当前文件中创建项目
$ cargo init .

# compile and run
$ cargo run

# check your code make sure it compiles, but does not produt an executable
$ cargo check

# build and store executable in target/debug folder
$ cargo build

# build and store executable in target/release folder, compile with optimization
$ cargo build --release
```

## 2. Programming a Guessing Game

By default, Rust has a few items defined in the standard library that it brings into the scope of every program.

This set is called the **prelude**.

`println!` is a _macro_.

```rs
// new is an associated function of the String type
String::new()
```

An _associated function_ is a function that's implemented on a type.

```rs
// The `&` indicates that this argument is a reference
// which gives you a way to let multiple parts of your code access
// one piece of data without needing to copy that data into memory multiple times
// use `&mut guess` instead of `&guess` to make it mutable
std::io::stdin().read_line(&mut guess).expect("FAILED TO READ LINE");

```

- `read_line` 返回 `io::Result`，这是一个 enum 类型，有 `Ok` 和 `Err` 两个 variants
- An instance of `io::Result` has an `expect` method that you can called
- 如果 `Result` instance is an `Err` value, expect will cause the program to crash and display the message that you passed
- 如果 `Result` instance is an `Ok` value, expect will take the return value that `Ok` is holding and return just that value

```sh
# 会自动安装依赖
$ cargo build

# 会更新到Cargo.lock指定返回的最新版本
$ cargo update

# build documentation provided by all of your dependencies locally
$ cargo doc --open
```

> A `match` expression is made up of `arms`.
> An `arm` consists of a pattern to match against, and the code that should be run if the value given to `match` fits that arm's pattern. Rust takes the value given to `match` and looks through each arm's pattern **in turn**.
