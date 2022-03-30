use c_crate::add;
#[cfg(feature = "with_cpp")]
use cpp_crate::sub;

fn main() {
    let x = add(2, 2);
    println!("hello world, x = 2 + 2 = {:}", x);
    #[cfg(feature = "with_cpp")]
    {
        let y = sub(x, 1);
        println!("hello world, y = x - 1 = {:}", y);
    }
}