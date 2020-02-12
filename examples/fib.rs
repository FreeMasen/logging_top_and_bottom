use logging_top_and_bottom::prelude::*;

fn main() {
    simple_logger::init().unwrap();
    fib(2);
}

#[log_top_and_bottom]
pub fn fib(arg: u32) -> u32 {
    match arg {
        0 | 1 => 1,
        _ => fib(arg - 1) + fib(arg - 2)
    }
}