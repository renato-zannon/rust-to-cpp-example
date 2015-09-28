extern crate gcc;

fn main() {
    gcc::compile_library("libhello.a", &["cpp/hello.cpp"]);
    println!("cargo:rustc-flags=-l dylib=stdc++");
}
