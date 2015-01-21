mod hello {
    extern { pub fn print_it(n: i32); }
}

fn main() {
    unsafe {
        hello::print_it(10);
    }
}
