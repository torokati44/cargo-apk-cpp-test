fn main() {
    if cfg!(feature = "with_cpp") {
        println!("cargo:rustc-link-lib=c++_shared");
    }
}