
fn main() {
    cc::Build::new()
        .file("src/add.c")
        .compile("c_add");
}