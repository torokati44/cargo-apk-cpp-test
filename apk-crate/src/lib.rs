use log::info;
use ndk::trace;

use c_crate::add;
#[cfg(feature = "with_cpp")]
use cpp_crate::sub;

#[cfg_attr(
    target_os = "android",
    ndk_glue::main(backtrace = "on", logger(level = "debug", tag = "hello-world"))
)]
fn main() {
    let _trace;
    if trace::is_trace_enabled() {
        _trace = trace::Section::new("ndk-rs example main").unwrap();
    }
    let x = add(2, 2);
    info!("hello world, x = 2 + 2 = {:}", x);

    #[cfg(feature = "with_cpp")]
    {
        let y = sub(x, 1);
        info!("hello world, y = x - 1 = {:}", y);
    }
}