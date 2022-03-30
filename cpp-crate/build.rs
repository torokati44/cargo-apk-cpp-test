
fn main() {
    cc::Build::new()
        .file("src/sub.cpp")
        .compile("cpp_sub");
}