pub fn run() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // a reference’s scope starts from where it is introduced and
                 // continues through the last time that reference is used.
                 // 所以在这里r1和r2还在scope中

    println!("{} and {}", r1, r2);
    // variables r1 and r2 scope ends here
    // because this is the last time they are used

    let r3 = &mut s; // error: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{}", r3);

    // if we still use r1 and r2 here, we cannot have mutable reference r3 in the above
    // println!("{} and {}", r1, r2);

    // let reference_to_nothing = dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
