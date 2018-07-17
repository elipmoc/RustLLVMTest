extern crate cc;

fn main() {
    cc::Build::new().file("src/c/foo.c").compile("foo");
}
