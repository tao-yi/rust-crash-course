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

## 3.Common Programming Concept

### 3.1 Variables and Mutability

> Variables are immutable by default.

#### Constants

like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are few differences between constants and variables.

1. constant 是不可变得，无法使用 `mut` 关键字
2. constant can be declared in any scope, including the global scope.
3. constant cannot be set to the result of a value that could only be computed at runtime

```rs
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

#### Shadowing

```rs
let x = 5;
// shadowing
let x = x + 1;
{
  let x = x * 2;
  println("inner scope {}", x);
}
println("outer scope {}", x);
```

> Shadowing is different from marking a variable as `mut`, because we'll get a compile-timer error if we accidentally try to reassign to this variable without using the `let` keyword.

1. By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

2. When we use the `let` keyword again, we are effectively creating a new variable, we can change the type of the value but reuse the same name.

```rs
let spaces = "  ";
let spaces = spaces.len();
```

#### Data Types

- Scalar Types
  - integer
  - float
  - boolean
  - character
- Compound Types
  - tuple
  - array

#### functions

function by default returns a tuple `fn a() -> ()`

#### statements and expressions

- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resulting value.

```rs
// statement do not return a value
let y = 6;

// let x = (let y = 6); will throw error

// expression evaluate to a value
let y = {
  let x = 3;
  x + 1 // no semicolon at the end
} // evaluate to value 4


// expressions do not include ending semicolons
// function with return value
fn five() -> i32 {
  5 // this is an expression
}

fn empty() {
  1; // this is a statement
}
```

- math operation `5 + 6` is expression that evaluates to the value 11
- calling a function is an expression
- calling a macro is an expression

#### control flow

```rs
let number = 3;

// error!
// if number {
//     println!("number was three");
// }
```

> Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-boolean types to a Boolean.

**Use `if` in a `let` statement**

Because `if` is an expression, we can use it on the right side of a `let` statement to assign the outcome to a variable

```rs
let condition = true;
let number = if condition { 5 } else { 6 };
```

#### loops

```rs
fn nested_loop() {
    let mut count = 0;
    // the outer label loop has label 'counting_up
    'counting_up: loop {
        println!("count={}", count);
        let mut remaining = 10;
        loop {
            println!("remaining={}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}
```

The outer loop has the label `'counting_up`

One of the uses of a `loop` is to retry an operation you know might fail, such as checking whether a thread has completed its job. You might need to pass the result of that operation out of the loop to the rest of your code.

```rs
let mut counter = 0;
let result = loop {
  counter+=1;
  if counter == 10 {
    break counter * 2; // break and return value
  }
}
```

You can use a `for` loop and execute some code for each item in a collection.

```rs
let a = [10, 20, 30, 40, 50];
for element in a {
    println!("the value is {}", element);
}

for (i, &num) in a.iter().enumerate() {
    println!("a[{}]={}", i, num);
}

// (1..4) is a `Range`, it generates [1,2,3]
for number in (1..4).rev() {
  println!("{}!", number)
}
```
