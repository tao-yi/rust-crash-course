pub fn run() {
    let a = expr();
    empty()
}

fn expr() -> i32 {
    5 + 6 // evaluate to value 11
}

fn empty() {
    1;
}
