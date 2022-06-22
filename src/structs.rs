struct Object {
    width: u32,
    height: u32,
}

impl Object {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn area(obj: Object) -> u32 {
    obj.width * obj.height
}

pub fn run() {
    let w = 32;
    let h = 64;
    let o = Object::new(w, h);
    println!("{} {}", w, h);
}
