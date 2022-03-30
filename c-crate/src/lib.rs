
extern {
    fn c_add(a: i32, b:i32) -> i32;
}

pub fn add(a: i32, b: i32) -> i32 {
    unsafe { c_add(a, b) }
}
