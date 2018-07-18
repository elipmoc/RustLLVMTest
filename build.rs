extern crate cc;

fn main() {
    cc::Build::new()
        .out_dir("./")
        .file("src/stdlib/c/foo.c")
        .compile("foo");
}
