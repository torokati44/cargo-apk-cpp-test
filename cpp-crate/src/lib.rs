
extern {
    fn cpp_sub(a: i32, b:i32) -> i32;
}

pub fn sub(a: i32, b: i32) -> i32 {
    unsafe { cpp_sub(a, b) }
}