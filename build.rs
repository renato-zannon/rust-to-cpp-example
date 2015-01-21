extern crate gcc;
use std::default::Default;

fn main() {
    gcc::compile_library("libhello.a", &Default::default(), &["cpp/hello.cpp"]);
    println!("cargo:rustc-flags=-l dylib=stdc++");
}
