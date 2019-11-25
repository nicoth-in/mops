extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/corelib/cpp/lib.cpp")
        .cpp(true)
        .compile("libcore.a");
}
