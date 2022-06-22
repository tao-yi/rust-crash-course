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

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
