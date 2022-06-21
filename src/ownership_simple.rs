fn re(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[120] + v[111]);
    v
}

fn borrow1(v: &Vec<i32>) {
    // deference explicitly
    println!("{}", (*v)[10] + (*v)[12]);
}

fn borrow2(v: &Vec<i32>) {
    // deference implicitly
    println!("{}", v[10] + v[11]);
}

fn copy(a: i32, b: i32) {
    println!("{}", a + b);
}

pub fn run() {
    let a = 32;
    let b = 45;
    // i32 are being copied because they exist on the stack
    copy(a, b);
    println!("Still own a {} b {}", a, b);

    let mut v = Vec::new();

    for i in 1..1000 {
        v.push(i);
    }

    // move and give back ownership
    v = re(v);
    println!("Still own v: {} {}", v[0], v[1]);

    // borrow ownership
    borrow1(&v);
    println!("Still own v: {} {}", v[0], v[1]);

    borrow2(&v);
    println!("Still own v: {} {}", v[0], v[1]);

    let v = vec![4, 5, 3, 6, 4, 1, 3, 42, 65, 7, 1, 3];
    for i in &v {
        let r = count(&v, *i);
        println!("{} is repeated {} times", i, r);
    }

    println!("{}", v[0]);
}

fn count(v: &Vec<i32>, val: i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}
