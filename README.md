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

## Ownership

In Rust, memory is managed through a system of ownership with a set of rules that the compiler checks.

> The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just `allocating` (pushing values onto the stack is not considered allocating).

> Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.

> Pushing to the stack is faster than allocating on the heap because _the allocator never has to search for a place to store new data_; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work, because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

**Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there.**

#### Ownership rules

- Each value in Rust has a variable that’s called its `owner`.
- There can only be `one owner at a time`.
- When the owner goes out of scope, the value will be dropped.

```rust
fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
}
```

### The `String` Type

- `string literals`: where a string value is hardcoded into our program
  - they are immutable
  - they need to be known at compile time

You can create a `String` from a string literal using the from function, like so:

```rs
let s: &str = "hello"; // string slice
let s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String

```

With the `String` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the `heap`, unknown at compile time, to hold the contents.

- The memory must be requested from the memory allocator at runtime.
- We need a way of returning this memory to the allocator when we’re done with our String.

当我们调用 `String::from` 时就会在 heap 中申请一段内存，然而回收内存时，rust 采用了和 GC 不同得方案:

```rs
fn main() {
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
}

```

当 `s` goes out of scope 时，Rust 会自动调用 `drop` 函数来回收内存。

```rs
let x = 5; // bind 5 to x          => x owns 5
let y = x; // copy 5 and bind to y => y owns copy of 5
```

Becausse integers are simple values with a known, fixed size, so these two 5 values are pushed onto the stack.

然而，对于 heap 中的数据的处理方式却不同

```rs
let s1 = String::from("hello");
let s2 = s1;
```

A `String` is made up of three parts:

1. `ptr`: a pointer to the memory that holds the contents of the string
2. `len`: how much memory in bytes that the `String` is currently using
3. `capacity`: how much memory in bytes that `String` has received from the allocator

**This group of data is stored on the stack**

To ensure memory safety, after the line `let s2 = s1;`, Rust consiter `s1` no longer valid. Therefore, Rust doesn't need to free anything when `s1` goes out of scope.

#### Move

This is known as a **move**. We say that `s1` was moved into `s2`.

```rs
let s1 = String::from("hello");
let s2 = s1.clone(); // s2 owns a clone of s1
println!("{} {}", s1, s2);
```

### Stack-only data: Copy

```rs
let x = 5;
let y = x; // 这里不会发生move，x is still valid
```

> The reason is that types **such as integers that have a known size at compile time are stored entirely on the stack**, so copies of the actual values are quick to make.

> That means there's no reason we would want to prevent x from being valid after we create the variable y.

Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the stack like integers are.

- If a type implements the `Copy` trait, a variable is still valid after assignment to another variable.
- Rust won't let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait.

What types implement the `Copy` trait?

- All integer types
- Boolean type
- All floating types
- Character type
- Tuples, if they only contain types that also implement `Copy`. 比如 `(i32, char)`
  - 但是 `(i32, String)` 不行

### Returning values can tranfer ownership

```rs
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

```

The ownership of a variable follows the same pattern every time:

1. assigning a value to another variable `moves` it.
2. when a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable.

### References and Borrowing

```rs
fn calculate_length(s: &String) -> usize {
    s.len()
}

// main
let s1 = String::from("hello");
let len = calculate_length(&s1);
println!("The length of '{}' is {}.", s1, len);
```

- `&s1` let use create a reference that _refers_ to the value of `s1` but does not own it (意味着 `x = &s1` 当 `x` 离开作用域时不会调用 drop 进行回收 s1 的 value)
- `*s` means deference

#### mutable reference

```rs
let mut s = String::from("hello");
// create a mutable reference to `s`
change(&mut s);

// borrow a mutable value
fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
```

```rs
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; // 报错！不可以同时有两个 mutable reference
```

防止脏写：Prevent two or more pointers access the same data at the same time.

在不同的 scope 中是可以的，因为互不影响

```rs
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

Rust 对于同时存在的 mutable 和 immutable reference 也有限制

情况 1: 同一时刻一个值只能有一个 mutable reference

```rs
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s; // error: cannot borrow `s` as mutable more than once at a time
```

情况 2：在有 immutable reference 还在 scope 的情况下，不能创建 mutable reference

> a reference’s scope starts from where it is introduced and continues through the last time that reference is used.

```rs
let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // error: cannot borrow `s` as mutable because it is also borrowed as immutable
```

```rs
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem

println!("{} and {}", r1, r2);
// variables r1 and r2 scope ends here
// because this is the last time they are used

let r3 = &mut s; // no problem
println!("{}", r3);

```

```rs
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
              // a reference’s scope starts from where it is introduced and
              // continues through the last time that reference is used.
              // 所以在这里r1和r2还在scope中

println!("{} and {}", r1, r2); // variables r1 and r2 scope continues

let r3 = &mut s; // error: cannot borrow `s` as mutable because it is also borrowed as immutable
println!("{}", r3);

println!("{} and {}", r1, r2); // variables r1 and r2 scope ends here
// because this is the last time they are used
```

#### Dangling References

In languages with pointers, it’s easy to erroneously create a `dangling pointer`--a pointer that references a location in memory that may have been given to someone else--by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, **the compiler guarantees that references will never be dangling references**: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

```rs
fn dangle() -> &String { // returns a reference to a String
    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

let reference_to_nothing = dangle();
```

Because `s` is created inside dangle, when the code of dangle is finished, `s` will be **deallocated**. But we tried to return a reference to it. That means this reference would be pointing to an invalid String. That’s no good! Rust won’t let us do this.

#### Recap: The Rules of References

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

#### The Slice Type

`Slices` let you reference a continguous sequence of elements in a collection rather than the whole colletion.

- **A slice is a kind of reference, so it does not have ownership.**

#### String Slices

A `string slice` is a `reference` to part of a String.

```rs
let s = String::from("hello world");
let hello = &s[..5]; // 不包括5
let hello = &s[..=4]; // 包括4
let world = &s[6..];

let s1: &str = "hello world";
let s2: String = String::from("hello world");
let s3: &str = &s2[..]; // slice of entire string
println!("{}", s1 == s3); // true
```

> String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.

**String Literals Are Slices**

```rs
// s is &str type
let s = "hello world!";
```

`s` is a slice pointing to that specific point of binary. This is also why string literals are immutable; `&str` is an immutable reference.

#### String Slices As Parameters

```rs
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

pub fn run() {
    let s = String::from("hello world");
    let word = first_word(&s[..]);
    println!("{}", word);
    // also works on &String, which are equivalent to whole slices of String s
    let word = first_word(&s);
    println!("{}", word);
}

```

#### Other Slices

String slices are specific to strings. but there's more general slice type, too.

```rs
let a = [1, 2, 3, 4, 5];

let slice: &[i32] = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

This slice has type `&[i32]`. It works the same way as string slices do, by storing a reference to the first element and a length. You'll use this kind of slice for all sorts of other collections.

## Structs

Structs are similar to tuples, in that both hold multiple related values.

```rs
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let w = 32;
    let h = 64;
    let o = Object::new(w, h);
    println!("{} {}", w, h);

    let mut user1 = User {
        active: false,
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        username: String::from("someusername123"),
    };
    user1.active = true;
    println!("{:#?}", user1);
}

```

**Note that the entire instance must be mutable.**

Rust doesn't allow us to mark only certain fields as mutable.

### Struct Update syntax

The syntax `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

```rs
let mut user1 = User {
    active: false,
    email: String::from("someone@example.com"),
    sign_in_count: 1,
    username: String::from("someusername123"),
};
user1.active = true;
println!("{:#?}", user1);

let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
println!("{:#?}", user2);
println!("{:#?}", user1); // error! we can no longer use user1
```

Because the `String` in the `username` field of `user1` was moved into `user2`.

If we had given `user2` new `String` value for both `email` and `username`, and thus only used the `active` and `sign_in_count` values from `user1`, then `user1` would still be valid after creating `user2`.

The type of `active` and `sign_in_count` are types that implement the `Copy` trait, so they are copied instead of moved.

```rs
let user2 = User {
    // these two field would be moved, so we need to give them new value
    email: String::from("another@example.com"),
    username: String::from("anotherusername123"),
    ..user1 // the rest field are types that implement the `Copy` trait
};
println!("{:#?}", user2);
println!("{:#?}", user1);
```

#### Tuple Structs

**Rust also supports structs that look similar to tuples, called `tuple structs`**.
Tuple structs have the added meaning the struct name provides but don't have names associated with their fields.

> Tuple structs are useful when you want to **give the whole tuple a name and make the tuple a different type from other tuples**, and when naming each field as in a regular struct would be verbose or redundant.

```rs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

println!("black({},{},{})", black.0, black.1, black.2);
println!("origin({},{},{})", origin.0, origin.1, origin.2);
```

#### Unit-Like Structs Without Any Fields

You can also define structs that don't have any fields! These are called `unit-like` structs because they behave similarly to `()`/

> Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

```rs
struct AlwaysEqual;

let subject = AlwaysEqual;
```

### Ownership of Struct Data

```rs
struct User {
    username: String,
    email: String,
}
```

Here we use `String` type rather than the `&str` string slice type. Because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.

> It’s also possible for structs to store references to data owned by something else, but to do so requires the use of `lifetimes`.

> `Lifetimes` ensure that the data referenced by a struct is valid for as long as the struct is.

```rs
struct User {
    active: bool,
    // username: &str,
    // email: &str, error: expected named lifetime parameter
    sign_in_count: u64,
}
```

#### example

```rs
struct Rectangle {
    width: u32,
    height: u32,
}

let rect1 = Rectangle {
    width: 30,
    height: 50,
};

fn area1(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

println!("The area of the rectangle is {}", area1(&rect1));

println!("rect1 is {}", rect1); // error: Rectangle doesn't implement `std::fmt::Display` trait
```

The `println!` macro can do many kinds of formatting, and by default, the curly brackets tell `println!` to use formatting known as `Display`: output intended for direct end user consumption.

The primitive types we've seen so far implement `Display` by default, because there's only one way you'd want to show a `1` or any other primitive type to a user.

But with `structs`, the way `println!` should format the output is less clear because there are more display possibilities: Do you want commas or not? Do you want to print the curly brackets? Should all fields be shown? Due to this ambiguity, Rust doesn't try to guess what we want, and structs don't have a provided implementation of `Display` to use with `println!` and the `{}` placeholder.

> Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct.

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

let scale = 2;
let rect1 = Rectangle {
    width: dbg!(30 * scale), // dbg! returns ownership of the expression's value
    height: 50,
};

println!("rect1 is {:?}", rect1);

dbg!(&rect1);
```

Another way to print out a value using the `Debug` format is to use the `dbg!` macro, which takes ownership of an expression, prints the file and line number of where that `dbg!` macro call occurs in your code along with the resulting value of that expression, and returns ownership of the value.

### Method syntax

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self is short for self: &Self
    // this method borrows the Self instance
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn grow(&mut self, scale: u32) {
        self.width += scale;
        self.height += scale;
    }

    fn width(&self) -> bool {
      self.width > 0
    }
}

pub fn run() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    rect1.grow(10);

    println!(
        "The area of the reactangle is {} square pixels.",
        rect1.area()
    );
    println!("{}", rect1.width());
}
```

> When we give methods the same name as a field we want it to only return the value in the field and do nothing else. Methods like this are called getters, and Rust does not implement them automatically for struct fields as some other language do.

Getters are useful because you can make the field private but the method public and thus enable `read-only access` to that field as part of the type's public API.

> Where's the `->` Operator?
> In C and C++, you use `.` if you're calling a method on the object directly and `->` if you're calling the method on a pointer to the object and need to dereference the pointer first.

Rust doesn't have an equivalent to the `->` operator; instead, Rust has a feature called **auto referencing and deferencing**.

Calling methods is one of the few places in Rust that has this behavior.

**When you call a method with `object.something()`, Rust automatically adds in `&`, `&mut` or `*` so `object` matches the signature of the method.**

```rs
// 采用了automatic reference
p1.distance(&p2);
// 等同于手动取reference
(&p1).distance(&p2);

```

Given the receiver and name of a method, Rust can figure out definitively whether the method is reading `&self`, mutating `&mut self` or consuming (`self`).

All functions defined within an `impl` block are called **associated functions**.

```rs
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

let rect3 = Rectangle::square(10);
```

#### Multiple `impl` Blocks

```rs
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

## Enums

Enums are a way of defining custom data types in a different way than you do with structs.

```rs
enum IpAddrKind {
    V4,
    V6,
}

let four: IpAddrKind = IpAddrKind::V4;
let six: IpAddrKind = IpAddrKind::V6;

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

We can create instances of each of the two variants of `IpAddrKind`.

**We can put data directly into each enum variant.**

```rs
enum IpAddrKind {
    V4(String),
    V6(String),
}

let home = IpAddrKind::V4(String::from("127.0.0.1"));
let loopback = IpAddrKind::V6(String::from("::1"));
```

You can put any kind of data inside an enum variant.

```rs
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

You can have a wide variety of types embedded in its variants.

```rs
enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

// defining an enum with variants is similar to defining different kinds of struct definitions

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

```

But if we used the different structs, which each have their own type, we couldn't as easily define a function to take any of these kinds of messages as we could with the `Message` enum.

```rs
// 我们只需要给enum Message添加方法，即可加到每一个variant上
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
```

### The `Option` Enum and its Advantages Over Null values

The `Option` enum is defined by the standard library.

It encodes the very common scenario in which a value could be something or it could be nothing.

Expressing this concept in terms of the type system means the compiler can check whether you've handled all the cases you should be handling. This functionality can prevent bugs that are extremely common in other languages.

**Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent.** This enum is `Option<T>`, and is defined by the standard library as follows:

```rs
enum Option<T> {
    None,
    Some(T),
}
```

> The `Option<T>` enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly.

> Its variants are also included in the prelude: you can use `Some` and `None` directly without the `Option::` prefix.

```rs
let some_number: Option<i32> = Some(5);
let some_string: Option<&str> = Some("a string");
let absent_number: Option<i32> = None;

println!(
    "{} {} {}",
    some_number.unwrap(),       // return the contained value
    some_string.unwrap(),       // caution: this method will panic if `None`
    absent_number.unwrap_or(5)  // return the contained value or a default one
);
```

- `i8` and `Option<i8>` are different types. You have to convert an `Option<T>` to a `T` before you can perform `T` operations with it.

In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>. Then, when you use that value, you are required to explicitly handle the case when the value is null.

> Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null.

#### The `match` control flow

```rs
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

An `arm` has two partds: a pattern and some code.

When the `match` expression executes, it compares the resulting value against the pattern of each `arm` **in order**. _If a pattern matches the value, the code associated with that pattern is executed. If that pattern doesn't match the value, execution continues to the next arm, much as in a coin-sorting machine._

- We don’t typically use curly brackets if the match arm code is short,

```rs
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

#### Patterns that Bind to Values

Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how we can _extract values out of enum variants_.

```rs
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // get the inner state value
            println!("State quarter from:{:?}", state);
            25
        }
    }
}

println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
```

#### Matching with `Option<T>`

In the previous section, we wanted to get the inner `T` value out of the `Some` case when using `Option<T>`

```rs
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

Does `Some(5)` match `Some(i)`? Yes it does! We have the same variant. The `i` binds to the value contained in `Some`, so `i` takes the value `5`.

#### Matches are exhaustive

> Rust knows that we didn’t cover **every possible case** and even knows which pattern we forgot! Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid.

#### Catch-all pattern and the `_` placeholder

```rs
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}let dice_roll = 9;
match dice_roll {
    3 => add,
    7 => remove_fancy_hat(),
    other => move_player(other),
}

match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(), // catch all pattern
}
```

> Rust also has a pattern we can use when we don’t want to use the value in the catch-all pattern: `_`, which is a special pattern that matches any value and does not bind to that value.

### `if ket`

The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest.

```rs
let config_max = Some(3u8);
// We don't want to do anything with the `None` value
// To satisfy the match expression, we have to add _ => ()
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}

// equivalent to the above
let config_max = Some(3_u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max)
}
```

If the value is Some, we print out the value in the Some variant by _binding the value to the variable max in the pattern_.

When we don't want to do anything with the `None` pattern, we can use `if let` expression

> Using `if let` means less typing, less indentation, and less boilerplate code. However, you lose the **exhaustive checking that match enforces**.

```rs
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```
